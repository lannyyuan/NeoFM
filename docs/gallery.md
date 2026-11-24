# NeoFM — screenshot gallery (R3)

Six real macOS captures of NeoFM 0.2.1 on Apple Silicon, iTerm2 3.7.9, Graphite theme, Menlo 21 pt, 1:2:4 panes. Showcase versions add only an external graphite surround, padding and shadow. The original captures are preserved. No simulated application UI.

| Scene | Original | Showcase |
|---|---|---|
| Markdown | [PNG](../assets/screenshots-r3/raw/01-markdown.png) | [PNG](../assets/screenshots-r3/showcase/01-markdown.png) |
| Mathematics and Mermaid | [PNG](../assets/screenshots-r3/raw/02-math-mermaid.png) | [PNG](../assets/screenshots-r3/showcase/02-math-mermaid.png) |
| Sixel image | [PNG](../assets/screenshots-r3/raw/03-image.png) | [PNG](../assets/screenshots-r3/showcase/03-image.png) |
| Markdown code preview | [PNG](../assets/screenshots-r3/raw/04-source-code.png) | [PNG](../assets/screenshots-r3/showcase/04-source-code.png) |
| HTML Quick Look | [PNG](../assets/screenshots-r3/raw/05-quicklook-html.png) | [PNG](../assets/screenshots-r3/showcase/05-quicklook-html.png) |
| PDF Quick Look | [PNG](../assets/screenshots-r3/raw/06-quicklook-pdf.png) | [PNG](../assets/screenshots-r3/showcase/06-quicklook-pdf.png) |

[Final image dimensions and SHA-256 checksums](../assets/screenshots-r3/published-manifest.json).

## Capture scope

- S01–S04: direct iTerm2 window captures. S03 is actual Sixel, capped at 384 × 288 by the current renderer. S04 shows syntax highlighting in a Markdown fenced Python block, not standalone Python-file highlighting.
- S05–S06: single live macOS captures clipped to the terminal frame, showing the selected file and Apple Quick Look together; no separate windows composited. A session-only background mapping kept the file selection visible. The Apple qlmanage diagnostic [DEBUG] title is deliberately retained.
- S06 shows physical PDF page 25 (Belcher Islands), facing the printed page 14 entry.
- These are macOS demonstrations, not screenshots of Windows or Linux. No application code was changed to produce them.

## Image credits

- S03: Bombetoka Bay, NASA, [Earth as Art](https://www.nasa.gov/ebooks/earth-as-art/) (2012), PDF page 29. Image-page derivative cropped before opening it in NeoFM.
- S05: [Blue Marble 2002](https://svs.gsfc.nasa.gov/30614/), NASA Goddard Scientific Visualization Studio; embedded in an independent local HTML demonstration.
- S06: Belcher Islands, NASA, [Earth as Art](https://www.nasa.gov/ebooks/earth-as-art/) (2012), PDF page 25.
- NASA [images and media guidance](https://www.nasa.gov/nasa-brand-center/images-and-media/). No affiliation, sponsorship or endorsement is implied. The gallery does not redistribute the original book PDF.

All final files were checked against the R3 capture manifest prior to publication. Intermediate theme studies, candidates and demo files remain local.
