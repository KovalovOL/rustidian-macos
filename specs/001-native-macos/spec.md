# 001 — Native macOS support

Status: implemented baseline with release follow-up work. Snapshot: 2026-11-25.
Baseline: version `0.2.0`, commit `5a877958a354680c854de1c7dd63115d7881d705`.

## Problem and user goal

The user already uses Obsidian and experiences lag on a MacBook Air M4. Upstream Rustidian primarily targets Linux. The user requested a separate fork with a native macOS appearance, system traffic-light window buttons, a system menu bar and, if feasible, an appearance similar to Obsidian's default light theme.

The existing Obsidian notes, attachments, settings and backups must remain intact. Testing must use a separate copy. The goal is macOS integration, not full Obsidian feature compatibility.

## Requirements

| ID | Requirement |
| --- | --- |
| R1 | Use an actual decorated AppKit window with system-managed corners, shadow, traffic lights, resizing and full-screen behavior. Do not draw substitute window controls on macOS. |
| R2 | Provide native Rustidian, File, Edit, Format, View, Window and Help menus. Route actions through existing application commands and editor input. Use Command shortcuts and keep Control+Tab for document navigation. |
| R3 | Provide an Obsidian-inspired neutral light/dark palette with a purple accent, plus an option to follow system appearance. Apply the selected appearance to AppKit UI as well as egui content. |
| R4 | Provide nonblocking native file/folder selection and Reveal in Finder without Linux desktop utilities. Support system fonts through native macOS font discovery/loading. |
| R5 | Open documents delivered through Finder/Open With after startup. Route system quit requests through the unsaved-edit guard. Preserve Unicode file paths. |
| R6 | Keep application state/recovery outside note vaults, using macOS Application Support by default and honoring explicit XDG overrides. Preserve atomic saves, external-change checks and no-overwrite moves/renames. |
| R7 | Retain the existing Linux implementation and regression coverage. |
| R8 | Produce a self-contained `.app`, ZIP and checksum, with repeatable verification and CI for Apple Silicon and Intel. Distinguish the macOS 12 deployment target from actually tested OS versions. |

## Theme baseline

- Light: white document, `#f6f6f6` sidebar, dark neutral text, purple accent.
- Dark: `#1e1e1e` document, `#262626` sidebar, light neutral text, purple accent.
- The fork retains Rustidian branding. This is an inspired palette, not a bundled proprietary Obsidian theme or plugin runtime.

## Acceptance criteria

1. A packaged app opens its writable demo copy on first launch without editing the installed bundle.
2. Native inspection confirms all three traffic lights, a titled/resizable window, shadow and the native menu structure.
3. Opening a synthetic Markdown file with Finder/Open With selects that file in the running app.
4. Both appearance modes pass the native GUI verifier and produce readable rendered content with the expected backgrounds.
5. Create/edit/save/reopen, Unicode paths, external-change protection and occupied-destination protection pass regression tests on disposable data.
6. Format/lint and the relevant unit/integration tests pass. Platform/CI status is reported accurately; a cancelled job is not a passing job.
7. Updating the user's test installation preserves the original-data protection and keeps tests on a separate vault copy.

## Out of scope

Obsidian community plugin execution; complete Canvas/Bases parity; cloud synchronization or automatic migration of the user's original vault; universal performance superiority; Apple notarization without a maintainer-provided Developer ID.

## Evidence and limits

- M4 / macOS 27.0.1: 394 local tests passed; formatting, Clippy, code-signature validation, native window/menu checks, Finder/Open With and light/dark GUI checks passed on 2026-11-24.
- Linux CI: [successful run](https://github.com/KovalovOL/rustidian-macos/actions/runs/45414590399).
- macOS CI, rechecked 2026-11-25: [Intel succeeded; Apple Silicon job was cancelled](https://github.com/KovalovOL/rustidian-macos/actions/runs/45414590558). Apple Silicon is locally verified on M4; a complete cloud matrix pass is still outstanding.
- Local builds are ad-hoc signed. Notarized public distribution and validation of the oldest advertised macOS release remain follow-up work.
