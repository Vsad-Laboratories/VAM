Implementation boundaries for VAM foundation:

- main: Thin process entry point. Only calls vam::run and handles exit code.
- lib: Application initialization, CLI dispatch, and module coordination. Not a god module.
- cli: Command-line argument interpretation and dispatch. No business logic.
- error: Coherent error type for the foundation. Expandable by future subsystems.
- config: Minimal configuration boundary. Initialized safely.
- log: Minimal diagnostic boundary. Initialized safely.
- system: System Interface boundary owning host-system interaction (the `args` entry point that reads process command-line arguments).
- package: Package model boundary owning the in-memory manifest representation and manifest validation.

No external dependencies. All functionality provided by the Rust standard library.

## Error and result contract

- `ErrorKind` categorizes failures: `Usage` for expected user/input mistakes, `Internal` for unexpected application failures.
- `Error` carries a kind, a concise user-facing message, and an optional chained source.
- User-facing output never echoes raw user input and never exposes filesystem paths, secrets, or implementation details.
- `Display` renders the user-facing message. `Debug` includes kind, message, and whether a source is present, but omits source details.
- `pub type Result<T>` is the project-wide result convention, re-exported from the crate root.
- Errors are created at the boundary where the failure is meaningfully understood: CLI creates `Usage` errors, application/core propagates them, internal failures use `Internal`.

## System Interface contract

- The System Interface (`system` module) owns VAM's interaction with the host operating system. It isolates host-system operations from VAM Core so OS details do not leak into application coordination.
- `system::args` reads host-provided process command-line arguments, returning the program path as the first element (mirroring `std::env::args`). It performs no shell expansion, validation, or parsing.
- `run()` reads arguments through `system::args` (the host boundary) and delegates to `run_with_args` (the explicit-argument, testable seam). Tests use `run_with_args` to avoid environment dependence.
- `args` is infallible: the host always provides arguments and `std::env::args` replaces non-UTF-8 bytes rather than erroring. When a later, fallible System Interface operation is added, OS failures map to `ErrorKind::Internal`; no new error taxonomy is introduced.
- Current scope (Goal 004): argument reading only. Command/process spawning, filesystem inspection, package-manager interaction, service management, and privileged operations are deferred until genuinely required and are not implemented here.
- Process termination remains the responsibility of the thin `main` entry point; stdout/stderr output remains the responsibility of the CLI presentation layer and the diagnostics boundary.
- The System Interface does not own CLI argument parsing, UI presentation, package policy, configuration, security policy, or package/runtime lifecycle.
- No external dependencies are used; the Rust standard library is sufficient for the current scope.

## Diagnostics / logging contract

- `Diagnostics` is the operational logging boundary. Other modules emit events through its public interface; they must not duplicate logging infrastructure.
- `LogLevel` defines the minimum useful levels: `Debug`, `Info`, `Warn`, `Error`.
- `Diagnostics::new()` initializes the logging boundary safely and returns `Result<Self>`.
- `Diagnostics` methods emit operational diagnostic messages: `debug()`, `info()`, `warn()`, `error()`.
- `Diagnostics::log_error(&Error, context)` logs an error with operational context using the user-facing message only. Source details are never emitted by the logging boundary.
- Errors and logs are distinct: `Error` propagates failures through `Result`; logs record operational context.
- Output routing: `Debug`/`Info` go to stdout, `Warn`/`Error` go to stderr.
- Logging does not introduce external dependencies. Output goes directly to stdout/stderr.
- No sensitive data is logged: secrets, credentials, raw environment variables, sensitive configuration values, and unnecessary absolute filesystem paths are not emitted by the logging boundary.
- If logging itself encounters an I/O error, the failure is handled internally and does not propagate.

## Package model contract

- The `package` module is the package-domain boundary of VAM Core. It owns the in-memory package representation and manifest validation, not package persistence or execution.
- `PackageId` models the `<developer>.<name>` composite identity (see Package.md, OD-03). `version` and `release` identify a build but are not part of the composite identity.
- `Manifest` holds the §4.1 required fields: `name`, `developer`, `version`, `release`, `description`, `purpose`, `package_type`. The optional `configuration` field is not modeled (configuration engine is out of scope); `dependencies`, `capabilities`, and `integrity` are out of scope and not represented here.
- `Manifest::validate` and `PackageId::validate` enforce required-field presence (non-empty after trim) and report failures through the existing `Error`/`Result` model as `ErrorKind::Usage` — invalid manifests are expected, correctable input problems, not internal failures.
- Validation messages name only the offending field via fixed templates; field values are never echoed, so sensitive data is not exposed.
- `Manifest::package_id` derives the identity from `developer` and `name`.
- No TOML parsing or `.vampkg` loading exists at this foundation; the manifest format is TOML (Package.md, OD-02) and is parsed by a future archive/repository layer. The current goal establishes only the in-memory model and validation.
- No external dependencies are used; the Rust standard library is sufficient for the current scope.
