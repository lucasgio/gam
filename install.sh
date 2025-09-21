#!/bin/bash

# SSH Manager Installation Script
# This script compiles and installs the SSH Manager tool

set -e

echo "🔑 SSH Manager Installation"
echo "=========================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "⚠️  This tool is designed for macOS due to Keychain integration."
    echo "   It might work on other systems but some features may not work properly."
fi

echo "🔄 Compiling SSH Manager..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Compilation successful!"
else
    echo "❌ Compilation failed!"
    exit 1
fi

# Ask user if they want to install globally
echo ""
read -p "Do you want to install SSH Manager globally? (requires sudo) [y/N]: " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "🔄 Installing globally..."
    sudo cp target/release/ssh-manager /usr/local/bin/
    echo "✅ SSH Manager installed to /usr/local/bin/ssh-manager"
    echo ""
    echo "You can now run: ssh-manager"
else
    echo "ℹ️  You can run SSH Manager using:"
    echo "   cd $(pwd)"
    echo "   ./target/release/ssh-manager"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "📖 Usage:"
echo "   ssh-manager          # Interactive mode"
echo "   ssh-manager add      # Add new account"
echo "   ssh-manager list     # List accounts"
echo "   ssh-manager switch   # Switch accounts"
echo "   ssh-manager status   # Show current account"
echo ""
echo "📚 See README.md for detailed documentation"