#!/bin/bash
set -e

# Build script for creating distribution packages
# Usage: ./scripts/build-packages.sh [deb|rpm|arch|alpine|all]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_ROOT"

print_usage() {
    echo "Usage: $0 [deb|rpm|arch|alpine|all]"
    echo ""
    echo "Options:"
    echo "  deb     - Build Debian/Ubuntu package"
    echo "  rpm     - Build RPM package"
    echo "  arch    - Build Arch Linux package"
    echo "  alpine  - Build Alpine Linux tarball"
    echo "  all     - Build all packages"
    exit 1
}

build_deb() {
    echo "Building Debian package..."
    if ! command -v cargo-deb &> /dev/null; then
        echo "Installing cargo-deb..."
        cargo install cargo-deb
    fi
    cargo deb
    echo "✓ Debian package built in target/debian/"
}

build_rpm() {
    echo "Building RPM package..."
    if ! command -v cargo-generate-rpm &> /dev/null; then
        echo "Installing cargo-generate-rpm..."
        cargo install cargo-generate-rpm
    fi
    cargo build --release
    cargo generate-rpm
    echo "✓ RPM package built in target/generate-rpm/"
}

build_arch() {
    echo "Building Arch Linux package..."
    cargo build --release
    cd target/release
    VERSION=$(grep '^version = ' ../../Cargo.toml | head -1 | cut -d'"' -f2)
    tar -I 'zstd -19' -cf "../bitbucket-cli-${VERSION}-x86_64.pkg.tar.zst" bitbucket
    cd ../..
    echo "✓ Arch package built in target/bitbucket-cli-${VERSION}-x86_64.pkg.tar.zst"
}

build_alpine() {
    echo "Building Alpine Linux tarball..."
    if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
        echo "Adding musl target..."
        rustup target add x86_64-unknown-linux-musl
    fi
    cargo build --release --target x86_64-unknown-linux-musl
    cd target/x86_64-unknown-linux-musl/release
    VERSION=$(grep '^version = ' ../../../Cargo.toml | head -1 | cut -d'"' -f2)
    tar -czvf "../../bitbucket-cli-${VERSION}-alpine-x86_64.tar.gz" bitbucket
    cd ../../..
    echo "✓ Alpine tarball built in target/bitbucket-cli-${VERSION}-alpine-x86_64.tar.gz"
}

case "${1:-all}" in
    deb)
        build_deb
        ;;
    rpm)
        build_rpm
        ;;
    arch)
        build_arch
        ;;
    alpine)
        build_alpine
        ;;
    all)
        build_deb
        build_rpm
        build_arch
        build_alpine
        echo ""
        echo "✓ All packages built successfully!"
        ;;
    *)
        print_usage
        ;;
esac
