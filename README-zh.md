<p align="center">
  <img src="src-tauri/icons/128x128.png" width="96" height="96" alt="Multi-Env Switcher Logo" />
</p>

<h1 align="center">Multi-Env Switcher</h1>

<p align="center">
  专为 macOS 打造的高性能开发运行时多版本切换助手，基于 <b>Tauri v2</b>、<b>Rust</b> 与 <b>Vue 3</b> 构建。
</p>

<p align="center">
  <a href="README.md">English</a> | <b>简体中文</b>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/平台-macOS-black?style=flat-square&logo=apple" alt="平台" />
  <img src="https://img.shields.io/badge/Tauri-v2-24C8D8?style=flat-square&logo=tauri&logoColor=white" alt="Tauri v2" />
  <img src="https://img.shields.io/badge/Rust-后端核心-dea584?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
  <img src="https://img.shields.io/badge/Vue-3.x-42b883?style=flat-square&logo=vue.js&logoColor=white" alt="Vue 3" />
  <img src="https://img.shields.io/badge/TailwindCSS-v4-38bdf8?style=flat-square&logo=tailwindcss&logoColor=white" alt="TailwindCSS v4" />
  <img src="https://img.shields.io/badge/开源协议-MIT-blue?style=flat-square" alt="协议" />
</p>

---

## ✨ 功能特性

- 🍏 **原生 macOS 系统设置（System Settings）设计语言**：
  - 严格像素级复刻 macOS Ventura / Sonoma / Sequoia 原生系统设置双栏布局与卡片致密排版。
  - 实时联动 macOS 系统深色/浅色外观切换（Dark/Light Mode）。
  - **智能 Overlay 滚动条**：静止或切换面板时完全隐形，仅在滚动中平滑浮现，停滚自动隐退。
  - 无边框平滑窗口拖拽，完美避让原生交通灯按钮；支持快捷键聚焦搜索。

- ☕ **Java (SDKMAN) 版本管理**：
  - 自动发现 `~/.sdkman/candidates/java` 下安装的 JDK（Zulu、Temurin、Oracle 等）。
  - 原子级 Symlink 瞬间切换，0 延迟、无需等待终端重启。
  - 支持后台下载安装新版本，卸载支持原生 macOS 警示二次确认弹窗。

- ⬢ **Node.js (nvm) 版本管理**：
  - 极速扫描 `~/.nvm/versions/node` 中已安装的版本。
  - 通过写入 `~/.nvm/alias/default` 实现全局即时切换。
  - 提供常用 LTS 版本（22、20、18 等）一键快捷安装。

- 🐍 **Python (uv) 版本管理**：
  - 基于 Astral 开发的超高速 Python 包/运行时管理器 `uv`。
  - 通过全局 `~/.python-version` 软链接与环境配置实现极速版本切换。
  - 支持后台免阻塞安装与安全卸载。

- 📋 **环境组合预设方案 (Profiles)**：
  - 将 Java、Node.js 与 Python 版本绑定为一个预设组合（如 *全栈项目 A*、*老项目 Spring Boot 2 + Node 16*）。
  - 像素级还原 macOS 原生绿色 Switch 开关，一键切换全套运行时版本。
  - 底部提供标准的 `+` / `—` 工具条，支持增删改查。
  - 预设数据保存在本地标准的 `~/.config/multi-env/profiles.json` 中。

- 🌐 **全量国际化（i18n）双语支持**：
  - 支持 **简体中文** 与 **English**。
  - 顶栏右侧提供 `🌐 EN / 中` 快捷切换按钮，同时可在「关于与设置」面板中随时更改。
  - 自动检测系统首选语言，并在本地持久化保存用户的语言偏好。

---

## 🛠️ 前置要求

请确保您的 macOS 系统中已安装对应的运行时管理器：

| 运行时 | 对应环境管理器 | 官方推荐安装命令 |
| :--- | :--- | :--- |
| **Java** | [SDKMAN!](https://sdkman.io/) | `curl -s "https://get.sdkman.io" \| bash` |
| **Node.js** | [nvm](https://github.com/nvm-sh/nvm) | `curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh \| bash` |
| **Python** | [uv](https://docs.astral.sh/uv/) | `curl -LsSf https://astral.sh/uv/install.sh \| sh` 或 `brew install uv` |

---

## 🚀 快速上手

### 1. 克隆代码仓库

```bash
git clone https://github.com/laoshiren1207/multi-env.git
cd multi-env
```

### 2. 安装前端依赖

```bash
npm install
```

### 3. 启动开发模式

```bash
npm run tauri dev
```

### 4. 构建生产发布包

生成 macOS 原生 `.app` 与 `.dmg` 安装包（支持 Apple Silicon 与 Intel）：

```bash
npm run tauri build
```

构建完成后产物位于：
```
src-tauri/target/release/bundle/macos/Multi-Env Switcher.app
src-tauri/target/release/bundle/dmg/Multi-Env Switcher_*.dmg
```

---

## 🏗️ 项目架构

```
multi-env/
├── src/                      # Vue 3 前端工程
│   ├── components/           # UI 组件 (ProfileForm 表单、StatusNotification 通知等)
│   ├── composables/          # EnvManager IPC 交互组合式函数 (useEnvManager)
│   ├── i18n/                 # 响应式国际化核心（中英文字典与切换逻辑）
│   ├── styles/               # TailwindCSS v4 与 macOS 原生滚动条样式
│   ├── types/                # TypeScript 类型声明
│   ├── App.vue               # macOS 设置风格双栏主界面
│   └── main.ts               # 前端入口与系统暗色模式监听
└── src-tauri/                # Rust 后端核心 (Tauri v2)
    ├── src/
    │   ├── commands.rs       # 异步 IPC 响应命令（tokio 异步非阻塞架构）
    │   ├── managers/         # SDKMAN、nvm、uv 运行时检测与调度实现
    │   ├── models.rs         # 数据结构定义与序列化
    │   └── lib.rs            # Tauri 应用初始化与窗口控制
    ├── capabilities/         # 权限策略配置文件
    └── tauri.conf.json       # 应用包配置与窗口属性
```

---

## ⌨️ 快捷键

| 快捷键 | 功能描述 |
| :--- | :--- |
| <kbd>⌘</kbd> + <kbd>F</kbd> | 快速聚焦左侧搜索框并全选 |
| <kbd>Esc</kbd> | 清空搜索关键词或取消聚焦搜索框 |

---

## 📄 开源许可

本项目采用 [MIT 许可证](LICENSE) 开源。
