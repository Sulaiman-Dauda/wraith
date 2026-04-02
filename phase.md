# WRAITH — Implementation Phases

> Execution plan for transforming `claw-code` into **WRAITH**.
> Every task has a verification step. Nothing moves forward unverified.

---

## PHASE 0: STAGING

**Goal**: Clean workspace, set up new repo, establish baseline.

### Tasks

- [x] **0.1** Create new directory `wraith/` outside the claw-code repos
- [x] **0.2** Copy the entire `rust/` directory from `claw-code-main` into `wraith/rust/`
- [x] **0.3** Do NOT copy: `src/`, `tests/`, `assets/`, `CLAW.md`, `PARITY.md`, root `README.md`
- [x] **0.4** Delete `rust/crates/compat-harness/` entirely from the new copy
- [x] **0.5** Remove `"crates/compat-harness"` from workspace `Cargo.toml` members (if explicitly listed)
- [x] **0.6** `git init` in `wraith/` — clean history, no upstream baggage
- [x] **0.7** Create `.gitignore`:
  ```
  /target
  Cargo.lock
  *.swp
  *.swo
  .DS_Store
  ```
- [x] **0.8** Initial commit: `"chore: initialize wraith from claw-code scaffold"`
- [x] **0.9** Verify: `cargo check` passes in the new workspace (even with claw names still present)

**Exit Criteria**: Clean git repo with Rust workspace that compiles. No Python, no compat-harness, no upstream docs.

---

## PHASE 1: THE GREAT RENAME

**Goal**: Mechanically replace every "claw" reference with "wraith". Zero functional changes, zero new features.

### 1A — Crate Rename

- [x] **1A.1** Rename directory: `rust/crates/claw-cli/` → `rust/crates/wraith-cli/`
- [x] **1A.2** Update `wraith-cli/Cargo.toml`:
  - `name = "wraith-cli"`
  - `[[bin]] name = "wraith"`
  - Update any `claw-cli` dependency references
- [x] **1A.3** Update workspace `Cargo.toml` if it references `claw-cli` explicitly
- [x] **1A.4** Update all internal crate dependencies that reference `claw-cli` (grep for `claw-cli` in all `Cargo.toml` files)
- [x] **1A.5** Verify: `cargo check` passes

### 1B — API Provider Rename

- [x] **1B.1** Rename file: `api/src/providers/claw_provider.rs` → `api/src/providers/anthropic.rs`
- [x] **1B.2** Update `api/src/providers/mod.rs`:
  - `mod claw_provider` → `mod anthropic`
  - `ProviderKind::ClawApi` → `ProviderKind::Anthropic`
  - `pub use claw_provider::*` → `pub use anthropic::*`
- [x] **1B.3** In `anthropic.rs` (was `claw_provider.rs`):
  - `ClawApiClient` → `AnthropicClient`
  - `ClawApiError` → `AnthropicApiError` (if exists)
  - Any doc comments referencing "Claw"
- [x] **1B.4** Update `api/src/client.rs`:
  - `ProviderClient::ClawApi(ClawApiClient)` → `ProviderClient::Anthropic(AnthropicClient)`
  - All match arms referencing `ClawApi`
- [x] **1B.5** Update `api/src/lib.rs` if it re-exports claw types
- [x] **1B.6** Verify: `cargo check` passes

### 1C — String Literals & User-Facing Text

Systematic file-by-file replacement. Each file gets its own checklist:

#### wraith-cli/src/main.rs (~45 occurrences)

- [x] **1C.1** `"Claw Code"` → `"Wraith"` (all display strings)
- [x] **1C.2** `"claw"` → `"wraith"` (binary name references, help text)
- [x] **1C.3** `"🦞 Claw Code"` → replace with WRAITH banner (Phase 3 will do ASCII art, for now just `"WRAITH"`)
- [x] **1C.4** `CLAW_PERMISSION_MODE` → `WRAITH_PERMISSION_MODE`
- [x] **1C.5** OAuth URLs: `platform.claw.dev` → comment out or replace with placeholder `// TODO: WRAITH OAuth endpoint`
- [x] **1C.6** OAuth `client_id` → comment out with `// TODO: WRAITH client_id`
- [x] **1C.7** OAuth scope `user:sessions:claw_code` → comment out
- [x] **1C.8** `claw_default()` function calls → `wraith_default()`
- [x] **1C.9** `.claw/sessions/` → `.wraith/sessions/`
- [x] **1C.10** Any error messages containing "claw" or "Claw"

