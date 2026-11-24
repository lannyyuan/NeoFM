//! Semantic terminal hyperlinks carried separately from visible TUI text.
//!
//! Layout code measures and wraps ordinary ratatui lines. Hyperlink annotations are applied only
//! when text reaches a terminal buffer or scrollback writer so OSC 8 bytes never affect geometry.

mod source;

pub(crate) use source::LineWrapPolicy;
pub(crate) use source::LogicalLineSource;

use std::ops::Range;

use ratatui::buffer::CellWidth;
use ratatui::text::Line;
use ratatui::text::Span;
use unicode_segmentation::UnicodeSegmentation;
use url::Url;

use crate::line_truncation::line_width;
use crate::markdown_render::FileLink;
use crate::markdown_render::SourceLocation;
use crate::render::line_utils::line_to_borrowed;
use crate::render::line_utils::line_to_static;
use crate::width::display_width;
use crate::wrapping::RtOptions;

// Destinations are repeated in every linked buffer cell. Leave oversized URLs as plain text.
const MAX_HYPERLINK_DESTINATION_BYTES: usize = 8 * 1024;

#[derive(Clone, Eq)]
pub(crate) struct TerminalHyperlink {
    pub(crate) columns: Range<usize>,
    pub(crate) destination: String,
    destination_kind: DestinationKind,
    /// Byte offset of an explicit Markdown link, preserved through wrapping.
    pub(crate) markdown_source: Option<usize>,
}

// Rendering equality excludes source provenance, which can differ between streaming and full renders.
impl PartialEq for TerminalHyperlink {
    fn eq(&self, other: &Self) -> bool {
        self.columns == other.columns
            && self.destination == other.destination
            && self.destination_kind == other.destination_kind
    }
}

// The link offset is layout metadata, like HyperlinkLine source provenance.
impl std::fmt::Debug for TerminalHyperlink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TerminalHyperlink")
            .field("columns", &self.columns)
            .field("destination", &self.destination)
            .field("destination_kind", &self.destination_kind)
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DestinationKind {
    Web,
    /// A local document target used only by an enabled workspace transcript.
    Document(Option<SourceLocation>),
}

impl TerminalHyperlink {
    pub(crate) fn terminal_destination(&self) -> Option<String> {
        match self.destination_kind {
            DestinationKind::Document(_) => None,
            DestinationKind::Web => web_destination(&self.destination),
        }
    }

    pub(crate) fn with_columns(&self, columns: Range<usize>) -> Self {
        Self {
            columns,
            destination: self.destination.clone(),
            destination_kind: self.destination_kind,
            markdown_source: self.markdown_source,
        }
    }


    pub(crate) fn web(columns: Range<usize>, destination: String) -> Self {
        Self {
            columns,
            destination,
            destination_kind: DestinationKind::Web,
            markdown_source: None,
        }
    }

    pub(crate) fn document(columns: Range<usize>, link: FileLink) -> Self {
        Self {
            columns,
            destination: link.path,
            destination_kind: DestinationKind::Document(link.location),
            markdown_source: None,
        }
    }

    pub(crate) fn is_document(&self) -> bool {
        matches!(self.destination_kind, DestinationKind::Document(_))
    }


}

#[derive(Clone, Default)]
pub(crate) struct HyperlinkLine {
    pub(crate) line: Line<'static>,
    pub(crate) hyperlinks: Vec<TerminalHyperlink>,
    pub(crate) source: Option<LogicalLineSource>,
}

// Source provenance is shared layout metadata; omit it from visual diagnostics to avoid
// repeating an entire logical line for every wrapped fragment.
impl std::fmt::Debug for HyperlinkLine {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("HyperlinkLine")
            .field("line", &self.line)
            .field("hyperlinks", &self.hyperlinks)
            .finish()
    }
}

impl PartialEq for HyperlinkLine {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line && self.hyperlinks == other.hyperlinks
    }
}

impl Eq for HyperlinkLine {}

/// Cache equality also tracks source whitespace hidden by display wrapping.
pub(crate) fn lines_with_sources_eq(left: &[HyperlinkLine], right: &[HyperlinkLine]) -> bool {
    left == right
        && left.iter().zip(right).all(|(left, right)| {
            left.source == right.source
                && left
                    .hyperlinks
                    .iter()
                    .zip(&right.hyperlinks)
                    .all(|(left, right)| left.markdown_source == right.markdown_source)
        })
}

