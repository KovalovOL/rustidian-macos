# macOS performance

## Purpose

Make macOS responsiveness and resource use measurable on representative synthetic vaults, so optimizations can be assessed without weakening correctness or relying on inherited Linux results.

## ADDED Requirements

### Requirement: Reproducible same-host measurements
Every performance result SHALL identify baseline and candidate builds, machine, macOS version, architecture, release settings, fixture identity, profile state and cache conditions. Startup, edit-to-save acknowledgement, search latency, idle CPU and resident memory SHALL be reported separately using the same host and inputs for each pair.

#### Scenario: Compare a candidate on Apple Silicon
- **WHEN** a candidate is compared with the imported baseline on the primary Apple Silicon Mac
- **THEN** the report records both build identities and matching conditions, retains raw samples and separates cold-start observations from warmed measurements

### Requirement: Observable endpoints and correctness
Startup timing SHALL end at an observable usable document frame, editing timing at acknowledgement of exact expected saved bytes and search timing at selectable expected results. Internal startup telemetry alone SHALL NOT establish end-to-end latency or a comparison with another editor.

#### Scenario: A fast run saves incorrect bytes
- **WHEN** a timed edit run produces content different from the expected fixture
- **THEN** the run fails correctness verification and cannot count as a performance success

### Requirement: Representative workloads and regression limits
Verification SHALL cover 100-, 1,000- and 10,000-note synthetic vaults and 100 KiB/1 MiB editing fixtures. A candidate SHALL keep warmed median startup, edit and search latency within 10% of baseline and p95 latency within 20%; median idle CPU and resident memory SHALL remain within 10% plus the baseline repeat-run noise allowance. Performance gains SHALL NOT excuse failed data-safety checks.

#### Scenario: A candidate exceeds the regression budget
- **WHEN** a matched repeatable measurement exceeds a latency or resource limit
- **THEN** the report marks the performance gate failed and identifies the workload and raw samples rather than hiding it in an aggregate improvement

### Requirement: Samples and bounded conclusions
Timing comparisons SHALL retain at least 30 measured samples per build and workload after documented warmups and report median and p95. Idle-resource comparisons SHALL use three 60-second samples after settling. An optimization SHALL identify a reproducible improvement beyond baseline repeat-run noise in its target metric without exceeding other regression limits.

#### Scenario: Publish an optimization claim
- **WHEN** documentation describes a measured improvement
- **THEN** it names the workload, hardware, build pair, sample counts and limitations, and does not extrapolate an older Linux or single-workload result to universal macOS superiority
