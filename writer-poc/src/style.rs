//! Standalone style policy for the Writer, without terminal color probing.

use ratatui::style::{Color, Style};

pub(crate) fn accent_color() -> Color {
    Color::Rgb(99, 168, 248)
}

pub(crate) fn table_separator_style() -> Style {
    Style::default().dim()
}

pub(crate) fn readable_color_on(preferred: Color, _background: Option<Color>) -> Color {
    preferred
}
