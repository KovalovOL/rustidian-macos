# Proposal

## Why

The owner needs a responsive Rust notebook optimized for macOS and existing Obsidian-format notes. The copied fork already implements a native macOS baseline, but inherited documentation still prioritizes Linux and the existing numbered spec has no OpenSpec capability contracts or measured macOS performance acceptance policy.

## What Changes

- Establish a macOS-focused product contract, with Apple Silicon/MacBook Air M4 as the primary optimization target and explicit, evidence-based Intel/OS support claims.
- Capture existing Markdown, native desktop and data-safety behavior as baseline requirements; specify follow-up verification separately from already implemented code.
- Define reproducible macOS startup, editing, search and idle-resource measurements with a same-host baseline and regression limits.
- Define isolated GUI verification and self-contained macOS release artifacts; carry forward unresolved clipboard/quit, CI and minimum-OS checks.
- Treat Linux as inherited code retained without a feature or release parity commitment. Removing its backend is outside this change.
- Plan reconciliation of root agent/contributor/product documentation. This proposal does not implement app changes or validate old test claims.

## Capabilities

### New Capabilities

- `macos-desktop`: Native windows, menus, keyboard input, appearance, panels and Finder lifecycle integration.
- `markdown-workspace`: Local Markdown vault workflows, linked notes, state isolation and honest Obsidian compatibility boundaries.
- `vault-integrity`: Safe persistence, external-change protection, recovery and guarded filesystem operations.
- `macos-performance`: Repeatable macOS responsiveness measurements, baseline-relative regression gates and bounded performance claims.
- `macos-distribution`: Self-contained application packaging, isolated artifact verification and architecture/OS/signing evidence.

### Modified Capabilities

None. No OpenSpec main capabilities exist yet. The numbered `specs/001-native-macos/` documents remain historical baseline evidence.

## Impact

Planning adds OpenSpec contracts and an implementation/verification backlog. Future work touches root documentation, native integration modules, existing persistence/editor tests, macOS QA/packaging scripts and `.github/workflows/macos.yml`. Keep the Rust/egui architecture, locked dependencies, vendored patches and license notices. No original vault migration, community-plugin runtime, Linux removal, public publishing or installed-app replacement is authorized by this planning change.
