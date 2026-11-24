//! OS-owned application folders. Explicit XDG overrides also work on macOS.
use std::path::PathBuf;

#[cfg(target_os = "macos")]
pub mod macos;

fn base(variable: &str, unix_suffix: &str) -> Option<PathBuf> {
    std::env::var_os(variable).map(PathBuf::from).or_else(|| {
        let home = PathBuf::from(std::env::var_os("HOME")?);
        if cfg!(target_os = "macos") {
            Some(home.join("Library/Application Support"))
        } else {
            Some(home.join(unix_suffix))
        }
    })
}

pub fn data_base() -> Option<PathBuf> {
    base("XDG_DATA_HOME", ".local/share")
}

pub fn state_base() -> Option<PathBuf> {
    base("XDG_STATE_HOME", ".local/state")
}

/// Match the platform's modifier in command palettes and tooltips.
pub fn shortcut(label: &str) -> String {
    if cfg!(target_os = "macos") {
        if label == "Ctrl+Tab" {
            return "⌃⇥".into();
        }
        if label == "Ctrl+Shift+Tab" {
            return "⌃⇧⇥".into();
        }
        label
            .replace("Ctrl+", "⌘")
            .replace("Alt+", "⌥")
            .replace("Shift+", "⇧")
    } else {
        label.to_owned()
    }
}
