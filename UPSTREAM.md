# Source provenance

NeoFM is an independent integration, not an official lf or OpenAI product.

| Component | Source | Pin | License |
|---|---|---|---|
| File manager | [gokcehan/lf](https://github.com/gokcehan/lf) | 8b8780299a834a589c4bcfdc54e59164bef89c8b | MIT |
| Extracted Markdown Writer, text/image utilities and Mermaid | [openai/codex](https://github.com/openai/codex) | a0d5b0a661db3b34cd87bdf1c93736b90fd3694b | Apache-2.0; preserve NOTICE |
| Terminal rendering dependencies | Cargo.lock / go.sum | Locked versions | See each dependency's license |
| Linux large preview | [Dzurkel](https://github.com/EdinUser/dzurkel), separately installed | Tested v1.0.0 | GPL-3.0-or-later |
| Windows large preview | [QuickLook](https://github.com/QL-Win/QuickLook), separately installed | Tested v4.0.2 | See upstream |

The lf source is an unmodified pinned Git archive (text line endings can be normalized by Git). No nested repository history is fabricated. Codex extraction is a modified standalone renderer; host-specific runtime APIs, palette and link adapters differ from upstream. Original licenses and notices are retained. No Codex authentication, model access, AI harness, app server or API keys are needed. Dzurkel, Qt and QuickLook binaries are **not** included in this repository. An optional Linux wrapper supports a separately provisioned compatible isolated Dzurkel runtime.
