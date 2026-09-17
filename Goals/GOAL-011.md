# VAM Goal 011 — TUI Foundation & Application Interface

## Status

Planned

## Branch

`goal/011-tui-foundation`

## Depends On

Goals 001–010

---

# 1. Objective

Establish the technical foundation for the VAM terminal user interface.

This goal introduces the VAM TUI application architecture, terminal handling, keyboard input, application state, rendering boundaries, theming infrastructure, and CLI/TUI integration.

The TUI must become a real VAM interface without taking ownership of VAM Core business logic.

The TUI Concept document is the source of truth for intended TUI workflows, behavior, navigation, screens, actions, and user experience.

Goal 011 must implement the required technical foundation without inventing or changing the intended visual/behavioral design.

---

# 2. Primary Architecture

Use this architectural flow:

```text
Terminal Input
      ↓
Event Handling
      ↓
Application State / Actions
      ↓
VAM Core
      ↓
Application State Update
      ↓
UI Rendering
      ↓
Terminal

The UI must not directly implement package-management business logic.

VAM Core remains responsible for actual operations.

3. TUI Concept Authority

Before implementation, read:

TUI Concept.md

Treat it as the authoritative specification for:

screens
navigation
workflows
user actions
application states
menus
package workflows
diagnostics workflows
visual behavior
keybindings
transitions

Do not invent major TUI workflows that are not specified.

If TUI Concept.md contains an unresolved or contradictory architectural decision:

STOP and report it before implementing the affected behavior.

Do not silently redefine the concept.

4. Technology Stack

Establish the VAM TUI using:

Ratatui
Crossterm
Clap with derive
Serde with derive
Color-eyre
Tokio only if asynchronous/background work is genuinely required

Use Rust stable.

Do not add additional frameworks without architectural justification.

5. Dependency Assessment

Before modifying Cargo.toml:

inspect all existing dependencies
identify which requested dependencies already exist
identify whether existing VAM infrastructure already provides equivalent functionality
determine whether each requested dependency is actually required now
avoid duplicate functionality

Dependency decisions:

Ratatui

Use for terminal UI rendering.

Crossterm

Use for terminal input, terminal mode management, events, and terminal restoration.

Clap

Use for CLI argument and command parsing.

Preserve the existing CLI architecture where practical.

Serde

Use only where serialization/deserialization is actually required by implemented functionality.

Do not introduce speculative serialized models.

Color-eyre

Assess compatibility with the existing VAM Error/ErrorKind/Result architecture before adoption.

Do not replace the existing VAM error model merely to use color-eyre.

If color-eyre would create competing application-wide error systems, STOP and report the architectural conflict.

Tokio

Do NOT add or use Tokio unless the current TUI or VAM architecture has a demonstrated asynchronous/background-task requirement.

6. Project Structure

Establish a clean TUI-oriented structure.

The exact structure must be determined after inspecting the existing source tree.

A candidate structure is:

src/
├── main.rs
├── lib.rs
├── cli.rs
├── config.rs
├── error.rs
├── log.rs
├── system.rs
├── package/
├── runtime/
├── executor.rs
└── tui/
    ├── app.rs
    ├── event.rs
    ├── ui.rs
    ├── theme.rs
    ├── actions.rs
    └── widgets/

Do NOT blindly create every proposed file.

Only create modules that have a real responsibility.

Avoid moving existing modules unless necessary.

7. Separation of Responsibilities
TUI

Owns:

terminal lifecycle
input events
UI state
rendering
navigation
presentation
user interaction mapping
VAM Core

Owns:

package representation
validation
package containers
installation
runtime preparation
execution
system interaction
configuration
future package/repository systems

The TUI must call VAM Core APIs instead of duplicating their logic.

8. Application State

Establish a central application state model.

The state should represent only information required by the implemented TUI.

Avoid a giant global state structure containing speculative future features.

The application state must support:

current screen/view
navigation state
selected item where required
current input mode where required
application lifecycle
quit handling

Additional state must be introduced only when justified by TUI Concept.md or actual implementation requirements.

9. Event System

Establish a clean event-handling boundary.

The event layer must handle at minimum:

keyboard input
terminal resize
quit action
navigation events required by the implemented concept

Do not put business logic directly inside raw terminal-event handling.

Prefer:

Crossterm Event
      ↓
VAM TUI Event
      ↓
Action
      ↓
Application State
10. Rendering

Use Ratatui for rendering.

Rendering responsibilities must remain separate from application/business logic.

Prefer:

App State
    ↓
ui.rs
    ↓
widgets/
    ↓
Ratatui

Do not place package installation, process execution, filesystem mutation, or other business operations inside rendering functions.

11. Theme

Centralize visual constants.

Create a theme boundary for:

colors
borders
typography/style choices
spacing constants where appropriate
status styles

The exact visual design must follow TUI Concept.md.

Do not independently redesign the VAM interface during implementation.

Do not scatter hard-coded styles throughout widgets.

12. Keyboard-First Interaction

The initial TUI must be keyboard-first.

Support the keybindings defined by TUI Concept.md.

At minimum, the architecture must be capable of handling:

Up
Down
Left
Right
Enter
Escape
Quit
Search/input actions where specified

Do not invent additional shortcuts unless necessary.

13. Terminal Lifecycle

The TUI must correctly:

initialize the terminal
enter the required terminal mode
run the application loop
handle terminal resize
restore the terminal on normal exit
restore the terminal on recoverable failure where practical

The user must not be left with a broken terminal state after exiting VAM.

14. CLI/TUI Integration

Preserve the existing CLI behavior.

The intended boundary is:

vam
 ├── version
 ├── help
 ├── existing CLI commands
 └── TUI

Determine the exact no-argument behavior from the existing architecture and TUI Concept.md.

Do not break existing CLI commands merely to introduce the TUI.

15. Setup & Development Environment

Set up all dependencies required by the approved architecture.

Use Cargo as the project dependency manager.

Do not use system-wide manual library installation when Cargo can provide the required Rust dependency.

After dependency setup:

run Cargo dependency resolution
verify Cargo.lock behavior according to repository policy
compile the project
run tests
run clippy
run formatting

Do not commit generated build artifacts.

16. Cargo Cache / Cleanup

Do NOT blindly delete the user's entire Cargo cache.

Cargo caches may contain dependencies required by other projects.

If cleanup is necessary, determine exactly what is being removed.

Safe project-level cleanup may include:

target/

when appropriate.

Do not introduce an automatic "clear all Cargo cache" operation into VAM.

Do not add destructive cleanup commands merely for development convenience.

Document any intentional cleanup procedure rather than silently deleting unrelated development state.

17. Mock / Placeholder Data

The TUI foundation may use controlled mock data only where required to demonstrate rendering architecture.

Mock data must not be confused with real VAM state.

Do not implement fake package installation, fake execution, fake diagnostics, or fake system operations that could later be mistaken for real functionality.

Where a real Core API already exists, prefer integrating the real API.

18. No Premature Feature Expansion

Goal 011 must NOT implement unrelated feature systems.

Do not add:

repository networking
package marketplace
package updates
dependency resolver
telemetry
authentication
plugin marketplace
configuration engine
capability enforcement
sandboxing
background services
database
arbitrary system-management commands

unless explicitly required by the TUI foundation and approved architecture.

19. Testing

Add focused tests for:

application state transitions
action handling
keyboard navigation logic
quit behavior
resize handling where testable
theme/state mapping where meaningful
CLI/TUI routing
existing VAM functionality remaining intact

Avoid tests that depend on a physical interactive terminal when a deterministic unit test can test the same logic.

20. Visual Verification

The implementation must be checked at:

80x24
120x40

Verify:

no rendering panic
no obvious layout overflow
navigation remains usable
terminal resize is handled
important information remains visible

Do not change visual design merely to satisfy personal preference during implementation.

Follow TUI Concept.md.

21. Documentation

Create or update:

docs/ui-spec.md

only if the repository documentation structure supports it.

The document should describe the implemented TUI foundation, including:

architecture
application state
event flow
rendering flow
theme ownership
keybindings actually implemented
terminal lifecycle
screen/view boundaries

Do not use this document to override TUI Concept.md.

If TUI Concept.md already contains these specifications, avoid duplicating the entire document unnecessarily.

22. Security

The TUI must not bypass VAM Core security boundaries.

User input must not directly become:

shell commands
arbitrary process execution
unrestricted filesystem operations
privilege escalation requests

Existing package execution and system boundaries remain authoritative.

The TUI is a presentation/input layer, not a security bypass.

23. Definition of Done

Goal 011 is complete when:

Ratatui/Crossterm TUI infrastructure exists
CLI integration is established without breaking existing behavior
application state is separated from rendering
event handling is separated from business logic
theme ownership is centralized
keyboard-first navigation works
terminal resize is handled
terminal state is restored correctly
TUI Concept.md is respected
no speculative major workflows are invented
required dependencies are justified
unnecessary dependencies are rejected
existing VAM Core functionality remains intact
tests pass
fmt/check/clippy pass
documentation reflects actual implementation
80x24 and 120x40 layouts are verified
final diff is scoped and reviewed
implementation is committed
24. Verification

Run:

cargo fmt --all -- --check

cargo check --all-targets --all-features

cargo test --all-targets --all-features

cargo clippy --all-targets --all-features -- -D warnings

cargo build --release

cargo run -- version

cargo run -- help

cargo run --

Also run:

git diff --check

git status

Review the final diff for:

architecture violations
unnecessary dependencies
duplicated business logic
speculative modules
broken CLI behavior
unsafe input handling
terminal restoration problems
visual design drift
scope creep
25. Git

Branch:

goal/011-tui-foundation

Commit convention:

goal/011-tui-foundation: <subject>

Do not rewrite history.

Do not force-push.

Do not modify unrelated goals.

26. Final Audit

Return:

GOAL 011 FINAL AUDIT

Status:
Architecture:
Dependencies:
TUI structure:
Application state:
Event system:
Rendering:
Theme:
CLI integration:
Terminal lifecycle:
Keyboard navigation:
Resize handling:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
