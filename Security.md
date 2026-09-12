# VAM Security Model

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the security model for VAM — VSAD Arch Manager.

---

## 1. Security Philosophy

VAM is a system-management platform. Security is a first-class architectural concern.

VAM's security architecture is designed to **reduce and control package risk**. Absolute security claims are not made.

VAM must never reduce package installation to:

```text
download random script
↓
sudo bash
↓
hope for the best
```

---

## 2. Security Principles

| Principle | Description |
|-----------|-------------|
| **Least Privilege** | Operations execute with minimum required privileges |
| **Explicit Trust** | Trust is declared, not assumed |
| **Defense in Depth** | Multiple security layers |
| **Fail Secure** | Failures deny by default |
| **Transparency** | Security operations are visible to the user |

---

## 3. Trust Model

VAM uses two related but distinct security concepts:

### 3.1 Package Trust

Package trust concerns the authenticity and integrity of packages.

| Concern | Description |
|---------|-------------|
| **Source Authenticity** | Who created the package |
| **Repository Trust** | Which repositories are trusted |
| **Signatures** | Cryptographic signatures |
| **Hashes/Checksums** | Integrity verification |
| **Integrity Verification** | Package has not been tampered with |
| **Version Verification** | Package version is expected |
| **Controlled Repositories** | Only trusted repositories are used |

### 3.2 Runtime Safety

Runtime safety concerns how packages execute.

| Concern | Description |
|---------|-------------|
| **Declared Capabilities** | Package declares what it needs |
| **Controlled Privilege** | Privilege is explicitly granted |
| **Execution Boundaries** | Package operates within defined boundaries |
| **Path Restrictions** | Package can only access declared paths |
| **Safe Argument Handling** | Input is validated |
| **Controlled Privileged Operations** | Elevated operations go through VAM broker |

### 3.3 Trust Boundaries

Metadata alone does **not** guarantee a package is safe.

VAM enforces trust through:

1. **Capability declarations** — package must declare what it needs
2. **Capability validation** — VAM checks declarations against policy
3. **Runtime monitoring** — execution is bounded by declared capabilities
4. **Privilege brokering** — elevated operations go through VAM

---

## 4. Capability Model

### 4.1 Capability Types

VAM uses **path/network/privilege grants**:

| Capability Type | Description | Example |
|----------------|-------------|---------|
| **path** | Filesystem access | `path = "/var/log"` |
| **network** | Network access | `network = "outbound"` |
| **privilege** | Elevated operations | `privilege = "systemctl"` |

### 4.2 Capability Declaration

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

[[capabilities]]
type = "privilege"
access = "systemctl"
operations = ["restart", "status"]
```

### 4.3 Capability Validation

Before package execution, VAM validates:

1. All declared capabilities are explicitly granted
2. No undeclared capabilities are requested
3. Capability scope matches declared scope
4. User has acknowledged capability requirements

### 4.4 Capability Enforcement

During execution:

- **Path capabilities** — VAM restricts filesystem access to declared paths
- **Network capabilities** — VAM controls network access based on declarations
- **Privilege capabilities** — VAM brokers privileged operations through its API

---

## 5. Privilege Model

### 5.1 Privilege Principles

1. **VAM handles privilege escalation** — the user interacts with VAM, not raw sudo
2. **VAM provides a controlled privileged-operation API** — packages request elevated operations through defined interfaces
3. **VAM itself does not simply run as root** — privilege is requested narrowly
4. **Packages do not freely call sudo** — they use VAM's privileged-operation API

### 5.2 Privilege Broker

Privileged operations are performed through **direct syscalls through VAM broker**.

```text
Package
    ↓
request privileged operation
    ↓
VAM validates capability
    ↓
VAM checks user consent
    ↓
VAM performs operation
    ↓
return result to package
```

### 5.3 Privilege Scope

VAM should use narrowly scoped privileged operations.

Examples of privileged operations:

- systemctl service management
- filesystem writes to protected locations
- network configuration changes
- user/group management
- system configuration changes

### 5.4 User Consent

Privileged operations require explicit user consent.

The consent model is an **OPEN DECISION**.

---

## 6. Package Verification

### 6.1 Verification Layers

```text
Package Signature
    ↓
