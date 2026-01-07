# Windows MSI Installer

This directory contains the WiX configuration for building Windows MSI installers.

## Files

- `main.wxs` - WiX source file defining the installer
- `icon.ico` - Application icon (placeholder - should be replaced with actual icon)

## Building

### Prerequisites

- WiX Toolset 4.0 or later
- Rust toolchain
- Visual Studio Build Tools

### Build Steps

1. Build the release binary:
```powershell
cargo build --release --target x86_64-pc-windows-msvc
```

2. Build the MSI:
```powershell
wix build -arch x64 -o bitbucket-cli.msi packaging/windows/main.wxs
```

Or use the automated script:
```powershell
.\scripts\build-packages.ps1
```

## Installer Features

- Installs `bitbucket.exe` to `C:\Program Files\BitbucketCLI\`
- Automatically adds to system PATH
- Includes README and LICENSE
- Provides uninstaller
- System-wide or per-user installation options

## Customization

To customize the installer:

1. **Change Icon**: Replace `icon.ico` with your application icon
2. **Add Files**: Add more `<File>` elements in `main.wxs`
3. **Modify Install Path**: Change `APPLICATIONFOLDER` directory structure
4. **Add Shortcuts**: Add `<Shortcut>` elements for Start Menu or Desktop

## Testing

Test the installer:
```powershell
# Install
msiexec /i bitbucket-cli.msi

# Verify installation
bitbucket --version

# Uninstall
msiexec /x bitbucket-cli.msi
```

## Notes

- The icon.ico file is currently a placeholder and should be replaced with an actual icon
- The UpgradeCode in main.wxs should remain constant across versions
- Product Id uses '*' for automatic GUID generation per build
