# WRAITH — Product Plan

> **"The ghost in your terminal."**

---

## 1. PRODUCT IDENTITY

### Name

**WRAITH** — a cyberpunk-coded, single-syllable, 6-character name. Evokes stealth, speed, and presence without substance. The agent that haunts your codebase.

### Availability

| Platform  | Status                                          |
| --------- | ----------------------------------------------- |
| crates.io | ✅ Available (verified 404 — no existing crate) |
| npm       | 🔍 To verify                                    |
| PyPI      | 🔍 To verify                                    |
| GitHub    | 🔍 To verify `wraith-code` or `wraith-dev`      |
| Domain    | 🔍 To verify `wraith.dev`, `wraith.codes`       |

### Tagline Options

- _"The ghost in your terminal."_
- _"Your codebase, possessed."_
- _"Haunt. Build. Ship."_

### Brand Mood

Disciplined sci-fi. Spaceship control console, not arcade cabinet. Think: Mass Effect terminal UIs, Alien isolation computer screens, Bloomberg Terminal meets Blade Runner.

---

## 2. WHAT WRAITH IS

An **open-source, terminal-native AI coding agent** written in Rust. It lives in your terminal, understands your codebase, and executes multi-step coding tasks autonomously — reading files, writing code, running commands, managing tools — all through a conversational REPL with a cyberpunk soul.

### Core Capabilities (Inherited from Claw Code)

- **Conversational REPL** — persistent session with slash commands, history, multi-line input
- **Vim mode** — full `Normal/Insert/Visual/Replace/Command` editing in the prompt
- **19 built-in tools** — file read/write, bash execution, glob search, grep, web fetch, LSP diagnostics, notebook editing, MCP integration, todo management, sub-agent spawning
- **Streaming markdown renderer** — syntax-highlighted code blocks, tables, lists rendered live in terminal
- **Multi-provider API** — Anthropic native + OpenAI-compatible (Bedrock, Vertex, local models)
- **Plugin system** — hook-based lifecycle (pre/post tool use, notification, session start/stop)
- **Permission system** — 4 modes (default, auto-accept, plan-only, deny-all) with per-tool rules
- **Session persistence** — resume previous conversations
- **Project-level config** — `.wraith.json`, `.wraith/settings.json`, `WRAITH.md` instruction files
- **OAuth authentication** — device flow for managed API access
- **Remote runtime** — SSE-based execution on remote machines
- **HTTP server mode** — Axum-based API for IDE/tool integration
- **LSP integration** — diagnostics from language servers fed into tool results
- **MCP client** — connect to any Model Context Protocol server (stdio transport)
- **Cost tracking** — real-time token/cost display with model-specific pricing
- **Sub-agent spawning** — delegate complex sub-tasks to child agent instances

### What Makes WRAITH Different

1. **Pure Rust** — single binary, no Node/Python runtime, instant startup
2. **Cyberpunk TUI** — not another bland terminal app; glass info panels, neon accents, progress HUD
3. **Gaming-developer DNA** — designed by and for developers who are also gamers
4. **Opinionated defaults** — works out of the box with zero config, customizable for power users
5. **Open source forever** — MIT license, community-driven, no vendor lock-in

---

## 3. DESIGN SYSTEM — "DEEP SPACE / NEON CORE"

### 3.1 Color Palette

```
BACKGROUND LAYER
─────────────────────────────────────────────────────
  Base Background    #0B0F14    Rgb(11, 15, 20)      Deep space black with blue undertone
  Code Block BG      #121821    Rgb(18, 24, 33)      Dark slate — elevated surface
  Panel Border       #1A2233    Rgb(26, 34, 51)      Faint blue-grey — structural lines

TEXT LAYER
─────────────────────────────────────────────────────
  Body Text          #C8C8D2    Rgb(200, 200, 210)   Blue-shifted off-white — easy on eyes
  Dim/Secondary      #646E82    Rgb(100, 110, 130)   Muted blue-grey — timestamps, metadata
  Bright/Emphasis    #FFFFFF    Rgb(255, 255, 255)   Pure white — only for critical emphasis

ACCENT LAYER
─────────────────────────────────────────────────────
  Primary (Cyan)     #00E5FF    Rgb(0, 229, 255)     Prompts, selection, active states, links
  Secondary (Violet) #BB86FC    Rgb(187, 134, 252)   AI output markers, code block frames
  Warning (Amber)    #FFCA28    Rgb(255, 202, 40)    Destructive actions, vim Normal mode
  Success (Green)    #00E676    Rgb(0, 230, 118)     Confirmations, completions, diffs (+)
  Error (Red)        #FF5252    Rgb(255, 82, 82)     Failures, permission denials, diffs (-)
```

