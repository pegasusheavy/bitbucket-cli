# Packaging

This directory contains packaging configurations for various Linux distributions and Windows.

## Debian/Ubuntu (.deb)

Configuration is in `Cargo.toml` under `[package.metadata.deb]`.

Build with:
```bash
cargo install cargo-deb
cargo deb
```

## Red Hat/Fedora/CentOS (.rpm)

Configuration is in `Cargo.toml` under `[package.metadata.generate-rpm]`.

Build with:
```bash
cargo install cargo-generate-rpm
cargo build --release
cargo generate-rpm
```

## Arch Linux

PKGBUILD is located in `packaging/arch/PKGBUILD`.

Build with:
```bash
cd packaging/arch
makepkg -si
```

Or install from AUR (when available):
```bash
yay -S bitbucket-cli
```

## Alpine Linux

APKBUILD is located in `packaging/alpine/APKBUILD`.

Build with:
```bash
cd packaging/alpine
abuild -r
```

## Windows (.msi)

WiX configuration is in `packaging/windows/main.wxs`.

Build with WiX Toolset:
```bash
cargo build --release
candle packaging/windows/main.wxs
light -ext WixUIExtension main.wixobj
```

## Notes

- All packages are automatically built and released via GitHub Actions
- Manual builds should follow the official packaging guidelines for each distribution
- The Windows installer requires WiX Toolset 3.11 or later
