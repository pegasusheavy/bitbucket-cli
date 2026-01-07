# Bitbucket CLI

[![CI](https://github.com/pegasusheavy/bitbucket-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/pegasusheavy/bitbucket-cli/actions/workflows/ci.yml)
[![Release](https://github.com/pegasusheavy/bitbucket-cli/actions/workflows/release.yml/badge.svg)](https://github.com/pegasusheavy/bitbucket-cli/releases)
[![Crates.io](https://img.shields.io/crates/v/bitbucket-cli.svg)](https://crates.io/crates/bitbucket-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful command-line interface for Bitbucket Cloud. Manage repositories, pull requests, issues, and pipelines directly from your terminal.

## ✨ Features

- 📁 **Repository Management** - List, view, clone, create, and manage repositories
- 🔀 **Pull Requests** - Create, review, merge, approve, and manage PRs
- 🐛 **Issue Tracking** - Create, view, comment on, and manage issues
- ⚡ **Pipelines** - Trigger, monitor, and manage CI/CD pipelines
- 🖥️ **Interactive TUI** - Beautiful terminal UI for browsing and managing resources
- 🔐 **Secure Authentication** - App passwords and OAuth with secure keyring storage

## 📦 Installation

### Using Cargo (Recommended)

```bash
cargo install bitbucket-cli
```

### Debian/Ubuntu

```bash
# Download the .deb package from releases
wget https://github.com/pegasusheavy/bitbucket-cli/releases/latest/download/bitbucket-cli_amd64.deb
sudo dpkg -i bitbucket-cli_amd64.deb
```

### Red Hat/Fedora/CentOS

```bash
# Download the .rpm package from releases
wget https://github.com/pegasusheavy/bitbucket-cli/releases/latest/download/bitbucket-cli.x86_64.rpm
sudo rpm -i bitbucket-cli.x86_64.rpm
# or with dnf
sudo dnf install bitbucket-cli.x86_64.rpm
```

### Arch Linux

```bash
# Download the package from releases
wget https://github.com/pegasusheavy/bitbucket-cli/releases/latest/download/bitbucket-cli-vX.X.X-x86_64.pkg.tar.zst
sudo pacman -U bitbucket-cli-vX.X.X-x86_64.pkg.tar.zst

# Or build from PKGBUILD
git clone https://github.com/pegasusheavy/bitbucket-cli.git
cd bitbucket-cli/packaging/arch
makepkg -si
```

### Alpine Linux

```bash
# Download the tarball from releases
wget https://github.com/pegasusheavy/bitbucket-cli/releases/latest/download/bitbucket-cli-vX.X.X-alpine-x86_64.tar.gz
tar -xzf bitbucket-cli-vX.X.X-alpine-x86_64.tar.gz
sudo mv bitbucket /usr/local/bin/
```

### Windows

Download the MSI installer from the [Releases](https://github.com/pegasusheavy/bitbucket-cli/releases) page and run it.

### From Source

```bash
git clone https://github.com/pegasusheavy/bitbucket-cli.git
cd bitbucket-cli
cargo install --path .
```

### Pre-built Binaries

Download pre-built binaries for your platform from the [Releases](https://github.com/pegasusheavy/bitbucket-cli/releases) page.

## 🚀 Quick Start

### 1. Authenticate

```bash
bitbucket auth login
```

You'll be prompted to enter your Bitbucket username and app password. [Create an app password](https://bitbucket.org/account/settings/app-passwords/) with the required permissions.

### 2. Start Using

```bash
# List repositories
bitbucket repo list myworkspace

# View a repository
bitbucket repo view myworkspace/myrepo

# List pull requests
bitbucket pr list myworkspace/myrepo

# Create a pull request
bitbucket pr create myworkspace/myrepo --title "My PR" --source feature-branch

# Launch interactive TUI
bitbucket tui --workspace myworkspace
```

## 📖 Commands

| Command | Description |
|---------|-------------|
| `bitbucket auth` | Manage authentication (login, logout, status) |
| `bitbucket repo` | Manage repositories (list, view, clone, create, fork, delete) |
| `bitbucket pr` | Manage pull requests (list, view, create, merge, approve, decline) |
| `bitbucket issue` | Manage issues (list, view, create, comment, close, reopen) |
| `bitbucket pipeline` | Manage pipelines (list, view, trigger, stop) |
| `bitbucket tui` | Launch interactive terminal UI |

## 🖥️ TUI Mode

Launch the interactive terminal UI for a visual way to browse and manage your Bitbucket resources:

```bash
bitbucket tui
```

**Keyboard shortcuts:**
- `q` - Quit
- `1-5` - Switch views (Dashboard, Repos, PRs, Issues, Pipelines)
- `j/k` or `↑/↓` - Navigate
- `Enter` - Select/Open
- `r` - Refresh

## ⚙️ Configuration

Configuration is stored in `~/.config/bitbucket/config.toml`:

```toml
[auth]
username = "your-username"
default_workspace = "your-workspace"

[defaults]
branch = "main"

[display]
color = true
pager = true
```

## 📚 Documentation

Full documentation is available at [pegasusheavy.github.io/bitbucket-cli](https://pegasusheavy.github.io/bitbucket-cli/)

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Support

If you find this project useful, please consider:

- ⭐ Starring the repository
- 🐛 Reporting bugs
- 💡 Suggesting features
- 💰 [Supporting on Patreon](https://www.patreon.com/c/PegasusHeavyIndustries)

---

Made with ❤️ by [Pegasus Heavy Industries](https://github.com/pegasusheavy)
