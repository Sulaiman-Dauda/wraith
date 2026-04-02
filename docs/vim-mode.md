# Vim Mode

WRAITH includes a built-in vim editing mode for the input prompt, bringing familiar vim keybindings to AI conversations.

## Enabling Vim Mode

Vim mode is enabled via configuration — there is no slash command toggle.

```sh
# Enable permanently in config
echo '{"ui": {"vimMode": true}}' > ~/.wraith/settings.json
```

Or add it to your project config (`.wraith.json` or `.wraith/settings.json`):

```json
{
  "ui": {
    "vimMode": true
  }
}
```

## Editor Modes

WRAITH implements five editing modes with visual indicators:

### Insert Mode (Default)

- **Indicator**: `INSERT`
- **Prompt Color**: Cyan (`#00E5FF`)
- **Behavior**: Standard text insertion
- **Entry**: Default mode when vim is enabled, or press `i` from Normal mode

### Normal Mode

- **Indicator**: `NORMAL`
- **Prompt Color**: Amber (`#FFCA28`)
- **Behavior**: Command mode for navigation and operations
- **Entry**: Press `Escape` from Insert/Visual mode

### Visual Mode

- **Indicator**: `VISUAL`
- **Prompt Color**: Amber (`#FFCA28`)
- **Behavior**: Text selection mode
- **Entry**: Press `v` from Normal mode

### Command Mode

- **Indicator**: `COMMAND`
- **Prompt Color**: Amber (`#FFCA28`)
- **Behavior**: Execute vim commands (`:`, `/`, etc.)
- **Entry**: Press `:` from Normal mode

### Plain Mode

- **Indicator**: `PLAIN`
- **Behavior**: Disabled vim mode, normal editing
- **Entry**: Set `"vimMode": false` in config to disable vim mode

## Movement Commands (Normal Mode)

### Basic Movement

| Key | Action                         |
| --- | ------------------------------ |
| `h` | Move cursor left               |
| `j` | Move cursor down (next line)   |
| `k` | Move cursor up (previous line) |
| `l` | Move cursor right              |
| `0` | Move to beginning of line      |
| `$` | Move to end of line            |

### Word Movement

| Key | Action                      |
| --- | --------------------------- |
| `w` | Move to next word start     |
| `b` | Move to previous word start |
| `e` | Move to end of current word |

### Line Navigation

| Key  | Action             |
| ---- | ------------------ |
| `G`  | Move to last line  |
| `gg` | Move to first line |

## Editing Commands (Normal Mode)

### Mode Transitions

| Key | Action                                      |
| --- | ------------------------------------------- |
| `i` | Enter Insert mode at cursor                 |
| `a` | Enter Insert mode after cursor              |
| `A` | Enter Insert mode at end of line            |
| `o` | Insert new line below and enter Insert mode |
| `O` | Insert new line above and enter Insert mode |
| `v` | Enter Visual mode                           |

### Text Modification

| Key  | Action                            |
| ---- | --------------------------------- |
| `x`  | Delete character under cursor     |
| `X`  | Delete character before cursor    |
| `dd` | Delete current line               |
| `D`  | Delete from cursor to end of line |
| `C`  | Change from cursor to end of line |
| `cc` | Change entire line                |

### Copy/Paste Operations

| Key  | Action                          |
| ---- | ------------------------------- |
| `yy` | Yank (copy) current line        |
| `Y`  | Yank from cursor to end of line |
| `p`  | Paste after cursor              |
| `P`  | Paste before cursor             |

### Undo/Redo

| Key      | Action           |
| -------- | ---------------- |
| `u`      | Undo last change |
| `Ctrl+r` | Redo             |

## Visual Mode

In Visual mode, you can select text for operations:

| Key           | Action                                |
| ------------- | ------------------------------------- |
| Movement keys | Extend selection                      |
| `y`           | Yank (copy) selection                 |
| `d`           | Delete selection                      |
| `c`           | Change (delete and enter Insert mode) |
| `Escape`      | Return to Normal mode                 |

## Command Mode

Press `:` in Normal mode to enter Command mode for advanced operations:

| Command | Action               |
| ------- | -------------------- |
| `:q`    | Quit (if no changes) |
| `:w`    | Write/save           |
| `:wq`   | Write and quit       |
| `:q!`   | Quit without saving  |

## WRAITH-Specific Extensions

### Command Mode Behavior

When you press `:` in Normal mode to enter Command mode, typing text and pressing Enter submits that text as input — the same as Insert mode. There are no special `:` commands (`:submit`, `:clear`, `:model`, etc. are not implemented). Use slash commands (e.g. `/model`, `/clear`) at the WRAITH prompt instead.

### Multi-line Input

WRAITH vim mode supports multi-line input naturally:

- Use `o` and `O` to create new lines
- Navigate between lines with `j` and `k`
- Edit across multiple lines in Visual mode

## Configuration

Customize vim behavior in your settings:

```json
{
  "ui": {
    "vimMode": true,
    "vimTimeout": 1000,
    "showModeIndicator": true
  }
}
```

### Settings

- **vimMode**: Enable/disable vim mode
- **vimTimeout**: Timeout for multi-key sequences (ms)
- **showModeIndicator**: Show mode indicator in prompt

## Prompt Colors

The prompt chevron (`›`) changes color based on the current mode:

| Mode                  | Color | Hex       |
| --------------------- | ----- | --------- |
| Insert/Plain          | Cyan  | `#00E5FF` |
| Normal/Visual/Command | Amber | `#FFCA28` |

This provides immediate visual feedback about your current editing context.

## Differences from Standard Vim

WRAITH's vim mode is optimized for conversational input:

### Simplified Feature Set

- **No file operations** — Focus on text editing
- **Single buffer** — One input prompt at a time
- **No registers** — Simplified yank/paste operations
- **Limited commands** — Essential editing operations only

### WRAITH Integrations

- **Submit on Enter** — Natural conversation flow from any mode
- **Multi-line support** — Handle complex prompts
- **History integration** — Works with readline history

### Conversation Workflow

```
# Normal vim workflow in WRAITH
# (vim mode must be enabled in config first)
wraith
> i                     # Enter Insert mode
> Type your complex prompt here
> Use vim editing to refine
> [Escape]              # Enter Normal mode
> [Enter]               # Submit to AI
```

## Tips and Tricks

### Efficient Editing

1. **Use `A`** to quickly append to lines
2. **Use `cc`** to replace entire prompts
3. **Use `o`** for multi-paragraph inputs
4. **Use Visual mode** for precise selections

### Conversation Flow

1. **Draft in Insert mode**
2. **Refine in Normal mode**
3. **Review in Visual mode**
4. **Submit with Enter** (from any mode)

### Muscle Memory

- **Escape** always returns to Normal mode
- **Enter** submits from any mode
- **Ctrl+C** cancels current input

## Troubleshooting

### Mode Confusion

- Press `Escape` to return to Normal mode
- Use `:q` to cancel and start over
- Check the mode indicator for current state

### Key Not Working

- Some terminal emulators may intercept keys
- Try alternative key sequences
- Check terminal settings for key mapping conflicts

### Performance

- Vim mode processes each keypress individually
- Large prompts may feel slower than plain mode
- Consider plain mode for very long inputs

The vim mode integration makes WRAITH feel like a natural extension of your development environment, bringing the power of vim editing to AI-assisted coding workflows.
