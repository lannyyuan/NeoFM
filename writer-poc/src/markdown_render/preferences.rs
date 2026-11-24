//! Fixed preferences for the standalone completed-message renderer.

pub(crate) struct Rendering {
    pub(crate) lists: bool,
    pub(crate) tables: bool,
    pub(crate) mermaid: bool,
    pub(crate) math: bool,
}

pub(crate) fn current() -> Rendering {
    Rendering { lists: true, tables: true, mermaid: true, math: true }
}

pub(crate) fn hide_transcript_destination(_: &str) -> bool {
    false
}
