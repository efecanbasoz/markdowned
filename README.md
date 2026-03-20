# Markdowned

> The fastest lightweight markdown editor for Linux, macOS, and Windows.

[![Version](https://img.shields.io/badge/version-1.5.0-blue)]()
[![License](https://img.shields.io/badge/license-MIT-green)]()
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey)]()

Built with [Tauri](https://tauri.app), [Svelte 5](https://svelte.dev), and [CodeMirror 6](https://codemirror.net). No Electron, no bloat.

## Features

- **Multi-tab editing** — Open multiple files with independent cursor, scroll, and dirty state
- **Split view** — Edit and preview side by side (horizontal or vertical)
- **Live preview** — GitHub-flavored Markdown with syntax-highlighted code blocks
- **Workspace search** — Search across all files with Ctrl+Shift+F
- **AI completion** — Inline suggestions from Ollama, OpenAI, Google, Anthropic, or custom providers
- **Auto-completion** — Optional debounced suggestions as you type
- **Command palette** — Quick file access with Ctrl+K
- **Dark & Light themes** — Toggle with Ctrl+Shift+T
- **Secure credentials** — API keys stored in OS keychain (GNOME Keyring / macOS Keychain)
- **File watcher** — Sidebar auto-updates on external file changes
- **Frontmatter support** — YAML metadata displayed as compact badge in preview
- **Lightweight** — ~50MB memory, <200ms startup, <10MB binary

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Ctrl+O | Open workspace |
| Ctrl+S | Save file |
| Ctrl+W | Close tab |
| Ctrl+K | Command palette |
| Ctrl+Shift+F | Search workspace |
| Ctrl+\\ | Cycle view mode (Edit → Split → Preview) |
| Ctrl+Shift+P | Toggle Edit/Preview |
| Ctrl+Shift+T | Toggle Dark/Light theme |
| Ctrl+Shift+E | Toggle sidebar |
| Ctrl+Enter | AI completion |
| Tab | Accept suggestion |
| Esc | Dismiss suggestion |
| Ctrl+B | Bold |
| Ctrl+I | Italic |
| Ctrl+Shift+K | Insert link |
| Ctrl+, | Settings |

## Installation

### Download

Grab the latest release from [GitHub Releases](https://github.com/efecanbasoz/markdowned/releases):

- **Linux:** `.deb` or `.AppImage`
- **macOS:** `.dmg` (ARM and Intel)
- **Windows:** `.msi` or `.exe`

### Build from Source

```bash
# Prerequisites: Node.js 20+, Rust 1.70+
# Linux: sudo apt install libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev libsoup-3.0-dev libgtk-3-dev libsecret-1-dev

git clone https://github.com/efecanbasoz/markdowned.git
cd markdowned
npm install
npx tauri build
```

## AI Completion Setup

Open Settings (Ctrl+,) to configure your AI provider:

| Provider | API Key Required | Default Model |
|----------|-----------------|---------------|
| Ollama (local) | No | llama3.2 |
| Google | Yes | gemini-2.0-flash |
| OpenAI | Yes | gpt-4o-mini |
| Anthropic | Yes | claude-3-5-haiku-latest |
| Custom | Optional | User-defined |

API keys are stored securely in your OS keychain.

## Configuration

Settings stored at `~/.config/markdowned/config.toml`:

```toml
theme = "dark"
split_direction = "horizontal"

[completion]
provider = "ollama"
model = ""
auto_completion = false
```

## Tech Stack

- **Shell:** [Tauri v2](https://tauri.app) (native, ~5MB overhead)
- **Frontend:** [Svelte 5](https://svelte.dev) (runes, zero runtime overhead)
- **Editor:** [CodeMirror 6](https://codemirror.net) (20+ language support)
- **Markdown:** [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) (GFM, fast Rust parser)
- **Highlighting:** [syntect](https://github.com/trishume/syntect) (code block syntax colors)
- **Security:** [ammonia](https://github.com/rust-ammonia/ammonia) (HTML sanitization)

## License

MIT
