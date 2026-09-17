# VAM Goal 009 — Package Runtime Foundation

## Status

Planned

## Branch

goal/009-package-runtime-foundation

## Depends On

Goals 001–008

---

# 1. Objective

Establish the foundational VAM Runtime responsible for preparing a controlled execution context for installed VAM packages.

The Runtime is the boundary between:

    Installed Package
          ↓
      VAM Runtime
          ↓
    Future Execution

Goal 009 does NOT execute package code.

It establishes the runtime representation and preparation mechanisms required before execution can safely be implemented in a later goal.

---

# 2. Architectural Boundary

The intended architecture is:

    VAM Core
       │
       ├── Package
       │
       └── Runtime
              │
              └── Installed Package
                       │
                       └── Future Executor

The Runtime owns:

- runtime context
- package runtime environment
- declared package paths
- package configuration context where already supported
- dependency context representation where already supported
- entrypoint resolution
- runtime preparation

The Runtime does NOT own:

- package installation
- package removal
- repository operations
- dependency resolution
- package execution
- privilege escalation
- sandbox implementation
- TUI
- CLI presentation

---

# 3. In Scope

## 3.1 Runtime Context

Define the minimum runtime context required to prepare an installed VAM package.

The runtime context should represent, where justified:

- package identity
- installed package location
- package manifest
- package payload location
- entrypoint
- runtime working directory
- declared paths
- environment context

Do not add speculative fields.

---

## 3.2 Runtime Preparation

Implement preparation of an installed package for execution.

Preparation must:

- locate the installed package
- load its manifest
- validate required runtime information
- resolve the package entrypoint
- establish the runtime working context
- reject invalid runtime state

Preparation must not execute the package.

---

## 3.3 Entrypoint Resolution

The Runtime must resolve the package entrypoint from the installed package representation.

It must verify that:

- the entrypoint exists
- the entrypoint refers to a valid package-relative path
- the resolved path remains inside the installed package
- the entrypoint is not an absolute host path
- the entrypoint cannot escape the package directory

Do not execute the entrypoint.

---

## 3.4 Runtime Working Directory

Establish a controlled working directory for future package execution.

The design must distinguish between:

- installed package files
- runtime working data
- host filesystem locations

Do not allow package execution to implicitly use an arbitrary caller working directory.

The exact directory layout must be determined during architectural assessment.

---

## 3.5 Environment Representation

Establish the runtime environment representation required by future execution.

Do not automatically inherit or expose the entire host environment as package-controlled state.

The Runtime should establish an explicit representation that future execution can consume.

Do not implement a full sandbox or environment isolation system in this goal.

---

## 3.6 Runtime Validation

Runtime preparation must reject:

- missing installed package
- malformed installed package
- missing manifest
- missing entrypoint
- invalid entrypoint
- path traversal
- paths escaping the installed package
- invalid runtime state

Use the existing VAM validation and Error/Result architecture.

---

# 4. Out of Scope

The following are explicitly NOT part of Goal 009:

- executing shell scripts
- spawning package processes
- command execution
- `sh -c`
- package entrypoint execution
- dependency installation
- dependency resolution
- repositories
- downloads
- updates
- package removal
- signatures
- cryptographic verification
- trust policy
- capability enforcement
- sandboxing
- privilege escalation
- configuration engine
- TUI
- telemetry
- plugin system

Goal 009 prepares a package.

It does not run it.

---

# 5. Runtime Model

The intended conceptual model is:

    Installed Package
          │
          ▼
    Runtime Preparation
          │
          ├── Package Identity
          ├── Manifest
          ├── Package Root
          ├── Entrypoint
          ├── Working Directory
          ├── Environment Context
          └── Runtime State
                    │
                    ▼
             Future Executor

Keep the model minimal.

Do not implement future executor behavior prematurely.

---

# 6. Runtime State

Define the minimum runtime state required to represent a prepared package.

Possible conceptual states:

- Unprepared
- Prepared
- Invalid

Do not implement the complete package lifecycle state machine.

The Runtime state must not imply that a package is running.

---

# 7. Filesystem Safety

All runtime paths must be derived from controlled package installation data.

The implementation must prevent:

- absolute entrypoint paths
- `..` traversal
- escaping the package root
- arbitrary host filesystem access
- accidental writes outside controlled runtime locations