#### wraith-cli/src/init.rs (~25 occurrences)

- [x] **1C.11** `.claw/` → `.wraith/`
- [x] **1C.12** `.claw.json` → `.wraith.json`
- [x] **1C.13** `CLAW.md` → `WRAITH.md`
- [x] **1C.14** Gitignore entries: `.claw/settings.local.json` → `.wraith/settings.local.json`
- [x] **1C.15** Gitignore entries: `.claw/sessions/` → `.wraith/sessions/`
- [x] **1C.16** Any function names containing `claw`
- [x] **1C.17** Any doc comments referencing "Claw"

#### wraith-cli/src/args.rs

- [x] **1C.18** `#[command(name = "claw-cli")]` → `#[command(name = "wraith")]`
- [x] **1C.19** `about = "Claw Code CLI"` → `about = "The ghost in your terminal."`

#### wraith-cli/src/app.rs

- [x] **1C.20** Any "claw" references in session config defaults or command descriptions

#### wraith-cli/src/input.rs

- [x] **1C.21** Any "claw" references in prompt text or help strings

#### wraith-cli/src/render.rs

- [x] **1C.22** Any "claw" references in comments or doc strings

#### runtime/src/config.rs (~30 occurrences)

- [x] **1C.23** `CLAW_SETTINGS_SCHEMA_NAME` → `WRAITH_SETTINGS_SCHEMA_NAME`
- [x] **1C.24** `.claw.json` → `.wraith.json`
- [x] **1C.25** `.claw/settings.json` → `.wraith/settings.json`
- [x] **1C.26** `.claw/settings.local.json` → `.wraith/settings.local.json`
- [x] **1C.27** `CLAW_CONFIG_HOME` → `WRAITH_CONFIG_HOME`
- [x] **1C.28** `~/.claw/` → `~/.wraith/`
- [x] **1C.29** All error messages and log strings

#### runtime/src/prompt.rs (~18 occurrences)

- [x] **1C.30** `CLAW.md` → `WRAITH.md`
- [x] **1C.31** `CLAW.local.md` → `WRAITH.local.md`
- [x] **1C.32** `.claw/CLAW.md` → `.wraith/WRAITH.md`
- [x] **1C.33** `.claw/instructions.md` → `.wraith/instructions.md`
- [x] **1C.34** System prompt text mentioning "Claw" or "claw"

#### runtime/src/bootstrap.rs

- [x] **1C.35** `claw_default()` → `wraith_default()`
- [x] **1C.36** Any string literals containing "claw"

#### runtime/src/oauth.rs

- [x] **1C.37** `~/.claw/credentials.json` → `~/.wraith/credentials.json`
- [x] **1C.38** `CLAW_CONFIG_HOME` → `WRAITH_CONFIG_HOME`

#### runtime/src/sandbox.rs

- [x] **1C.39** `CLAW_SANDBOX_FILESYSTEM_MODE` → `WRAITH_SANDBOX_FILESYSTEM_MODE`
- [x] **1C.40** `CLAW_SANDBOX_ALLOWED_MOUNTS` → `WRAITH_SANDBOX_ALLOWED_MOUNTS`

#### runtime/src/remote.rs (~7 occurrences)

- [x] **1C.41** `CLAW_CODE_REMOTE` → `WRAITH_REMOTE`
- [x] **1C.42** `CLAW_CODE_REMOTE_SESSION_ID` → `WRAITH_REMOTE_SESSION_ID`
- [x] **1C.43** Any error messages or log text

#### tools/src/lib.rs (~11 occurrences)

