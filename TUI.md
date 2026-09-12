# VAM TUI Specification

**Version:** 0.1 (Draft)
**Status:** Specification Phase

This document defines the VAM terminal user interface.

---

## 1. Overview

Running `vam` with no arguments launches the primary VAM terminal user interface.

The TUI is designed as a Linux engineering console — professional, keyboard-first, and efficient.

---

## 2. Design Principles

| Principle | Description |
|-----------|-------------|
| **Keyboard-First** | Fully navigable without a mouse |
| **Minimal Clutter** | No unnecessary visual elements |
| **Clear Hierarchy** | Information is organized logically |
| **Fast Startup** | Launches quickly, responds immediately |
| **Low Resource Usage** | Minimal CPU and memory footprint |
| **Professional** | Serious engineering software aesthetic |
| **Consistent** | Uniform behavior across all screens |

---

## 3. Visual Identity

### 3.1 Color Palette

| Element | Color |
|---------|-------|
| **Primary** | VantaBlack (background) |
| **Secondary** | White / Light Gray |
| **Accent** | Purple |
| **Success** | Green |
| **Error** | Red |
| **Warning** | Yellow |
| **Info** | Blue |
| **Muted** | Dark Gray |

### 3.2 Aesthetic

- Aerospace / space / astronautical engineering feel
- Mission-control / instrumentation aesthetic
- Futuristic but professional
- Minimal and sophisticated
- Technical, not playful

### 3.3 Avoid

- Generic hacker aesthetics
- Excessive neon
- Gamer styling
- Childish UI
- Unnecessary animations
- Clutter

---

## 4. Navigation

### 4.1 Key Bindings

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `←` / `h` | Move left (if applicable) |
| `→` / `l` | Move right (if applicable) |
| `Enter` | Select / Confirm |
| `Esc` | Back / Cancel |
| `/` | Search |
| `q` | Quit |
| `Tab` | Switch focus (if applicable) |
| `?` | Show help |

### 4.2 Universal Navigation

- `Esc` always goes back one level
- `q` always quits (with confirmation if unsaved changes)
- `/` always opens search from any screen
- `?` shows context-sensitive help

---

## 5. Screen Hierarchy

```text
VAM TUI
│
├── Dashboard (Home)
│   ├── Categories
│   │   ├── System
│   │   ├── Storage
│   │   ├── Network
│   │   ├── Security
│   │   ├── Updates
│   │   ├── Cleanup
│   │   ├── Privacy
│   │   ├── Backup
│   │   ├── Services
│   │   ├── Processes
│   │   ├── Logs
│   │   ├── Hardware
│   │   └── Configuration
│   └── Quick Actions
│
├── Package List
│   ├── All Packages
│   ├── Installed
│   ├── Available
│   └── Updates
│
├── Package Detail
│   ├── Info
│   ├── Dependencies
│   ├── Capabilities
│   ├── Configuration
│   └── Rollback
│
├── Search
│
├── Configuration
│
└── Diagnostics
```

---

## 6. Dashboard

The dashboard is the home screen. It provides an overview and quick access to categories.

### 6.1 Layout

```text
┌─────────────────────────────────────────────────────────────┐
│  VAM — VSAD Arch Manager                        v0.1.0     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─ SYSTEM ──────────────────────────────────────────────┐  │
│  │  3 packages installed    1 update available           │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌─ CATEGORIES ──────────────────────────────────────────┐  │
│  │  > System                                            │  │
│  │    Storage                                           │  │
│  │    Network                                           │  │
│  │    Security                                          │  │
│  │    Updates                                           │  │
│  │    Cleanup                                           │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ┌─ QUICK ACTIONS ───────────────────────────────────────┐  │
│  │  [i-list]  [p-install]  [t-doctor]  [i-search]       │  │
│  └───────────────────────────────────────────────────────┘  │
│                                                             │
│  ↑↓ navigate  Enter select  / search  q quit  ? help       │
└─────────────────────────────────────────────────────────────┘
```

### 6.2 Categories

Each category contains related packages.

```text
System        → system info, diagnostics, hardware
Storage       → disk usage, cleanup, backup
Network       → network config, connectivity
Security      → security tools, permissions
Updates       → package updates, version management
Cleanup       → cache cleanup, log rotation
Privacy       → privacy tools, data management
Backup        → backup tools, restore
Services      → service management
Processes     → process management, monitoring
Logs          → log viewing, analysis
Hardware      → hardware detection, drivers
Configuration → system and package configuration
```

### 6.3 Quick Actions

Quick actions provide immediate access to common operations:

- `i-list` — list installed packages
- `p-install` — install a package
- `t-doctor` — run diagnostics
- `i-search` — search packages

---

## 7. Package List

### 7.1 Views

| View | Description |
|------|-------------|
| **All Packages** | All known packages |
| **Installed** | Only installed packages |
| **Available** | Only available (not installed) packages |
| **Updates** | Packages with available updates |

### 7.2 Layout