### 3.2 ColorTheme Struct Mapping

These map 1:1 to the existing `ColorTheme` struct fields in `render.rs`:

```rust
ColorTheme {
    heading:          Color::Rgb { r: 0,   g: 229, b: 255 },   // Cyan — section headers
    code_border:      Color::Rgb { r: 187, g: 134, b: 252 },   // Violet — code block frames
    code_bg:          Color::Rgb { r: 18,  g: 24,  b: 33  },   // Dark slate — code background
    inline_code:      Color::Rgb { r: 0,   g: 229, b: 255 },   // Cyan — inline `code`
    link:             Color::Rgb { r: 187, g: 134, b: 252 },   // Violet — [links](url)
    list_marker:      Color::Rgb { r: 100, g: 110, b: 130 },   // Dim — bullet points
    blockquote:       Color::Rgb { r: 100, g: 110, b: 130 },   // Dim — quoted text
    table_border:     Color::Rgb { r: 100, g: 110, b: 130 },   // Dim — table structure
    bold:             Color::Rgb { r: 255, g: 202, b: 40  },   // Amber — **bold** emphasis
    emphasis:         Color::Rgb { r: 187, g: 134, b: 252 },   // Violet — *italic* emphasis
    tool_use_border:  Color::Rgb { r: 100, g: 110, b: 130 },   // Dim — tool call frames
}
```

### 3.3 Typography & Symbols

| Element           | Symbol/Style           | Color                      |
| ----------------- | ---------------------- | -------------------------- |
| Prompt (Insert)   | `›`                    | Cyan                       |
| Prompt (Normal)   | `›`                    | Amber                      |
| Spinner           | `▰▰▰▱▱▱` (sliding bar) | Cyan                       |
| Success indicator | `✓`                    | Green                      |
| Error indicator   | `✗`                    | Red                        |
| Warning indicator | `⚠`                    | Amber                      |
| Section separator | `───` (thin rule)      | Dim                        |
| Panel borders     | `┏━┓┃┗━┛` (heavy box)  | Dim border, bright content |
| Info bullet       | `▸`                    | Cyan                       |
| Dim bullet        | `·`                    | Dim                        |

### 3.4 Glass Info Panels

Structured data (status, cost, model info) rendered as bordered card grids instead of plain text dumps:

```
┏━━ Status ━━━━━━━━━━━━━━━━━━━━┓
┃  Model     claude-opus-4-6      ┃
┃  Context   42.1K / 200K tokens  ┃
┃  Session   wraith_a7f3c2        ┃
┃  Cost      $0.23                 ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
```

For multi-section reports, use 2-column card grids:

```
┏━━ Model ━━━━━━━━━━━┓  ┏━━ Cost ━━━━━━━━━━━━┓
┃  Active  opus-4-6   ┃  ┃  Input    $0.12     ┃
┃  Temp    1.0        ┃  ┃  Output   $0.08     ┃
┃  Max     8192       ┃  ┃  Total    $0.20     ┃
┗━━━━━━━━━━━━━━━━━━━━┛  ┗━━━━━━━━━━━━━━━━━━━━┛
```

### 3.5 Startup Banner

```
 ██╗    ██╗██████╗  █████╗ ██╗████████╗██╗  ██╗
 ██║    ██║██╔══██╗██╔══██╗██║╚══██╔══╝██║  ██║
 ██║ █╗ ██║██████╔╝███████║██║   ██║   ███████║
 ██║███╗██║██╔══██╗██╔══██║██║   ██║   ██╔══██║
 ╚███╔███╔╝██║  ██║██║  ██║██║   ██║   ██║  ██║
  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝  ╚═╝

  The ghost in your terminal.            v0.1.0
  Model: claude-opus-4-6   Workspace: ~/project
```

