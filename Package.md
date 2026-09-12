# VAM Package Specification

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the formal specification for the VAM Package system.

---

## 1. Package Concept

A **VAM Package** is the logical software unit in the VAM ecosystem.

A package is **not**:

- a renamed shell script
- an Arch package
- a raw executable
- a collection of files without structure

A package **is** a structured software unit that:

- integrates with the VAM platform
- declares metadata, capabilities, and configuration
- executes inside the VAM Runtime
- follows the VAM package lifecycle
- is managed by the VAM Engine

---

## 2. Core Distinctions

The VAM Package system distinguishes between several related concepts:

| Concept | Description | Example |
|---------|-------------|---------|
| **Logical Package** | The abstract software unit | `SudoCleaner` |
| **Source** | Developer-authored package source tree | `SudoCleaner/` |
| **Build Artifact** | Generated distributable container | `SudoCleaner.vampkg` |
| **Manifest** | Machine-readable package description | contained within `.vampkg` |
| **Runtime Installation** | Installed package in managed location | `VamRuntims/SudoCleaner/` |

These concepts must remain distinct throughout the system.

---

## 3. The `.vampkg` Artifact

The `.vampkg` file is the distributable package artifact/container.

It is **not** a renamed script. It is a real VAM-defined package container.

### 3.1 Contents

A `.vampkg` may contain:

- **manifest** — machine-readable package description
- **executable payload** — the package's primary executable or script
- **shell scripts** — supporting scripts
- **configuration schema** — declares configuration requirements
- **package files and directories** — supporting resources
- **metadata** — version, developer, dependencies, etc.
- **integrity data** — checksums, signatures
- **other declared resources** — as specified by the package

### 3.2 Build Model

Source is canonical. Generated `.vampkg` files are release/build artifacts.

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

The generated `.vampkg` is **not** the canonical source of truth.

### 3.3 Container Format

The `.vampkg` file is a **tar.zst** archive (tar with Zstandard compression).

Zstandard is Arch's native compression (used by pacman). It provides fast compression/decompression and good ratios. Rust support is available via the `zstd` crate with no external dependencies required.

### 3.4 Internal Structure

```text
SudoCleaner.vampkg (tar.zst archive)
├── manifest.toml          # Package metadata (TOML)
├── payload/               # Executable scripts, binaries, resources
│   └── sudo-cleaner.sh
├── schema/                # Configuration schema (custom DSL)
│   └── config.dsl
└── integrity/             # Checksums, signatures
    └── checksums.sha256
```

- `manifest.toml` is always at the archive root
- `payload/` contains the executable code and resources
- `schema/` contains the configuration schema (if applicable)
- `integrity/` contains verification data

---

## 4. Package Metadata

Every `.vampkg` contains a manifest with metadata.

### 4.1 Confirmed Metadata Fields

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Human-readable package name |
| `developer` | Yes | Package author/developer identity |
| `version` | Yes | Package version |
| `release` | Yes | Package release identifier |
| `description` | Yes | Brief description |
| `purpose` | Yes | Package purpose/category |
| `dependencies` | Yes | Required packages (VAM and/or Arch) |
| `capabilities` | Yes | Required system capabilities |
| `configuration` | No | Configuration schema reference |
| `integrity` | Yes | Integrity verification data |
| `package_type` | Yes | Package type classification |

### 4.2 Developer Identity

The `developer:` field is a confirmed required metadata field.

Developer identity is part of package identity. Two packages with the same name, version, and release from different developers are **not** the same package.

The developer identity format is the `developer` field in `manifest.toml`:

```toml
developer = "vsad"
```

### 4.3 Manifest Format

The manifest format is **TOML**.

The manifest must be:

- machine-readable
- human-inspectable
- versioned
- extensible
- formally validated

---

## 5. Package Identity

### 5.1 Human-Readable Naming

Display convention:

```text
<package name> <version> <release>
```

Example:

```text
SudoCleaner 1.0.2 Stable
```

### 5.2 Machine-Readable Identity

The display string is **not** the complete machine identity.

Every package has a `developer:` metadata field. Packages from separate developers must be uniquely identifiable even if they share a name and version.

```text
Developer A / SudoCleaner / 1.0.2 / Stable
Developer B / SudoCleaner / 1.0.2 / Stable
```

These are **not** the same package.

The machine-readable package identifier is the `<developer>.<name>` composite key:

