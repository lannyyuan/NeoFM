![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>Moderner Dateimanager für das Terminal.</strong><br>Navigieren wie in Vim. Lesen wie in einem modernen Dokumentbetrachter.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · **Deutsch** · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
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

## Vielseitige Vorschauen über SSH — kein Desktop nötig

NeoFM ist zugleich ein lokaler Dateimanager und ein entfernter Dokumentbetrachter. Auf einem Linux-Server über normales SSH bleiben Navigation, Dateioperationen und die meisten alltäglichen Seitenvorschauen ohne X11-Weiterleitung, Remote-Desktop oder Dateidownload verfügbar. Der Renderer läuft auf dem entfernten Rechner; das Terminal zeigt seine Ausgabe.

| Format | Live-Vorschau über SSH |
| :-- | :-- |
| Markdown: Überschriften, Listen, Tabellen, CJK, Codeblöcke, Formeln, Mermaid | Formatierte Darstellung |
| UTF-8-Text, Quellcode, JSON, Logs, HTML-Quelltext | Textvorschau |
| PNG, JPEG, erstes GIF-Bild, WebP | Sixel oder ANSI-Farbblöcke innerhalb der Rendergrenzen |
| PDF, Office, Medien und Archive | Typ/Größe oder binäre Metadaten, keine vollständige Darstellung |

Unter Linux-SSH oder ohne grafische Oberfläche öffnet Space den Terminal-Reader (less -R -X, falls verfügbar): formatiertes Markdown, begrenzten Text oder Metadaten. Bilder werden in der Seitenleiste gerendert, nicht vergrößert im Reader. Vollständiges PDF/Office, gerendertes HTML und Multimedia-Quick-Look benötigen geeignete lokale GUI-Programme. macOS und Windows behalten derzeit ihre eigenen Space-Belegungen; der automatische SSH-Reader ist hier der Linux-Pfad. Bilderkennung hängt von Terminal und weitergegebener Umgebung ab; tmux/screen nutzen ANSI. Sixel nur bei Kompatibilität erzwingen. Ausgabe ist begrenzt; hochauflösende SSH-Bildqualität ist noch nicht vollständig visuell geprüft.

Über eine echte Linux-SSH-Verbindung geprüft: Markdown, Formeln/Mermaid, Sixel-Frame-Erzeugung, ANSI-Bilder und Terminal-Reader-Routing (siehe Validierung). [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

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

Go 1.26+ und Rust/Cargo 1.94+ werden auf der Zielplattform benötigt; Windows zusätzlich MSVC-Build-Tools. Beim ersten Build werden die festgeschriebenen Abhängigkeiten geladen. Vorgefertigte Pakete sind unter [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest) verfügbar; das Quellcode-Repository ist öffentlich.

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
.\dist\NeoFM-win-x64\neofm.exe
~~~

**QuickLook konfigurieren:** Standardpfad ist %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe. Andere Installationsarten können abweichen. In PowerShell den vollständigen EXE-Pfad setzen, prüfen und bei Bedarf ersetzen.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

Entsprechung für CMD:

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**Wichtig:** immer neofm.exe statt lf.exe starten. Der Starter ergänzt System32, ComSpec und PATHEXT im eigenen Prozess und ändert den globalen PATH nicht. Auch aus PowerShell 5.1 oder 7.x bleibt die interne lf-Shell CMD. Markiert Space eine Datei statt der Vorschau, wurde möglicherweise die benutzerdefinierte lfrc nicht geladen. Bei 'exec: "cmd": executable file not found in %PATH%' über den Starter neu starten und System32 prüfen. Zuerst eine Test-PDF direkt in QuickLook öffnen, dann Space bei derselben Datei testen. Diagnoseprotokoll und ANSI-Fallback:

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd verwendet Windows PowerShell 5.1 und die native Shell-API; mit einer entbehrlichen Datei auf einem lokalen Festlaufwerk testen. QuickLook-Formatplugins, Codecs und Terminalbildqualität sind vom Kern getrennt. Der verallgemeinerte Pfad benötigt noch einen vollständigen GUI-Regressionstest.

## Tastenbelegung

### Navigation und Vorschau

| Taste | Aktion |
| :-- | :-- |
| `↑ / k · ↓ / j` | Nach oben/unten |
| `← / h` | Übergeordnetes Verzeichnis |
| `→ / l · Enter` | Ordner betreten; Dateimarkierung umschalten |
| `Home / gg · End / G` | Erster/letzter Eintrag |
| `Ctrl+U / Ctrl+D` | Halbe Seite hoch/runter |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | Seite hoch/runter |
| `Ctrl+Y / Ctrl+E` | Eine Zeile hoch/runter scrollen |
| `H / M / L` | Oberer/mittlerer/unterer Bildschirmbereich |
| `gh` | Home-Verzeichnis |
| `[ / ]` | Vorheriger/nächster Verzeichnisverlauf |
| `Space` | Große Vorschau / Terminal-Reader |
| `o` | Extern öffnen |

### Auswahl und Dateioperationen

| Taste | Aktion |
| :-- | :-- |
| `Ctrl+Space` | Aktuelle Markierung umschalten |
| `u` | Alle Markierungen in allen Ordnern aufheben |
| `v` | Auswahl im aktuellen Ordner umkehren |
| `V` | Bereichsauswahl starten |
| `yy` | In NeoFM-Puffer kopieren |
| `x` | In NeoFM-Puffer ausschneiden |
| `p` | Im aktuellen Ordner einfügen |
| `c` | Dateipuffer leeren |
| `dd` | Bestätigen und in den Papierkorb |
| `r` | Umbenennen |
| `t` | Dateitag umschalten |

### Suche und Verlauf

| Taste | Aktion |
| :-- | :-- |
| `/ / ?` | Dateinamen vorwärts/rückwärts suchen |
| `n / N` | Nächster/vorheriger Suchtreffer |
| `f / F` | Schnellsuche vorwärts/rückwärts |
| `; / ,` | Nächster/vorheriger Schnellsuchtreffer |
| `m / ' / "` | Ordnerlesezeichen speichern/laden/löschen |

### Sortierung und Anzeige

| Taste | Aktion |
| :-- | :-- |
| `zh` | Versteckte Einträge umschalten |
| `zr` | Sortierung umkehren |
| `zn / zs / zt / za` | Info aus / Größe / Zeit / beides |
| `sn / ss / st` | Nach Name / Größe / Änderungszeit sortieren |
| `sa / sb / sc / se` | Nach Zugriff / Erzeugung / Statusänderung / Endung sortieren |
| `Ctrl+L / Ctrl+R` | Neu zeichnen / neu laden |

### Befehle und Modi

| Taste | Aktion |
| :-- | :-- |
| `:` | Befehlseingabe |
| `$ / % / ! / &` | Shell / Pipe / Warten / asynchron |
| `Ctrl+P / Ctrl+N` | Vorheriger/nächster Befehlsverlauf |
| `q` | Beenden |

Standardtasten im Normalmodus, Groß-/Kleinschreibung beachten. Mehrere Einträge mit Enter oder Ctrl+Space markieren, yy (Kopieren) oder x (Ausschneiden), zum Ziel navigieren und p drücken. Ohne Markierung wird die aktuelle Datei genutzt. Kopieren/Ausschneiden hebt Markierungen auf, behält aber den internen Dateipuffer. u hebt Markierungen auf, c leert den Puffer. yy und dd sind aufeinanderfolgende Tasten. dd verlangt Bestätigung und nutzt wiederherstellbares Löschen ohne beabsichtigten Wechsel zur endgültigen Löschung. Space und o hängen von Plattform und Sitzung ab.

Visual Mode: V startet einen Bereich, Bewegung erweitert ihn, V übernimmt, Esc verwirft, o tauscht die Endpunkte. Die geerbten y/d-Tasten bleiben im Visual Mode erhalten, anders als yy/x im Normalmodus. Erst mit V übernehmen, dann yy/x/dd normal verwenden. Befehlszeile: Enter ausführen, Esc abbrechen, Tab vervollständigen, Pfeile hoch/runter für Verlauf, links/rechts und Home/End für Cursor. Vollständige geerbte Bearbeitungstasten und Befehle: neofm -doc oder [lf documentation](lf-upstream/doc.txt).

Mausradbelegungen sind geerbt, Mausunterstützung ist standardmäßig aus.

## Architektur und Validierung

~~~text
                  NeoFM launcher
                        │
                   lf (Go)
             ┌──────────┴───────────┐
             │                      │
    neofm-preview (Rust)               Space
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
