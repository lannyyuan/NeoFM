//! Syntax highlighting engine for the TUI.

use ratatui::style::Color as RtColor;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use std::sync::OnceLock;
use syntect::easy::HighlightLines;
use syntect::highlighting::Color as SyntectColor;
use syntect::highlighting::FontStyle;
use syntect::highlighting::Highlighter;
use syntect::highlighting::Style as SyntectStyle;
use syntect::highlighting::Theme;
use syntect::parsing::Scope;
use syntect::parsing::SyntaxReference;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use two_face::theme::EmbeddedThemeName;

static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
static THEME: OnceLock<Theme> = OnceLock::new();

#[cfg(test)]
thread_local! {
    static TEST_THEME: std::cell::RefCell<Option<Theme>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn set_syntax_theme(theme: Theme) {
    TEST_THEME.replace(Some(theme));
}

pub(crate) fn current_syntax_theme() -> Theme { with_syntax_theme(Clone::clone) }

pub(crate) fn with_syntax_theme<T>(f: impl FnOnce(&Theme) -> T) -> T {
    #[cfg(test)]
    let f = match TEST_THEME.with_borrow(|theme| match theme {
        Some(theme) => Ok(f(theme)),
        None => Err(f),
    }) {
        Ok(value) => return value,
        Err(f) => f,
    };
    f(THEME.get_or_init(|| {
        two_face::theme::extra().get(EmbeddedThemeName::CatppuccinMocha).clone()
    }))
}

// Syntect/bat encode ANSI palette semantics in alpha:
// `a=0` => indexed ANSI palette via RGB payload, `a=1` => terminal default.
const ANSI_ALPHA_INDEX: u8 = 0x00;
const ANSI_ALPHA_DEFAULT: u8 = 0x01;
const OPAQUE_ALPHA: u8 = 0xFF;

fn syntax_set() -> &'static SyntaxSet {
    SYNTAX_SET.get_or_init(two_face::syntax::extra_newlines)
}

pub(crate) fn foreground_style_for_scopes(scope_names: &[&str]) -> Option<Style> {
    with_syntax_theme(|theme| foreground_style_for_scopes_with_theme(theme, scope_names))
}

pub(crate) fn foreground_style_for_scopes_with_theme(
    theme: &Theme,
    scope_names: &[&str],
) -> Option<Style> {
    foreground_color_for_scopes(&Highlighter::new(theme), scope_names)
        .map(|fg| Style::default().fg(fg))
}

pub(crate) fn foreground_color_for_scopes(
    highlighter: &Highlighter<'_>,
    scope_names: &[&str],
) -> Option<RtColor> {
    scope_names.iter().find_map(|scope_name| {
        let scope = Scope::new(scope_name).ok()?;
        let fg = highlighter.style_mod_for_stack(&[scope]).foreground?;
        convert_syntect_color(fg)
    })
}

// -- Style conversion (syntect -> ratatui) ------------------------------------

/// Map a low ANSI palette index (0–7) to ratatui's named color variants,
/// falling back to `Indexed(n)` for indices 8–255.
///
/// Named variants are preferred over `Indexed(0)`…`Indexed(7)` because many
/// terminals apply bold/bright treatment differently for named vs indexed
/// colors, and ANSI themes expect the named behavior.
///
/// `clippy::disallowed_methods` is explicitly allowed here because this helper
/// intentionally constructs `ratatui::style::Color::Indexed`.
#[allow(clippy::disallowed_methods)]
fn ansi_palette_color(index: u8) -> RtColor {
    match index {
        0x00 => RtColor::Black,
        0x01 => RtColor::Red,
        0x02 => RtColor::Green,
        0x03 => RtColor::Yellow,
        0x04 => RtColor::Blue,
        0x05 => RtColor::Magenta,
        0x06 => RtColor::Cyan,
        // ANSI code 37 is "white", represented as `Gray` in ratatui.
        0x07 => RtColor::Gray,
        n => RtColor::Indexed(n),
    }
}

