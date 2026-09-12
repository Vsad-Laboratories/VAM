# VAM — VSAD Arch Manager

**A lightweight, secure, extensible utility platform for Arch Linux and Arch-based systems.**

> One lightweight binary. One unified interface. An extensible ecosystem of Arch utilities.

VAM (**VSAD Arch Manager**) is a native Linux utility platform developed by **VSAD (Vortex Systems and Defenses)**.

VAM provides a unified CLI and terminal user interface for discovering, installing, configuring, running, updating, and managing **VAM Packages** — utilities and applications developed or distributed as part of the VSAD software ecosystem.

VAM is designed to feel like a professional Linux engineering tool rather than a collection of unrelated shell scripts.

---

## What VAM Is

VAM is a platform for the VSAD utility ecosystem.

```text
                         VAM
                          │
             ┌────────────┴────────────┐
             │                         │
            CLI                       TUI
             │                         │
             └────────────┬────────────┘
                          │
                       VAM Core
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
     Engine          Configuration        System
        │               Engine          Interface
        │
        ├── Package discovery
        ├── Package verification
        ├── Package installation
        ├── Package removal
        ├── Package extraction
        ├── Package execution
        ├── Dependency handling
        ├── Lifecycle management
        ├── Repository interaction
        ├── Security
        └── VAM Runtime
```

CLI and TUI share the same underlying VAM Core. Business logic is never duplicated between interfaces.

---

## What VAM Is Not

VAM is **not**:

- A replacement for pacman
- A replacement for the Arch Linux package system
- A generic app store
- A desktop environment
- A window manager
- A GUI package manager
- A random collection of shell scripts
- A web application
- An Electron application
- A telemetry platform
- A universal package manager for every software ecosystem

---

## Why VAM Exists

Linux already has excellent system and package-management tools.

VAM does **not** attempt to replace them.

Instead, VAM provides a unified platform for VSAD-built utilities that would otherwise exist as separate programs, scripts, or tools.

Rather than:

```text
utility 1
utility 2
utility 3
utility 4
utility 5
```

VAM provides:

```text
                    VAM
                     │
          ┌──────────┼──────────┐
          ▼          ▼          ▼
       Utility    Utility     Utility
```

The goal is a small, coherent platform with a consistent user experience, configuration model, lifecycle model, and security architecture.

---

## Design Philosophy

VAM should be:

- **lightweight** — small binary size, memory usage, dependencies, and operational complexity
- **fast** — responsive on old and new hardware alike
- **professional** — serious engineering software, not a toy
- **maintainable** — clean architecture, clear boundaries, deliberate design
- **security-conscious** — integrity, authenticity, least privilege, explicit trust
- **offline-capable** — fully functional without internet where possible
- **extensible** — new packages without modifying VAM Core internals
- **keyboard-first** — TUI navigable without a mouse
- **native** — compiled Rust binary, not web or Electron
- **resource-efficient** — suitable for old hardware without being designed exclusively for it

The complexity should exist in the engineering and architecture, **not** in unnecessary runtime dependencies or a bloated UI.

The aspirational binary size target is around 1 MB, but this is **not** a hard constraint. Never sacrifice security, correctness, maintainability, or reliability merely to hit a size target.

---

## Core Principles

### One Core

The CLI and TUI use the same underlying VAM Core.

Business logic is never duplicated between interfaces.

### Least Privilege

VAM should perform operations as an unprivileged user whenever possible.

Privileged operations should be narrowly scoped and explicitly controlled. Packages should **not** freely call `sudo`. VAM itself handles privilege escalation through a controlled privileged-operation API.

### Declarative Configuration

Packages describe their configuration requirements through a schema.

VAM validates those declarations and provides a consistent configuration interface across all packages.

### Secure by Design

Package acquisition, installation, configuration, and execution must be designed around integrity, authenticity, least privilege, safe process execution, and explicit trust.

### Transparent

VAM should make important operations understandable to the user.

### Private

VAM does not require telemetry, advertising, tracking, or user profiling.

### Small Core, Growing Ecosystem

The VAM core remains lightweight while the package ecosystem grows independently.

---

## What Is a VAM Package?

A VAM Package is a logical software unit that integrates with the VAM platform.

It is **not** simply an existing Arch package or a renamed shell script.

### Key Distinctions

```text
Logical Package:    SudoCleaner
Artifact:           SudoCleaner.vampkg
Source:             SudoCleaner/
Runtime Install:    VamRuntims/SudoCleaner/
```

These are distinct concepts. The `.vampkg` file is a real VAM-defined package container — not merely a renamed script.

### What a Package May Contain

A `.vampkg` package artifact may include:

- manifest (machine-readable package description)
- executable payload
- shell scripts
- configuration schema
- package files and directories
- metadata
- other declared resources

### What a Package May Provide

