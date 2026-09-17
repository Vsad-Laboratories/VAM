# VAM Goal 011 — TUI Foundation

## Status

Planned

## Branch

`goal/011-tui-foundation`

## Depends On

Goals 001–010

---

# 1. Objective

Establish the foundational Terminal User Interface (TUI) for VAM.

Goal 011 introduces the first real interactive VAM terminal interface.

The TUI must remain:

* terminal-native
* minimal
* professional
* responsive
* keyboard-first
* mouse-capable
* visually alive
* lightweight
* fast to launch
* strongly integrated with the existing VAM command architecture

The TUI is not intended to imitate a desktop GUI.

It is a sophisticated terminal interface for operating VAM.

The core experience is:

```text
                         VAM ASCII

────────────────────────────────────────────────────────────

                    VAM TERMINAL OUTPUT


Vam:> █
```

The interface must prioritize the command prompt and useful VAM output over decorative UI elements.

---

# 2. Architectural Boundary

```text
                         VAM
                          │
             ┌────────────┴────────────┐
             │                         │
            CLI                       TUI
             │                         │
             └────────────┬────────────┘
                          │
                     VAM Command
                       System
                          │
                    VAM Core APIs
```

The TUI is a presentation and interaction layer.

It must consume existing VAM functionality rather than reimplementing VAM core behavior.

The TUI owns:

* terminal initialization
* terminal restoration
* rendering
* layout
* input handling
* command-line editing
* visual command feedback
* TUI animation state
* responsive layout decisions
* TUI-specific interaction state

The TUI does NOT own:

* package installation logic
* package execution logic
* package validation logic
* runtime policy
* package format logic
* repository logic
* security policy
* authentication architecture
* business/domain logic

---

# 3. Core UX Principle

The TUI must feel like:

> **A serious VAM control console, not a GUI application.**

The interface should communicate:

* precision
* control
* technical quality
* speed
* professionalism
* VAM identity

Visual complexity must not interfere with usability.

The TUI should be visually impressive through:

* typography
* spacing
* motion
* transitions
* structured output
* subtle status feedback
* clean geometry

rather than through excessive panels, menus, widgets, or decorative elements.

---

# 4. Primary Interface

The initial VAM TUI must consist primarily of:

```text
┌────────────────────────────────────────────────────────────┐
│                                                            │
│                         VAM ASCII                           │
│                                                            │
│                                                            │
│                                                            │
│                    VAM TERMINAL OUTPUT                      │
│                                                            │
│                                                            │
│                                                            │
│ Vam:> █                                                    │
└────────────────────────────────────────────────────────────┘
```

The exact geometry is implementation-defined but must preserve the conceptual hierarchy:

```text
        Branding
           ↓
     Terminal output
           ↓
      Command prompt
```

There must NOT be a permanent:

* sidebar
* tab bar
* dashboard
* application menu
* package browser
* multi-panel desktop layout

unless a later goal explicitly introduces one.

---

# 5. In Scope

## 5.1 TUI Runtime

Implement the minimum runtime required to:

* initialize the terminal
* enter TUI/raw interaction mode where required
* render VAM
* process keyboard input
* process mouse input
* restore the terminal cleanly
* handle terminal resize events
* exit without leaving the terminal in a broken state

Terminal restoration must occur even when the TUI exits through an error path.

---

## 5.2 TUI Rendering

Establish the base rendering architecture.

The renderer must support:

* full-screen terminal rendering
* responsive dimensions
* text regions
* borders
* rounded visual geometry where supported
* command prompt rendering
* output rendering
* status rendering
* animation frames

Avoid creating a large generic widget abstraction system.

Use concrete components until repeated behavior proves an abstraction necessary.

---

## 5.3 VAM Branding

The TUI must include the existing VAM ASCII identity.

Conceptually:

```text
██╗     ██╗ █████╗ ███╗   ███╗
╚██╗   ██╔╝██╔══██╗████╗ ████║
 ╚██╗ ██╔╝ ███████║██╔████╔██║
  ╚████╔╝  ██╔══██║██║╚██╔╝██║
   ╚██╔╝   ██║  ██║██║ ╚═╝ ██║
    ╚═╝    ╚═╝  ╚═╝╚═╝     ╚═╝

VSAD Arch Manager
```

The branding must remain visually restrained.

Branding should not consume excessive terminal space.

The TUI must remain useful at smaller terminal dimensions.

---

