# VAM Goal 010 — Package Executor Foundation

## Status

Planned

## Branch

`goal/010-package-executor-foundation`

## Depends On

Goals 001–009

---

# 1. Objective

Establish the VAM package executor.

Goal 009 established the VAM Runtime and prepared an installed package for execution.

Goal 010 adds the process-execution layer that consumes a prepared RuntimeContext and executes the package's declared entrypoint.

The result must allow VAM to execute a valid installed package through its runtime context.

The executor must:

- spawn the declared package entrypoint
- execute it as a child process
- use the Runtime working directory
- use the Runtime environment
- provide controlled standard input/output/error behavior
- return the process result to VAM
- never execute through an implicit shell
- never perform privilege escalation

This is the first goal that crosses from package preparation into actual package execution.

---

# 2. Architectural Boundary

```text
VAM Core
   │
   ├── Package
   │
   ├── Runtime
   │      │
   │      └── RuntimeContext
   │
   └── Executor
          │
          └── Child Process

Runtime prepares the execution context.

Executor performs process spawning.

Package remains responsible for package representation and validation.

Executor must not become responsible for:

package installation
package validation policy
dependency resolution
repositories
package discovery
configuration management
privilege escalation
sandboxing
trust verification
3. In Scope
3.1 Executor Module

Create the smallest appropriate executor boundary.

The executor receives a prepared RuntimeContext and executes its declared entrypoint.

Determine the exact API during architectural assessment.

Prefer concrete APIs over speculative traits.

3.2 Process Spawning

Use the existing System Interface where appropriate.

The executor must spawn the package entrypoint as a child process.

The implementation must:

execute the resolved entrypoint directly
provide the Runtime working directory
provide the Runtime environment
avoid implicit shell invocation
avoid sh -c
avoid bash -c
avoid string-based command construction

The package entrypoint itself may be a shell script, but VAM must invoke the declared executable directly rather than constructing a shell command string.

3.3 Runtime Environment

The executor must use the environment established by RuntimeContext.

Do not automatically inherit the complete host environment.

Only explicitly provided Runtime environment variables should be passed to the package process.

Do not introduce environment discovery or environment policy in this goal.

3.4 Working Directory

The child process must start in the Runtime working directory established by Goal 009.

It must NOT use:

the caller's current working directory
an arbitrary package path
the VAM source directory
3.5 Standard Streams

Define the minimum standard stream behavior.

The initial implementation may inherit the parent's:

stdin
stdout
stderr

unless the existing architecture establishes a more appropriate mechanism.

Do not implement a complete terminal/session subsystem.

3.6 Process Result

The executor must return a structured result representing process completion.

At minimum distinguish:

successful exit
non-zero exit
process spawn failure

Do not introduce a large process-state abstraction.

Use the existing Error/ErrorKind/Result model where appropriate.

3.7 CLI Integration

If the existing CLI architecture permits it without introducing unrelated work, connect the executor to the existing package execution command.

The intended user-facing flow is conceptually:

vam run <package>

The command should:

identify the installed package
prepare its RuntimeContext
execute through the Executor
report the resulting process status

Do not implement package discovery, dependency resolution, repositories, or a TUI in this goal.

If CLI integration would materially expand the architecture, STOP and report the boundary instead of forcing it into this goal.

4. Out of Scope

The following are explicitly NOT part of Goal 010:

dependency resolution
dependency installation
repository support
package downloads
package updates
package removal
package signatures
cryptographic verification
trust policies
capability enforcement
sandboxing
privilege escalation
sudo handling
configuration engine
package configuration UI
lifecycle manager
telemetry
plugin system
TUI
terminal multiplexer/session management
arbitrary user command execution through VAM
shell command construction
automatic host-environment inheritance

Goal 010 executes an already-installed, already-prepared package.

5. Execution Lifecycle

The intended flow is:

vam run <package>
       │
       ▼
Locate Installed Package
       │
       ▼
Runtime::prepare()
       │
       ▼
RuntimeContext
       │
       ▼
Executor
       │
       ▼
Spawn Entrypoint
       │
       ▼
Child Process
       │
       ▼
Process Result
       │
       ▼
VAM Result

The executor must not bypass Runtime preparation.

6. Entrypoint Execution

The executor must use the entrypoint resolved by Runtime.

It must NOT:

reconstruct the path from raw manifest strings
bypass Runtime path validation
execute arbitrary manifest-provided command strings
prepend a shell
interpret shell syntax itself

Runtime remains responsible for establishing the safe executable path.

Executor consumes the prepared context.

7. Security Requirements

Package code must be treated as executable and potentially untrusted.

Therefore:

execution must require an explicitly installed package
execution must require successful Runtime preparation
execution must use the Runtime-resolved entrypoint
no sh -c
no bash -c
no shell command-string interpretation by VAM
no arbitrary executable path supplied directly to the executor
no automatic sudo
no privilege escalation
no automatic host environment inheritance
no execution outside the prepared package context

Do NOT claim that this provides sandboxing.

Goal 010 establishes controlled process invocation, not a security sandbox.

8. Privilege Model

Do not introduce a privilege broker.

The package process must execute with the privileges of the VAM process unless an existing, explicitly defined architecture states otherwise.

VAM must not automatically invoke:

sudo
su
pkexec
doas
other privilege escalation mechanisms

A future privilege/capability architecture must be designed separately.

9. Process API Assessment

Before implementation determine:

whether std::process::Command is sufficient
whether system.rs should own process spawning mechanics
whether Executor should call System directly
what process result representation is required
how exit codes are represented
how spawn failures map to ErrorKind
whether CLI integration belongs in this goal

Prefer the smallest architecture that satisfies the objective.

Do not introduce a process abstraction merely for theoretical future implementations.

If the process boundary creates a significant architectural decision:

STOP and report it before implementation.

10. Testing Requirements

Add deterministic tests covering at minimum:

Executor
valid entrypoint executes
successful process returns success
non-zero exit is represented correctly
spawn failure is controlled
Runtime Integration
executor uses Runtime-resolved entrypoint
executor uses Runtime working directory
executor uses Runtime environment
Security
executor does not invoke sh -c
executor does not invoke bash -c
executor does not automatically inherit the full host environment
executor cannot bypass Runtime path validation
Execution Boundary
package entrypoint executes only after successful Runtime preparation
invalid package context cannot be executed

Tests must use controlled temporary package/runtime directories.

Tests must not modify the user's real VAM installation.

11. Documentation

Update only documentation describing implemented behavior.

Potential files:

Package.md
Architecture.md
Security.md
CLI.md
FUNDATION.md

Document:

executor architecture
execution lifecycle
entrypoint invocation
environment behavior
working directory behavior
process result behavior
current security limitations

Explicitly state that Goal 010 does NOT provide sandboxing or cryptographic trust.

12. Dependencies

Prefer:

Rust standard library
existing project dependencies

Do not add a dependency unless genuinely required.

If a new dependency is proposed:

identify it
explain why existing functionality is insufficient
assess security and maintenance implications
STOP for architectural approval if materially significant
13. Verification

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

If CLI vam run is implemented:

cargo run -- run <controlled-test-package>

Use a controlled test package.

Do not execute arbitrary system commands during verification.

14. Definition of Done

Goal 010 is complete when:

Executor exists as a clear architectural boundary
RuntimeContext is required for execution
a valid installed package can execute its entrypoint
execution uses the Runtime working directory
execution uses the Runtime environment
standard streams behave as documented
process results are represented correctly
non-zero exits are handled correctly
spawn failures are controlled
execution does not use implicit shell construction
Runtime path validation cannot be bypassed
privilege escalation is not performed
focused tests pass
full test suite passes
fmt/check/clippy pass
no unjustified dependencies are added
documentation reflects actual behavior
final diff is scoped and clean
implementation is committed using the required convention
15. Git

Branch:

goal/010-package-executor-foundation

Commit convention:

goal/010-package-executor-foundation: <subject>

Do not modify unrelated goals.

Do not rewrite history.

Do not force-push.

16. Final Audit Report

At completion, report:

GOAL 010 FINAL AUDIT

Status:
Architecture:
Executor:
Process spawning:
Runtime integration:
Entrypoint:
Working directory:
Environment:
Process result:
CLI integration:
Dependencies:
Tests:
Verification:
Security:
Documentation:
Git:
Concerns:
Next:

Stop after the final audit.
