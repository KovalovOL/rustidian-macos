# Vault integrity

## Purpose

Protect local notes and attachments against accidental overwrite, failed writes and conflicting edits throughout saving, recovery and filesystem operations.

## ADDED Requirements

### Requirement: Atomic exact-content persistence
Successful manual saves and autosaves SHALL persist the intended UTF-8 document bytes atomically. Failed saves SHALL preserve the last valid on-disk document and retain the unsaved buffer or recovery data. Opening, reading or rendering a note SHALL NOT normalize its bytes without an explicit edit.

#### Scenario: Save fails without corrupting a note
- **WHEN** an isolated test injects a write or replacement failure during save
- **THEN** the prior document remains intact, the failure is visible and the edited content remains recoverable

### Requirement: External modifications are protected
The application SHALL refuse a save that would silently overwrite a note modified externally since it was loaded. It SHALL preserve the external bytes and local edits and offer the existing save-copy or recovery workflow.

#### Scenario: External editor wins no implicit overwrite
- **WHEN** an external writer changes a loaded note before the local edited buffer is saved
- **THEN** the external file is unchanged by the rejected save and the local version can be preserved separately

### Requirement: Moves and renames never clobber destinations
File and folder operations SHALL enforce atomic no-overwrite destination protection and source-identity checks, including paths with Unicode and canonical path aliases. A failed operation SHALL leave unrelated destination data intact.

#### Scenario: Destination becomes occupied
- **WHEN** a destination is created by another process before a pending rename commits
- **THEN** the rename fails without overwriting that destination or losing the original source

#### Scenario: Source changes before the operation commits
- **WHEN** another process replaces the selected source before a move commits
- **THEN** source-identity protection rejects the stale operation instead of moving the replacement as if it were the selected file

### Requirement: Recovery and recoverable deletion
The application SHALL preserve its existing recovery snapshots and recoverable system Trash workflow. Recovery SHALL let a user preserve unsaved content without silently replacing a conflicting on-disk version.

#### Scenario: Recover after interrupted editing
- **WHEN** a disposable profile contains recovery content after an interrupted editing session
- **THEN** the user can recover that content while any newer external document remains intact until an explicit resolution

### Requirement: Verification isolates personal data
Automated verification SHALL use synthetic notes or a verified disposable copy with isolated profile and recovery directories. It SHALL NOT mutate an original personal vault, its backups or the protected installed test application's launcher/profile.

#### Scenario: Run a native GUI check
- **WHEN** the verifier starts a candidate package
- **THEN** all edits, screenshots, recovery files and cleanup remain confined to the synthetic test workspace and isolated profile
