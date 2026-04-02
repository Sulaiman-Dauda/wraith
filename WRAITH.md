# WRAITH Project Instructions

This file provides context to WRAITH about your project.
Place it in your project root or in `.wraith/WRAITH.md`.

WRAITH reads this file automatically when you start a session in this directory.

---

## Project Overview

[Describe your project here — what it does, what problem it solves, who uses it.]

---

## Tech Stack

[Languages, frameworks, libraries, and tools your project uses.]

---

## Conventions

[List your coding conventions, naming patterns, and style rules.]

Examples:

- Use `snake_case` for variables and functions, `PascalCase` for types
- All public functions must have doc comments
- New features require tests in the same file
- Run `cargo fmt` and `cargo clippy -- -D warnings` before committing

---

## Repository Structure

[Describe the key directories and what they contain.]

Examples:

```
src/           Application source code
tests/         Integration tests
docs/          Documentation
scripts/       Build and deployment scripts
```

---

## Build & Test

[How to build, run, and test the project.]

Examples:

```sh
cargo build          # debug build
cargo test           # run all tests
cargo run -- --help  # run with args
```

---

## Important Context

[Anything WRAITH should know when working in this codebase.]

Examples:

- This codebase targets Rust edition 2021
- Do not modify generated files under `gen/`
- The `legacy/` directory is read-only — do not refactor it
- Ask before running any command that touches the database

---

## Preferences

[Your preferences for how WRAITH should work in this project.]

Examples:

- Prefer small, focused commits over large changesets
- Always explain what you're about to change before making edits
- Run tests after every functional change
- Keep existing code style — don't reformat code you're not changing
