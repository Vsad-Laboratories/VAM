# VAM Configuration Specification

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the VAM configuration system.

---

## 1. Overview

VAM uses a schema-driven configuration system.

Packages declare their configuration requirements through a schema. VAM validates the schema and provides a consistent configuration interface.

---

## 2. Configuration Philosophy

### 2.1 Principles

| Principle | Description |
|-----------|-------------|
| **Declarative** | Packages declare what they need |
| **Validated** | Schemas are validated by VAM |
| **Consistent** | Uniform configuration interface across packages |
| **Secure** | Configuration is constrained to declared scope |
| **Inspectable** | Users can view and modify configuration |

### 2.2 Benefits

- Package developers don't need to implement custom configuration UIs
- Users get a consistent configuration experience
- Configuration is validated before use
- Security is enforced through scope constraints

---

## 3. Configuration Flow

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

---

## 4. Schema Language

### 4.1 Custom DSL

Configuration schemas use a **custom DSL** with bracket-based syntax.

### 4.2 Syntax Rules

- `[-pkg.config-]` — section header (required at top of config schema)
- `<spt>` / `<ept>` — block delimiters (required for every configuration entry)
- `<identifier>` — configuration key name
- `|` — statement terminator (optional, conventional, like `;` in C)
- Indentation is cosmetic, not significant

### 4.3 Example Schema

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

<spt>
<orphan_handling>
    type = "enum"
    default = "ask"
    options = ["ask", "remove", "keep"]
<ept>

<spt>
<log_path>
    type = "path"
    default = "/var/log/vam"
    mode = "directory"
<ept>
```

---

## 5. Schema Types

### 5.1 Type Reference

| Type | Description | Example |
|------|-------------|---------|
| **boolean** | True/false toggle | `type = "boolean"` |
| **integer** | Whole number | `type = "integer"` |
| **string** | Text value | `type = "string"` |
| **enum** | Selection from list | `type = "enum"` |
| **path** | Filesystem path | `type = "path"` |
| **duration** | Time duration | `type = "duration"` |
| **command** | Shell command | `type = "command"` |
| **toggle** | On/off with states | `type = "toggle"` |
| **list** | List of values | `type = "list"` |

### 5.2 Type Properties

Each type supports specific properties:

#### boolean

```dsl
<flag>
    type = "boolean"
    default = true
<ept>
```

Properties:

- `default` — initial value (true/false)

#### integer

```dsl
<count>
    type = "integer"
    default = 7
    min = 1
    max = 365
    unit = "days"
<ept>
```

Properties:

- `default` — initial value
- `min` — minimum value
- `max` — maximum value
- `unit` — display unit

#### string

```dsl
<name>
    type = "string"
    default = "default"
    min_length = 1
    max_length = 100
<ept>
```

Properties:

- `default` — initial value
- `min_length` — minimum length
- `max_length` — maximum length
- `pattern` — regex pattern (optional)

#### enum

```dsl
<option>
    type = "enum"
    default = "ask"
    options = ["ask", "remove", "keep"]
<ept>
```

Properties:

- `default` — initial value
- `options` — list of allowed values

#### path

```dsl
<log_path>
    type = "path"
    default = "/var/log/vam"
    mode = "directory"
    writable = true
<ept>
```

Properties:

- `default` — initial value
- `mode` — "file", "directory", or "any"
- `writable` — whether path must be writable

#### duration

```dsl
<retention>
    type = "duration"
    default = "7d"
    min = "1d"
    max = "365d"
<ept>
```

Properties:

- `default` — initial value (e.g., "7d", "24h", "30m")
- `min` — minimum duration
- `max` — maximum duration

#### command

```dsl
<pre_hook>
    type = "command"
    default = ""
    optional = true
<ept>
```

Properties:

- `default` — initial value
- `optional` — whether command is required

#### toggle

```dsl
<mode>
    type = "toggle"
    default = "on"
    states = ["on", "off", "auto"]
<ept>
```

Properties:

- `default` — initial state
- `states` — list of toggle states

#### list

```dsl
<excludes>
    type = "list"
    default = []
    item_type = "string"