Repository Signature
    ↓
Manifest Validation
    ↓
Integrity Checksums
    ↓
Capability Validation
    ↓
Dependency Verification
```

### 6.2 Signature Model

The package trust/signature cryptographic model is an **OPEN DECISION**.

### 6.3 Verification Flow

```text
1. Verify repository signature
2. Verify package signature
3. Validate manifest structure
4. Check integrity checksums
5. Validate capability declarations
6. Check dependency availability
7. Grant or deny installation
```

---

## 7. Repository Trust

### 7.1 Repository Types

| Repository | Trust Level | Control |
|-----------|-------------|---------|
| **Main VAM Repository** | High | Tightly controlled by VSAD |
| **VAM Plugins Repository** | Variable | Open with review process |

### 7.2 Trust Configuration

Users configure which repositories are trusted.

Trust configuration is an **OPEN DECISION**.

### 7.3 Local Package Trust

A local `.vampkg` is **not** automatically trusted merely because it is local.

Official repository trust and local package trust are distinct concepts.

---

## 8. Runtime Security

### 8.1 Runtime Isolation

VAM uses **minimal isolation with capability checks**.

The runtime provides:

- Environment control
- Declared path restrictions
- Capability validation
- Privilege boundary enforcement

Full sandboxing is **not** used. VAM relies on capability declarations and validation.

### 8.2 Environment Control

Packages receive only declared environment variables.

Undeclared environment variables are not passed to package code.

### 8.3 Path Restrictions

Packages can only access declared paths.

Undeclared path access is denied.

### 8.4 Execution Monitoring

Package execution is monitored for:

- Undeclared capability usage
- Path access violations
- Privilege escalation attempts
- Unexpected behavior patterns

---

## 9. Secure Package Installation

### 9.1 Installation Security

```text
1. Verify repository trust
2. Verify package signature
3. Validate manifest
4. Check integrity
5. Validate capabilities
6. Check dependencies
7. Request user consent for capabilities
8. Extract to temporary location
9. Verify extracted contents
10. Move to final location
11. Register package
```

### 9.2 Safe Extraction

- Extract to isolated temporary directory
- Validate extracted contents before moving
- Prevent path traversal attacks
- Validate file permissions
- Check for unexpected files

### 9.3 Secure Update

```text
1. Verify update authenticity
2. Validate update compatibility
3. Create rollback point
4. Install update
5. Verify update success
6. Register rollback
```

---

## 10. Audit and Logging

### 10.1 Audit Events

VAM should log:

- Package installations
- Package removals
- Privilege escalations
- Capability grants
- Security violations
- Configuration changes

### 10.2 Audit Model

The logging and audit model is an **OPEN DECISION**.

---

## 11. Security Boundaries

### 11.1 What VAM Protects Against

- Unauthorized package installation
- Privilege escalation without consent
- Path traversal attacks
- Command injection
- Undeclared capability usage
- Tampered packages
- Untrusted repositories

### 11.2 What VAM Does Not Protect Against

- Malicious packages with declared capabilities
- Social engineering attacks
- Compromised development environments
- Zero-day vulnerabilities in dependencies

### 11.3 Trust Assumptions

VAM assumes:

- The VAM binary itself is trustworthy
- The user understands capability implications
- The system is not already compromised
- Repository infrastructure is secure

---

## 12. Open Security Decisions

| ID | Decision | Status |
|----|----------|--------|
| OD-06 | Package trust/signature cryptographic model | OPEN |
| OD-27 | User consent model for privileged operations | OPEN |
| OD-28 | Trust configuration format | OPEN |
| OD-29 | Security violation response | OPEN |
| OD-30 | Audit log format and storage | OPEN |

---

## 13. Relationship to Other Specifications

This security model interacts with:

- **Architecture** — how security is implemented in VAM Core
- **Package Specification** — capability declarations, manifest format
- **Repository Specification** — repository trust, signatures
- **Configuration Specification** — security configuration
