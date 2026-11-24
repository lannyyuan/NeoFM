<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>Gestor de archivos moderno para terminal.</strong><br>Navega como en Vim. Lee como en un visor de documentos moderno.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · **Español** · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM combina el probado gestor de archivos **lf** con un renderizador Rust independiente extraído de la TUI de Codex. Ofrece vistas previas enriquecidas en un espacio de trabajo ligero, centrado en el teclado, y delega las vistas grandes en las herramientas de cada plataforma. Sin servicios de IA, nuevos emuladores de terminal ni cuentas en la nube obligatorias.

## Filosofía de diseño — menos, pero mejor

- **KISS de Unix (Keep It Simple, Stupid):** cada componente tiene una responsabilidad concreta. lf gestiona la navegación y los archivos; el renderizador produce salida limitada para el terminal; las herramientas nativas se encargan de las vistas grandes. Componer herramientas probadas en lugar de reinventarlas.
- **Inspirado en [suckless](https://suckless.org/):** componentes pequeños y comprensibles, pocas dependencias, prioridad al teclado y moderación deliberada. NeoFM no es un proyecto oficial de suckless.
- **Estética por sustracción:** sin pantalla de inicio, panel decorativo, botones innecesarios, navegador integrado ni harness de IA. El logotipo pertenece a los materiales del proyecto, no al espacio del terminal. Solo se añade lo que mejora un flujo de trabajo frecuente.
- **Comportamiento uniforme, implementación nativa:** la misma disposición y teclas de Vim en macOS, Windows y Linux, aprovechando las vistas previas y papeleras de cada sistema. Los archivos locales permanecen locales.

## Una disposición, tres plataformas

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

La proporción predeterminada es **1:2:4**: directorio superior a la izquierda, directorio actual y selección en el centro, vista previa en directo a la derecha. El contenido recibe el mayor espacio, sin editor separado ni barra permanente. Los elementos ocultos se muestran por defecto; zh alterna su visibilidad. Los lanzadores controlan la disposición sin introducir otro framework de interfaz de terminal.

| Contenido | Vista previa en directo | Space — vista grande |
|:--|:--|:--|
| Markdown | Encabezados, listas, tablas adaptables, CJK, resaltado de código, fórmulas, Mermaid | Visor nativo compatible |
| Imágenes | PNG, JPEG, primer fotograma GIF, WebP; Sixel o ANSI | Visor externo de tamaño completo |
| Texto / código | Renderizado de texto limitado | Visor nativo / lector de terminal |
| PDF, Office, multimedia | Metadatos e indicación de vista previa | Depende de los formatos de la aplicación externa |

## Configuración completa recomendada

| Plataforma | Terminal / escritorio | Space | Papelera |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/) (Sixel) o Terminal (ANSI) | Quick Look integrado | Papelera del sistema |
| Windows x64 | [Windows Terminal](https://github.com/microsoft/terminal) reciente compatible con Sixel; PowerShell o CMD | [QL-Win QuickLook](https://github.com/QL-Win/QuickLook), instalación independiente | Papelera de Windows |
| Linux x64 de escritorio | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) u otro terminal; Sixel para imágenes ráster | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux sin escritorio / SSH | Terminal existente; se recomienda less | Texto / metadatos | gio / trash-put |

Son combinaciones recomendadas, no entornos obligatorios. El núcleo no requiere Qt. Sin Sixel o en tmux/screen se usa ANSI. LFP_IMAGE_BACKEND admite auto, ansi, sixel y none; fuerza sixel únicamente en terminales compatibles.

## Compilación y ejecución

Se necesitan Go 1.26+ y Rust/Cargo 1.94+ en la plataforma de destino; Windows también requiere las herramientas MSVC. La primera compilación descarga las dependencias bloqueadas. Es un repositorio privado de desarrollo, no un paquete público; clonar requiere acceso autorizado a GitHub.

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

Usa el lanzador, no lf directamente. Para un comando local, enlaza bin/neofm de la compilación elegida en ~/.local/bin sin sobrescribir una entrada existente. Conserva tu terminal y gestor de ventanas.

