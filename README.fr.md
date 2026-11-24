<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>Gestionnaire de fichiers moderne pour terminal.</strong><br>Naviguez comme dans Vim. Lisez comme dans une visionneuse moderne.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · **Français** · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM associe le gestionnaire de fichiers éprouvé **lf** à un moteur de rendu Rust autonome extrait de la TUI de Codex. Il offre des aperçus riches dans un espace de travail léger, centré sur le clavier, et confie les grands aperçus aux outils de chaque plateforme. Aucun service d'IA, nouvel émulateur de terminal ou compte cloud obligatoire.

## Philosophie — moins, mais mieux

- **KISS Unix (Keep It Simple, Stupid) :** chaque composant a une responsabilité précise. lf gère la navigation et les fichiers, le moteur fournit un rendu terminal borné, les outils natifs gèrent les grands aperçus. Assembler des outils éprouvés plutôt que les réinventer.
- **Inspiré de [suckless](https://suckless.org/) :** petits composants compréhensibles, peu de dépendances, priorité au clavier et retenue délibérée. NeoFM n'est pas un projet officiel de suckless.
- **L'esthétique par soustraction :** pas d'écran de démarrage, de tableau de bord décoratif, de boutons superflus, de navigateur intégré ni de harness IA. Le logo reste dans la documentation, pas dans l'espace terminal. N'ajouter que ce qui améliore un usage fréquent.
- **Comportement cohérent, réalisation native :** mêmes disposition et raccourcis Vim sous macOS, Windows et Linux ; aperçu et corbeille utilisent les mécanismes éprouvés de chaque système. Les fichiers locaux restent locaux.

## Une disposition, trois plateformes

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

Le rapport par défaut est **1:2:4** : dossier parent à gauche, dossier courant et sélection au centre, aperçu en direct à droite. Le contenu bénéficie du plus grand espace, sans éditeur distinct ni barre d'outils permanente. Les éléments cachés sont visibles par défaut ; zh les masque ou les affiche. Les lanceurs définissent la disposition sans nouveau framework d'interface terminal.

| Contenu | Aperçu en direct | Space — grand aperçu |
|:--|:--|:--|
| Markdown | Titres, listes, tableaux adaptatifs, CJK, code coloré, formules, Mermaid | Visionneuse native compatible |
| Images | PNG, JPEG, première image GIF, WebP ; Sixel ou ANSI | Visionneuse externe grand format |
| Texte / code | Rendu texte borné | Visionneuse native / lecteur terminal |
| PDF, Office, médias | Métadonnées et indication d'aperçu | Selon les formats de l'application externe |

## Configuration complète recommandée

| Plateforme | Terminal / bureau | Space | Corbeille |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/) (Sixel) ou Terminal (ANSI) | Quick Look intégré | Corbeille système |
| Windows x64 | [Windows Terminal](https://github.com/microsoft/terminal) récent compatible Sixel ; PowerShell ou CMD | [QL-Win QuickLook](https://github.com/QL-Win/QuickLook), à installer | Corbeille Windows |
| Linux x64 graphique | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) ou autre terminal ; Sixel pour les images matricielles | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux sans interface / SSH | Terminal existant ; less conseillé | Texte / métadonnées | gio / trash-put |

Ces combinaisons sont recommandées, pas obligatoires. Le cœur ne dépend pas de Qt. Sans Sixel ou sous tmux/screen, utiliser ANSI. LFP_IMAGE_BACKEND accepte auto, ansi, sixel et none ; ne forcer sixel que sur un terminal compatible.

## Compilation et lancement

Go 1.26+ et Rust/Cargo 1.94+ sont requis sur la plateforme cible, ainsi que MSVC sous Windows. La première compilation télécharge les dépendances verrouillées. Il s'agit d'un dépôt privé de développement, non d'un paquet public : le clonage nécessite un accès GitHub autorisé.

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

Utilisez le lanceur, pas lf seul. Pour une commande locale, créez un lien vers bin/neofm de la compilation choisie dans ~/.local/bin sans écraser une entrée existante. Conservez votre terminal et gestionnaire de fenêtres.