pub(crate) fn source_span_width(span: &Span<'_>) -> usize {
    span.content
        .graphemes(/*is_extended*/ true)
        .map(display_width)
        .sum()
}

impl HyperlinkLine {
    pub(crate) fn new(line: Line<'static>) -> Self {
        Self {
            line,
            hyperlinks: Vec::new(),
            source: None,
        }
    }

    pub(crate) fn width(&self) -> usize {
        line_width(&self.line)
    }

    /// The transcript expands tabs after wrapping. Keep their source columns until then.
    pub(crate) fn source_width(&self) -> usize {
        self.line.iter().map(source_span_width).sum()
    }

    pub(crate) fn push_span(&mut self, span: Span<'static>, destination: Option<&str>) {
        if let Some(destination) = destination.and_then(markdown_destination) {
            self.push_linked_span(span, self.source_width(), destination);
        } else {
            self.source = None;
            self.line.push_span(span);
        }
    }

    pub(crate) fn push_linked_span(
        &mut self,
        span: Span<'static>,
        start: usize,
        destination: String,
    ) {
        self.source = None;
        let end = start + source_span_width(&span);
        self.line.push_span(span);
        if end > start {
            self.hyperlinks
                .push(TerminalHyperlink::web(start..end, destination));
        }
    }

    pub(crate) fn clip_hyperlinks_to_width(&mut self, width: usize) {
        self.hyperlinks.retain_mut(|link| {
            link.columns.end = link.columns.end.min(width);
            !link.columns.is_empty()
        });
    }

    /// Native scrollback paints the row style last. Resolve text styles onto the spans so
    /// inline Markdown and gutter overrides survive, keeping the background for empty columns.
    pub(crate) fn resolve_span_styles(mut self) -> Self {
        let style = std::mem::take(&mut self.line.style);
        self.line.style.bg = style.bg;
        for span in &mut self.line.spans {
            span.style = style.patch(span.style);
        }
        self
    }

    pub(crate) fn style(mut self, style: ratatui::style::Style) -> Self {
        self.line = self.line.style(style);
        if let Some(source) = &mut self.source {
            source.line_style = style;
        }
        self
    }
}

impl From<Line<'static>> for HyperlinkLine {
    fn from(line: Line<'static>) -> Self {
        Self::new(line)
    }
}

impl From<&'static str> for HyperlinkLine {
    fn from(text: &'static str) -> Self {
        Self::new(Line::from(text))
    }
}

impl From<String> for HyperlinkLine {
    fn from(text: String) -> Self {
        Self::new(Line::from(text))
    }
}

pub(crate) fn visible_lines(lines: Vec<HyperlinkLine>) -> Vec<Line<'static>> {
    lines.into_iter().map(|line| line.line).collect()
}

pub(crate) fn visible_lines_ref(lines: &[HyperlinkLine]) -> Vec<Line<'_>> {
    lines
        .iter()
        .map(|line| line_to_borrowed(&line.line))
        .collect()
}

pub(crate) fn plain_hyperlink_lines(lines: Vec<Line<'static>>) -> Vec<HyperlinkLine> {
    lines.into_iter().map(HyperlinkLine::new).collect()
}

pub(crate) fn prefix_hyperlink_lines(
    lines: Vec<HyperlinkLine>,
    initial_prefix: Span<'static>,
    subsequent_prefix: Span<'static>,
) -> Vec<HyperlinkLine> {
    lines
        .into_iter()
        .enumerate()
        .map(|(index, mut line)| {
            let prefix = if index == 0 {
                initial_prefix.clone()
            } else {
                subsequent_prefix.clone()
            };
            let shift = display_width(prefix.content.as_ref());
            let mut source = line
                .source
                .take()
                .unwrap_or_else(|| LogicalLineSource::from_line(&line.line));
            source.prefix_bytes += prefix.content.len();
            source
                .continuation_indent
                .spans
                .insert(/*index*/ 0, subsequent_prefix.clone());
            line.source = Some(source);
            let mut spans = Vec::with_capacity(line.line.spans.len() + 1);
            spans.push(prefix);
            spans.extend(line.line.spans);
            line.line = Line::from(spans).style(line.line.style);
            for hyperlink in &mut line.hyperlinks {
                hyperlink.columns = hyperlink.columns.start + shift..hyperlink.columns.end + shift;
            }
            line
        })
        .collect()
}

