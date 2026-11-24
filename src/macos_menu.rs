//! Native application menu. Menu callbacks wake egui and use its existing input/command paths.
use eframe::egui::{self, Event, Key, Modifiers};
use muda::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};
use std::sync::mpsc::{self, Receiver};

#[derive(Clone, Debug)]
pub enum Action {
    Command(&'static str),
    Key(Key, bool),
    Copy,
    Cut,
    Paste,
    Quit,
    Appearance(Option<bool>),
}

impl Action {
    pub fn input(&self) -> Option<Vec<Event>> {
        match self {
            Self::Key(key, shift) => {
                let modifiers = Modifiers {
                    shift: *shift,
                    ..Modifiers::COMMAND
                };
                Some(
                    [true, false]
                        .into_iter()
                        .map(|pressed| Event::Key {
                            key: *key,
                            physical_key: None,
                            pressed,
                            repeat: false,
                            modifiers,
                        })
                        .collect(),
                )
            }
            Self::Copy => Some(vec![Event::Copy]),
            Self::Cut => Some(vec![Event::Cut]),
            Self::Paste => {
                if let Some(text) = rustidian::platform::macos::clipboard_text() {
                    return Some(vec![Event::Paste(text)]);
                }
                let image = arboard::Clipboard::new()
                    .ok()
                    .and_then(|mut clipboard| clipboard.get_image().ok());
                Some(
                    image
                        .map(|image| {
                            Event::PasteImage(std::sync::Arc::new(
                                egui::ColorImage::from_rgba_unmultiplied(
                                    [image.width, image.height],
                                    &image.bytes,
                                ),
                            ))
                        })
                        .into_iter()
                        .collect(),
                )
            }
            _ => None,
        }
    }
}

pub struct NativeMenu {
    _menu: Menu,
    actions: Receiver<String>,
    bindings: Vec<(MenuItem, Action)>,
}

fn add(
    menu: &Submenu,
    bindings: &mut Vec<(MenuItem, Action)>,
    id: &str,
    title: &str,
    shortcut: Option<&str>,
    action: Action,
) -> Result<(), Box<dyn std::error::Error>> {
    let accelerator = shortcut.map(str::parse).transpose()?;
    let item = MenuItem::with_id(id, title, true, accelerator);
    menu.append(&item)?;
    bindings.push((item, action));
    Ok(())
}

impl NativeMenu {
    pub fn new(ctx: egui::Context) -> Result<Self, Box<dyn std::error::Error>> {
        let menu = Menu::new();
        let mut bindings = Vec::new();
        let app = Submenu::new("Rustidian", true);
        app.append(&PredefinedMenuItem::about(
            Some("About Rustidian"),
            Some(muda::AboutMetadata {
                name: Some("Rustidian for macOS".into()),
                version: Some(env!("CARGO_PKG_VERSION").into()),
                comments: Some(
                    "A native Markdown notebook. Independent; not affiliated with Obsidian.".into(),
                ),
                ..Default::default()
            }),
        ))?;
        app.append(&PredefinedMenuItem::separator())?;
        add(
            &app,
            &mut bindings,
            "settings",
            "Settings…",
            Some("Cmd+Comma"),
            Action::Command("Open settings"),
        )?;
        app.append_items(&[
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::services(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::hide_others(None),
            &PredefinedMenuItem::show_all(None),
            &PredefinedMenuItem::separator(),
        ])?;
        add(
            &app,
            &mut bindings,
            "quit",
            "Quit Rustidian",
            Some("Cmd+Q"),
            Action::Quit,
        )?;

        let file = Submenu::new("File", true);
        for (id, label, shortcut, command) in [
            ("new-note", "New Note", "Cmd+N", "New note"),
            ("new-tab", "New Tab", "Cmd+T", "New tab"),
            ("quick-open", "Quick Open…", "Cmd+O", "Open a file"),
            (
                "open-file",
                "Open File…",
                "Cmd+Shift+O",
                rustidian::commands::OPEN_FILE,
            ),
            (
                "open-folder",
                "Open Folder…",
                "Cmd+Alt+O",
                rustidian::commands::OPEN_FOLDER,
            ),
        ] {
            add(
                &file,
                &mut bindings,
                id,
                label,
                Some(shortcut),
                Action::Command(command),
            )?;
        }
        file.append(&PredefinedMenuItem::separator())?;
        add(
            &file,
            &mut bindings,
            "save",
            "Save",
            Some("Cmd+S"),
            Action::Command("Save note"),
        )?;
        add(
            &file,
            &mut bindings,
            "save-copy",
            "Save a Copy…",
            None,
            Action::Command("Save a copy"),
        )?;
        file.append(&PredefinedMenuItem::separator())?;
        add(
            &file,
            &mut bindings,
            "close-tab",
            "Close Tab",
            Some("Cmd+W"),
            Action::Command("Close current tab"),
        )?;
        add(
            &file,
            &mut bindings,
            "reopen-tab",
            "Reopen Closed Tab",
            Some("Cmd+Shift+T"),
            Action::Command(rustidian::commands::UNDO_CLOSE_TAB),
        )?;

        let edit = Submenu::new("Edit", true);
        for (id, title, shortcut, action) in [
            ("undo", "Undo", "Cmd+Z", Action::Key(Key::Z, false)),
            ("redo", "Redo", "Cmd+Shift+Z", Action::Key(Key::Z, true)),
            ("cut", "Cut", "Cmd+X", Action::Cut),
            ("copy", "Copy", "Cmd+C", Action::Copy),
            ("paste", "Paste", "Cmd+V", Action::Paste),
            (
                "select-all",
                "Select All",
                "Cmd+A",
                Action::Key(Key::A, false),
            ),
        ] {
            add(&edit, &mut bindings, id, title, Some(shortcut), action)?;
        }
        edit.append(&PredefinedMenuItem::separator())?;
        add(
            &edit,
            &mut bindings,
            "find",
            "Find in Note…",
            Some("Cmd+F"),
            Action::Command("Find in note"),
        )?;
        add(
            &edit,
            &mut bindings,
            "search",
            "Search Vault…",
            Some("Cmd+Shift+F"),
            Action::Command("Search vault"),
        )?;

        let format = Submenu::new("Format", true);
        add(
            &format,
            &mut bindings,
            "bold",
            "Bold",
            Some("Cmd+B"),
            Action::Key(Key::B, false),
        )?;
        add(
            &format,
            &mut bindings,
            "italic",
            "Italic",
            Some("Cmd+I"),
            Action::Key(Key::I, false),
        )?;
        add(
            &format,
            &mut bindings,
            "link",
            "Insert Link",
            Some("Cmd+K"),
            Action::Key(Key::K, false),
        )?;

        let view = Submenu::new("View", true);
        for (id, title, command) in [
            ("live", "Live Preview", "Live preview"),
            ("source", "Source Mode", "Source editing"),
            ("reading", "Reading View", "Reading view"),
            ("split", "Split View", "Split source and preview"),
            ("left-sidebar", "Toggle Left Sidebar", "Toggle left sidebar"),
            (
                "right-sidebar",
                "Toggle Right Sidebar",
                "Toggle right sidebar",
            ),
        ] {
            add(
                &view,
                &mut bindings,
                id,
                title,
                None,
                Action::Command(command),
            )?;
        }
        view.append(&PredefinedMenuItem::separator())?;
        add(
            &view,
            &mut bindings,
            "commands",
            "Command Palette…",
            Some("Cmd+P"),
            Action::Command("Command palette"),
        )?;
        let appearance = Submenu::new("Appearance", true);
        for (id, title, value) in [
            ("appearance-system", "Follow System", None),
            ("appearance-light", "Light", Some(false)),
            ("appearance-dark", "Dark", Some(true)),
        ] {
            add(
                &appearance,
                &mut bindings,
                id,
                title,
                None,
                Action::Appearance(value),
            )?;
        }
        view.append(&appearance)?;

        let window = Submenu::new("Window", true);
        window.append_items(&[
            &PredefinedMenuItem::minimize(None),
            &PredefinedMenuItem::maximize(Some("Zoom")),
            &PredefinedMenuItem::fullscreen(None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::bring_all_to_front(None),
        ])?;
        window.set_as_windows_menu_for_nsapp();
        let help = Submenu::new("Help", true);
        add(
            &help,
            &mut bindings,
            "help-commands",
            "Show Commands…",
            None,
            Action::Command("Command palette"),
        )?;
        help.set_as_help_menu_for_nsapp();
        menu.append_items(&[&app, &file, &edit, &format, &view, &window, &help])?;
        menu.init_for_nsapp();
        let (sender, actions) = mpsc::channel();
        MenuEvent::set_event_handler(Some(move |event: MenuEvent| {
            let _ = sender.send(event.id.0);
            ctx.request_repaint();
        }));
        Ok(Self {
            _menu: menu,
            actions,
            bindings,
        })
    }

    pub fn take_actions(&self) -> Vec<Action> {
        self.actions
            .try_iter()
            .filter_map(|id| {
                self.bindings
                    .iter()
                    .find(|(item, _)| item.id().0 == id)
                    .map(|(_, action)| action.clone())
            })
            .collect()
    }

    pub fn set_enabled(&self, ready: bool, document: bool) {
        for (item, action) in &self.bindings {
            let enabled = match action {
                Action::Quit | Action::Key(..) | Action::Copy | Action::Cut | Action::Paste => true,
                Action::Command(command)
                    if matches!(
                        *command,
                        "Save note"
                            | "Save a copy"
                            | "Close current tab"
                            | "Live preview"
                            | "Source editing"
                            | "Reading view"
                            | "Split source and preview"
                            | "Find in note"
                    ) =>
                {
                    ready && document
                }
                _ => ready,
            };
            item.set_enabled(enabled);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menu_undo_and_redo_feed_balanced_command_events_to_the_editor() {
        for shift in [false, true] {
            let events = Action::Key(Key::Z, shift).input().unwrap();
            assert_eq!(events.len(), 2);
            for (event, expected_pressed) in events.iter().zip([true, false]) {
                assert!(
                    matches!(event, Event::Key { key: Key::Z, pressed, modifiers, .. }
                    if *pressed == expected_pressed && modifiers.command && modifiers.shift == shift)
                );
            }
        }
    }
}