- [x] **1C.44** `CLAW_WEB_SEARCH_BASE_URL` → `WRAITH_WEB_SEARCH_BASE_URL`
- [x] **1C.45** `CLAW_TODO_STORE` → `WRAITH_TODO_STORE`
- [x] **1C.46** `CLAW_AGENT_STORE` → `WRAITH_AGENT_STORE`
- [x] **1C.47** `CLAW_CONFIG_HOME` → `WRAITH_CONFIG_HOME`
- [x] **1C.48** `.claw-todos.json` → `.wraith-todos.json`
- [x] **1C.49** `.claw-agents` → `.wraith-agents`

#### commands/src/lib.rs (~12 occurrences)

- [x] **1C.50** `DefinitionSource::ProjectClaw` → `DefinitionSource::ProjectWraith`
- [x] **1C.51** `.claw/agents` → `.wraith/agents`
- [x] **1C.52** `.claw/skills` → `.wraith/skills`
- [x] **1C.53** `.claw/commands` → `.wraith/commands`
- [x] **1C.54** Any display strings or doc comments

#### plugins/src/lib.rs (~8 occurrences)

- [x] **1C.55** `.claw-plugin/plugin.json` → `.wraith-plugin/plugin.json`
- [x] **1C.56** `CLAW_PLUGIN_ID` → `WRAITH_PLUGIN_ID`
- [x] **1C.57** `CLAW_PLUGIN_NAME` → `WRAITH_PLUGIN_NAME`
- [x] **1C.58** `CLAW_PLUGIN_ROOT` → `WRAITH_PLUGIN_ROOT`
- [x] **1C.59** `CLAW_TOOL_NAME` → `WRAITH_TOOL_NAME`
- [x] **1C.60** `CLAW_TOOL_INPUT` → `WRAITH_TOOL_INPUT`

#### api/src/client.rs

- [x] **1C.61** `"Hello from Claw"` or similar test strings → `"Hello from Wraith"`

#### runtime/src/lib.rs

- [x] **1C.62** Any re-exports or doc strings with "claw"

### 1D — Final Verification

- [x] **1D.1** Run: `grep -ri "claw" --include="*.rs" --include="*.toml" rust/` — expect 0 results
- [x] **1D.2** Run: `cargo check` — must pass clean
- [x] **1D.3** Run: `cargo build --release` — must produce `wraith` binary in `target/release/`
- [x] **1D.4** Run: `./target/release/wraith --help` — verify branding says "Wraith"
- [x] **1D.5** Git commit: `"feat: complete claw → wraith rebrand"`

**Exit Criteria**: Zero "claw" references remain. Binary compiles as `wraith`. All types renamed. All paths updated.

---

## PHASE 2: IDENTITY FILES

**Goal**: Create all new product identity files. The repo should look like a real open-source project.

- [x] **2.1** Create `LICENSE` (MIT) with:

  ```
  MIT License

  Copyright (c) 2025 WRAITH Contributors
  Originally derived from claw-code by instructkr (MIT License)

  [standard MIT text]
  ```

- [x] **2.2** Create `NOTICE` or `THIRD-PARTY-LICENSES`:
  ```
  This project was originally derived from:
    claw-code — https://github.com/instructkr/claw-code
    License: MIT
  ```
- [x] **2.3** Create `README.md` — full project README:
  - WRAITH ASCII banner
  - One-line description
  - Install: `cargo install wraith`
  - Quick start (3-command demo)
  - Feature list with terminal screenshots
  - Configuration section
  - Environment variables table
  - Multi-provider setup (Anthropic, OpenAI, Bedrock)
  - Plugin guide
  - Contributing link
  - License section
- [x] **2.4** Create `CONTRIBUTING.md`:
  - Development setup
  - Code style (rustfmt, clippy)
  - PR process
  - Issue templates
  - Architecture overview
- [x] **2.5** Create `CHANGELOG.md`:

  ```
  # Changelog

  ## [0.1.0] — Unreleased
  ### Added
  - Initial release of WRAITH
  - Deep Space / Neon Core terminal theme
  - [all features from product-plan.md]
  ```