Banner renders in gradient: cyan top → violet bottom. Tagline in dim. Version/model/workspace in body text.

### 3.6 Status Bar (Compact HUD)

Single-line persistent HUD at bottom of REPL output:

```
▸ opus-4-6 │ ~/project │ 42.1K tokens │ $0.23 │ session:a7f3c2
```

Cyan `▸`, dim `│` separators, body text values. Replaces verbose multi-line status dumps.

### 3.7 Hierarchy Principles

1. **Visual hierarchy > color variety** — use whitespace, borders, and indentation to separate sections
2. **Dim by default, bright on demand** — most structural elements are dim; only actionable/important data is bright
3. **Consistent accent mapping** — cyan = interactive/user, violet = AI/system, amber = caution, green = success, red = failure
4. **No color without purpose** — every colored element has a semantic reason

---

## 4. ARCHITECTURE

### 4.1 Crate Map (Post-Rebrand)

```
rust/
├── Cargo.toml                    # Workspace root
└── crates/
    ├── wraith-cli/               # (was: claw-cli) Binary crate — TUI, REPL, rendering
    │   └── src/
    │       ├── main.rs           # Entry point, REPL loop, slash commands
    │       ├── app.rs            # Session config, state management
    │       ├── args.rs           # CLI argument parsing (clap)
    │       ├── init.rs           # Project initialization (.wraith/)
    │       ├── input.rs          # Line editor, vim mode, tab completion
    │       └── render.rs         # Markdown renderer, color theme, spinner
    │
    ├── runtime/                  # Core agent runtime (no name change needed)
    │   └── src/
    │       ├── conversation.rs   # ConversationRuntime — main agent loop
    │       ├── config.rs         # ConfigLoader, settings schema
    │       ├── prompt.rs         # System prompt assembly
    │       ├── permissions.rs    # Permission checking & rules
    │       ├── session.rs        # Session persistence
    │       ├── oauth.rs          # Device flow authentication
    │       ├── sandbox.rs        # Filesystem sandboxing
    │       ├── remote.rs         # Remote runtime (SSE)
    │       ├── hooks.rs          # Hook execution pipeline
    │       ├── bootstrap.rs      # Default config generation
    │       ├── mcp.rs            # MCP protocol types
    │       ├── mcp_client.rs     # MCP client implementation
    │       ├── mcp_stdio.rs      # MCP stdio transport
    │       ├── file_ops.rs       # File operation utilities
    │       ├── bash.rs           # Bash command execution
    │       ├── compact.rs        # Compact message format
    │       ├── json.rs           # JSON extraction utilities
    │       ├── usage.rs          # Token usage tracking
    │       ├── sse.rs            # SSE stream handling
    │       └── lib.rs            # Module exports
    │
    ├── api/                      # API client layer (no name change needed)
    │   └── src/
    │       ├── client.rs         # ProviderClient dispatch
    │       ├── providers/
    │       │   ├── mod.rs        # ProviderKind enum
    │       │   ├── anthropic.rs  # (was: claw_provider.rs) Anthropic API client
    │       │   └── openai_compat.rs  # OpenAI-compatible client
    │       ├── types.rs          # API message types
    │       ├── error.rs          # API error types
    │       ├── sse.rs            # SSE stream parsing
    │       └── lib.rs
    │
    ├── tools/                    # Tool registry & execution (no name change)
    │   └── src/lib.rs            # 19 built-in tools
    │
    ├── commands/                 # Slash command & agent/skill discovery
    │   └── src/lib.rs            # DefinitionSource, command loading
    │
    ├── plugins/                  # Plugin lifecycle & hooks
    │   └── src/
    │       ├── lib.rs            # Plugin manager, hook execution
    │       └── hooks.rs          # Hook type definitions
    │
    ├── lsp/                      # LSP client & manager
    │   └── src/
    │       ├── client.rs         # LSP client connection
    │       ├── manager.rs        # Multi-server management
    │       ├── types.rs          # LSP protocol types
    │       ├── error.rs          # LSP errors
    │       └── lib.rs
    │
    └── server/                   # HTTP API server (Axum)
        └── src/lib.rs            # REST endpoints
```

