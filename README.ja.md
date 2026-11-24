<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/brand/png/neofm-horizontal-white.png">
    <img alt="NeoFM — Panel N" src="assets/brand/png/neofm-horizontal-black.png" width="420">
  </picture>
</p>

<p align="center"><strong>モダンなターミナルファイルマネージャー</strong><br>Vim のように移動し、モダンなドキュメントビューアーのように読む。</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · **日本語** · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.2.1" src="https://img.shields.io/badge/version-0.2.1-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM は、実績のある **lf** ファイルマネージャーと Codex TUI から抽出した独立した Rust レンダラーを組み合わせ、軽量でキーボード中心のターミナル環境に豊かな文書プレビューを提供します。大型プレビューは各 OS の既存ツールに任せます。AI サービス、新しいターミナルエミュレーター、必須のクラウドアカウントは不要です。

## 設計思想 — より少なく、より良く

- **Unix KISS（Keep It Simple, Stupid）：** コンポーネントの責務を限定します。lf は移動とファイル操作、レンダラーは出力範囲を制限した端末描画、ネイティブツールは大型プレビューを担当します。既存の優れたツールを組み合わせ、車輪を再発明しません。
- **[suckless](https://suckless.org/) の思想：** 小さく理解しやすい部品、少ない依存関係、キーボード優先、意識的な抑制。NeoFM はこの哲学に着想を得た独立プロジェクトです。
- **引き算の美学：** スプラッシュ画面、装飾的なダッシュボード、不要なボタン、内蔵ブラウザー、AI Harness を追加しません。ロゴはプロジェクト資料に使い、端末の作業領域には表示しません。頻繁な作業を改善する機能だけを加えます。
- **統一された操作、ネイティブな実装：** macOS、Windows、Linux で同じレイアウトと Vim キー操作を使い、各 OS のプレビューとゴミ箱を活用します。ローカルファイルはローカルに留まります。

## 3 プラットフォーム、1 つのレイアウト

~~~text
┌────────────┬────────────────────────┬────────────────────────────────────────────────┐
│     1      │           2            │                       4                        │
│     ↑      │           →            │                       ◇                        │
└────────────┴────────────────────────┴────────────────────────────────────────────────┘
                 set ratios 1:2:4
~~~

既定の比率は **1:2:4**。左は親ディレクトリ、中央は現在のディレクトリと選択、右はライブプレビューです。コンテンツに最大の領域を割り当て、別のエディターや常設ツールバーを設けません。隠しファイルは既定で表示し、zh で切り替えます。レイアウトは各プラットフォームのランチャーで設定し、新しい端末 UI フレームワークは導入しません。

| 内容 | ライブプレビュー | Space — 大型プレビュー |
|:--|:--|:--|
| Markdown | 見出し、リスト、可変幅テーブル、CJK、コード強調、数式、Mermaid | 対応するネイティブビューアー |
| 画像 | PNG、JPEG、GIF の先頭フレーム、WebP；Sixel または ANSI | 外部の大型ビューアー |
| テキスト / コード | 範囲を制限したテキスト描画 | ネイティブビューアー / 端末リーダー |
| PDF、Office、メディア | メタデータとプレビュー案内 | 外部アプリの対応形式に依存 |

## 推奨するフル機能構成

| プラットフォーム | ターミナル / デスクトップ | Space | ゴミ箱 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)（Sixel）または Terminal（ANSI） | 内蔵 Quick Look | システムのゴミ箱 |
| Windows x64 | Sixel 対応の新しい [Windows Terminal](https://github.com/microsoft/terminal)；PowerShell または CMD | 別途導入する [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows ごみ箱 |
| Linux x64 デスクトップ | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) など；ビットマップ表示には Sixel 対応端末 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux ヘッドレス / SSH | 既存の端末；less を推奨 | テキスト / メタデータ | gio / trash-put |

推奨構成であり、特定のデスクトップ環境は必須ではありません。コアは Qt に依存しません。Sixel 非対応端末や tmux/screen では ANSI にフォールバックします。LFP_IMAGE_BACKEND は auto、ansi、sixel、none を指定可能です。sixel の強制は対応端末に限定してください。

## ビルドと起動

対象プラットフォームに Go 1.26+、Rust/Cargo 1.94+ が必要です。Windows には MSVC ビルドツールも必要です。初回ビルドは固定された依存関係をダウンロードします。プライベート開発リポジトリであり公開パッケージではないため、クローンには認証済み GitHub アクセスが必要です。

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

lf を直接実行せずランチャーを使用します。ユーザー用コマンドが必要なら、既存の入口を上書きせず対象ビルドの bin/neofm を ~/.local/bin にリンクします。既存のターミナルとウィンドウマネージャーはそのまま使えます。