- [x] **2.6** Create `WRAITH.md` (default project instruction template):

  ```markdown
  # WRAITH Project Instructions

  This file provides context to WRAITH about your project.
  Place it in your project root or in `.wraith/WRAITH.md`.

  ## Project Overview

  [Describe your project here]

  ## Conventions

  [List your coding conventions]

  ## Important Context

  [Anything WRAITH should know when working in this codebase]
  ```

- [x] **2.7** Update `.gitignore` in repo root
- [x] **2.8** Git commit: `"docs: add identity files (README, LICENSE, CONTRIBUTING, CHANGELOG)"`

**Exit Criteria**: Repo has all standard open-source project files. Landing page (README) is polished and compelling.

---

## PHASE 3: CYBERPUNK THEME — COLORS & SPINNER

**Goal**: Apply the Deep Space / Neon Core color palette and new spinner animation.

### 3A — Color Theme

- [x] **3A.1** Open `wraith-cli/src/render.rs`
- [x] **3A.2** Replace the `ColorTheme::default()` implementation with cyberpunk palette:
  ```rust
  impl Default for ColorTheme {
      fn default() -> Self {
          Self {
              heading:         Color::Rgb { r: 0,   g: 229, b: 255 },
              code_border:     Color::Rgb { r: 187, g: 134, b: 252 },
              code_bg:         Color::Rgb { r: 18,  g: 24,  b: 33  },
              inline_code:     Color::Rgb { r: 0,   g: 229, b: 255 },
              link:            Color::Rgb { r: 187, g: 134, b: 252 },
              list_marker:     Color::Rgb { r: 100, g: 110, b: 130 },
              blockquote:      Color::Rgb { r: 100, g: 110, b: 130 },
              table_border:    Color::Rgb { r: 100, g: 110, b: 130 },
              bold:            Color::Rgb { r: 255, g: 202, b: 40  },
              emphasis:        Color::Rgb { r: 187, g: 134, b: 252 },
              tool_use_border: Color::Rgb { r: 100, g: 110, b: 130 },
          }
      }
  }
  ```
- [x] **3A.3** Verify: `cargo build` passes
- [x] **3A.4** Visual test: run `wraith` and confirm colors render correctly
- [x] **3A.5** Git commit: `"style: apply Deep Space / Neon Core color palette"`

### 3B — Spinner

- [x] **3B.1** In `render.rs`, locate `Spinner` struct and its frame animation
- [x] **3B.2** Replace braille dots with sliding progress bar:
  ```rust
  const SPINNER_FRAMES: &[&str] = &[
      "▰▱▱▱▱▱▱", "▰▰▱▱▱▱▱", "▰▰▰▱▱▱▱", "▰▰▰▰▱▱▱",
      "▱▰▰▰▰▱▱", "▱▱▰▰▰▰▱", "▱▱▱▰▰▰▰", "▱▱▱▱▰▰▰",
      "▱▱▱▱▱▰▰", "▱▱▱▱▱▱▰",
  ];
  ```
- [x] **3B.3** Set spinner color to Cyan (`Rgb(0, 229, 255)`)
- [x] **3B.4** Verify: spinner animates smoothly during API calls
- [x] **3B.5** Git commit: `"style: replace braille spinner with progress bar animation"`

### 3C — Prompt Character

- [x] **3C.1** In `wraith-cli/src/input.rs`, locate prompt string
- [x] **3C.2** Change prompt from `"> "` to `"› "` with Cyan color
- [x] **3C.3** For vim Normal mode, change prompt color to Amber (`Rgb(255, 202, 40)`)
- [x] **3C.4** Verify: prompt renders correctly in both Insert and Normal modes
- [x] **3C.5** Git commit: `"style: cyberpunk prompt character with vim mode color"`

**Exit Criteria**: WRAITH looks cyberpunk. Colors are neon on dark. Spinner is a progress bar. Prompt is `›` with mode-aware colors.

---

