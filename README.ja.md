![NeoFM — Panel N](assets/brand/png/neofm-logo-universal.png)

<p align="center"><strong>モダンなターミナルファイルマネージャー</strong><br>Vim のように移動し、モダンなドキュメントビューアーのように読む。</p>

[English](README.md) · [简体中文](README.zh-CN.md) · [繁體中文](README.zh-TW.md) · **日本語** · [한국어](README.ko.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Português](README.pt.md) · [Italiano](README.it.md) · [Русский](README.ru.md)

<p align="center">
  <img alt="Version 0.3.0" src="https://img.shields.io/badge/version-0.3.0-171717?style=flat-square">
  <img alt="macOS · Windows · Linux" src="https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-555555?style=flat-square">
  <img alt="Rust + Go" src="https://img.shields.io/badge/core-Rust%20%2B%20Go-444444?style=flat-square">
</p>

---

NeoFM は、実績のある **lf** ファイルマネージャーと Codex TUI から抽出した独立した Rust レンダラーを組み合わせ、軽量でキーボード中心のターミナル環境に豊かな文書プレビューを提供します。大型プレビューは各 OS の既存ツールに任せます。AI サービス、新しいターミナルエミュレーター、必須のクラウドアカウントは不要です。

[![Space で実際の PDF を macOS Quick Look に表示。NASA『Earth as Art』— ベルチャー諸島。](assets/screenshots-r3/showcase/06-quicklook-pdf.png)](assets/screenshots-r3/raw/06-quicklook-pdf.png)

*Space で実際の PDF を macOS Quick Look に表示。NASA『Earth as Art』— ベルチャー諸島。*

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

## SSH でも豊かなプレビュー — デスクトップ不要

NeoFM はローカルファイルマネージャーであると同時に、リモート文書ビューアーです。Linux サーバー上で通常の SSH 接続から起動すれば、X11 転送、リモートデスクトップ、ファイルのダウンロードなしに、移動、ファイル操作、日常的なサイドバー表示の大部分を利用できます。レンダラーはリモート側で動作し、端末に出力します。

| 形式 | SSH のライブプレビュー |
| :-- | :-- |
| Markdown：見出し、リスト、表、CJK、コードブロック、数式、Mermaid | リッチレンダリング |
| UTF-8 テキスト、ソース、JSON、ログ、HTML ソース | テキスト表示 |
| PNG、JPEG、GIF 先頭フレーム、WebP | 描画制限内の Sixel または ANSI 色ブロック |
| PDF、Office、メディア、アーカイブ | 種類・サイズまたはバイナリのメタデータ。完全な内容表示ではない |

Linux の SSH / ヘッドレス環境で Space は端末リーダー（less があれば less -R -X）に切り替わり、Markdown、範囲を制限したテキスト、メタデータを表示します。画像はサイドバーで描画し、リーダーでは拡大画像を表示しません。完全な PDF/Office、HTML 描画、マルチメディア Quick Look には適切なローカル GUI が必要です。macOS と Windows は現時点で固有の Space 割り当てを保持し、この自動 SSH リーダーは Linux の機能です。画像の自動判定は端末と転送環境に依存し、tmux/screen は ANSI を使用します。対応端末のみ Sixel を強制してください。出力には制限があり、高解像度 SSH 描画の視覚検証は未完了です。

