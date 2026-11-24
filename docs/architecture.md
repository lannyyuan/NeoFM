# Architecture

NeoFM preserves the lightweight lf navigation engine and adds an independent renderer. It is not a new terminal emulator or a full Codex TUI port.

    Terminal
       |
    NeoFM launcher (platform config + isolated lf data)
       |
    lf (Go) ───── six-argument previewer ───── lfpreview (Rust)
       |                                       |
       |                                       +-- extracted Codex Markdown Writer
       |                                       +-- bounded text / ANSI rendering
       |                                       +-- image decode / Sixel / ANSI
       |
       +-- Space ── native Quick Look / QuickLook / Dzurkel
       |            or terminal fallback over SSH
       |
       +-- dd ──── native recoverable-delete adapter

## Source integration

The Mac extraction is the baseline. NUC source matched it exactly, except for absent unaccepted local test snapshot candidates. The Windows copy contributed three modified Rust files: image_preview.rs (Windows Terminal route), main.rs (selected-path handling), and quicklook.rs (Windows backend). Those were imported after normalized-line-ending SHA256 comparison. No unrelated Git history was merged. The platform layer preserves independently tested behavior: shell discovery, QuickLook path, safe-delete adapters, hidden-file default, platform-specific preview routes.

Build artifacts and device-specific runtime folders remain outside Git. The initial import has two identifiable commits (baseline and platform integration). Development should continue on this single repository; other hosts build from it instead of independently editing source copies.

## Boundaries

- The Markdown preview is viewport-limited. It does not embed interactive Codex sidebar widgets.
- Common raster formats PNG, JPEG, GIF (static frame), WebP use bounded decoding. Automatic terminal detection is conservative; multiplexers use ANSI.
- The terminal fallback is primarily text/metadata, not a full PDF or Office viewer.
- Windows external preview uses a configured QuickLook executable path. The portable profile generalizes the tested local path with LFP_QUICKLOOK_EXE.
- Linux desktop requires a working Dzurkel and compatible Qt. Isolated-runtime provisioning is separate; no system Qt update is required by the core.
- dd asks for confirmation, uses recoverable-delete APIs, and has no intentional permanent-delete command fallback. OS-native limitations still apply. No delete tests on user files.
