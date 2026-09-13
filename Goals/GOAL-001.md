# VAM Goal 001 — Foundation

**Goal ID:** `001`
**Goal Name:** `vam-foundation`
**Branch:** `goal/001-vam-foundation`
**Status:** PLANNED
**Authority:** VAM project documentation + `AGENTS.md` + global `development-discipline` skill

---

## 1. Objective

Establish the minimal, professional Rust foundation of VSAD Arch Manager (VAM).

The result must provide a clean executable architecture that can support future VAM subsystems without requiring structural rewrites.

This goal establishes the application spine only.

It does NOT implement the VAM package system, VAM Runtime, repository client, privilege broker, security enforcement system, full TUI, or system-management functionality.

---

## 2. Required Outcome

At completion, VAM must have:

* a valid Rust binary project
* a clean Cargo structure
* a compilable `vam` executable
* a defined application entry point
* a defined CLI boundary
* a defined core/application boundary
* a defined error/result boundary
* a defined configuration boundary
* a defined logging/diagnostic boundary
* a basic application lifecycle
* automated tests for the foundation
* formatting and linting compliance
* documentation describing the implemented foundation boundaries

The architecture must remain intentionally small.

---

## 3. Architectural Principle

The foundation must establish the spine:

```text
OS
 │
 ▼
main
 │
 ▼
CLI boundary
 │
 ▼
Application/Core
 │
 ├── Configuration boundary
 ├── Error boundary
 └── Logging/diagnostic boundary
```

The exact Rust module structure must be derived from the existing VAM architecture documentation.

Do not blindly copy this diagram into a directory structure.

---

## 4. Scope

### IN SCOPE

* Cargo project initialization
* Rust source tree
* executable entry point
* application initialization
* CLI/application separation
* core/application boundary
* error/result architecture
* configuration subsystem boundary
* logging/diagnostic subsystem boundary
* basic lifecycle
* foundation tests
* Rust formatting
* Clippy compliance
* minimal developer documentation
* required Cargo metadata
* appropriate `.gitignore` updates if genuinely required

### OUT OF SCOPE

Do NOT implement:

* `.vampkg` parsing
* `.vampkg` extraction
* package installation
* package removal
* package execution
* VAM Runtime
* package dependency resolution
* repository synchronization
* package signatures
* package trust enforcement
* privilege escalation
* capability enforcement
* sandboxing
* system package management
* system diagnostics
* network management
* storage management
* cleanup tools
* full TUI
* complex CLI commands
* telemetry
* GUI
* daemon/background service
* database
* persistent package registry
* speculative plugin system
* unnecessary abstractions
* unrelated refactoring

If any of these appear necessary, STOP and explain why before proceeding.

---

## 5. Architectural Requirements

### 5.1 Main

`main` must remain a thin process entry point.

It must not become the location for business logic.

### 5.2 CLI

The CLI layer handles:

* command-line argument interpretation
* command dispatch
* user-facing CLI concerns

It must not contain VAM business logic.

### 5.3 Core/Application

The core/application layer coordinates application behavior.

It must not become a god module.

### 5.4 Configuration

Establish only the configuration boundary required by the foundation.

Do not implement the complete VAM configuration system unless it is explicitly required by existing documentation.

### 5.5 Errors

Establish a coherent error model suitable for future subsystem errors.

Do not create a premature giant error hierarchy containing every hypothetical future error.

### 5.6 Logging/Diagnostics

Establish the foundation required for structured application diagnostics.

Do not build the complete audit/security logging system in this goal.

### 5.7 Dependencies

Every dependency must have a concrete justification.

Before adding a dependency, determine whether the Rust standard library or an already-approved dependency can satisfy the requirement.

Do not add dependencies merely for convenience.

---

## 6. Required CLI Behavior

The foundation must provide a minimal executable interface sufficient to prove that the architecture works.

At minimum, establish:

```text
vam
vam help
vam version
```

Exact output formatting should follow the existing CLI documentation where already specified.

Do not implement future commands merely to make the CLI appear complete.

---

## 7. Testing Requirements

The foundation must include automated tests proving:

* application initialization succeeds
* basic CLI dispatch works
* version behavior works
* help behavior works
* error propagation works where applicable
* configuration boundary can initialize safely
* diagnostic/logging boundary can initialize safely

