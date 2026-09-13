# VAM Architecture

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the technical architecture of VAM — VSAD Arch Manager.

---

## 1. System Overview

VAM is a native Linux utility platform for Arch Linux and Arch-based systems.

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

---

## 2. Design Principles

| Principle | Description |
|-----------|-------------|
| **One Core** | CLI and TUI share the same VAM Core. Business logic is never duplicated. |
| **Small Core** | VAM Core remains lightweight. Package-specific logic belongs to packages. |
| **Native** | Compiled Rust binary. No web, Electron, or heavyweight runtimes. |
| **Least Privilege** | Operations execute as unprivileged user when possible. |
| **Offline-First** | Fully functional without internet where possible. |
| **Extensible** | New packages without modifying VAM Core internals. |

---

## 3. VAM Core

VAM Core is the central subsystem. It provides the shared business logic used by both CLI and TUI.

### 3.1 Core Responsibilities

```text
VAM Core
├── Package Engine
├── Configuration Engine
├── System Interface
└── VAM Runtime
```

### 3.2 Core Boundaries

VAM Core does **not** contain:

- package-specific business logic
- special-case code for individual packages
- UI rendering (CLI/TUI handle presentation)
- network protocols (delegated to repository layer)

VAM Core **does** contain:

- package lifecycle management
- manifest parsing and validation
- capability checking
- privilege brokering
- configuration schema parsing
- system information gathering
- dependency resolution interface

---

## 4. VAM Engine

The VAM Engine is responsible for package mechanics.

### 4.1 Engine Subsystems

```text
VAM Engine
├── Package Discovery
├── Package Verification
├── Package Installation
├── Package Removal
├── Package Extraction
├── Package Execution
├── Dependency Handling
├── Lifecycle Management
├── Repository Interaction
└── Security
```

### 4.2 Package Discovery

- Search local installed packages
- Search remote repositories
- Resolve package names to identifiers
- Handle version queries

### 4.3 Package Verification

- Validate manifest structure
- Check integrity checksums
- Verify package signatures
- Validate capability declarations
- Check dependency availability

### 4.4 Package Installation

```text
User request
    ↓
Verify package integrity
    ↓
Check capabilities
    ↓
Check dependencies
    ↓
Install ephemeral dependencies
    ↓
Extract .vampkg (tar.zst)
    ↓
Place files in /usr/lib/vam/packages/
    ↓
Place runtime data in /usr/lib/vam/runtims/
    ↓
Register package
    ↓
Remove ephemeral dependencies (if any)
```

### 4.5 Package Removal

```text
User request
    ↓
Check package state
    ↓
Check dependent packages
    ↓
Remove runtime data
    ↓
Remove package files
    ↓
Deregister package
    ↓
Clean up configuration (if requested)
```

### 4.6 Package Extraction

The `.vampkg` is a tar.zst archive.

Extraction process:

1. Decompress zstd stream
2. Extract tar archive
3. Validate manifest presence
4. Validate directory structure
5. Place files in appropriate locations

### 4.7 Package Execution

Packages execute inside the VAM Runtime.

```text
Package entrypoint
    ↓
VAM Runtime establishes context
    ↓
Environment configured
    ↓
Capabilities validated
    ↓
Configuration loaded
    ↓
Package code executes
```

---

## 5. VAM Runtime

The VAM Runtime is the execution context between VAM Core and package code.

### 5.1 Runtime Concept

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

### 5.2 Runtime Isolation

VAM uses **minimal isolation with capability checks**.

The runtime provides:

- **Environment control** — package receives declared environment variables
- **Path restrictions** — package can only access declared paths
- **Capability validation** — package capabilities are checked before execution
- **Privilege boundary** — privileged operations go through VAM broker

Full sandboxing is **not** used. VAM relies on capability declarations and validation.

### 5.3 Runtime Lifecycle

```text
1. Load package manifest
2. Validate capabilities
3. Check configuration
4. Establish environment
5. Set up path context
6. Execute package entrypoint
7. Monitor execution
8. Collect exit status
9. Clean up runtime context
```

---

## 6. Configuration Engine

The Configuration Engine manages package configuration schemas.

### 6.1 Configuration Flow

```text
Package declares configuration schema
        ↓
VAM validates schema
        ↓
VAM renders configuration controls
        ↓
User configures package
        ↓
Package receives/uses generated configuration
```

### 6.2 Schema Processing

1. Parse custom DSL schema file
2. Validate schema structure
3. Generate configuration interface
4. Validate user input against schema
5. Write configuration to appropriate location
6. Notify package of configuration changes

### 6.3 Configuration Storage

Configuration files are stored per-package.

The exact configuration file layout is an **OPEN DECISION**.

---

## 7. System Interface

The System Interface provides VAM with access to system information and operations.

### 7.1 System Information

- OS detection
- Arch version
- Kernel information
- Hardware information
- Disk usage
- Network status
- Package counts

### 7.2 System Operations

- Package management (via pacman interaction where needed)
- Service management (systemd integration)
- Log access
- Process information
- Hardware detection

### 7.3 Foundation Implementation (Goal 004)

The System Interface boundary is established as the `system` module, owning
host-system interaction so that OS details do not leak into VAM Core. Only
operations with a current, genuine requirement are implemented; the full
interface in sections 7.1 and 7.2 is planned capability, not current
functionality.

**Currently implemented (Goal 004):**

- `system::args` — reads the host process command-line arguments. This is the
  only host-system operation the current foundation requires. Command/process
  spawning, filesystem inspection, package-manager interaction, service
  management, privileged operations, hardware detection, and system
  information queries are deferred until genuinely required.

**Ownership and boundaries:**

- The System Interface owns host-system mechanics and translates OS failures
  into `ErrorKind::Internal` for operations that can fail.
