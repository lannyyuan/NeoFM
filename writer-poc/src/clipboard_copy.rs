//! Copy annotation formats without a clipboard backend.

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum CopyFormat {
    PlainText,
    Markdown,
}
