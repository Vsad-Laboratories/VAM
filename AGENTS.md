# AGENTS.md — VAM Engineering Control Layer

**Project:** VAM — VSAD Arch Manager
**Organization:** VSAD (Vortex Systems and Defenses)
**Status:** Specification Phase — no source code exists yet
**Language:** Rust (native compiled binary)

---

## 1. VAM Identity and Purpose

VAM is a lightweight, secure, extensible utility platform for Arch Linux and Arch-based systems.

**VAM is:**
- A unified CLI and TUI for managing VAM Packages
- A package ecosystem with controlled runtime
- A system-management utility platform

**VAM is NOT:**
- A replacement for pacman
- A replacement for the Arch Linux package system
- A generic app store, desktop environment, GUI manager
- A web app, Electron app, telemetry platform
- A universal package manager for every ecosystem

---

## 2. Architecture Boundaries

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
        └── VAM Runtime
```

**Boundary rules:**
- CLI and TUI are thin presentation layers over VAM Core
- All business logic lives in VAM Core
- VAM Core does NOT contain package-specific logic
- VAM Core does NOT contain UI rendering
- VAM Core does NOT contain network protocols (delegated to repository layer)

---

## 3. Rust as Implementation Language

VAM is implemented in Rust. This is a hard constraint.

**Avoid:**
- Electron, web UI runtimes, unnecessary daemons
- Unnecessary databases, heavyweight dependencies
- Telemetry of any kind

**Aspirational target:** ~1 MB binary. Never sacrifice security, correctness, or maintainability for size.

**Rust libraries:** Not all frozen yet. TUI, serialization, networking, and supporting libraries selected during architecture phase.

---

## 4. Core Module Ownership

| Module | Owner | Responsibility |
|--------|-------|----------------|
| `vam_core` | Core team | Shared business logic, package lifecycle, manifest parsing, capability checking, privilege brokering |
| `vam_engine` | Core team | Package discovery, verification, installation, removal, extraction, execution, dependency handling |
| `vam_runtime` | Core team | Execution context between VAM Core and package code |
| `vam_config_engine` | Core team | Configuration schema parsing, validation, interface generation |
| `vam_system` | Core team | OS detection, system info, pacman interaction, systemd integration |
| `vam_cli` | Core team | Argument parsing, output formatting (thin layer) |
| `vam_tui` | Core team | Terminal rendering, user input (thin layer) |
| `vam_repo` | Core team | Repository interaction, download, caching |

---

## 5. VAM Engine vs VAM Runtime Distinction

**VAM Engine** = package mechanics (discovery, verification, installation, removal, extraction, execution, dependencies, lifecycle, repository interaction)

**VAM Runtime** = execution context for package code (environment, declared paths, capabilities, configuration, dependency context, package entrypoint)

The Engine manages packages. The Runtime executes them.

---

## 6. `.vampkg` Package Architecture

**Format:** tar.zst (tar with Zstandard compression)

**Internal structure:**
```text
SudoCleaner.vampkg
├── manifest.toml          # Package metadata (TOML)
├── payload/               # Executable scripts, binaries, resources
├── schema/                # Configuration schema (custom DSL)
└── integrity/             # Checksums, signatures
```

**Key rules:**
- `manifest.toml` is always at archive root
- Source is canonical; `.vampkg` is a build artifact
- Package identity: `<developer>.<name>` composite key
- Packages are shell-script based (not compiled binaries)
- Packages execute inside VAM Runtime, not as bare scripts

---

## 7. Security-First Development

Security is a first-class architectural concern. VAM reduces and controls package risk — absolute security claims are not made.

**Never reduce package installation to:**
```text
download random script → sudo bash → hope for the best
```

**Security architecture must be defined before the package repository is implemented.**

---

## 8. Capability and Privilege Boundaries

**Capability types:** path, network, privilege grants

**Privilege model:**
1. VAM handles privilege escalation (user interacts with VAM, not raw sudo)
2. VAM provides a controlled privileged-operation API
3. VAM itself does not simply run as root
4. Packages do NOT freely call sudo — they use VAM's privileged-operation API

**Runtime isolation:** Minimal isolation with capability checks. Full sandboxing is NOT used.

---

## 9. Configuration Ownership

Configuration is schema-driven using a custom DSL with `<spt>`/`<ept>` delimiters.

**Flow:** Package declares schema → VAM validates → VAM renders controls → User configures → Package receives configuration

**Ownership:** VAM owns configuration storage, validation, and interface. Packages declare requirements; VAM enforces scope.

---

## 10. Repository/Package Trust Boundaries

**Two repositories:**
- Main VAM Repository (VSAD-controlled, high trust)
- VAM Plugins Repository (community, variable trust)

**Trust rules:**
- Local `.vampkg` is NOT automatically trusted
- Official repository trust and local package trust are distinct
- Repository is Git-based (metadata, versions, integrity in Git)

---

## 11. CLI/TUI Separation from Core Logic

**CLI prefix system:**
| Prefix | Domain |
|--------|--------|
| `i` | Info / Standard |
| `p` | Package Management |
| `c` | Configuration |
| `s` | Showing / Listing |
| `t` | Diagnostics |

**Commands:** `vam i-help`, `vam p-install`, `vam c-config`, `vam s-show-rollback`, `vam t-doctor`

**Rule:** CLI and TUI are thin layers. All business logic lives in VAM Core. Never duplicate logic between CLI and TUI.

---

## 12. Testing Requirements

**Status:** OPEN DECISION (OD-23)

Testing and benchmarking requirements are not yet defined. When defined:
- All core modules must have unit tests
- Integration tests for package operations
- Security tests for capability validation
- Performance benchmarks for critical paths

---

## 13. Error Handling Requirements

**Status:** OPEN DECISION (OD-21)

Error and recovery model not yet defined. When defined:
- Errors include what went wrong, why, and what user can do
- Exit codes defined in CLI spec (0-9)
- Fail secure: failures deny by default

---

## 14. Logging/Audit Requirements

**Status:** OPEN DECISION (OD-22)

Logging and audit model not yet defined. When defined, must log:
- Package installations/removals
- Privilege escalations
- Capability grants
- Security violations
- Configuration changes

---

## 15. Dependency Discipline

**VAM dependencies:** Other VAM packages (ephemeral: auto-install, auto-remove; permanent: stay installed)

**Arch dependencies:** System packages/commands. VAM detects/checks but does NOT manage Arch package installation.

**Rule:** VAM does NOT become a second pacman. pacman remains responsible for Arch system packages.

---

## 16. No Duplicated Logic

CLI and TUI share the same VAM Core. Business logic is never duplicated between interfaces.

If you find yourself writing the same logic in both CLI and TUI, it belongs in VAM Core.

---

## 17. No God Modules

VAM Core is split into focused modules: Engine, Configuration Engine, System Interface, Runtime.

No single module should own everything. Each module has clear boundaries and responsibilities.

---

## 18. No Speculative Abstractions

Build what the specification requires. Do not invent abstractions for problems that don't exist yet.

Open decisions are marked as OPEN in specifications. Do not resolve them silently.

---

## 19. No Temporary Architecture Without Approval

All architectural decisions must be documented in the relevant specification.

Temporary workarounds require explicit approval and must be tracked for removal.

---

## 20. Module-by-Module Implementation

**Implementation sequence (from README.md):**
```text
Project Definition → Architecture → Security Architecture → Package Specification
→ Technology Selection → Minimal Core → CLI → TUI → Package Engine
→ Configuration Engine → Repository → Initial VAM Packages
→ Security Hardening → Distribution
```

Implement in order. Do not skip ahead. Each module must be complete before moving to the next.

---

## 21. Documentation Synchronization

When code changes affect behavior described in documentation, update the documentation in the same changeset.

Documentation must clearly distinguish implemented functionality from planned functionality.

Use "planned", "designed", "proposed", "in development" where appropriate.

---

## 22. Completion Gate

A module is complete when:
- All specified functionality is implemented
- Unit tests pass
- Integration tests pass (where applicable)
- Documentation is updated
- No unresolved TODOs for specified functionality

---

## 23. Developer Authority Over Unresolved Decisions

Open decisions (OD-*) are tracked in specification documents. The developer (VSAD) has authority to resolve them.

Do not resolve open decisions silently. When encountering an open decision:
1. Check the relevant specification for current status
2. If still OPEN, implement with a clear TODO and follow-up
3. If resolved, follow the resolution

**Current open decisions:** See Package.md Section 22, Architecture.md Section 17, Security.md Section 12, Repository.md Section 12, Configuration.md Section 12.

---

## Documentation Files

| File | Purpose |
|------|---------|
| `README.md` | Project overview and positioning |
| `Package.md` | Formal package specification |
| `Architecture.md` | Technical architecture |
| `Security.md` | Security model |
| `CLI.md` | Command-line reference |
| `TUI.md` | Terminal UI specification |
| `Repository.md` | Distribution model |
| `Configuration.md` | Configuration system |
| `AGENTS.md` | This file — engineering control layer |

---

## Contradictions Discovered

1. **README.md CLI commands** show `vam install`, `vam remove`, `vam help`, `vam version` without prefix system. CLI.md and Architecture.md define prefix-based commands (`vam p-install`, `vam i-help`, etc.). README.md should be updated to match the resolved prefix system.

2. **README.md Package Identity** says "The exact machine-readable package identifier and namespace is an open architectural decision." Package.md has resolved this to `<developer>.<name>` composite key. README.md should be updated.

3. **README.md Documentation links** reference `docs/TUI.md` but TUI.md is at root level. Links should be updated.

**Status:** All three contradictions resolved. README.md updated to match resolved specifications.

---

## 24. Git Workflow Discipline

**Status:** ACTIVE — Enforced from first commit.

### 24.1 Branch Rules

| Branch | Purpose | Protection |
|--------|---------|------------|
| `main` | Protected/releasable integration branch | Branch protection: no direct pushes, PR required |
| `goal/<number>-<short-name>` | Isolated implementation goal | No direct commits to `main` |

### 24.2 Branch Naming Convention

```text
goal/<number>-<short-name>
```

The prefix numbering represents implementation milestones starting from `goal/001-vam-foundation`.

**Completed specification/architecture work** (project definition, architecture, security architecture, package specification) has already been completed as documentation. These do not correspond to future implementation branches.

Implementation branch examples:
```text
goal/001-vam-foundation
goal/002-...
goal/003-...
```

Future goal numbering represents implementation milestones only, not historical documentation work.

### 24.3 Unresolved Repository Decision — Tool Directory Policy

The following directories are currently ignored in `.gitignore`:

- `.agents/`
- `.claude/`
- `.kilo/`
- `agent/`

**Open decision (OD-GIT-01):** Project-authoritative Kilo agents/skills may eventually need to be tracked in the repository, while local/session/cache state should remain ignored.

**Status:** OPEN — Requires developer approval before any change to the ignore rules. Do not remove or modify the current ignore rules without explicit developer decision.

---

### 24.4 Core Discipline Rules

1. `main` is the protected/releasable integration branch. No direct development on `main`.
2. Development work happens on isolated `goal/<number>-<short-name>` branches.
3. One implementation goal owns one primary development branch.
4. Independent agents must not concurrently modify the same implementation area.
5. Research/review agents should avoid modifying implementation files unless explicitly assigned.
6. No force-push on any branch.
7. No history rewriting on shared branches (`main`).
8. No unrelated changes inside a goal branch.
9. Commits must represent coherent logical changes.
10. Working tree must be clean before starting a new goal branch.
11. Every goal must pass the VAM completion gate (AGENTS.md §22) before merge to `main`.
12. Failing CI must be fixed, not bypassed.
13. Architectural changes require developer approval (development-discipline skill).
14. Git will eventually enforce CI as an independent verification layer.

### 24.5 Commit Convention

Commits on `goal/` branches must use the prefix format:

```text
goal/<number>-<short-name>: <subject>
```

Example:
```text
goal/06-cli: add i-help and i-version commands
```

The commit message subject must start with the goal prefix matching the current branch.

### 24.6 Merge Convention

- Merge to `main` via Pull Request (squash merge preferred).
- Each PR must reference its goal number.
- Completion gate (AGENTS.md §22) must be verified before merging.
- `main` must always be left in a buildable state.

### 24.7 Module-by-Module Sequence

Implementation follows the sequence defined in AGENTS.md §20. Each phase becomes a `goal/` branch when work begins:

```text
Project Definition → Architecture → Security Architecture → Package Specification
→ Technology Selection → Minimal Core → CLI → TUI → Package Engine
→ Configuration Engine → Repository → Initial VAM Packages
→ Security Hardening → Distribution
```