**macOS:** no se necesita una aplicación Quick Look adicional. El lanzador usa /usr/bin/qlmanage -p; iTerm2 se recomienda para Sixel, mientras que Terminal puede usar ANSI.

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux:** instala Dzurkel y sus dependencias Qt compatibles por separado. ANSI funciona en DWM/st sin Sixel. Comprueba las siguientes herramientas e instala las que falten mediante el gestor de paquetes. Dzurkel solo se usa en una sesión gráfica local; SSH normal recurre al terminal. Si Qt es incompatible, prepara un entorno Dzurkel compatible y aislado en vez de actualizar todo el escritorio.

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — configuración completa

Instala Windows Terminal compatible con Sixel, [QuickLook](https://github.com/QL-Win/QuickLook) y las herramientas de compilación nativas indicadas. QuickLook es una aplicación de terceros, no una función integrada en Windows. Iníciala una vez en tu sesión de escritorio después de instalarla y compila NeoFM.

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**Configurar QuickLook:** la ruta predeterminada es %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe. Otras instalaciones pueden usar otra ruta. En PowerShell, define y comprueba la ruta absoluta del EXE y sustitúyela si es necesario.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.cmd
~~~

Equivalente en CMD:

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**Importante:** inicia siempre neofm.cmd, no lf.exe. El lanzador completa System32, ComSpec y PATHEXT en su propio proceso, sin cambiar el PATH global. El shell interno de lf sigue siendo CMD aunque se inicie desde PowerShell 5.1 o 7.x. Si Space marca un archivo en vez de mostrarlo, puede que no se haya cargado el lfrc personalizado. Si aparece 'exec: "cmd": executable file not found in %PATH%', reinicia mediante el lanzador y comprueba System32. Abre primero un PDF de prueba directamente con QuickLook y después prueba Space sobre el mismo archivo. Registro de diagnóstico y alternativa ANSI:

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.cmd
~~~

dd utiliza Windows PowerShell 5.1 y la API nativa del Shell; pruébalo con un archivo desechable en una unidad fija local. Los complementos de formatos, códecs y la fidelidad gráfica del terminal son independientes del núcleo. La ruta generalizada aún requiere una prueba de regresión GUI completa.

## Teclas

| Tecla | Acción | Tecla | Acción |
|:--|:--|:--|:--|
| `h / j / k / l` | Navegar | `Space` | Quick Look |
| `Enter` | Marcar/desmarcar archivo; entrar en carpeta | `Ctrl+Space` | Alternar marca |
| `yy` | Copiar | `p` | Pegar |
| `x` | Cortar | `dd` | Confirmar y enviar a la papelera |
| `o` | Abrir externamente | `zh` | Alternar elementos ocultos |
| `gg / G` | Primero / último | `/` | Buscar |
| `r` | Renombrar | `q` | Salir |

Los archivos marcados tienen prioridad; de lo contrario, se actúa sobre el archivo actual. dd solicita confirmación y no cambia deliberadamente a eliminación permanente si la papelera nativa no está disponible.

## Arquitectura y validación

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

Un árbol de código compartido, lanzadores específicos de plataforma. Consulta [arquitectura](docs/architecture.md), [validación](docs/validation.md) y [procedencia](UPSTREAM.md).

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

Las compilaciones nativas Rust Release unificadas pasaron en Mac ARM64, Windows x64 y Linux x64, con cuatro pruebas básicas del renderizador por plataforma; adaptador Mac 19/19. Los despliegues anteriores probaron las vistas nativas y la papelera. El Writer adaptado conserva tres fallos conocidos derivados del upstream. No se afirma paridad total con el upstream, repetición completa de pruebas GUI, pestañas interactivas ni renderizado PDF/Office completo o gráficos de alta resolución por SSH sin escritorio.

## Créditos y licencia

Basado en [lf](https://github.com/gokcehan/lf) (MIT) y componentes extraídos de [OpenAI Codex](https://github.com/openai/codex) (Apache-2.0). QuickLook y Dzurkel se instalan aparte, no se incluyen. NeoFM es independiente. Consulta [LICENSE](LICENSE), [NOTICE](NOTICE), [licencia lf](lf-upstream/LICENSE), [UPSTREAM.md](UPSTREAM.md) y [recursos de marca](assets/brand/).

<p align="center"><sub>Herramientas pequeñas. Archivos claros. Vistas nativas.</sub></p>