## PHASE 4: STARTUP BANNER & STATUS HUD

**Goal**: Replace the bland startup message with the WRAITH ASCII art banner and implement compact status display.

### 4A — ASCII Banner

- [x] **4A.1** In `wraith-cli/src/main.rs`, locate `startup_banner()` or equivalent function
- [x] **4A.2** Replace with WRAITH block-letter ASCII art:
  ```
   ██╗    ██╗██████╗  █████╗ ██╗████████╗██╗  ██╗
   ██║    ██║██╔══██╗██╔══██╗██║╚══██╔══╝██║  ██║
   ██║ █╗ ██║██████╔╝███████║██║   ██║   ███████║
   ██║███╗██║██╔══██╗██╔══██║██║   ██║   ██╔══██║
   ╚███╔███╔╝██║  ██║██║  ██║██║   ██║   ██║  ██║
    ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝  ╚═╝
  ```
- [x] **4A.3** Render banner in cyan-to-violet gradient (top lines cyan, bottom lines violet)
- [x] **4A.4** Add tagline below: `"  The ghost in your terminal."` in dim
- [x] **4A.5** Add compact info line: `"  Model: {model}   Workspace: {cwd}   v{version}"` in body text
- [x] **4A.6** Verify: banner displays on startup, fits in 80-column terminal
- [x] **4A.7** Git commit: `"feat: WRAITH ASCII art startup banner"`

### 4B — Glass Info Panels

- [x] **4B.1** Create a `render_glass_panel()` helper function in `render.rs`:

  ```rust
  fn render_glass_panel(title: &str, rows: &[(&str, &str)], width: usize) -> String
  ```

  Renders:

  ```
  ┏━━ Title ━━━━━━━━━━━━━━━━━━━━┓
  ┃  Key     Value                ┃
  ┃  Key     Value                ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
  ```

  - Title in cyan, border in dim, keys in dim, values in body text

- [x] **4B.2** Create `render_glass_panel_pair()` for side-by-side 2-column panels
- [x] **4B.3** Verify: panels render correctly at various terminal widths
- [x] **4B.4** Git commit: `"feat: glass info panel rendering primitives"`

### 4C — Refactor Status/Cost/Model Reports

- [x] **4C.1** Refactor `format_status_report()` to use glass panels:
  ```
  ┏━━ Status ━━━━━━━━━━━━━━━━━━━━┓
  ┃  Model     claude-opus-4-6      ┃
  ┃  Context   42.1K / 200K tokens  ┃
  ┃  Session   wraith_a7f3c2        ┃
  ┃  Cost      $0.23                 ┃
  ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
  ```
- [x] **4C.2** Refactor `format_cost_report()` to use glass panel
- [x] **4C.3** Refactor `format_model_report()` to use glass panel
- [x] **4C.4** Refactor `format_compact_report()` to use compact HUD line:
  ```
  ▸ opus-4-6 │ ~/project │ 42.1K tokens │ $0.23 │ session:a7f3c2
  ```
- [x] **4C.5** Refactor `format_permissions_report()` to use glass panel
- [x] **4C.6** Verify: all `/status`, `/cost`, `/model` slash commands render with panels
- [x] **4C.7** Git commit: `"feat: glass panel rendering for status/cost/model reports"`

**Exit Criteria**: Startup shows WRAITH ASCII art with gradient. All structured data uses glass panels. Compact HUD available for inline status.

---

## PHASE 5: CODE BLOCK & TOOL OUTPUT STYLING

**Goal**: Make code blocks and tool call output visually distinctive and cyberpunk.

### 5A — Code Blocks

- [x] **5A.1** In `render.rs`, update `start_code_block()`:
  - Border color: Violet (`Rgb(187, 134, 252)`)
  - Background: Dark slate (`Rgb(18, 24, 33)`)
  - Language label rendered in dim text at top-right of border
- [x] **5A.2** Update `finish_code_block()` with matching violet bottom border
- [x] **5A.3** Verify: code blocks have visible violet frame, dark background, language label
- [x] **5A.4** Git commit: `"style: violet-framed code blocks with language labels"`

