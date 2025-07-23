# 🎙️ Speakr Documentation

> [!NOTE]
> **Speakr** is a privacy-first, hot-key–driven dictation utility that turns your speech into
> typed text entirely on-device. No cloud, no latency, no compromises.

---

## ✨ What is Speakr?

Speakr transforms the way you capture thoughts into text. With a single keystroke, record speech,
transcribe it locally using Whisper models, and have the text instantly typed into any application.
Perfect for developers, writers, and anyone who thinks faster than they type.

### 🔐 **Privacy First**

- **100% offline processing** – your voice never leaves your device
- **No cloud dependencies** – works in air-gapped environments
- **Minimal permissions** – only microphone and accessibility access

### ⚡ **Built for Speed**

- **≤ 3 second** end-to-end latency for 5-second recordings
- **Global hotkeys** work across all applications
- **Lightweight** universal macOS binary < 20 MB

---

## 🧭 Navigate the Documentation

> [!TIP]
> Use the search box (<kbd>⌘</kbd>/<kbd>Ctrl</kbd> + <kbd>K</kbd>) to quickly jump to any topic,
> or browse by your role below.

### 📋 **Product & Planning**

| Document                                          | Description                               | Audience                     |
| ------------------------------------------------- | ----------------------------------------- | ---------------------------- |
| **[Product Requirements](PRD.md)**                | Vision, goals, and feature specifications | Product owners, stakeholders |
| **[Implementation Plan](IMPLEMENTATION_PLAN.md)** | Development roadmap and milestones        | Project managers, engineers  |

### 🏗️ **Architecture & Engineering**

| Document                                            | Description                          | Audience                |
| --------------------------------------------------- | ------------------------------------ | ----------------------- |
| **[Technical Architecture](ARCHITECTURE.md)**       | System design and component overview | Engineers, architects   |
| **[System Description](SYSTEM_DESCRIPTION.md)**     | Detailed system behaviour and flows  | Developers, maintainers |
| **[Development Overview](DEVELOPMENT_OVERVIEW.md)** | Getting started with development     | New contributors        |

### 📝 **Functional Specifications**

| Document                                                         | Description                       | Status        |
| ---------------------------------------------------------------- | --------------------------------- | ------------- |
| **[FR-1: Global Hotkey](specs/FR-1-global-hotkey.md)**           | Hot-key registration and handling | ✅ Implemented |
| **[FR-2: Audio Capture](specs/FR-2-audio-capture.md)**           | Microphone access and recording   | ✅ Implemented |
| **[FR-3: Transcription](specs/FR-3-transcription.md)**           | Local Whisper integration         | 🔄 In Progress |
| **[FR-4: Text Injection](specs/FR-4-text-injection.md)**         | Cross-app text insertion          | 🔄 In Progress |
| **[FR-5: Injection Fallback](specs/FR-5-injection-fallback.md)** | Clipboard fallback mechanism      | 📋 Planned     |
| **[FR-6: Settings UI](specs/FR-6-settings-ui.md)**               | Configuration interface           | ✅ Implemented |

> [!WARNING]
> See **[Specs Overview](specs/README.md)** for the complete functional requirements including
> non-functional requirements (NFRs) for security, performance, and accessibility.

### 🔧 **Development & Debugging**

| Document                              | Description                           | Audience           |
| ------------------------------------- | ------------------------------------- | ------------------ |
| **[Debug Panel](DEBUG_PANEL.md)**     | Development and troubleshooting tools | Developers, QA     |
| **[Pre-commit Hooks](PRE_COMMIT.md)** | Code quality and testing setup        | Contributors       |
| **[Tauri Plugins](TAURI_PLUGINS.md)** | Plugin architecture and integrations  | Backend developers |

---

## 🚀 Quick Start

> [!NOTE]
> **New to the project?** Start with the [Development Overview](DEVELOPMENT_OVERVIEW.md) for
> setup instructions.

### For Product People

1. Read the **[Product Requirements](PRD.md)** to understand the vision
2. Check the **[Implementation Plan](IMPLEMENTATION_PLAN.md)** for current progress
3. Review **[Functional Specs](specs/README.md)** for detailed features

### For Engineers

1. Study the **[Technical Architecture](ARCHITECTURE.md)** for system design
2. Follow **[Development Setup](DEVELOPMENT_OVERVIEW.md)** to get coding
3. Reference **[System Description](SYSTEM_DESCRIPTION.md)** for implementation details

### For Contributors

1. Set up **[pre-commit hooks](PRE_COMMIT.md)** for code quality
2. Browse **[functional requirements](specs/README.md)** to find tasks
3. Use the **[Debug Panel](DEBUG_PANEL.md)** for development workflow

---

## 📊 Project Status

> [!TIP]
> **Current Focus**: Core transcription engine and text injection reliability

| Component            | Status     | Notes                                 |
| -------------------- | ---------- | ------------------------------------- |
| **Global Hotkeys**   | ✅ Complete | Cross-app hotkey registration working |
| **Audio Capture**    | ✅ Complete | High-quality microphone input         |
| **Settings UI**      | ✅ Complete | Leptos-based configuration interface  |
| **Transcription**    | 🔄 Active   | Whisper integration in progress       |
| **Text Injection**   | 🔄 Active   | Cross-app compatibility improvements  |
| **Model Management** | 📋 Planned  | GGUF model download and validation    |

---

## 🤝 Contributing

> [!NOTE]
> This documentation is a living document. Found something unclear or outdated?

- **📂 Browse specs** in the [specs directory](specs/) for implementation tasks
- **🐛 Report issues** via GitHub Issues
- **📝 Improve docs** by opening a pull request
- **💡 Suggest features** in GitHub Discussions

---

### Built with 🦀 Rust, ⚡ Tauri 2, and 🎨 Leptos

Privacy-first dictation for the modern developer
