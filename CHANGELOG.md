# Changelog

All notable changes to WRAITH will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
WRAITH uses [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

## [0.1.0] — Unreleased

### Added

- Initial release of WRAITH — an open-source, terminal-native AI coding agent in Rust
- **Conversational REPL** with slash commands, scrollback history, and multi-line input
- **Vim editing mode** — full Normal/Insert/Visual/Replace/Command modes in the prompt
- **19 built-in tools**: file read/write, bash execution, glob search, grep, web fetch, LSP
  diagnostics, notebook editing, MCP integration, todo management, sub-agent spawning
- **Streaming markdown renderer** — syntax-highlighted code, tables, and lists rendered live
- **Multi-provider API** — Anthropic native + OpenAI-compatible (Bedrock, Vertex, Ollama)
- **Plugin system** — hook-based lifecycle events (pre/post tool use, session start/stop)
- **Permission modes** — default, auto-accept, plan-only, deny-all with per-tool rules
- **Session persistence** — resume previous conversations with `wraith --resume SESSION.json`
- **Cost tracking** — real-time token and cost display with model-specific pricing
- **Sub-agent spawning** — delegate complex sub-tasks to child agent instances
- **HTTP server mode** — Axum-based API for IDE and tool integration
- **MCP client** — connect to any Model Context Protocol server via stdio transport
- **Remote runtime** — SSE-based execution on remote machines
- **Deep Space / Neon Core** theme — cyberpunk color palette (cyan, violet, amber on dark)
- **Project config system** — `.wraith.json`, `.wraith/settings.json`, `WRAITH.md`
- `wraith init` — scaffolds `WRAITH.md`, `.wraith/`, `.wraith.json` in any project
- Layered config merging: user → project → local overrides
- `WRAITH_CONFIG_HOME`, `WRAITH_PERMISSION_MODE`, `WRAITH_SANDBOX_*` environment variables

### Changed

- Renamed from `claw-code` — complete rebrand to WRAITH identity
- API provider module renamed from `claw_provider` to `anthropic`
- `ClawApiClient` → `AnthropicClient`
- All config paths migrated: `.claw/` → `.wraith/`, `CLAW.md` → `WRAITH.md`
- All environment variable prefixes migrated: `CLAW_*` → `WRAITH_*`
- Binary name: `claw` → `wraith`

### Removed

- `compat-harness` crate (upstream compatibility shim, not needed for WRAITH)
- Upstream OAuth endpoints (`platform.claw.dev`) — replaced with direct API key auth
