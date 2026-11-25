# macOS desktop

## Purpose

Provide a coherent native macOS desktop experience for a local Rust notebook, including system window behavior, input and document lifecycle events.

## ADDED Requirements

### Requirement: Native window behavior
The application SHALL expose a system-decorated macOS window with functional traffic lights, resizing, minimization, full screen and system window switching.

#### Scenario: Window controls belong to macOS
- **WHEN** a user opens the packaged application and uses each window control
- **THEN** the system performs the corresponding operation and the document remains usable after restoration

### Requirement: Native menus and editing shortcuts
The application SHALL provide native Rustidian, File, Edit, Format, View, Window and Help menus. Menu and keyboard equivalents SHALL perform the same document actions, including Undo/Redo, text/image clipboard operations and save. Command+Tab SHALL remain system application switching; Control+Tab SHALL navigate document tabs.

#### Scenario: Native input edits the focused document
- **WHEN** a user pastes text or an image, undoes and redoes through the native menu, then saves with Command+S on disposable notes
- **THEN** the focused note and its saved content reflect the same operations as the editor shortcuts without duplicated input or lost history

#### Scenario: Tab navigation respects system shortcuts
- **WHEN** the user presses Control+Tab with two document tabs and Command+Tab with another application available
- **THEN** Control+Tab switches documents and Command+Tab switches applications

### Requirement: Native selection and Finder lifecycle
The application SHALL support native asynchronous file/folder selection, Reveal in Finder and Finder/Open With document delivery both on launch and while running. Cancelling a panel SHALL preserve the current document. Unicode, spaces and URL-special characters SHALL preserve path identity.

#### Scenario: Deliver a file to the running app
- **WHEN** Finder opens a synthetic Markdown file whose path contains Ukrainian text, spaces, a hash and a percent sign
- **THEN** the running app selects that exact file without opening an incorrect path or losing existing unsaved edits

#### Scenario: Cancel a folder selection
- **WHEN** the user cancels the native folder panel
- **THEN** the existing vault and active document remain selected and the UI remains responsive

### Requirement: Appearance follows the selected mode
The application SHALL offer readable Obsidian-inspired light and dark appearances and a follow-system mode. Native panels and notebook content SHALL follow the selected appearance without changing note bytes.

#### Scenario: System appearance changes
- **WHEN** follow-system mode is selected and macOS changes between light and dark
- **THEN** notebook content and subsequent native panels match the system mode with readable text and visible controls

### Requirement: System quit preserves edits
The application SHALL route menu Quit, Command+Q and system quit requests through its unsaved-edit and save-failure protection. A blocked or cancelled quit SHALL leave the document buffer recoverable.

#### Scenario: Quit during a conflicting save
- **WHEN** an external edit prevents saving an unsaved note and the user requests Quit
- **THEN** the application preserves both versions and provides its existing guard/recovery flow rather than silently discarding the local edits