/// Retain a known first-line label in copy text without changing the existing wrapping.
///
/// The label may precede Markdown indentation or follow a display-only gutter.
/// Only the first row's shared logical source is updated.
pub(crate) fn retain_initial_prefix(lines: &mut [HyperlinkLine], prefix: &str) {
    let Some(origin) = lines.first().and_then(|line| line.source.clone()) else {
        return;
    };
    let first = LogicalLineSource::from_line(&lines[0].line);
    let indent = &first.text[..origin.prefix_bytes];
    let Some(label_start) = indent.rfind(prefix) else {
        return;
    };
    let retained = &indent[label_start..];
    let prefix_line = first.styled_range(label_start..origin.prefix_bytes);
    let mut styles = LogicalLineSource::from_line(&prefix_line).styles.to_vec();
    styles.extend(origin.styles.iter().map(|(range, style)| {
        (
            range.start + retained.len()..range.end + retained.len(),
            origin.line_style.patch(*style).patch(origin.span_style),
        )
    }));
    let styles = std::sync::Arc::from(styles);
    let text: std::sync::Arc<str> = format!("{retained}{}", origin.text).into();
    let labelled_body = origin
        .copy
        .as_ref()
        .map(|_| std::sync::Arc::new((prefix.len(), origin.clone())));
    for (index, line) in lines.iter_mut().enumerate() {
        let Some(source) = line
            .source
            .as_mut()
            .filter(|source| std::sync::Arc::ptr_eq(&source.text, &origin.text))
        else {
            continue;
        };
        source.text = std::sync::Arc::clone(&text);
        source.styles = std::sync::Arc::clone(&styles);
        source.labelled_body = labelled_body.clone();
        source.line_style = ratatui::style::Style::default();
        source.span_style = ratatui::style::Style::default();
        source.range = source.range.start + retained.len()..source.range.end + retained.len();
        if index == 0 {
            source.range.start = 0;
            source.prefix_bytes -= retained.len();
        }
    }
}

pub(crate) fn adaptive_wrap_hyperlink_lines(
    lines: &[HyperlinkLine],
    options: RtOptions<'static>,
) -> Vec<HyperlinkLine> {
    let mut out = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let options = if index == 0 {
            options.clone()
        } else {
            options
                .clone()
                .initial_indent(options.subsequent_indent.clone())
        };
        let mut source = line.clone();
        source
            .source
            .get_or_insert_with(|| LogicalLineSource::from_line(&line.line))
            .continuation_indent = options.subsequent_indent.clone();
        out.extend(remap_source_wrapped_line(
            &source,
            crate::wrapping::adaptive_wrap_line_with_source(&line.line, options),
        ));
    }
    out
}

pub(crate) fn annotate_web_urls(lines: Vec<Line<'static>>) -> Vec<HyperlinkLine> {
    lines.into_iter().map(annotate_web_urls_in_line).collect()
}

pub(crate) fn annotate_web_urls_in_line(line: Line<'static>) -> HyperlinkLine {
    let text = line
        .spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect::<String>();
    let mut out = HyperlinkLine::new(line);
    out.hyperlinks = web_links_in_text(&text);
    out
}

/// Project annotations from the exact source slices used by the existing wrapping algorithm.
pub(crate) fn remap_source_wrapped_line(
    source: &HyperlinkLine,
    wrapped: Vec<crate::wrapping::WrappedLine<'_>>,
) -> Vec<HyperlinkLine> {
    let text = line_text(&source.line);
    let mut logical = source
        .source
        .clone()
        .unwrap_or_else(|| LogicalLineSource::from_line(&source.line));
    // Wrapping bakes the row style into each span; mirror it once for all source fragments.
    if logical.line_style != ratatui::style::Style::default() {
        logical.styles = logical
            .styles
            .iter()
            .map(|(range, style)| (range.clone(), logical.line_style.patch(*style)))
            .collect::<Vec<_>>()
            .into();
    }
    let mut source_byte = 0;
    let mut source_column = 0;
    wrapped
        .into_iter()
        .map(|wrapped| {
            let line = line_to_static(&wrapped.line);
            let displayed = line_text(&line);
            let prefix_columns = display_width(&displayed[..wrapped.prefix_bytes]);
            // Some wrapping paths retain a one-past-the-end cursor sentinel. Span slicing already
            // clips that sentinel to the visible line; keep hyperlink and source projection on the
            // same bounded range instead of indexing past the flattened text.
            let range = wrapped.range.start.min(text.len())..wrapped.range.end.min(text.len());
            source_column += display_width(&text[source_byte..range.start]);
            let start = source_column;
            let end = start + display_width(&text[range.clone()]);
            source_byte = range.end;
            source_column = end;
            let hyperlinks =
                project_hyperlink_columns(&source.hyperlinks, start..end, prefix_columns);
            HyperlinkLine {
                line,
                hyperlinks,
                source: Some(logical.wrapped(range, wrapped.prefix_bytes)),
            }
        })
        .collect()
}

