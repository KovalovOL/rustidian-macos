//! AppKit integration, kept out of Linux builds. All panel calls run on the UI thread.
use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained};
use objc2_app_kit::{
    NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSApplication, NSModalResponseOK,
    NSOpenPanel, NSPasteboard, NSPasteboardTypeString, NSWorkspace,
};
use objc2_foundation::{NSArray, NSString, NSURL};
use std::{
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver, TryRecvError},
};

fn main_thread() -> Result<MainThreadMarker, String> {
    MainThreadMarker::new().ok_or_else(|| "Native macOS dialogs require the main thread.".into())
}

/// Set `None` to follow the OS; panels, title bars and menus share the chosen appearance.
pub fn appearance(dark: Option<bool>) {
    let Ok(mtm) = main_thread() else { return };
    let application = NSApplication::sharedApplication(mtm);
    let appearance = dark.and_then(|dark| {
        // SAFETY: AppKit exports immutable, process-lifetime appearance names.
        let name = unsafe {
            if dark {
                NSAppearanceNameDarkAqua
            } else {
                NSAppearanceNameAqua
            }
        };
        NSAppearance::appearanceNamed(name)
    });
    application.setAppearance(appearance.as_deref());
}

pub fn clipboard_text() -> Option<String> {
    // SAFETY: called by the main-thread menu bridge, using AppKit's string pasteboard type.
    unsafe { NSPasteboard::generalPasteboard().stringForType(NSPasteboardTypeString) }
        .map(|text| text.to_string())
}

pub fn reveal(path: &Path) -> Result<(), String> {
    let url = NSURL::from_file_path(path)
        .ok_or_else(|| "Cannot represent this file as a macOS file URL.".to_owned())?;
    // NSWorkspace permits this operation from a worker thread.
    NSWorkspace::sharedWorkspace()
        .activateFileViewerSelectingURLs(&NSArray::from_retained_slice(&[url]));
    Ok(())
}

#[derive(Default)]
pub struct Picker {
    panel: Option<Retained<NSOpenPanel>>,
    result: Option<Receiver<Result<Option<PathBuf>, String>>>,
    kind: crate::native_picker::Kind,
}

impl Picker {
    pub fn active(&self) -> bool {
        self.panel.is_some()
    }
    pub fn kind(&self) -> crate::native_picker::Kind {
        self.kind
    }

    pub fn start(
        &mut self,
        kind: crate::native_picker::Kind,
        initial: Option<&Path>,
        title: &str,
    ) -> Result<(), String> {
        if self.active() {
            return Err("A file browser is already open. Finish or cancel it first.".into());
        }
        let panel = NSOpenPanel::openPanel(main_thread()?);
        panel.setTitle(Some(&NSString::from_str(title)));
        panel.setCanChooseDirectories(kind == crate::native_picker::Kind::Folder);
        panel.setCanChooseFiles(kind == crate::native_picker::Kind::File);
        panel.setAllowsMultipleSelection(false);
        panel.setCanCreateDirectories(kind == crate::native_picker::Kind::Folder);
        panel.setResolvesAliases(true);
        if let Some(path) = initial {
            let directory = if path.is_dir() {
                Some(path)
            } else {
                path.parent()
            };
            if let Some(url) = directory.and_then(NSURL::from_file_path) {
                panel.setDirectoryURL(Some(&url));
            }
        }
        let (sender, result) = mpsc::channel();
        let selected_panel = panel.clone();
        let completion = RcBlock::new(move |response| {
            let result = if response == NSModalResponseOK {
                selected_panel
                    .URL()
                    .and_then(|url| url.to_file_path())
                    .map(Some)
                    .ok_or_else(|| "The file browser returned no local file.".into())
            } else {
                Ok(None)
            };
            let _ = sender.send(result);
        });
        // AppKit copies the completion block; its captured panel stays retained until completion.
        panel.beginWithCompletionHandler(&completion);
        self.kind = kind;
        self.result = Some(result);
        self.panel = Some(panel);
        Ok(())
    }

    pub fn poll(&mut self) -> Option<Result<Option<PathBuf>, String>> {
        match self.result.as_ref()?.try_recv() {
            Ok(result) => {
                self.result = None;
                self.panel = None;
                Some(result)
            }
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => {
                self.cancel();
                Some(Err("The macOS file browser closed unexpectedly.".into()))
            }
        }
    }

    pub fn cancel(&mut self) {
        if let Some(panel) = self.panel.take() {
            // SAFETY: nil is a valid sender; the retained panel is used on its owning UI thread.
            unsafe { panel.cancel(None) };
        }
        self.result = None;
    }
}

impl Drop for Picker {
    fn drop(&mut self) {
        self.cancel();
    }
}
