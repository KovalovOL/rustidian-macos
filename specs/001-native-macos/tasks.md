# Task ledger

Snapshot: 2026-11-25. Checked items describe verified baseline work, not new work requested in the handoff.

## Completed baseline

- [x] T01 Create the independent GitHub fork and publish version 0.2.0 (`5a87795`).
- [x] T02 Enable native AppKit window decorations and remove custom window controls on macOS. (R1)
- [x] T03 Add native menus, Command shortcuts and existing-command/editor routing. (R2)
- [x] T04 Add Obsidian-inspired light/dark palettes and native appearance switching. (R3)
- [x] T05 Add asynchronous native panels, Finder reveal and system font loading. (R4)
- [x] T06 Handle Finder/Open With and system quit requests; verify document delivery after Cocoa startup. (R5)
- [x] T07 Add native application-data paths and atomic no-overwrite renames; fix canonical-path handling. (R6)
- [x] T08 Preserve Linux behavior and pass Linux CI. (R7)
- [x] T09 Add `.app`/ZIP/checksum packaging and the native GUI verifier. (R8)
- [x] T10 Pass 394 local tests, formatting and Clippy on M4; verify native controls, Finder delivery and both themes.
- [x] T11 Verify Intel macOS CI success. See the run linked in `spec.md`.
- [x] T12 Install the protected personal test app, verify its copied data, and move only its test copy outside iCloud.

## Next priorities

- [ ] T13 Obtain a complete Apple Silicon cloud CI run. The first cloud job was cancelled, not failed or passed. Inspect the current Actions state before rerunning; local M4 evidence already exists. Done when the matrix run records an Apple Silicon success and its artifact is verified.
- [ ] T14 Expand end-to-end menu/clipboard coverage: Undo/Redo, Cut/Copy/Paste including images, and unsaved changes during Quit. Use synthetic notes and isolated profiles. Done when the real native-to-editor path is exercised, not just menu presence or source-text assertions.
- [ ] T15 Validate the oldest advertised macOS release, or update the minimum supported version to match evidence. Keep Intel/Apple Silicon and OS-version claims separate.
- [ ] T16 If public trusted distribution is requested, add Developer ID signing/notarization using owner-provided credentials and test a downloaded artifact. Never commit signing secrets or remove OS security checks to make a test pass.

## Definition of done for a new task

- The intended behavior and acceptance criteria are in the relevant spec.
- The implementation preserves original data and platform boundaries.
- Relevant tests/checks pass; UI changes are verified in the rendered app.
- The actual result and remaining limits are recorded here and in the handoff.
- Code, docs and artifacts contain no real vaults, backups, credentials or private screenshots.

The baseline test count is evidence, not a quota. Documentation-only maintenance does not require rerunning the application suite.