/// Translate links intersecting a source fragment into columns on the displayed row.
pub(crate) fn project_hyperlink_columns(
    links: &[TerminalHyperlink],
    source: Range<usize>,
    prefix: usize,
) -> Vec<TerminalHyperlink> {
    links
        .iter()
        .filter_map(|link| {
            let first = link.columns.start.max(source.start);
            let last = link.columns.end.min(source.end);
            (first < last).then(|| {
                link.with_columns(prefix + first - source.start..prefix + last - source.start)
            })
        })
        .collect()
}

/// Re-attach source hyperlink ranges after visible-text wrapping has split a line.
///
/// Link text is matched in display order so a URL split across table rows retains the complete
/// destination on every rendered fragment. Whitespace inserted or removed at line boundaries is
/// ignored while matching; hyperlink destinations themselves are never reconstructed from output.
/// This legacy projection does not infer logical source provenance. New wrapping callers should
/// pass authoritative ranges to [`remap_source_wrapped_line`].
pub(crate) fn remap_wrapped_line(
    source: &HyperlinkLine,
    wrapped: Vec<Line<'static>>,
) -> Vec<HyperlinkLine> {
    let mut out = plain_hyperlink_lines(wrapped);
    if source.hyperlinks.is_empty() && source.source.is_none() {
        return out;
    }
    let source_text = line_text(&source.line);
    if source_text.trim().is_empty() {
        for line in &mut out {
            line.source = source.source.clone();
        }
        return out;
    }
    let mut source_byte = 0usize;
    let mut source_column = 0usize;
    let mut link_index = 0usize;
    for (index, line) in out.iter_mut().enumerate() {
        if index > 0 {
            let trimmed = source_text[source_byte..].trim_start_matches(char::is_whitespace);
            let skipped = source_text[source_byte..].len() - trimmed.len();
            source_column += display_width(&source_text[source_byte..source_byte + skipped]);
            source_byte += skipped;
        }

        let rendered = line_text(&line.line);
        let remaining = &source_text[source_byte..];
        let Some(rendered_start) = longest_suffix_matching_prefix(&rendered, remaining) else {
            continue;
        };
        let mapped = &rendered[rendered_start..];
        line.source = source
            .source
            .as_ref()
            .map(|source| source.wrapped(source_byte..source_byte + mapped.len(), rendered_start));
        let mut output_column = display_width(&rendered[..rendered_start]);
        for grapheme in mapped.graphemes(/*is_extended*/ true) {
            let width = display_width(grapheme);
            while source
                .hyperlinks
                .get(link_index)
                .is_some_and(|link| link.columns.end <= source_column)
            {
                link_index += 1;
            }
            if let Some(link) = source
                .hyperlinks
                .get(link_index)
                .filter(|link| link.columns.contains(&source_column))
            {
                push_link_range(line, output_column..output_column + width, link);
            }
            source_column += width;
            output_column += width;
        }
        source_byte += mapped.len();
    }
    out
}

fn line_text(line: &Line<'_>) -> String {
    line.spans
        .iter()
        .map(|span| span.content.as_ref())
        .collect()
}

fn longest_suffix_matching_prefix(rendered: &str, source: &str) -> Option<usize> {
    rendered
        .grapheme_indices(/*is_extended*/ true)
        .map(|(index, _)| index)
        .chain(std::iter::once(rendered.len()))
        .find(|index| source.starts_with(&rendered[*index..]) && *index < rendered.len())
}

