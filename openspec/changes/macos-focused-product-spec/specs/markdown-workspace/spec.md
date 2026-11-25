# Markdown workspace

## Purpose

Let users work with ordinary local Markdown folders and familiar linked-note workflows while keeping compatibility and application-state boundaries explicit.

## ADDED Requirements

### Requirement: Local files without account or import
The application SHALL open a user-selected local Markdown folder or file without an account or mandatory network access. Opening a vault SHALL NOT automatically migrate or rewrite unrelated notes, attachments or `.obsidian` configuration.

#### Scenario: Open a copied Obsidian vault offline
- **WHEN** the user opens a disposable Obsidian-format vault copy with network access unavailable
- **THEN** local notes and attachments are usable and unopened content and `.obsidian` files retain their original bytes

### Requirement: Core editing and navigation
The application SHALL retain source, live and reading modes, create/edit/save/reopen, chronological Undo/Redo, tabs, wiki-link navigation, backlinks, quick switching and vault search. Rendering a supported Markdown construct SHALL NOT rewrite the document source.

#### Scenario: Edit and follow a linked note
- **WHEN** the user edits a synthetic note, undoes and redoes the edit, saves, follows a wiki link and reopens the first note
- **THEN** the first note contains the expected saved bytes and navigation selects the intended linked note

#### Scenario: Search and switch
- **WHEN** the user searches for a known phrase and uses the quick switcher in a disposable multi-note vault
- **THEN** selecting a result opens the matching note and its expected content

### Requirement: Application state outside the vault
Application state and recovery SHALL use macOS Application Support by default, honoring explicit supported XDG overrides. First launch SHALL create a writable demo copy outside the installed bundle. Merely opening a vault SHALL NOT insert application-private state files into it.

#### Scenario: First launch with an isolated profile
- **WHEN** the packaged app starts without an existing profile or selected vault
- **THEN** it opens a writable demo copy, leaves bundle resources unchanged and keeps private state outside the vault

### Requirement: Compatibility claims reflect supported workflows
Product and contributor documentation SHALL identify this fork as macOS-focused and distinguish Markdown/vault compatibility from full Obsidian parity. Existing Obsidian community JavaScript plugins SHALL be described as unsupported, with Canvas/Bases gaps explicit. Linux release and feature parity SHALL NOT be promised by this fork.

#### Scenario: A user evaluates existing Obsidian workflows
- **WHEN** the user reads the project overview and macOS setup instructions
- **THEN** they can identify supported note workflows, unsupported community plugins, remaining Canvas/Bases gaps and the macOS support scope before trying a vault copy
