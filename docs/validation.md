# Validation record

## Prior deployed v0.2.1

Native applications were installed on Mac M4, Windows x64, Debian 13 NUC, and CachyOS x64/DWM. All four passed startup, hidden-file configuration inspection and direct Markdown smoke tests. Earlier tests included real lf PTY/ConPTY navigation and QuickLook windows, with Linux DWM tested on the target VM. Arch installation used an isolated Dzurkel runtime because system Qt was older than the official Arch package required. These results refer to the deployed builds, not automatically to every future source commit.

- Mac: adapter 19/19; native Quick Look process observed; file-operation test passed except independent Trash enumeration restricted by macOS permissions.
- Windows: real ConPTY file-operation assertions 9/9; recycle namespace item observed. PowerShell 5.1, 7.7 and CMD passed QuickLook route in controlled shells. Actual terminal-specific profile may differ.
- Linux target CachyOS: renderer 4/4, file-operation/Trash 10/10, DWM preview route/window 4/4, hidden-directory navigation 5/5.
- Adapted Codex Writer upstream-derived suite: 325 passed / 3 failed (directive semantic copy and two palette/link snapshots); not full upstream parity. Snapshot candidates were not accepted.
- Physical terminal image fidelity, real SSH high-resolution graphics, focus recovery, broader file formats and Windows CMD path edge cases remain owner UAT or future work.

## Unified repository

Tested source commit: a64669cea7844d1c3bfa817eb9243ac82bb7182f (the three-platform consolidation). Actual native offline Rust release builds on Mac M4 arm64, Windows x64 MSVC, and Debian NUC x86_64 succeeded. All used the same source snapshot and locked dependencies.

| Host | Native release renderer build | Smoke tests |
|---|---|---|
| macOS ARM64 | PASS | 4/4 |
| Windows x64 | PASS | 4/4 |
| Linux x86_64 (NUC) | PASS | 4/4 |

The four checks per host exercise the real compiled binary: Markdown table, math/Mermaid, Sixel frame, ANSI image fallback. The merged adapter unit tests ran on Mac: 19/19. No full upstream Writer parity claim. The existing binaries on the four hosts have not been replaced by this consolidation alone. The new full Go + Rust packaging scripts and portable Windows QuickLook path generalization have not received a complete end-to-end GUI retest.

To rerun:

    cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
    INSTA_UPDATE=no cargo test --locked --manifest-path writer-poc/Cargo.toml --lib
    python3 tests/smoke.py dist/neofm-macos-arm64/bin/neofm-preview

The second command is expected to retain the three known host-adaptation failures until their semantics are resolved. Do not update snapshots just to make it green.

## v0.3.0 native entry point / packaging

The Go main executable was built on macOS for darwin/arm64, windows/amd64 and linux/amd64. The Linux binary was hash-verified on both target hosts. The existing verified renderer binaries were retained and renamed, not rebuilt. All four hosts activated v0.3.0 only after native PTY/ConPTY checks of Markdown preview, hidden/dotfile listing, recoverable-delete confirmation and cancellation, and normal exit. Windows also passed with restricted PATH and PATHEXT, and both neofm.exe and the legacy neofm.cmd returned v0.3.0. The old deployment was retained for rollback. This was a focused packaging regression, not a fresh graphical QuickLook/Dzurkel or full file-operation UAT. The gallery screenshots still document v0.2.1.

## SSH preview verification

On 2026-11-25, a real SSH connection from macOS to the deployed Linux NUC (NeoFM v0.3.0) executed the installed neofm-preview renderer against synthetic fixtures. The original renderer smoke suite passed 4/4 (Markdown/table, math/Mermaid, Sixel frame generation and ANSI fallback). Additional SSH-context checks passed 5/5 (terminal-reader Markdown output, explicit PDF metadata limitation, image metadata in the terminal reader, ANSI image output and Sixel frame generation). SSH_CONNECTION was present and less was installed. These are actual remote-process output checks; they do not constitute a new interactive terminal pixel-fidelity or Space-key GUI retest. The Linux native entry point selects the terminal-reader path in SSH/headless sessions. macOS/Windows Space bindings remain platform-specific. High-resolution Sixel display and full PDF/Office reading over SSH are not claimed.
