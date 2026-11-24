#![cfg(target_os = "macos")]

#[test]
fn native_file_urls_round_trip_unicode_spaces_hashes_and_newlines() {
    use objc2_foundation::NSURL;
    let root = tempfile::tempdir().unwrap();
    let path = root.path().join("Нотатки #1 % test\n.md");
    std::fs::write(&path, "Українська нотатка").unwrap();
    let url = NSURL::from_file_path(&path).unwrap();
    assert_eq!(url.to_file_path().unwrap(), path);
}

#[test]
fn macos_displays_command_shortcuts_but_keeps_control_tab_for_document_navigation() {
    use rustidian::platform::shortcut;
    assert_eq!(shortcut("Ctrl+S"), "⌘S");
    assert_eq!(shortcut("Ctrl+Shift+O"), "⌘⇧O");
    assert_eq!(shortcut("Ctrl+Alt+O"), "⌘⌥O");
    assert_eq!(shortcut("Ctrl+Tab"), "⌃⇥");
    assert_eq!(shortcut("Ctrl+Shift+Tab"), "⌃⇧⇥");
}