```toml
developer = "vsad"
name = "sudo-cleaner"
```

Machine identity: `vsad.sudo-cleaner`

### 5.3 Identity Fields

The machine identity consists of:

- developer identifier (from `developer` field in manifest)
- package name (from `name` field in manifest)

The `version` and `release` are metadata fields but not part of the package's unique identity — they represent a specific build of the package.

---

## 6. Package Validation

Validation should eventually cover:

- [ ] manifest validity
- [ ] package structure
- [ ] payload integrity
- [ ] permission correctness
- [ ] dependency resolution
- [ ] capability declarations
- [ ] configuration schema validity
- [ ] integrity information

Exact validation rules and enforcement are **OPEN DECISIONS**.

---

## 7. Package Inspection

An advanced user **must** be able to inspect a local `.vampkg` without installing it.

Command:

```bash
vam i-info ./SudoCleaner.vampkg
```

### 7.1 Inspectable Information

- name
- developer
- version
- release
- description
- purpose
- dependencies
- capabilities
- configuration schema
- integrity information
- package type
- contained files/resources

Exact output format is an **OPEN DECISION**.

---

## 8. Package Installation

### 8.1 Local Installation

Users **must** be able to install local `.vampkg` files.

Command:

```bash
vam p-install ./SudoCleaner.vampkg
```

A local `.vampkg` is **not** automatically trusted merely because it is local. Official repository trust and local package trust are distinct concepts.

### 8.2 Repository Installation

Users can install packages from configured VAM repositories.

Command:

```bash
vam p-install SudoCleaner
```

### 8.3 Installation Locations

Package installation location:

```text
/usr/lib/vam/packages/
```

Installed package runtime data uses the project concept **VamRuntims** at:

```text
/usr/lib/vam/runtims/
```

---

## 9. Developer Mode

VAM **must** support development installation/testing from unpacked source.

Command:

```bash
vam p-dev-install ./SudoCleaner/
```

This allows developers to test packages without publishing or rebuilding the final `.vampkg` every time.

Exact developer workflow, source layout expectations, and runtime behavior in dev mode are **OPEN DECISIONS**.

---

## 10. Dependencies

### 10.1 VAM Package Dependencies

VAM packages can depend on other VAM packages.

```text
Package A → requires Package B
```

**Ephemeral dependencies** are auto-installed for the operation and auto-removed afterward. This is the default behavior for dependencies that are not explicitly declared as permanent.

**Permanent dependencies** stay installed after the operation.

The distinction between ephemeral and permanent dependencies is declared in the package manifest.

### 10.2 Arch Package Dependencies

VAM packages can also depend on normal Arch packages/commands.

VAM should detect and check these dependencies where appropriate.

VAM does **not** become a second pacman. pacman remains responsible for Arch system packages. VAM packages may declare required Arch packages as part of their dependency declarations, but VAM does not manage Arch package installation itself.

### 10.3 Dependency Resolution

The dependency resolution model (resolution order, conflict handling, version constraints) is an **OPEN DECISION**.

---

## 11. Capabilities

Packages declare **capabilities** — what the package requires from the system or runtime to function.

Capabilities are part of the security and runtime model. They allow VAM to:

- validate that a package can run before execution
- enforce least-privilege execution
- declare required system resources
- inform the user of requirements

### 11.1 Capability Types

The capability model uses **path/network/privilege grants**:

| Capability Type | Description | Example |
|----------------|-------------|---------|
| **path** | Filesystem access | `path = "/var/log"` |
| **network** | Network access | `network = "outbound"` |
| **privilege** | Elevated operations | `privilege = "systemctl"` |

### 11.2 Capability Declaration

Capabilities are declared in the package manifest:

```toml
[[capabilities]]
type = "path"
access = "/var/log"
mode = "read"

[[capabilities]]
type = "network"
access = "outbound"
protocol = "https"
```

---

## 12. VAM Runtime

### 12.1 Concept

Packages are not executed as bare scripts. VAM provides a controlled **VAM Runtime**.

The runtime is the execution context between VAM Core and package code.

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

### 12.2 Runtime Contract

The package declares what it needs. VAM validates and establishes the runtime context. The package entrypoint then executes inside that controlled context.

### 12.3 Runtime Isolation

VAM uses **minimal isolation with capability checks**.

