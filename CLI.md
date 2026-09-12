# VAM CLI Reference

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the VAM command-line interface.

---

## 1. Overview

VAM provides a command-line interface alongside its TUI.

Running `vam` with no arguments launches the TUI. Running `vam` with arguments executes the specified command.

---

## 2. Command Structure

Commands use a single-letter prefix system:

```bash
vam <prefix>-<command> [arguments]
```

### 2.1 Prefix System

| Prefix | Domain | Description |
|--------|--------|-------------|
| `i` | Info / Standard | Information, listing, search |
| `p` | Package Management | Install, remove, update, run |
| `c` | Configuration | Package configuration |
| `s` | Showing / Listing | Display information |
| `t` | Diagnostics | Troubleshooting, health checks |

---

## 3. Commands

### 3.1 TUI Launch

```bash
vam
```

Launches the VAM TUI. No arguments required.

### 3.2 Info / Standard (i prefix)

#### `vam i-help`

Show help information.

```bash
vam i-help
```

Output: list of available commands and usage information.

#### `vam i-version`

Show VAM version information.

```bash
vam i-version
```

Output: VAM version, build information, Rust version.

#### `vam i-list`

List installed VAM packages.

```bash
vam i-list
```

Output: table of installed packages with name, version, status.

Options:

- `--all` — include all package states
- `--updates` — show only packages with available updates

#### `vam i-search`

Search for packages.

```bash
vam i-search "query"
```

Arguments:

- `query` — search string

Output: list of matching packages from configured repositories.

Options:

- `--local` — search only locally installed packages
- `--remote` — search only remote repositories
- `--limit <n>` — limit results

#### `vam i-info`

Show detailed package information.

```bash
vam i-info "package"
```

Arguments:

- `package` — package name or path to `.vampkg`

Output:

- name
- developer
- version
- release
- description
- purpose
- dependencies
- capabilities
- configuration
- integrity
- status

Options:

- `--raw` — show raw manifest data

### 3.3 Package Management (p prefix)

#### `vam p-install`

Install a package.

```bash
vam p-install "package"
```

Arguments:

- `package` — package name (from repository) or path to `.vampkg`

Output: installation progress, dependency resolution, success/failure.

Options:

- `--force` — force reinstall
- `--no-deps` — skip dependency installation
- `--dry-run` — show what would be installed without installing

#### `vam p-remove`

Remove an installed package.

```bash
vam p-remove "package"
```

Arguments:

- `package` — package name

Output: removal progress, success/failure.

Options:

- `--keep-config` — preserve configuration files
- `--force` — force removal

#### `vam p-update`

Update packages.

```bash
vam p-update
```

Without arguments: update all installed packages.

```bash
vam p-update "package"
```

With argument: update specific package.

Output: update progress, versions, success/failure.

Options:

- `--check` — check for updates without installing
- `--force` — force update

#### `vam p-run`

Run a package.

```bash
vam p-run "package"
```

Arguments:

- `package` — package name

Output: package execution output.

Options:

- `--args "..."` — pass arguments to package

#### `vam p-dev-install`

Install package from source directory (developer mode).

```bash
vam p-dev-install "path"
```

Arguments:

- `path` — path to package source directory

Output: development installation progress, success/failure.

Options:

- `--watch` — watch for source changes and reinstall

### 3.4 Configuration (c prefix)

#### `vam c-config`

Configure a package.

```bash
vam c-config "package"
```

Arguments:

- `package` — package name

Output: configuration interface (opens TUI configuration screen).

Options:

- `--show` — display current configuration without editing
- `--reset` — reset to default configuration
- `--set "key=value"` — set a configuration value directly

### 3.5 Showing / Listing (s prefix)

#### `vam s-show-rollback`

Show available rollback versions for a package.

```bash
vam s-show-rollback "package" "version range"
```

Arguments:

- `package` — package name
- `version range` — optional version range (e.g., "1.2.0 - 1.3.4")

Output: list of available versions with status.

Options:

- `--all` — show all available versions
- `--current` — highlight current version

### 3.6 Diagnostics (t prefix)

#### `vam t-doctor`

Run system diagnostics.

```bash
vam t-doctor
```

Output: diagnostic report covering:

- VAM installation status
- Package health
- Repository connectivity
- Configuration validity
- System compatibility

Options:

- `--fix` — attempt automatic fixes
- `--verbose` — show detailed output
- `--report` — generate diagnostic report file

---

## 4. Global Options

These options apply to all commands:

| Option | Description |
|--------|-------------|
| `--help` | Show help for command |
| `--version` | Show VAM version |
| `--quiet` | Suppress non-essential output |
| `--verbose` | Show detailed output |
| `--no-color` | Disable colored output |
| `--config <path>` | Use alternate configuration file |

---

## 5. Output Format

### 5.1 Standard Output

- Success messages: plain text
- Errors: prefixed with `error:`
- Warnings: prefixed with `warning:`
- Info: prefixed with `info:`

### 5.2 Tables

List commands output tables with:

- Column headers
- Aligned columns
- Truncation for long values

### 5.3 Colored Output

VAM uses colors for:

- Success: green
- Error: red
- Warning: yellow
- Info: blue
- Package names: bold
- Versions: cyan

Colors can be disabled with `--no-color`.

---

## 6. Error Handling

### 6.1 Exit Codes

| Code | Description |
|------|-------------|
| 0 | Success |
| 1 | General error |
| 2 | Package not found |
| 3 | Installation failed |
| 4 | Removal failed |
| 5 | Update failed |
| 6 | Configuration error |
| 7 | Permission denied |
| 8 | Network error |
| 9 | Integrity check failed |

### 6.2 Error Messages

Error messages include:

- What went wrong
- Why it might have happened
- What the user can do about it

---

## 7. Examples

### 7.1 Install a Package

```bash
$ vam p-install vam-clean
Installing vam-clean...
Dependencies: none
Capabilities: path:/var/log (read), path:/tmp (write)
Installation complete.
```

### 7.2 List Packages

```bash
$ vam i-list
Name              Version    Status
vam-clean         1.0.2      installed
vam-doctor        1.0.0      installed
sudo-cleaner      1.0.1      installed
```

### 7.3 Show Package Info

```bash
$ vam i-info vam-clean
Name:        vam-clean
Developer:   vsad
Version:     1.0.2
Release:     Stable
Description: System cleanup utility
Purpose:     Cleanup
Status:      installed
```

### 7.4 Show Rollback Versions

```bash
$ vam s-show-rollback vam-clean "1.0.0 - 1.0.2"
Version    Status
1.0.2      current
1.0.1      available
1.0.0      available
```

### 7.5 Run Diagnostics

```bash
$ vam t-doctor
VAM Diagnostic Report
====================
VAM Version:    0.1.0
Packages:       3 installed
Repositories:   2 configured
Health:         OK
```

---

## 8. Implementation Status

**Implemented:**

- No commands implemented yet

**Planned:**

- All commands as specified above
- Tab completion for bash/zsh/fish
- Man page generation
- Shell integration

---

## 9. Relationship to Other Specifications

This CLI reference interacts with:

- **Architecture** — how CLI connects to VAM Core
- **Package Specification** — package operations
- **Security Model** — capability checks, privilege escalation
- **TUI Specification** — TUI launch
- **Configuration Specification** — configuration commands