- system utilities
- diagnostics
- maintenance tools
- configuration-driven utilities
- automation
- networking utilities
- storage management
- security or privacy utilities
- other VSAD-developed Linux applications

### Package Identity

Every package has a human-readable display name:

```text
SudoCleaner 1.0.2 Stable
```

But this display string is **not** the complete machine identity. Every package has a `developer` field in its manifest. Packages created by separate developers with the same name and version are **not** the same package.

The machine-readable package identifier is the `<developer>.<name>` composite key:

```toml
developer = "vsad"
name = "sudo-cleaner"
```

Machine identity: `vsad.sudo-cleaner`

### Official Packages

The initial ecosystem intends to contain five main evergreen/official packages:

- **vam-clean** — system cleanup
- **vam-update** — update management
- **vam-doctor** — diagnostics and health checks
- **vam-storage** — storage management
- **vam-network** — network utilities

These are candidate examples, not necessarily the final five. Official packages should be useful, stable, general-purpose, Arch-relevant, lightweight, safe, maintainable, and independent from any user's personal desktop configuration.

---

## The VAM Engine

The VAM Engine is responsible for package mechanics. It is a core subsystem of VAM Core.

```text
VAM Engine
├── Package discovery
├── Package verification
├── Package installation
├── Package removal
├── Package extraction
├── Package execution
├── Dependency handling
├── Lifecycle management
├── Repository interaction
└── Runtime integration
```

Package-specific business logic belongs to the package. VAM Core defines the package contract and execution/management infrastructure. VAM Core does **not** contain special-case code for individual packages.

---

## The VAM Runtime

Packages are not simply executed as `bash script.sh`.

VAM provides a controlled **VAM Runtime** — the execution context between VAM Core and package code.

```text
VAM Runtime
│
├── environment
├── declared paths
├── declared capabilities
├── configuration
├── dependency context
└── package entrypoint
```

The package declares what it needs. VAM validates and establishes the runtime context. The package entrypoint executes inside that controlled context.

The exact sandbox mechanism, process model, environment rules, filesystem restrictions, and syscall restrictions are open architectural decisions.

---

## Package Trust and Runtime Safety

VAM uses two related but distinct concepts:

**Package Trust** — source authenticity, repository trust, signatures, hashes/checksums, integrity, version verification.

**Runtime Safety** — declared capabilities, controlled privilege, execution boundaries, filesystem/path restrictions where practical, safe argument handling, controlled privileged operations.

Metadata alone does **not** guarantee a package is safe. VAM's security architecture is designed to reduce and control package risk, not to make absolute claims of invulnerability.

---

## Offline Philosophy

VAM should work offline wherever possible.

**Offline operations** include:

- launching TUI and CLI
- viewing installed packages
- package information
- configuration
- local diagnostics
- local system management
- local package operations

**Internet required** only for:

- repository search
- remote package discovery
- downloading
- updates
- fetching remote source

---

## CLI

VAM provides a command-line interface alongside its TUI.

Commands use a single-letter prefix system:

```bash
vam                         # launch TUI

# Info / Standard (i prefix)
vam i-help                  # show help
vam i-version               # show version
vam i-list                  # list installed packages
vam i-search "query"        # search packages
vam i-info "package"        # show package info

# Package Management (p prefix)
vam p-install "package"     # install package
vam p-remove "package"      # remove package
vam p-update                # update packages
vam p-run "package"         # run package
vam p-dev-install "path"    # dev mode install

# Configuration (c prefix)
vam c-config "package"      # configure package

# Showing / Listing (s prefix)
vam s-show-rollback "pkg" "versions"  # show rollback versions

# Diagnostics (t prefix)
vam t-doctor                # run diagnostics
```

See [CLI Reference](CLI.md) for complete command documentation.

---

## TUI

Running `vam` launches the primary VAM terminal user interface.

Navigation:

```text
↑ ↓     navigate
ENTER   select
ESC     back
/       search
q       quit
```

The TUI is designed around:

- keyboard-first navigation
- minimal visual clutter
- clear information hierarchy
- fast startup
- low resource usage
- contextual help
- consistent package management
- professional technical instrumentation aesthetics

The interface is intentionally designed as a Linux engineering console rather than a desktop application.

The exact TUI hierarchy is not yet frozen. The TUI must not be tightly coupled to any user's personal desktop configuration.

---

## System Management Domains

VAM is intended to cover domains such as:

- System Information
- Diagnostics
- Storage
- Network
- Updates
- Cleanup
- Security
- Privacy
- Backup
- Services
- Processes
- Logs
- Hardware
- Configuration

Not every domain automatically becomes a top-level TUI category. The UI uses sensible hierarchy to avoid clutter.

---

## Supported Systems

VAM is primarily designed for:

- Arch Linux
- compatible Arch-based distributions

Arch Linux is the primary development target.