実際の Linux SSH 接続で Markdown、数式/Mermaid、Sixel フレーム生成、ANSI 画像、端末リーダー経路を確認済みです（検証記録参照）。 [↗](docs/validation.md#ssh-preview-verification)

~~~sh
ssh user@linux-server
neofm
# Optional: reliable color-block preview in any compatible color terminal
LFP_IMAGE_BACKEND=ansi neofm
~~~

## NeoFM の実際の画面

[![Markdown ライブプレビュー — 見出し、リスト、可変幅テーブル。](assets/screenshots-r3/showcase/01-markdown.png)](assets/screenshots-r3/raw/01-markdown.png)

*Markdown ライブプレビュー — 見出し、リスト、可変幅テーブル。*

[![数式と Mermaid — 分数の組版と端末のフローチャート。](assets/screenshots-r3/showcase/02-math-mermaid.png)](assets/screenshots-r3/raw/02-math-mermaid.png)

*数式と Mermaid — 分数の組版と端末のフローチャート。*

[![HTML Quick Look — ネイティブプレビューに表示したローカル HTML。](assets/screenshots-r3/showcase/05-quicklook-html.png)](assets/screenshots-r3/raw/05-quicklook-html.png)

*HTML Quick Look — ネイティブプレビューに表示したローカル HTML。*

[![Sixel 画像 — 実際の端末画像描画。レンダラーの画素数制限あり。](assets/screenshots-r3/showcase/03-image.png)](assets/screenshots-r3/raw/03-image.png)

*Sixel 画像 — 実際の端末画像描画。レンダラーの画素数制限あり。*

[![Markdown コード — Python のフェンス付きコードブロックの構文強調。単独のソースファイルではありません。](assets/screenshots-r3/showcase/04-source-code.png)](assets/screenshots-r3/raw/04-source-code.png)

*Markdown コード — Python のフェンス付きコードブロックの構文強調。単独のソースファイルではありません。*

実際の macOS 画面。Graphite / Menlo。NASA 画像の出典は [撮影記録と画像クレジット](docs/gallery.md).

## 推奨するフル機能構成

| プラットフォーム | ターミナル / デスクトップ | Space | ゴミ箱 |
|:--|:--|:--|:--|
| macOS ARM64 | [iTerm2](https://iterm2.com/)（Sixel）または Terminal（ANSI） | 内蔵 Quick Look | システムのゴミ箱 |
| Windows x64 | Sixel 対応の新しい [Windows Terminal](https://github.com/microsoft/terminal)；PowerShell または CMD | 別途導入する [QL-Win QuickLook](https://github.com/QL-Win/QuickLook) | Windows ごみ箱 |
| Linux x64 デスクトップ | [DWM](https://dwm.suckless.org/) / [st](https://st.suckless.org/) など；ビットマップ表示には Sixel 対応端末 | [Dzurkel](https://github.com/EdinUser/dzurkel) | gio / trash-put |
| Linux ヘッドレス / SSH | 既存の端末；less を推奨 | テキスト / メタデータ | gio / trash-put |

推奨構成であり、特定のデスクトップ環境は必須ではありません。コアは Qt に依存しません。Sixel 非対応端末や tmux/screen では ANSI にフォールバックします。LFP_IMAGE_BACKEND は auto、ansi、sixel、none を指定可能です。sixel の強制は対応端末に限定してください。

## ビルドと起動

対象プラットフォームに Go 1.26+、Rust/Cargo 1.94+ が必要です。Windows には MSVC ビルドツールも必要です。初回ビルドは固定された依存関係をダウンロードします。ビルド済みパッケージは [GitHub Releases](https://github.com/lannyyuan/NeoFM/releases/latest) から入手できます。ソースリポジトリは公開されています。

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
.\dist\NeoFM-win-x64\neofm.exe
~~~

**QuickLook の設定：** 既定の場所は %LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe です。インストール方法によって異なります。PowerShell で EXE のフルパスを設定・確認し、必要なら実際のパスに置き換えます。

~~~powershell
$env:LFP_QUICKLOOK_EXE = Join-Path $env:LOCALAPPDATA "Programs\QuickLook\QuickLook.exe"
Test-Path -LiteralPath $env:LFP_QUICKLOOK_EXE
# If False, replace the value with the actual full EXE path.
# Optional direct QuickLook test from the repository root (uncomment):
# & $env:LFP_QUICKLOOK_EXE (Resolve-Path ".\dist\NeoFM-win-x64\samples\fixtures-quicklook\01 sample 中文.pdf").Path
# Run from the NeoFM repository root:
.\dist\NeoFM-win-x64\neofm.exe
~~~

CMD の場合：

~~~bat
set "LFP_QUICKLOOK_EXE=%LOCALAPPDATA%\Programs\QuickLook\QuickLook.exe"
if exist "%LFP_QUICKLOOK_EXE%" echo QuickLook OK
where cmd.exe
.\dist\NeoFM-win-x64\neofm.exe
~~~

**重要：** lf.exe ではなく必ず neofm.exe から起動します。ランチャーは自身のプロセスで System32、ComSpec、PATHEXT を補い、グローバル PATH は変更しません。PowerShell 5.1/7.x から起動しても lf 内部のシェルは CMD です。Space がプレビューではなくマークになる場合はカスタム lfrc が読み込まれていない可能性があります。'exec: "cmd": executable file not found in %PATH%' が出たら、ランチャーで再起動し System32 を確認します。まずテスト PDF を QuickLook で直接開き、同じファイルを Space で確認します。診断ログと ANSI フォールバック：

~~~powershell
Get-Content "$env:LOCALAPPDATA\LFpreview\data\lfwin.log" -Tail 30
$env:LFP_IMAGE_BACKEND = "ansi"
.\dist\NeoFM-win-x64\neofm.exe
~~~

dd は Windows PowerShell 5.1 とネイティブ Shell API を利用します。ローカル固定ドライブの使い捨てファイルで確認してください。QuickLook の形式プラグイン、コーデック、端末の画質はコアと別です。汎用化したパスの完全な GUI 回帰テストは未完了です。

## キー操作

### 移動とプレビュー

| キー | 操作 |
| :-- | :-- |
| `↑ / k · ↓ / j` | 上下移動 |
| `← / h` | 親ディレクトリへ |
| `→ / l · Enter` | ディレクトリに入る／ファイルのマーク切替 |
| `Home / gg · End / G` | 先頭／末尾 |
| `Ctrl+U / Ctrl+D` | 半ページ上／下 |
| `PageUp / Ctrl+B · PageDown / Ctrl+F` | ページ上／下 |
| `Ctrl+Y / Ctrl+E` | 1 行上／下にスクロール |
| `H / M / L` | 画面上部／中央／下部 |
| `gh` | ホームディレクトリ |
| `[ / ]` | 前／次のディレクトリ履歴 |
| `Space` | 大型プレビュー／端末リーダー |
| `o` | 外部アプリで開く |

### 選択とファイル操作

| キー | 操作 |
| :-- | :-- |
| `Ctrl+Space` | 現在項目のマーク切替 |
| `u` | 全ディレクトリの選択解除 |
| `v` | 現在ディレクトリの選択反転 |
| `V` | 範囲選択開始 |
| `yy` | NeoFM のバッファーにコピー |
| `x` | NeoFM のバッファーに切り取り |
| `p` | 現在ディレクトリへ貼り付け |
| `c` | ファイルバッファー消去 |
| `dd` | 確認後ゴミ箱へ |
| `r` | 名前変更 |
| `t` | タグ切替 |

### 検索と履歴

| キー | 操作 |
| :-- | :-- |
| `/ / ?` | ファイル名前方／後方検索 |
| `n / N` | 次／前の検索結果 |
| `f / F` | 前方／後方クイック検索 |
| `; / ,` | 次／前のクイック検索結果 |
| `m / ' / "` | ディレクトリブックマーク保存／読込／削除 |

### 並べ替えと表示

| キー | 操作 |
| :-- | :-- |
| `zh` | 隠し項目切替 |
| `zr` | ソート順反転 |
| `zn / zs / zt / za` | 情報非表示／サイズ／時刻／両方 |
| `sn / ss / st` | 名前／サイズ／更新時刻順 |
| `sa / sb / sc / se` | アクセス／作成／状態変更時刻／拡張子順 |
| `Ctrl+L / Ctrl+R` | 再描画／再読込 |

### コマンドとモード

| キー | 操作 |
| :-- | :-- |
| `:` | コマンド入力 |
| `$ / % / ! / &` | シェル／パイプ／待機／非同期 |
| `Ctrl+P / Ctrl+N` | 前／次のコマンド履歴 |
| `q` | 終了 |

通常モードの既定キーは大文字小文字を区別します。Enter または Ctrl+Space で複数選択し、yy（コピー）または x（切り取り）、移動先で p（貼り付け）。マークがなければ現在のファイルを使用します。コピー／切り取り後はマークが解除されますが内部バッファーは残ります。u はマーク解除、c はバッファー消去。yy と dd は連続入力です。dd は確認後に復元可能な削除を行い、意図的に完全削除へ切り替えません。Space と o の動作はプラットフォームとセッションに依存します。

Visual Mode：V で範囲選択を開始し、移動で拡張、V で確定、Esc で破棄、o で端点を交換します。Visual Mode では上流の y/d が残り、通常モードの yy/x と異なります。V で確定後に通常モードで yy/x/dd を使うことを推奨します。コマンドラインモード：Enter 実行、Esc 取消、Tab 補完、上下矢印で履歴、左右矢印と Home/End でカーソル移動。全編集キーとコマンドは neofm -doc または次の資料を参照： [lf documentation](lf-upstream/doc.txt).

マウスホイールの割り当ても継承しますが、既定ではマウス無効です。

## アーキテクチャと検証

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

共有ソースツリーと独立したプラットフォームランチャー。[アーキテクチャ](docs/architecture.md)、[検証](docs/validation.md)、[上流の出典](UPSTREAM.md) を参照してください。

~~~sh
cargo test --locked --manifest-path lf-integration-poc/adapter/Cargo.toml
~~~

統一 Rust Release は Mac ARM64、Windows x64、Linux x64 でネイティブビルドと各 4 項目のレンダラースモークテストに成功しました。Mac Adapter は 19/19。以前の展開でネイティブプレビューとゴミ箱も確認しました。適応した Writer には既知の上流由来の失敗が 3 件あります。完全な上流互換、GUI 全体の再試験、対話型 Sidebar Tabs、ヘッドレス SSH での完全な PDF/Office や高解像度画像表示は保証していません。

## 謝辞とライセンス

[lf](https://github.com/gokcehan/lf)（MIT）と [OpenAI Codex](https://github.com/openai/codex)（Apache-2.0）の抽出コンポーネントを使用しています。QuickLook、Dzurkel は別途導入し、同梱しません。NeoFM は独立したプロジェクトです。[LICENSE](LICENSE)、[NOTICE](NOTICE)、[lf ライセンス](lf-upstream/LICENSE)、[UPSTREAM.md](UPSTREAM.md)、[ブランド素材](assets/brand/) を参照してください。

<p align="center"><sub>小さなツール、明快なファイル、ネイティブプレビュー。</sub></p>
