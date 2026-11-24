# Rustidian for macOS

This fork adds native macOS integration to [MasihMoafi/rustidian](https://github.com/MasihMoafi/rustidian), preserving the Linux implementation. It is independent of Obsidian.

## Native integration

- AppKit windows with system-managed rounded corners, shadows, traffic lights, resizing, tiling and full-screen support.
- A real application menu: Rustidian, File, Edit, Format, View, Window and Help. Menu actions use the same editor and save paths as keyboard commands.
- Asynchronous AppKit file/folder dialogs and Reveal in Finder; no Zenity, D-Bus or Fontconfig requirement on macOS.
- Finder/Open With document events, with alternate Markdown/text associations that leave the default editor unchanged.
- CoreText-backed system font discovery and font loading.
- Command-key shortcuts and macOS shortcut labels; Control+Tab switches document tabs, leaving Command+Tab to the OS.
- Obsidian-inspired neutral light/dark palettes with a purple accent. The macOS default is this palette; Appearance can follow the OS or stay light/dark.
- Application data and recovery in `~/Library/Application Support/rustidian`, outside the vault. Explicit XDG overrides remain supported.
- Atomic, no-overwrite file/folder renames via `renamex_np(RENAME_EXCL)`, with source-identity and external-change checks retained.

## Build and package

Install Xcode Command Line Tools and Rust 1.95+, then:

```sh
cargo build --locked --release --bin rustidian
python3 scripts/package-macos.py --binary target/release/rustidian --output dist/Rustidian.app
```

The script packages the app, fonts already embedded in the executable, a writable-on-first-run demo vault, license notices, a ZIP and its SHA-256 checksum. Run the app directly or move it to Applications. First launch copies the bundled demo into application data; it never edits notes inside the installed app.

Local/CI builds are ad-hoc signed. Public distribution with Gatekeeper trust requires the maintainer's Apple Developer ID and notarization; the script accepts `--sign-identity`. No signing credentials are stored in this repository.

The macOS workflow builds/tests both Apple Silicon and Intel. macOS 12+ is the packaging deployment target; verify the specific OS/architecture before claiming it was tested.

## Verify

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --release --all-targets --no-fail-fast
```

An opt-in GUI probe captures native window/menu assertions and the application's own rendered content, without screen-recording or accessibility permissions:

```sh
RUSTIDIAN_MACOS_QA_DIR=/tmp/rustidian-qa RUSTIDIAN_MACOS_QA_EXIT=1 \
  dist/Rustidian.app/Contents/MacOS/rustidian demo-vault
```

Use a disposable demo/copy and isolated `XDG_DATA_HOME`/`XDG_STATE_HOME` for QA. Keep real notes out of tests, screenshots and commits.

## Compatibility

Try a complete copy of an Obsidian vault and retain a verified backup. Markdown, attachments and `.obsidian` files remain ordinary files; the fork does not run Obsidian community plugins. Canvas, Bases and other upstream compatibility gaps remain. macOS integration does not imply full Obsidian feature compatibility.
