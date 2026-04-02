use std::fmt::Write as FmtWrite;
use std::io::{self, Write};

use crossterm::cursor::{MoveToColumn, RestorePosition, SavePosition};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{execute, queue};
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// ── Cyber Neon palette ────────────────────────────────────────
// Background tones
const BORDER_DIM: Color = Color::Rgb { r: 30, g: 42, b: 56 };     // #1E2A38
const TEXT_DIM: Color = Color::Rgb { r: 58, g: 90, b: 120 };       // #3A5A78
// Accent colours
pub const CYAN: Color = Color::Rgb { r: 0, g: 229, b: 204 };      // #00E5CC
pub const VIOLET: Color = Color::Rgb { r: 153, g: 102, b: 255 };   // #9966FF
pub const AMBER: Color = Color::Rgb { r: 240, g: 160, b: 48 };     // #F0A030
pub const GREEN: Color = Color::Rgb { r: 34, g: 212, b: 122 };     // #22D47A
pub const RED: Color = Color::Rgb { r: 192, g: 64, b: 64 };        // #C04040

// ── Raw ANSI escape constants (for pre-formatted banner / panel output) ──
const ANSI_BORDER: &str = "\x1b[38;2;30;42;56m";     // #1E2A38
const ANSI_DIM: &str    = "\x1b[38;2;58;90;120m";     // #3A5A78
const ANSI_CYAN: &str   = "\x1b[38;2;0;229;204m";     // #00E5CC
const ANSI_VIOLET: &str = "\x1b[38;2;153;102;255m";   // #9966FF
const ANSI_GREEN: &str  = "\x1b[38;2;34;212;122m";    // #22D47A
const ANSI_AMBER: &str  = "\x1b[38;2;240;160;48m";    // #F0A030
const ANSI_NEON: &str   = "\x1b[38;2;0;180;160m";     // #00B4A0  (input box accent)
const ANSI_RST: &str    = "\x1b[0m";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorTheme {
    heading: Color,
    emphasis: Color,
    strong: Color,
    inline_code: Color,
    link: Color,
    quote: Color,
    table_border: Color,
    code_block_border: Color,
    spinner_active: Color,
    spinner_done: Color,
    spinner_failed: Color,
    tool_use_border: Color,
}

impl Default for ColorTheme {
    fn default() -> Self {
        Self {
            heading: CYAN,
            emphasis: VIOLET,
            strong: AMBER,
            inline_code: CYAN,
            link: VIOLET,
            quote: TEXT_DIM,
            table_border: BORDER_DIM,
            code_block_border: BORDER_DIM,
            spinner_active: CYAN,
            spinner_done: GREEN,
            spinner_failed: RED,
            tool_use_border: BORDER_DIM,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Spinner {
    frame_index: usize,
}

impl Spinner {
    const FRAMES: [&str; 10] = [
        "⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏",
    ];

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        let frame = Self::FRAMES[self.frame_index % Self::FRAMES.len()];
        self.frame_index += 1;
        queue!(
            out,
            SavePosition,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_active),
            Print(format!("{frame} {label}")),
            ResetColor,
            RestorePosition
        )?;
        out.flush()
    }

    #[allow(dead_code)]
    pub fn finish(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        self.frame_index = 0;
        // Clear the spinner line — the caller prints any follow-up.
        execute!(
            out,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_done),
            Print(format!("✔ {label}\n")),
            ResetColor
        )?;
        out.flush()
    }