### 5B — Tool Call Rendering

- [x] **5B.1** Locate tool use rendering in `main.rs` (tool call display)
- [x] **5B.2** Style tool calls with:
  - Dim border (not same as code blocks — tool_use_border color)
  - Tool name in cyan bold
  - Tool input in body text
  - Tool result with success (green ✓) or error (red ✗) prefix
- [x] **5B.3** Verify: tool calls are visually distinct from AI prose and code blocks
- [x] **5B.4** Git commit: `"style: tool call rendering with status indicators"`

### 5C — Diff Output

- [x] **5C.1** If diff/patch output exists, ensure:
  - Added lines: Green prefix `+`
  - Removed lines: Red prefix `-`
  - Context lines: Dim
  - File headers: Cyan bold
- [x] **5C.2** Git commit: `"style: colored diff output"`

**Exit Criteria**: Code blocks have violet frames. Tool calls have clear visual separation. Diffs are color-coded.

---

## PHASE 6: SYSTEM PROMPT POLISH

**Goal**: Ensure the system prompt identifies as WRAITH, not Claw or Claude.

- [ ] **6.1** In `runtime/src/prompt.rs`, locate the system prompt assembly
- [ ] **6.2** Replace all identity references:
  - "You are Claw Code" → "You are Wraith"
  - "Claw Code is..." → "Wraith is..."
  - Any "Claude Code" references → remove or replace
- [ ] **6.3** Update the WRAITH identity in system prompt to match product positioning:
  - "You are Wraith, an AI coding agent that lives in the terminal."
  - Reference cyberpunk personality if appropriate
- [ ] **6.4** Verify: system prompt contains zero "claw" or "Claude" identity references
- [ ] **6.5** Review tool descriptions for any "claw" references and update
- [ ] **6.6** Git commit: `"feat: WRAITH system prompt identity"`

**Exit Criteria**: The AI introduces itself as Wraith. System prompt reflects WRAITH identity. No upstream identity leakage.

---

## PHASE 7: OAUTH CLEANUP & API KEY MODE

**Goal**: Remove dead OAuth infrastructure, ensure API key auth works cleanly.

- [ ] **7.1** In `main.rs`, remove or disable the OAuth device flow code that references `platform.claw.dev`:
  - Comment out `platform.claw.dev` URLs
  - Comment out the `client_id` UUID
  - Comment out the `user:sessions:claw_code` scope
  - Keep the OAuth flow structure intact for future WRAITH OAuth (if needed)
- [ ] **7.2** Ensure API key authentication via `ANTHROPIC_API_KEY` env var works without OAuth
- [ ] **7.3** Ensure OpenAI-compatible auth via `OPENAI_API_KEY` works
- [ ] **7.4** Update startup flow:
  - If no API key found, show helpful error with instructions
  - Don't attempt OAuth flow to dead endpoints
- [ ] **7.5** In `runtime/src/oauth.rs`:
  - Update credential path to `~/.wraith/credentials.json`
  - Ensure no hardcoded upstream URLs remain
- [ ] **7.6** Verify: `wraith` starts cleanly with `ANTHROPIC_API_KEY` set, no OAuth errors
- [ ] **7.7** Git commit: `"fix: disable upstream OAuth, clean API key auth flow"`

**Exit Criteria**: No calls to `platform.claw.dev`. API key auth works. Clear error message when no key is set.

---

## PHASE 8: FUNCTIONAL TESTING

**Goal**: Verify all 19 tools, all slash commands, and all core flows work under the WRAITH identity.

### 8A — Build Verification

- [ ] **8A.1** `cargo build --release` — clean build, no warnings
- [ ] **8A.2** `cargo clippy -- -D warnings` — no clippy warnings
- [ ] **8A.3** Binary size check: `ls -la target/release/wraith`
- [ ] **8A.4** Startup time check: `time ./target/release/wraith --help`

### 8B — REPL Smoke Test

