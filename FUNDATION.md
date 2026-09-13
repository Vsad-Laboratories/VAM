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
