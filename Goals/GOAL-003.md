
# VAM Goal 003 — Diagnostics & Operational Logging Foundation

**Goal ID:** 003
**Goal Name:** `diagnostics-logging-foundation`
**Branch:** `goal/003-diagnostics-logging-foundation`
**Status:** Planned
**Depends On:** Goal 001 — VAM Application Foundation; Goal 002 — Error & Result Model

---

## 1. Objective

Establish the smallest coherent diagnostics and operational logging foundation for VAM.

The goal is to make VAM able to explain what it is doing, diagnose controlled failures, and provide useful operational information without prematurely designing the full VAM diagnostics subsystem, audit system, telemetry system, or package runtime.

This goal must strengthen the existing logging boundary without expanding VAM into unrelated system-management features.

---

## 2. Why This Goal Exists

VAM will eventually manage operations such as:

* package installation and removal
* package execution
* configuration changes
* system diagnostics
* repository operations
* privileged operations

Those operations need a consistent way to report:

* what operation was attempted
* whether it succeeded or failed
* what category of failure occurred
* useful diagnostic context
* information suitable for future TUI/CLI presentation

Goal 002 established the error/result model.

Goal 003 establishes the operational diagnostics boundary that can consume that model.

---

## 3. Scope

### In Scope

* Review the existing `src/log.rs`.
* Define the current logging responsibility.
* Define the smallest useful log levels/categories required by the current application.
* Establish consistent internal diagnostic messages.
* Ensure errors can be logged without exposing secrets or unnecessary sensitive data.
* Define the relationship between `Error`, `Result`, and operational logging.
* Integrate the current CLI/application path where appropriate.
* Add tests for the logging/diagnostic contract.
* Document the current logging boundary in `FUNDATION.md`.
* Remove duplicated or contradictory logging behavior if discovered.

### Out of Scope

Do NOT implement:

* package logging
* package runtime logging
* `.vampkg` logging
* security audit architecture
* telemetry
* analytics
* remote logging
* databases
* journald integration
* syslog integration
* log aggregation
* GUI/TUI log viewer
* repository/network logging architecture
* privilege broker
* sandboxing
* configuration DSL changes
* package lifecycle
* package installation
* system-wide diagnostics commands
* speculative logging frameworks
* external dependencies unless explicitly approved

If implementation requires one of these, STOP.

---

## 4. Architectural Requirements

### 4.1 Logging Has a Boundary

The logging module owns operational diagnostic recording.

Other modules may produce diagnostic events through its public interface, but they must not duplicate logging infrastructure.

### 4.2 Errors and Logs Are Different

An `Error` represents a failure that must propagate through the application.

A log/diagnostic event records useful operational context.

Do not make logging a second error-propagation mechanism.

### 4.3 No Hidden Global Complexity

Do not introduce a large global logger, event bus, database, singleton registry, or dependency-injection framework merely to support this goal.

Prefer the smallest architecture that is correct for the current application.

### 4.4 Security

Never log:

* passwords
* credentials
* authentication tokens
* API keys
* private keys
* secrets
* complete environment variables
* sensitive configuration values

Avoid logging raw user input when it provides no diagnostic value.

Avoid unnecessary absolute filesystem paths.

### 4.5 Current Simplicity

The current VAM application is small.

Do not design the logging system as though VAM already has hundreds of packages and thousands of operations.

The architecture must be extensible, but current implementation should remain minimal.

---

## 5. Required Architectural Assessment

Before implementation, inspect:

* `AGENTS.md`
* `README.md`
* `Architecture.md`
* `Security.md`
* `CLI.md`
* `TUI.md`
* `Package.md`
* `Repository.md`
* `Configuration.md`
* `FUNDATION.md`
* `Goals/GOAL-001.md`
* `Goals/GOAL-002.md`
* all current files under `src/`
* the `development-discipline` skill

Then determine:

