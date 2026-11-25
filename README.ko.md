![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>현대적인 터미널 파일 관리자</strong><br>Vim처럼 탐색하고, 현대적인 문서 뷰어처럼 읽으세요.</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · **한국어** · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM은 검증된 **lf** 파일 관리자와 Codex TUI에서 추출한 독립 Rust 렌더러를 결합하여 가볍고 키보드 중심인 터미널 작업 공간에 풍부한 문서 미리보기를 제공합니다. 큰 미리보기는 각 플랫폼의 기존 도구에 맡깁니다. AI 서비스, 새로운 터미널 에뮬레이터, 필수 클라우드 계정은 필요하지 않습니다.

[![Space로 실제 PDF를 macOS Quick Look에서 봅니다. NASA 《Earth as Art》 — 벨처 제도.](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*Space로 실제 PDF를 macOS Quick Look에서 봅니다. NASA 《Earth as Art》 — 벨처 제도.*

## 설계 철학 — 더 적게, 더 좋게

- **Unix KISS(Keep It Simple, Stupid):** 각 구성 요소의 책임을 제한합니다. lf는 탐색과 파일 작업, 렌더러는 범위가 제한된 터미널 출력, 네이티브 도구는 큰 미리보기를 담당합니다. 검증된 도구를 조합하고 같은 기능을 다시 만들지 않습니다.
- **[suckless](https://suckless.org/) 철학에서 영감:** 작고 이해하기 쉬운 구성 요소, 적은 의존성, 키보드 우선, 의도적인 절제. NeoFM은 suckless 공식 프로젝트가 아닙니다.
- **덜어내는 디자인:** 시작 화면, 장식용 대시보드, 불필요한 버튼, 내장 브라우저 또는 AI Harness를 추가하지 않습니다. 로고는 프로젝트 자료에만 사용하고 터미널 작업 공간을 차지하지 않습니다. 자주 사용하는 작업 흐름을 개선할 때만 기능을 추가합니다.
- **일관된 동작, 네이티브 구현:** macOS, Windows, Linux에서 동일한 레이아웃과 Vim 키맵을 사용하며 각 OS의 성숙한 미리보기 및 휴지통을 활용합니다. 로컬 파일은 로컬에 남습니다.

## 하나의 레이아웃, 세 플랫폼

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

기본 비율은 **1:2:4**입니다. 왼쪽은 상위 디렉터리, 가운데는 현재 디렉터리와 선택 항목, 오른쪽은 실시간 미리보기입니다. 콘텐츠에 가장 넓은 공간을 할당하고 별도 편집기나 상시 도구 모음을 두지 않습니다. 숨김 항목은 기본 표시되며 zh로 전환합니다. 각 플랫폼의 실행기가 레이아웃을 제어하며 새 터미널 UI 프레임워크를 도입하지 않습니다.

| 내용 | 실시간 미리보기 | Space — 큰 미리보기 |
|:--|:--|:--|
| Markdown | 제목, 목록, 적응형 표, CJK, 코드 강조, 수식, Mermaid | 지원되는 네이티브 뷰어 |
| 이미지 | PNG, JPEG, GIF 첫 프레임, WebP; Sixel 또는 ANSI | 외부 전체 크기 뷰어 |
| 텍스트 / 코드 | 제한된 텍스트 렌더링 | 네이티브 뷰어 / 터미널 리더 |
| PDF, Office, 미디어 | 메타데이터와 미리보기 안내 | 외부 앱의 형식 지원에 따름 |

## SSH에서도 풍부한 미리보기 — 데스크톱 불필요

NeoFM은 로컬 파일 관리자이자 원격 문서 브라우저입니다. Linux 서버에서 일반 SSH로 실행하면 X11 포워딩, 원격 데스크톱 또는 파일 다운로드 없이 탐색, 파일 작업과 대부분의 일상적인 사이드바 미리보기를 사용할 수 있습니다. 렌더러는 원격 호스트에서 실행되고 터미널에 출력합니다.

| 형식 | SSH 실시간 미리보기 |
| :-- | :-- |
| Markdown: 제목, 목록, 표, CJK, 코드 블록, 수식, Mermaid | 풍부한 렌더링 |
| UTF-8 텍스트, 소스, JSON, 로그, HTML 소스 | 텍스트 미리보기 |
| PNG, JPEG, GIF 첫 프레임, WebP | 렌더링 한도 내 Sixel 또는 ANSI 색상 블록 |
| PDF, Office, 미디어, 압축 파일 | 유형/크기 또는 바이너리 메타데이터; 전체 콘텐츠 렌더링 아님 |

Linux SSH 또는 헤드리스 세션에서 Space는 터미널 리더(less가 있으면 less -R -X)로 전환되어 렌더링된 Markdown, 제한된 텍스트 또는 메타데이터를 표시합니다. 이미지는 사이드바에서 렌더링되며 리더는 확대 이미지를 표시하지 않습니다. 전체 PDF/Office, 렌더링된 HTML, 멀티미디어 Quick Look에는 적절한 로컬 GUI 뷰어가 필요합니다. macOS와 Windows는 현재 플랫폼별 Space 바인딩을 유지하며 여기의 자동 SSH 리더는 Linux 경로입니다. 이미지 자동 감지는 터미널 및 전달된 환경 변수에 의존하며 tmux/screen은 ANSI를 사용합니다. 호환 터미널에서만 Sixel을 강제하십시오. 출력은 제한되며 SSH 고해상도 시각 검증은 완료되지 않았습니다.

실제 Linux SSH 연결에서 Markdown, 수식/Mermaid, Sixel 프레임 생성, ANSI 이미지 및 터미널 리더 경로를 검증했습니다(검증 기록 참조). [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

## NeoFM 실제 실행 화면

[![Markdown 실시간 미리보기 — 제목, 목록, 적응형 표.](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Markdown 실시간 미리보기 — 제목, 목록, 적응형 표.*

[![수식과 Mermaid — 분수 렌더링과 터미널 흐름도.](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*수식과 Mermaid — 분수 렌더링과 터미널 흐름도.*

[![HTML Quick Look — 네이티브 미리보기 창의 로컬 HTML.](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*HTML Quick Look — 네이티브 미리보기 창의 로컬 HTML.*

[![Sixel 이미지 — 실제 터미널 그래픽이며 렌더러 픽셀 예산의 제한을 받습니다.](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Sixel 이미지 — 실제 터미널 그래픽이며 렌더러 픽셀 예산의 제한을 받습니다.*

[![Markdown 코드 — Python 코드 블록 구문 강조이며 독립 소스 파일 강조는 아닙니다.](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Markdown 코드 — Python 코드 블록 구문 강조이며 독립 소스 파일 강조는 아닙니다.*

실제 macOS 캡처. Graphite / Menlo. NASA 이미지 출처는 [캡처 기록 및 이미지 출처](docs/gallery.md).

## 권장 전체 기능 구성

| 플랫폼 | 터미널 / 데스크톱 | Space | 휴지통 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)(Sixel) 또는 Terminal(ANSI) | 내장 Quick Look | 시스템 휴지통 |
| Windows x64 | Sixel 지원 최신 [Windows Terminal](https://github.com/microsoft/terminal); PowerShell 또는 CMD | 별도 설치 [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows 휴지통 |
| Linux x64 데스크톱 | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) 또는 다른 터미널; 래스터 그래픽에는 Sixel 지원 터미널 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux 헤드리스 / SSH | 기존 터미널; less 권장 | 텍스트 / 메타데이터 | gio / trash-put |

권장 조합이며 특정 데스크톱 환경이 필수는 아닙니다. 코어는 Qt에 의존하지 않습니다. Sixel 미지원 터미널 또는 tmux/screen에서는 ANSI로 대체합니다. LFP_IMAGE_BACKEND는 auto, ansi, sixel, none을 지원하며 호환 터미널에서만 sixel을 강제하십시오.

## 빌드 및 실행

대상 플랫폼에 Go 1.26+, Rust/Cargo 1.94+가 필요하며 Windows는 MSVC 빌드 도구도 필요합니다. 첫 빌드에서 잠긴 버전의 의존성을 다운로드합니다. 빌드된 패키지는 [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest)에서 받을 수 있으며 소스 저장소는 공개되어 있습니다.

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

lf를 직접 실행하지 말고 실행기를 사용합니다. 사용자 수준 명령이 필요하면 기존 항목을 덮어쓰지 않고 해당 빌드의 bin/neofm을 ~/.local/bin에 연결합니다. 기존 터미널과 창 관리자를 유지합니다.

**macOS:** 별도 Quick Look 앱이 필요 없습니다. 실행기는 /usr/bin/qlmanage -p를 호출합니다. Sixel에는 iTerm2를 권장하고 시스템 Terminal은 ANSI를 사용할 수 있습니다.

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux:** 그래픽 미리보기를 위해 Dzurkel과 호환 Qt 의존성을 별도로 설치합니다. Sixel이 없는 DWM/st에서도 ANSI가 작동합니다. 다음 도구를 확인하고 누락된 도구는 배포판 패키지 관리자로 설치합니다. 로컬 그래픽 세션에서만 Dzurkel을 사용하고 일반 SSH에서는 터미널로 대체합니다. Qt 버전 충돌이 있다면 NeoFM 때문에 전체 데스크톱을 업그레이드하는 대신 호환되는 격리 Dzurkel 런타임을 구성합니다.

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — 전체 구성

Sixel을 지원하는 Windows Terminal, [QuickLook](https://github.com/QL-Win/QuickLook), 위의 네이티브 빌드 도구를 설치합니다. QuickLook은 Windows 내장 기능이 아닌 타사 앱입니다. 설치 후 데스크톱 세션에서 한 번 실행한 다음 NeoFM을 빌드합니다.

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.exe
~~~

**QuickLook 설정:** 기본 경로는 %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe입니다. 설치 방법에 따라 다를 수 있습니다. PowerShell에서 EXE 전체 경로를 설정하고 확인하며 필요하면 실제 경로로 바꿉니다.

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

CMD에서의 방법:

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**중요:** lf.exe가 아니라 항상 neofm.exe로 실행합니다. 실행기는 자체 프로세스에서 System32, ComSpec, PATHEXT를 보완하고 전역 PATH를 변경하지 않습니다. PowerShell 5.1 또는 7.x에서 시작해도 lf 내부 셸은 CMD입니다. Space가 미리보기 대신 표시를 토글하면 사용자 정의 lfrc가 로드되지 않았을 수 있습니다. 'exec: "cmd": executable file not found in %PATH%'가 나오면 실행기로 다시 시작하고 System32를 확인합니다. 먼저 테스트 PDF를 QuickLook으로 직접 연 뒤 같은 파일에서 Space를 시험합니다. 진단 로그와 ANSI 대체:

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd는 Windows PowerShell 5.1과 네이티브 Shell API를 사용합니다. 로컬 고정 드라이브의 일회용 파일로 확인합니다. QuickLook 형식 플러그인, 코덱 및 터미널 화질은 코어와 별개입니다. 일반화된 경로는 아직 전체 GUI 회귀 시험이 필요합니다.

## 키맵

### 탐색 및 미리보기

| 키 | 동작 |
| :-- | :-- |
| `↑ / k · ↓ / j` | 위/아래 이동 |
| `← / h` | 상위 디렉터리 |
| `→ / l · Enter` | 디렉터리 진입 / 파일 표시 전환 |
| `Home / gg · End / G` | 처음/마지막 |
| `Ctrl+U / Ctrl+D` | 반 페이지 위/아래 |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | 페이지 위/아래 |
| `Ctrl+Y / Ctrl+E` | 한 줄 위/아래 스크롤 |
| `H / M / L` | 화면 위/중간/아래 |
| `gh` | 홈 디렉터리 |
| `[ / ]` | 이전/다음 디렉터리 기록 |
| `Space` | 큰 미리보기 / 터미널 리더 |
| `o` | 외부 앱 열기 |

### 선택 및 파일 작업

| 키 | 동작 |
| :-- | :-- |
| `Ctrl+Space` | 현재 항목 표시 전환 |
| `u` | 모든 디렉터리 선택 해제 |
| `v` | 현재 디렉터리 선택 반전 |
| `V` | 범위 선택 시작 |
| `yy` | NeoFM 버퍼에 복사 |
| `x` | NeoFM 버퍼에 잘라내기 |
| `p` | 현재 디렉터리에 붙여넣기 |
| `c` | 파일 버퍼 비우기 |
| `dd` | 확인 후 휴지통 |
| `r` | 이름 변경 |
| `t` | 파일 태그 전환 |

### 검색 및 기록

| 키 | 동작 |
| :-- | :-- |
| `/ / ?` | 파일명 앞/뒤 검색 |
| `n / N` | 다음/이전 검색 결과 |
| `f / F` | 앞/뒤 빠른 파일명 찾기 |
| `; / ,` | 다음/이전 찾기 결과 |
| `m / ' / "` | 디렉터리 북마크 저장/열기/삭제 |

### 정렬 및 표시

| 키 | 동작 |
| :-- | :-- |
| `zh` | 숨김 항목 전환 |
| `zr` | 정렬 순서 반전 |
| `zn / zs / zt / za` | 정보 숨김 / 크기 / 시간 / 둘 다 |
| `sn / ss / st` | 이름 / 크기 / 수정 시간 정렬 |
| `sa / sb / sc / se` | 접근 / 생성 / 상태 변경 시간 / 확장자 정렬 |
| `Ctrl+L / Ctrl+R` | 다시 그리기 / 다시 로드 |

### 명령 및 모드

| 키 | 동작 |
| :-- | :-- |
| `:` | 명령 입력 |
| `$ / % / ! / &` | 셸 / 파이프 / 대기 / 비동기 |
| `Ctrl+P / Ctrl+N` | 이전/다음 명령 기록 |
| `q` | 종료 |

일반 모드 기본 키는 대소문자를 구분합니다. Enter 또는 Ctrl+Space로 여러 항목을 선택하고 yy(복사) 또는 x(잘라내기), 대상 디렉터리에서 p(붙여넣기)를 누릅니다. 선택 항목이 없으면 현재 파일을 사용합니다. 복사/잘라내기는 표시를 해제하지만 내부 파일 버퍼는 유지합니다. u는 선택 해제, c는 버퍼 비우기입니다. yy와 dd는 연속 입력입니다. dd는 확인 후 복구 가능한 삭제를 사용하며 의도적으로 영구 삭제로 전환하지 않습니다. Space와 o는 플랫폼 및 세션에 따릅니다.

Visual Mode: V로 범위 시작, 이동하여 확장, V로 수락, Esc로 취소, o로 양 끝 전환. Visual Mode에는 업스트림 y/d가 유지되어 일반 모드 yy/x와 다릅니다. V로 확정 후 일반 모드에서 yy/x/dd를 권장합니다. 명령줄 모드: Enter 실행, Esc 취소, Tab 완성, 위/아래 기록, 좌우 및 Home/End 커서 이동. 전체 상속된 편집 키 및 명령은 neofm -doc 또는 다음 자료를 참고하십시오: [lf documentation](lf-upstream/doc.txt).

마우스 휠 바인딩도 상속되지만 기본적으로 마우스는 비활성화됩니다.

## 아키텍처 및 검증

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

하나의 공유 소스 트리와 플랫폼별 실행기. [아키텍처](docs/architecture.md), [검증](docs/validation.md), [업스트림 출처](UPSTREAM.md)를 참조하십시오.

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

통합 Rust Release는 Mac ARM64, Windows x64, Linux x64에서 네이티브 빌드와 각 4개 렌더러 스모크 테스트를 통과했습니다. Mac Adapter는 19/19입니다. 이전 배포에서 실제 플랫폼의 미리보기와 휴지통도 시험했습니다. 적응된 Writer에는 알려진 업스트림 파생 실패 3건이 남아 있습니다. 전체 업스트림 일치, 전체 GUI 재시험, 대화형 Sidebar Tabs, 헤드리스 SSH에서 완전한 PDF/Office 및 고해상도 그래픽을 주장하지 않습니다.

## 감사 및 라이선스

NeoFM은 오픈 소스 커뮤니티의 성과에 기반합니다. 다음 프로젝트의 유지보수자와 기여자에게 진심으로 감사드립니다.

- [lf](https://github.com/gokcehan/lf) — gokcehan & contributors — 파일 관리자 기반, 탐색 및 파일 작업.
- [OpenAI Codex](https://github.com/openai/codex) — OpenAI & contributors — 추출하고 조정한 Markdown Writer, 수식, Mermaid 및 터미널 이미지 렌더링 구성 요소.
- [Ratatui](https://github.com/ratatui/ratatui) — Codex를 통해 계승한 코드를 포함한 터미널 렌더링 기반.
- [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) / [Dzurkel](https://github.com/EdinUser/dzurkel) — 각각 Windows와 Linux의 네이티브 대형 미리보기 통합(별도 설치).

다음 지원 라이브러리의 유지보수자에게도 감사드립니다: [tcell](https://github.com/gdamore/tcell), [pulldown-cmark](https://crates.io/crates/pulldown-cmark), [syntect](https://crates.io/crates/syntect), [two-face](https://crates.io/crates/two-face), [image](https://crates.io/crates/image).

[lf](https://github.com/gokcehan/lf)(MIT)와 [OpenAI Codex](https://github.com/openai/codex)(Apache-2.0)의 추출 구성 요소를 사용합니다. QuickLook과 Dzurkel은 별도 설치이며 포함되지 않습니다. NeoFM은 독립 프로젝트입니다. [LICENSE](LICENSE), [NOTICE](NOTICE), [lf 라이선스](lf-upstream/LICENSE), [UPSTREAM.md](UPSTREAM.md), [브랜드 리소스](assets/brand/)를 참조하십시오.

<p align="center"><sub>작은 도구, 명확한 파일, 네이티브 미리보기.</sub></p>