Compatibility with other Arch-based distributions will depend on system capabilities, available tooling, package-management behavior, and distribution-specific differences.

VAM should detect relevant system capabilities rather than blindly assuming that every Arch-based system behaves identically.

---

## Technology

VAM is being designed in **Rust**. VAM is a native compiled binary.

The technology stack will be selected based on:

- correctness
- security
- maintainability
- binary size
- memory usage
- Linux compatibility
- dependency footprint
- developer experience

The exact Rust libraries and frameworks have not all been frozen yet. The final TUI, serialization, networking, and supporting libraries will be selected during the architecture phase.

VAM avoids:

- Electron
- unnecessary daemons
- unnecessary databases
- web UI runtimes
- telemetry
- unnecessary heavyweight dependencies

---

## Visual and Brand Direction

VAM/VSAD visual identity:

- VantaBlack primary
- white/light gray secondary elements
- purple accent
- subtle status colors
- aerospace/space/astronautical engineering feel
- mission-control/instrumentation aesthetic
- futuristic but professional
- minimal
- sophisticated
- technical

The product should feel like serious engineering software.

Avoid:

- generic hacker aesthetics
- excessive neon
- gamer styling
- childish UI
- unnecessary animations
- clutter

---

## Lightweight Architecture

The core objective:

```text
                VAM CORE
             small + native
                    │
       ┌────────────┼────────────┐
       ▼            ▼            ▼
    Engine      Config        System
                Engine       Interface
       │
       ▼
  VAM Runtime
       │
       ▼
  VAM Packages
```

The VAM core should not contain package-specific business logic. Packages communicate with VAM through defined interfaces and specifications.

---

## Security

VAM is a system-management platform, so security is a first-class architectural concern.

The project will address:

- package authenticity
- package integrity
- cryptographic verification
- repository trust
- artifact verification
- safe extraction
- path traversal prevention
- command injection prevention
- safe argument handling
- privilege separation
- least-privilege execution
- package capability requirements
- downgrade considerations
- auditability

VAM must never reduce package installation to:

```text
download random script
↓
sudo bash
↓
hope for the best
```

Security architecture will be defined before the package repository is implemented.

---

## Privacy

VAM does not require telemetry.

The project does not intend to include:

- advertising
- user profiling
- mandatory analytics
- unnecessary tracking

Network communication is limited to functionality required by VAM, such as package metadata and artifact retrieval.

---

## Multi-User

Initial VAM implementation is single-user oriented.

However, the architecture avoids making future multi-user support impossible. A complicated multi-user system is not planned for V1 unless explicitly decided later.

---

## Development Status

VAM is currently in the **architecture and specification phase**.

No source code has been committed yet. The repository contains project definition and specification documents.

The planned development sequence:

```text
Project Definition
        ↓
Architecture
        ↓
Security Architecture
        ↓
Package Specification
        ↓
Technology Selection
        ↓
Minimal Core
        ↓
CLI
        ↓
TUI
        ↓
Package Engine
        ↓
Configuration Engine
        ↓
Repository
        ↓
Initial VAM Packages
        ↓
Security Hardening
        ↓
Distribution
```

The roadmap may change as architectural decisions are validated through implementation and testing.

---

## Installation

### Development

During development, VAM can be built directly from source using Rust tooling.

```bash
cargo build --release
```

The exact developer installation workflow may change as the project develops.

### Production

The intended mature installation experience is:

```bash
sudo pacman -S vam
```

Distribution through the Arch ecosystem, including potential AUR or repository distribution, will be addressed after VAM reaches an appropriate level of stability and security.

---

## Quick Start

Once installed:

```bash
vam             # launch the VAM TUI
vam i-help      # CLI usage
vam i-version   # version information
```

---

## Documentation

VAM documentation will be developed alongside the implementation.

Planned documentation:

- [Package Specification](Package.md)
- [TUI Specification](TUI.md)
- [Architecture](Architecture.md)
- [Configuration](Configuration.md)
- [Repository Specification](Repository.md)
- [Security Model](Security.md)
- [CLI Reference](CLI.md)
- [Package Development](PackageDevelopment.md)
- [Contributing Guide](CONTRIBUTING.md)
- [Engineering Control Layer](AGENTS.md)

Documentation describes actual VAM behavior and clearly distinguishes implemented functionality from planned functionality.

---

## Project Philosophy

VAM is built around a simple idea:

> **A small core should provide a strong foundation for a serious ecosystem.**

Every subsystem should earn its existence.

The project prioritizes:

```text
Correctness
Security
Maintainability
Simplicity
Performance
UX
Extensibility
```

while balancing these goals according to the requirements of each subsystem.

---

## VSAD

**VAM is a VSAD software project.**

**VSAD**
Vortex Systems and Defenses

> **We Build the Future.**

---

## License

License information will be added when the project's licensing decision is finalized.
