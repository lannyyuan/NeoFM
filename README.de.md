<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>Moderner Dateimanager für das Terminal.</strong><br>Navigieren wie in Vim. Lesen wie in einem modernen Dokumentbetrachter.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · **Deutsch** · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM verbindet den bewährten Dateimanager **lf** mit einem eigenständigen, aus der Codex-TUI extrahierten Rust-Renderer. So entstehen vielseitige Dokumentvorschauen in einer schlanken, tastaturorientierten Terminalumgebung. Große Vorschauen übernehmen vorhandene Werkzeuge der jeweiligen Plattform. Kein KI-Dienst, neuer Terminalemulator oder verpflichtendes Cloud-Konto.

[![Mit Space ein echtes PDF in macOS Quick Look öffnen. NASA, Earth as Art — Belcherinseln.](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*Mit Space ein echtes PDF in macOS Quick Look öffnen. NASA, Earth as Art — Belcherinseln.*

## Designphilosophie — weniger, aber besser

- **Unix KISS (Keep It Simple, Stupid):** klare Aufgaben für jede Komponente. lf übernimmt Navigation und Dateioperationen, der Renderer begrenzte Terminalausgabe, native Werkzeuge große Vorschauen. Bewährte Werkzeuge kombinieren statt neu erfinden.
- **Inspiriert von [suckless](https://suckless.org/):** kleine verständliche Komponenten, wenige Abhängigkeiten, Tastatur zuerst und bewusste Zurückhaltung. NeoFM ist kein offizielles suckless-Projekt.
- **Ästhetik durch Weglassen:** kein Startbildschirm, dekoratives Dashboard, überflüssige Schaltflächen, eingebetteter Browser oder KI-Harness. Das Logo gehört in die Projektunterlagen, nicht in den Terminalarbeitsbereich. Neue Funktionen nur bei verbessertem häufigem Arbeitsablauf.
- **Einheitliches Verhalten, native Umsetzung:** gleiche Anordnung und Vim-Tastenbelegung unter macOS, Windows und Linux. Vorschau und Papierkorb nutzen die bewährten Mechanismen des jeweiligen Systems. Lokale Dateien bleiben lokal.

## Ein Layout, drei Plattformen

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

Das Standardverhältnis ist **1:2:4**: links das übergeordnete Verzeichnis, in der Mitte das aktuelle Verzeichnis mit Auswahl, rechts die Live-Vorschau. Der Inhalt erhält den größten Bereich; kein separater Editor und keine dauerhafte Werkzeugleiste. Versteckte Einträge sind standardmäßig sichtbar; zh schaltet um. Die Starter steuern das Layout ohne neues Terminal-UI-Framework.

| Inhalt | Live-Vorschau | Space — große Vorschau |
|:--|:--|:--|
| Markdown | Überschriften, Listen, adaptive Tabellen, CJK, Codehervorhebung, Formeln, Mermaid | Kompatibler nativer Betrachter |
| Bilder | PNG, JPEG, erstes GIF-Bild, WebP; Sixel oder ANSI | Externer Großbildbetrachter |
| Text / Code | Begrenzte Textdarstellung | Nativer Betrachter / Terminal-Reader |
| PDF, Office, Medien | Metadaten und Vorschauhinweis | Abhängig von externen Formatunterstützungen |

## NeoFM in Aktion

[![Markdown-Live-Vorschau — Überschriften, Listen und adaptive Tabellen.](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Markdown-Live-Vorschau — Überschriften, Listen und adaptive Tabellen.*

[![Mathematik und Mermaid — gesetzte Brüche und Flussdiagramme im Terminal.](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*Mathematik und Mermaid — gesetzte Brüche und Flussdiagramme im Terminal.*

[![HTML Quick Look — eine lokale HTML-Seite im nativen Vorschaufenster.](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*HTML Quick Look — eine lokale HTML-Seite im nativen Vorschaufenster.*

[![Sixel-Bilder — echte Terminalgrafik, begrenzt durch das Pixelbudget des Renderers.](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Sixel-Bilder — echte Terminalgrafik, begrenzt durch das Pixelbudget des Renderers.*

[![Markdown-Code — Syntaxhervorhebung eines Python-Codeblocks, nicht einer eigenständigen Quelldatei.](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Markdown-Code — Syntaxhervorhebung eines Python-Codeblocks, nicht einer eigenständigen Quelldatei.*

Echte macOS-Aufnahmen; Graphite / Menlo. NASA-Bildnachweise: [Aufnahmehinweise und Bildnachweise](docs/gallery.md).

## Empfohlene vollständige Konfiguration

| Plattform | Terminal / Desktop | Space | Papierkorb |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/) (Sixel) oder Terminal (ANSI) | Integriertes Quick Look | Systempapierkorb |
| Windows x64 | Aktuelles Sixel-fähiges [Windows Terminal](https://github.com/microsoft/terminal); PowerShell oder CMD | Separates [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows-Papierkorb |
| Linux x64 Desktop | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) oder anderes Terminal; Sixel für Rasterbilder | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux Headless / SSH | Vorhandenes Terminal; less empfohlen | Text / Metadaten | gio / trash-put |

Das sind Empfehlungen, keine vorgeschriebenen Desktopumgebungen. Der Kern benötigt kein Qt. Ohne Sixel oder in tmux/screen wird ANSI genutzt. LFP_IMAGE_BACKEND unterstützt auto, ansi, sixel, none; sixel nur auf kompatiblen Terminals erzwingen.

## Bauen und starten

Go 1.26+ und Rust/Cargo 1.94+ werden auf der Zielplattform benötigt; Windows zusätzlich MSVC-Build-Tools. Beim ersten Build werden die festgeschriebenen Abhängigkeiten geladen. Dies ist ein privates Entwicklungsrepository, kein öffentliches Paket; Klonen erfordert autorisierten GitHub-Zugriff.

### macOS / Linux

~~~sh
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
./scripts/build.sh

# macOS / Apple Silicon
./dist/neofm-macos-arm64/bin/neofm

# Linux / x86_64
./dist/neofm-linux-x64/bin/neofm
~~~

Den Starter verwenden, nicht lf direkt. Für einen lokalen Befehl bin/neofm des gewählten Builds nach ~/.local/bin verlinken, ohne vorhandene Einträge zu überschreiben. Terminal und Fenstermanager bleiben erhalten.

**macOS:** keine zusätzliche Quick-Look-App nötig. Der Starter verwendet /usr/bin/qlmanage -p. Für Sixel ist iTerm2 empfehlenswert; das System-Terminal kann ANSI verwenden.

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux:** Dzurkel und kompatible Qt-Abhängigkeiten separat installieren. ANSI funktioniert auch mit DWM/st ohne Sixel. Folgende Werkzeuge prüfen und fehlende über den Paketmanager installieren. Dzurkel wird nur in einer lokalen grafischen Sitzung eingesetzt; normales SSH nutzt den Terminal-Fallback. Bei Qt-Inkompatibilität eine isolierte kompatible Dzurkel-Laufzeit einrichten, statt den gesamten Desktop zu aktualisieren.

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — vollständige Einrichtung

Sixel-fähiges Windows Terminal, [QuickLook](https://github.com/QL-Win/QuickLook) und die genannten nativen Build-Tools installieren. QuickLook ist eine Drittanbieter-App, kein integrierter Windows-Bestandteil. Nach der Installation einmal in der Desktop-Sitzung starten und dann NeoFM bauen.

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**QuickLook konfigurieren:** Standardpfad ist %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe. Andere Installationsarten können abweichen. In PowerShell den vollständigen EXE-Pfad setzen, prüfen und bei Bedarf ersetzen.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.cmd
~~~

Entsprechung für CMD:

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**Wichtig:** immer neofm.cmd statt lf.exe starten. Der Starter ergänzt System32, ComSpec und PATHEXT im eigenen Prozess und ändert den globalen PATH nicht. Auch aus PowerShell 5.1 oder 7.x bleibt die interne lf-Shell CMD. Markiert Space eine Datei statt der Vorschau, wurde möglicherweise die benutzerdefinierte lfrc nicht geladen. Bei 'exec: "cmd": executable file not found in %PATH%' über den Starter neu starten und System32 prüfen. Zuerst eine Test-PDF direkt in QuickLook öffnen, dann Space bei derselben Datei testen. Diagnoseprotokoll und ANSI-Fallback:

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.cmd
~~~

dd verwendet Windows PowerShell 5.1 und die native Shell-API; mit einer entbehrlichen Datei auf einem lokalen Festlaufwerk testen. QuickLook-Formatplugins, Codecs und Terminalbildqualität sind vom Kern getrennt. Der verallgemeinerte Pfad benötigt noch einen vollständigen GUI-Regressionstest.

## Tastenbelegung

| Taste | Aktion | Taste | Aktion |
|:--|:--|:--|:--|
| `h / j / k / l` | Navigieren | `Space` | Quick Look |
| `Enter` | Datei markieren/entmarkieren; Ordner betreten | `Ctrl+Space` | Markierung umschalten |
| `yy` | Kopieren | `p` | Einfügen |
| `x` | Ausschneiden | `dd` | Bestätigen, dann Papierkorb |
| `o` | Extern öffnen | `zh` | Versteckte Einträge umschalten |
| `gg / G` | Erster / letzter Eintrag | `/` | Suchen |
| `r` | Umbenennen | `q` | Beenden |

Markierte Dateien haben Vorrang, sonst gilt die aktuelle Datei. dd verlangt Bestätigung und wechselt bei nicht verfügbarem Papierkorb nicht absichtlich zur endgültigen Löschung.

## Architektur und Validierung

~~~text
                  NeoFM launcher
                        │
                   lf (Go)
             ┌──────────┴───────────┐
             │                      │
    lfpreview (Rust)               Space
       ┌─────┴─────┐                │
   Markdown      Images       Native preview
   Math/Mermaid  Sixel/ANSI   / terminal fallback
~~~

Ein gemeinsamer Quellbaum, plattformspezifische Starter. Siehe [Architektur](docs/architecture.md), [Validierung](docs/validation.md) und [Quellherkunft](UPSTREAM.md).

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

Die vereinheitlichten nativen Rust-Release-Builds auf Mac ARM64, Windows x64 und Linux x64 waren erfolgreich, jeweils mit vier Renderer-Smoke-Tests; Mac-Adapter 19/19. Frühere Installationen prüften native Vorschau und Papierkorb. Der angepasste Writer enthält drei bekannte Fehler in upstream-abgeleiteten Tests. Vollständige Upstream-Parität, vollständiger GUI-Retest, interaktive Sidebar-Tabs und komplette PDF/Office- oder hochauflösende Grafikdarstellung über Headless-SSH werden nicht beansprucht.

## Danksagung und Lizenz

Basiert auf [lf](https://github.com/gokcehan/lf) (MIT) und extrahierten Komponenten aus [OpenAI Codex](https://github.com/openai/codex) (Apache-2.0). QuickLook und Dzurkel werden separat installiert, nicht mitgeliefert. NeoFM ist eigenständig. Siehe [LICENSE](LICENSE), [NOTICE](NOTICE), [lf-Lizenz](lf-upstream/LICENSE), [UPSTREAM.md](UPSTREAM.md) und [Markenmaterial](assets/brand/).

<p align="center"><sub>Kleine Werkzeuge. Klare Dateien. Native Vorschauen.</sub></p>