/// Decode a syntect foreground `Color` into a ratatui color, respecting the
/// alpha-channel encoding that bat's `ansi`, `base16`, and `base16-256` themes
/// use to signal ANSI palette semantics instead of true RGB.
///
/// Returns `None` when the color signals "use the terminal's default
/// foreground", allowing the caller to omit the foreground attribute entirely.
///
/// Passing a color from a standard RGB theme (alpha 0xFF) returns
/// `Some(Rgb(..))`, so this function is backward-compatible with non-ANSI
/// themes. Unexpected intermediate alpha values are treated as RGB.
///
/// `clippy::disallowed_methods` is explicitly allowed here because this helper
/// intentionally constructs `ratatui::style::Color::Rgb`.
#[allow(clippy::disallowed_methods)]
pub(crate) fn convert_syntect_color(color: SyntectColor) -> Option<RtColor> {
    match color.a {
        // Bat-compatible encoding used by `ansi`, `base16`, and `base16-256`:
        // alpha 0x00 means `r` stores an ANSI palette index, not RGB red.
        ANSI_ALPHA_INDEX => Some(ansi_palette_color(color.r)),
        // alpha 0x01 means "use terminal default foreground/background".
        ANSI_ALPHA_DEFAULT => None,
        OPAQUE_ALPHA => Some(RtColor::Rgb(color.r, color.g, color.b)),
        // Non-ANSI alpha values appear in some bundled themes; treat as plain RGB.
        _ => Some(RtColor::Rgb(color.r, color.g, color.b)),
    }
}

/// Convert a syntect `Style` to a ratatui `Style`.
///
/// Most themes produce RGB colors. The built-in `ansi`/`base16`/`base16-256`
/// themes encode ANSI palette semantics in the alpha channel, matching bat. Preserve
/// the decoded foreground so the final renderer can use its actual background.
fn convert_style(syn_style: SyntectStyle) -> Style {
    let mut rt_style = Style::default();

    if let Some(fg) = convert_syntect_color(syn_style.foreground) {
        rt_style = rt_style.fg(fg);
    }
    // Intentionally skip background to avoid overwriting terminal bg.
    // If background support is added later, decode with `convert_syntect_color`
    // to reuse the same alpha-marker semantics as foreground.

    if syn_style.font_style.contains(FontStyle::BOLD) {
        rt_style.add_modifier |= Modifier::BOLD;
    }
    // Intentionally skip italic — many terminals render it poorly or not at all.
    // Intentionally skip underline — themes like Dracula use underline on type
    // scopes (entity.name.type, support.class) which produces distracting
    // underlines on type/module names in terminal output.

    rt_style
}

// -- Syntax lookup ------------------------------------------------------------

/// Try to find a syntect `SyntaxReference` for the given language identifier.
///
/// two-face's extended syntax set (~250 languages) resolves most names and
/// extensions directly.  We only patch the few aliases it cannot handle.
fn find_syntax(lang: &str) -> Option<&'static SyntaxReference> {
    let ss = syntax_set();

    // Aliases that two-face does not resolve on its own.
    let normalized = lang.to_ascii_lowercase();
    let patched = match normalized.as_str() {
        "csharp" | "c-sharp" => "c#",
        // CUDA source (.cu) and header (.cuh) files use C++ highlighting as a fallback.
        "cu" | "cuh" => "cpp",
        "cppm" | "cxxm" | "ixx" => "cpp",
        "golang" => "go",
        "python3" => "python",
        "shell" => "bash",
        _ => lang,
    };

    // Try by token (matches file_extensions case-insensitively).
    if let Some(s) = ss.find_syntax_by_token(patched) {
        return Some(s);
    }
    // Try by exact syntax name (e.g. "Rust", "Python").
    if let Some(s) = ss.find_syntax_by_name(patched) {
        return Some(s);
    }
    // Try case-insensitive name match (e.g. "rust" -> "Rust").
    let lower = patched.to_ascii_lowercase();
    if let Some(s) = ss
        .syntaxes()
        .iter()
        .find(|s| s.name.to_ascii_lowercase() == lower)
    {
        return Some(s);
    }
    // Try raw input as file extension.
    if let Some(s) = ss.find_syntax_by_extension(lang) {
        return Some(s);
    }
    None
}

// -- Guardrail constants ------------------------------------------------------

/// Skip highlighting for inputs larger than 512 KB to avoid excessive memory
/// and CPU usage.  Callers fall back to plain unstyled text.
const MAX_HIGHLIGHT_BYTES: usize = 512 * 1024;

