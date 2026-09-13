# VAM Goal 002 — Error & Result Model

**Goal ID:** `002`
**Goal Name:** `error-result-model`
**Branch:** `goal/002-error-result-model`
**Status:** PLANNED
**Authority:** VAM project documentation + `AGENTS.md` + global `development-discipline` skill

---

# 1. Objective

Establish a coherent, extensible, and maintainable error and result model for VSAD Arch Manager (VAM).

The error model must provide a stable foundation for future VAM subsystems without prematurely encoding every possible future error.

The result of this goal must allow future modules to:

* return meaningful failures
* preserve useful error context
* propagate failures across module boundaries
* distinguish expected operational failures from programmer/internal failures where appropriate
* provide appropriate user-facing error information
* test failure behavior deterministically

This goal modifies the existing foundation error boundary.

---

# 2. Required Outcome

At completion, VAM must have:

* a clearly defined application error model
* coherent error categories suitable for current VAM functionality
* predictable error propagation
* appropriate ownership of error creation
* stable module-facing error interfaces
* useful user-facing error representation
* tests covering important failure paths
* no duplicated error-handling logic
* no premature hierarchy containing hypothetical future subsystems
* documentation describing the established error contract

The implementation must remain small and standard-library-only unless a dependency is proven necessary.

---

# 3. Current Context

Goal 001 established the initial foundation:

```text
OS
 │
 ▼
main
 │
 ▼
CLI
 │
 ▼
Application/Core
 │
 ├── Error boundary
 ├── Configuration boundary
 └── Logging/diagnostic boundary
```

Goal 002 strengthens the **Error boundary**.

It must not redesign the rest of the foundation.

---

# 4. Scope

## IN SCOPE

* Review the existing error implementation from Goal 001
* Define the VAM application error contract
* Improve the error type where justified
* Establish error categories required by currently implemented functionality
* Establish error propagation conventions
* Establish conversion rules between lower-level errors and VAM errors
* Establish user-facing error rendering/formatting
* Establish appropriate distinction between operational and internal failures
* Add comprehensive foundation error tests
* Document the error model
* Remove unnecessary or duplicated error handling discovered during implementation
* Verify integration with existing CLI/application boundaries

## OUT OF SCOPE

Do NOT implement:

* package-specific error systems
* `.vampkg` errors beyond generic extensibility considerations
* VAM Runtime errors
* repository/network error architecture
* security/privilege error architecture
* configuration DSL implementation
* TUI error presentation framework
* logging/audit redesign
* telemetry
* database errors
* plugin errors
* dependency-resolution errors
* package lifecycle errors
* speculative error categories for future subsystems
* a global exception-like framework
* unnecessary external crates
* unrelated refactoring

If any out-of-scope system appears necessary, STOP and ask.

---

# 5. Architectural Requirements

## 5.1 Error Ownership

Errors should be created at the boundary where the failure is meaningfully understood.

Do not force every module to know about every other module's errors.

Avoid a giant centralized error enumeration containing hypothetical future functionality.

---

## 5.2 Error Propagation

Errors should propagate predictably through:

```text
module
  ↓
application/core
  ↓
CLI
  ↓
user-facing output
```

Lower-level implementation details should not unnecessarily leak through public VAM interfaces.

---

## 5.3 Error Context

Errors should preserve useful diagnostic context where appropriate.

Context must be meaningful.

Do not add verbose or redundant context to every function merely because an error-context mechanism exists.

---

## 5.4 User-Facing Representation

CLI users should receive errors that are:

* understandable
* concise
* actionable where possible
* free from unnecessary internal implementation details

Internal diagnostics may contain additional technical information where appropriate.

Do not expose secrets, credentials, or sensitive system information.

---

## 5.5 Programmer Errors

Do not create a complicated runtime mechanism for every possible programming mistake.

Use Rust's normal type system, invariants, assertions, and testing where appropriate.

Do not convert every programmer defect into a user-facing operational error.

---

## 5.6 Stability

The error boundary should be designed so future subsystems can integrate without requiring a rewrite of the foundation.

However:

> Extensibility must not become speculative abstraction.

Implement only the minimum structure justified by current VAM requirements.

---

# 6. Error Categories

Determine the minimum useful categories based on the existing implementation.

Possible categories may include concepts such as:

```text
Usage
Configuration
IO
Internal
```

but these are **examples, not mandatory final categories**.

The agent must inspect the existing VAM architecture and determine the smallest correct model.

Do not add categories merely because future modules might eventually need them.

If selecting the error taxonomy requires an architectural decision not already determined by existing documentation, use:

```text
ARCHITECTURAL STOP
```

and ask the developer.

---

# 7. Result Model

Evaluate whether VAM requires an explicit project-level Result alias.

If an alias improves consistency without introducing unnecessary abstraction, establish one.

If Rust's native `Result<T, E>` is sufficient and an alias provides no meaningful benefit, do not introduce one merely for naming purposes.

The decision must be based on actual project needs.

Do not create a generic result framework.

---

# 8. CLI Integration

Verify that:

```text
vam
vam help
vam version
```

continue to behave correctly.

Verify that application and CLI failures propagate through the established error boundary.

Where practical, tests should verify both:

1. internal error correctness
2. user-facing output correctness

Do not expand the CLI command set.

---

# 9. Testing Requirements

Add tests covering at minimum:

### Error construction

* [ ] valid error creation
* [ ] correct error category/type
* [ ] useful error message

### Propagation

* [ ] lower-level failure reaches application/core correctly
* [ ] application/core propagates failure correctly
* [ ] CLI handles application failure correctly

