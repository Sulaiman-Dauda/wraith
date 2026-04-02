```
 ██╗    ██╗██████╗  █████╗ ██╗████████╗██╗  ██╗
 ██║    ██║██╔══██╗██╔══██╗██║╚══██╔══╝██║  ██║
 ██║ █╗ ██║██████╔╝███████║██║   ██║   ███████║
 ██║███╗██║██╔══██╗██╔══██║██║   ██║   ██╔══██║
 ╚███╔███╔╝██║  ██║██║  ██║██║   ██║   ██║  ██║
  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝  ╚═╝
```

> **The ghost in your terminal.**

[![License: MIT](https://img.shields.io/badge/License-MIT-cyan.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-violet.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/wraith-cli.svg)](https://crates.io/crates/wraith-cli)

WRAITH is an **open-source, terminal-native AI coding agent** written in Rust. It lives in your terminal, understands your codebase, and executes multi-step coding tasks autonomously — reading files, writing code, running commands — all through a conversational REPL with a cyberpunk soul.

---

## Install

```sh
cargo install wraith-cli
```

> **Note**: The binary is named `wraith`. You run it as `wraith` after install.

Or build from source:

```sh
git clone https://github.com/your-org/wraith
cd wraith/rust
cargo build --release
# binary at: target/release/wraith
```

---

## Quick Start

```sh
# Set your API key
export ANTHROPIC_API_KEY=sk-ant-...

# Start the interactive REPL
wraith

# Or run a one-shot prompt
wraith "explain the architecture of this repo"

# Explicit prompt mode
wraith prompt "refactor src/main.rs to use the builder pattern"
```

---

## Features

| Feature                 | Description                                                                                                |
| ----------------------- | ---------------------------------------------------------------------------------------------------------- |
| **Conversational REPL** | Persistent session with slash commands, scrollback history, multi-line input                               |
| **Vim mode**            | Full `Normal/Insert/Visual/Replace/Command` editing in the prompt                                          |
| **19 built-in tools**   | File read/write, bash, glob, grep, web fetch, LSP diagnostics, notebook editing, MCP, todos, sub-agents    |
| **Streaming markdown**  | Syntax-highlighted code blocks, tables, lists rendered live — no buffering                                 |
| **Multi-provider**      | Anthropic native, Google Gemini, OpenRouter, xAI Grok + OpenAI-compatible (Bedrock, Vertex, Ollama, local) |
| **Plugin system**       | Hook-based lifecycle events: pre/post tool use, session start/stop, notifications                          |
| **Permission modes**    | 3 modes: `read-only`, `workspace-write`, `danger-full-access` — with per-tool rules                        |
| **Session persistence** | Sessions are automatically persisted to `.wraith/sessions/`                                                |
| **Cost tracking**       | Real-time token and cost display with model-specific pricing                                               |
| **Sub-agent spawning**  | Delegate complex sub-tasks to parallel child agent instances                                               |
| **HTTP server mode**    | Axum-based API for IDE and tool integration                                                                |
| **MCP client**          | Connect to any Model Context Protocol server via stdio transport                                           |
| **Remote runtime**      | SSE-based execution on remote machines                                                                     |

---

## Demo

> A terminal recording showing Wraith in action — coming soon.
>
> ![Wraith demo](docs/assets/demo.gif)

---

## Documentation

- [Configuration](docs/configuration.md) — Settings, environment variables, and config files
- [Providers](docs/providers.md) — Multi-provider AI setup (Anthropic, OpenAI, local models)
- [Tools](docs/tools.md) — Complete reference for all 19 built-in tools
- [Plugin System](docs/plugins.md) — Extend WRAITH with custom tools and hooks
- [Vim Mode](docs/vim-mode.md) — Full vim editing in the conversation prompt

---

## Configuration

WRAITH uses a layered config system with zero required config:

| File                          | Scope        | Purpose                                     |
| ----------------------------- | ------------ | ------------------------------------------- |
| `~/.wraith.json`              | User         | Global defaults                             |
| `.wraith.json`                | Project root | Project settings                            |
| `.wraith/settings.json`       | Project      | Extended settings                           |
| `.wraith/settings.local.json` | Machine      | Local overrides (gitignored)                |
| `WRAITH.md`                   | Project root | Natural language instructions for the agent |
| `.wraith/WRAITH.md`           | Project      | Alternative instruction location            |

Initialize a new project:

```sh
wraith init
```

This scaffolds `WRAITH.md`, `.wraith/`, and `.wraith.json` for you.

---

## Environment Variables

| Variable                         | Description                                            |
| -------------------------------- | ------------------------------------------------------ |
| `ANTHROPIC_API_KEY`              | Anthropic API key (primary auth)                       |
| `GEMINI_API_KEY`                 | Google Gemini API key                                  |
| `XAI_API_KEY`                    | xAI (Grok) API key                                     |
| `OPENAI_API_KEY`                 | OpenAI-compatible API key                              |
| `OPENAI_BASE_URL`                | Base URL for OpenAI-compatible providers               |
| `GEMINI_BASE_URL`                | Override Gemini API base URL                           |
| `OPENROUTER_API_KEY`             | OpenRouter API key                                     |
| `OPENROUTER_BASE_URL`            | Override OpenRouter API base URL                       |
| `WRAITH_MODEL`                   | Default model override (e.g. `gemini-2.5-flash`)       |
| `WRAITH_CONFIG_HOME`             | Override config directory (default: `~/.wraith/`)      |
| `WRAITH_PERMISSION_MODE`         | `read-only`, `workspace-write`, `danger-full-access`   |
| `WRAITH_SANDBOX_FILESYSTEM_MODE` | `read-only`, `disk-write-dangerous`, `containers-only` |
| `WRAITH_REMOTE`                  | Set to `true` to enable remote runtime                 |
| `WRAITH_REMOTE_SESSION_ID`       | Session ID for remote runtime                          |
| `WRAITH_WEB_SEARCH_BASE_URL`     | Override web search endpoint                           |
| `WRAITH_TODO_STORE`              | Override todo store path                               |
| `WRAITH_AGENT_STORE`             | Override agent manifest store path                     |

---

## Multi-Provider Setup

### Anthropic (default)

```sh
export ANTHROPIC_API_KEY=sk-ant-...
wraith
```

### OpenAI / Bedrock / Vertex / Ollama

```sh
export OPENAI_API_KEY=your-key
export OPENAI_BASE_URL=https://api.openai.com/v1
wraith --model gpt-4o
```

### Google Gemini

```sh
export GEMINI_API_KEY=your-key
wraith --model gemini-2.5-flash
```

Supported aliases: `gemini`, `gemini-2.5-pro`, `gemini-2.5-flash`, `gemini-2.0-flash`, `gemini-1.5-pro`.

### OpenRouter

```sh
export OPENROUTER_API_KEY=your-key
wraith --model anthropic/claude-sonnet-4
```

OpenRouter supports hundreds of models via a single API key. Use the full model ID (e.g. `google/gemini-2.5-flash`, `meta-llama/llama-4-maverick`).

### xAI (Grok)

```sh
export XAI_API_KEY=your-key
wraith --model grok-3
```

### Ollama (local)

export OPENAI_BASE_URL=http://localhost:11434/v1
export OPENAI_API_KEY=ollama
wraith --model llama3.1:8b

```

---

## Slash Commands

```

/help Show all commands
/status Session status: model, context, cost
/cost Detailed cost breakdown
/model Show or change active model
/agents List configured agent manifests
/skills List installed skill files
/login Start OAuth authentication flow
/logout Clear saved credentials
/init Scaffold WRAITH.md and config files
/resume Resume a previous session
/vim Toggle vim editing mode
/clear Clear conversation history
/exit Exit WRAITH

````

---

## Plugin System

Plugins are external executables described by a `.wraith-plugin/plugin.json` manifest:

```json
{
  "id": "my-plugin",
  "name": "My Plugin",
  "version": "0.1.0",
  "tools": [
    {
      "name": "my_tool",
      "description": "Does something useful",
      "input_schema": { "type": "object", "properties": {} }
    }
  ]
}
````

The plugin binary receives `WRAITH_PLUGIN_ID`, `WRAITH_PLUGIN_NAME`, `WRAITH_TOOL_NAME`, and `WRAITH_TOOL_INPUT` as environment variables.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup, code style, and PR process.

---

## License

MIT — see [LICENSE](LICENSE).

Originally derived from [claw-code](https://github.com/instructkr/claw-code) (MIT). See [NOTICE](NOTICE) for full attribution.