# 6. Command Prompt

The command prompt is the central interaction mechanism.

The prompt must use:

```text
Vam:>
```

Example:

```text
Vam:> version
Vam:> list
Vam:> doctor
Vam:> info foo
Vam:> install foo
```

The TUI command prompt must only accept VAM commands.

It must NOT become a general shell.

The TUI must never execute arbitrary shell input merely because it was typed into the VAM prompt.

---

# 7. Dynamic Input Limiter

The command prompt must implement the VAM input boundary concept.

Conceptually:

```text
Vam:> version                    ║
```

As text grows:

```text
Vam:> package                    ║
```

Eventually:

```text
Vam:> sinfisnfisnfisnfisnfisnfisnfisnfisnfisnfisnfisnfisnfisn ║
```

When the maximum input boundary is reached:

* additional input must be rejected
* existing input must remain intact
* the cursor must remain visually meaningful
* the interface must not overflow its intended region

The limiter represents both:

1. visual cursor boundary
2. maximum text-input boundary

The maximum input boundary must be calculated from the current terminal dimensions.

---

# 8. Command Editing

The command input layer must support, where practical within the existing architecture:

* text insertion
* deletion
* cursor movement
* Home
* End
* Backspace
* Delete
* command submission
* command history
* command recall

Command history must remain lightweight and local to the TUI session unless persistent history is explicitly introduced by a later goal.

---

# 9. Command Intelligence

The TUI command prompt should provide the foundation for:

* autocomplete
* suggestions
* command validation
* syntax highlighting
* tab completion
* fuzzy matching
* aliases

However, Goal 011 must NOT create speculative command-intelligence architecture.

Only implement functionality that can be cleanly integrated with the existing CLI/command model.

Do not duplicate command definitions between CLI and TUI.

The TUI should consume a shared command representation or existing command parsing where appropriate.

---

# 10. TAB Behavior

TAB must not perform an arbitrary action.

If TAB completion is not explicitly implemented in the current scope, pressing TAB must have no destructive or surprising behavior.

The interface must never accidentally insert unrelated characters or trigger unrelated operations.

---

# 11. Keyboard Input

The TUI must be keyboard-first.

At minimum, establish handling for:

```text
↑ ↓       command history
← →       cursor movement
Home      beginning of input
End       end of input
Enter     execute command
Backspace delete character
Delete    delete character
Esc       context-appropriate cancellation
Ctrl+C    cancel current operation where appropriate
```

The command prompt visibility toggle is:

```text
SHIFT + *
```

The exact key-event representation must follow the selected terminal/input library.

---

# 12. Mouse Support

The TUI should support mouse interaction where practical.

Initial support may include:

* selecting/clicking interactive elements
* scrolling output
* interacting with supported controls

Mouse support must not compromise keyboard-first operation.

Every important operation must remain usable without a mouse.

---

# 13. Output Region

VAM command output must appear in a structured terminal-style output region.

Example:

```text
Vam:> doctor

Running VAM diagnostics...

✓ Core
✓ Runtime
✓ Package database
✓ Configuration

Diagnostics completed successfully.

Vam:>
```

Output must remain readable and spatially separated from the command prompt.

The implementation should avoid excessive decorative containers around ordinary command output.

---

# 14. Operation Feedback

Long-running VAM operations must have visual feedback.

Example:

```text
INSTALLING

[██████████░░░░░]
```

The rendering system must establish the foundation for:

* progress indicators
* activity indicators
* state transitions
* animated status
* operation completion
* operation failure

Animations must communicate state rather than exist purely for decoration.

---

# 15. Animation System

Goal 011 establishes a minimal animation mechanism.

The animation architecture must support:

```text
Idle
  ↓
Activity
  ↓
Progress
  ↓
Completion
```

Example:

```text
Installing
[██████░░░░░░░░]

Installing.
Installing..
Installing...

Package installed successfully.
```

Animation timing must not block command processing.

Do NOT implement a complex animation framework.

Prefer a small deterministic animation/update mechanism.

---

# 16. Performance Requirements

VAM must launch extremely quickly.

The initial TUI startup path should target:

```text
VAM command
    ↓
initialization
    ↓
TUI ready
```

with the visible readiness sequence occurring within approximately:

```text
< 1 second
```

The interface must not use unnecessary startup animations.

The TUI should feel immediate.

Rendering and animation must not unnecessarily consume CPU.

---

# 17. Startup Sequence

