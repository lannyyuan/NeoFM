# Validation record

## Prior deployed v0.2.1

Native applications were installed on Mac M4, Windows x64, Debian 13 NUC, and CachyOS x64/DWM. All four passed startup, hidden-file configuration inspection and direct Markdown smoke tests. Earlier tests included real lf PTY/ConPTY navigation and QuickLook windows, with Linux DWM tested on the target VM. Arch installation used an isolated Dzurkel runtime because system Qt was older than the official Arch package required. These results refer to the deployed builds, not automatically to every future source commit.

- Mac: adapter 19/19; native Quick Look process observed; file-operation test passed except independent Trash enumeration restricted by macOS permissions.
- Windows: real ConPTY file-operation assertions 9/9; recycle namespace item observed. PowerShell 5.1, 7.7 and CMD passed QuickLook route in controlled shells. Actual terminal-specific profile may differ.
- Linux target CachyOS: renderer 4/4, file-operation/Trash 10/10, DWM preview route/window 4/4, hidden-directory navigation 5/5.
- Adapted Codex Writer upstream-derived suite: 325 passed / 3 failed (directive semantic copy and two palette/link snapshots); not full upstream parity. Snapshot candidates were not accepted.
- Physical terminal image fidelity, real SSH high-resolution graphics, focus recovery, broader file formats and Windows CMD path edge cases remain owner UAT or future work.

## Unified repository

See the latest commit and the integration test results recorded below. The existing binaries on the four hosts have not been replaced by this consolidation alone.

To rerun:

    cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
    INSTA_UPDATE=no cargo test --locked --manifest-path writer-poc/Cargo.toml --lib
    python3 tests/smoke.py dist/neofm-macos-arm64/bin/lfpreview

The second command is expected to retain the three known host-adaptation failures until their semantics are resolved. Do not update snapshots just to make it green.
