
# VAM Goal 005 — Package Specification & Manifest Foundation

**Goal ID:** 005
**Goal Name:** `package-specification-foundation`
**Branch:** `goal/005-package-specification-foundation`
**Status:** Planned
**Depends On:** Goals 001–004

---

## 1. Objective

Establish the foundational specification and in-memory representation of a **VAM Package**.

Define the minimum manifest required for VAM to identify, describe, validate, and reason about a package before installation or execution exists.

The `.vampkg` container format itself is **not implemented in this goal**.

---

## 2. In Scope

* Establish the package module/boundary.
* Define the minimum package identity model.
* Define the manifest data model.
* Define required vs optional metadata.
* Define package name, version, developer/author identity, and description fields as justified by existing documentation.
* Define manifest validation.
* Integrate the existing `Error` / `Result` model.
* Add focused package-model and validation tests.
* Document the implemented package specification.

---

## 3. Out of Scope

Do NOT implement:

* `.vampkg` archive creation
* tar/zstd packaging
* package extraction
* package installation
* package removal
* package execution
* VAM Runtime
* dependencies/resolution
* repositories
* downloads
* signatures
* cryptographic verification
* capabilities
* sandboxing
* privilege escalation
* package configuration engine
* package lifecycle
* TUI package management

If any of these become necessary, STOP.

---

## 4. Architecture

The intended boundary is:

```text
Package System
├── package identity
├── manifest model
└── manifest validation
```

The package module owns package-domain representation and validation.

It does NOT own:

* filesystem operations
* archive handling
* OS execution
* repository communication
* security enforcement
* CLI presentation

Use the existing System Interface for future OS interaction rather than bypassing it.

---

## 5. Manifest Principles

The manifest must be:

* deterministic
* explicit
* versionable
* machine-readable
* human-understandable
* minimal
* extensible without speculative fields

Do not invent large metadata schemas.

Only fields justified by the current VAM specification should be implemented.

---

## 6. Architectural Assessment

Before implementation, inspect:

```text
AGENTS.md
README.md
Architecture.md
Security.md
CLI.md
Package.md
Repository.md
Configuration.md
FUNDATION.md
Goals/GOAL-001.md
Goals/GOAL-002.md
Goals/GOAL-003.md
Goals/GOAL-004.md
src/*
```

Determine:

1. Exact current package requirements.
2. Required manifest fields.
3. Package identity rules.
4. Validation ownership.
5. Whether serialization is required now.
6. Whether an external serialization dependency is justified.

If an unresolved architectural decision appears, STOP.

---

## 7. Dependency Rule

Prefer the Rust standard library.

Do NOT introduce a serialization crate merely because the future `.vampkg` format may use one.

If an external dependency is genuinely required:

```text
ARCHITECTURAL STOP

Problem:
<what requires the dependency>

Risk:
<architectural/maintenance impact>

Safer approach:
<alternative>

Decision required:
<exact decision>
```

Wait for approval.

---

## 8. Error & Security

Use the existing:

```text
ErrorKind
├── Usage
└── Internal

Result<T>
```

Validation failures must be represented consistently.

Do not expose:

* secrets
* credentials
* environment variables
* private data
* unnecessary filesystem paths

Do not implement package trust/security enforcement in this goal.

---

## 9. Testing

Add focused tests covering:

* valid package identity
* required-field validation
* invalid package metadata
* version validation where applicable
* developer/author metadata
* description/metadata handling
* error categorization
* stable validation behavior

All existing tests must continue passing.

---

## 10. Documentation

Update the appropriate package documentation with the **implemented** manifest contract.

Do not document installation, runtime, repository, or security functionality as implemented.

---

## 11. Verification

Run:

```bash
cargo fmt --all -- --check
cargo check --all-targets --all-features
cargo test --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo run -- version
cargo run -- help
cargo run
```

Then:

```bash
git status
git diff
git diff --check
```

---

## 12. Git

Branch:

```text
goal/005-package-specification-foundation
```

Commit:

```text
goal/005-package-specification-foundation: <subject>
```

No force pushes, history rewriting, unrelated refactors, or Goal 006 implementation.

---

## 13. Definition of Done

* [ ] Package boundary established.
* [ ] Package identity defined.
* [ ] Manifest model defined.
* [ ] Validation implemented.
* [ ] Error/Result integrated.
* [ ] Tests added.
* [ ] Existing tests pass.
* [ ] fmt/check/test/clippy pass.
* [ ] CLI smoke tests pass.
* [ ] Documentation updated.
* [ ] No unnecessary dependency.
* [ ] No package installation/runtime functionality.
* [ ] Git diff contains only Goal 005 work.
* [ ] Correct commit created.

---

## 14. Final Report

Report:

```text
GOAL 005 FINAL AUDIT

Status:
PASS / STOP / FAIL

Architecture:
- package ownership:
- package identity:
- manifest model:
- validation:

Implementation:
- files changed:
- dependencies:

Tests:
- total:
- new:
- failures:

Verification:
- fmt:
- check:
- test:
- clippy:
- CLI:

Security:
- validation:
- sensitive-data handling:

Git:
- branch:
- commit:
- working tree:

Concerns:

Next:
Goal 005 complete. Awaiting developer direction.
```

STOP after the report.