fn push_link_range(line: &mut HyperlinkLine, range: Range<usize>, link: &TerminalHyperlink) {
    if range.is_empty() {
        return;
    }
    if let Some(previous) = line.hyperlinks.last_mut()
        && previous.destination == link.destination
        && previous.destination_kind == link.destination_kind
        && previous.markdown_source == link.markdown_source
        && previous.columns.end == range.start
    {
        previous.columns.end = range.end;
        return;
    }
    line.hyperlinks.push(link.with_columns(range));
}

pub(crate) fn web_links_in_text(text: &str) -> Vec<TerminalHyperlink> {
    let mut links = Vec::new();
    let mut search_from = 0usize;
    let mut source_byte = 0usize;
    let mut source_column = 0usize;
    for raw_token in text.split_whitespace() {
        let Some(relative_start) = text[search_from..].find(raw_token) else {
            continue;
        };
        let raw_start = search_from + relative_start;
        search_from = raw_start + raw_token.len();
        let trimmed_start = raw_token
            .find(|ch: char| !is_leading_punctuation(ch))
            .unwrap_or(raw_token.len());
        let trimmed_end = trailing_url_end(&raw_token[trimmed_start..]) + trimmed_start;
        if trimmed_start >= trimmed_end {
            continue;
        }
        let candidate = &raw_token[trimmed_start..trimmed_end];
        let Some(destination) = web_destination(candidate) else {
            continue;
        };
        let candidate_start = raw_start + trimmed_start;
        // Measure disjoint prefixes so scanning a draft with many URLs stays linear.
        source_column += display_width(&text[source_byte..candidate_start]);
        source_byte = candidate_start;
        let end = source_column + display_width(candidate);
        links.push(TerminalHyperlink::web(source_column..end, destination));
    }
    links
}

fn is_leading_punctuation(ch: char) -> bool {
    matches!(
        ch,
        '(' | ')' | '[' | ']' | '{' | '}' | '<' | '>' | ',' | '.' | ';' | '!' | '\'' | '"'
    )
}

fn trailing_url_end(candidate: &str) -> usize {
    // Count delimiter balances once rather than rescanning for every trailing closer.
    let mut balances = [0isize; 4];
    for ch in candidate.chars() {
        match ch {
            '(' => balances[0] += 1,
            ')' => balances[0] -= 1,
            '[' => balances[1] += 1,
            ']' => balances[1] -= 1,
            '{' => balances[2] += 1,
            '}' => balances[2] -= 1,
            '<' => balances[3] += 1,
            '>' => balances[3] -= 1,
            _ => {}
        }
    }
    let mut end = candidate.len();
    while end > 0 {
        let remaining = &candidate[..end];
        let Some(ch) = remaining.chars().next_back() else {
            break;
        };
        let balance = match ch {
            ')' => Some(&mut balances[0]),
            ']' => Some(&mut balances[1]),
            '}' => Some(&mut balances[2]),
            '>' => Some(&mut balances[3]),
            _ => None,
        };
        let trim = if let Some(balance) = balance {
            let unmatched = *balance < 0;
            *balance += 1;
            unmatched
        } else {
            matches!(ch, ',' | '.' | ';' | '!' | '\'' | '"')
                || ch == '?'
                    && match remaining.chars().rev().nth(1) {
                        Some(')') => balances[0] < 0,
                        Some(']') => balances[1] < 0,
                        Some('}') => balances[2] < 0,
                        Some('>') => balances[3] < 0,
                        _ => false,
                    }
        };
        if !trim {
            break;
        }
        end -= ch.len_utf8();
    }
    end
}

pub(crate) fn markdown_destination(destination: &str) -> Option<String> {
    web_destination(destination)
}

pub(crate) fn web_destination(destination: &str) -> Option<String> {
    let safe_destination = sanitized_destination(destination)?;
    let parsed = Url::parse(&safe_destination).ok()?;
    matches!(parsed.scheme(), "http" | "https")
        .then(|| parsed.host_str())
        .flatten()?;
    Some(safe_destination)
}

fn sanitized_destination(destination: &str) -> Option<String> {
    if destination.len() > MAX_HYPERLINK_DESTINATION_BYTES {
        return None;
    }
    Some(destination.chars().filter(|ch| !ch.is_control()).collect())
}
