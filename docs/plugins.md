# Plugin System

WRAITH supports a powerful plugin system that allows extending functionality through external executables and hook-based lifecycle events.

## Plugin Structure

A plugin is a directory containing:

- **`.wraith-plugin/plugin.json`** — Plugin manifest
- **Executable files** — Tools and hook handlers

### Plugin Manifest

**`.wraith-plugin/plugin.json`**

```json
{
  "name": "my-plugin",
  "version": "0.1.0",
  "description": "A sample WRAITH plugin",
  "permissions": ["read", "write", "execute"],
  "defaultEnabled": true,
  "tools": [
    {
      "name": "my_tool",
      "description": "Does something useful",
      "input_schema": {
        "type": "object",
        "properties": {
          "input": {
            "type": "string",
            "description": "Input parameter"
          }
        },
        "required": ["input"],
        "additionalProperties": false
      }
    }
  ],
  "hooks": {
    "PreToolUse": ["./hooks/pre-tool"],
    "PostToolUse": ["./hooks/post-tool"]
  },
  "lifecycle": {
    "Init": ["./lifecycle/init"],
    "Shutdown": ["./lifecycle/shutdown"]
  },
  "commands": [
    {
      "name": "my-command",
      "description": "Custom slash command",
      "executable": "./commands/my-command"
    }
  ]
}
```

## Hook Types

Plugins can register handlers for lifecycle events that fire during WRAITH execution.

### PreToolUse

Fires **before** any tool is executed. Can deny tool execution.

**Environment Variables:**

- `HOOK_EVENT` — Always `"PreToolUse"`
- `HOOK_TOOL_NAME` — Name of the tool about to execute
- `HOOK_TOOL_INPUT` — JSON string of tool input parameters

**Exit Codes:**

- `0` — Allow tool execution
- `2` — Deny tool execution
- any other non-zero — Warn (tool executes but a warning is shown)

**Example Handler:**

```bash
#!/bin/bash
# ./hooks/pre-tool

if [[ "$HOOK_TOOL_NAME" == "bash" ]]; then
    echo "Blocking dangerous bash command"
    exit 2
fi

echo "Allowing $HOOK_TOOL_NAME"
exit 0
```

### PostToolUse

Fires **after** any tool execution, regardless of success or failure.

**Environment Variables:**

- `HOOK_EVENT` — Always `"PostToolUse"`
- `HOOK_TOOL_NAME` — Name of the tool that executed
- `HOOK_TOOL_INPUT` — JSON string of tool input parameters
- `HOOK_TOOL_OUTPUT` — Tool output (if successful)
- `HOOK_TOOL_IS_ERROR` — `"1"` if tool failed, `"0"` if successful

**Example Handler:**

```bash
#!/bin/bash
# ./hooks/post-tool

if [[ "$HOOK_TOOL_IS_ERROR" == "1" ]]; then
    echo "Tool $HOOK_TOOL_NAME failed!"
    # Log error, send notification, etc.
fi

echo "Tool $HOOK_TOOL_NAME completed"
```

## Plugin Environment Variables

When any plugin executable runs, WRAITH sets these environment variables:

| Variable             | Description                          |
| -------------------- | ------------------------------------ |
| `WRAITH_PLUGIN_ID`   | Unique plugin identifier             |
| `WRAITH_PLUGIN_NAME` | Human-readable plugin name           |
| `WRAITH_PLUGIN_ROOT` | Absolute path to plugin directory    |
| `WRAITH_TOOL_NAME`   | Name of the currently executing tool |
| `WRAITH_TOOL_INPUT`  | JSON string of tool input parameters |

## Tool Plugins

Plugins can define custom tools that integrate seamlessly with WRAITH's built-in tool set.

### Tool Implementation

**`./tools/my-tool`**

```bash
#!/bin/bash

# Read tool input from environment
INPUT=$(echo "$WRAITH_TOOL_INPUT" | jq -r .input)

# Perform tool logic
result="Processed: $INPUT"

# Return JSON result
echo "{\"result\": \"$result\", \"status\": \"success\"}"
```

### Tool Registration

The plugin is automatically registered when WRAITH starts. Users can then invoke your tool:

```
> Use my_tool to process "hello world"

Tool my_tool executed successfully:
{
  "result": "Processed: hello world",
  "status": "success"
}
```

## Bundled Plugins

WRAITH includes sample plugins in `rust/crates/plugins/bundled/`:

### example-bundled

A reference implementation showing:

- Basic plugin structure
- Tool definition and implementation
- Hook handlers
- Proper error handling

Located at: `rust/crates/plugins/bundled/example-bundled/`

### sample-hooks

Demonstrates hook-based functionality:

- Pre-tool validation
- Post-tool logging
- Permission checking

## Plugin Permissions

Plugins declare required permissions in their manifest:

```json
{
  "permissions": ["read", "write", "execute"]
}
```

### Permission Types

| Permission | Description             |
| ---------- | ----------------------- |
| `read`     | Read files in workspace |
| `write`    | Write/modify files      |
| `execute`  | Run system commands     |

### Permission Enforcement

WRAITH enforces permissions at the system level:

- **read**: Plugin can access workspace files
- **write**: Plugin can modify workspace files
- **execute**: Plugin can run subprocess commands

## Plugin Discovery

WRAITH searches for plugins in these locations:

1. **Project plugins**: `.wraith/plugins/`
2. **User plugins**: `~/.wraith/plugins/`
3. **Bundled plugins**: Built into WRAITH binary

## Plugin Lifecycle

1. **Discovery** — WRAITH scans plugin directories
2. **Validation** — Plugin manifests are parsed and validated
3. **Registration** — Tools and hooks are registered
4. **Init** — `lifecycle.Init` commands are executed
5. **Runtime** — Hooks fire during normal operation
6. **Shutdown** — `lifecycle.Shutdown` commands are executed

## Development Tips

### Testing Plugins

```bash
# Test plugin manifest
wraith --validate-plugin ./my-plugin

# Debug hook execution
WRAITH_DEBUG=1 wraith
```

### Plugin Templates

Use the bundled plugins as starting points:

```bash
cp -r rust/crates/plugins/bundled/example-bundled ./my-plugin
cd ./my-plugin
# Modify .wraith-plugin/plugin.json and tools/
```

### Error Handling

Always handle errors gracefully in plugin executables:

```bash
#!/bin/bash
set -e  # Exit on any error

# Validate input
if [[ -z "$WRAITH_TOOL_INPUT" ]]; then
    echo '{"error": "No input provided"}' >&2
    exit 1
fi

# Your plugin logic here
```

## Security Considerations

- **Validate inputs** — Plugin tools receive user input
- **Sandbox execution** — Consider containerizing dangerous operations
- **Permission model** — Request minimal required permissions
- **Audit dependencies** — External executables run with plugin permissions

## Plugin Examples

Check the bundled plugins for working examples:

- [example-bundled](../rust/crates/plugins/bundled/example-bundled/) — Basic plugin structure
- [sample-hooks](../rust/crates/plugins/bundled/sample-hooks/) — Hook-based functionality