1. What exactly does `src/log.rs` currently own?
2. What logging behavior already exists?
3. What is the minimum useful diagnostic contract?
4. Which log levels/categories are actually justified by current code?
5. Does the current design need structured events, or would that be premature?
6. Does logging require any new dependency?
7. Does logging need persistent storage at this stage?
8. How should errors and diagnostics interact?
9. Does any proposed change alter an architectural boundary?

If an unresolved architectural decision is required, STOP.

Use:

```text
ARCHITECTURAL STOP

Problem:
<what was detected>

Risk:
<why continuing could create technical debt, coupling, security risk, or architectural drift>

Safer approach:
<recommended approach>

Decision required:
<exact decision required from developer>
```

Do not invent the decision.

---

## 6. Implementation Principles

Follow:

```text
SPEC
→ DESIGN
→ ARCHITECTURAL ASSESSMENT
→ IMPLEMENT
→ COMPILE
→ TEST
→ REVIEW
→ CLEANUP
→ COMMIT
```

Rules:

* Work only on `goal/003-diagnostics-logging-foundation`.
* Do not modify `main`.
* Do not redesign unrelated modules.
* Do not add dependencies without approval.
* Do not create speculative abstractions.
* Do not duplicate error handling.
* Do not silently change existing public behavior unless required by this goal.
* Keep CLI behavior compatible with Goal 002.
* Preserve the existing `vam`, `vam help`, and `vam version` behavior.

---

## 7. Error / Result Integration

Use the Goal 002 error model.

Current taxonomy:

```text
Usage
Internal
```

Logging must not create a competing error taxonomy.

If logging itself can fail, determine whether that failure should:

* propagate through `Result`
* degrade gracefully
* be handled internally

Do not assume the answer if the existing architecture does not define it.

---

## 8. Testing

Add focused tests for the actual implemented contract.

At minimum consider:

* log message creation
* log level/category behavior
* diagnostic formatting
* error + diagnostic interaction
* sensitive-value exclusion where applicable
* stable output where output is part of the public contract

Do not add tests for functionality that does not exist.

All existing tests must continue to pass.

---

## 9. Verification

Before completion:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo run -- version
cargo run -- help
cargo run
```

Also inspect:

```bash
git status
git diff
git diff --check
```

No unrelated changes may remain.

---

## 10. Documentation

Update `FUNDATION.md` with the finalized current logging/diagnostics contract.

Documentation must describe implemented behavior only.

Do not document future package/runtime/audit behavior as though it already exists.

---

## 11. Git Requirements

Branch:

```text
goal/003-diagnostics-logging-foundation
```

Commit format:

```text
goal/003-diagnostics-logging-foundation: <subject>
```

Do not:

* force push
* rewrite history
* modify unrelated commits
* bundle unrelated documentation changes
* create Goal 004 work in this branch

---

## 12. Definition of Done

Goal 003 is complete only when:

* [ ] Existing logging implementation has been reviewed.
* [ ] Logging ownership is explicit.
* [ ] Minimum diagnostic contract is defined.
* [ ] Error/Result interaction is coherent.
* [ ] No sensitive information is exposed.
* [ ] No unnecessary dependency was introduced.
* [ ] Tests cover the implemented contract.
* [ ] Existing tests pass.
* [ ] `cargo fmt --check` passes.
* [ ] `cargo check` passes.
* [ ] `cargo test` passes.
* [ ] Clippy passes with `-D warnings`.
* [ ] CLI smoke tests pass.
* [ ] `FUNDATION.md` documents the implemented behavior.
* [ ] Git diff contains only Goal 003 work.
* [ ] Commit follows the VAM commit convention.

---

## 13. Final Report

At completion, report:

### Status

PASS / STOP / FAIL

### Architecture

* logging ownership
* diagnostic contract
* relationship with Error/Result
* any architectural decisions made

### Implementation

* files changed
* major changes
* dependencies added, if any

### Tests

* test count
* new tests
* failures, if any

### Verification

* fmt
* check
* test
* clippy
* CLI smoke tests

### Security

* sensitive-data handling
* relevant risks checked

### Git

* branch
* commit
* working-tree status

### Concerns

List only real concerns.

### Next

Stop after Goal 003.

Do not begin Goal 004 automatically.
