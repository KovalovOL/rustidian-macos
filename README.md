# Rustidian for macOS

A native Rust Markdown notebook optimized for macOS, forked from
[MasihMoafi/rustidian](https://github.com/MasihMoafi/rustidian).

Rust and egui/eframe render the editor and notebook. AppKit provides native
window controls, menus, file panels and Finder integration. Apple Silicon is
the primary optimization target.

## Build and run

Requires macOS, Xcode Command Line Tools and Rust 1.95 or newer.

```sh
cargo build --locked --release --bin rustidian
./target/release/rustidian ./demo-vault
```

To open your notes, pass a folder or Markdown file path. Try a separate vault
copy first; keep original notes and backups intact.

```sh
./target/release/rustidian "/path/to/vault-copy"
```

## Build a macOS application

```sh
python3 scripts/package-macos.py --binary target/release/rustidian --output dist/Rustidian.app
```

The packager creates a self-contained `.app`, ZIP and SHA-256 checksum. It
includes license notices and a demo vault copied into writable application
data on first launch. An existing output `.app` is never overwritten.
Local builds are ad-hoc signed; Developer ID signing and notarization are
separate distribution steps. The macOS 12 deployment setting is not proof
of runtime testing on that version.

## Editing

Open ordinary Markdown folders without an account. Use Command+P for commands,
Command+O for the note switcher, Command+S to save and Command+E to switch
editing/reading. Control+Tab switches document tabs.

The notebook supports Markdown editing, wiki links, backlinks, search,
attachments, themes and recovery. External-change protection prevents silent
overwrites. Existing Obsidian community JavaScript plugins do not run;
Canvas/Bases and full Obsidian parity remain incomplete.

## Source layout

- `src/`: application code, including source-local unit tests.
- `vendor/`: required patched egui/epaint sources and notices.
- `assets/`: embedded fonts, UI icons, palette logos and their notices.
- `demo-vault/`: first-launch notebook resources.
- `scripts/package-macos.py`: macOS bundle source.
- [OpenSpec proposal](openspec/changes/macos-focused-product-spec/proposal.md),
  [design](openspec/changes/macos-focused-product-spec/design.md) and
  [tasks](openspec/changes/macos-focused-product-spec/tasks.md).
- [Historical native macOS baseline](specs/001-native-macos/spec.md).

Standalone integration/GUI tests, CI workflows, experiments, benchmarks,
artwork generation/proof files and Linux installers were removed in the
source-only cleanup. Existing platform-gated application code is retained.

```sh
cargo fmt --all -- --check
cargo test --locked --lib --bins
```

## License and attribution

Original application copyright: 2026 Masih Moafi. Application code is
[MIT licensed](LICENSE). Retain [third-party notices](THIRD_PARTY_NOTICES.txt),
`vendor/LICENSE-*`, `vendor/NOTICE.txt` and notices/licenses under `assets/`
when distributing the retained components. The macOS packager includes them.

Rustidian is independent and unaffiliated with Obsidian. The source license
does not grant rights to use the Rustidian name or logo in a way that implies
endorsement; see `assets/brand/NOTICE.txt`.
