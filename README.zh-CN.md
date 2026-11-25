![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>现代终端文件管理器</strong><br>像 Vim 一样导航，像现代文档阅读器一样浏览。</p>

[English](README.md) · **简体中文** · [繁體中文](README.zh-TW.md) · [日本語](README.ja.md) · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM 将成熟的 **lf** 文件管理器与从 Codex TUI 抽取的独立 Rust Renderer 结合，为轻量、键盘优先的终端工作空间提供丰富的文档预览。大窗口预览交给各平台成熟的工具。不依赖 AI 服务，不重新发明终端，不强制使用云端账号。

[![按 Space，在 macOS 原生 Quick Look 中浏览真实 PDF。NASA《Earth as Art》— 贝尔彻群岛。](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*按 Space，在 macOS 原生 Quick Look 中浏览真实 PDF。NASA《Earth as Art》— 贝尔彻群岛。*

## 设计思想：少，但更好

- **Unix KISS（Keep It Simple, Stupid）原则：** 每个组件只专注于自己的职责。lf 负责导航和文件操作，Renderer 负责有界终端渲染，原生工具负责大窗口预览。组合成熟部件，而非重复造轮子。
- **受 [suckless](https://suckless.org/) 极简哲学启发：** 小而可理解的组件、少量依赖、键盘优先、有意识的克制。NeoFM 并非 suckless 官方项目。
- **以减法形成品味：** 不做启动画面、装饰性 Dashboard、多余按钮、内嵌浏览器或 AI Harness。Logo 用于项目资料，不占据终端空间。只有显著改善高频工作流的功能才值得加入。
- **统一行为、原生实现：** macOS、Windows、Linux 使用一致布局和 Vim Keymap；预览和回收站复用各系统的成熟能力。本地文件保持本地。

## 一个布局，三个平台

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

默认比例 **1:2:4**：左侧父目录、中间当前目录与文件选取、右侧实时预览。最大空间留给内容，不新增编辑器或常驻工具栏。默认显示隐藏项，zh 切换。布局由各平台启动配置控制，不引入新的终端 UI 框架。

| 文件 | 侧栏实时预览 | Space 大窗口 |
|:--|:--|:--|
| Markdown | 标题、列表、自适应表格、中日韩文字、代码高亮、公式、Mermaid | 系统支持的原生查看器 |
| 图片 | PNG、JPEG、GIF 首帧、WebP；Sixel 或 ANSI | 外部大图窗口 |
| 文本 / 代码 | 有界文本渲染 | 原生查看器 / 终端阅读器 |
| PDF、Office、媒体 | 元数据与预览提示 | 取决于外部应用的格式支持 |

## SSH 远程预览 — 无需图形桌面

NeoFM 不只是本地文件管理器，也是远程文档浏览器。在 Linux 服务器上通过普通 SSH 运行 NeoFM，无需 X11 转发、远程桌面或将文件下载到本机，就能完成导航、文件操作和大部分日常侧栏预览工作。Renderer 在远端执行，终端负责显示输出。

| 文件类型 | SSH 实时侧栏 |
| :-- | :-- |
| Markdown：标题、列表、表格、中日韩文字、代码块、公式和 Mermaid | 富文本渲染 |
| UTF-8 文本、源码、JSON、日志、HTML 源码 | 文本预览 |
| PNG、JPEG、GIF 首帧、WebP | 在渲染限制内使用 Sixel 或 ANSI 色块 |
| PDF、Office、媒体与压缩包 | 类型/大小或二进制元数据，非完整文档/媒体渲染 |

在 Linux SSH 或无桌面会话中，Space 自动进入终端阅读降级路径（有 less 时使用 less -R -X），可阅读渲染后的 Markdown、有界文本或元数据。图片在侧栏中渲染，终端阅读器不提供放大后的图形图片。完整 PDF/Office、渲染后的 HTML 和多媒体 Quick Look 需要合适的本地图形查看器。macOS 和 Windows 目前仍保留各自的 Space 绑定；这里的自动 SSH 阅读路径指 Linux。图像自动识别依赖终端能力及转发的环境变量；tmux/screen 使用 ANSI。只在兼容终端强制 Sixel。输出有大小限制，SSH 高清图形的实际视觉效果尚未完整验收。

已通过真实 SSH 连接 Linux 验证 Markdown、公式/Mermaid、Sixel 帧生成、ANSI 图像和终端阅读路径（见验收记录）。 [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

## NeoFM 实际运行展示

[![Markdown 实时预览 — 标题、列表与自适应表格。](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Markdown 实时预览 — 标题、列表与自适应表格。*

[![数学与 Mermaid — 堆叠分式与终端流程图。](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*数学与 Mermaid — 堆叠分式与终端流程图。*

[![HTML Quick Look — 原生预览窗口中的本地网页。](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*HTML Quick Look — 原生预览窗口中的本地网页。*

[![Sixel 图片 — 真实终端图像，受 Renderer 像素预算限制。](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Sixel 图片 — 真实终端图像，受 Renderer 像素预算限制。*

[![Markdown 代码 — Python 围栏代码块语法高亮，非独立源码高亮。](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Markdown 代码 — Python 围栏代码块语法高亮，非独立源码高亮。*

真实 macOS 截图；Graphite / Menlo。NASA 影像署名详见 [截图说明与素材署名](docs/gallery.md).

## 推荐的全功能组合

| 平台 | Terminal / 桌面 | Space | 回收站 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)（Sixel）或系统 Terminal（ANSI） | 系统 Quick Look | 系统 Trash |
| Windows x64 | 支持 Sixel 的新版 [Windows Terminal](https://github.com/microsoft/terminal)；PowerShell 或 CMD | 独立安装 [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows Recycle Bin |
| Linux x64 桌面 | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) 或其他终端；位图需要 Sixel 终端 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux Headless / SSH | 现有终端；建议 less | 文本与元数据降级 | gio / trash-put |

这些是推荐组合，而非强制桌面环境。核心不依赖 Qt。不支持 Sixel 的终端或 tmux/screen 使用 ANSI 降级。LFP_IMAGE_BACKEND 可设为 auto、ansi、sixel、none，仅在支持的终端强制 sixel。

## 构建与运行

目标平台需要 Go 1.26+、Rust/Cargo 1.94+；Windows 还需 MSVC 工具链。首次构建下载锁定依赖。源代码仓库已公开，可从 [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest) 下载预编译运行包。

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

使用启动器而非裸 lf。若需用户级统一命令，可将对应构建的 bin/neofm 链接到 ~/.local/bin，不覆盖已有入口。保留自己的终端和窗口管理器。

**macOS：** 不需额外 Quick Look 应用。启动器调用 /usr/bin/qlmanage -p；iTerm2 推荐用于 Sixel，系统 Terminal 可以使用 ANSI。

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux：** 图形预览需独立安装 Dzurkel 及兼容 Qt 依赖。DWM/st 未支持 Sixel 时仍可用 ANSI。检查以下工具，缺少时用发行版包管理器安装。仅在本地图形会话调用 Dzurkel；普通 SSH 使用终端降级。若 Qt 版本冲突，可单独配置兼容的隔离 Dzurkel Runtime，而不是为 NeoFM 升级整个桌面。

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows：完整配置方法

安装支持 Sixel 的 Windows Terminal、[QuickLook](https://github.com/QL-Win/QuickLook) 及上述原生编译工具。QuickLook 是 Windows 第三方应用，不是系统自带组件。安装后在当前桌面会话启动一次，再构建 NeoFM。

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.exe
~~~

**配置 QuickLook：** 启动器默认查找 %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe。其他安装方式可能使用不同路径。在 PowerShell 中设置并检查完整 EXE 路径，不存在则替换为实际路径：

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

CMD 对应方法：

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**注意：** 始终通过 neofm.exe 启动，不直接运行 lf.exe。启动器在自身进程内补齐 System32、ComSpec 和 PATHEXT，不修改全局 PATH。即使从 PowerShell 5.1 或 7.x 启动，lf 内部仍用 CMD。若 Space 变成标记而非预览，可能未加载自定义 lfrc；若出现 'exec: "cmd": executable file not found in %PATH%'，退出后通过启动器重启并检查 System32。先确认 QuickLook 能直接打开测试 PDF，再在 NeoFM 中对同一文件按 Space。诊断日志与 ANSI 降级：

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd 通过系统 Windows PowerShell 5.1 与 Shell API 调用回收站；用本地固定磁盘的临时文件确认。QuickLook 格式插件、编解码器及终端像素效果与核心功能分开。通用化路径仍待完整 GUI 回归，详见验收记录。

## 快捷键

### 导航与预览

| 快捷键 | 功能 |
| :-- | :-- |
| `↑ / k · ↓ / j` | 向上/向下移动 |
| `← / h` | 返回父目录 |
| `→ / l · Enter` | 进入目录；普通文件切换选中状态 |
| `Home / gg · End / G` | 首项/末项 |
| `Ctrl+U / Ctrl+D` | 向上/向下半页 |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | 向上/向下整页 |
| `Ctrl+Y / Ctrl+E` | 向上/向下滚动一行 |
| `H / M / L` | 可见列表顶部/中间/底部 |
| `gh` | 主目录 |
| `[ / ]` | 上一个/下一个目录历史 |
| `Space` | 大窗口预览 / 终端阅读 |
| `o` | 使用外部应用打开 |

### 选择与文件操作

| 快捷键 | 功能 |
| :-- | :-- |
| `Ctrl+Space` | 切换当前项选中状态 |
| `u` | 取消所有目录中的选择（Unselect All） |
| `v` | 反选当前目录 |
| `V` | 开始连续区域选择 |
| `yy` | 复制文件到 NeoFM 缓冲区 |
| `x` | 剪切文件到 NeoFM 缓冲区 |
| `p` | 粘贴至当前目录 |
| `c` | 清空复制/剪切缓冲区 |
| `dd` | 确认后移入回收站 |
| `r` | 重命名 |
| `t` | 切换文件标签 |

### 搜索与历史

| 快捷键 | 功能 |
| :-- | :-- |
| `/ / ?` | 向前/向后搜索文件名 |
| `n / N` | 下一个/上一个搜索结果 |
| `f / F` | 向前/向后快速查找文件名 |
| `; / ,` | 下一个/上一个查找结果 |
| `m / ' / "` | 保存/读取/删除目录书签 |

### 排序与显示

| 快捷键 | 功能 |
| :-- | :-- |
| `zh` | 切换隐藏项 |
| `zr` | 反转排序 |
| `zn / zs / zt / za` | 隐藏信息 / 大小 / 时间 / 两者 |
| `sn / ss / st` | 按名称 / 大小 / 修改时间排序 |
| `sa / sb / sc / se` | 按访问 / 创建 / 状态变更时间 / 扩展名排序 |
| `Ctrl+L / Ctrl+R` | 重绘 / 重新加载 |

### 命令与模式

| 快捷键 | 功能 |
| :-- | :-- |
| `:` | 命令提示符 |
| `$ / % / ! / &` | Shell / 管道 / 等待 / 异步 Shell |
| `Ctrl+P / Ctrl+N` | 上一条/下一条命令历史 |
| `q` | 退出 |

以上为正常模式默认键位，区分大小写。用 Enter 或 Ctrl+Space 选择多个文件，按 yy（复制）或 x（剪切），进入目标目录后按 p（粘贴）。没有已选项时，复制/剪切针对当前文件；操作后取消标记，但保留内部文件缓冲区。u 清除选择，c 清空文件缓冲区。yy 和 dd 是连续按两次，不是组合键。dd 必须确认并调用可恢复删除助手，不主动降级为永久删除。Space 和 o 的查看器取决于平台及会话环境。

Visual Mode：V 开始区域选择，移动光标扩展区域，再按 V 接受，Esc 放弃，o 交换区域端点。Visual Mode 保留上游 y/d 复制/剪切键位，与正常模式 yy/x 不同；建议先 V 接受，再回正常模式使用 yy/x/dd。命令行模式：Enter 执行，Esc 取消，Tab 补全，上下箭头查历史，左右箭头及 Home/End 移动光标。完整继承的命令行编辑键位与命令可运行 neofm -doc，或查看 [lf documentation](lf-upstream/doc.txt).

继承鼠标滚轮键位，但默认关闭鼠标支持。

## 架构与验证

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

一套共享源码，独立平台启动层。详见[架构](docs/architecture.md)、[验收记录](docs/validation.md)和[上游来源](UPSTREAM.md)。

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

统一 Rust Release 已在 Mac ARM64、Windows x64、Linux x64 原生编译并各通过 4 项 Renderer 冒烟测试；Mac Adapter 19/19。此前部署还验证了实际平台的 Quick Look 和回收站。适配 Writer 保留三个已知上游派生回归失败；不宣称完整上游一致性、完整 GUI 回归、交互式 Sidebar Tabs 或 Headless SSH 下完整 PDF/Office 与高清图形能力。

## 致谢与许可

NeoFM 离不开开源社区的长期积累。衷心感谢以下项目的维护者和贡献者：

- [lf](https://github.com/gokcehan/lf) — gokcehan & contributors — 文件管理器的基础、导航与文件操作。
- [OpenAI Codex](https://github.com/openai/codex) — OpenAI & contributors — 抽取并适配的 Markdown Writer、数学公式、Mermaid 和终端图像渲染组件。
- [Ratatui](https://github.com/ratatui/ratatui) — 终端渲染基础，包括通过 Codex 继承的代码。
- [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) / [Dzurkel](https://github.com/EdinUser/dzurkel) — 分别提供 Windows 和 Linux 的原生大窗口预览集成（需单独安装）。

同时感谢相关基础库的维护者，包括 [tcell](https://github.com/gdamore/tcell), [pulldown-cmark](https://crates.io/crates/pulldown-cmark), [syntect](https://crates.io/crates/syntect), [two-face](https://crates.io/crates/two-face), [image](https://crates.io/crates/image).

基于 [lf](https://github.com/gokcehan/lf)（MIT）和 [OpenAI Codex](https://github.com/openai/codex)（Apache-2.0）的抽取组件。QuickLook、Dzurkel 单独安装，不随仓库打包。NeoFM 是独立项目。参见 [LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf 许可](lf-upstream/LICENSE)、[上游来源](UPSTREAM.md) 与 [品牌资源](assets/brand/)。

<p align="center"><sub>小工具，清晰的文件，原生的预览。</sub></p>