Tests must test behavior and contracts rather than implementation details wherever practical.

---

## 8. Verification Requirements

Before declaring the goal complete, run:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

If a command is not applicable because of an explicitly documented project configuration, explain why.

Do not suppress warnings merely to make CI pass.

Do not disable Clippy rules globally to hide design problems.

---

## 9. Documentation Requirements

Update documentation only where the implementation establishes or changes an actual VAM contract.

Documentation must not describe speculative future behavior as implemented functionality.

If implementation reveals a contradiction in existing documentation:

STOP.

Report the contradiction and request a decision.

---

## 10. Git Requirements

Work only on:

```text
goal/001-vam-foundation
```

Do not modify `main`.

Do not force-push.

Do not rewrite shared history.

Do not make unrelated changes.

Commits must follow the repository's established commit convention:

```text
goal/001-vam-foundation: <subject>
```

Use coherent commits.

Before completion:

```bash
git diff --check
git status
git diff
```

The final working tree must be clean except for intentionally uncommitted artifacts explicitly approved by the developer.

---

## 11. Completion Checklist

The goal is complete only when ALL applicable items are satisfied.

### Project

* [ ] Cargo project exists
* [ ] `vam` binary builds
* [ ] Rust source structure follows approved architecture
* [ ] no unnecessary modules exist
* [ ] no unnecessary dependencies exist

### Architecture

* [ ] `main` is thin
* [ ] CLI boundary exists
* [ ] application/core boundary exists
* [ ] error boundary exists
* [ ] configuration boundary exists
* [ ] logging/diagnostic boundary exists
* [ ] no god module exists
* [ ] no duplicated responsibility exists
* [ ] no speculative architecture exists

### CLI

* [ ] `vam` executes
* [ ] `vam help` works
* [ ] `vam version` works

### Testing

* [ ] foundation tests exist
* [ ] `cargo test` passes
* [ ] `cargo fmt --check` passes
* [ ] `cargo check` passes
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes

### Documentation

* [ ] implemented architecture is documented where necessary
* [ ] no documentation claims unimplemented functionality exists
* [ ] no unresolved contradiction was silently decided

### Git

* [ ] changes are limited to Goal 001
* [ ] diff has been reviewed
* [ ] no accidental files are included
* [ ] coherent commit(s) exist
* [ ] working tree is clean

---

## 12. Stop Conditions

Immediately use the global `ARCHITECTURAL STOP` procedure if:

* an unresolved architecture decision is required
* existing documentation contradicts the implementation
* the goal requires expanding into another subsystem
* a new dependency introduces a meaningful architectural tradeoff
* a proposed abstraction affects future subsystem boundaries
* implementation requires changing the package/runtime/security architecture
* the task cannot be completed within the defined scope
* another module must be redesigned to continue
* a "temporary" architecture is being proposed
* the implementation begins accumulating unrelated cleanup
* the task starts becoming a multi-module refactor

Do not solve these issues silently.

---

## 13. Definition of Done

Goal 001 is complete when:

1. Every applicable checklist item is checked.
2. The foundation compiles cleanly.
3. All foundation tests pass.
4. Formatting passes.
5. Clippy passes without suppressed architectural problems.
6. The implementation matches the existing VAM architecture.
7. No unresolved architectural decision was invented by the agent.
8. The final diff contains only Goal 001 work.
9. The implementation is documented where necessary.
10. The branch is ready for review and CI.

At that point, stop.

Do not begin Goal 002.

---

## 14. Developer Authority

The developer has final authority over unresolved architectural decisions.

The agent is responsible for:

* implementation quality
* verification
* identifying risks
* identifying architectural problems
* proposing safer alternatives
* stopping when a decision is required

The agent must not treat autonomous execution as permission to invent architecture.

---

## 15. Final Report

When the goal reaches completion, report:

```text
GOAL 001 COMPLETE

Implemented:
- ...

Architecture:
- ...

Tests:
- ...

Verification:
- cargo fmt --check: PASS/FAIL
- cargo check: PASS/FAIL
- cargo test: PASS/FAIL
- cargo clippy: PASS/FAIL

Dependencies added:
- ...

Files changed:
- ...

Commits:
- ...

Remaining concerns:
- ...

Next goal:
- NOT STARTED
```

Do not begin another goal automatically.