The runtime provides:
- Environment control
- Declared path restrictions
- Capability validation
- Privilege boundary enforcement

Full sandboxing (Landbubble, bubblewrap, namespaces) is **not** used. Instead, VAM relies on capability declarations and validation to enforce runtime safety.

---

## 13. Privilege Model

VAM uses a combined privilege model:

1. **VAM handles privilege escalation** — the user interacts with VAM, not raw sudo
2. **VAM provides a controlled privileged-operation API** — packages request elevated operations through defined interfaces
3. **VAM itself does not simply run as root** — privilege is requested narrowly
4. **Packages do not freely call sudo** — they use VAM's privileged-operation API

### 13.1 Privilege Broker

Privileged operations are performed through **direct syscalls through VAM broker**.

Packages request elevated operations through the VAM API. VAM validates the request against the package's declared capabilities and either performs the operation or denies it.

---

## 14. Security Model

### 14.1 Package Trust

Package trust concerns:

- source authenticity
- repository trust
- signatures
- hashes/checksums
- integrity verification
- version verification
- controlled repositories

### 14.2 Runtime Safety

Runtime safety concerns:

- declared capabilities
- controlled privilege
- execution boundaries
- filesystem/path restrictions where practical
- safe argument handling
- controlled privileged operations

### 14.3 Trust Boundaries

Metadata alone does **not** guarantee a package is safe.

VAM's security architecture is designed to **reduce and control package risk**. Absolute security claims are not made.

---

## 15. Package Configuration

Configuration is schema-driven.

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

### 15.1 Schema Types

The configuration schema uses a **custom DSL**.

Schema types:

- boolean
- integer
- string
- enum
- path
- duration
- command
- toggle
- list

### 15.2 Schema Syntax

```dsl
[-pkg.config-]

<spt>
<journal_retention>
    type = "integer"
    default = 7
    unit = "days"
<ept>

<spt>
<cache_cleanup>
    type = "boolean"
    default = true
<ept>
```

Syntax rules:
- `[-pkg.config-]` — section header (required at top of config schema)
- `<spt>` / `<ept>` — block delimiters (required for every configuration entry)
- `<identifier>` — configuration key name
- `|` — statement terminator (optional, conventional, like `;` in C)
- Indentation is cosmetic, not significant

### 15.3 Configuration Scope

Configuration should eventually be constrained to declared/appropriate configuration areas. Packages should not be able to arbitrarily write configuration outside their declared scope.

---

## 16. Package Lifecycle

### 16.1 Lifecycle States

Known conceptual states:

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

### 16.2 State Transitions

Exact state transition rules and enforcement are **OPEN DECISIONS**.

### 16.3 Broken Package Recovery

A broken package should be recoverable. An advanced user may inspect/edit package source externally and then use a reload/retest workflow.

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

Exact reload/retest commands are **OPEN DECISIONS**.

---

## 17. Rollback

VAM **must** support rollback.

### 17.1 Retention Model

Several previous package versions should be retained.

Example:

```text
SudoCleaner
├── 1.0.3  ← current
├── 1.0.2
├── 1.0.1
└── 1.0.0
```

### 17.2 Retention Configuration

Retention should eventually be configurable.

The exact default retention count and configuration mechanism are **OPEN DECISIONS**.

---

## 18. Package Updates

VAM package updates are managed independently from normal Arch packages.

Conceptual flow:

```text
VAM
 ↓
check repository
 ↓
find update
 ↓
download package
 ↓
verify
 ↓
install/update
 ↓
retain rollback history
```

VAM does **not** replace pacman.

---

## 19. Repository Model

### 19.1 Two Repositories

VAM is intended to use two repositories:

| Repository | Purpose | Control |
|-----------|---------|---------|
| **Main VAM Repository** | VAM core, official/evergreen packages | Tightly controlled by VSAD |
| **VAM Plugins Repository** | External/community packages | Open with review process |

### 19.2 Main Repository

Contains:

- VAM core components
- Official/evergreen packages
- Tightly controlled by VSAD

### 19.3 Plugins Repository

Contains:

- External/community-developed packages
- Developers follow VAM package guidelines
- Contribution via pull requests
- Review/policy process

Critical/important packages receive stricter VSAD review.

### 19.4 Repository Backend

The repository is **Git-based**.

Package metadata, versions, compatibility information, integrity information, signatures, and artifacts are stored in Git repositories. This provides:

