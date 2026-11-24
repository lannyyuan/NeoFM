<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N 标志" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>现代终端文件管理器</strong><br>像 Vim 一样导航，像现代文档阅读器一样浏览。</p>

<p align="center">
  <a href="README.md">English</a> · 简体中文 ·
  <a href="#快速开始">快速开始</a> ·
  <a href="docs/architecture.md">架构</a> ·
  <a href="docs/validation.md">验收记录</a>
</p>

<p align="center">
  <img alt="版本 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM 将 lf 的轻量文件管理能力与 Codex TUI 中抽取的独立 Rust 渲染器结合，构建一个以键盘为中心、支持现代文档和图像预览的跨平台终端工作空间。大型文件预览交给各平台成熟的 Quick Look 工具。

**不需要 AI 服务，不重新发明终端，不依赖云端账号。** 文件、键盘与预览，回归工具本身。

## 双级预览

| 文件类型 | 一级：侧栏实时预览 | 二级：Space 大窗口 |
|:--|:--|:--|
| Markdown | 标题、列表、自适应表格、代码、数学公式、Mermaid | 系统预览器支持的格式 |
| 图片 | PNG、JPEG、GIF 首帧、WebP | 独立大图窗口 |
| 文本及代码 | 有界文本渲染 | 原生查看器 / 终端阅读器 |
| PDF、Office、媒体 | 元数据与预览提示 | 系统或第三方预览器 |

### 核心设计

- **Vim 工作流**：保留肌肉记忆，用少量按键完成文件操作。
- **真实 Markdown Writer**：支持中文、自适应表格、代码高亮、数学与 Mermaid。
- **终端图像**：支持 Sixel，普通终端自动降级为 ANSI。
- **可恢复删除**：dd 先确认，再调用系统回收站。
- **轻量原生**：核心不依赖重量级 GUI，充分复用各操作系统的成熟能力。

## 平台

| 环境 | Space 预览 | 当前状态 |
|:--|:--|:--|
| macOS ARM64 | 系统 Quick Look | 已部署 |
| Windows x64 | [QuickLook](https://github.com/QL-Win/QuickLook) | 已部署 |
| Linux x64 / DWM | [Dzurkel](https://github.com/EdinUser/dzurkel) | CachyOS 已部署 |
| Linux Headless / SSH | 终端文本与元数据降级 | Debian NUC 已部署 |

状态指已有部署版本的测试结果，详见[验收记录](docs/validation.md)。SSH 下完整 PDF / Office 阅读及交互式 Sidebar Tabs 尚未实现。

## 快速开始

在目标平台安装 **Go 1.26+、Rust/Cargo 1.94+**；Windows 还需要 MSVC 工具链。首次构建需要下载锁定依赖。当前为私有开发仓库，尚非公开包管理器发行版。

**macOS / Linux**

~~~sh
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
./scripts/build.sh
# macOS Apple Silicon
./dist/neofm-macos-arm64/bin/neofm
# Linux x86_64
./dist/neofm-linux-x64/bin/neofm
~~~

Linux 图形预览需要另外安装 Dzurkel；回收站使用 gio 或 trash-put，终端阅读建议安装 less。核心不依赖 Qt。需要统一命令时，可将对应构建的 bin/neofm 链接到 ~/.local/bin，注意不要覆盖已有入口。

**Windows（PowerShell）**

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

QuickLook 独立安装。默认查找标准用户安装目录，也可用 LFP_QUICKLOOK_EXE 指定完整路径。启动器只在自身进程范围内补齐 CMD 搜索路径，不修改全局环境变量。

## 快捷键

| 按键 | 功能 | 按键 | 功能 |
|:--|:--|:--|:--|
| h / j / k / l | Vim 导航 | Space | Quick Look |
| Enter | 标记文件 / 进入目录 | Ctrl+Space | 切换标记 |
| yy | 复制 | p | 粘贴 |
| x | 剪切 | dd | 确认后移入回收站 |
| o | 外部打开 | zh | 切换隐藏项 |
| gg / G | 顶部 / 底部 | / | 搜索 |
| r | 重命名 | q | 退出 |

默认显示隐藏文件与目录。存在标记时，文件操作作用于标记集合，否则作用于当前文件。不主动将回收站删除降级为永久删除。

## 架构

~~~text
                    NeoFM 启动器
                          │
                     lf 导航引擎
                    ┌─────┴─────┐
                    │           │
                侧栏预览       Space
                    │           │
              Rust Renderer    原生 Quick Look
            ┌───────┴──────┐    / 终端降级
         Markdown        图像
         公式/Mermaid    Sixel/ANSI
~~~

一套共享源码，少量平台适配层。详见[架构说明](docs/architecture.md)与[上游来源](UPSTREAM.md)。

## 开发与验证

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

仓库保留固定版本 lf、抽取后的 Writer、Preview Adapter、各平台脚本、合成样例、构建脚本及 [品牌资源](assets/brand/)。适配后的 Writer 仍有三个已知上游派生回归失败；Adapter 通过不等同于完整 Codex 一致性。

## 致谢与许可

基于 [lf](https://github.com/gokcehan/lf) 和 [OpenAI Codex](https://github.com/openai/codex) 的渲染组件，外部预览复用 QuickLook / Dzurkel。NeoFM 是独立项目，并非上游官方产品。参见 [LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf MIT 许可](lf-upstream/LICENSE) 与 [上游来源说明](UPSTREAM.md)。

<p align="center"><sub>小工具，清晰的文件，原生的预览。</sub></p>