- The System Interface does **not** own CLI argument parsing, UI presentation,
  package policy, configuration, security policy, or package/runtime lifecycle.
- Process termination remains the responsibility of the thin `main` entry
  point; stdout/stderr output remains the responsibility of the CLI
  presentation layer and the diagnostics boundary.
- No external dependencies back the System Interface; the Rust standard
  library is sufficient for the current scope.

Future operations will be added incrementally, each only when justified by an
actual VAM requirement.

---

## 8. CLI

The CLI is a command-line interface to VAM Core.

### 8.1 Prefix System

Commands use a single-letter prefix system:

| Prefix | Domain | Examples |
|--------|--------|----------|
| `i` | Info / Standard | `i-help`, `i-version`, `i-list` |
| `p` | Package Management | `p-install`, `p-remove`, `p-update` |
| `c` | Configuration | `c-config` |
| `s` | Showing / Listing | `s-show-rollback` |
| `t` | Diagnostics | `t-doctor` |

### 8.2 Command Reference

```bash
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

# Launch
vam                         # launch TUI (no arguments)
```

### 8.3 CLI Implementation

- CLI is a thin layer over VAM Core
- All business logic lives in VAM Core
- CLI handles argument parsing and output formatting
- CLI does not duplicate TUI logic

---

## 9. TUI

The TUI is a terminal user interface to VAM Core.

### 9.1 Hierarchy

```text
VAM TUI
├── Dashboard (home screen with categories)
│   ├── System
│   ├── Storage
│   ├── Network
│   ├── Security
│   └── ...
├── Package List (flat, searchable)
├── Search (/)
└── Installed Packages
```

### 9.2 Navigation

```text
↑ ↓     navigate
ENTER   select
ESC     back
/       search
q       quit
```

### 9.3 Design Principles

- keyboard-first
- minimal visual clutter
- clear information hierarchy
- fast startup
- low resource usage
- professional technical instrumentation aesthetics

### 9.4 TUI Implementation

- TUI is a thin layer over VAM Core
- All business logic lives in VAM Core
- TUI handles rendering and user input
- TUI does not duplicate CLI logic

---

## 10. Package Build Pipeline

### 10.1 Build Flow

```text
Developer Source
      ↓
Package Build
      ↓
Validate
      ↓
Integrity data
      ↓
.vampkg
      ↓
Release
```

### 10.2 Build Stages

1. **Source Collection** — gather package source files
2. **Manifest Generation** — create or update manifest.toml
3. **Schema Processing** — validate configuration schema
4. **Payload Assembly** — prepare executable payload
5. **Integrity Generation** — create checksums
6. **Archive Creation** — compress to tar.zst
7. **Validation** — verify the built package

### 10.3 Source vs Artifact

Source is canonical. The generated `.vampkg` is a release artifact.

The `.vampkg` is **not** the canonical source of truth.

---

## 11. Dependency Resolution

### 11.1 Dependency Types

| Type | Behavior |
|------|----------|
| **VAM Dependency** | Another VAM package |
| **Arch Dependency** | System package or command |
| **Ephemeral** | Auto-install, auto-remove |
| **Permanent** | Stay installed |

### 11.2 Resolution Process

```text
1. Parse dependency declarations
2. Check installed packages
3. Identify missing dependencies
4. Resolve version conflicts
5. Install ephemeral dependencies
6. Execute package operation
7. Remove ephemeral dependencies (if applicable)
```

### 11.3 Conflict Handling

The dependency conflict resolution model is an **OPEN DECISION**.

---

## 12. Lifecycle Management

### 12.1 Package States

```text
Available
    ↓
Downloaded
    ↓
Installed
    ↓
Configured
    ↓
Running / Disabled
    ↓
Update Available / Broken
    ↓
Removed
```

### 12.2 State Transitions

State transition rules are an **OPEN DECISION**.

### 12.3 Rollback

VAM retains several previous package versions.

```text
SudoCleaner
├── 1.0.3  ← current
├── 1.0.2
├── 1.0.1
└── 1.0.0
```

Rollback retention configuration is an **OPEN DECISION**.

---

## 13. Error and Recovery

### 13.1 Error Model

The error and recovery model is an **OPEN DECISION**.

### 13.2 Broken Package Recovery

A broken package should be recoverable.

Conceptual workflow:

```text
edit package externally
        ↓
reload package
        ↓
test/revalidate
        ↓
continue using package
```

VAM itself should **not** become an IDE or code editor.

---

## 14. Logging and Audit

The logging and audit model is an **OPEN DECISION**.

---

## 15. Testing and Benchmarking

Testing and benchmarking requirements are **OPEN DECISIONS**.

---

## 16. Release and Versioning

The release and versioning strategy is an **OPEN DECISION**.

---

## 17. Open Architectural Decisions

| ID | Decision | Status |
|----|----------|--------|
| OD-11 | Dependency resolution model (order, conflicts, version constraints) | OPEN |
| OD-13 | Package lifecycle state transition rules | OPEN |
| OD-14 | Rollback retention default and configuration | OPEN |
| OD-20 | VAM core module boundaries | OPEN |
| OD-21 | Error and recovery model | OPEN |
| OD-22 | Logging and audit model | OPEN |
| OD-23 | Testing and benchmarking requirements | OPEN |
| OD-24 | Release and versioning strategy | OPEN |
| OD-25 | Package validation rules and enforcement | OPEN |
| OD-26 | Developer mode exact workflow | OPEN |

---

## 18. Relationship to Other Specifications

This architecture interacts with:

- **Package Specification** — how packages are structured and managed
- **Security Model** — trust, capabilities, privilege
- **CLI Reference** — command-line interface
- **TUI Specification** — terminal user interface
- **Repository Specification** — package distribution
- **Configuration Specification** — package configuration
