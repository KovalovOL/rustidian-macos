//! Finder/Open With document delivery without replacing winit's application delegate.
use eframe::egui;
use objc2::{
    DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send, rc::Retained, sel,
};
use objc2_app_kit::NSApplication;
use objc2_foundation::{
    NSAppleEventDescriptor, NSAppleEventManager, NSObject, NSObjectProtocol, NSURL,
};
use std::{
    path::PathBuf,
    sync::{
        Arc, OnceLock,
        mpsc::{self, Receiver, Sender},
    },
};

const CORE_EVENT: u32 = u32::from_be_bytes(*b"aevt");
const OPEN_DOCUMENTS: u32 = u32::from_be_bytes(*b"odoc");
const QUIT: u32 = u32::from_be_bytes(*b"quit");
const DIRECT_OBJECT: u32 = u32::from_be_bytes(*b"----");
const FILE_URL: u32 = u32::from_be_bytes(*b"furl");

pub enum Request {
    Open(PathBuf),
    Quit,
}

struct State {
    sender: Sender<Request>,
    context: Arc<OnceLock<egui::Context>>,
}

define_class!(
    // SAFETY: NSObject has no additional subclass requirements; the class is main-thread-only.
    #[unsafe(super = NSObject)]
    #[name = "RustidianOpenDocumentsHandler"]
    #[thread_kind = MainThreadOnly]
    #[ivars = State]
    struct Handler;
    unsafe impl NSObjectProtocol for Handler {}
    impl Handler {
        #[unsafe(method(handleOpenDocuments:withReplyEvent:))]
        fn open_documents(&self, event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            eprintln!("macos-open: received a Finder document event");
            let Some(list) = event.paramDescriptorForKeyword(DIRECT_OBJECT) else { return };
            for index in 1..=list.numberOfItems() {
                let path = list.descriptorAtIndex(index)
                    .and_then(|item| item.coerceToDescriptorType(FILE_URL))
                    .and_then(|item| NSURL::URLWithDataRepresentation_relativeToURL(&item.data(), None).to_file_path());
                if let Some(path) = path {
                    eprintln!("macos-open: {}", path.display());
                    let _ = self.ivars().sender.send(Request::Open(path));
                }
            }
            if let Some(ctx) = self.ivars().context.get() { ctx.request_repaint(); }
        }
        #[unsafe(method(handleQuit:withReplyEvent:))]
        fn quit_requested(&self, _event: &NSAppleEventDescriptor, _reply: &NSAppleEventDescriptor) {
            let _ = self.ivars().sender.send(Request::Quit);
            if let Some(ctx) = self.ivars().context.get() { ctx.request_repaint(); }
        }
    }
);

pub struct OpenDocuments {
    _handler: Retained<Handler>,
    pub receiver: Receiver<Request>,
}

impl OpenDocuments {
    pub fn new(context: Arc<OnceLock<egui::Context>>) -> Self {
        let mtm = MainThreadMarker::new().expect("The application starts on the macOS main thread");
        // AppKit installs its defaults first; only open/quit are replaced below.
        let _application = NSApplication::sharedApplication(mtm);
        let (sender, receiver) = mpsc::channel();
        let allocated = Handler::alloc(mtm).set_ivars(State { sender, context });
        // SAFETY: NSObject's init has the standard retained-object signature.
        let handler: Retained<Handler> = unsafe { msg_send![super(allocated), init] };
        let events = Self {
            _handler: handler,
            receiver,
        };
        events.install();
        events
    }

    pub fn install(&self) {
        // SAFETY: Handler implements precisely this AppleEvent callback signature and is retained
        // for the life of the application. Existing winit delegate methods are left intact.
        unsafe {
            NSAppleEventManager::sharedAppleEventManager()
                .setEventHandler_andSelector_forEventClass_andEventID(
                    &self._handler,
                    sel!(handleOpenDocuments:withReplyEvent:),
                    CORE_EVENT,
                    OPEN_DOCUMENTS,
                )
        };
        // SAFETY: same retained handler; Quit is routed through the existing unsaved-edit guard.
        unsafe {
            NSAppleEventManager::sharedAppleEventManager()
                .setEventHandler_andSelector_forEventClass_andEventID(
                    &self._handler,
                    sel!(handleQuit:withReplyEvent:),
                    CORE_EVENT,
                    QUIT,
                )
        };
    }
}

impl Drop for OpenDocuments {
    fn drop(&mut self) {
        NSAppleEventManager::sharedAppleEventManager()
            .removeEventHandlerForEventClass_andEventID(CORE_EVENT, OPEN_DOCUMENTS);
        NSAppleEventManager::sharedAppleEventManager()
            .removeEventHandlerForEventClass_andEventID(CORE_EVENT, QUIT);
    }
}
