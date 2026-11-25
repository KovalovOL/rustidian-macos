# Design

## Context

Source-only cleanup, 2026-11-25: the original root Markdown review below is
historical. `AGENTS.md`, `CONTRIBUTING.md`, `docs/`, standalone tests, CI and
generated skills were removed at the owner's request. Current guidance lives
in README and OpenSpec. Palette logos moved to `assets/brand/`; the macOS
packager and demo remain runtime/build resources. Existing unit-test fixtures
were inlined into Rust source. Recreate any standalone verification tooling
needed by future tasks; do not assume the old verifier or CI still exists.

See [proposal.md](proposal.md) for motivation and scope. This is a planning change, not a new port: the imported project already contains the macOS implementation and `specs/001-native-macos/` as-built records. Its native stack is Rust 2024/1.95+, egui/eframe 0.37 and Glow, with objc2/AppKit, muda, CoreText/font-kit and customized `vendor/egui`/`vendor/epaint`.

### Root Markdown review

Before cleanup, all three root Markdown files were read. Only README is retained as current root guidance.

| File | Finding | Treatment |
| --- | --- | --- |
| `AGENTS.md` | Correct source map, vault-safety rules, native lifecycle constraints and historical evidence; requires Linux preservation and numbered `specs/` for every feature. | Preserve its safety/architecture rules. Owner direction on 2026-11-25 supersedes platform priority: optimize for macOS; retain inherited Linux code without a parity promise. A future documentation task will make OpenSpec the entry point for new changes and keep numbered specs as historical evidence. |
| `README.md` | macOS introduction sits above Linux installation links, Ubuntu/glibc requirements, Ctrl shortcuts and older Linux performance claims. | Make macOS setup and Command shortcuts primary; label inherited Linux material and benchmark context. Do not restate old Linux timings as macOS results. |
| `CONTRIBUTING.md` | Sound behavioral-test, licensing and privacy rules, but Linux bug-report fields, packages and a CI-only-build preference dominate. | Keep the engineering rules; describe macOS/Xcode setup, relevant architecture/OS fields, isolated native GUI checks and local locked verification. |

Referenced context was also reviewed: `specs/001-native-macos/{spec,plan,tasks}.md`, `docs/MACOS.md`, `docs/GUIDE.md`, `docs/PLAN.md`, `docs/RELEASING.md`, `docs/ANSWERS.md`, `Cargo.toml` and the macOS workflow. The older release guide assumes Linux CI packaging; `docs/ANSWERS.md` contains stale personal Linux paths and links. Neither defines this fork's current macOS release process. The old spec reports 394 local tests and a cancelled Apple Silicon CI job; these are historical statements, not newly verified facts or current remote status.

### Implementation observations

| Contract | Existing implementation / checks | Follow-up gap |
| --- | --- | --- |
| Native desktop | `src/main.rs`, `window.rs`, `macos_menu.rs`, `macos_events.rs`, `platform/macos.rs`; `tests/macos_behavior.rs` | Existing tests cover URL round-trips and shortcut labels; real clipboard/Undo/Redo/guarded-quit coverage is still required. Preserve early and post-Cocoa Finder event registration and main-thread AppKit ownership. |
| Workspace / integrity | `src/vault.rs`, `operations.rs`, `autosave.rs`, `paths.rs`, `vaults.rs`, recovery modules and existing operation/editor tests | Preserve the single save pipeline, background autosave worker, external-change/source checks and macOS `renamex_np(RENAME_EXCL)` semantics. Verify behavior, not source strings. |
| Packaging | `scripts/package-macos.py`, `scripts/verify-macos.py`, `src/macos_qa.rs`, `.github/workflows/macos.yml` | Verifier already uses temporary profiles, Finder delivery and light/dark captures. Extend behavior coverage; metadata macOS 12 and a two-architecture matrix are not proof of tested support. |
| Performance | Startup telemetry and inherited `docs/performance/` material | No matched macOS baseline covering all newly specified metrics has been established in this task. |

## Goals / Non-Goals

**Goals:** Map durable behavior contracts to existing implementation, expose concrete verification gaps and provide a small sequence for macOS optimization backed by measurements.

**Non-Goals:** Renderer replacement, a second persistence path, dependency updates, Linux backend removal, automatic original-vault migration, installed-app replacement, full Obsidian parity or publication. Performance limits below are proposed engineering budgets, not claims that the imported code currently satisfies them.

## Decisions

### 1. Introduce OpenSpec alongside the imported baseline

The five capability deltas are new OpenSpec contracts, even where behavior is already implemented. Keep the active change unarchived and implementation tasks unchecked until evidence exists. Preserve `specs/001-native-macos/` and all original root files during this handoff; plan their routing/priorities reconciliation rather than rewriting historical evidence now.

