# VAM Goal 004 — System Interface Foundation

**Goal ID:** 004
**Goal Name:** `system-interface-foundation`
**Branch:** `goal/004-system-interface-foundation`
**Status:** Planned
**Depends On:** Goal 001 — Application Foundation; Goal 002 — Error & Result Model; Goal 003 — Diagnostics & Operational Logging Foundation

---

# 1. Objective

Establish the smallest coherent **System Interface** boundary for VAM.

The System Interface is the controlled boundary between VAM's application/core logic and the underlying Linux/Arch operating system.

It exists so future VAM subsystems can interact with the host system through explicit, testable interfaces rather than scattering direct OS operations throughout the codebase.

This goal establishes the foundation only.

It does **not** implement VAM's complete system-management functionality.

---

# 2. Why This Goal Exists

Future VAM functionality will need to interact with the host system for operations such as:

* filesystem inspection
* process inspection
* command execution
* package-manager interaction
* system information
* service inspection
* network inspection
* package installation/runtime operations
* privileged operations

Without a clear boundary, these operations could become scattered throughout:

```text
CLI
TUI
Package Engine
Runtime
Configuration
Diagnostics
```

That would create tight coupling to Linux implementation details and make testing significantly harder.

The intended architecture is:

```text
VAM Application/Core
        │
        ▼
  System Interface
        │
        ▼
     Linux OS
```

The System Interface owns host-system interaction.

---

# 3. Scope

## In Scope

* Inspect the current VAM architecture.
* Establish the initial System Interface module/boundary.
* Define the smallest useful host-system abstraction justified by the current code.
* Establish controlled command/process interaction if required by the current foundation.
* Establish controlled filesystem interaction only where genuinely required.
* Integrate the existing Error/Result model.
* Integrate the existing diagnostics/logging model where appropriate.
* Define ownership and boundaries.
* Add focused tests.
* Ensure the design remains usable for future package/runtime work.
* Document the implemented System Interface contract in `FUNDATION.md` and/or the appropriate architecture documentation.
* Keep the implementation standard-library-first.

## Out of Scope

Do NOT implement:

* `.vampkg`
* package installation
* package extraction
* package runtime
* package execution
* package dependencies
* repository support
* network downloading
* pacman wrapper/subsystem
* privilege broker
* sudo management
* sandboxing
* capability enforcement
* system diagnostics commands
* process manager
* service manager
* network manager
* filesystem manager
* storage manager
* hardware manager
* TUI
* GUI
* telemetry
* database
* asynchronous runtime
* plugin system
* generic command framework
* speculative abstractions
* unnecessary external dependencies

If the implementation requires any of these, STOP.

---

# 4. Architectural Intent

The System Interface is an architectural boundary, not a dumping ground for every Linux operation.

It should eventually allow VAM to express operations such as:

```text
Application/Core
        │
        │ request
        ▼
System Interface
        │
        │ controlled OS operation
        ▼
Linux
        │
        ▼
result
        │
        ▼
System Interface
        │
        ▼
Application/Core
```

The exact API must be derived from actual current requirements.

Do not design a giant interface containing hypothetical future operations.

---

# 5. Ownership

## System Interface Owns

The System Interface owns the mechanics of interacting with the host operating system.

Potential responsibilities may include:

* process spawning
* command execution
* filesystem operations
* OS-level queries
* translating OS failures into VAM errors

Only implement responsibilities actually justified by the current application.

## System Interface Does NOT Own

The System Interface must not own:

* CLI argument parsing
* TUI presentation
* package policy
* package lifecycle
* repository policy
* package trust
* package capabilities
* security policy
* user-facing workflow
* business logic
* package manifests
* package configuration schemas

Those belong to higher-level VAM subsystems.

---

# 6. Architectural Assessment

Before implementation, inspect:

```text
AGENTS.md
README.md
Architecture.md
Security.md
CLI.md
TUI.md
Package.md
Repository.md
Configuration.md
FUNDATION.md

Goals/GOAL-001.md
Goals/GOAL-002.md
Goals/GOAL-003.md
Goals/GOAL-004.md

src/main.rs
src/lib.rs
src/cli.rs
src/error.rs
src/config.rs
src/log.rs
```

