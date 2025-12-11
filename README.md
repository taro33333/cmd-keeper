# cmd-keeper

[![CI](https://github.com/taro33333/cmd-keeper/actions/workflows/ci.yml/badge.svg)](https://github.com/taro33333/cmd-keeper/actions/workflows/ci.yml)
[![Release](https://github.com/taro33333/cmd-keeper/actions/workflows/release.yml/badge.svg)](https://github.com/taro33333/cmd-keeper/actions/workflows/release.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**[日本語](./README_ja.md)** | English

📝 **A CLI tool to save, manage, and search frequently used commands locally**

## Features

- 🖥️ **TUI Mode**: Interactive terminal UI
- 🚀 **Fast**: Built with Rust for high performance
- 📦 **Simple**: Minimal dependencies, ready to use
- 🏷️ **Tags**: Organize commands with tags
- 🔍 **Search**: Search by command, description, or tags
- 📋 **Clipboard**: Copy commands with one keystroke
- 💾 **Local Storage**: Data stored safely in local JSON file
- ⌨️ **Vim-like**: Navigation with `j`/`k` keys

## Demo

![cmd-keeper demo](./demo.gif)

## Installation

### Homebrew (Recommended)

For macOS / Linux users:

```bash
brew install taro33333/tap/cmd-keeper
```

### GitHub Releases

Download binaries from the [Releases](https://github.com/taro33333/cmd-keeper/releases) page:

| OS | Architecture | Filename |
|----|--------------|----------|
| macOS | Apple Silicon (M1/M2) | cmd-keeper-darwin-arm64 |
| macOS | Intel | cmd-keeper-darwin-amd64 |
| Linux | x86_64 | cmd-keeper-linux-amd64 |
| Windows | x86_64 | cmd-keeper-windows-amd64.exe |

```bash
# Example: macOS Apple Silicon
curl -LO https://github.com/taro33333/cmd-keeper/releases/latest/download/cmd-keeper-darwin-arm64
chmod +x cmd-keeper-darwin-arm64
sudo mv cmd-keeper-darwin-arm64 /usr/local/bin/cmd-keeper
```

### Build from Source

```bash
git clone https://github.com/taro33333/cmd-keeper.git
cd cmd-keeper
cargo install --path .
```

### Cargo

```bash
cargo install cmd-keeper
```

## Quick Start

```bash
# Launch TUI mode (recommended)
cmd-keeper

# Add a command (CLI)
cmd-keeper add -c "git log --oneline -n 10" -d "Show last 10 commits" -t git

# List all commands
cmd-keeper list

# Search commands
cmd-keeper search git

# Copy to clipboard
cmd-keeper copy 1
```

## TUI Mode (Interactive UI)

Running without arguments launches an interactive UI:

```bash
cmd-keeper
# or
cmd-keeper tui
```

### Key Bindings

#### Normal Mode (List View)

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Go to top |
| `G` | Go to bottom |
| `a` | Add command mode |
| `d` | Delete confirmation |
| `Enter` / `y` | Copy to clipboard |

#### Adding Mode (Add Command)

| Key | Action |
|-----|--------|
| `Tab` | Next field |
| `Shift+Tab` | Previous field |
| `Ctrl+S` | Save |
| `Enter` | Next field / Save on Tags field |
| `Esc` | Cancel |

#### Delete Confirmation

| Key | Action |
|-----|--------|
| `y` | Confirm delete |
| `n` / `Esc` | Cancel |

## CLI Mode

Traditional command-line operations are also supported.

### Add a Command

```bash
# Basic usage
cmd-keeper add -c "git log --oneline -n 10" -d "Show last 10 commits"

# With tags
cmd-keeper add -c "docker ps -a" -d "List all containers" -t docker,container
```

### List Commands

```bash
# List all commands
cmd-keeper list

# Show full content without truncation
cmd-keeper list --full
```

Example output:

```
╭────┬───────────────────────────────┬──────────────────────────┬─────────────╮
│ ID │ Command                       │ Description              │ Tags        │
├────┼───────────────────────────────┼──────────────────────────┼─────────────┤
│ 1  │ git log --oneline -n 10       │ Show last 10 commits     │ git         │
│ 2  │ docker ps -a                  │ List all containers      │ docker      │
╰────┴───────────────────────────────┴──────────────────────────┴─────────────╯

Total: 2 command(s)
```

### Search Commands

```bash
# Search by keyword (searches command, description, and tags)
cmd-keeper search docker

# Show full content
cmd-keeper search git --full
```

### Delete a Command

```bash
# With confirmation
cmd-keeper delete 1

# Force delete (skip confirmation)
cmd-keeper delete 1 --force
```

### Copy to Clipboard

```bash
cmd-keeper copy 1
```

### Show Database Path

```bash
cmd-keeper path
```

## Command Reference

| Command | Alias | Description |
|---------|-------|-------------|
| (none) | - | Launch TUI mode |
| `tui` | `ui` | Launch TUI mode |
| `add` | `a` | Add a command |
| `list` | `ls` | List all commands |
| `search` | `s` | Search by keyword |
| `delete` | `rm` | Delete by ID |
| `copy` | `cp` | Copy to clipboard |
| `path` | - | Show database path |

## Data Storage

Commands are stored in JSON format at:

- **Linux/macOS**: `~/.config/cmd-keeper/commands.json`
- **Windows**: `C:\Users\<USER>\AppData\Roaming\cmd-keeper\commands.json`

### Data Structure

```json
{
  "next_id": 3,
  "entries": [
    {
      "id": 1,
      "command": "git log --oneline -n 10",
      "description": "Show last 10 commits",
      "tags": ["git"],
      "created_at": "2024-01-15T10:30:00Z"
    }
  ]
}
```

## Development

### Build

```bash
cargo build --release
```

### Test

```bash
cargo test
```

### Format

```bash
cargo fmt
```

### Lint

```bash
cargo clippy
```

## Tech Stack

- **Language**: Rust (Edition 2021)
- **CLI**: clap (derive feature)
- **TUI**: ratatui + crossterm
- **Serialization**: serde + serde_json
- **DateTime**: chrono
- **Error Handling**: anyhow + thiserror

## Release Process

1. Update version in `Cargo.toml`
2. Create and push a tag:

```bash
git tag v1.0.0
git push origin v1.0.0
```

3. GitHub Actions will automatically build and release
4. Homebrew Formula will be updated automatically

## License

MIT License

## Links

- [GitHub Repository](https://github.com/taro33333/cmd-keeper)
- [Releases](https://github.com/taro33333/cmd-keeper/releases)
