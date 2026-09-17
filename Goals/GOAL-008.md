# VAM Goal 008 — Package Installation Foundation

## Status

Planned

## Branch

`goal/008-package-installation-foundation`

## Depends On

Goals 001–007

---

# 1. Objective

Establish the foundational VAM package installation system.

Goal 006 established the `.vampkg` package container format.

Goal 007 established package validation.

Goal 008 connects validated package artifacts to an installed package representation on the host system.

The installation system must:

1. accept a `.vampkg` artifact
2. validate it before installation
3. safely extract its package contents
4. install it into the VAM package runtime storage
5. preserve package identity and metadata
6. prevent unsafe filesystem writes
7. provide controlled installation errors
8. avoid executing package payloads

Installation is a filesystem/package-state operation.

It is NOT package execution.

---

# 2. Architectural Boundary

```text
.vampkg
   │
   ▼
Package Validation
   │
   ▼
Package Installation
   │
   ▼
Installed Package
   │
   ├── Manifest
   ├── Payload
   ├── Entrypoint
   └── Installation Metadata

The installation layer belongs to VAM Core.

It must not become responsible for:

    package execution

    Runtime behavior

    dependency resolution

    repositories

    network access

    package trust policy

    capability enforcement

    sandboxing

    configuration management

    TUI presentation

3. In Scope
3.1 Installation API

Establish the smallest useful API for installing a validated .vampkg.

The API must support a local package artifact as input.

Determine the exact API during architectural assessment.

Do not create speculative abstractions.
3.2 Pre-Installation Validation

Installation must never blindly extract an arbitrary .vampkg.

Before filesystem mutation:

    validate the container

    validate the manifest

    validate package paths

    validate required package structure

    determine package identity

    determine installation destination

Invalid packages must be rejected before installation begins.
3.3 Installation Destination

Establish the canonical VAM package installation location.

The implementation must use the architecture established by previous goals.

The current intended runtime package location is conceptually:

/usr/lib/vam/packages/

Do not introduce a second competing package location.

If the existing implementation/documentation establishes a different canonical location, follow the existing architecture and update documentation only if necessary.
3.4 Installed Package Structure

Define the minimum installed representation required by the current architecture.

The installed package must preserve:

    package identity

    manifest

    package version/release

    package payload

    entrypoint information

Do not introduce runtime state that belongs to future goals.
3.5 Safe Extraction

Package extraction must be filesystem-safe.

Reject or prevent:

    absolute paths

    .. traversal

    path escaping

    conflicting file/directory paths

    unexpected filesystem targets

    writes outside the package installation root

Extraction must operate relative to a controlled installation destination.
3.6 Existing Package Handling

Determine and document behavior when a package with the same identity is already installed.

The implementation must not silently overwrite an existing package without an explicitly defined policy.

Possible behavior may include:

    reject installation

    require an explicit replacement/update operation

Do not implement package updates in Goal 008 unless required to establish safe replacement semantics.
3.7 Filesystem Permissions

Installation may require elevated privileges when writing to the system package directory.

Do NOT implement a general privilege broker in this goal.

If privilege escalation is required, establish only the smallest controlled mechanism required by the existing VAM architecture.

Do not allow package payload scripts to obtain privileges.

Installation itself may perform privileged filesystem operations; package contents must never be executed as part of installation.
3.8 Installation State

Establish the minimum persistent information required to identify an installed package.

Do not build a full package database unless the architecture proves it necessary.

If installation metadata must be stored, define its smallest useful representation.

The design must support future:

    package removal

    package execution

    package updates

without prematurely implementing those systems.
4. Out of Scope

The following are explicitly NOT part of Goal 008:

    package execution

    VAM Runtime

    entrypoint execution

    shell execution

    dependency resolution

    dependency installation

    repositories

    package downloads

    package updates

    package removal

    signatures

    cryptographic verification

    trust policies

    capability enforcement

    sandboxing

    configuration engine

    TUI

    telemetry

    plugin system

    package marketplace

    Arch package replacement

    arbitrary command execution

A package being installed does NOT mean it is running.
5. Installation Lifecycle

The intended conceptual flow is:

Input .vampkg
      │
      ▼
Read Container
      │
      ▼
Validate Package
      │
      ▼
Determine Identity
      │
      ▼
Check Installation State
      │
      ▼
Prepare Controlled Destination
      │
      ▼
Extract Safely
      │
      ▼
Write Installation Metadata
      │
      ▼
Installed

If an installation step fails, the implementation must avoid leaving a misleading partially-installed package.

Atomicity/rollback behavior must be assessed before implementation.
6. Atomic Installation

Determine the smallest reliable mechanism for preventing partially-installed packages from appearing as successfully installed.

Preferred behavior:

temporary installation area
        ↓
complete extraction/validation
        ↓
atomic placement
        ↓
installed package

Do not mark a package installed until the installation operation has completed successfully.

If full atomic installation is impractical for the current filesystem model, document the limitation and implement the safest practical behavior.
7. Security Requirements

Package contents are untrusted input.

Installation must:

    never execute package scripts

    never execute the declared entrypoint

    validate paths before extraction

    prevent path traversal

    prevent absolute-path writes

    prevent writes outside the installation root

    avoid arbitrary shell execution

    avoid interpreting package payload as commands

    avoid following unsafe filesystem targets where applicable

    avoid leaking sensitive filesystem information through errors

A .vampkg extension does not establish trust.

Cryptographic trust remains outside this goal.
8. Error Handling

Use the existing:

Error
ErrorKind
Result<T>

model.

Installation failures must produce controlled errors.

Expected user/package errors should normally use ErrorKind::Usage.

Unexpected filesystem/internal failures should use ErrorKind::Internal.

Do not introduce a new error taxonomy without architectural justification.

Errors must not unnecessarily expose:

    credentials

    secrets

    unrelated filesystem contents

    sensitive environment information

9. Module Ownership

The installation layer owns:

    installation orchestration

    installation destination handling

    safe extraction

    installed package placement

    installation-state creation

It does NOT own:

    package execution

    Runtime

    repositories

    dependency resolution

    CLI presentation

    TUI

    package trust

    sandboxing

    privilege policy beyond the minimum installation operation

Keep responsibilities separated from the existing package container and validation layers.
10. API / Architecture Assessment

Before implementation determine:

    exact installation entrypoint

    ownership of filesystem operations

    whether existing system.rs is sufficient

    whether package/container APIs need extension

    whether installation metadata requires a new structure

    how installation atomicity should work

    how existing packages are detected

    how privilege requirements are handled

    how failures clean up partial state

Prefer concrete APIs.

Do not introduce traits or dependency injection unless there is a demonstrated need.

If an unresolved architectural decision materially affects installation safety or future package architecture:

STOP and report the decision instead of guessing.
11. Dependencies

Prefer the Rust standard library and existing dependencies.

Do not add a dependency unless it is genuinely required.

If a new dependency is proposed:

    identify it

    explain why existing functionality is insufficient

    assess security/maintenance implications

    stop for architectural approval if it materially changes the core

Do not silently add dependencies.
12. Testing Requirements

Add focused deterministic tests covering at minimum:
Installation

    valid package installs successfully

    installed package identity is preserved

    manifest is preserved

    payload files are installed

    entrypoint is preserved

    installation destination is correct

Validation

    invalid package rejected before filesystem mutation

    malformed package rejected

    invalid manifest rejected

Path Security

    absolute path rejected

    .. traversal rejected

    path escaping installation root rejected

    conflicting paths rejected

    unsafe package structure rejected

Existing Package

    duplicate identity behavior is deterministic

    existing installation is not silently corrupted

Failure Handling

    failed installation does not report success

    partial installation cleanup works according to the chosen atomicity model

Execution Safety

Tests must verify that package payloads are NOT executed during installation.

Tests must not depend on the user's real /usr/lib/vam/packages/ directory.

Use controlled temporary test directories where appropriate.
13. Documentation

Update only documentation describing implemented behavior.

Potential files:

    Package.md

    Architecture.md

    Security.md

    FUNDATION.md

Document:

    installation lifecycle

    installation destination

    installed package structure

    duplicate-package behavior

    path-safety rules

    atomicity/cleanup behavior

    security boundary

Do not describe package execution as implemented.
14. Verification

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

    scope creep

    unsafe extraction

    partial installation problems

    accidental execution

    privilege mistakes

    unnecessary dependencies

    duplicated logic

    speculative abstractions

    incorrect documentation

15. Definition of Done

Goal 008 is complete when:

    a valid .vampkg can be installed

    package validation occurs before installation

    package contents are safely extracted

    installation occurs in the canonical VAM package location

    package identity and manifest are preserved

    duplicate-package behavior is defined and tested

    partial installations are handled safely

    package payloads are never executed

    path traversal is prevented

    existing VAM behavior remains intact

    focused tests pass

    full test suite passes

    fmt/check/clippy pass

    no unjustified dependencies are introduced

    documentation reflects actual implementation

    final diff is scoped and clean

    implementation is committed using the required convention

16. Git

Branch:

goal/008-package-installation-foundation

Commit convention:

goal/008-package-installation-foundation: <subject>

Do not modify unrelated goals.

Do not rewrite history.

Do not force-push.
17. Final Audit Report

At completion, report:

GOAL 008 FINAL AUDIT

Status:
Architecture:
Installation API:
Validation:
Extraction:
Installation destination:
Atomicity:
Duplicate-package behavior:
Installation state:
Dependencies:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