Use canonical or equivalent safe path validation where appropriate.

Do not follow arbitrary package-provided filesystem locations without validation.

---

# 8. Privilege Boundary

The Runtime must not introduce privilege escalation.

The Runtime itself should operate without elevated privileges unless a future architecture explicitly requires otherwise.

Do not introduce:

- sudo calls
- su calls
- setuid behavior
- privilege brokers

Privilege handling remains a separate architectural concern.

---

# 9. Security Boundary

The Runtime must treat installed package content as executable-capable but untrusted data.

Runtime preparation must never execute package code.

Do not claim that the Runtime provides a sandbox.

Do not claim that packages are trusted merely because they were installed.

Security guarantees beyond path and preparation safety require later architecture.

---

# 10. API Design

Before implementation determine the smallest useful Runtime API.

Prefer concrete structures/functions.

Avoid speculative traits for:

- executors
- sandboxes
- repositories
- dependency resolvers
- privilege brokers
- plugin systems

Only introduce abstraction where the existing implementation demonstrates a real need.

---

# 11. Integration

Integrate Runtime with the existing package architecture without changing package installation behavior unnecessarily.

The Runtime should consume the installed package representation produced by Goal 008.

Do not duplicate package validation logic.

Reuse existing package/container/path validation where appropriate.

---

# 12. Dependencies

Prefer the existing dependency set and Rust standard library.

Do not add a dependency unless genuinely necessary.

If a new dependency is proposed:

1. identify it
2. explain why existing functionality is insufficient
3. assess maintenance/security implications
4. stop for architectural approval if it materially affects the core

Never silently introduce a new dependency.

---

# 13. Testing Requirements

Add deterministic tests covering:

## Runtime Preparation

- valid installed package prepares successfully
- package identity is preserved
- manifest is loaded correctly
- package root is resolved correctly
- entrypoint resolves correctly
- runtime working context is established

## Invalid Runtime State

- missing package rejected
- missing manifest rejected
- missing entrypoint rejected
- malformed runtime state rejected

## Path Security

- absolute entrypoint rejected
- `..` traversal rejected
- entrypoint outside package root rejected
- invalid package paths rejected

## Execution Safety

Tests must demonstrate that Runtime preparation does NOT execute package code.

Tests must use controlled temporary directories.

Tests must not modify the real installed VAM package directory.

---

# 14. Documentation

Update only documentation describing implemented behavior.

Potential files:

- Package.md
- Architecture.md
- Security.md
- FUNDATION.md

Document:

- Runtime responsibility
- Runtime preparation
- runtime context
- entrypoint resolution
- filesystem boundary
- security limitations

Do not document package execution as implemented.

---

# 15. Verification

Run:

    cargo fmt --all -- --check
    cargo check --all-targets --all-features
    cargo test --all-targets --all-features
    cargo clippy --all-targets --all-features -- -D warnings
    cargo build --release

Run CLI smoke tests:

    cargo run -- version
    cargo run -- help
    cargo run --

Also run:

    git diff --check
    git status

Review the final diff for:

- scope creep
- accidental execution
- unsafe path handling
- duplicated validation
- unnecessary abstractions
- unnecessary dependencies
- privilege escalation
- misleading security claims

---

# 16. Definition of Done

Goal 009 is complete when:

- VAM has a defined Runtime boundary
- an installed package can be prepared for future execution
- runtime context is represented
- package root is controlled
- entrypoint resolution is safe
- runtime working context is defined
- invalid runtime state is rejected
- package code is never executed
- no privilege escalation is introduced
- focused tests pass
- all existing tests pass
- fmt/check/clippy pass
- no unjustified dependencies are introduced
- documentation reflects actual behavior
- final diff is scoped and clean
- implementation is committed using the required convention

---

# 17. Git

Branch:

goal/009-package-runtime-foundation

Commit convention:

goal/009-package-runtime-foundation: <subject>

Do not modify unrelated goals.

Do not rewrite history.

Do not force-push.

---

# 18. Final Audit Report

At completion, report:

GOAL 009 FINAL AUDIT

Status:
Architecture:
Runtime Context:
Runtime Preparation:
Entrypoint Resolution:
Working Directory:
Environment:
Path Security:
Execution Safety:
Dependencies:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
