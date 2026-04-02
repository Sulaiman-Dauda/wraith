# Built-in Tools

WRAITH includes 19 built-in tools that cover common development tasks. These tools are available in every session and can be used directly in conversation.

## File Operations

### read_file

Read a text file from the workspace.

**Parameters:**

- `path` (required) — File path to read
- `offset` (optional) — Byte offset to start reading from
- `limit` (optional) — Maximum bytes to read

**Example:**

```
Read the main.rs file
```

### write_file

Write a text file in the workspace.

**Parameters:**

- `path` (required) — File path to write
- `content` (required) — File content

**Example:**

```
Create a new README.md with installation instructions
```

### edit_file

Replace text in a workspace file.

**Parameters:**

- `path` (required) — File path to edit
- `old_string` (required) — Text to replace
- `new_string` (required) — Replacement text
- `replace_all` (optional) — Replace all occurrences (default: false)

**Example:**

```
In src/main.rs, change the function name from old_function to new_function
```

## Search & Discovery

### glob_search

Find files by glob pattern.

**Parameters:**

- `pattern` (required) — Glob pattern (e.g., `**/*.rs`, `src/**/*.py`)
- `path` (optional) — Directory to search within

**Example:**

```
Find all TypeScript files in the project
```

### grep_search

Search file contents with regex patterns.

**Parameters:**

- `pattern` (required) — Regex pattern to search for
- `path` (optional) — File or directory to search
- `glob` (optional) — Limit search to files matching glob
- `output_mode` (optional) — Output format
- `-B`, `-A`, `-C` (optional) — Context lines before/after/around matches
- `context` (optional) — Context lines around matches
- `-n` (optional) — Show line numbers
- `-i` (optional) — Case-insensitive search
- `type` (optional) — File type filter
- `head_limit` (optional) — Maximum results
- `offset` (optional) — Skip results
- `multiline` (optional) — Enable multiline matching

**Example:**

```
Search for all function definitions in the codebase
```

## Command Execution

### bash

Execute a shell command in the current workspace.

**Parameters:**

- `command` (required) — Shell command to execute
- `timeout` (optional) — Timeout in seconds
- `description` (optional) — Human-readable description
- `run_in_background` (optional) — Run command in background
- `dangerouslyDisableSandbox` (optional) — Disable safety restrictions

**Example:**

```
Run the test suite
```

### PowerShell

Execute a PowerShell command with optional timeout.

**Parameters:**

- `command` (required) — PowerShell command to execute
- `timeout` (optional) — Timeout in seconds
- `description` (optional) — Human-readable description
- `run_in_background` (optional) — Run command in background

**Example:**

```
Get directory listing using PowerShell
```

### REPL

Execute code in a REPL-like subprocess.

**Parameters:**

- `code` (required) — Code to execute
- `language` (required) — Programming language
- `timeout_ms` (optional) — Timeout in milliseconds

**Example:**

```
Run this Python code in a REPL: print("Hello world")
```

## Web & Network

### WebFetch

Fetch a URL, convert it to readable text, and answer questions about it.

**Parameters:**

- `url` (required) — URL to fetch (must be valid URI)
- `prompt` (required) — Question to answer about the content

**Example:**

```
Fetch the latest release notes from https://github.com/rust-lang/rust/releases and summarize the key changes
```

### WebSearch

Search the web for current information and return cited results.

**Parameters:**

- `query` (required) — Search query (minimum 2 characters)
- `allowed_domains` (optional) — Array of domains to limit search to
- `blocked_domains` (optional) — Array of domains to exclude

**Example:**

```
Search for the latest best practices for Rust async programming
```

## Productivity & Organization

### TodoWrite

Update the structured task list for the current session.

**Parameters:**

- `todos` (required) — Array of todo items with:
  - `content` — Task description
  - `activeForm` — Current state of the task
  - `status` — `"pending"`, `"in_progress"`, or `"completed"`

**Example:**

```
Add these tasks to my todo list: implement user authentication, write tests, update documentation
```

### Config

Get or set WRAITH settings.

