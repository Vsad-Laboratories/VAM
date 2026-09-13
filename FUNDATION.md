Implementation boundaries for VAM foundation:

- main: Thin process entry point. Only calls vam::run and handles exit code.
- lib: Application initialization, CLI dispatch, and module coordination. Not a god module.
- cli: Command-line argument interpretation and dispatch. No business logic.
- error: Coherent error type for the foundation. Expandable by future subsystems.
- config: Minimal configuration boundary. Initialized safely.
- log: Minimal diagnostic boundary. Initialized safely.

No external dependencies. All functionality provided by the Rust standard library.

## Error and result contract

- `ErrorKind` categorizes failures: `Usage` for expected user/input mistakes, `Internal` for unexpected application failures.
- `Error` carries a kind, a concise user-facing message, and an optional chained source.
- User-facing output never echoes raw user input and never exposes filesystem paths, secrets, or implementation details.
- `Display` renders the user-facing message. `Debug` includes kind, message, and whether a source is present, but omits source details.
- `pub type Result<T>` is the project-wide result convention, re-exported from the crate root.
- Errors are created at the boundary where the failure is meaningfully understood: CLI creates `Usage` errors, application/core propagates them, internal failures use `Internal`.

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