**macOS :** aucune application Quick Look supplémentaire. Le lanceur utilise /usr/bin/qlmanage -p ; iTerm2 est conseillé pour Sixel, Terminal peut utiliser ANSI.

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux :** installez Dzurkel et ses dépendances Qt compatibles séparément. ANSI fonctionne aussi avec DWM/st sans Sixel. Vérifiez les outils suivants et installez les manquants via le gestionnaire de paquets. Dzurkel n'est utilisé que dans une session graphique locale ; SSH ordinaire utilise le repli terminal. En cas d'incompatibilité Qt, préférez un environnement Dzurkel compatible et isolé plutôt qu'une mise à niveau de tout le bureau.

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — configuration complète

Installez Windows Terminal compatible Sixel, [QuickLook](https://github.com/QL-Win/QuickLook) et les outils de compilation natifs ci-dessus. QuickLook est une application tierce, pas une fonction intégrée de Windows. Lancez-la une fois dans votre session graphique après installation, puis compilez NeoFM.

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**Configurer QuickLook :** le lanceur recherche par défaut %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe. Le chemin peut varier selon l'installation. Dans PowerShell, définissez et vérifiez le chemin absolu de l'EXE ; remplacez-le si nécessaire.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.cmd
~~~

Équivalent CMD :

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**Important :** démarrez toujours neofm.cmd, jamais lf.exe directement. Le lanceur complète System32, ComSpec et PATHEXT dans son propre processus, sans modifier le PATH global. Le shell interne de lf reste CMD même depuis PowerShell 5.1 ou 7.x. Si Space marque un fichier au lieu de l'afficher, le lfrc personnalisé n'a peut-être pas été chargé. En cas de 'exec: "cmd": executable file not found in %PATH%', redémarrez via le lanceur et vérifiez System32. Ouvrez d'abord un PDF de test directement dans QuickLook, puis testez Space sur ce même fichier. Journal et repli ANSI :

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.cmd
~~~

dd utilise Windows PowerShell 5.1 et l'API Shell native ; testez avec un fichier jetable sur un disque fixe local. Les extensions de formats QuickLook, codecs et la qualité d'image du terminal sont distincts du cœur. Le chemin généralisé doit encore subir une régression GUI complète.

## Raccourcis

| Touche | Action | Touche | Action |
|:--|:--|:--|:--|
| `h / j / k / l` | Naviguer | `Space` | Quick Look |
| `Enter` | Marquer/démarquer un fichier ; entrer dans un dossier | `Ctrl+Space` | Basculer le marquage |
| `yy` | Copier | `p` | Coller |
| `x` | Couper | `dd` | Confirmer puis mettre à la corbeille |
| `o` | Ouvrir à l'extérieur | `zh` | Afficher/masquer les éléments cachés |
| `gg / G` | Premier / dernier | `/` | Rechercher |
| `r` | Renommer | `q` | Quitter |

Les fichiers marqués sont prioritaires ; sinon, l'action concerne le fichier courant. dd demande confirmation et ne bascule pas volontairement vers une suppression définitive si la corbeille est indisponible.

## Architecture et validation

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

Un arbre de sources partagé, des lanceurs par plateforme. Voir [architecture](docs/architecture.md), [validation](docs/validation.md) et [provenance](UPSTREAM.md).

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

Les compilations Rust Release natives unifiées ont réussi sur Mac ARM64, Windows x64 et Linux x64, avec quatre tests de bon fonctionnement du rendu par plateforme ; l'adaptateur Mac a obtenu 19/19. Les déploiements précédents ont testé les aperçus natifs et la corbeille. Le Writer adapté conserve trois échecs connus de tests dérivés de l'amont. Nous ne revendiquons ni parité amont complète, ni nouvelle validation GUI intégrale, ni onglets interactifs, ni rendu complet PDF/Office ou graphique haute résolution en SSH sans interface.

## Remerciements et licences

Basé sur [lf](https://github.com/gokcehan/lf) (MIT) et des composants extraits d'[OpenAI Codex](https://github.com/openai/codex) (Apache-2.0). QuickLook et Dzurkel sont installés séparément, non inclus. NeoFM est indépendant. Voir [LICENSE](LICENSE), [NOTICE](NOTICE), [licence lf](lf-upstream/LICENSE), [UPSTREAM.md](UPSTREAM.md) et [identité visuelle](assets/brand/).

<p align="center"><sub>Petits outils. Fichiers clairs. Aperçus natifs.</sub></p>