    /// Clear the spinner line without printing a replacement label.
    pub fn clear(&mut self, out: &mut impl Write) -> io::Result<()> {
        self.frame_index = 0;
        execute!(out, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        out.flush()
    }

    pub fn fail(
        &mut self,
        label: &str,
        theme: &ColorTheme,
        out: &mut impl Write,
    ) -> io::Result<()> {
        self.frame_index = 0;
        execute!(
            out,
            MoveToColumn(0),
            Clear(ClearType::CurrentLine),
            SetForegroundColor(theme.spinner_failed),
            Print(format!("✘ {label}\n")),
            ResetColor
        )?;
        out.flush()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListKind {
    Unordered,
    Ordered { next_index: u64 },
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct TableState {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    current_row: Vec<String>,
    current_cell: String,
    in_head: bool,
}

impl TableState {
    fn push_cell(&mut self) {
        let cell = self.current_cell.trim().to_string();
        self.current_row.push(cell);
        self.current_cell.clear();
    }

    fn finish_row(&mut self) {
        if self.current_row.is_empty() {
            return;
        }
        let row = std::mem::take(&mut self.current_row);
        if self.in_head {
            self.headers = row;
        } else {
            self.rows.push(row);
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct RenderState {
    emphasis: usize,
    strong: usize,
    heading_level: Option<u8>,
    quote: usize,
    list_stack: Vec<ListKind>,
    link_stack: Vec<LinkState>,
    table: Option<TableState>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinkState {
    destination: String,
    text: String,
}

impl RenderState {
    fn style_text(&self, text: &str, theme: &ColorTheme) -> String {
        let mut style = text.stylize();

        if matches!(self.heading_level, Some(1 | 2)) || self.strong > 0 {
            style = style.bold();
        }
        if self.emphasis > 0 {
            style = style.italic();
        }

        if let Some(level) = self.heading_level {
            style = match level {
                1 => style.with(theme.heading),
                2 => style.white(),
                3 => style.with(Color::Blue),
                _ => style.with(Color::Grey),
            };
        } else if self.strong > 0 {
            style = style.with(theme.strong);
        } else if self.emphasis > 0 {
            style = style.with(theme.emphasis);
        }

        if self.quote > 0 {
            style = style.with(theme.quote);
        }

        format!("{style}")
    }

    fn append_raw(&mut self, output: &mut String, text: &str) {
        if let Some(link) = self.link_stack.last_mut() {
            link.text.push_str(text);
        } else if let Some(table) = self.table.as_mut() {
            table.current_cell.push_str(text);
        } else {
            output.push_str(text);
        }
    }

    fn append_styled(&mut self, output: &mut String, text: &str, theme: &ColorTheme) {
        let styled = self.style_text(text, theme);
        self.append_raw(output, &styled);
    }
}

#[derive(Debug)]
pub struct TerminalRenderer {
    syntax_set: SyntaxSet,
    syntax_theme: Theme,
    color_theme: ColorTheme,
}

impl Default for TerminalRenderer {
    fn default() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let syntax_theme = ThemeSet::load_defaults()
            .themes
            .remove("base16-ocean.dark")
            .unwrap_or_default();
        Self {
            syntax_set,
            syntax_theme,
            color_theme: ColorTheme::default(),
        }
    }
}

impl TerminalRenderer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn color_theme(&self) -> &ColorTheme {
        &self.color_theme
    }

    #[must_use]
    pub fn render_markdown(&self, markdown: &str) -> String {
        let mut output = String::new();
        let mut state = RenderState::default();
        let mut code_language = String::new();
        let mut code_buffer = String::new();
        let mut in_code_block = false;

        for event in Parser::new_ext(markdown, Options::all()) {
            self.render_event(
                event,
                &mut state,
                &mut output,
                &mut code_buffer,
                &mut code_language,
                &mut in_code_block,
            );
        }

        output.trim_end().to_string()
    }

    #[must_use]
    pub fn markdown_to_ansi(&self, markdown: &str) -> String {
        self.render_markdown(markdown)
    }

    #[allow(clippy::too_many_lines)]
    fn render_event(
        &self,
        event: Event<'_>,
        state: &mut RenderState,
        output: &mut String,
        code_buffer: &mut String,
        code_language: &mut String,
        in_code_block: &mut bool,
    ) {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                self.start_heading(state, level as u8, output);
            }
            Event::End(TagEnd::Paragraph) => output.push_str("\n\n"),
            Event::Start(Tag::BlockQuote(..)) => self.start_quote(state, output),
            Event::End(TagEnd::BlockQuote(..)) => {
                state.quote = state.quote.saturating_sub(1);
                output.push('\n');
            }
            Event::End(TagEnd::Heading(..)) => {
                state.heading_level = None;
                output.push_str("\n\n");
            }
            Event::End(TagEnd::Item) | Event::SoftBreak | Event::HardBreak => {
                state.append_raw(output, "\n");
            }
            Event::Start(Tag::List(first_item)) => {
                let kind = match first_item {
                    Some(index) => ListKind::Ordered { next_index: index },
                    None => ListKind::Unordered,
                };
                state.list_stack.push(kind);
            }
            Event::End(TagEnd::List(..)) => {
                state.list_stack.pop();
                output.push('\n');
            }
            Event::Start(Tag::Item) => Self::start_item(state, output),
            Event::Start(Tag::CodeBlock(kind)) => {
                *in_code_block = true;
                *code_language = match kind {
                    CodeBlockKind::Indented => String::from("text"),
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                };
                code_buffer.clear();
                self.start_code_block(code_language, output);
            }
            Event::End(TagEnd::CodeBlock) => {
                self.finish_code_block(code_buffer, code_language, output);
                *in_code_block = false;
                code_language.clear();
                code_buffer.clear();
            }
            Event::Start(Tag::Emphasis) => state.emphasis += 1,
            Event::End(TagEnd::Emphasis) => state.emphasis = state.emphasis.saturating_sub(1),
            Event::Start(Tag::Strong) => state.strong += 1,
            Event::End(TagEnd::Strong) => state.strong = state.strong.saturating_sub(1),
            Event::Code(code) => {
                let rendered =
                    format!("{}", format!("`{code}`").with(self.color_theme.inline_code));
                state.append_raw(output, &rendered);
            }
            Event::Rule => output.push_str("---\n"),
            Event::Text(text) => {
                self.push_text(text.as_ref(), state, output, code_buffer, *in_code_block);
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                state.append_raw(output, &html);
            }
            Event::FootnoteReference(reference) => {
                state.append_raw(output, &format!("[{reference}]"));
            }
            Event::TaskListMarker(done) => {
                state.append_raw(output, if done { "[x] " } else { "[ ] " });
            }
            Event::InlineMath(math) | Event::DisplayMath(math) => {
                state.append_raw(output, &math);
            }
            Event::Start(Tag::Link { dest_url, .. }) => {
                state.link_stack.push(LinkState {
                    destination: dest_url.to_string(),
                    text: String::new(),
                });
            }
            Event::End(TagEnd::Link) => {
                if let Some(link) = state.link_stack.pop() {
                    let label = if link.text.is_empty() {
                        link.destination.clone()
                    } else {
                        link.text
                    };
                    let rendered = format!(
                        "{}",
                        format!("[{label}]({})", link.destination)
                            .underlined()
                            .with(self.color_theme.link)
                    );
                    state.append_raw(output, &rendered);
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                let rendered = format!(
                    "{}",
                    format!("[image:{dest_url}]").with(self.color_theme.link)
                );
                state.append_raw(output, &rendered);
            }
            Event::Start(Tag::Table(..)) => state.table = Some(TableState::default()),
            Event::End(TagEnd::Table) => {
                if let Some(table) = state.table.take() {
                    output.push_str(&self.render_table(&table));
                    output.push_str("\n\n");
                }
            }
            Event::Start(Tag::TableHead) => {
                if let Some(table) = state.table.as_mut() {
                    table.in_head = true;
                }
            }
            Event::End(TagEnd::TableHead) => {
                if let Some(table) = state.table.as_mut() {
                    table.finish_row();
                    table.in_head = false;
                }
            }
            Event::Start(Tag::TableRow) => {
                if let Some(table) = state.table.as_mut() {
                    table.current_row.clear();
                    table.current_cell.clear();
                }
            }
            Event::End(TagEnd::TableRow) => {
                if let Some(table) = state.table.as_mut() {
                    table.finish_row();
                }
            }
            Event::Start(Tag::TableCell) => {
                if let Some(table) = state.table.as_mut() {
                    table.current_cell.clear();
                }
            }
            Event::End(TagEnd::TableCell) => {
                if let Some(table) = state.table.as_mut() {
                    table.push_cell();
                }
            }
            Event::Start(Tag::Paragraph | Tag::MetadataBlock(..) | _)
            | Event::End(TagEnd::Image | TagEnd::MetadataBlock(..) | _) => {}
        }
    }

    #[allow(clippy::unused_self)]
    fn start_heading(&self, state: &mut RenderState, level: u8, output: &mut String) {
        state.heading_level = Some(level);
        if !output.is_empty() {
            output.push('\n');
        }
    }

    fn start_quote(&self, state: &mut RenderState, output: &mut String) {
        state.quote += 1;
        let _ = write!(output, "{}", "│ ".with(self.color_theme.quote));
    }

    fn start_item(state: &mut RenderState, output: &mut String) {
        let depth = state.list_stack.len().saturating_sub(1);
        output.push_str(&"  ".repeat(depth));

        let marker = match state.list_stack.last_mut() {
            Some(ListKind::Ordered { next_index }) => {
                let value = *next_index;
                *next_index += 1;
                format!("{value}. ")
            }
            _ => "• ".to_string(),
        };
        output.push_str(&marker);
    }

    fn start_code_block(&self, code_language: &str, output: &mut String) {
        let language_label = if code_language.is_empty() {
            "code"
        } else {
            code_language
        };
        let cols = terminal_width();
        let header_text = format!(" {} ", language_label);
        let remaining = cols.saturating_sub(header_text.len() + 4); // ┌─ + ─
        let left_dash = "─";
        let right_fill = "─".repeat(remaining);
        let _ = writeln!(
            output,
            "  \x1b[38;2;30;42;56m┌{left_dash}\x1b[38;2;0;229;204m{language_label} \x1b[38;2;30;42;56m{right_fill}\x1b[0m"
        );
    }

    fn finish_code_block(&self, code_buffer: &str, code_language: &str, output: &mut String) {
        output.push_str(&self.highlight_code(code_buffer, code_language));
        let cols = terminal_width();
        let fill = "─".repeat(cols.saturating_sub(4)); // ─ + └ + 2 spaces
        let _ = write!(
            output,
            "  \x1b[38;2;30;42;56m└{fill}\x1b[0m"
        );
        output.push_str("\n\n");
    }

    fn push_text(
        &self,
        text: &str,
        state: &mut RenderState,
        output: &mut String,
        code_buffer: &mut String,
        in_code_block: bool,
    ) {
        if in_code_block {
            code_buffer.push_str(text);
        } else {
            state.append_styled(output, text, &self.color_theme);
        }
    }

    fn render_table(&self, table: &TableState) -> String {
        let mut rows = Vec::new();
        if !table.headers.is_empty() {
            rows.push(table.headers.clone());
        }
        rows.extend(table.rows.iter().cloned());

        if rows.is_empty() {
            return String::new();
        }

        let column_count = rows.iter().map(Vec::len).max().unwrap_or(0);
        let widths = (0..column_count)
            .map(|column| {
                rows.iter()
                    .filter_map(|row| row.get(column))
                    .map(|cell| visible_width(cell))
                    .max()
                    .unwrap_or(0)
            })
            .collect::<Vec<_>>();

        let border = format!("{}", "│".with(self.color_theme.table_border));
        let separator = widths
            .iter()
            .map(|width| "─".repeat(*width + 2))
            .collect::<Vec<_>>()
            .join(&format!("{}", "┼".with(self.color_theme.table_border)));
        let separator = format!("{border}{separator}{border}");

        let mut output = String::new();
        if !table.headers.is_empty() {
            output.push_str(&self.render_table_row(&table.headers, &widths, true));
            output.push('\n');
            output.push_str(&separator);
            if !table.rows.is_empty() {
                output.push('\n');
            }
        }

        for (index, row) in table.rows.iter().enumerate() {
            output.push_str(&self.render_table_row(row, &widths, false));
            if index + 1 < table.rows.len() {
                output.push('\n');
            }
        }

        output
    }

    fn render_table_row(&self, row: &[String], widths: &[usize], is_header: bool) -> String {
        let border = format!("{}", "│".with(self.color_theme.table_border));
        let mut line = String::new();
        line.push_str(&border);

        for (index, width) in widths.iter().enumerate() {
            let cell = row.get(index).map_or("", String::as_str);
            line.push(' ');
            if is_header {
                let _ = write!(line, "{}", cell.bold().with(self.color_theme.heading));
            } else {
                line.push_str(cell);
            }
            let padding = width.saturating_sub(visible_width(cell));
            line.push_str(&" ".repeat(padding + 1));
            line.push_str(&border);
        }

        line
    }

    #[must_use]
    pub fn highlight_code(&self, code: &str, language: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());
        let mut syntax_highlighter = HighlightLines::new(syntax, &self.syntax_theme);
        let mut colored_output = String::new();

        for line in LinesWithEndings::from(code) {
            match syntax_highlighter.highlight_line(line, &self.syntax_set) {
                Ok(ranges) => {
                    let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                    colored_output.push_str(&apply_code_block_background(&escaped));
                }
                Err(_) => colored_output.push_str(&apply_code_block_background(line)),
            }
        }

        colored_output
    }

    pub fn stream_markdown(&self, markdown: &str, out: &mut impl Write) -> io::Result<()> {
        let rendered_markdown = self.markdown_to_ansi(markdown);
        write!(out, "{rendered_markdown}")?;
        if !rendered_markdown.ends_with('\n') {
            writeln!(out)?;
        }
        out.flush()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct MarkdownStreamState {
    pending: String,
}

impl MarkdownStreamState {
    #[must_use]
    pub fn push(&mut self, renderer: &TerminalRenderer, delta: &str) -> Option<String> {
        self.pending.push_str(delta);
        let split = find_stream_safe_boundary(&self.pending)?;
        let ready = self.pending[..split].to_string();
        self.pending.drain(..split);
        Some(apply_response_gutter(&renderer.markdown_to_ansi(&ready)))
    }

    #[must_use]
    pub fn flush(&mut self, renderer: &TerminalRenderer) -> Option<String> {
        if self.pending.trim().is_empty() {
            self.pending.clear();
            None
        } else {
            let pending = std::mem::take(&mut self.pending);
            Some(apply_response_gutter(&renderer.markdown_to_ansi(&pending)))
        }
    }
}

/// Apply dim violet left-border gutter to AI response text
fn apply_response_gutter(text: &str) -> String {
    text.split('\n')
        .map(|line| format!("\x1b[38;2;80;60;140m│\x1b[0m {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn apply_code_block_background(line: &str) -> String {
    let trimmed = line.trim_end_matches('\n');
    let trailing_newline = if trimmed.len() == line.len() {
        ""
    } else {
        "\n"
    };
    // Dark slate background: Rgb(18, 24, 33)
    let with_background = trimmed.replace("\u{1b}[0m", "\u{1b}[0;48;2;18;24;33m");
    format!("\u{1b}[48;2;18;24;33m{with_background}\u{1b}[0m{trailing_newline}")
}

fn find_stream_safe_boundary(markdown: &str) -> Option<usize> {
    let mut in_fence = false;
    let mut last_boundary = None;

    for (offset, line) in markdown.split_inclusive('\n').scan(0usize, |cursor, line| {
        let start = *cursor;
        *cursor += line.len();
        Some((start, line))
    }) {
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            in_fence = !in_fence;
            if !in_fence {
                last_boundary = Some(offset + line.len());
            }
            continue;
        }

        if in_fence {
            continue;
        }

        if trimmed.is_empty() {
            last_boundary = Some(offset + line.len());
        }
    }

    last_boundary
}

fn visible_width(input: &str) -> usize {
    strip_ansi(input).chars().count()
}

pub(crate) fn strip_ansi(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if chars.peek() == Some(&'[') {
                chars.next();
                for next in chars.by_ref() {
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            output.push(ch);
        }
    }

    output
}

/// Render a glass info panel with a title and key-value rows.
///
/// ```text
/// ┏━━ Title ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
/// ┃  Key              Value                            ┃
/// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
/// ```
///
/// When `color` is true, the title is rendered in cyan, borders in dim, and
/// keys in dim. Values use the default terminal colour.
pub fn render_glass_panel(title: &str, rows: &[(&str, &str)], width: usize, color: bool) -> String {

    let panel_width = width.max(title.len() + 10).max(30);
    let key_width = rows.iter().map(|(k, _)| k.chars().count()).max().unwrap_or(8);

    // Top border: ┏━━ Title ━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
    let after_title = panel_width.saturating_sub(title.chars().count() + 5); // ┏━━ + " " + ┓
    let top_fill = "━".repeat(after_title);
    let top = if color {
        format!("{ANSI_BORDER}┏━━ {ANSI_RST}{ANSI_CYAN}{title}{ANSI_RST}{ANSI_BORDER} {top_fill}┓{ANSI_RST}")
    } else {
        format!("┏━━ {title} {top_fill}┓")
    };

    // Rows: ┃  Key     Value     ┃
    let inner_width = panel_width.saturating_sub(2); // subtract ┃┃
    let row_lines: Vec<String> = rows
        .iter()
        .map(|(key, val)| {
            // available chars after ┃  key  (pad to key_width)  space space
            let val_area = inner_width.saturating_sub(key_width + 4); // "  " before key + "  " between key and val
            let val_trimmed: String = val.chars().take(val_area).collect();
            let val_padded = format!("{val_trimmed:<val_area$}");
            if color {
                format!("{ANSI_BORDER}┃{ANSI_RST}  {ANSI_DIM}{key:<key_width$}{ANSI_RST}  {val_padded}{ANSI_BORDER}┃{ANSI_RST}")
            } else {
                format!("┃  {key:<key_width$}  {val_padded}┃")
            }
        })
        .collect();

    // Bottom border: ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛
    let bottom_fill = "━".repeat(panel_width.saturating_sub(2));
    let bottom = if color {
        format!("{ANSI_BORDER}┗{bottom_fill}┛{ANSI_RST}")
    } else {
        format!("┗{bottom_fill}┛")
    };

    let mut lines = vec![top];
    lines.extend(row_lines);
    lines.push(bottom);
    lines.join("\n")
}

/// Return the number of columns to use as left margin for centering content.
///
/// Aims to centre a block of `content_width` chars within the terminal.  Falls
/// back to 2 when detection fails or the terminal is narrow.
#[allow(dead_code)]
pub fn terminal_margin(content_width: usize) -> usize {
    let cols = terminal_width();
    if cols <= content_width + 4 {
        return 2;
    }
    (cols.saturating_sub(content_width)) / 2
}

/// Render two glass panels side-by-side.
///
/// `total_width` is shared equally between the two panels (with 1 space gap).
#[allow(dead_code)]
pub fn render_glass_panel_pair(
    left: (&str, &[(&str, &str)]),
    right: (&str, &[(&str, &str)]),
    total_width: usize,
    color: bool,
) -> String {
    let half = total_width / 2;
    let left_panel = render_glass_panel(left.0, left.1, half.saturating_sub(1), color);
    let right_panel = render_glass_panel(right.0, right.1, half.saturating_sub(1), color);

    // Zip lines side by side. Pad shorter panel's lines to `half` chars width.
    let left_lines: Vec<&str> = left_panel.lines().collect();
    let right_lines: Vec<&str> = right_panel.lines().collect();
    let max_lines = left_lines.len().max(right_lines.len());

    let blank_left = " ".repeat(half.saturating_sub(1));
    let blank_right = " ".repeat(half.saturating_sub(1));

    (0..max_lines)
        .map(|i| {
            let l = left_lines.get(i).copied().unwrap_or(&blank_left);
            let r = right_lines.get(i).copied().unwrap_or(&blank_right);
            format!("{l} {r}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render the "✦ wraith" AI response label.
///
/// Printed once before the model's first text chunk. Full-width with a thin
/// top separator.
pub fn format_ai_label() -> String {
    let cols = terminal_width();
    let label = "✦ wraith";
    let remaining = cols.saturating_sub(label.chars().count() + 3);
    let fill = "━".repeat(remaining);
    format!(
        "\x1b[38;2;80;60;140m━\x1b[0m \x1b[1;38;2;153;102;255m{label}\x1b[0m \x1b[38;2;80;60;140m{fill}\x1b[0m\n"
    )
}

/// Full-width status line shown after a successful turn.
///
/// Thin bar with model name, token counts, timing, and progress bar.
pub fn format_turn_status(model: &str, input_tokens: u32, output_tokens: u32, context_tokens: u32, duration: std::time::Duration) -> String {
    let cols = terminal_width();
    let total = input_tokens + output_tokens;
    let secs = duration.as_secs_f64();
    let context_window = context_window_for_model(model);
    let percentage = ((f64::from(context_tokens) / f64::from(context_window)) * 100.0).round() as u32;
    
    // Create progress bar
    let bar_width = 10;
    let filled = (((f64::from(percentage) / 100.0) * (bar_width as f64)).round() as usize).min(bar_width);
    let empty = bar_width - filled;
    let progress_bar = format!(
        "{}\x1b[38;2;0;229;204m{}\x1b[0m{}\x1b[38;2;30;42;56m{}\x1b[0m{} \x1b[38;2;58;90;120m{}%\x1b[0m",
        " ",
        "━".repeat(filled),
        " ",
        "░".repeat(empty),
        " ",
        percentage
    );
    
    let pct_label = format!("{percentage}%");
    let progress_bar_visible_width = bar_width + pct_label.len() + 5;
    let stats = format!("{model} · {total} tokens ({input_tokens} in / {output_tokens} out) · {secs:.1}s · ");
    let remaining = cols.saturating_sub(stats.chars().count() + progress_bar_visible_width + 3);
    let fill = "━".repeat(remaining);
    format!(
        "\x1b[38;2;80;60;140m━\x1b[0m \x1b[38;2;88;120;160m{stats}\x1b[0m{progress_bar} \x1b[38;2;80;60;140m{fill}\x1b[0m"
    )
}

/// Get context window capacity for a model
fn context_window_for_model(model: &str) -> u32 {
    if model.contains("opus") || model.contains("sonnet") || model.contains("haiku") {
        200_000
    } else if model.contains("gpt-4") {
        128_000
    } else if model.contains("gemini") {
        1_000_000
    } else {
        128_000
    }
}

/// Render the keyboard hints line (displayed above the input box).
pub fn render_input_hints(color: bool) -> String {
    let margin = input_box_margin();
    let pad = " ".repeat(margin + 1);
    if !color {
        return format!("{pad}Enter · send | Ctrl+C · cancel | /exit · quit");
    }
    format!(
        "{pad}\x1b[38;2;88;120;160mEnter\x1b[38;2;55;75;100m · send  \x1b[38;2;60;80;110m│\x1b[0m  \x1b[38;2;88;120;160mCtrl+C\x1b[38;2;55;75;100m · cancel  \x1b[38;2;60;80;110m│\x1b[0m  \x1b[38;2;88;120;160m/exit\x1b[38;2;55;75;100m · quit\x1b[0m"
    )
}

/// Compute side margin for the input box (matches welcome panel: full-width).
pub fn input_box_margin() -> usize {
    0
}

/// Render the input box top border with corners and side margins.
///
/// ```text
///   ╭──────────────────────────────────────────────────╮
/// ```
pub fn render_input_top_border(color: bool) -> String {
    let cols = terminal_width();
    let margin = input_box_margin();
    let inner = cols.saturating_sub(margin * 2 + 2); // subtract margins + corners
    let pad = " ".repeat(margin);
    let fill = "─".repeat(inner);
    if color {
        format!("{pad}{ANSI_NEON}╭{fill}╮{ANSI_RST}")
    } else {
        format!("{pad}╭{fill}╮")
    }
}

/// Render the input box bottom border with corners and side margins.
pub fn render_separator(color: bool) -> String {
    let cols = terminal_width();
    let margin = input_box_margin();
    let inner = cols.saturating_sub(margin * 2 + 2);
    let pad = " ".repeat(margin);
    let fill = "─".repeat(inner);
    if color {
        format!("{pad}{ANSI_NEON}╰{fill}╯{ANSI_RST}")
    } else {
        format!("{pad}╰{fill}╯")
    }
}

/// Render the "You:" user message label with the message text.
///
/// Full-width separator with the label, like the AI label.
#[allow(dead_code)]
pub fn format_user_label() -> String {
    let cols = terminal_width();
    let label = "● You";
    let remaining = cols.saturating_sub(label.chars().count() + 3);
    let fill = "━".repeat(remaining);
    format!(
        "\x1b[38;2;80;60;140m━\x1b[0m \x1b[1;38;2;240;160;48m{label}{ANSI_RST} \x1b[38;2;80;60;140m{fill}{ANSI_RST}"
    )
}

/// Return the terminal width, defaulting to 80 when detection fails.
pub fn terminal_width() -> usize {
    crossterm::terminal::size().map(|(c, _)| c as usize).unwrap_or(80)
}

/// Render the WRAITH ASCII-art banner — full-width responsive layout.
///
/// The frame stretches to fill the entire terminal width. The ASCII art is
/// centred inside it. A status bar appears below the frame.
pub fn render_wraith_banner(tagline: &str, info_line: &str, color: bool) -> String {
    const ART: &[&str] = &[
        " ██╗    ██╗██████╗  █████╗ ██╗████████╗██╗  ██╗",
        " ██║    ██║██╔══██╗██╔══██╗██║╚══██╔══╝██║  ██║",
        " ██║ █╗ ██║██████╔╝███████║██║   ██║   ███████║",
        " ██║███╗██║██╔══██╗██╔══██║██║   ██║   ██╔══██║",
        " ╚███╔███╔╝██║  ██║██║  ██║██║   ██║   ██║  ██║",
        "  ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═╝   ╚═╝   ╚═╝  ╚═╝",
    ];

    let cols = terminal_width();

    // ── Narrow terminal fallback: skip bordered frame ────────
    if cols < 60 {
        let mut lines: Vec<String> = Vec::with_capacity(ART.len() + 4);
        lines.push(String::new());
        for (i, art_line) in ART.iter().enumerate() {
            if color {
                let total = ART.len() as f32 - 1.0;
                let t = if total > 0.0 { i as f32 / total } else { 0.0 };
                let r = (t * 153.0) as u8;
                let g = (229.0 + t * (102.0_f32 - 229.0)) as u8;
                let b = (204.0 + t * (255.0_f32 - 204.0)) as u8;
                lines.push(format!("\x1b[38;2;{r};{g};{b}m{art_line}{ANSI_RST}"));
            } else {
                lines.push((*art_line).to_string());
            }
        }
        if color {
            lines.push(format!("  {ANSI_DIM}{tagline}{ANSI_RST}"));
            lines.push(format!("  {ANSI_DIM}{info_line}{ANSI_RST}"));
        } else {
            lines.push(format!("  {tagline}"));
            lines.push(format!("  {info_line}"));
        }
        lines.push(String::new());
        return lines.join("\n");
    }
    // Frame spans the full terminal width (leave 0 margin)
    let frame_inner = cols.saturating_sub(2); // subtract ║ on each side

    let mut lines: Vec<String> = Vec::with_capacity(ART.len() + 12);

    // ── Top border (full-width) ─────────────────────────────────
    let top_fill = "═".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}╔{top_fill}╗{ANSI_RST}"));
    } else {
        lines.push(format!("╔{top_fill}╗"));
    }

    // ── Blank line inside frame ─────────────────────────────────
    let blank_inner = " ".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}║{ANSI_RST}{blank_inner}{ANSI_BORDER}║{ANSI_RST}"));
    } else {
        lines.push(format!("║{blank_inner}║"));
    }

    // ── ASCII art lines (gradient) ─────────────────────────────
    let total = ART.len() as f32 - 1.0;
    for (i, art_line) in ART.iter().enumerate() {
        let line_width = art_line.chars().count();
        let inner_pad_left = (frame_inner.saturating_sub(line_width)) / 2;
        let inner_pad_right = frame_inner.saturating_sub(line_width + inner_pad_left);
        let lp = " ".repeat(inner_pad_left);
        let rp = " ".repeat(inner_pad_right);
        if color {
            let t = if total > 0.0 { i as f32 / total } else { 0.0 };
            let r = (t * 153.0) as u8;
            let g = (229.0 + t * (102.0_f32 - 229.0)) as u8;
            let b = (204.0 + t * (255.0_f32 - 204.0)) as u8;
            lines.push(format!(
                "{ANSI_BORDER}║{ANSI_RST}{lp}\x1b[38;2;{r};{g};{b}m{art_line}{ANSI_RST}{rp}{ANSI_BORDER}║{ANSI_RST}"
            ));
        } else {
            lines.push(format!("║{lp}{art_line}{rp}║"));
        }
    }

    // ── Blank line after art ────────────────────────────────────
    if color {
        lines.push(format!("{ANSI_BORDER}║{ANSI_RST}{blank_inner}{ANSI_BORDER}║{ANSI_RST}"));
    } else {
        lines.push(format!("║{blank_inner}║"));
    }

    // ── Tagline (centred) ───────────────────────────────────────
    let tag_pad_left = (frame_inner.saturating_sub(tagline.len())) / 2;
    let tag_pad_right = frame_inner.saturating_sub(tagline.len() + tag_pad_left);
    if color {
        lines.push(format!(
            "{ANSI_BORDER}║{ANSI_RST}{}{ANSI_DIM}{tagline}{ANSI_RST}{}{ANSI_BORDER}║{ANSI_RST}",
            " ".repeat(tag_pad_left),
            " ".repeat(tag_pad_right),
        ));
    } else {
        lines.push(format!(
            "║{}{tagline}{}║",
            " ".repeat(tag_pad_left),
            " ".repeat(tag_pad_right),
        ));
    }

    // ── Bottom border ───────────────────────────────────────────
    let bot_fill = "═".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}╚{bot_fill}╝{ANSI_RST}"));
    } else {
        lines.push(format!("╚{bot_fill}╝"));
    }

    // ── Status bar (below frame) ────────────────────────────────
    // Format: ● connected │ Model: xxx │ Workspace │ v0.1.0
    if color {
        let model_part = info_line.split("   ").next().unwrap_or("");
        let version_part = info_line.split("   ").last().unwrap_or("");
        let workspace_part = info_line.split("   ").nth(1).unwrap_or("");

        // Truncate model/workspace if the status bar would exceed terminal width.
        // Fixed chrome: " ● connected  │    │    │  " = 27 visible chars
        let chrome_len: usize = 27;
        let budget = cols.saturating_sub(chrome_len + version_part.len());
        let model_vis: std::borrow::Cow<'_, str> = if model_part.len() > budget / 2 {
            let max = (budget / 2).max(3);
            let end = model_part.floor_char_boundary(max.saturating_sub(4));
            format!("{}…", &model_part[..end]).into()
        } else {
            model_part.into()
        };
        let ws_budget = budget.saturating_sub(model_vis.len());
        let ws_vis: std::borrow::Cow<'_, str> = if workspace_part.len() > ws_budget {
            let max = ws_budget.max(3);
            let end = workspace_part.floor_char_boundary(max.saturating_sub(4));
            format!("{}…", &workspace_part[..end]).into()
        } else {
            workspace_part.into()
        };

        lines.push(format!(
            " {ANSI_GREEN}●{ANSI_RST} {ANSI_DIM}connected{ANSI_RST}  {ANSI_BORDER}│{ANSI_RST}  {ANSI_CYAN}{model_vis}{ANSI_RST}  {ANSI_BORDER}│{ANSI_RST}  {ANSI_DIM}{ws_vis}{ANSI_RST}  {ANSI_BORDER}│{ANSI_RST}  {ANSI_VIOLET}{version_part}{ANSI_RST}"
        ));
    }

    lines.join("\n")
}

/// Render a bordered onboarding / welcome panel below the banner.
///
/// Full-width, with tips and shortcuts in a clean layout.
pub fn render_welcome_panel(has_wraith_md: bool, color: bool) -> String {
    let cols = terminal_width();
    let frame_inner = cols.saturating_sub(2);

    let mut lines: Vec<String> = Vec::with_capacity(16);

    // ── Top border ─────────────────────────────────────────────
    let top_fill = "─".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}╭{top_fill}╮{ANSI_RST}"));
    } else {
        lines.push(format!("╭{top_fill}╮"));
    }

    // Helper: render a row with left-aligned content, padded to fill width
    let row = |content: &str, visible_len: usize| -> String {
        let padding = frame_inner.saturating_sub(visible_len + 2); // 2 for "  " indent
        if color {
            format!("{ANSI_BORDER}│{ANSI_RST}  {content}{}{ANSI_BORDER}│{ANSI_RST}", " ".repeat(padding))
        } else {
            format!("│  {content}{}│", " ".repeat(padding))
        }
    };

    // Blank row helper
    let blank_row = || -> String {
        let inner = " ".repeat(frame_inner);
        if color {
            format!("{ANSI_BORDER}│{ANSI_RST}{inner}{ANSI_BORDER}│{ANSI_RST}")
        } else {
            format!("│{inner}│")
        }
    };

    // ── Section: Getting Started ────────────────────────────────
    if color {
        lines.push(row(
            &format!("{ANSI_AMBER}Getting Started{ANSI_RST}"),
            "Getting Started".len(),
        ));
    } else {
        lines.push(row("Getting Started", "Getting Started".len()));
    }
    lines.push(blank_row());

    if has_wraith_md {
        let tips: &[(&str, &str)] = &[
            ("Ask a question", "\"explain the auth flow in this project\""),
            ("Edit code", "\"add error handling to the login endpoint\""),
            ("Run commands", "\"run the test suite and fix failures\""),
            ("Explore", "\"/help for commands · /status for context\""),
        ];
        for (i, (label, example)) in tips.iter().enumerate() {
            let num = i + 1;
            if color {
                let content = format!(
                    "{ANSI_DIM}{num}.{ANSI_RST} {ANSI_CYAN}{label}{ANSI_RST}  {ANSI_DIM}— {example}{ANSI_RST}"
                );
                let visible = format!("{num}. {label}  — {example}");
                lines.push(row(&content, visible.chars().count()));
            } else {
                let content = format!("{num}. {label}  — {example}");
                lines.push(row(&content, content.chars().count()));
            }
        }
    } else {
        let tips: &[(&str, &str)] = &[
            ("Initialize workspace", "/init to scaffold WRAITH.md and config"),
            ("Ask a question", "\"what does this project do?\""),
            ("Get help", "/help to see all available commands"),
        ];
        for (i, (label, example)) in tips.iter().enumerate() {
            let num = i + 1;
            if color {
                let content = format!(
                    "{ANSI_DIM}{num}.{ANSI_RST} {ANSI_CYAN}{label}{ANSI_RST}  {ANSI_DIM}— {example}{ANSI_RST}"
                );
                let visible = format!("{num}. {label}  — {example}");
                lines.push(row(&content, visible.chars().count()));
            } else {
                let content = format!("{num}. {label}  — {example}");
                lines.push(row(&content, content.chars().count()));
            }
        }
    }

    lines.push(blank_row());

    // ── Section: Shortcuts ──────────────────────────────────────
    let sep = "─".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}├{sep}┤{ANSI_RST}"));
    } else {
        lines.push(format!("├{sep}┤"));
    }

    if color {
        lines.push(row(
            &format!("{ANSI_AMBER}Shortcuts{ANSI_RST}"),
            "Shortcuts".len(),
        ));
    } else {
        lines.push(row("Shortcuts", "Shortcuts".len()));
    }
    lines.push(blank_row());

    let shortcuts: &[(&str, &str)] = &[
        ("Tab", "autocomplete slash commands"),
        ("Shift+Enter / Ctrl+J", "insert newline (multiline input)"),
        ("/vim", "toggle modal editing"),
        ("/compact", "compress conversation history"),
    ];
    for (key, desc) in shortcuts {
        if color {
            let content = format!("{ANSI_VIOLET}{key:<24}{ANSI_RST}{ANSI_DIM}{desc}{ANSI_RST}");
            let visible = format!("{key:<24}{desc}");
            lines.push(row(&content, visible.chars().count()));
        } else {
            let content = format!("{key:<24}{desc}");
            lines.push(row(&content, content.chars().count()));
        }
    }

    lines.push(blank_row());

    // ── Bottom border ───────────────────────────────────────────
    let bot_fill = "─".repeat(frame_inner);
    if color {
        lines.push(format!("{ANSI_BORDER}╰{bot_fill}╯{ANSI_RST}"));
    } else {
        lines.push(format!("╰{bot_fill}╯"));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{strip_ansi, MarkdownStreamState, Spinner, TerminalRenderer};

    #[test]
    fn renders_markdown_with_styling_and_lists() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output = terminal_renderer
            .render_markdown("# Heading\n\nThis is **bold** and *italic*.\n\n- item\n\n`code`");

        assert!(markdown_output.contains("Heading"));
        assert!(markdown_output.contains("• item"));
        assert!(markdown_output.contains("code"));
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn renders_links_as_colored_markdown_labels() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.render_markdown("See [Wraith](https://example.com/docs) now.");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("[Wraith](https://example.com/docs)"));
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn highlights_fenced_code_blocks() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.markdown_to_ansi("```rust\nfn hi() { println!(\"hi\"); }\n```");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("┌─") && plain_text.contains("rust"));
        assert!(plain_text.contains("fn hi"));
        assert!(markdown_output.contains('\u{1b}'));
        assert!(markdown_output.contains("[48;2;18;24;33m"));
    }

    #[test]
    fn renders_ordered_and_nested_lists() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output =
            terminal_renderer.render_markdown("1. first\n2. second\n   - nested\n   - child");
        let plain_text = strip_ansi(&markdown_output);

        assert!(plain_text.contains("1. first"));
        assert!(plain_text.contains("2. second"));
        assert!(plain_text.contains("  • nested"));
        assert!(plain_text.contains("  • child"));
    }

    #[test]
    fn renders_tables_with_alignment() {
        let terminal_renderer = TerminalRenderer::new();
        let markdown_output = terminal_renderer
            .render_markdown("| Name | Value |\n| ---- | ----- |\n| alpha | 1 |\n| beta | 22 |");
        let plain_text = strip_ansi(&markdown_output);
        let lines = plain_text.lines().collect::<Vec<_>>();

        assert_eq!(lines[0], "│ Name  │ Value │");
        assert_eq!(lines[1], "│───────┼───────│");
        assert_eq!(lines[2], "│ alpha │ 1     │");
        assert_eq!(lines[3], "│ beta  │ 22    │");
        assert!(markdown_output.contains('\u{1b}'));
    }

    #[test]
    fn streaming_state_waits_for_complete_blocks() {
        let renderer = TerminalRenderer::new();
        let mut state = MarkdownStreamState::default();

        assert_eq!(state.push(&renderer, "# Heading"), None);
        let flushed = state
            .push(&renderer, "\n\nParagraph\n\n")
            .expect("completed block");
        let plain_text = strip_ansi(&flushed);
        assert!(plain_text.contains("Heading"));
        assert!(plain_text.contains("Paragraph"));

        assert_eq!(state.push(&renderer, "```rust\nfn main() {}\n"), None);
        let code = state
            .push(&renderer, "```\n")
            .expect("closed code fence flushes");
        assert!(strip_ansi(&code).contains("fn main()"));
    }

    #[test]
    fn spinner_advances_frames() {
        let terminal_renderer = TerminalRenderer::new();
        let mut spinner = Spinner::new();
        let mut out = Vec::new();
        spinner
            .tick("Working", terminal_renderer.color_theme(), &mut out)
            .expect("tick succeeds");
        spinner
            .tick("Working", terminal_renderer.color_theme(), &mut out)
            .expect("tick succeeds");

        let output = String::from_utf8_lossy(&out);
        assert!(output.contains("Working"));
    }
}
