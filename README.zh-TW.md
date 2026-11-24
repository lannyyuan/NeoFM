<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>現代終端機檔案管理員</strong><br>像 Vim 一樣導覽，像現代文件閱讀器一樣瀏覽。</p>

[English](README.md) · [简体中文](README.zh-CN.md) · **繁體中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM 結合成熟的 **lf** 檔案管理員，以及從 Codex TUI 擷取的獨立 Rust 渲染器，為輕量、鍵盤優先的終端機工作空間提供豐富的文件預覽。大型預覽交給各平台成熟的工具。不依賴 AI 服務，不重新打造終端機，也不強制使用雲端帳號。

## 設計理念：少，但更好

- **Unix KISS（Keep It Simple, Stupid）原則：** 每個元件專注於自己的職責。lf 負責導覽與檔案操作，渲染器提供有界的終端機輸出，原生工具處理大型預覽。組合成熟工具，不重複造輪子。
- **受 [suckless](https://suckless.org/) 極簡哲學啟發：** 小而易於理解的元件、少量相依性、鍵盤優先、有意識的節制。NeoFM 並非 suckless 官方專案。
- **以減法建立品味：** 不加入啟動畫面、裝飾性儀表板、多餘按鈕、內嵌瀏覽器或 AI Harness。Logo 用於專案資料，不佔用終端機空間。只有改善高頻工作流程的功能才值得加入。
- **一致行為，原生實作：** macOS、Windows、Linux 採用相同版面與 Vim 按鍵配置；預覽及垃圾桶重用各系統成熟的功能。本機檔案留在本機。

## 一種版面，三個平台

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

預設比例 **1:2:4**：左側父目錄、中間目前目錄與檔案選取、右側即時預覽。將最大空間留給內容，不新增編輯器或常駐工具列。預設顯示隱藏項目，zh 切換。版面由各平台啟動設定控制，不新增終端機 UI 框架。

| 檔案 | 側欄即時預覽 | Space 大型預覽 |
|:--|:--|:--|
| Markdown | 標題、清單、自適應表格、中日韓文字、程式碼醒目提示、公式、Mermaid | 系統支援的原生檢視器 |
| 圖片 | PNG、JPEG、GIF 首幀、WebP；Sixel 或 ANSI | 外部大圖視窗 |
| 文字 / 程式碼 | 有界文字渲染 | 原生檢視器 / 終端機閱讀器 |
| PDF、Office、媒體 | 中繼資料與預覽提示 | 取決於外部應用程式的格式支援 |

## 建議的完整功能組合

| 平台 | 終端機 / 桌面 | Space | 垃圾桶 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)（Sixel）或系統 Terminal（ANSI） | 系統 Quick Look | 系統垃圾桶 |
| Windows x64 | 支援 Sixel 的新版 [Windows Terminal](https://github.com/microsoft/terminal)；PowerShell 或 CMD | 獨立安裝 [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows 資源回收筒 |
| Linux x64 桌面 | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) 或其他終端機；點陣圖需要 Sixel 終端機 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux Headless / SSH | 現有終端機；建議 less | 文字與中繼資料備援 | gio / trash-put |

以上為建議組合，並非強制桌面環境。核心不依賴 Qt。不支援 Sixel 的終端機或 tmux/screen 使用 ANSI 備援。LFP_IMAGE_BACKEND 可設為 auto、ansi、sixel、none；僅在相容終端機強制 sixel。

## 建置與執行

目標平台需要 Go 1.26+、Rust/Cargo 1.94+；Windows 另需 MSVC 工具鏈。首次建置下載鎖定的相依套件。目前為私人開發儲存庫，尚無公開套件發行；複製需要授權的 GitHub 存取權限。

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

使用啟動器而非直接執行 lf。若需要使用者層級的統一命令，可將對應建置的 bin/neofm 連結到 ~/.local/bin，避免覆寫既有入口。保留自己的終端機與視窗管理員。

