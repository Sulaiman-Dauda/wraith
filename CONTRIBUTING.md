# Contributing to WRAITH

Thank you for wanting to contribute. WRAITH is MIT-licensed and community-driven.

---

## Development Setup

### Prerequisites

- **Rust** 1.75 or later ([rustup.rs](https://rustup.rs))
- A terminal emulator with true-color support (recommended)

### Build from source

```sh
git clone https://github.com/your-org/wraith
cd wraith/rust
cargo build
```

### Run the dev binary

```sh
cargo run -p wraith-cli -- --help
cargo run -p wraith-cli -- "explain this repo"
```

### Run tests

```sh
cargo test --workspace
```

---

## Architecture Overview

WRAITH is a Rust workspace with the following crates:

| Crate        | Purpose                                                               |
| ------------ | --------------------------------------------------------------------- |
| `wraith-cli` | Binary entry point — REPL, CLI args, startup, rendering               |
| `runtime`    | Core agent loop, config, session, OAuth, sandbox, remote              |
| `api`        | Multi-provider API client (Anthropic, OpenAI-compatible)              |
| `tools`      | 19 built-in tool implementations                                      |
| `commands`   | Slash command implementations (`/agents`, `/skills`, `/status`, etc.) |
| `plugins`    | External plugin loader and hook dispatch                              |
| `server`     | Axum HTTP server mode                                                 |

Key files:

- `wraith-cli/src/main.rs` — Agent event loop, tool dispatch, rendering pipeline
- `runtime/src/config.rs` — Config file loading and merging
- `runtime/src/prompt.rs` — System prompt assembly from `WRAITH.md` files
- `api/src/providers/anthropic.rs` — Anthropic streaming client
- `tools/src/lib.rs` — All tool implementations

---

## Code Style

- **Formatter**: `rustfmt` (run `cargo fmt`)
- **Linter**: `clippy` with `-D warnings` (run `cargo clippy -- -D warnings`)
- **No unsafe**: Avoid `unsafe` blocks unless absolutely necessary with a documented justification
- **Error handling**: Use `anyhow::Result` in binary code, define typed errors in library code
- **No panics in library code**: Use `Result`/`Option` instead of `unwrap()`/`expect()` in crate libraries

Format and lint before submitting:

```sh
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

---

## PR Process

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```sh
   git checkout -b feat/my-feature
   ```
3. **Make your changes** with focused, atomic commits
4. **Add tests** for new functionality
5. **Run the full check**:
   ```sh
   cargo fmt --all && cargo clippy --workspace -- -D warnings && cargo test --workspace
   ```
6. **Open a PR** against `main` with a clear description of what changed and why

### Commit message format

```
type: short description (max 72 chars)

Optional longer body explaining the why, not the what.
```

Types: `feat`, `fix`, `style`, `refactor`, `test`, `docs`, `chore`

---

## Issue Templates

### Bug Report

Include:

- WRAITH version (`wraith --version`)
- OS and terminal emulator
- Steps to reproduce
- Expected vs actual behavior
- Relevant logs or error output

### Feature Request

Include:

- What problem are you trying to solve?
- What is your proposed solution?
- Are there alternative approaches you considered?

---

## Questions

Open an issue or start a discussion. We aim to respond within 48 hours.
