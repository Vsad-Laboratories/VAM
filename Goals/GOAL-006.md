# VAM Goal 006 — Package Container Format Foundation

## Status

Planned

## Branch

`goal/006-package-container-format`

## Depends On

Goals 001–005

---

# 1. Objective

Establish the first real on-disk `.vampkg` package container format for VAM.

Goal 005 established the in-memory `PackageId` and `Manifest` domain model.

Goal 006 establishes how a complete VAM package is represented as a distributable `.vampkg` artifact.

The result must provide a deterministic, versioned, inspectable package container that can represent:

- package manifest
- package payload files
- package directory structure
- package entrypoint
- format/version metadata

The `.vampkg` file must be a real VAM-defined package container, not merely a renamed shell script.

---

# 2. Architectural Boundary

```text
VAM Core
   │
   └── Package Domain
          │
          ├── PackageId
          ├── Manifest
          └── Package Container Format
                    │
                    └── .vampkg

The package container layer is responsible for representing package artifacts on disk.

It must NOT become responsible for installing, executing, trusting, or managing packages.

3. In Scope
3.1 Package Container Specification

Define the structure of a .vampkg artifact.

The format must define:

format identifier / magic
format version
manifest representation
payload representation
entrypoint representation
required package structure
deterministic ordering requirements where applicable
malformed-package behavior

The format must be explicitly versionable so future VAM releases can evolve it without silently misinterpreting older packages.

3.2 Manifest Serialization

Establish a machine-readable representation of the existing Goal 005 Manifest.

The serialization format must preserve the fields currently defined by Manifest without introducing speculative package metadata.

Do not add:

dependencies
capabilities
signatures
integrity metadata
repository metadata
lifecycle state
configuration engine
runtime state

unless the existing architecture proves one is strictly required by the container format itself.

3.3 Payload Representation

Define how package files are stored inside .vampkg.

The format must support:

files
directories
package entrypoint
executable package scripts
future package resources

Path representation must be explicit and must not permit ambiguous traversal semantics.

3.4 Package Creation

Implement the minimum functionality required to construct a valid .vampkg from an existing package representation/source structure.

The implementation must:

validate the manifest before packaging
reject invalid package structures
produce a structurally valid package
produce deterministic output where practical
avoid executing package payload code
3.5 Package Inspection

Implement the minimum functionality required to inspect a .vampkg without installing or executing it.

Inspection must be able to determine at minimum:

whether the container is structurally valid
format version
package identity
package manifest
package entrypoint
contained payload paths

Malformed or unsupported packages must return controlled VAM errors.

4. Out of Scope

The following are explicitly NOT part of Goal 006:

package installation
package removal
package extraction into VamRuntims
package execution
VAM Runtime
dependency resolution
Arch package dependencies
repositories
downloading
package updates
signatures
cryptographic verification
trust policies
capability enforcement
sandboxing
privilege escalation
configuration engine
lifecycle management
TUI integration
network functionality
telemetry
database/state persistence
package manager replacement functionality

Goal 006 creates and understands package artifacts.

It does not operate them.

5. Format Requirements

The .vampkg format must be:

versioned
deterministic where practical
explicitly structured
inspectable
resistant to path ambiguity
extensible
independent from package execution
suitable for shell-based VAM packages
appropriate for Arch Linux
maintainable using a reasonable Rust implementation

The implementation must reject:

invalid headers
unsupported format versions
malformed manifests
duplicate/conflicting package paths
invalid package paths
missing required package components
malformed container structure
6. Compression / Archive Technology

The implementation must perform an architectural assessment before selecting an archive or compression implementation.

Possible approaches may include an established archive/container format or a VAM-defined envelope around an established archive representation.

Do NOT invent a complex archive implementation unnecessarily.

If an external Rust dependency is required for archive or compression support:

identify the exact dependency
explain why the Rust standard library is insufficient
assess maintenance/security/licensing implications
stop for architectural approval if the dependency materially affects the core format

Do not silently add a dependency.

7. Package Path Rules

Package payload paths must use a canonical representation.

At minimum:

no absolute paths
no .. traversal components
no ambiguous path normalization
no duplicate logical paths
directories and files must not conflict
platform-specific path semantics must not alter package meaning

The package format must represent paths independently from the host filesystem's current working directory.

8. Error Handling

Use the existing:

Error
ErrorKind
Result<T>

model from Goal 002.

Malformed or user-provided .vampkg input should normally produce controlled Usage errors.

Unexpected internal failures should use Internal.

Do not introduce a new error taxonomy unless architectural assessment proves it necessary.

Error messages must not unnecessarily echo package contents, credentials, or sensitive data.

9. Module Ownership

The package module/container layer owns:

package artifact representation
package format definitions
manifest serialization/deserialization
package structure validation
package creation
package inspection

It does NOT own:

filesystem policy outside package construction/inspection
package installation
package execution
privilege escalation
repositories
network access
runtime policy
CLI presentation
TUI presentation

Keep the boundary explicit.

10. API Design

Before implementation, determine the smallest useful public API.

Do not create speculative abstractions such as:

generic archive traits
plugin systems
repository interfaces
runtime interfaces
package registries
dependency graphs

unless required by the actual implementation.

Prefer concrete APIs until multiple implementations genuinely require abstraction.

11. Security Requirements

Package creation and inspection must never execute package payload code.

Package inspection must treat .vampkg contents as untrusted input.

The implementation must:

validate container structure before consuming it
validate paths
reject malformed metadata
avoid arbitrary command execution
avoid privilege escalation
avoid writing outside explicitly controlled destinations
avoid trusting package contents merely because the file extension is .vampkg

Cryptographic trust and signatures are intentionally deferred to a later goal.

12. Testing Requirements

Add focused deterministic tests covering at minimum:

Format
valid package creation
valid package inspection
format identifier
supported format version
unsupported format version
malformed header/container
Manifest
manifest serialization
manifest deserialization
round-trip preservation
invalid manifest rejection
Payload
files
directories
entrypoint
nested paths
duplicate paths
conflicting file/directory paths
Path Security
absolute paths rejected
.. traversal rejected
invalid/ambiguous paths rejected
Determinism

Where the selected format permits deterministic output:

identical package inputs produce equivalent artifacts
ordering is stable

Tests must not execute package payloads.

13. Documentation

Update documentation only for behavior actually implemented.

Potential documentation targets:

Package.md
Architecture.md
Security.md
FUNDATION.md

Document:

.vampkg format
package structure
format versioning
package path rules
inspection behavior
security boundary

Do not document future installation/runtime behavior as implemented functionality.

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

Also perform:

git diff --check
git status

Review the final diff for:

scope creep
unnecessary dependencies
duplicated logic
unsafe path handling
accidental execution behavior
unnecessary abstractions
documentation claiming unimplemented behavior
15. Definition of Done

Goal 006 is complete when:

.vampkg has a documented format
format versioning exists
the existing Goal 005 manifest can be serialized/deserialized
valid package artifacts can be created
valid package artifacts can be inspected
malformed artifacts are rejected safely
package paths are validated
package payloads are never executed
focused tests pass
existing VAM tests remain passing
fmt/check/test/clippy pass
no unjustified dependencies were introduced
documentation reflects implemented behavior
final git diff is clean and scoped
implementation is committed using the required convention
16. Git

Branch:

goal/006-package-container-format

Commit convention:

goal/006-package-container-format: <subject>

Do not modify unrelated goals.

Do not rewrite history.

Do not force-push.

17. Final Audit Report

At completion, report:

GOAL 006 FINAL AUDIT

Status:
Architecture:
Format:
Manifest serialization:
Payload representation:
Path validation:
Inspection:
Dependencies:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