### 4.2 Key Types & Traits

| Type                        | Crate      | Purpose                                     |
| --------------------------- | ---------- | ------------------------------------------- |
| `ConversationRuntime<C, T>` | runtime    | Generic agent turn loop                     |
| `ApiClient` (trait)         | api        | Provider abstraction                        |
| `ToolExecutor` (trait)      | tools      | Tool execution abstraction                  |
| `AnthropicClient`           | api        | Anthropic API (was `ClawApiClient`)         |
| `OpenAiCompatClient`        | api        | OpenAI-compatible providers                 |
| `ProviderKind`              | api        | `Anthropic`, `OpenAiCompat` (was `ClawApi`) |
| `LiveCli`                   | wraith-cli | REPL state machine                          |
| `LineEditor`                | wraith-cli | Input with vim mode                         |
| `TerminalRenderer`          | wraith-cli | Markdown-to-terminal renderer               |
| `ColorTheme`                | wraith-cli | Theme configuration                         |
| `Spinner`                   | wraith-cli | Async progress indicator                    |
| `PluginManager`             | plugins    | Hook lifecycle manager                      |
| `ConfigLoader`              | runtime    | Hierarchical config resolution              |

### 4.3 Binary

```
Binary name:  wraith
Install:      cargo install wraith
Run:          wraith
              wraith "fix the login bug"
              wraith --model sonnet "explain this function"
              wraith --server --port 8080
```

---

## 5. ENVIRONMENT VARIABLES (Post-Rebrand)

All `CLAW_*` → `WRAITH_*`:

| Old Name                       | New Name                         | Purpose                                           |
| ------------------------------ | -------------------------------- | ------------------------------------------------- |
| `CLAW_MODEL`                   | `WRAITH_MODEL`                   | Default model override                            |
| `CLAW_PERMISSION_MODE`         | `WRAITH_PERMISSION_MODE`         | Permission mode (default/auto-accept/plan/deny)   |
| `CLAW_CONFIG_HOME`             | `WRAITH_CONFIG_HOME`             | Config directory override (default: `~/.wraith/`) |
| `CLAW_CODE_REMOTE`             | `WRAITH_REMOTE`                  | Enable remote runtime                             |
| `CLAW_CODE_REMOTE_SESSION_ID`  | `WRAITH_REMOTE_SESSION_ID`       | Remote session identifier                         |
| `CLAW_SANDBOX_FILESYSTEM_MODE` | `WRAITH_SANDBOX_FILESYSTEM_MODE` | Sandbox constraint mode                           |
| `CLAW_SANDBOX_ALLOWED_MOUNTS`  | `WRAITH_SANDBOX_ALLOWED_MOUNTS`  | Sandbox mount allowlist                           |
| `CLAW_WEB_SEARCH_BASE_URL`     | `WRAITH_WEB_SEARCH_BASE_URL`     | Web search endpoint                               |
| `CLAW_TODO_STORE`              | `WRAITH_TODO_STORE`              | Todo file path override                           |
| `CLAW_AGENT_STORE`             | `WRAITH_AGENT_STORE`             | Agent store path override                         |
| `CLAW_PLUGIN_ID`               | `WRAITH_PLUGIN_ID`               | Plugin ID (set by runtime)                        |
| `CLAW_PLUGIN_NAME`             | `WRAITH_PLUGIN_NAME`             | Plugin name (set by runtime)                      |
| `CLAW_PLUGIN_ROOT`             | `WRAITH_PLUGIN_ROOT`             | Plugin root dir (set by runtime)                  |
| `CLAW_TOOL_NAME`               | `WRAITH_TOOL_NAME`               | Tool name (set by runtime)                        |
| `CLAW_TOOL_INPUT`              | `WRAITH_TOOL_INPUT`              | Tool input JSON (set by runtime)                  |
| `CLAW_CODE_UPSTREAM`           | _(deleted)_                      | Was for compat-harness only                       |