**Parameters:**

- `setting` (required) — Setting name to get/set
- `value` (optional) — Value to set (omit to get current value)

**Example:**

```
Set the default model to claude-3-opus
```

### Sleep

Wait for a specified duration without holding a shell process.

**Parameters:**

- `duration_ms` (required) — Duration in milliseconds

**Example:**

```
Wait 5 seconds then continue
```

## Communication & Messaging

### SendUserMessage

Send a message to the user.

**Parameters:**

- `message` (required) — Message content
- `attachments` (optional) — Array of file paths
- `status` (required) — `"normal"` or `"proactive"`

**Example:**

```
Send a notification that the background task is complete
```

### StructuredOutput

Return structured output in the requested format.

**Parameters:**

- Accepts any JSON object structure

**Example:**

```
Format the results as a JSON array with name and version fields
```

## Development Tools

### NotebookEdit

Replace, insert, or delete a cell in a Jupyter notebook.

**Parameters:**

- `notebook_path` (required) — Path to notebook file
- `cell_id` (optional) — ID of cell to modify
- `new_source` (optional) — New cell content
- `cell_type` (optional) — `"code"` or `"markdown"`
- `edit_mode` (optional) — `"replace"`, `"insert"`, or `"delete"`

**Example:**

```
Add a new markdown cell to the notebook explaining the data analysis
```

## Advanced Tools

### Skill

Load a local skill definition and its instructions.

**Parameters:**

- `skill` (required) — Skill name to load
- `args` (optional) — Arguments to pass to the skill

**Example:**

```
Load the rust-development skill for this project
```

### Agent

Launch a specialized agent task and persist its handoff metadata.

**Parameters:**

- `description` (required) — Task description
- `prompt` (required) — Detailed prompt for the agent
- `subagent_type` (optional) — Type of specialized agent
- `name` (optional) — Agent instance name
- `model` (optional) — Model to use for the agent

**Example:**

```
Spawn a code review agent to analyze the security of this authentication system
```

### ToolSearch

Search for deferred or specialized tools by name or keywords.

**Parameters:**

- `query` (required) — Search query
- `max_results` (optional) — Maximum results to return

**Example:**

```
Find tools related to database operations
```

## Tool Permissions

Each tool requires certain permission levels:

| Permission Level     | Description                 | Tools                                                                                                                                     |
| -------------------- | --------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| **ReadOnly**         | Can read files and data     | `read_file`, `glob_search`, `grep_search`, `WebFetch`, `WebSearch`, `Skill`, `ToolSearch`, `Sleep`, `SendUserMessage`, `StructuredOutput` |
| **WorkspaceWrite**   | Can modify workspace files  | `write_file`, `edit_file`, `TodoWrite`, `Config`, `NotebookEdit`                                                                          |
| **DangerFullAccess** | Can execute system commands | `bash`, `PowerShell`, `REPL`, `Agent`                                                                                                     |

## Permission Modes

Control tool execution behavior with permission modes:

- **`read-only`**: Wraith can only read files and run safe commands
- **`workspace-write`**: Wraith can read and write files within the workspace
- **`danger-full-access`**: Wraith has full system access (use with extreme caution)

Set via:

```sh
export WRAITH_PERMISSION_MODE=workspace-write
wraith --permission-mode read-only
```

## Tool Allowlist/Denylist

Restrict available tools using configuration:

```json
{
  "toolAllowlist": ["read_file", "write_file", "bash"],
  "toolDenylist": ["WebFetch", "WebSearch"]
}
```

## Usage Tips

1. **Combine tools**: "Read the config file, then update the database connection string"
2. **Chain operations**: "Search for TODO comments, then create a task list"
3. **Iterate**: "Run tests, fix any failures, then run again"
4. **Explore**: "Find all the API endpoints and document their purpose"

## Error Handling

Tools provide structured error responses:

- **File not found**: Clear path resolution
- **Permission denied**: Explanation of required permissions
- **Command failed**: Exit code and error output
- **Network error**: Details about connection issues

All errors include actionable next steps when possible.