When the user runs:

```text
vam tui
```

VAM may briefly display technical readiness information.

Conceptually:

```text
Initializing VAM...
Loading core...
Loading TUI...
Preparing terminal...
Ready.
```

This sequence must be extremely short.

Target:

```text
≈ 0.9 seconds maximum
```

under normal conditions.

No cinematic startup animation is required.

After initialization, the primary TUI should appear immediately.

---

# 18. Responsive Terminal Layout

The TUI must dynamically adapt to terminal dimensions in real time.

Conceptually:

```text
Large terminal
──────────────────────────────────────────────
                 VAM ASCII

             terminal output

Vam:> █
──────────────────────────────────────────────
```

and:

```text
Small terminal
────────────────────────
        VAM

    output

Vam:> █
────────────────────────
```

The renderer must react to:

* terminal width changes
* terminal height changes

without restarting VAM.

The layout must avoid:

* text overflow
* broken borders
* inaccessible controls
* corrupted rendering
* command prompt exceeding the available width

---

# 19. Visual Language

The initial VAM TUI visual language is:

### Background

Pure black.

```text
#000000
```

### Primary text

White/light neutral text.

### Accent

Purple.

Purple may be used for:

* selection
* focus
* hover
* branding
* animation
* active state

### Status

Use restrained semantic status colors where appropriate:

```text
Success  → green
Warning  → yellow
Error    → red
Normal   → white/light gray
Accent   → purple
```

Do not turn the entire interface into a rainbow.

---

# 20. Typography

The intended VAM TUI font is:

```text
Maple Mono
```

The TUI must not make assumptions that a specific font is installed.

Font selection is controlled by the user's terminal environment.

The TUI itself should simply render correctly using terminal-provided typography.

---

# 21. Geometry

The visual system should use:

* clean spacing
* breathing room
* rounded borders where appropriate
* restrained separators
* consistent alignment

Avoid:

* excessive boxes
* unnecessary nested panels
* clutter
* decorative ASCII everywhere
* information overload

The interface should feel spacious while remaining information-dense when useful.

---

# 22. Dynamic System Information

System information may become visible when meaningful thresholds are crossed.

Example:

```text
RAM 82%
```

The current conceptual threshold is:

```text
78%
```

Information may appear when a monitored value reaches or exceeds its threshold and disappear when the value falls below it.

Goal 011 should only establish the rendering mechanism required for this behavior.

Full system telemetry architecture is outside this goal.

Telemetry must remain disabled by default.

---

# 23. TUI Error Presentation

Errors should be presented clearly.

Example:

```text
┌─ ERROR ──────────────────────────┐
│                                  │
│ Package validation failed.       │
│                                  │
│ Invalid package path.            │
│                                  │
└──────────────────────────────────┘
```

Errors must use the existing VAM:

```text
Error
ErrorKind
Result<T>
```

architecture.

The TUI must translate existing errors into appropriate presentation.

It must not create a separate error model.

---

# 24. Success Presentation

Successful operations should use concise feedback.

Example:

```text
Package foo installed successfully.
```

Do not turn ordinary success messages into excessive animations or dialogs.

---

# 25. Terminal Safety

The TUI must:

* restore terminal state on exit
* restore terminal state on unexpected failure where practical
* avoid leaving raw mode enabled
* avoid corrupting terminal output
* avoid leaking control sequences into normal shell usage
* avoid executing arbitrary shell commands
* avoid privilege escalation

TUI cleanup is a correctness requirement.

---

# 26. Dependency Policy

Use existing project dependencies whenever possible.

Ratatui may be used as the primary TUI rendering framework if already approved/selected by the project architecture.

Terminal/input dependencies must be assessed before introduction.

Do not silently introduce multiple overlapping TUI frameworks.

If a new dependency is required:

```text
Dependency:
Purpose:
Why existing dependencies are insufficient:
Maintenance/security considerations:
Architectural impact:
```

Do not add dependencies merely for visual effects.

---

# 27. API Design

Before implementation, determine the smallest useful TUI API.

Prefer concrete components such as:

```text
Tui
TuiState
CommandInput
Renderer
```

only where they represent actual responsibilities.

Do not create speculative abstractions such as:

```text
GenericWidgetEngine
UniversalLayoutManager
PluginUI
ThemeFramework
AnimationFramework
ViewRegistry
ScreenManager
```

unless the implementation genuinely requires them.

---