### Formatting

* [ ] user-facing representation is stable and useful
* [ ] internal/debug representation remains useful for diagnostics

### Failure paths

* [ ] representative initialization failure
* [ ] representative CLI failure
* [ ] representative configuration-boundary failure where applicable

Tests must verify behavior rather than implementation details whenever practical.

---

# 10. Security Requirements

Error handling must not:

* leak passwords
* leak authentication material
* expose unnecessary environment variables
* expose unnecessary filesystem information
* reveal sensitive configuration
* accidentally print secrets through debug formatting

Review error messages and debug representations for accidental information disclosure.

If secure error handling requires a broader security architecture decision, STOP.

---

# 11. Dependency Requirements

The preferred implementation remains:

```text
Rust standard library
```

Do not add an external dependency merely to provide:

* error enums
* Result aliases
* string formatting
* basic error propagation
* simple context

If an external crate appears materially beneficial, STOP before adding it and explain:

* what it provides
* why std is insufficient
* dependency cost
* security/maintenance implications
* whether it changes the architecture

---

# 12. Documentation Requirements

Update the appropriate VAM documentation to describe the actual implemented error contract.

Documentation must specify:

* error ownership
* propagation behavior
* user-facing representation
* current categories
* conventions future modules should follow

Do not document hypothetical future error categories as implemented functionality.

If existing documentation contradicts the implementation:

```text
ARCHITECTURAL STOP
```

---

# 13. Git Requirements

Work only on:

```text
goal/002-error-result-model
```

Do not modify `main`.

Do not force-push.

Do not rewrite shared history.

Do not make unrelated cleanup changes.

Commit messages must follow the established VAM convention:

```text
goal/002-error-result-model: <subject>
```

Prefer coherent commits such as:

```text
goal/002-error-result-model: refine application error model
goal/002-error-result-model: add error propagation tests
```

Do not create unnecessary commits.

---

# 14. Verification Requirements

Before declaring completion, run:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

Also verify:

```bash
cargo run -- version
cargo run -- help
cargo run
```

Use the exact supported command syntax established by the CLI implementation.

If testing failure paths requires a dedicated test harness or controlled test input, implement the smallest appropriate mechanism.

Do not introduce production functionality solely for testing.

---

# 15. Completion Checklist

## Architecture

* [ ] Existing Goal 001 error implementation reviewed
* [ ] Error ownership clearly defined
* [ ] Error propagation clearly defined
* [ ] Error model remains small
* [ ] No speculative error hierarchy
* [ ] No god error module
* [ ] No duplicated error handling
* [ ] No architecture outside Goal 002 scope introduced

## Error Model

* [ ] Current error categories established
* [ ] Error creation conventions established
* [ ] Error propagation conventions established
* [ ] User-facing representation established
* [ ] Internal/debug representation reviewed
* [ ] Result model decision made
* [ ] No unnecessary abstraction introduced

## CLI/Application

* [ ] CLI still initializes correctly
* [ ] `vam` works
* [ ] `vam help` works
* [ ] `vam version` works
* [ ] representative failure propagation tested

## Security

* [ ] error output reviewed for information leakage
* [ ] no secrets exposed
* [ ] debug/error formatting reviewed

## Tests

* [ ] error construction tests exist
* [ ] propagation tests exist
* [ ] formatting tests exist
* [ ] representative failure-path tests exist
* [ ] all tests pass

## Verification

* [ ] `cargo fmt --check` passes
* [ ] `cargo check` passes
* [ ] `cargo test` passes
* [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes

## Documentation

* [ ] error model documented
* [ ] conventions documented
* [ ] documentation matches implementation
* [ ] no speculative behavior documented as implemented

## Git

* [ ] only Goal 002 work is included
* [ ] final diff reviewed
* [ ] no generated/unrelated files included
* [ ] coherent commit(s) created
* [ ] working tree clean

---

# 16. Stop Conditions

Use the global `ARCHITECTURAL STOP` procedure if:

* the error model requires redesigning the application architecture
* a new dependency is meaningfully required
* existing documentation conflicts with the required implementation
* a future subsystem must be designed to complete this goal
* the error taxonomy becomes substantially larger than current requirements justify
* the implementation requires a new persistence/configuration/security mechanism
* the task begins expanding into logging/audit architecture
* a temporary error architecture is proposed
* the agent is uncertain about a public contract
* unrelated refactoring becomes necessary
* a security issue requires architectural changes

Normal implementation failures are NOT stop conditions.

Compilation errors, failing tests, formatting failures, and Clippy warnings should be investigated and fixed autonomously.

---

# 17. Definition of Done

Goal 002 is complete only when:

1. The existing error boundary has been independently reviewed.
2. A minimal coherent VAM error model is implemented.
3. Error propagation is predictable.
4. User-facing errors are appropriate.
5. Security-sensitive information is not unnecessarily exposed.
6. Tests cover the important current failure paths.
7. All required Rust verification commands pass.
8. No unnecessary dependencies were introduced.
9. Existing VAM architecture remains intact.
10. Documentation matches the implementation.
11. The final Git diff contains only Goal 002 work.
12. All applicable checklist items are satisfied.
13. No unresolved architectural decision has been silently invented.
14. The branch is ready for independent audit/CI.

When these conditions are satisfied:

**STOP.**

Do not begin Goal 003.

---

# 18. Final Report

When complete, report:

```text
GOAL 002 COMPLETE

Implemented:
- ...

Error Model:
- ...

Propagation:
- ...

Security:
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

The agent must not begin Goal 003 automatically.