Also inspect the development-discipline skill.

Determine:

1. What host-system operations does VAM actually need **right now**?
2. Which of those operations already exist?
3. Which module currently owns them?
4. Is a new `system` module actually justified?
5. What is the smallest useful System Interface?
6. Which operations should remain outside the interface?
7. How should OS failures map into `ErrorKind`?
8. Does the interface need traits, or would concrete functions/types be simpler?
9. Does the interface need dependency injection for testing?
10. Does the interface require an external crate?

Do not answer hypothetical future requirements as though they are current requirements.

---

# 7. Mandatory Architectural Stop

STOP if the design requires an unresolved decision involving:

* privilege escalation
* sandboxing
* capabilities
* security policy
* process isolation
* asynchronous execution architecture
* external system libraries
* external crates
* persistent state
* global system service
* package/runtime integration
* major public API design
* dependency injection architecture
* cross-platform abstraction
* Windows/macOS support
* repository/network architecture

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

Then STOP.

Do not make the architectural decision yourself.

---

# 8. Arch/Linux Scope

VAM targets:

```text
Arch Linux
Arch-based distributions
```

Do not introduce unnecessary cross-platform abstraction.

However, avoid hardcoding implementation details into unrelated modules.

Linux-specific behavior should remain behind the System Interface boundary where appropriate.

Do not assume that every Arch-based distribution has identical configuration.

Do not build distribution detection or compatibility logic unless Goal 004 genuinely requires it.

---

# 9. Process / Command Execution

If command execution is required by the current foundation:

* use controlled process spawning
* avoid shell invocation unless explicitly required
* prefer direct executable + argument invocation
* keep argument boundaries explicit
* capture relevant exit status
* handle stdout/stderr deliberately
* map failures through `Result`
* avoid blindly inheriting uncontrolled environment state where unnecessary
* do not expose sensitive command arguments in diagnostics

Avoid patterns equivalent to:

```text
sh -c "<arbitrary string>"
```

unless there is a documented architectural reason.

Do not create a generic "run anything" API merely for convenience.

---

# 10. Filesystem Interaction

If filesystem access is required:

* use explicit paths
* propagate failures through `Result`
* avoid silently ignoring errors
* do not introduce a global filesystem abstraction unless justified
* do not add a virtual filesystem
* do not implement path sandboxing in this goal
* do not redesign VAM configuration storage

The future VAM Runtime may require significantly stronger filesystem controls.

Goal 004 must not prematurely implement those controls.

---

# 11. Error Integration

Use the existing Goal 002 error model:

```text
ErrorKind
├── Usage
└── Internal

Result<T>
```

System-level failures are application/internal failures unless the existing architecture provides a more precise justified classification.

Do not create a new taxonomy merely because Linux has many error types.

Preserve useful failure context.

Do not leak:

* secrets
* credentials
* tokens
* sensitive environment variables
* unnecessary filesystem information

---

# 12. Diagnostics Integration

Use the Goal 003 diagnostics/logging boundary.

System Interface operations may generate diagnostics when useful.

However:

```text
Error ≠ Log
```

Do not:

1. log an error
2. create another error
3. log it again at every layer
4. return duplicated output to the user

Avoid diagnostic noise.

The System Interface should provide useful operational context to higher layers without becoming responsible for CLI presentation.

---

# 13. API Design

Prefer the smallest API that satisfies actual Goal 004 requirements.

Before creating a public function, type, trait, or abstraction, ask:

```text
Why does this exist?
Where does it belong?
Who owns it?
Who consumes it?
Can it be tested independently?
Does it solve a current problem?
```

Do not create interfaces solely because future packages may need them.

In particular, do not automatically create:

```text
SystemManager
SystemService
SystemContext
SystemProvider
SystemBackend
SystemExecutor
SystemRegistry
```

unless the current architecture demonstrates a concrete need.

---

# 14. Testing Strategy

Tests must verify the actual System Interface contract.

Prefer deterministic tests.

Where OS behavior makes tests environment-dependent:

* isolate the behavior
* test error translation
* test argument construction
* test success/failure handling
* avoid requiring a specific machine configuration where possible