- [ ] **8B.1** Start `wraith` — verify banner, prompt, and colors
- [ ] **8B.2** Type a simple question — verify streaming response with styled markdown
- [ ] **8B.3** Test vim mode: `Esc` → Normal, `i` → Insert, verify prompt color change
- [ ] **8B.4** Test multi-line input: `\` at end of line → continuation
- [ ] **8B.5** Test tab completion: start typing a slash command, press Tab

### 8C — Slash Commands

Test each slash command:

- [ ] **8C.1** `/help` — shows all commands with WRAITH branding
- [ ] **8C.2** `/status` — renders glass panel with model/context/session/cost
- [ ] **8C.3** `/cost` — renders cost glass panel
- [ ] **8C.4** `/model` — shows/switches model, renders glass panel
- [ ] **8C.5** `/compact` — renders compact HUD line
- [ ] **8C.6** `/permissions` — shows permission rules
- [ ] **8C.7** `/init` — creates `.wraith/` directory and `WRAITH.md`
- [ ] **8C.8** `/clear` — clears conversation
- [ ] **8C.9** `/quit` or `/exit` — clean exit
- [ ] **8C.10** `/vim` — toggles vim mode
- [ ] **8C.11** All other slash commands — verify they reference "Wraith" not "Claw"

### 8D — Tool Verification

Test key tools with real interactions:

- [ ] **8D.1** File read — ask to read a file
- [ ] **8D.2** File write — ask to create a test file
- [ ] **8D.3** Bash execution — ask to run a command
- [ ] **8D.4** Glob — ask to find files matching a pattern
- [ ] **8D.5** Grep — ask to search for a string
- [ ] **8D.6** Web fetch — ask to fetch a URL
- [ ] **8D.7** Todo management — create/list/complete todos

### 8E — Config Verification

- [ ] **8E.1** Create `.wraith.json` in a test directory, verify it's loaded
- [ ] **8E.2** Create `.wraith/settings.json`, verify settings apply
- [ ] **8E.3** Create `WRAITH.md` in a test directory, verify it's included in context
- [ ] **8E.4** Set `WRAITH_MODEL` env var, verify model changes

### 8F — Session Persistence

- [ ] **8F.1** Start a conversation, exit
- [ ] **8F.2** Start again with `--resume` or session flag, verify conversation continues
- [ ] **8F.3** Verify sessions stored in `.wraith/sessions/`

### 8G — Grep Audit

- [ ] **8G.1** `grep -ri "claw" --include="*.rs" --include="*.toml"` → must return 0 results
- [ ] **8G.2** `grep -ri "claude code" --include="*.rs"` → must return 0 results (allow "claude" as model name)
- [ ] **8G.3** `grep -ri "instructkr" --include="*.rs" --include="*.toml"` → must return 0 results

- [ ] **8G.4** Git commit: `"test: full functional smoke test passed"`

**Exit Criteria**: All features work. Zero naming regressions. Binary is functional end-to-end.

---

## PHASE 9: DOCUMENTATION & POLISH

**Goal**: Final documentation, README screenshots, and polish before public release.

- [ ] **9.1** Take terminal screenshots of:
  - Startup banner
  - Code block rendering
  - Glass panel status report
  - Tool call execution
  - Vim mode (Normal vs Insert)
- [ ] **9.2** Add screenshots to README.md
- [ ] **9.3** Write `docs/` directory:
  - `docs/configuration.md` — full config reference
  - `docs/providers.md` — multi-provider setup guide
  - `docs/plugins.md` — plugin development guide
  - `docs/tools.md` — built-in tools reference
  - `docs/vim-mode.md` — vim keybindings reference
- [ ] **9.4** Final README review:
  - Install instructions verified
  - Feature list accurate
  - No "claw" references
  - Links work
- [ ] **9.5** Update `CHANGELOG.md` with complete v0.1.0 feature list
- [ ] **9.6** Update `Cargo.toml` version to `0.1.0`
- [ ] **9.7** Git commit: `"docs: complete documentation for v0.1.0"`

**Exit Criteria**: README is launch-ready with screenshots. Documentation covers all major features. Version is 0.1.0.

---

## PHASE 10: PUBLISH & LAUNCH

**Goal**: Push to GitHub, publish to crates.io, announce.

- [ ] **10.1** Create GitHub repo: `wraith-code/wraith` (or chosen org/name)
- [ ] **10.2** Push all commits
- [ ] **10.3** Set up GitHub:
  - Description: "The ghost in your terminal — an AI coding agent in Rust"
  - Topics: `ai`, `coding-agent`, `terminal`, `rust`, `cli`, `llm`, `developer-tools`
  - License: MIT
- [ ] **10.4** `cargo publish` — reserve and publish `wraith` crate
- [ ] **10.5** Create GitHub Release v0.1.0 with:
  - Release notes from CHANGELOG
  - Pre-built binaries (Linux x86_64, macOS arm64, macOS x86_64)
- [ ] **10.6** Pre-built binaries:
  - `cargo build --release --target x86_64-unknown-linux-gnu`
  - `cargo build --release --target aarch64-apple-darwin`
  - `cargo build --release --target x86_64-apple-darwin`
- [ ] **10.7** Write launch posts:
  - GitHub Discussions announcement
  - Reddit r/rust, r/programming, r/commandline
  - Hacker News "Show HN"
  - Twitter/X
- [ ] **10.8** Git tag: `v0.1.0`

**Exit Criteria**: WRAITH is publicly available via `cargo install wraith` and GitHub releases. Announcement posted.

---

## PHASE DEPENDENCY GRAPH

```
Phase 0 ─── Phase 1 ─── Phase 2 ─── Phase 3 ─── Phase 4 ─── Phase 5
  (Stage)    (Rename)    (Identity)   (Colors)    (Banner)    (Code Blocks)
                                         │
                                         ├─── Phase 6 ─── Phase 7
                                         │    (Prompt)    (OAuth)
                                         │
                                         └─── Phase 8 ────── Phase 9 ─── Phase 10
                                              (Testing)      (Docs)       (Launch)