**macOS：** 無須額外 Quick Look 應用程式。啟動器呼叫 /usr/bin/qlmanage -p；建議 iTerm2 用於 Sixel，系統 Terminal 可用 ANSI。

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux：** 圖形預覽需獨立安裝 Dzurkel 與相容 Qt 相依套件。DWM/st 不支援 Sixel 時仍可使用 ANSI。檢查下列工具，缺少時由發行版套件管理員安裝。僅在本機圖形工作階段呼叫 Dzurkel；一般 SSH 採用終端機備援。Qt 版本不相容時，建議單獨配置相容且隔離的 Dzurkel 執行環境，而不是為 NeoFM 升級整個桌面。

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows：完整設定方式

安裝支援 Sixel 的 Windows Terminal、[QuickLook](https://github.com/QL-Win/QuickLook) 與上述原生編譯工具。QuickLook 是 Windows 第三方應用程式，不是系統內建功能。安裝後在目前桌面工作階段啟動一次，再建置 NeoFM。

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**設定 QuickLook：** 啟動器預設查找 %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe。其他安裝方式可能使用不同路徑。在 PowerShell 設定並檢查完整 EXE 路徑；若不存在，請換成實際路徑：

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.cmd
~~~

CMD 對應方式：

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**注意：** 一律使用 neofm.cmd 啟動，不直接執行 lf.exe。啟動器在自身程序補齊 System32、ComSpec 和 PATHEXT，不修改全域 PATH。即使從 PowerShell 5.1 或 7.x 啟動，lf 內部仍使用 CMD。若 Space 變成標記而非預覽，可能沒有載入自訂 lfrc。若出現 'exec: "cmd": executable file not found in %PATH%'，請離開後使用啟動器重啟並檢查 System32。先確認 QuickLook 可直接開啟測試 PDF，再對同一檔案按 Space。診斷記錄與 ANSI 備援：

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.cmd
~~~

dd 使用系統 Windows PowerShell 5.1 與原生 Shell API 移至資源回收筒；請以本機固定磁碟的暫存檔測試。QuickLook 格式外掛、編解碼器及終端機像素效果與核心分開。通用路徑仍待完整 GUI 迴歸測試，詳見驗證記錄。

## 快捷鍵

| 按鍵 | 功能 | 按鍵 | 功能 |
|:--|:--|:--|:--|
| `h / j / k / l` | Vim 導覽 | `Space` | Quick Look |
| `Enter` | 標記/取消標記檔案；進入目錄 | `Ctrl+Space` | 切換標記 |
| `yy` | 複製 | `p` | 貼上 |
| `x` | 剪下 | `dd` | 確認後移至垃圾桶 |
| `o` | 外部開啟 | `zh` | 切換隱藏項目 |
| `gg / G` | 首項 / 末項 | `/` | 搜尋 |
| `r` | 重新命名 | `q` | 離開 |

已標記項目優先，否則操作目前檔案。dd 需要確認，垃圾桶不可用時不主動改為永久刪除。

## 架構與驗證

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

一套共享原始碼，獨立平台啟動層。參閱[架構](docs/architecture.md)、[驗證記錄](docs/validation.md)及[上游來源](UPSTREAM.md)。

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

統一 Rust Release 在 Mac ARM64、Windows x64、Linux x64 原生編譯成功，各通過 4 項渲染器冒煙測試；Mac Adapter 19/19。先前部署也驗證實際平台的 Quick Look 與垃圾桶。適配 Writer 保留三項已知上游衍生迴歸失敗；不宣稱完整上游一致性、完整 GUI 迴歸、互動式 Sidebar Tabs 或 Headless SSH 下完整 PDF/Office 與高解析圖形能力。

## 致謝與授權

基於 [lf](https://github.com/gokcehan/lf)（MIT）和 [OpenAI Codex](https://github.com/openai/codex)（Apache-2.0）的擷取元件。QuickLook、Dzurkel 獨立安裝，不隨儲存庫打包。NeoFM 是獨立專案。參閱 [LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf 授權](lf-upstream/LICENSE)、[上游來源](UPSTREAM.md) 與 [品牌資源](assets/brand/)。

<p align="center"><sub>小工具、清楚的檔案、原生的預覽。</sub></p>