- version control for package metadata
- pull request workflow for contributions
- audit trail for changes
- distributed access

### 19.5 Developer Documentation

Package-development guidance will be provided through the VAM GitHub repository Wiki.

---

## 20. Package Competition

The ecosystem may allow multiple developers to submit implementations for the same general purpose.

### 20.1 Evaluation Dimensions

Performance alone **must not** determine the winner.

Evaluation dimensions:

- correctness
- security
- reliability
- performance
- resource usage
- package size
- startup time
- maintainability
- compatibility

### 20.2 Identity

Because developer identity is part of package metadata, separate developers must not be treated as identical packages merely because their names/purposes are similar.

The exact competition/replacement mechanism is an **OPEN DECISION**.

---

## 21. System Management Domains

VAM packages may target domains such as:

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

Not every domain automatically becomes a top-level TUI category. The UI uses sensible hierarchy.

---

## 22. Open Architectural Decisions

The following decisions remain unresolved and must be determined before implementation:

| ID | Decision | Status |
|----|----------|--------|
| OD-06 | Package trust/signature cryptographic model | OPEN |
| OD-11 | Dependency resolution model (order, conflicts, version constraints) | OPEN |
| OD-13 | Package lifecycle state transition rules | OPEN |
| OD-14 | Rollback retention default and configuration | OPEN |
| OD-17 | Package competition/replacement mechanism | OPEN |
| OD-20 | VAM core module boundaries | OPEN |
| OD-21 | Error and recovery model | OPEN |
| OD-22 | Logging and audit model | OPEN |
| OD-23 | Testing and benchmarking requirements | OPEN |
| OD-24 | Release and versioning strategy | OPEN |
| OD-25 | Package validation rules and enforcement | OPEN |
| OD-26 | Developer mode exact workflow | OPEN |

### Resolved Decisions

| ID | Decision | Resolution |
|----|----------|------------|
| OD-01 | `.vampkg` internal binary format and container structure | tar.zst |
| OD-02 | Manifest syntax | TOML |
| OD-03 | Machine-readable package identifier / namespace | `<developer>.<name>` composite key |
| OD-04 | Developer identity format | `developer` field in manifest.toml |
| OD-05 | VamRuntims exact filesystem placement | `/usr/lib/vam/runtims/` |
| OD-07 | Runtime isolation mechanism | Minimal isolation with capability checks |
| OD-08 | Capability declaration format and types | Path/network/privilege grants |
| OD-09 | Privileged-operation API design | Direct syscalls through VAM broker |
| OD-10 | Configuration schema syntax and storage layout | Custom DSL with `<spt>`/`<ept>` delimiters |
| OD-12 | Ephemeral dependency semantics | Auto-install, auto-remove |
| OD-15 | Repository backend, database, and API structure | Git-based |
| OD-18 | CLI command contract finalization | Prefix-based (i, p, c, s, t) |
| OD-19 | TUI hierarchy and navigation structure | Dashboard → categories → flat list with search |

---

## 23. Implementation Status

This specification defines the **designed** package system.

**Resolved design decisions:**

- `.vampkg` format: tar.zst
- Manifest format: TOML
- Package identity: `<developer>.<name>` composite key
- Container structure: manifest.toml, payload/, schema/, integrity/
- Runtime isolation: minimal with capability checks
- Capability model: path/network/privilege grants
- Privilege model: direct syscalls through VAM broker
- Configuration DSL: custom bracket syntax with `<spt>`/`<ept>` delimiters
- Dependencies: ephemeral (auto-install, auto-remove)
- Repository: Git-based
- VamRuntims: `/usr/lib/vam/runtims/`

**Not yet implemented:**

- No `.vampkg` builder exists
- No manifest parser exists
- No VAM Runtime implementation exists
- No package validation exists
- No repository infrastructure exists
- No package signing exists
- No configuration DSL parser exists

All implementation details should be traced back to this specification and updated as decisions are made and code is written.

---

## 24. Relationship to Other Specifications

This specification interacts with:

- **TUI Specification** — how packages are presented to the user
- **CLI Reference** — how packages are managed via command line
- **Security Model** — trust, signatures, runtime safety
- **Repository Specification** — how packages are distributed
- **Configuration Specification** — how package configuration works
- **Architecture Document** — how the VAM Engine and Runtime are structured
