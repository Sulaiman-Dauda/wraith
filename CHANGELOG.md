# Changelog

All notable changes to WRAITH will be documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
WRAITH uses [Semantic Versioning](https://semver.org/).

---

## [Unreleased]

## [0.1.0] — 2026-04-02

### Added

- **WRAITH Identity**: Complete rebrand from claw-code to WRAITH with full cyberpunk theming
- **Deep Space / Neon Core Design**: Cyan (#00E5FF), Violet (#BB86FC), Amber (#FFCA28), Green (#00E676), Red (#FF5252) color palette
- **ASCII Art Banner**: WRAITH logo displayed on startup with "The ghost in your terminal" tagline
- **Glass Info Panels**: Transparent-styled info displays with neon borders
- **Violet Code Blocks**: Syntax-highlighted code blocks with cyberpunk styling
- **Tool Call Styling**: Visual indicators (✓/✗) for tool execution status
- **Colored Diff Output**: Syntax-highlighted file diffs and changes
- **Enhanced System Prompt**: "You are Wraith, an AI coding agent..." with WRAITH identity
- **Conversational REPL** with slash commands, scrollback history, and multi-line input
- **Vim Editing Mode**: Complete Normal/Insert/Visual/Command mode implementation in prompt
  - Movement commands (h/j/k/l, w/b/e, 0/$, gg/G)
  - Text operations (i/a/o/A, x/dd/D/cc/C, yy/p/P)
  - Visual selection mode and command mode
  - Color-coded prompt indicator (cyan for insert, amber for normal)
- **19 Built-in Tools**: Complete toolkit for development workflows
  - **File Operations**: read_file, write_file, edit_file
  - **Search & Discovery**: glob_search, grep_search
  - **Command Execution**: bash, PowerShell, REPL
  - **Web & Network**: WebFetch, WebSearch
  - **Productivity**: TodoWrite, Config, Sleep
  - **Communication**: SendUserMessage, StructuredOutput
  - **Development**: NotebookEdit
  - **Advanced**: Skill, Agent, ToolSearch
- **Streaming Markdown Renderer**: Real-time syntax highlighting, tables, lists without buffering
- **Multi-Provider API Support**:
  - Anthropic Claude (native implementation)
  - OpenAI-compatible providers (OpenAI, Bedrock, Vertex, Ollama, LM Studio)
  - Model aliases and switching via --model flag
- **Plugin System**: Hook-based architecture for extensibility
  - PreToolUse and PostToolUse lifecycle hooks
  - Plugin manifests with tools, permissions, and metadata
  - Environment variable injection (`WRAITH_PLUGIN_*`, `WRAITH_TOOL_*`, `HOOK_*`)
  - Bundled example plugins and documentation
- **Permission System**: Four-tier security model
  - default: Prompt for dangerous operations
  - acceptEdits: Auto-approve file edits, prompt for commands
  - plan: Show execution plan without running tools
  - bypassPermissions: Run all tools without prompting
  - Per-tool allowlist/denylist configuration
- **Session Persistence**: Resume conversations with --resume SESSION.json
- **Cost Tracking**: Real-time token usage and cost display with provider-specific pricing
- **Sub-Agent Spawning**: Delegate complex tasks to specialized child agents
- **HTTP Server Mode**: Axum-based API for IDE integration
- **MCP Client**: Model Context Protocol support for external server integration
- **Remote Runtime**: SSE-based execution on remote machines
- **Comprehensive Configuration**:
  - Layered config: user → project → local → environment variables
  - .wraith.json project config with model, permissions, tools, MCP servers
  - .wraith/settings.json and settings.local.json (git-ignored)
  - ~/.wraith/settings.json user-level config
  - WRAITH.md instruction files (project context)
  - WRAITH\_\* environment variables for all settings
- **Project Initialization**: wraith init scaffolds config files and instructions
- **Complete Documentation**: 5 detailed guides covering all features
  - Configuration management and environment setup
  - Multi-provider AI model configuration
  - Plugin development and hook system
  - Complete tool reference with examples
  - Vim mode keybindings and workflow

### Changed

- **Binary Name**: claw → wraith (published as wraith-cli crate)
- **Project Identity**: Complete migration from claw-code branding to WRAITH
- **API Architecture**: Clean provider abstraction with AnthropicClient
- **Configuration Paths**: .claw/ → .wraith/, CLAW.md → WRAITH.md
- **Environment Variables**: All CLAW*\* → WRAITH*\* prefixes
- **Color Scheme**: From generic terminal colors to cyberpunk Deep Space theme
- **UI/UX**: Glass panels, neon highlights, and structured information display
- **Authentication**: Direct API key flow replacing platform OAuth

### Removed

- **Legacy OAuth**: Removed platform.claw.dev authentication endpoints
- **Compatibility Shim**: Removed compat-harness crate (upstream compatibility layer)
- **Old Branding**: All claw-code references, logos, and identity elements

### Technical Details

- **Rust Edition**: 2021 with modern async/await patterns
- **Dependencies**: Tokio runtime, crossterm terminal handling, rustyline input
- **Architecture**: Modular crate structure (api, runtime, tools, plugins, wraith-cli)
- **Performance**: Zero-allocation streaming, efficient terminal rendering
- **Security**: Sandboxed execution modes, permission validation, safe defaults
