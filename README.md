<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N logo" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>Modern terminal file manager.</strong><br>Navigate like Vim. Read like a modern document viewer.</p>

<p align="center">
  <a href="README.zh-CN.md">简体中文</a> · English ·
  <a href="#quick-start">Quick start</a> ·
  <a href="docs/architecture.md">Architecture</a> ·
  <a href="docs/validation.md">Validation</a>
</p>

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust and Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM brings modern document previews to a small, keyboard-first terminal workspace. It pairs the proven **lf** navigation engine with an independent Rust renderer extracted from the Codex TUI, then delegates large previews to the operating system's native or separately installed preview app.

No AI service. No new terminal emulator. No mandatory cloud account. Just your files and your keyboard.

## Two levels of preview

| | **Live sidebar** | **Space — Quick Look** |
|:--|:--|:--|
| Markdown | Headings, lists, tables, code, math, Mermaid | Native preview where supported |
| Images | PNG, JPEG, GIF (first frame), WebP | External full-size viewer |
| Text and code | Bounded terminal rendering | Native viewer / terminal reader |
| PDF, Office, media | Metadata and preview hint | System or external preview app |

### Why NeoFM?

- **Vim muscle memory** — familiar navigation and simple file operations.
- **Real Markdown rendering** — adaptive tables, CJK text, code highlighting, math, and Mermaid.
- **Terminal graphics** — Sixel where supported; ANSI fallback for other terminals.
- **Recoverable deletion** — explicit confirmation before moving files to Trash / Recycle Bin.
- **Native integration** — no heavyweight GUI runtime required by the core.

## Platform support

| Platform | Native large preview | Status |
|:--|:--|:--|
| macOS ARM64 | macOS Quick Look | Deployed |
| Windows x64 | [QuickLook](https://github.com/QL-Win/QuickLook) | Deployed |
| Linux x64 desktop | [Dzurkel](https://github.com/EdinUser/dzurkel) | Deployed on CachyOS / DWM |
| Linux headless / SSH | Terminal text/metadata fallback | Deployed on Debian NUC |

Status describes previous tested installations; see [validation details](docs/validation.md). Full PDF/Office rendering over headless SSH and interactive sidebar tabs are not implemented.

## Quick start

Build from source on the target machine with **Go 1.26+** and **Rust/Cargo 1.94+**. Windows requires an MSVC toolchain. The first build downloads locked dependencies. This is a private development repository, not a public package registry release.

**macOS / Linux**

~~~sh
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
./scripts/build.sh
# macOS Apple Silicon
./dist/neofm-macos-arm64/bin/neofm
# Linux x86_64
./dist/neofm-linux-x64/bin/neofm
~~~

On Linux, install Dzurkel separately if you want graphical Quick Look. Use gio or trash-put for recoverable deletion and less for the terminal reader. The core does not require Qt. For a user-local command, symlink the chosen build's bin/neofm into ~/.local/bin (without overwriting an existing command).

**Windows (PowerShell)**

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

Install QuickLook separately. The launcher defaults to the standard user installation path; set LFP_QUICKLOOK_EXE to its full executable path if needed. The launcher supplies the Windows CMD search path without editing global environment variables.

## Keymap

| Key | Action | Key | Action |
|:--|:--|:--|:--|
| h / j / k / l | Navigate | Space | Quick Look |
| Enter | Select file / enter folder | Ctrl+Space | Toggle selection |
| yy | Copy | p | Paste |
| x | Cut | dd | Confirm and trash |
| o | External open | zh | Toggle hidden entries |
| gg / G | First / last entry | / | Search |
| r | Rename | q | Quit |

Hidden entries are shown by default. File actions target marked files when present, otherwise the current file. The native trash backend must support the operation; there is no deliberate permanent-delete fallback.

## Architecture

~~~text
                    NeoFM launcher
                           │
                    lf navigation
                    ┌──────┴──────┐
                    │             │
            Live preview          Space
                    │             │
              Rust renderer       Native Quick Look
            ┌───────┴──────┐      / terminal fallback
         Markdown       Images
         math/Mermaid    Sixel/ANSI
~~~

One shared source tree. Platform-specific launchers and native preview integrations. See [architecture](docs/architecture.md) and [source provenance](UPSTREAM.md).

## Development

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

The repository contains the pinned lf source, extracted writer, preview adapter, platform scripts, synthetic fixtures, build scripts and the NeoFM [brand assets](assets/brand/). Three known upstream-derived Writer regression failures are documented separately; do not confuse the adapter's passing tests with complete upstream parity.

## Credits & license

Built on [lf](https://github.com/gokcehan/lf) and renderer components from [OpenAI Codex](https://github.com/openai/codex), with separately installed QuickLook and Dzurkel. NeoFM is an independent project, not an official upstream product. See [LICENSE](LICENSE), [NOTICE](NOTICE), [lf's MIT license](lf-upstream/LICENSE) and [UPSTREAM.md](UPSTREAM.md).

<p align="center"><sub>Small tools. Clear files. Native previews.</sub></p>