# 28. Architectural Diagram

The intended architecture is:

```text
                    ┌─────────────────┐
                    │   VAM CLI       │
                    └────────┬────────┘
                             │
                             │
                    ┌────────▼────────┐
                    │   TUI Layer     │
                    │                 │
                    │  Input          │
                    │  Rendering      │
                    │  Layout         │
                    │  Animation      │
                    │  Presentation   │
                    └────────┬────────┘
                             │
                             │
                    ┌────────▼────────┐
                    │ VAM Command     │
                    │ Interface       │
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │    VAM Core     │
                    └─────────────────┘
```

The critical dependency direction is:

```text
TUI
 ↓
VAM command/core APIs
```

not:

```text
Core
 ↓
TUI
```

Core functionality must not become dependent on terminal presentation.

---

# 29. Explicitly Out of Scope

Goal 011 must NOT implement:

* sidebar
* tab system
* dashboard architecture
* multi-workspace system
* persistent regions
* package browser GUI
* repository browser
* package store
* graphical package manager
* arbitrary shell terminal
* shell command execution
* sound system
* telemetry backend
* authentication/password system
* sudo replacement
* persistent application database
* package execution changes
* package format changes
* package installation changes
* repository functionality
* network functionality
* plugin UI system
* theme plugin architecture
* desktop GUI functionality

These may be considered in future goals only if the architecture requires them.

---

# 30. Testing Requirements

Add focused tests covering the TUI logic that can be tested without requiring an interactive terminal.

At minimum test:

## Command Input

* empty input
* normal input
* insertion
* deletion
* cursor movement
* Home
* End
* maximum input boundary
* input rejection after boundary

## Command History

* empty history
* command insertion
* previous command
* next command
* history boundaries

## Layout

* normal terminal dimensions
* narrow terminal
* short terminal
* terminal resize
* command prompt boundary calculation

## Animation

* initial state
* frame progression
* completion
* reset
* non-blocking state progression

## Error Presentation

* Usage error rendering
* Internal error rendering
* multiline errors

## Terminal Lifecycle

Where practical, test initialization/cleanup logic without requiring a real interactive terminal.

Tests must remain deterministic.

---

# 31. Verification

Run:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release

cargo run -- version
cargo run -- help
cargo run -- tui

git diff --check
git status
```

Perform a manual interactive TUI verification for:

```text
- startup
- command input
- command execution
- cursor movement
- history
- resize
- mouse input
- prompt toggle
- maximum input boundary
- animation
- error presentation
- clean exit
```

Review the final diff for:

```text
scope creep
duplicate command logic
unnecessary dependencies
generic abstractions
shell execution
terminal cleanup problems
layout overflow
blocking animation logic
documentation claiming unimplemented behavior
```

---

# 32. Definition of Done

Goal 011 is complete when:

* VAM can launch the TUI
* the TUI runs correctly inside a terminal
* the terminal is restored correctly on exit
* the TUI has the VAM ASCII branding
* the TUI has the Vam:> command prompt
* VAM commands can be entered and processed
* the TUI does not act as a general shell
* command editing works
* command history works
* the dynamic input limiter works
* terminal resizing works
* keyboard input works
* mouse support works where implemented
* basic animations work
* operation feedback can be rendered
* errors use the existing VAM error architecture
* the visual system follows the defined VAM style
* startup remains fast
* no sidebar exists
* no tab system exists
* no dashboard architecture exists
* no unnecessary abstractions were introduced
* no unjustified dependencies were introduced
* existing tests remain passing
* new focused tests pass
* fmt/check/test/clippy pass
* documentation reflects only implemented behavior
* final git diff is scoped and clean
* implementation is committed using the required convention

---

# 33. Git

Branch:

```text
goal/011-tui-foundation
```

Commit convention:

```text
goal/011-tui-foundation: <subject>
```

Do not:

* modify unrelated goals
* rewrite history
* force-push
* mix unrelated architectural changes into Goal 011

---

# 34. Final Audit Report

At completion, report:

```text
GOAL 011 FINAL AUDIT

Status:
Architecture:
TUI Runtime:
Rendering:
Command Input:
Command History:
Input Limiter:
Responsive Layout:
Mouse Support:
Keyboard Support:
Animation:
Error Presentation:
Dependencies:
Tests:
Verification:
Terminal Safety:
Documentation:
Git:
Concerns:
Next:
```

Stop after the final audit.
