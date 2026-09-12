# VAM Repository Specification

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the VAM repository system.

---

## 1. Overview

VAM uses a repository model for package distribution.

The repository is **Git-based**, providing version control, pull request workflows, and audit trails.

---

## 2. Repository Model

### 2.1 Two Repositories

VAM is intended to use two repositories:

| Repository | Purpose | Control |
|-----------|---------|---------|
| **Main VAM Repository** | VAM core, official/evergreen packages | Tightly controlled by VSAD |
| **VAM Plugins Repository** | External/community packages | Open with review process |

### 2.2 Main Repository

Contains:

- VAM core components
- Official/evergreen packages
- Tightly controlled by VSAD

Trust level: **High**

### 2.3 Plugins Repository

Contains:

- External/community-developed packages
- Developers follow VAM package guidelines
- Contribution via pull requests
- Review/policy process

Trust level: **Variable**

Critical/important packages receive stricter VSAD review.

---

## 3. Repository Backend

### 3.1 Git-Based

The repository is **Git-based**.

Package metadata, versions, compatibility information, integrity information, signatures, and artifacts are stored in Git repositories.

### 3.2 Benefits

- Version control for package metadata
- Pull request workflow for contributions
- Audit trail for changes
- Distributed access
- Branching for package versions
- Tagging for releases

### 3.3 Repository Structure

```text
vam-repository/
├── packages/
│   ├── vam-clean/
│   │   ├── manifest.toml
│   │   ├── payload/
│   │   ├── schema/
│   │   └── integrity/
│   ├── vam-doctor/
│   │   ├── manifest.toml
│   │   ├── payload/
│   │   ├── schema/
│   │   └── integrity/
│   └── ...
├── index/
│   ├── packages.toml
│   └── versions.toml
├── signatures/
│   └── ...
└── README.md
```

---

## 4. Package Metadata

### 4.1 Manifest Format

Package metadata is stored as `manifest.toml` in TOML format.

```toml
name = "vam-clean"
developer = "vsad"
version = "1.0.2"
release = "Stable"
description = "System cleanup utility"
purpose = "Cleanup"

[dependencies]
vam-packages = []
arch-packages = ["pacman"]

[[capabilities]]
type = "path"
access = "/var/log"
mode = "read"

[[capabilities]]
type = "path"
access = "/tmp"
mode = "write"
```

### 4.2 Index Files

Repository index files track available packages.

`packages.toml`:

```toml
[vam-clean]
developer = "vsad"
versions = ["1.0.0", "1.0.1", "1.0.2"]
latest = "1.0.2"

[vam-doctor]
developer = "vsad"
versions = ["1.0.0"]
latest = "1.0.0"
```

`versions.toml`:

```toml
[vam-clean."1.0.2"]
release = "Stable"
compatibility = ">=0.1.0"
integrity = "sha256:abc123..."

[vam-clean."1.0.1"]
release = "Stable"
compatibility = ">=0.1.0"
integrity = "sha256:def456..."
```

---

## 5. Package Distribution

### 5.1 Artifact Storage

Package artifacts (`.vampkg` files) are stored in the repository.

The exact storage mechanism is an **OPEN DECISION**:

- Git LFS
- Release assets
- Separate artifact server
- Other

### 5.2 Download Flow

```text
User requests package
    ↓
VAM queries repository index
    ↓
VAM finds package and version
    ↓
VAM downloads artifact
    ↓
VAM verifies integrity
    ↓
VAM installs package
```

### 5.3 Caching

Local caching of downloaded packages is an **OPEN DECISION**.

---

## 6. Package Trust

### 6.1 Repository Signatures

The repository itself is signed.

The exact signing mechanism is an **OPEN DECISION**:

- GPG signatures
- SSH signatures
- Other

### 6.2 Package Signatures

Individual packages can be signed.

The exact package signing mechanism is an **OPEN DECISION**.

### 6.3 Trust Configuration

Users configure which repositories are trusted.

Trust configuration format is an **OPEN DECISION**.

---

## 7. Version Management

### 7.1 Versioning Scheme

Packages use semantic versioning:

```text
MAJOR.MINOR.PATCH
```

### 7.2 Compatibility

Each version declares compatibility:

```toml
compatibility = ">=0.1.0"
```

### 7.3 Rollback Versions

Multiple versions are retained for rollback.

```text
vam-clean
├── 1.0.2  ← current
├── 1.0.1
└── 1.0.0
```

Rollback retention is an **OPEN DECISION**.

---

## 8. Repository Updates

### 8.1 Update Check

```text
VAM
    ↓
Query repository index
    ↓
Compare installed versions
    ↓
Identify available updates
    ↓
Present to user
```

### 8.2 Update Flow

```text
User requests update
    ↓
VAM downloads update
    ↓
VAM verifies integrity
    ↓
VAM creates rollback point
    ↓
VAM installs update
    ↓
VAM retains rollback history
```

---

## 9. Contribution Model

### 9.1 Main Repository

- Controlled by VSAD
- Only VSAD-approved packages
- Strict review process

### 9.2 Plugins Repository

- Open to community developers
- Pull request workflow
- Review/policy process

### 9.3 Contribution Guidelines

Package-development guidance will be provided through the VAM GitHub repository Wiki.

Requirements:

- Follow VAM package specification
- Include complete manifest
- Declare all capabilities
- Include documentation
- Include tests (if applicable)

### 9.4 Review Process

The exact review process is an **OPEN DECISION**.

---

## 10. Repository Configuration

### 10.1 Configuration File

Repository configuration is stored locally.

The exact configuration format is an **OPEN DECISION**.

Example:

```toml
[[repositories]]
name = "main"
url = "https://github.com/vsad/vam-repository"
trusted = true

[[repositories]]
name = "plugins"
url = "https://github.com/vsad/vam-plugins"
trusted = true
```

### 10.2 Multiple Repositories

Users can configure multiple repositories.

Repository priority and conflict resolution are **OPEN DECISIONS**.

---

## 11. Offline Support

### 11.1 Offline Operations

VAM should work offline where possible:

- View installed packages
- Package information
- Configuration
- Local diagnostics
- Local package operations

### 11.2 Online Operations

Internet required for:

- Repository search
- Package discovery
- Downloading
- Updates
- Fetching remote source

---

## 12. Open Repository Decisions

| ID | Decision | Status |
|----|----------|--------|
| OD-31 | Artifact storage mechanism | OPEN |
| OD-32 | Local caching strategy | OPEN |
| OD-33 | Repository signature format | OPEN |
| OD-34 | Package signature format | OPEN |
| OD-35 | Trust configuration format | OPEN |
| OD-36 | Rollback retention default | OPEN |
| OD-37 | Review process details | OPEN |
| OD-38 | Repository priority model | OPEN |
| OD-39 | Conflict resolution model | OPEN |

---

## 13. Relationship to Other Specifications

This repository specification interacts with:

- **Architecture** — how repository connects to VAM Core
- **Package Specification** — package format, manifest
- **Security Model** — trust, signatures
- **CLI Reference** — repository commands