**macOS：** 追加の Quick Look アプリは不要です。ランチャーは /usr/bin/qlmanage -p を使用します。Sixel には iTerm2 を推奨し、システム Terminal では ANSI が使用できます。

~~~sh
# Optional: prefer Sixel only in a compatible terminal
LFP_IMAGE_BACKEND=auto ./dist/neofm-macos-arm64/bin/neofm
# Inspect the native preview command
command -v qlmanage
~~~

**Linux：** グラフィカルプレビューには Dzurkel と互換性のある Qt を別途導入します。Sixel 非対応の DWM/st でも ANSI は動作します。以下のツールを確認し、不足分はディストリビューションのパッケージマネージャーで導入してください。Dzurkel はローカルのグラフィカルセッションのみで使用し、通常の SSH では端末にフォールバックします。Qt が非互換なら、デスクトップ全体を更新する代わりに互換性のある隔離 Dzurkel ランタイムを用意します。

~~~sh
command -v dzurkel
command -v gio || command -v trash-put
command -v less
command -v xdg-open
~~~

### Windows — 完全な構成

Sixel 対応 Windows Terminal、[QuickLook](https://github.com/QL-Win/QuickLook)、上記のネイティブビルドツールを導入します。QuickLook は Windows の標準機能ではなくサードパーティーアプリです。インストール後、デスクトップセッションで一度起動してから NeoFM をビルドします。

~~~powershell
git clone https://github.com/lannyyuan/NeoFM.git
cd NeoFM
powershell -ExecutionPolicy Bypass -File .\scripts\build.ps1
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**QuickLook の設定：** 既定の場所は %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe です。インストール方法によって異なります。PowerShell で EXE のフルパスを設定・確認し、必要なら実際のパスに置き換えます。

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.cmd
~~~

CMD の場合：

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.cmd
~~~

**重要：** lf.exe ではなく必ず neofm.cmd から起動します。ランチャーは自身のプロセスで System32、ComSpec、PATHEXT を補い、グローバル PATH は変更しません。PowerShell 5.1/7.x から起動しても lf 内部のシェルは CMD です。Space がプレビューではなくマークになる場合はカスタム lfrc が読み込まれていない可能性があります。'exec: "cmd": executable file not found in %PATH%' が出たら、ランチャーで再起動し System32 を確認します。まずテスト PDF を QuickLook で直接開き、同じファイルを Space で確認します。診断ログと ANSI フォールバック：

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.cmd
~~~

dd は Windows PowerShell 5.1 とネイティブ Shell API を利用します。ローカル固定ドライブの使い捨てファイルで確認してください。QuickLook の形式プラグイン、コーデック、端末の画質はコアと別です。汎用化したパスの完全な GUI 回帰テストは未完了です。

## キー操作

| キー | 操作 | キー | 操作 |
|:--|:--|:--|:--|
| `h / j / k / l` | 移動 | `Space` | Quick Look |
| `Enter` | ファイルのマーク切替 / フォルダーに入る | `Ctrl+Space` | マーク切替 |
| `yy` | コピー | `p` | 貼り付け |
| `x` | 切り取り | `dd` | 確認後ゴミ箱へ |
| `o` | 外部アプリで開く | `zh` | 隠し項目切替 |
| `gg / G` | 先頭 / 末尾 | `/` | 検索 |
| `r` | 名前変更 | `q` | 終了 |

マークがあればその集合を、なければ現在のファイルを対象にします。dd は確認を求め、ゴミ箱が利用できない場合に意図的な完全削除へ切り替えません。

## アーキテクチャと検証

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

共有ソースツリーと独立したプラットフォームランチャー。[アーキテクチャ](docs/architecture.md)、[検証](docs/validation.md)、[上流の出典](UPSTREAM.md) を参照してください。

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

統一 Rust Release は Mac ARM64、Windows x64、Linux x64 でネイティブビルドと各 4 項目のレンダラースモークテストに成功しました。Mac Adapter は 19/19。以前の展開でネイティブプレビューとゴミ箱も確認しました。適応した Writer には既知の上流由来の失敗が 3 件あります。完全な上流互換、GUI 全体の再試験、対話型 Sidebar Tabs、ヘッドレス SSH での完全な PDF/Office や高解像度画像表示は保証していません。

## 謝辞とライセンス

[lf](https://github.com/gokcehan/lf)（MIT）と [OpenAI Codex](https://github.com/openai/codex)（Apache-2.0）の抽出コンポーネントを使用しています。QuickLook、Dzurkel は別途導入し、同梱しません。NeoFM は独立したプロジェクトです。[LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf ライセンス](lf-upstream/LICENSE)、[UPSTREAM.md](UPSTREAM.md)、[ブランド素材](assets/brand/) を参照してください。

<p align="center"><sub>小さなツール、明快なファイル、ネイティブプレビュー。</sub></p>
