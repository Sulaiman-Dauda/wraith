# Configuration

WRAITH uses a layered configuration system that allows settings to be inherited and overridden at multiple levels. No configuration is required to get started — WRAITH works with zero config.

## Configuration Files

### Project Configuration

**`.wraith.json`** — Project root settings

```json
{
  "model": "claude-3.5-sonnet",
  "maxTokens": 8192,
  "temperature": 0.0,
  "permissionMode": "default",
  "toolAllowlist": ["read_file", "write_file", "bash"],
  "toolDenylist": [],
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": [
        "@modelcontextprotocol/server-filesystem",
        "/path/to/allowed/dir"
      ]
    }
  }
}
```

**`.wraith/settings.json`** — Extended project settings

```json
{
  "providers": {
    "anthropic": {
      "model": "claude-3.5-sonnet",
      "maxTokens": 8192
    },
    "openai": {
      "model": "gpt-4o",
      "baseUrl": "https://api.openai.com/v1"
    }
  },
  "permissions": {
    "mode": "default",
    "toolRules": {
      "bash": "deny",
      "read_file": "allow"
    }
  }
}
```

**`.wraith/settings.local.json`** — Local machine overrides (git-ignored)

```json
{
  "providers": {
    "anthropic": {
      "apiKey": "sk-ant-your-local-key"
    }
  },
  "sandbox": {
    "mode": "read-only"
  }
}
```

### User Configuration

**`~/.wraith/settings.json`** — Global user defaults

```json
{
  "defaultModel": "claude-3.5-sonnet",
  "providers": {
    "anthropic": {
      "apiKey": "sk-ant-your-key-here"
    }
  },
  "ui": {
    "vimMode": true,
    "colorTheme": "deep-space"
  }
}
```

### Instruction Files

**`WRAITH.md`** — Project context instructions

```markdown
# Project Context

This is a Rust web service using Axum for the HTTP layer and SQLx for database access.

## Architecture

- `src/handlers/` - HTTP request handlers
- `src/models/` - Database models and business logic
- `src/db/` - Database connection and migrations

## Coding Standards

- Use `anyhow::Result` for error handling
- All public functions must have documentation
- Run `cargo fmt` and `cargo clippy` before committing
```

**`.wraith/WRAITH.md`** — Alternative instruction location
Same format as `WRAITH.md`, but can be used when you want to keep project instructions in the `.wraith/` directory instead of the project root.

**`WRAITH.local.md`** — Local instruction overrides (git-ignored)

```markdown
# Local Development Notes

## Database Setup

Use the local PostgreSQL instance at localhost:5432 with database `myapp_dev`.

## Environment

Set `DEBUG=1` for verbose logging.
```

## Environment Variables

All WRAITH behavior can be controlled via environment variables, which take precedence over config files:

| Variable                         | Default              | Description                                                                     |
| -------------------------------- | -------------------- | ------------------------------------------------------------------------------- |
| `ANTHROPIC_API_KEY`              | -                    | Anthropic API key for Claude models                                             |
| `OPENAI_API_KEY`                 | -                    | OpenAI-compatible API key                                                       |
| `OPENAI_BASE_URL`                | -                    | Base URL for OpenAI-compatible providers                                        |
| `WRAITH_CONFIG_HOME`             | `~/.wraith/`         | Override config directory                                                       |
| `WRAITH_MODEL`                   | `claude-3.5-sonnet`  | Default model to use                                                            |
| `WRAITH_PERMISSION_MODE`         | `default`            | Permission mode: `default`, `acceptEdits`, `plan`, `bypassPermissions`          |
| `WRAITH_SANDBOX_FILESYSTEM_MODE` | -                    | Filesystem sandbox mode: `read-only`, `disk-write-dangerous`, `containers-only` |
| `WRAITH_REMOTE`                  | `false`              | Enable remote runtime execution                                                 |
| `WRAITH_REMOTE_SESSION_ID`       | -                    | Session ID for remote runtime                                                   |
| `WRAITH_WEB_SEARCH_BASE_URL`     | -                    | Override web search endpoint                                                    |
| `WRAITH_TODO_STORE`              | `.wraith/todos.json` | Override todo store path                                                        |
| `WRAITH_AGENT_STORE`             | `.wraith/agents/`    | Override agent manifest store path                                              |

## Configuration Resolution Order

WRAITH resolves configuration in this order (later entries override earlier ones):

1. **Built-in defaults**
2. **User settings** (`~/.wraith/settings.json`)
3. **Project settings** (`.wraith.json` or `.wraith/settings.json`)
4. **Local settings** (`.wraith/settings.local.json`)
5. **Environment variables** (`WRAITH_*`)

## Schema

### Model Settings

- `model`: Model name or alias (e.g., `"claude-3.5-sonnet"`, `"gpt-4o"`)
- `maxTokens`: Maximum tokens per request (number)
- `temperature`: Temperature for model inference (0.0-1.0)

### Permission Settings

- `permissionMode`: Controls how tools are executed
  - `"default"`: Prompt for dangerous operations
  - `"acceptEdits"`: Auto-approve file edits, prompt for shell commands
  - `"plan"`: Show execution plan without running tools
  - `"bypassPermissions"`: Run all tools without prompting
- `toolAllowlist`: Only these tools are allowed (array of strings)
- `toolDenylist`: These tools are forbidden (array of strings)

### MCP Server Configuration

- `mcpServers`: Map of server name to connection config
  - `command`: Executable command (string)
  - `args`: Command arguments (array of strings)

## Initialization

Use `wraith init` to scaffold configuration files in any project:

```sh
wraith init
```

This creates:

- `WRAITH.md` with a basic instruction template
- `.wraith/settings.json` with common project settings
- `.wraith.json` with minimal project config