<ept>
```

Properties:

- `default` — initial list
- `item_type` — type of list items

---

## 6. Schema Structure

### 6.1 Section Header

Every schema file begins with:

```dsl
[-pkg.config-]
```

### 6.2 Configuration Entries

Each configuration entry is wrapped in `<spt>` / `<ept>`:

```dsl
<spt>
<entry_name>
    type = "type"
    default = value
<ept>
```

### 6.3 Multiple Entries

```dsl
[-pkg.config-]

<spt>
<entry_1>
    type = "boolean"
    default = true
<ept>

<spt>
<entry_2>
    type = "integer"
    default = 7
<ept>

<spt>
<entry_3>
    type = "enum"
    default = "ask"
    options = ["ask", "remove", "keep"]
<ept>
```

---

## 7. Schema Validation

### 7.1 Validation Rules

VAM validates schemas for:

- Correct syntax
- Valid type declarations
- Required properties present
- Default values within constraints
- No duplicate entry names
- Valid enum options
- Valid path modes

### 7.2 Validation Errors

Schema validation errors include:

- Line number
- Error description
- Suggested fix

### 7.3 Schema Versioning

Schema versioning is an **OPEN DECISION**.

---

## 8. Configuration Storage

### 8.1 Storage Location

Configuration files are stored per-package.

The exact configuration file layout is an **OPEN DECISION**.

### 8.2 Storage Format

Configuration values are stored in a format determined by VAM.

The exact storage format is an **OPEN DECISION**:

- TOML
- JSON
- Custom binary
- Other

### 8.3 Configuration Scope

Configuration should be constrained to declared/appropriate configuration areas.

Packages should not be able to arbitrarily write configuration outside their declared scope.

---

## 9. Configuration Interface

### 9.1 TUI Configuration

The TUI provides a configuration screen for each package.

Controls are generated based on schema type:

| Type | Control |
|------|---------|
| **boolean** | Toggle checkbox |
| **integer** | Number input |
| **string** | Text input |
| **enum** | Selection list |
| **path** | File browser |
| **duration** | Time input |
| **command** | Command input |
| **toggle** | Toggle switch |
| **list** | List editor |

### 9.2 CLI Configuration

The CLI provides configuration via command line:

```bash
vam c-config "package" --show
vam c-config "package" --set "key=value"
vam c-config "package" --reset
```

---

## 10. Configuration Flow

### 10.1 Package Installation

```text
1. Parse package schema
2. Validate schema
3. Generate default configuration
4. Store configuration
5. Notify package of configuration
```

### 10.2 Configuration Update

```text
1. User modifies configuration
2. VAM validates changes against schema
3. VAM stores new configuration
4. VAM notifies package of changes
5. Package applies new configuration
```

### 10.3 Configuration Reset

```text
1. User requests reset
2. VAM loads default values from schema
3. VAM stores default configuration
4. VAM notifies package of changes
```

---

## 11. Package Configuration Access

### 11.1 Configuration File

Packages receive configuration through a generated file.

The exact mechanism is an **OPEN DECISION**:

- Configuration file in runtime directory
- Environment variables
- Command-line arguments
- Other

### 11.2 Configuration Reload

Packages can request configuration reload.

The exact mechanism is an **OPEN DECISION**.

---

## 12. Open Configuration Decisions

| ID | Decision | Status |
|----|----------|--------|
| OD-40 | Configuration file layout | OPEN |
| OD-41 | Configuration storage format | OPEN |
| OD-42 | Schema versioning | OPEN |
| OD-43 | Configuration file vs env vs args | OPEN |
| OD-44 | Configuration reload mechanism | OPEN |
| OD-45 | Configuration encryption | OPEN |

---

## 13. Relationship to Other Specifications

This configuration specification interacts with:

- **Architecture** — how Configuration Engine works
- **Package Specification** — schema declaration in manifest
- **TUI Specification** — configuration UI
- **CLI Reference** — configuration commands
- **Security Model** — configuration scope constraints