/// Skip highlighting for inputs with more than 10,000 lines.
const MAX_HIGHLIGHT_LINES: usize = 10_000;

/// Skip highlighting when an individual line is longer than 4 KiB.
pub(crate) const MAX_HIGHLIGHT_LINE_BYTES: usize = 4 * 1024;

/// Check whether an input exceeds the safe highlighting limits.
///
/// Callers that highlight content in a loop (e.g. per diff-line) should
/// pre-check the aggregate size with this function and skip highlighting
/// entirely when it returns `true`.
pub(crate) fn exceeds_highlight_limits(total_bytes: usize, total_lines: usize) -> bool {
    total_bytes > MAX_HIGHLIGHT_BYTES || total_lines > MAX_HIGHLIGHT_LINES
}

// -- Core highlighting --------------------------------------------------------

/// Core highlighter that accepts an explicit theme reference.
///
/// This keeps production behavior and test behavior on the same code path:
/// production callers pass the global theme lock, while tests can pass a
/// concrete theme without mutating process-global state.
fn highlight_to_line_spans_with_theme(
    code: &str,
    lang: &str,
    theme: &Theme,
) -> Option<Vec<Vec<Span<'static>>>> {
    // Empty input has nothing to highlight; fall back to the plain text path
    // which correctly produces a single empty Line.
    if code.is_empty() {
        return None;
    }

    // Bail out early for oversized inputs to avoid excessive resource usage.
    // Count actual lines (not newline bytes) to avoid an off-by-one when
    // the input does not end with a newline.
    if code.len() > MAX_HIGHLIGHT_BYTES
        || code.lines().count() > MAX_HIGHLIGHT_LINES
        || code
            .lines()
            .any(|line| line.len() > MAX_HIGHLIGHT_LINE_BYTES)
    {
        return None;
    }

    let syntax = find_syntax(lang)?;
    let mut h = HighlightLines::new(syntax, theme);
    let mut lines: Vec<Vec<Span<'static>>> = Vec::new();

    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, syntax_set()).ok()?;
        lines.push(highlighted_line_spans(ranges));
    }

    Some(lines)
}

/// Convert one highlighted line into the spans shared by full and streaming renders.
///
/// Source line endings are omitted, while empty lines retain their canonical empty span.
fn highlighted_line_spans(ranges: Vec<(SyntectStyle, &str)>) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    for (style, text) in ranges {
        // Line breaks are represented by the surrounding Line, not its spans.
        let text = text.trim_end_matches(['\n', '\r']);
        if !text.is_empty() {
            spans.push(Span::styled(text.to_string(), convert_style(style)));
        }
    }
    if spans.is_empty() {
        spans.push(Span::raw(String::new()));
    }
    spans
}

/// Parse `code` using syntect for `lang` and return per-line styled spans.
/// Each inner Vec represents one source line.  Returns None when the language
/// is not recognized or the input exceeds safety limits.
fn highlight_to_line_spans(code: &str, lang: &str) -> Option<Vec<Vec<Span<'static>>>> {
    with_syntax_theme(|theme| highlight_to_line_spans_with_theme(code, lang, theme))
}

// -- Public API ---------------------------------------------------------------

/// Highlight code in any supported language, returning styled ratatui `Line`s.
///
/// Falls back to plain unstyled text when the language is not recognized or the
/// input exceeds safety guardrails.  Callers can always render the result
/// directly -- the fallback path produces equivalent plain-text lines.
///
/// Preserves theme foregrounds: callers may apply a shaded background after rendering.
/// Used by `markdown_render` for fenced code blocks and by `exec_cell` for bash
/// command highlighting.
pub(crate) fn highlight_code_to_lines(code: &str, lang: &str) -> Vec<Line<'static>> {
    if let Some(line_spans) = highlight_to_line_spans(code, lang) {
        line_spans.into_iter().map(Line::from).collect()
    } else {
        // Fallback: plain text, one Line per source line.
        // Use `lines()` instead of `split('\n')` to avoid a phantom trailing
        // empty element when the input ends with '\n' (as pulldown-cmark emits).
        let mut result: Vec<Line<'static>> =
            code.lines().map(|l| Line::from(l.to_string())).collect();
        if result.is_empty() {
            result.push(Line::from(String::new()));
        }
        result
    }
}