```

Phases 3-7 can be parallelized after Phase 2. Phase 8 must come after all functional changes. Phase 9 after testing. Phase 10 after docs.

---

## RISK REGISTER

| Risk                                             | Impact | Mitigation                                                            |
| ------------------------------------------------ | ------ | --------------------------------------------------------------------- |
| `wraith` crate name taken before publish         | HIGH   | Publish placeholder crate ASAP (Phase 0)                              |
| OAuth removal breaks non-API-key auth            | MED    | Keep OAuth code structure, just disable upstream URLs                 |
| Syntect/crossterm color compatibility            | MED    | Test in multiple terminals (kitty, alacritty, iTerm2, gnome-terminal) |
| Hidden "claw" references in binary strings       | MED    | `strings target/release/wraith \| grep -i claw` post-build            |
| Missing dependency after compat-harness removal  | LOW    | `cargo check` at Phase 0 exit catches this                            |
| Glass panel rendering breaks on narrow terminals | MED    | Graceful fallback to plain text at <60 cols                           |

---

## COMMIT CONVENTION

All commits follow Conventional Commits:

```
feat:     New feature
fix:      Bug fix
style:    Visual/formatting changes (no logic change)
refactor: Code restructuring (no behavior change)
docs:     Documentation only
chore:    Build/tooling/maintenance
test:     Test additions or fixes
```

Format: `type: concise description`

Examples:

- `feat: complete claw → wraith rebrand`
- `style: apply Deep Space / Neon Core color palette`
- `docs: add identity files (README, LICENSE, CONTRIBUTING)`
- `fix: disable upstream OAuth, clean API key auth flow`

---

## TRACKING

Progress on this plan is tracked by checking off items in this file. After each phase completes:

1. ✅ Mark all tasks in the phase as done
2. 📝 Note any deviations or decisions made
3. 🔀 Git commit with the specified message
4. ➡️ Move to next phase

When all phases are complete, this file should have zero unchecked boxes.

---

_This is the execution bible. Follow it task by task. No skipping. No shortcuts._
