![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>Gerenciador de arquivos moderno para terminal.</strong><br>Navegue como no Vim. Leia como em um visualizador moderno de documentos.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · **Português** · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM combina o gerenciador de arquivos consolidado **lf** com um renderizador Rust independente extraído da TUI do Codex. Oferece prévias ricas em um espaço de trabalho leve e centrado no teclado, delegando prévias maiores às ferramentas de cada plataforma. Sem serviço de IA, novo emulador de terminal ou conta obrigatória na nuvem.

[![Pressione Space para ver um PDF real no Quick Look do macOS. NASA, Earth as Art — ilhas Belcher.](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*Pressione Space para ver um PDF real no Quick Look do macOS. NASA, Earth as Art — ilhas Belcher.*

## Filosofia de design — menos, porém melhor

- **KISS do Unix (Keep It Simple, Stupid):** cada componente tem uma responsabilidade específica. lf cuida da navegação e das operações com arquivos; o renderizador produz saída limitada no terminal; ferramentas nativas cuidam das prévias maiores. Combine ferramentas consolidadas em vez de reinventá-las.
- **Inspirado em [suckless](https://suckless.org/):** componentes pequenos e compreensíveis, poucas dependências, teclado em primeiro lugar e contenção deliberada. NeoFM não é um projeto oficial do suckless.
- **Estética por subtração:** sem tela de abertura, painel decorativo, botões desnecessários, navegador integrado ou harness de IA. O logotipo pertence aos materiais do projeto, não ao espaço de trabalho do terminal. Adicione apenas o que melhora um fluxo frequente.
- **Comportamento consistente, implementação nativa:** mesma disposição e teclas Vim no macOS, Windows e Linux, aproveitando os mecanismos maduros de prévia e lixeira de cada sistema. Arquivos locais permanecem locais.

## Um layout, três plataformas

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

A proporção padrão é **1:2:4**: diretório pai à esquerda, diretório atual e seleção no centro, prévia ao vivo à direita. O conteúdo recebe a maior área, sem editor separado nem barra de ferramentas permanente. Itens ocultos são visíveis por padrão; zh alterna a exibição. Os inicializadores controlam o layout sem introduzir um novo framework de interface de terminal.

| Conteúdo | Prévia ao vivo | Space — prévia ampliada |
|:--|:--|:--|
| Markdown | Títulos, listas, tabelas adaptativas, CJK, destaque de código, fórmulas, Mermaid | Visualizador nativo compatível |
| Imagens | PNG, JPEG, primeiro quadro GIF, WebP; Sixel ou ANSI | Visualizador externo em tamanho maior |
| Texto / código | Renderização de texto limitada | Visualizador nativo / leitor de terminal |
| PDF, Office, mídia | Metadados e indicação de prévia | Depende dos formatos do aplicativo externo |

## Prévias ricas via SSH — sem desktop

NeoFM também é um navegador remoto de documentos. Execute-o em um servidor Linux via SSH comum: navegação, operações de arquivos e a maioria das prévias laterais cotidianas continuam disponíveis sem encaminhamento X11, desktop remoto ou download de arquivos. O renderizador roda no host remoto e o terminal exibe a saída.

| Formato | Prévia lateral via SSH |
| :-- | :-- |
| Markdown: títulos, listas, tabelas, CJK, blocos de código, fórmulas, Mermaid | Renderização rica |
| Texto UTF-8, código-fonte, JSON, logs, fonte HTML | Prévia de texto |
| PNG, JPEG, primeiro quadro GIF, WebP | Sixel ou blocos ANSI dentro dos limites |
| PDF, Office, mídia e compactados | Tipo/tamanho ou metadados binários, não conteúdo completo |

Em Linux por SSH ou headless, Space abre o leitor de terminal (less -R -X quando disponível): Markdown renderizado, texto limitado ou metadados. Imagens são renderizadas na lateral, não ampliadas no leitor. PDF/Office completos, HTML renderizado e Quick Look multimídia exigem um visualizador gráfico local adequado. macOS e Windows mantêm as próprias teclas Space; o leitor SSH automático descrito aqui é o de Linux. A detecção depende do terminal e do ambiente encaminhado; tmux/screen usam ANSI. Force Sixel apenas em terminal compatível. A saída é limitada; a fidelidade visual SSH em alta resolução ainda não foi completamente validada.

Verificado em conexão SSH real com Linux: Markdown, fórmulas/Mermaid, geração de quadros Sixel, imagens ANSI e caminho do leitor de terminal (veja validação). [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

## NeoFM em ação

[![Prévia Markdown — títulos, listas e tabelas adaptativas.](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Prévia Markdown — títulos, listas e tabelas adaptativas.*

[![Matemática e Mermaid — frações empilhadas e fluxogramas no terminal.](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*Matemática e Mermaid — frações empilhadas e fluxogramas no terminal.*

[![Quick Look HTML — uma página HTML local na janela nativa.](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*Quick Look HTML — uma página HTML local na janela nativa.*

[![Imagens Sixel — gráficos reais no terminal, limitados pelo orçamento de pixels.](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Imagens Sixel — gráficos reais no terminal, limitados pelo orçamento de pixels.*

[![Código Markdown — destaque de um bloco Python, não de um arquivo-fonte independente.](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Código Markdown — destaque de um bloco Python, não de um arquivo-fonte independente.*

Capturas reais do macOS; Graphite / Menlo. Créditos da NASA: [Notas de captura e créditos](docs/gallery.md).

## Configuração completa recomendada

| Plataforma | Terminal / desktop | Space | Lixeira |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/) (Sixel) ou Terminal (ANSI) | Quick Look integrado | Lixeira do sistema |
| Windows x64 | [Windows Terminal](https://github.com/microsoft/terminal) recente compatível com Sixel; PowerShell ou CMD | [QL-Win QuickLook](https://github.com/QL-Win/QuickLook), instalação separada | Lixeira do Windows |
| Linux x64 desktop | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) ou outro terminal; Sixel para imagens raster | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux headless / SSH | Terminal existente; less recomendado | Texto / metadados | gio / trash-put |

São combinações recomendadas, não ambientes obrigatórios. O núcleo não depende de Qt. Sem Sixel ou em tmux/screen, use ANSI. LFP_IMAGE_BACKEND aceita auto, ansi, sixel e none; force sixel somente em terminais compatíveis.

## Compilação e execução

Go 1.26+ e Rust/Cargo 1.94+ são necessários na plataforma de destino; Windows também requer as ferramentas MSVC. A primeira compilação baixa as dependências fixadas. Pacotes pré-compilados estão disponíveis em [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest); o repositório de código-fonte é público.

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

Use o inicializador, não execute lf diretamente. Para um comando local, crie um link para bin/neofm da compilação escolhida em ~/.local/bin, sem sobrescrever uma entrada existente. Mantenha seu terminal e gerenciador de janelas.

**macOS:** não é necessário um aplicativo Quick Look adicional. O inicializador usa /usr/bin/qlmanage -p; iTerm2 é recomendado para Sixel, e o Terminal pode usar ANSI.

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux:** instale Dzurkel e dependências Qt compatíveis separadamente. ANSI também funciona em DWM/st sem Sixel. Verifique as ferramentas abaixo e instale as ausentes pelo gerenciador de pacotes. Dzurkel só é usado em sessão gráfica local; SSH normal usa a alternativa no terminal. Se houver incompatibilidade de Qt, configure um runtime Dzurkel compatível e isolado em vez de atualizar todo o desktop.

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — configuração completa

Instale Windows Terminal compatível com Sixel, [QuickLook](https://github.com/QL-Win/QuickLook) e as ferramentas de compilação nativas citadas. QuickLook é um aplicativo de terceiros, não um recurso interno do Windows. Após instalar, execute-o uma vez na sessão de desktop e compile o NeoFM.

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.exe
~~~

**Configurar QuickLook:** o caminho padrão é %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe. Outros métodos de instalação podem usar caminhos diferentes. No PowerShell, defina e verifique o caminho completo do EXE, substituindo-o se necessário.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

Equivalente no CMD:

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**Importante:** inicie sempre neofm.exe, não lf.exe. O inicializador complementa System32, ComSpec e PATHEXT em seu próprio processo, sem alterar o PATH global. O shell interno de lf continua sendo CMD mesmo ao iniciar pelo PowerShell 5.1 ou 7.x. Se Space marcar um arquivo em vez de abrir a prévia, o lfrc personalizado talvez não tenha sido carregado. Se aparecer 'exec: "cmd": executable file not found in %PATH%', reinicie pelo inicializador e verifique System32. Abra primeiro um PDF de teste diretamente com QuickLook e depois teste Space no mesmo arquivo. Log de diagnóstico e alternativa ANSI:

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd utiliza Windows PowerShell 5.1 e a API nativa do Shell; teste com um arquivo descartável em uma unidade fixa local. Plug-ins de formatos, codecs e fidelidade de imagem do terminal são separados do núcleo. O caminho generalizado ainda exige regressão GUI completa.

## Teclas

### Navegação e prévia

| Tecla | Ação |
| :-- | :-- |
| `↑ / k · ↓ / j` | Subir/descer |
| `← / h` | Diretório pai |
| `→ / l · Enter` | Entrar na pasta; alternar marca do arquivo |
| `Home / gg · End / G` | Primeiro/último |
| `Ctrl+U / Ctrl+D` | Meia página acima/abaixo |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | Página acima/abaixo |
| `Ctrl+Y / Ctrl+E` | Rolar uma linha acima/abaixo |
| `H / M / L` | Topo/meio/base da tela |
| `gh` | Diretório pessoal |
| `[ / ]` | Diretório anterior/próximo no histórico |
| `Space` | Prévia ampliada / leitor de terminal |
| `o` | Abrir externamente |

### Seleção e arquivos

| Tecla | Ação |
| :-- | :-- |
| `Ctrl+Space` | Alternar marca atual |
| `u` | Desmarcar tudo em todos os diretórios |
| `v` | Inverter seleção no diretório atual |
| `V` | Iniciar seleção de intervalo |
| `yy` | Copiar para o buffer NeoFM |
| `x` | Recortar para o buffer NeoFM |
| `p` | Colar no diretório atual |
| `c` | Limpar buffer de arquivos |
| `dd` | Confirmar e enviar à lixeira |
| `r` | Renomear |
| `t` | Alternar etiqueta |

### Pesquisa e histórico

| Tecla | Ação |
| :-- | :-- |
| `/ / ?` | Pesquisar nome para frente/trás |
| `n / N` | Próximo/anterior resultado |
| `f / F` | Busca rápida para frente/trás |
| `; / ,` | Próxima/anterior correspondência |
| `m / ' / "` | Salvar/carregar/remover marcador de diretório |

### Ordenação e exibição

| Tecla | Ação |
| :-- | :-- |
| `zh` | Alternar ocultos |
| `zr` | Inverter ordenação |
| `zn / zs / zt / za` | Ocultar info / tamanho / data / ambos |
| `sn / ss / st` | Ordenar por nome / tamanho / modificação |
| `sa / sb / sc / se` | Ordenar por acesso / criação / alteração de estado / extensão |
| `Ctrl+L / Ctrl+R` | Redesenhar / recarregar |

### Comandos e modos

| Tecla | Ação |
| :-- | :-- |
| `:` | Entrada de comandos |
| `$ / % / ! / &` | Shell / pipe / espera / assíncrono |
| `Ctrl+P / Ctrl+N` | Comando anterior/próximo |
| `q` | Sair |

Teclas padrão no modo normal distinguem maiúsculas. Marque vários arquivos com Enter ou Ctrl+Space, pressione yy (copiar) ou x (recortar), vá ao destino e pressione p. Sem marcação, usa-se o arquivo atual. Copiar/recortar limpa marcas, mas mantém o buffer interno; u limpa marcas, c limpa o buffer. yy e dd são duas teclas consecutivas. dd pede confirmação e usa exclusão recuperável, sem recorrer intencionalmente à exclusão permanente. Space e o dependem da plataforma e sessão.

Modo visual: V inicia intervalo, mova para ampliar, V aceita, Esc descarta, o troca as extremidades. y/d herdados permanecem no modo visual, diferentes de yy/x no modo normal. Aceite com V e use yy/x/dd no modo normal. Linha de comando: Enter executa, Esc cancela, Tab completa, setas cima/baixo para histórico, esquerda/direita e Home/End para cursor. Para todas as teclas de edição herdadas e comandos, execute neofm -doc ou consulte [lf documentation](lf-upstream/doc.txt).

Atalhos de roda do mouse são herdados, mas o mouse fica desativado por padrão.

## Arquitetura e validação

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

Uma árvore de código compartilhada, inicializadores específicos por plataforma. Consulte [arquitetura](docs/architecture.md), [validação](docs/validation.md) e [proveniência](UPSTREAM.md).

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

As compilações nativas Rust Release unificadas passaram em Mac ARM64, Windows x64 e Linux x64, com quatro testes básicos do renderizador por plataforma; adaptador Mac 19/19. Implantações anteriores testaram prévias nativas e lixeira. O Writer adaptado conserva três falhas conhecidas derivadas do upstream. Não afirmamos paridade total com upstream, repetição integral de testes GUI, abas interativas nem renderização completa PDF/Office ou gráficos de alta resolução em SSH headless.

## Créditos e licença

Baseado em [lf](https://github.com/gokcehan/lf) (MIT) e componentes extraídos de [OpenAI Codex](https://github.com/openai/codex) (Apache-2.0). QuickLook e Dzurkel são instalados separadamente, não incluídos. NeoFM é independente. Consulte [LICENSE](LICENSE), [NOTICE](NOTICE), [licença lf](lf-upstream/LICENSE), [UPSTREAM.md](UPSTREAM.md) e [recursos de marca](assets/brand/).

<p align="center"><sub>Ferramentas pequenas. Arquivos claros. Prévias nativas.</sub></p>