---

## 6. CONFIG FILE PATHS (Post-Rebrand)

| Old Path                    | New Path                      | Purpose                      |
| --------------------------- | ----------------------------- | ---------------------------- |
| `~/.claw/`                  | `~/.wraith/`                  | User config home             |
| `~/.claw/credentials.json`  | `~/.wraith/credentials.json`  | OAuth credentials            |
| `~/.claw/settings.json`     | `~/.wraith/settings.json`     | User settings                |
| `.claw/`                    | `.wraith/`                    | Project config directory     |
| `.claw.json`                | `.wraith.json`                | Project config file          |
| `.claw/settings.json`       | `.wraith/settings.json`       | Project settings             |
| `.claw/settings.local.json` | `.wraith/settings.local.json` | Local-only settings          |
| `.claw/sessions/`           | `.wraith/sessions/`           | Session storage              |
| `.claw/agents/`             | `.wraith/agents/`             | Project agent definitions    |
| `.claw/skills/`             | `.wraith/skills/`             | Project skill definitions    |
| `.claw/commands/`           | `.wraith/commands/`           | Project command definitions  |
| `CLAW.md`                   | `WRAITH.md`                   | Project instruction file     |
| `CLAW.local.md`             | `WRAITH.local.md`             | Local instruction file       |
| `.claw/CLAW.md`             | `.wraith/WRAITH.md`           | Alt instruction location     |
| `.claw/instructions.md`     | `.wraith/instructions.md`     | Alt instructions (unchanged) |
| `.claw-plugin/plugin.json`  | `.wraith-plugin/plugin.json`  | Plugin manifest              |
| `.claw-todos.json`          | `.wraith-todos.json`          | Todo storage                 |
| `.claw-agents/`             | `.wraith-agents/`             | Agent store                  |

---

## 7. FILES TO DELETE

These exist in the source repo and serve no purpose in WRAITH:

### 7.1 Entire Directories to Remove

| Path                          | Reason                                                               |
| ----------------------------- | -------------------------------------------------------------------- |
| `src/` (Python)               | Porting scaffolding — 22 placeholder stubs + 30 skeleton dirs        |
| `tests/`                      | Python test for porting workspace — irrelevant                       |
| `assets/`                     | Empty omx directory — no assets                                      |
| `rust/crates/compat-harness/` | Compatibility testing with upstream claw-code — our divergence point |

### 7.2 Individual Files to Remove

| Path                   | Reason                                          |
| ---------------------- | ----------------------------------------------- |
| `CLAW.md`              | Product instruction file for Claw Code identity |
| `PARITY.md`            | Parity tracking with upstream — we're diverging |
| `README.md` (root)     | Claw Code README — replaced by WRAITH README    |
| `rust/README.md`       | Claw Code Rust README — replaced                |
| `rust/CONTRIBUTING.md` | Claw Code contribution guide — replaced         |
| `src/__init__.py`      | Python package init                             |

---

## 8. FILES TO CREATE

| Path                                | Purpose                                             |
| ----------------------------------- | --------------------------------------------------- |
| `README.md`                         | WRAITH project README with install, usage, features |
| `LICENSE`                           | MIT license file (currently missing!)               |
| `WRAITH.md`                         | Default project instruction template                |
| `CONTRIBUTING.md`                   | WRAITH contribution guide                           |
| `CHANGELOG.md`                      | Release changelog                                   |
| `.gitignore`                        | Clean gitignore for Rust project                    |
| `rust/crates/wraith-cli/Cargo.toml` | Renamed from claw-cli                               |

---

## 9. COMPETITIVE POSITIONING

### 9.1 Landscape

| Tool        | Language   | Interface | Stars | Weakness WRAITH Exploits           |
| ----------- | ---------- | --------- | ----- | ---------------------------------- |
| Claude Code | TypeScript | Terminal  | 105K  | Closed-source, Anthropic-only      |
| Aider       | Python     | Terminal  | 43K   | Python startup overhead, plain UI  |
| Codex CLI   | TypeScript | Terminal  | 72K   | OpenAI-only, basic rendering       |
| Cline       | TypeScript | VS Code   | 60K   | IDE-locked, heavy extension        |
| OpenHands   | Python     | Web UI    | 70K   | Not terminal-native, complex setup |
| Goose       | Rust       | Terminal  | 34K   | Enterprise-focused, generic brand  |
| Continue    | TypeScript | VS Code   | 32K   | IDE-locked, not standalone         |

