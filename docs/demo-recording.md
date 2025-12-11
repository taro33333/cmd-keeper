# デモGIFの作成方法

このドキュメントでは、README に掲載するデモ GIF の作成方法を説明します。

## 必要なツール

### VHS (Video Home System)

[VHS](https://github.com/charmbracelet/vhs) は Charm 製のターミナル録画ツールです。
スクリプトファイル（`.tape`）を使って、ターミナル操作を自動化・録画できます。

## インストール

### macOS (Homebrew)

```bash
brew install vhs
```

### その他のプラットフォーム

```bash
# Go がインストールされている場合
go install github.com/charmbracelet/vhs@latest
```

詳細は [VHS公式リポジトリ](https://github.com/charmbracelet/vhs) を参照してください。

## 使用方法

### 1. デモGIFを生成

プロジェクトルートで以下を実行：

```bash
vhs demo.tape
```

これで `demo.gif` が生成されます。

### 2. 生成されたGIFを確認

```bash
open demo.gif  # macOS
xdg-open demo.gif  # Linux
```

## demo.tape の構文

### 基本設定

```tape
# 出力ファイル名
Output demo.gif

# フォントサイズ
Set FontSize 14

# 画面サイズ
Set Width 1200
Set Height 800

# テーマ（Catppuccin Mocha, Dracula, etc.）
Set Theme "Catppuccin Mocha"
```

### コマンド

| コマンド | 説明 | 例 |
|---------|------|-----|
| `Type` | 文字を入力 | `Type "cmd-keeper"` |
| `Enter` | Enterキーを押す | `Enter` |
| `Tab` | Tabキーを押す | `Tab` |
| `Sleep` | 待機 | `Sleep 500ms`, `Sleep 2s` |
| `Up` / `Down` | 矢印キー | `Up`, `Down` |
| `Escape` | Escキー | `Escape` |
| `Backspace` | Backspaceキー | `Backspace` |

### 現在の demo.tape の内容

```tape
# VHS Demo Script for cmd-keeper
# Run with: vhs demo.tape

# Configuration
Output demo.gif
Set FontSize 14
Set Width 1200
Set Height 800
Set Theme "Catppuccin Mocha"

# Start
Type "cmd-keeper"
Enter
Sleep 2s

# Navigate down
Type "j"
Sleep 500ms
Type "j"
Sleep 500ms

# Navigate up
Type "k"
Sleep 500ms

# Copy to clipboard
Type "y"
Sleep 1s

# Add new command
Type "a"
Sleep 1s

# Type command
Type "docker compose up -d"
Sleep 500ms
Tab
Sleep 300ms

# Type description
Type "Start all containers in background"
Sleep 500ms
Tab
Sleep 300ms

# Type tags
Type "docker,compose"
Sleep 500ms

# Save with Enter (in Tags field, Enter saves)
Enter
Sleep 1s

# Show the new command
Type "G"
Sleep 1s

# Quit
Type "q"
Sleep 500ms
```

## カスタマイズ

### テーマの変更

利用可能なテーマ一覧：

- `Catppuccin Mocha` (デフォルト)
- `Dracula`
- `Tokyo Night`
- `Nord`
- `Gruvbox`

```tape
Set Theme "Dracula"
```

### 出力形式の変更

GIF以外にも対応しています：

```tape
Output demo.mp4   # MP4動画
Output demo.webm  # WebM動画
Output demo.png   # 最終フレームのスクリーンショット
```

### タイピング速度の調整

```tape
Set TypingSpeed 50ms  # デフォルトは 50ms
```

## トラブルシューティング

### エラー: `Not a valid modifier`

`Ctrl+s` のような書き方はサポートされていません。
代わりに以下の形式を使用してください：

```tape
# NG
Ctrl+s

# OK (ただし、現在の実装では Enter で保存)
Enter
```

### GIFが大きすぎる

1. 画面サイズを小さくする
2. フレームレートを下げる
3. 録画時間を短くする

```tape
Set Width 800
Set Height 600
Set Framerate 10
```

## 参考リンク

- [VHS 公式ドキュメント](https://github.com/charmbracelet/vhs)
- [VHS コマンドリファレンス](https://github.com/charmbracelet/vhs#commands)

