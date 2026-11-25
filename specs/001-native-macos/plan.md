# Implementation plan and architecture

This is the as-built plan for spec 001. Keep it synchronized when changing these decisions.

## Design

1. **Keep the Rust renderer and application model.** egui/eframe/Glow continues to render notes and application content. Native macOS integration is additive behind `cfg(target_os = "macos")`; Linux behavior retains its own backend.
2. **Let AppKit own window chrome.** `main.rs` enables system decorations on macOS. `window.rs` omits the custom Windows-style controls there. The initial macOS size is 1180×760 logical pixels.
3. **Bridge native menus to existing behavior.** `macos_menu.rs` uses muda. Its callbacks enqueue actions and request an egui repaint. `app.rs` dispatches existing commands or injects normal editor input, including text/image paste.
4. **Bridge Finder and system quit events.** `macos_events.rs` registers retained AppleEvent handlers without replacing winit's delegate. Registration is repeated after Cocoa startup because early registration alone did not deliver Open With events reliably. Requests wake the existing UI and use its document/close paths.
5. **Use typed native services.** `platform/macos.rs` owns asynchronous NSOpenPanel selection, native appearance and Finder reveal. `fonts.rs` uses CoreText-backed font-kit on macOS. Keep AppKit thread/ownership constraints explicit.
6. **Keep data handling safe.** `platform.rs`, `paths.rs` and `vaults.rs` handle OS directories and canonical paths. `operations.rs` uses `renamex_np(RENAME_EXCL)` on macOS, preserving occupied destinations and source checks. Canonical-path handling accounts for `/var` versus `/private/var`.
7. **Apply the visual theme through shared styling.** `palette.rs` supplies the macOS-default Obsidian-inspired palette. `theme.rs` applies light/dark styles. Terminal ANSI colors are adjusted when necessary to maintain contrast on each palette.
8. **Package and verify the real artifact.** `package-macos.py` creates/signs the bundle and ZIP. `verify-macos.py` launches isolated profiles, opens a synthetic document through Finder and checks the native report plus rendered content. `macos_qa.rs` is opt-in test instrumentation, not a user workflow.

## Validation

From the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --release --all-targets --no-fail-fast
cargo build --locked --release --bin rustidian
python3 scripts/package-macos.py --binary target/release/rustidian --output dist/Rustidian.app
python3 scripts/verify-macos.py --app dist/Rustidian.app --output reports/macos
```

Use a new package output path or remove only a disposable output created by the current task. With `CARGO_TARGET_DIR` set, use its release binary path. Keep Rust toolchains and build products outside iCloud; delayed file hydration was observed during development.

Relevant regressions include `tests/macos_behavior.rs`, operation/folder-move/file-move tests, attachment/opening tests, macOS atomic-rename unit tests, editor input tests and native menu input tests. Existing tests were made portable by comparing canonical paths; containment checks were not weakened. The FreeDesktop Trash-layout fixture remains Linux-only.

## Release and installation boundaries

- Generic `.app`: starts a writable demo or a user-selected vault. It does not contain the original user's private protection profile.
- Existing personal test installation: a separate launcher and sandbox profile use a verified local copy, block access to the original vault and protect the backup. These machine-specific artifacts are outside this repository.
- Never equate building a generic app with safely updating that personal installation. Inspect the existing bundle/profile and verify copied data before replacing it.
- Preserve MIT attribution and bundled asset notices. Do not embed proprietary system fonts or private notes in release artifacts.
- A macOS 12 minimum in bundle metadata is a deployment setting, not proof of testing on macOS 12. Gatekeeper-trusted distribution needs a Developer ID/notarization step.

## Future changes

For a new feature, add `specs/002-<feature>/spec.md`, `plan.md`, and `tasks.md`. Start with observable user behavior and acceptance criteria, then identify the smallest code change and meaningful verification. Do not create a second editor/save system or a large planning framework just to use SDD.