Do not create tests that require:

* network access
* root privileges
* installed third-party software
* a specific desktop environment
* a specific WM
* a specific hardware device

unless absolutely required.

All existing tests must continue passing.

---

# 15. Security Requirements

Review specifically for:

### Command Injection

Arguments must remain separated.

Do not construct shell commands from arbitrary strings.

### Path Handling

Avoid unintended path interpretation.

Do not introduce arbitrary filesystem writes.

### Privileges

Do not introduce privilege escalation.

Do not call `sudo`.

Do not design the privilege broker in this goal.

### Environment

Do not blindly expose sensitive environment data to diagnostics.

### Error Leakage

Ensure OS errors do not accidentally expose sensitive information through user-facing output.

---

# 16. Dependency Discipline

Prefer:

```text
Rust standard library
```

If an external crate appears necessary:

STOP.

Explain:

1. Why the standard library is insufficient.
2. What the dependency provides.
3. What architectural dependency it creates.
4. Whether the dependency is appropriate for VAM's lightweight design.

Wait for developer approval.

---

# 17. Documentation

Document only implemented behavior.

Update the appropriate documentation, most likely:

```text
FUNDATION.md
Architecture.md
```

Only modify `Architecture.md` if Goal 004 genuinely changes the documented architecture.

Document:

* System Interface responsibility
* ownership
* current supported operations
* Error/Result integration
* diagnostics integration
* important limitations

Do not claim that VAM can already:

* install packages
* execute `.vampkg`
* manage services
* manage networking
* perform privileged operations
* sandbox packages

unless those capabilities actually exist.

---

# 18. Verification

Run:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Then:

```bash
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

Review the complete final diff manually.

Check specifically for:

* unnecessary abstractions
* duplicated functionality
* accidental CLI changes
* accidental security architecture
* external dependencies
* unrelated refactors
* excessive code for the current requirement
* documentation drift

---

# 19. Git Requirements

Work ONLY on:

```text
goal/004-system-interface-foundation
```

Do not modify `main`.

Do not:

* force push
* rewrite history
* amend unrelated commits
* reset developer work
* bundle unrelated changes
* implement Goal 005
* create package functionality

Commit format:

```text
goal/004-system-interface-foundation: <subject>
```

Commit only after the complete verification gate passes.

---

# 20. Completion Gate

Goal 004 is complete only when:

* [ ] System Interface responsibility is explicitly defined.
* [ ] Current host-system requirements were identified.
* [ ] The smallest justified interface was implemented.
* [ ] No speculative system-management subsystem was created.
* [ ] Error/Result integration is correct.
* [ ] Diagnostics integration is correct.
* [ ] Command execution, if implemented, does not unnecessarily invoke a shell.
* [ ] No privilege escalation was introduced.
* [ ] No security architecture was prematurely implemented.
* [ ] No unnecessary dependency was introduced.
* [ ] Tests cover the implemented contract.
* [ ] All existing tests pass.
* [ ] Formatting passes.
* [ ] Compilation passes.
* [ ] Clippy passes with `-D warnings`.
* [ ] CLI smoke tests pass.
* [ ] Documentation accurately reflects implementation.
* [ ] Git diff contains only Goal 004 work.
* [ ] Commit follows the VAM convention.

---

# 21. Final Report

After implementation and commit, report:

```text
GOAL 004 FINAL AUDIT

Status:
PASS / STOP / FAIL

Architecture:
- System Interface ownership:
- Current supported operations:
- Error/Result integration:
- Diagnostics integration:
- Architectural decisions:

Implementation:
- files changed:
- major changes:
- dependencies:

Tests:
- total tests:
- new tests:
- failures:

Verification:
- cargo fmt:
- cargo check:
- cargo test:
- cargo clippy:
- cargo run -- version:
- cargo run -- help:
- cargo run:

Security:
- command execution safety:
- path handling:
- privilege handling:
- sensitive-data handling:

Git:
- branch:
- commit:
- working tree:

Concerns:
- <real concerns only>

Next:
Goal 004 complete. Awaiting developer direction.
```

STOP after the final report.

Do NOT begin Goal 005 automatically.
