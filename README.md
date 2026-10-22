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

- 🔄 **Multi-Runtime Version Switching**:
  - **Java (SDKMAN)**: Switch global JDK versions instantly without restarting your terminal.
  - **Node.js (nvm)**: Switch Node.js versions on the fly.
  - **Python (uv)**: Fast Python version switching powered by `uv`.
  - Background installation and uninstallation of runtime versions.

- 📋 **Environment Presets (Profiles)**:
  - Combine specific Java, Node.js, and Python versions into one-click profiles (e.g. *Fullstack Project A*, *Legacy Spring Boot 2 + Node 16*).
  - Switch entire development environments with a single toggle.
  - Easily add, edit, and manage custom presets.

- 🌐 **Bilingual Support (i18n)**:
  - Switch between **English** and **简体中文** anytime.

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
│   ├── styles/               # TailwindCSS v4 styles
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
