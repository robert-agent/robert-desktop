#!/bin/bash
#
# Setup script for Robert browser automation development on Linux
# Installs all system dependencies required to build and test the project
#
# Usage:
#   chmod +x setup-linux-dev.sh
#   sudo ./setup-linux-dev.sh
#

set -e  # Exit on any error

echo "=============================================="
echo "Robert Browser Automation - Linux Setup"
echo "=============================================="
echo ""

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    echo "⚠️  This script needs to be run with sudo privileges"
    echo "Please run: sudo ./setup-linux-dev.sh"
    exit 1
fi

# Get the actual user (not root when using sudo)
ACTUAL_USER="${SUDO_USER:-$USER}"

echo "Installing dependencies for user: $ACTUAL_USER"
echo ""

# Detect Linux distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
else
    echo "❌ Cannot detect Linux distribution"
    exit 1
fi

echo "Detected: $OS"
echo ""

# Ubuntu/Debian installation
if [[ "$OS" == "ubuntu" || "$OS" == "debian" || "$OS" == "pop" || "$OS" == "linuxmint" ]]; then
    echo "📦 Updating package lists..."
    apt-get update

    echo ""
    echo "📦 Installing GTK dependencies for Tauri..."
    apt-get install -y \
        libgtk-3-dev \
        libwebkit2gtk-4.1-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        patchelf \
        libglib2.0-dev \
        libcairo2-dev \
        libpango1.0-dev \
        libgdk-pixbuf2.0-dev \
        libatk1.0-dev \
        libsoup-3.0-dev

    echo ""
    echo "📦 Installing build essentials..."
    apt-get install -y \
        build-essential \
        pkg-config \
        libssl-dev \
        curl \
        wget \
        git

    echo ""
    echo "🌐 Installing Google Chrome..."
    if ! command -v google-chrome &> /dev/null; then
        wget -q -O /tmp/google-chrome-stable_current_amd64.deb \
            https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
        apt-get install -y /tmp/google-chrome-stable_current_amd64.deb || apt-get install -f -y
        rm -f /tmp/google-chrome-stable_current_amd64.deb
        echo "✅ Google Chrome installed"
    else
        echo "✅ Google Chrome already installed"
    fi

    echo ""
    echo "🚗 Installing ChromeDriver..."
    if ! command -v chromedriver &> /dev/null; then
        CHROME_VERSION=$(google-chrome --version | awk '{print $3}' | cut -d. -f1)
        echo "   Chrome version: $CHROME_VERSION"
        CHROMEDRIVER_VERSION=$(curl -s "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_$CHROME_VERSION")
        echo "   ChromeDriver version: $CHROMEDRIVER_VERSION"
        wget -q -O /tmp/chromedriver-linux64.zip \
            "https://storage.googleapis.com/chrome-for-testing-public/$CHROMEDRIVER_VERSION/linux64/chromedriver-linux64.zip"
        unzip -q -o /tmp/chromedriver-linux64.zip -d /tmp/
        mv /tmp/chromedriver-linux64/chromedriver /usr/local/bin/
        chmod +x /usr/local/bin/chromedriver
        rm -rf /tmp/chromedriver-linux64.zip /tmp/chromedriver-linux64
        echo "✅ ChromeDriver installed"
    else
        echo "✅ ChromeDriver already installed"
    fi
else
    echo "❌ Unsupported distribution: $OS"
    echo "This script supports Ubuntu, Debian, Linux Mint, and Pop!_OS"
    exit 1
fi

echo ""
echo "=============================================="
echo "📋 Verifying Installation"
echo "=============================================="
echo ""

# Verify installations
if command -v google-chrome &> /dev/null; then
    echo "✅ Chrome: $(google-chrome --version)"
else
    echo "❌ Chrome: Not found"
fi

if command -v chromedriver &> /dev/null; then
    echo "✅ ChromeDriver: $(chromedriver --version | head -1)"
else
    echo "❌ ChromeDriver: Not found"
fi

if pkg-config --exists gtk+-3.0; then
    echo "✅ GTK 3: $(pkg-config --modversion gtk+-3.0)"
else
    echo "❌ GTK 3: Not found"
fi

if pkg-config --exists webkit2gtk-4.1; then
    echo "✅ WebKit2GTK 4.1: $(pkg-config --modversion webkit2gtk-4.1)"
else
    echo "❌ WebKit2GTK 4.1: Not found"
fi

echo ""
echo "=============================================="
echo "🦀 Rust Installation"
echo "=============================================="
echo ""

# Check if Rust is installed for the actual user
if sudo -u $ACTUAL_USER bash -c 'command -v rustc &> /dev/null'; then
    echo "✅ Rust is already installed: $(sudo -u $ACTUAL_USER rustc --version)"
else
    echo "📦 Rust is not installed. Installing for user: $ACTUAL_USER"
    sudo -u $ACTUAL_USER bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'
    echo "✅ Rust installed. Please run: source ~/.cargo/env"
fi

echo ""
echo "=============================================="
echo "✅ Setup Complete!"
echo "=============================================="
echo ""
echo "📝 Next steps:"
echo "1. If Rust was just installed: source ~/.cargo/env"
echo "2. Build the project: cargo build"
echo "3. Run tests: cargo test"
echo "4. Run validation tests: cargo test --package robert-webdriver validation"
echo ""