```text
┌─────────────────────────────────────────────────────────────┐
│  PACKAGE LIST — Installed                              /   │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Name              Version    Status       Size             │
│  ─────────────────────────────────────────────────────────  │
│  > vam-clean       1.0.2      installed    12K             │
│    vam-doctor      1.0.0      installed    8K              │
│    sudo-cleaner    1.0.1      installed    4K              │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│  3 packages                                    1 update     │
│                                                             │
│  ↑↓ navigate  Enter info  / search  Esc back  q quit       │
└─────────────────────────────────────────────────────────────┘
```

### 7.3 Sorting

Packages can be sorted by:

- Name (default)
- Version
- Status
- Size

Toggle sort with `s` key.

---

## 8. Package Detail

Shows detailed information about a selected package.

### 8.1 Tabs

| Tab | Content |
|-----|---------|
| **Info** | Name, developer, version, description, purpose |
| **Dependencies** | Required packages, dependency tree |
| **Capabilities** | Required capabilities, granted permissions |
| **Configuration** | Package configuration options |
| **Rollback** | Available versions for rollback |

### 8.2 Layout

```text
┌─────────────────────────────────────────────────────────────┐
│  PACKAGE — vam-clean                           [Info][Deps] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Name:        vam-clean                                     │
│  Developer:   vsad                                          │
│  Version:     1.0.2                                         │
│  Release:     Stable                                        │
│  Description: System cleanup utility                        │
│  Purpose:     Cleanup                                       │
│  Status:      installed                                     │
│                                                             │
│  ┌─ CAPABILITIES ─────────────────────────────────────────┐ │
│  │  path:/var/log (read)                                  │ │
│  │  path:/tmp (write)                                     │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  [p-remove]  [c-config]  [s-show-rollback]                 │
│                                                             │
│  ←→ switch tabs  Esc back  q quit                          │
└─────────────────────────────────────────────────────────────┘
```

---

## 9. Search

### 9.1 Search Interface

```text
┌─────────────────────────────────────────────────────────────┐
│  SEARCH                                                     │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  > search: clean_                                           │
│                                                             │
│  ┌─ RESULTS ──────────────────────────────────────────────┐ │
│  │  > vam-clean       1.0.2      installed                │ │
│    sudo-cleaner    1.0.1      installed                │ │
│    cache-clean     0.9.0      available                │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  ↑↓ navigate  Enter select  Esc back  q quit               │
└─────────────────────────────────────────────────────────────┘
```

### 9.2 Search Behavior

- Real-time filtering as user types
- Matches against package name, description, purpose
- Case-insensitive
- Supports partial matches

---

## 10. Configuration Screen

### 10.1 Layout

```text
┌─────────────────────────────────────────────────────────────┐
│  CONFIGURATION — vam-clean                                  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  Journal retention       [ 7 days        ]                  │
│  User cache cleanup      [ ✓ Enabled     ]                  │
│  Pacman cache cleanup    [ ✓ Enabled     ]                  │
│  Orphan packages         [ Ask           ]                  │
│  Temporary files         [ ✓ Enabled     ]                  │
│                                                             │
│  ─────────────────────────────────────────────────────────  │
│                                                             │
│              [ SAVE ]        [ CANCEL ]                     │
│                                                             │
│  ↑↓ navigate  Enter toggle  Esc cancel  Tab switch         │
└─────────────────────────────────────────────────────────────┘
```

### 10.2 Controls

| Type | Control |
|------|---------|
| **boolean** | Toggle checkbox |
| **integer** | Number input |
| **string** | Text input |
| **enum** | Selection list |
| **path** | File browser |
| **duration** | Time input |
| **command** | Command input |

---

## 11. Diagnostics Screen

### 11.1 Layout

```text
┌─────────────────────────────────────────────────────────────┐
│  DIAGNOSTICS                                                │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  VAM Diagnostic Report                                      │
│  ═══════════════════════════════════════════════════════    │
│                                                             │
│  VAM Version:    0.1.0                                      │
│  Packages:       3 installed                                │
│  Repositories:   2 configured                               │
│  Health:         ✓ OK                                       │
│                                                             │
│  ┌─ CHECKS ───────────────────────────────────────────────┐ │
│  │  ✓ VAM binary intact                                   │ │
│  │  ✓ Package directory accessible                        │ │
│  │  ✓ Runtime directory accessible                        │ │
│  │  ✓ Repository configuration valid                      │ │
│  │  ✓ All packages healthy                                │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                             │
│  [t-doctor --fix]  [t-doctor --report]                      │
│                                                             │
│  Esc back  q quit                                           │
└─────────────────────────────────────────────────────────────┘
```

---

## 12. Status Bar

The status bar appears at the bottom of every screen.

```text
↑↓ navigate  Enter select  / search  Esc back  q quit  ? help
```

Context-sensitive key hints change based on current screen.

---

## 13. Theming

### 13.1 Current Theme

VantaBlack with purple accent.

### 13.2 Future Themes

Theme support is an **OPEN DECISION**.

---

## 14. Implementation Status

**Implemented:**

- No TUI implemented yet

**Planned:**

- All screens as specified above
- Mouse support (optional)
- Terminal resize handling
- Unicode support
- True color support

---

## 15. Relationship to Other Specifications

This TUI specification interacts with:

- **Architecture** — how TUI connects to VAM Core
- **CLI Reference** — shared commands
- **Package Specification** — package display
- **Security Model** — capability display
- **Configuration Specification** — configuration interface