Alternative: copying the numbered spec verbatim into a single OpenSpec file would omit performance acceptance and preserve contradictory platform priorities. Replacing it outright would obscure provenance.

### 2. Keep native integration on the shared Rust editor

Use existing menu action queues, document navigation, close guard and save paths. AppKit work remains on the main thread; callbacks retain required ownership and request UI repaint. Preserve both Finder event registrations until a real Open With regression check justifies a change. Test actual native-to-editor behavior with synthetic content and exact saved-byte checks, extending the current opt-in QA approach.

Alternative: a browser frontend or parallel native editor would increase compatibility and data-safety work without evidence that rendering architecture is the bottleneck.

### 3. Make performance changes follow measurement

Begin with a named imported release build, identified by commit plus dirty-tree/file-manifest identity, compiler, dependency lock and release flags. Build baseline and candidate under the same settings outside iCloud-synced paths. Use deterministic synthetic generators with stable fixture hashes: 100/1,000/10,000 notes, links and attachments, and 100 KiB/1 MiB documents. Keep profile/cache state controlled and record thermal/power conditions; alternate baseline/candidate run order.

Capture at least 30 warmed samples per timing workload after documented warmups. Report median and p95; keep cold-start observations separate without claiming that an unflushed filesystem cache is cold. Startup ends at an observed usable document frame; edit-to-save ends at exact expected bytes; search ends at selectable known results. Existing startup telemetry is diagnostic only.

After background work settles, take three 60-second idle CPU/RSS samples per build. Record baseline repeat-run variation in native units: CPU percentage points and memory bytes. Resource allowance is `1.10 * baseline median + (baseline max - baseline min)`; a zero/noisy baseline does not create a divide-by-zero comparison. Latency gates are median <= 1.10x baseline and p95 <= 1.20x baseline. These baseline-relative budgets avoid inventing absolute speed claims before measurement.

Only optimize a measured bottleneck. An optimization must improve its named target beyond repeat-run noise, stay within other budgets and pass data-safety regressions. Keep same-host results separate from cloud CI timing; hardware differences must not become regressions.

Alternative: adopting the old Linux benchmark as the macOS baseline would compare different hardware, OS, builds and endpoints. Choosing Metal, changing cache policy or dropping safety checks before profiling has no supporting evidence.

### 4. Verify packaged behavior before release claims

Reuse the existing `.app` packager and isolated verifier. Run relevant Rust regressions with `--locked`, then verify the exact optimized packaged executable. Primary validation is Apple Silicon/M4. Intel remains represented by the existing CI matrix, with its status reported separately. Treat the macOS 12 deployment target as unverified until runtime evidence exists; documentation can instead narrow tested-support claims to actual evidence.

Ad-hoc signing stays the development default. Developer ID/notarization is a separate owner-requested distribution task using supplied credentials. Preserve the protected personal installation and never replace its launcher with a generic release bundle as incidental verification.

Alternative: considering compilation, menu presence or internal telemetry alone sufficient would miss input routing, Finder lifecycle, original-data isolation and actual rendering failures.

## Risks / Trade-offs

- Historical and active specs coexist → route future agents through README to OpenSpec and retain explicit provenance in the numbered baseline.
- The pre-cleanup review describes removed documents → use the current README and OpenSpec context for active platform priorities.
- Strict performance gates can be noisy on a laptop → alternate runs, record power/thermal state, retain samples and repeat inconclusive measurements before diagnosing a regression.
- A native QA hook can prove structure without proving editing → drive real menu/input actions and verify exact saved bytes and conflicting versions.
- iCloud hydration can distort startup/build measurements → use a local nonsynced build/test workspace and fully materialized fixtures.
- Compatibility gaps can be mistaken for port defects → publish the supported Markdown workflows and unsupported plugin/Canvas/Bases boundaries explicitly.

## Migration Plan

1. Copy the entire source snapshot, including `.git`, hidden configuration, vendored sources and untracked agent/spec files; verify original relative paths and SHA-256 content. Preserve the source directory.
2. Initialize repo-local OpenSpec and add this proposal, capability deltas, design, tasks and project context. Do not archive or apply during this planning request.
3. On a later implementation request, reconcile agent/contributor/product docs first, then establish the isolated baseline, close native verification gaps and introduce performance gates.
4. Optimize only after a measured bottleneck is identified. Record exact candidate evidence and unresolved architecture/OS limits.

Rollback of planning consists of removing only the newly added OpenSpec setup/generated skills and documents after review; imported source and historical specs require no migration. Future app changes remain separable and must not roll back or delete user notes to restore a binary.
