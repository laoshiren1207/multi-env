<p align="center">
  <img src="src-tauri/icons/128x128.png" width="96" height="96" alt="Multi-Env Switcher Logo" />
</p>

<h1 align="center">Multi-Env Switcher</h1>

<p align="center">
  A native, blazing-fast macOS runtime environment switcher crafted with <b>Tauri v2</b>, <b>Rust</b>, and <b>Vue 3</b>.
</p>

<p align="center">
  <b>English</b> | <a href="README-zh.md">简体中文</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-macOS-black?style=flat-square&logo=apple" alt="Platform" />
  <img src="https://img.shields.io/badge/Tauri-v2-24C8D8?style=flat-square&logo=tauri&logoColor=white" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/Rust-Backend-dea584?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Vue-3.x-42b883?style=flat-square&logo=vue.js&logoColor=white" alt="Vue 3" />
  <img src="https://img.shields.io/badge/TailwindCSS-v4-38bdf8?style=flat-square&logo=tailwindcss&logoColor=white" alt="TailwindCSS v4" />
  <img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License" />
</p>

---

## ✨ Features

- 🍏 **Native macOS System Settings Aesthetics**:
  - Two-pane compact sidebar layout strictly modeled after modern macOS System Settings.
  - Inset grouped cards, refined micro-typography, and seamless window dragging.
  - Real-time automatic Light / Dark mode synchronization with system appearance.
  - **Smart Overlay Scrollbars**: Completely invisible when idle or browsing; smoothly appears only while actively scrolling.
  - **⌘F Quick Search**: Instantly filter runtimes, versions, and presets.

- ☕ **Java (SDKMAN) Management**:
  - Automatically scans installed JDKs in `~/.sdkman/candidates/java`.
  - Zero-latency version switching via atomic symlink updates.
  - Background installation and uninstallation with secondary confirmation dialogs.

- ⬢ **Node.js (nvm) Management**:
  - Scans versions installed in `~/.nvm/versions/node`.
  - Instant default version switching by updating `~/.nvm/alias/default`.
  - Install new versions or uninstall obsolete releases effortlessly.

- 🐍 **Python (uv) Management**:
  - High-performance Python runtime management powered by Astral's `uv`.
  - Global runtime switching via `~/.python-version`.
  - One-click downloads for popular Python releases (3.12, 3.11, etc.).

- 📋 **Environment Presets (Profiles)**:
  - Bundle Java, Node.js, and Python versions into unified environment sets (e.g. *Fullstack Project A*, *Legacy Spring Boot 2 + Node 16*).
  - One-click preset switching with native macOS green toggle switches.
  - Add, edit, and delete presets with intuitive `+` / `—` toolbar controls.
  - Configurations stored locally and cleanly in `~/.config/multi-env/profiles.json`.

- 🌐 **Built-in Internationalization (i18n)**:
  - Toggle between **English** and **简体中文** in real-time from the header button (`🌐 EN / 中`) or settings.
  - Automatically detects system locale and persists language preference.

---

## 🛠️ Prerequisites

Make sure you have installed the respective runtime managers on your macOS system:

| Runtime | Manager | Installation Command |
| :--- | :--- | :--- |
| **Java** | [SDKMAN!](https://sdkman.io/) | `curl -s "https://get.sdkman.io" \| bash` |
| **Node.js** | [nvm](https://github.com/nvm-sh/nvm) | `curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh \| bash` |
| **Python** | [uv](https://docs.astral.sh/uv/) | `curl -LsSf https://astral.sh/uv/install.sh \| sh` or `brew install uv` |

---

## 🚀 Quick Start

### 1. Clone the repository

```bash
git clone https://github.com/laoshiren1207/multi-env.git
cd multi-env
```

### 2. Install dependencies

```bash
npm install
```

### 3. Run in development mode

```bash
npm run tauri dev
```

### 4. Build standalone application bundle

To build the native `.app` and `.dmg` for macOS (Apple Silicon or Intel):

```bash
npm run tauri build
```

The compiled release will be available under:
```
src-tauri/target/release/bundle/macos/Multi-Env Switcher.app
src-tauri/target/release/bundle/dmg/Multi-Env Switcher_*.dmg
```

---

## 🏗️ Architecture

```
multi-env/
├── src/                      # Vue 3 Frontend
│   ├── components/           # UI Components (ProfileForm, StatusNotification, etc.)
│   ├── composables/          # EnvManager IPC composables (useEnvManager)
│   ├── i18n/                 # Reactive i18n module (English & 简体中文)
│   ├── styles/               # TailwindCSS v4 and native macOS overlay scrollbars
│   ├── types/                # TypeScript interface definitions
│   ├── App.vue               # Main macOS Settings two-pane layout
│   └── main.ts               # App entry and system dark mode listener
└── src-tauri/                # Rust Core (Tauri v2)
    ├── src/
    │   ├── commands.rs       # Async IPC commands (tokio non-blocking)
    │   ├── managers/         # SDKMAN, nvm, and uv runtime handlers
    │   ├── models.rs         # Data structures and serialization
    │   └── lib.rs            # Tauri application builder & window setup
    ├── capabilities/         # Tauri security capabilities & permissions
    └── tauri.conf.json       # App metadata & window configurations
```

---

## ⌨️ Shortcuts

| Shortcut | Description |
| :--- | :--- |
| <kbd>⌘</kbd> + <kbd>F</kbd> | Focus the search input |
| <kbd>Esc</kbd> | Clear search query or defocus search input |

---

## 📄 License

This project is open-sourced under the [MIT License](LICENSE).
