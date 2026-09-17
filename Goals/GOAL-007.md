# VAM Goal 007 — Package Validation Foundation

## Status

Planned

## Branch

`goal/007-package-validation`

## Depends On

Goals 001–006

---

# 1. Objective

Establish the authoritative package validation layer for VAM.

Goal 006 established the `.vampkg` container format and basic container/path handling.

Goal 007 establishes semantic validation of a complete `.vampkg` package after it has been decoded from its container representation.

The validator must determine whether a package satisfies the currently implemented VAM Package specification without installing or executing it.

---

# 2. Architectural Boundary

```text
.vampkg
   │
   ▼
Container Layer
   │
   ▼
Decoded Package
   │
   ▼
Package Validator
   │
   ├── Package identity validation
   ├── Manifest validation
   ├── Entrypoint validation
   ├── Payload structure validation
   └── Package consistency validation

Validation is a VAM Core concern.

It does not install, execute, trust, or manage packages.

3. In Scope
3.1 Package-Level Validation

Create the smallest useful validation API for validating a complete decoded VAM package.

Validation must build on Goal 005's existing:

PackageId
Manifest
Manifest::validate

and Goal 006's container representation.

3.2 Manifest Validation

Validate that the package manifest:

contains all required fields
contains valid package identity
has valid version/release representation according to the currently implemented specification
contains required descriptive metadata
contains a supported package type only if package type validation has already been formally defined

Do not invent additional semantic restrictions that are not documented.

3.3 Entrypoint Validation

Validate that:

an entrypoint is declared
the declared entrypoint exists in the package payload
the entrypoint is represented as a package-relative path
the entrypoint does not escape the package root
the entrypoint does not conflict with an invalid payload structure

The validator must NOT execute the entrypoint.

3.4 Payload Validation

Validate package payload consistency.

At minimum detect:

duplicate logical paths
file/directory conflicts
invalid package paths
missing required directories/files
invalid entrypoint references
structurally inconsistent payloads

Reuse Goal 006 path-validation logic where appropriate.

Do not duplicate path-validation rules unnecessarily.

3.5 Container + Semantic Validation

Provide a validation path capable of taking a .vampkg artifact and determining:

Container valid
        ↓
Manifest valid
        ↓
Payload valid
        ↓
Entrypoint valid
        ↓
Package valid

Failures must be reported through the existing VAM error model.

4. Validation Layers

Keep these concepts distinct:

Container Validation

Determines whether the .vampkg container itself is readable and structurally valid.

Examples:

bad magic
unsupported format version
malformed archive
invalid archive structure
Package Validation

Determines whether the decoded package satisfies the VAM Package specification.

Examples:

invalid manifest
missing entrypoint
entrypoint does not exist
invalid package structure
inconsistent package metadata

Do not merge these responsibilities into one unstructured validation function if doing so damages the existing architecture.

5. Out of Scope

The following are explicitly NOT part of Goal 007:

package installation
package removal
extraction into VamRuntims
package execution
VAM Runtime
dependency resolution
Arch package dependency management
repositories
downloading
package updates
package signatures
cryptographic verification
trust policies
capability enforcement
sandboxing
privilege escalation
configuration engine
lifecycle management
TUI
telemetry
database/state management
network functionality
package execution permissions
arbitrary command execution

Goal 007 validates packages.

It does not operate them.

6. API Design

Before implementation:

inspect the APIs introduced by Goals 005 and 006
determine whether validation belongs in package or a dedicated package-validation module
identify the smallest useful public API
avoid speculative traits and abstractions

Do not create a large validation framework.

Prefer concrete functions/types until real architectural pressure requires otherwise.

7. Error Handling

Use the existing:

Error
ErrorKind
Result<T>

model.

Invalid user/package input should normally produce controlled Usage errors.

Unexpected internal failures should use Internal.

Validation errors should identify the violated package rule without unnecessarily echoing package contents.

Do not introduce a new error taxonomy unless strictly justified.

8. Security Requirements

Treat every .vampkg package as untrusted input.

Validation must:

never execute package payloads
never invoke a shell
never invoke sudo
never require elevated privileges
validate package-relative paths
reject traversal paths
reject absolute paths
avoid arbitrary filesystem writes
avoid trusting package metadata merely because the container is readable

Do not claim that validation establishes package trust.

Trust/signature mechanisms belong to a later security goal.

9. Testing Requirements

Add focused deterministic tests covering at minimum:

Manifest
valid manifest
invalid identity
missing required fields
invalid manifest data
Entrypoint
valid entrypoint
missing entrypoint
entrypoint path traversal
absolute entrypoint
entrypoint pointing to nonexistent payload
entrypoint conflicting with payload structure
Payload
valid payload
duplicate paths
file/directory conflicts
invalid paths
missing required package components
Complete Package
valid .vampkg passes validation
malformed package fails validation
invalid manifest fails validation
invalid payload fails validation
invalid entrypoint fails validation
Safety
validation never executes payload code
invalid package paths are rejected

Tests must remain deterministic and must not depend on network access.

10. Documentation

Update only documentation describing behavior actually implemented.

Potential files:

Package.md
Security.md
Architecture.md
FUNDATION.md

Document:

package validation boundary
container validation vs package validation
validation rules
entrypoint requirements
security limitations

Do not document installation/runtime behavior as implemented.

11. Dependency Policy

Reuse existing dependencies from Goal 006 where justified.

Do not add a new dependency unless the implementation genuinely requires it.

If a new dependency is proposed:

identify why it is required
determine whether existing code/std can provide the functionality
assess its architectural impact
STOP for approval if it materially changes the core design

Do not add dependencies merely for convenience.

12. Verification

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

duplicated validation logic
scope creep
unnecessary abstractions
unnecessary dependencies
path-security issues
accidental execution behavior
documentation claiming unimplemented functionality
13. Definition of Done

Goal 007 is complete when:

a complete .vampkg can be semantically validated
container validation and package validation remain conceptually separated
manifest validation is integrated with package validation
entrypoint validity is checked
payload consistency is checked
package paths are securely validated
invalid packages are rejected deterministically
package payloads are never executed
existing Error/ErrorKind/Result infrastructure is used
focused tests pass
all existing tests pass
fmt/check/test/clippy pass
no unjustified dependencies are added
documentation reflects implemented behavior
final diff is scoped and clean
implementation is committed using the required convention
14. Git

Branch:

goal/007-package-validation

Commit convention:

goal/007-package-validation: <subject>

Do not modify unrelated goals.

Do not rewrite history.

Do not force-push.

15. Final Audit Report

At completion, report:

GOAL 007 FINAL AUDIT

Status:
Architecture:
Validation API:
Manifest validation:
Entrypoint validation:
Payload validation:
Container/semantic boundary:
Dependencies:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
