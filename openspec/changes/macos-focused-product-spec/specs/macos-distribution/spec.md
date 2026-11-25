# macOS distribution

## Purpose

Deliver a self-contained macOS application with reproducible artifact checks and explicit architecture, operating-system and signing support evidence.

## ADDED Requirements

### Requirement: Self-contained release artifacts
Packaging SHALL produce a runnable `.app`, ZIP archive and SHA-256 checksum from an explicitly identified optimized executable. The bundle SHALL include required runtime resources and license notices, contain no private notes or credentials, and run without Linux desktop utilities or a separately installed Rust toolchain.

#### Scenario: Start the packaged app with a fresh profile
- **WHEN** an isolated user profile launches the packaged application without developer tooling in its runtime path
- **THEN** the notebook opens its writable demo and required native workflows work using bundled and macOS-provided resources

### Requirement: Verify the actual candidate artifact
Release verification SHALL exercise the packaged executable, native window/menu behavior, Finder delivery and light/dark rendered content using disposable data. It SHALL additionally verify real menu-to-editor clipboard/Undo/Redo actions and guarded quit. Reports SHALL identify the exact candidate build and distinguish passed, failed, cancelled and not-run checks.

#### Scenario: A CI job is cancelled
- **WHEN** an architecture job is cancelled after another architecture succeeds
- **THEN** the release report records incomplete matrix verification and does not label the full matrix passed

### Requirement: Architecture and operating-system evidence
Apple Silicon SHALL be the primary optimization and verification target. Any advertised Intel or minimum-macOS support SHALL have successful verification for the claimed architecture and OS, or be explicitly labeled unverified. A deployment target in metadata SHALL NOT count as runtime evidence.

#### Scenario: Minimum version is only a packaging setting
- **WHEN** the bundle declares macOS 12 but no check on that OS is available
- **THEN** support documentation distinguishes the deployment target from tested macOS versions and records the outstanding validation

### Requirement: Signing status is explicit
Distribution documentation SHALL distinguish ad-hoc signing from Developer ID signing and notarization. Trusted public distribution SHALL require verified signing/notarization with owner-provided credentials. Credentials SHALL remain outside source and release artifacts.

#### Scenario: Deliver an ad-hoc-signed build
- **WHEN** a local or CI package has only an ad-hoc signature
- **THEN** the report identifies that signing state and makes no notarization or Gatekeeper-trust claim

### Requirement: Installation and packaging avoid destructive replacement
Packaging SHALL reject an occupied output application path. Generic release creation SHALL NOT replace the owner's protected installed test app or its launcher/profile; any separately requested update SHALL preserve original-vault and backup protections.

#### Scenario: Packaging targets an existing app
- **WHEN** the selected output `.app` already exists
- **THEN** packaging stops without modifying it and requires a new disposable output path