### 9.2 WRAITH's Angle

```
        Terminal-Native
             ↑
     Goose  │  WRAITH ★
     Codex  │  Aider
             │
  Closed ────┼──── Open Source
             │
    Claude   │  Continue
    Cursor   │  Cline
             │
             ↓
        IDE-Integrated
```

WRAITH occupies the **open-source + terminal-native** quadrant with a unique cyberpunk identity that no competitor has. The Rust single-binary advantage (instant startup, zero dependencies) is shared only with Goose, but Goose targets enterprise — WRAITH targets individual developers and power users.

### 9.3 Viral Hooks

1. **Screenshot-worthy output** — glass panels and neon render beautifully in tweets/posts
2. **"wraith" as a verb** — "just wraith it" is naturally memeable
3. **Gaming crossover** — appeals to the massive developer-gamer overlap audience
4. **Single `cargo install wraith`** — zero-friction onboarding
5. **Startup banner** — ASCII art logo is shareable/recognizable

---

## 10. LEGAL

### 10.1 Source License

The upstream `claw-code` repo uses **MIT License**. MIT permits:

- ✅ Commercial use
- ✅ Modification
- ✅ Distribution
- ✅ Private use
- ✅ Complete rebrand

**Obligation**: Include the original MIT license notice in the distribution. This goes in a `LICENSE` or `NOTICE` file.

### 10.2 What to Remove

- All references to `platform.claw.dev` OAuth endpoints (these are upstream infrastructure we don't control)
- The `claw_code` OAuth scope
- The hardcoded `client_id` UUID (belongs to upstream)
- Any references to `instructkr` or the original repo URL

### 10.3 What to Preserve

- Original MIT license text in a `NOTICE` or `THIRD-PARTY-LICENSES` file
- Attribution comment at the top of `LICENSE`: "Originally derived from claw-code (MIT License)"

### 10.4 Trademark

- "WRAITH" is not trademarked in the software tools category (to be verified with USPTO/EUIPO before commercial use)
- The name is available on crates.io (verified)
- Register the crate name early with a placeholder publish to reserve it

---

## 11. SUCCESS METRICS (v1.0)

| Metric                   | Target                                                         |
| ------------------------ | -------------------------------------------------------------- |
| Binary compiles          | Single `cargo build` produces `wraith` binary                  |
| Zero "claw" references   | `grep -ri claw` returns 0 hits in source                       |
| All 19 tools functional  | Tool smoke test passes                                         |
| Streaming markdown works | Code blocks, tables, lists render correctly                    |
| Vim mode works           | Insert/Normal/Visual/Replace/Command modes                     |
| Session persistence      | Resume works across restarts                                   |
| Config loading           | `.wraith.json` and `~/.wraith/` paths work                     |
| Plugin loading           | `.wraith-plugin/plugin.json` discovered                        |
| Multi-provider           | Anthropic + OpenAI-compat both connect                         |
| Theme applied            | Cyberpunk palette renders in 256-color and truecolor terminals |
| Star count               | 1K stars within 3 months of launch                             |
| Binary size              | < 20MB release build                                           |
| Startup time             | < 100ms to REPL prompt                                         |

---

## 12. NON-GOALS (v1.0)

To keep scope controlled, these are explicitly **NOT** in v1.0:

- ❌ Web UI / browser interface
- ❌ VS Code extension
- ❌ GUI / Electron wrapper
- ❌ Built-in model hosting / local inference
- ❌ Team/collaboration features
- ❌ Billing / payment system
- ❌ Custom OAuth infrastructure (use API keys directly)
- ❌ Windows native support (WSL works fine)
- ❌ Plugin marketplace
- ❌ Telemetry / analytics

---

_This document is the single source of truth for WRAITH's product identity, design system, architecture, and competitive strategy. Every implementation decision should reference this plan._
