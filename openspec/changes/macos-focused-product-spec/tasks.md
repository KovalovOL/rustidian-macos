# Tasks

These are future implementation and verification tasks. The imported baseline and this planning handoff do not mark them complete. Use the five capability specs for acceptance and `design.md` for the observed source map and measurement protocol.

## 1. Maintain README and specification guidance

- [ ] 1.1 Keep new behavior changes linked from README to OpenSpec, preserve the numbered baseline as historical evidence and retain the macOS focus and data-safety rules; verify all current local document links when updating guidance.
- [ ] 1.2 Update README build/Command-shortcut instructions and compatibility boundaries for the next verified candidate; verify commands against its toolchain and scope any performance claims to measured inputs.

## 2. Establish an isolated baseline and data checks

- [ ] 2.1 Create deterministic synthetic 100/1,000/10,000-note vault and 100 KiB/1 MiB document fixtures with links, attachments, Unicode paths and `.obsidian` sentinel files outside personal data; verify fixture hashes and independent temporary profile roots in a dry run.
- [ ] 2.2 Build a named baseline with Rust 1.95+ and `--locked` in a nonsynced target directory, record commit/dirty-tree identity, toolchain and build flags, and run formatting, Clippy and retained source-local tests and any newly added focused behavioral checks; retain actual command results rather than adopting the old 394-test count.
- [ ] 2.3 Add only missing behavioral regressions for failed saves, external edits, raced occupied destinations, replaced source identities and recovery; verify the expected failure path preserves exact external/source bytes and recoverable local content.
- [ ] 2.4 Verify offline open/edit/link/search/reopen, first-launch demo copying and application-state isolation on disposable fixtures; compare unopened notes, attachments, `.obsidian` sentinels and bundle resources before/after and document supported workflows in README.

## 3. Close native desktop verification gaps

- [ ] 3.1 Create an isolated verifier around the retained opt-in macOS QA path to drive real native menu Undo/Redo and text/image Cut/Copy/Paste through focused editor input; verify exact note/attachment bytes and chronological undo behavior, and document its isolated invocation in README.
- [ ] 3.2 Cover Finder document delivery at launch and while running, special-character paths, panel cancellation, Reveal in Finder, native controls and Control+Tab/Command+Tab behavior; verify selected file identity and rendered state, using manual checks only where automation cannot drive the system behavior and recording that limit.
- [ ] 3.3 Exercise menu Quit, Command+Q and system quit with unsaved content and an external-change/save-failure case; verify no silent local/external data loss and record actual guard/recovery outcomes in the candidate verification report.
- [ ] 3.4 Verify fixed light/dark modes and follow-system changes in the packaged app with synthetic content; inspect rendered captures and native panels for readable appearance and record observed results without claiming new OS coverage from a single host.

## 4. Measure and gate macOS performance

- [ ] 4.1 Add a macOS measurement runner using observable document-frame, exact-save and selectable-search endpoints, with separate internal diagnostics; verify it rejects wrong saved bytes/results and records fixture/build/host/profile/cache identities and raw samples.
- [ ] 4.2 Capture alternating matched baseline runs for every specified workload with documented warmups, at least 30 timing samples and three 60-second settled idle samples; verify median/p95, CPU/RSS units and repeat-run noise are reproducible and document the protocol and machine limits.
- [ ] 4.3 Implement baseline-relative latency and resource checks from `macos-performance` and `design.md`; verify meaningful sample reports just below/above each gate, zero/noisy resource baselines, failed correctness runs and mismatched-host/build inputs produce correct pass/fail or invalid-comparison outcomes.
- [ ] 4.4 Profile a repeatable macOS bottleneck identified by the runner, make the smallest focused optimization and add its relevant regression coverage; verify the named target improves beyond repeat-run noise while all other budgets and data-safety checks pass, and record measured results and limits.

## 5. Verify the packaged candidate and support claims

- [ ] 5.1 Package the exact optimized candidate into a new disposable `.app` path and verify ZIP/checksum, native architecture, required resources, license notices and existing-output refusal; run the extended GUI verifier on that packaged executable and retain build-linked results.
- [ ] 5.2 Run locked Rust checks and packaged GUI verification for the Apple Silicon candidate, then obtain separate Intel verification where available or label it unverified; recreate CI only if requested; document architecture outcomes separately, treating cancelled/not-run jobs as incomplete rather than passed.
- [ ] 5.3 Validate the oldest advertised macOS runtime where available or narrow the tested-support statement to evidence; verify documentation distinguishes runtime coverage from the macOS 12 deployment setting and labels signing/notarization status accurately.
- [ ] 5.4 Complete the cross-capability integration report with links to actual test, GUI, performance and packaging results, remaining limits and a check that original notes/backups/protected installation were untouched; verify every advertised behavior is linked to evidence or explicitly pending.

## Workflow follow-up

- Review all artifacts before starting implementation through `openspec-apply-change`.
- Archive/sync this change only after tracked work is complete and verified; preserve the historical numbered baseline and record any support limitations.
- Public publishing, Developer ID credentials/notarization and replacing the protected personal installation require their own explicitly requested work and are not prerequisites for this development-focused change.
