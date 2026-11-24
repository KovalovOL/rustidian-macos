//! Opt-in, app-owned QA capture. It needs neither screen-recording nor accessibility access.
use eframe::egui;
use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSView, NSWindowButton, NSWindowStyleMask};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::path::PathBuf;

#[derive(Default)]
pub struct Capture {
    directory: Option<PathBuf>,
    passes: u8,
}

impl Capture {
    pub fn from_env() -> Self {
        Self {
            directory: std::env::var_os("RUSTIDIAN_MACOS_QA_DIR").map(PathBuf::from),
            passes: 0,
        }
    }

    pub fn tick(
        &mut self,
        ctx: &egui::Context,
        frame: &eframe::Frame,
        ready: bool,
    ) -> Result<(), String> {
        let Some(directory) = self.directory.as_ref() else {
            return Ok(());
        };
        self.passes = self.passes.saturating_add(1);
        if !ready || self.passes < 5 {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
            return Ok(());
        }
        let directory = directory.clone();
        std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        let RawWindowHandle::AppKit(handle) =
            frame.window_handle().map_err(|e| e.to_string())?.as_raw()
        else {
            return Err("Expected a native AppKit window".into());
        };
        // SAFETY: eframe owns this live main-thread NSView for the duration of the UI pass.
        let view = unsafe { &*handle.ns_view.as_ptr().cast::<NSView>() };
        let window = view.window().ok_or("The AppKit view has no window")?;
        if let Ok(expected) = std::env::var("RUSTIDIAN_MACOS_QA_EXPECT_TITLE")
            && !window.title().to_string().contains(&expected)
        {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
            return Ok(());
        }
        self.directory = None;
        let style = window.styleMask();
        let traffic_lights: Vec<_> = [
            NSWindowButton::CloseButton,
            NSWindowButton::MiniaturizeButton,
            NSWindowButton::ZoomButton,
        ]
        .into_iter()
        .map(|kind| {
            window
                .standardWindowButton(kind)
                .is_some_and(|button| !button.isHidden())
        })
        .collect();
        let mtm = MainThreadMarker::new().ok_or("Expected the main thread")?;
        let app = NSApplication::sharedApplication(mtm);
        let menu = app
            .mainMenu()
            .ok_or("The native application menu is missing")?;
        let menus: Vec<_> = menu
            .itemArray()
            .iter()
            .map(|item| item.title().to_string())
            .collect();
        let titled = style.contains(NSWindowStyleMask::Titled);
        let resizable = style.contains(NSWindowStyleMask::Resizable);
        let native_chrome = titled && resizable && traffic_lights.iter().all(|visible| *visible);
        let expected_menus = ["File", "Edit", "Format", "View", "Window", "Help"];
        let menu_complete = expected_menus
            .iter()
            .all(|name| menus.iter().any(|item| item == name));
        let theme = ctx.style_of(ctx.theme()).visuals.clone();
        let report = serde_json::json!({
            "native_chrome": native_chrome, "titled": titled, "resizable": resizable,
            "traffic_lights_visible": traffic_lights, "native_menus": menus, "menus_complete": menu_complete,
            "has_shadow": window.hasShadow(), "dark": theme.dark_mode,
            "window_title": window.title().to_string(),
            "document_background": theme.window_fill.to_array(),
            "sidebar_background": theme.panel_fill.to_array(),
            "appearance": app.appearance().as_deref().map(|appearance| appearance.name().to_string()),
        });
        std::fs::write(
            directory.join("native.json"),
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        if !native_chrome || !menu_complete {
            return Err("Native macOS chrome/menu assertions failed".into());
        }
        let quit = std::env::var_os("RUSTIDIAN_MACOS_QA_EXIT").is_some();
        let ctx = ctx.clone();
        ctx.clone().request_screenshot(move |image| {
            let pixels: Vec<u8> = image
                .pixels
                .iter()
                .flat_map(|pixel| pixel.to_array())
                .collect();
            match image::save_buffer(
                directory.join("content.png"),
                &pixels,
                image.size[0] as u32,
                image.size[1] as u32,
                image::ColorType::Rgba8,
            ) {
                Ok(()) => eprintln!("macos-qa: native chrome, menus, and screenshot passed"),
                Err(error) => eprintln!("macos-qa: screenshot failed: {error}"),
            }
            if quit {
                crate::window::close(&ctx, "macOS QA complete");
            }
        });
        Ok(())
    }
}
