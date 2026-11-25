![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>現代終端機檔案管理員</strong><br>像 Vim 一樣導覽，像現代文件閱讀器一樣瀏覽。</p>

[English](README.md) · [简体中文](README.zh-CN.md) · **繁體中文** · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM 結合成熟的 **lf** 檔案管理員，以及從 Codex TUI 擷取的獨立 Rust 渲染器，為輕量、鍵盤優先的終端機工作空間提供豐富的文件預覽。大型預覽交給各平台成熟的工具。不依賴 AI 服務，不重新打造終端機，也不強制使用雲端帳號。

[![按 Space，在 macOS 原生 Quick Look 中瀏覽真實 PDF。NASA《Earth as Art》— 貝爾徹群島。](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*按 Space，在 macOS 原生 Quick Look 中瀏覽真實 PDF。NASA《Earth as Art》— 貝爾徹群島。*

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

## SSH 遠端預覽 — 無需圖形桌面

NeoFM 不只是本機檔案管理員，也是遠端文件瀏覽器。透過一般 SSH 在 Linux 伺服器執行，無需 X11 轉送、遠端桌面或下載檔案，即可完成導覽、檔案操作與大部分日常側欄預覽。渲染器在遠端執行，終端機顯示輸出。

| 檔案類型 | SSH 即時側欄 |
| :-- | :-- |
| Markdown：標題、清單、表格、中日韓文字、程式碼區塊、公式與 Mermaid | 富文字渲染 |
| UTF-8 文字、原始碼、JSON、記錄、HTML 原始碼 | 文字預覽 |
| PNG、JPEG、GIF 首幀、WebP | 在渲染限制內使用 Sixel 或 ANSI 色塊 |
| PDF、Office、媒體與壓縮檔 | 類型/大小或二進位中繼資料，非完整內容渲染 |

Linux SSH 或無桌面工作階段中，Space 自動使用終端機閱讀備援（有 less 時使用 less -R -X），顯示已渲染 Markdown、有界文字或中繼資料。圖片在側欄渲染，閱讀器不提供放大圖形。完整 PDF/Office、渲染 HTML 與多媒體 Quick Look 需要合適的本機圖形檢視器。macOS 與 Windows 目前保留各自的 Space 綁定；此處自動 SSH 閱讀指 Linux。圖像自動偵測取決於終端機能力及轉送環境變數；tmux/screen 採 ANSI。僅在相容終端機強制 Sixel。輸出有限制，SSH 高解析度圖形的實際視覺效果尚未完整驗收。

已透過實際 SSH 連線至 Linux 驗證 Markdown、公式/Mermaid、Sixel 幀輸出、ANSI 圖片及終端機閱讀路徑（見驗證記錄）。 [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

## NeoFM 實際運作展示

[![Markdown 即時預覽 — 標題、清單與自適應表格。](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Markdown 即時預覽 — 標題、清單與自適應表格。*

[![數學與 Mermaid — 堆疊分式與終端機流程圖。](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*數學與 Mermaid — 堆疊分式與終端機流程圖。*

[![HTML Quick Look — 原生預覽視窗中的本機網頁。](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*HTML Quick Look — 原生預覽視窗中的本機網頁。*

[![Sixel 圖片 — 真實終端機圖形，受渲染器像素預算限制。](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Sixel 圖片 — 真實終端機圖形，受渲染器像素預算限制。*

[![Markdown 程式碼 — Python 圍欄程式碼區塊醒目提示，非獨立原始碼醒目提示。](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Markdown 程式碼 — Python 圍欄程式碼區塊醒目提示，非獨立原始碼醒目提示。*

真實 macOS 截圖；Graphite / Menlo。NASA 影像署名詳見 [截圖說明與素材署名](docs/gallery.md).

## 建議的完整功能組合

| 平台 | 終端機 / 桌面 | Space | 垃圾桶 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)（Sixel）或系統 Terminal（ANSI） | 系統 Quick Look | 系統垃圾桶 |
| Windows x64 | 支援 Sixel 的新版 [Windows Terminal](https://github.com/microsoft/terminal)；PowerShell 或 CMD | 獨立安裝 [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows 資源回收筒 |
| Linux x64 桌面 | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) 或其他終端機；點陣圖需要 Sixel 終端機 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux Headless / SSH | 現有終端機；建議 less | 文字與中繼資料備援 | gio / trash-put |

以上為建議組合，並非強制桌面環境。核心不依賴 Qt。不支援 Sixel 的終端機或 tmux/screen 使用 ANSI 備援。LFP_IMAGE_BACKEND 可設為 auto、ansi、sixel、none；僅在相容終端機強制 sixel。

## 建置與執行

目標平台需要 Go 1.26+、Rust/Cargo 1.94+；Windows 另需 MSVC 工具鏈。首次建置下載鎖定的相依套件。原始碼儲存庫已公開，可從 [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest) 下載預先建置的執行套件。

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
.\dist\NeoFM-win-x64\neofm.exe
~~~

**設定 QuickLook：** 啟動器預設查找 %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe。其他安裝方式可能使用不同路徑。在 PowerShell 設定並檢查完整 EXE 路徑；若不存在，請換成實際路徑：

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

CMD 對應方式：

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**注意：** 一律使用 neofm.exe 啟動，不直接執行 lf.exe。啟動器在自身程序補齊 System32、ComSpec 和 PATHEXT，不修改全域 PATH。即使從 PowerShell 5.1 或 7.x 啟動，lf 內部仍使用 CMD。若 Space 變成標記而非預覽，可能沒有載入自訂 lfrc。若出現 'exec: "cmd": executable file not found in %PATH%'，請離開後使用啟動器重啟並檢查 System32。先確認 QuickLook 可直接開啟測試 PDF，再對同一檔案按 Space。診斷記錄與 ANSI 備援：

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd 使用系統 Windows PowerShell 5.1 與原生 Shell API 移至資源回收筒；請以本機固定磁碟的暫存檔測試。QuickLook 格式外掛、編解碼器及終端機像素效果與核心分開。通用路徑仍待完整 GUI 迴歸測試，詳見驗證記錄。

## 快捷鍵

### 導覽與預覽

| 快捷鍵 | 功能 |
| :-- | :-- |
| `↑ / k · ↓ / j` | 向上/向下移動 |
| `← / h` | 返回父目錄 |
| `→ / l · Enter` | 進入目錄；一般檔案切換選取 |
| `Home / gg · End / G` | 首項/末項 |
| `Ctrl+U / Ctrl+D` | 向上/向下半頁 |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | 向上/向下整頁 |
| `Ctrl+Y / Ctrl+E` | 向上/向下捲動一行 |
| `H / M / L` | 可見清單頂部/中間/底部 |
| `gh` | 家目錄 |
| `[ / ]` | 上一個/下一個目錄歷程 |
| `Space` | 大型預覽 / 終端機閱讀 |
| `o` | 使用外部應用程式開啟 |

### 選取與檔案操作

| 快捷鍵 | 功能 |
| :-- | :-- |
| `Ctrl+Space` | 切換目前項目選取 |
| `u` | 取消所有目錄的選取（Unselect All） |
| `v` | 反選目前目錄 |
| `V` | 開始連續區域選取 |
| `yy` | 複製檔案至 NeoFM 緩衝區 |
| `x` | 剪下檔案至 NeoFM 緩衝區 |
| `p` | 貼上至目前目錄 |
| `c` | 清空複製/剪下緩衝區 |
| `dd` | 確認後移至垃圾桶 |
| `r` | 重新命名 |
| `t` | 切換檔案標籤 |

### 搜尋與歷程

| 快捷鍵 | 功能 |
| :-- | :-- |
| `/ / ?` | 向前/向後搜尋檔名 |
| `n / N` | 下一個/上一個搜尋結果 |
| `f / F` | 向前/向後快速尋找檔名 |
| `; / ,` | 下一個/上一個尋找結果 |
| `m / ' / "` | 儲存/載入/刪除目錄書籤 |

### 排序與顯示

| 快捷鍵 | 功能 |
| :-- | :-- |
| `zh` | 切換隱藏項目 |
| `zr` | 反轉排序 |
| `zn / zs / zt / za` | 隱藏資訊 / 大小 / 時間 / 兩者 |
| `sn / ss / st` | 依名稱 / 大小 / 修改時間排序 |
| `sa / sb / sc / se` | 依存取 / 建立 / 狀態變更時間 / 副檔名排序 |
| `Ctrl+L / Ctrl+R` | 重繪 / 重新載入 |

### 命令與模式

| 快捷鍵 | 功能 |
| :-- | :-- |
| `:` | 命令提示字元 |
| `$ / % / ! / &` | Shell / 管線 / 等待 / 非同步 Shell |
| `Ctrl+P / Ctrl+N` | 上一條/下一條命令歷程 |
| `q` | 離開 |

正常模式預設鍵位，區分大小寫。以 Enter 或 Ctrl+Space 選取多個檔案，按 yy（複製）或 x（剪下），進入目的目錄再按 p（貼上）。沒有選取時操作目前檔案；複製/剪下清除選取但保留內部檔案緩衝區。u 清除選取，c 清空緩衝區。yy 與 dd 為連續按兩次。dd 先確認並使用可恢復刪除助手，不主動改為永久刪除。Space 與 o 的檢視器依平台及工作階段而定。

Visual Mode：V 開始範圍選取，移動游標擴展範圍，V 接受，Esc 捨棄，o 交換端點。Visual Mode 保留上游 y/d 複製/剪下，與正常模式 yy/x 不同；建議先以 V 接受，再回正常模式使用 yy/x/dd。命令列模式：Enter 執行、Esc 取消、Tab 補全、上下箭頭查歷程、左右箭頭及 Home/End 移動游標。完整繼承的命令列編輯鍵位與命令請執行 neofm -doc，或參閱 [lf documentation](lf-upstream/doc.txt).

繼承滑鼠滾輪鍵位，但預設關閉滑鼠支援。

## 架構與驗證

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

一套共享原始碼，獨立平台啟動層。參閱[架構](docs/architecture.md)、[驗證記錄](docs/validation.md)及[上游來源](UPSTREAM.md)。

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

統一 Rust Release 在 Mac ARM64、Windows x64、Linux x64 原生編譯成功，各通過 4 項渲染器冒煙測試；Mac Adapter 19/19。先前部署也驗證實際平台的 Quick Look 與垃圾桶。適配 Writer 保留三項已知上游衍生迴歸失敗；不宣稱完整上游一致性、完整 GUI 迴歸、互動式 Sidebar Tabs 或 Headless SSH 下完整 PDF/Office 與高解析圖形能力。

## 致謝與授權

基於 [lf](https://github.com/gokcehan/lf)（MIT）和 [OpenAI Codex](https://github.com/openai/codex)（Apache-2.0）的擷取元件。QuickLook、Dzurkel 獨立安裝，不隨儲存庫打包。NeoFM 是獨立專案。參閱 [LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf 授權](lf-upstream/LICENSE)、[上游來源](UPSTREAM.md) 與 [品牌資源](assets/brand/)。

<p align="center"><sub>小工具、清楚的檔案、原生的預覽。</sub></p>
