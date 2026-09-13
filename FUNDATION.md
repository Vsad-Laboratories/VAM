Implementation boundaries for VAM foundation:

- main: Thin process entry point. Only calls vam::run and handles exit code.
- lib: Application initialization, CLI dispatch, and module coordination. Not a god module.
- cli: Command-line argument interpretation and dispatch. No business logic.
- error: Coherent error type for the foundation. Expandable by future subsystems.
- config: Minimal configuration boundary. Initialized safely.
- log: Minimal diagnostic boundary. Initialized safely.

No external dependencies. All functionality provided by the Rust standard library.
