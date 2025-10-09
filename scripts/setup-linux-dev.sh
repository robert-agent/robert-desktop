#!/bin/bash
#
# Setup script for Robert browser automation development on Linux
# Installs all system dependencies required to build and test the project
#
# Usage:
#   chmod +x setup-linux-dev.sh
#   sudo ./setup-linux-dev.sh
#
# Or run directly:
#   bash setup-linux-dev.sh
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
ACTUAL_HOME=$(eval echo ~$ACTUAL_USER)

echo "Installing dependencies for user: $ACTUAL_USER"
echo ""

# Detect Linux distribution
if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS=$ID
    VER=$VERSION_ID
else
    echo "❌ Cannot detect Linux distribution"
    exit 1
fi

echo "Detected: $OS $VER"
echo ""

# Function to install on Ubuntu/Debian
install_ubuntu_debian() {
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
        # Download Chrome
        wget -q -O /tmp/google-chrome-stable_current_amd64.deb \
            https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb

        # Install Chrome
        apt-get install -y /tmp/google-chrome-stable_current_amd64.deb || {
            # Fix dependencies if needed
            apt-get install -f -y
        }

        # Cleanup
        rm -f /tmp/google-chrome-stable_current_amd64.deb

        echo "✅ Google Chrome installed"
    else
        echo "✅ Google Chrome already installed"
    fi

    echo ""
    echo "🚗 Installing ChromeDriver..."
    if ! command -v chromedriver &> /dev/null; then
        # Get Chrome version
        CHROME_VERSION=$(google-chrome --version | awk '{print $3}' | cut -d. -f1)
        echo "   Chrome version: $CHROME_VERSION"

        # Get matching ChromeDriver version
        CHROMEDRIVER_VERSION=$(curl -s "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_$CHROME_VERSION")
        echo "   ChromeDriver version: $CHROMEDRIVER_VERSION"

        # Download ChromeDriver
        wget -q -O /tmp/chromedriver-linux64.zip \
            "https://storage.googleapis.com/chrome-for-testing-public/$CHROMEDRIVER_VERSION/linux64/chromedriver-linux64.zip"

        # Extract and install
        unzip -q -o /tmp/chromedriver-linux64.zip -d /tmp/
        mv /tmp/chromedriver-linux64/chromedriver /usr/local/bin/
        chmod +x /usr/local/bin/chromedriver

        # Cleanup
        rm -rf /tmp/chromedriver-linux64.zip /tmp/chromedriver-linux64

        echo "✅ ChromeDriver installed"
    else
        echo "✅ ChromeDriver already installed"
    fi
}

# Function to install on Fedora/RHEL/CentOS
install_fedora_rhel() {
    echo "📦 Updating package lists..."
    dnf check-update || true

    echo ""
    echo "📦 Installing GTK dependencies for Tauri..."
    dnf install -y \
        gtk3-devel \
        webkit2gtk4.1-devel \
        librsvg2-devel \
        patchelf \
        glib2-devel \
        cairo-devel \
        pango-devel \
        gdk-pixbuf2-devel \
        atk-devel \
        libsoup3-devel

    echo ""
    echo "📦 Installing build essentials..."
    dnf install -y \
        gcc \
        gcc-c++ \
        make \
        pkgconfig \
        openssl-devel \
        curl \
        wget \
        git

    echo ""
    echo "🌐 Installing Google Chrome..."
    if ! command -v google-chrome &> /dev/null; then
        # Add Chrome repo
        cat > /etc/yum.repos.d/google-chrome.repo <<'EOF'
[google-chrome]
name=google-chrome
baseurl=http://dl.google.com/linux/chrome/rpm/stable/x86_64
enabled=1
gpgcheck=1
gpgkey=https://dl.google.com/linux/linux_signing_key.pub
EOF

        # Install Chrome
        dnf install -y google-chrome-stable

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
}

# Function to install on Arch Linux
install_arch() {
    echo "📦 Updating package lists..."
    pacman -Sy

    echo ""
    echo "📦 Installing GTK dependencies for Tauri..."
    pacman -S --noconfirm --needed \
        gtk3 \
        webkit2gtk-4.1 \
        librsvg \
        patchelf \
        glib2 \
        cairo \
        pango \
        gdk-pixbuf2 \
        atk \
        libsoup3

    echo ""
    echo "📦 Installing build essentials..."
    pacman -S --noconfirm --needed \
        base-devel \
        pkgconf \
        openssl \
        curl \
        wget \
        git

    echo ""
    echo "🌐 Installing Google Chrome..."
    if ! command -v google-chrome &> /dev/null; then
        # Install from AUR or direct download
        wget -q -O /tmp/google-chrome-stable_current_amd64.deb \
            https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb

        # Note: You might need to use yay or another AUR helper
        echo "⚠️  On Arch, you may need to install Chrome via AUR:"
        echo "   yay -S google-chrome"
        echo ""
        echo "Attempting direct installation..."
        # Try to convert and install
        if command -v debtap &> /dev/null; then
            cd /tmp
            debtap -q google-chrome-stable_current_amd64.deb
            pacman -U --noconfirm google-chrome*.pkg.tar.zst
            cd -
        else
            echo "⚠️  debtap not found. Install Chrome manually with:"
            echo "   yay -S google-chrome"
        fi
    else
        echo "✅ Google Chrome already installed"
    fi

    echo ""
    echo "🚗 Installing ChromeDriver..."
    if ! command -v chromedriver &> /dev/null; then
        CHROME_VERSION=$(google-chrome --version 2>/dev/null | awk '{print $3}' | cut -d. -f1 || echo "131")
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
}

# Install based on distribution
case "$OS" in
    ubuntu|debian|pop|linuxmint)
        install_ubuntu_debian
        ;;
    fedora|rhel|centos|rocky|almalinux)
        install_fedora_rhel
        ;;
    arch|manjaro)
        install_arch
        ;;
    *)
        echo "❌ Unsupported distribution: $OS"
        echo ""
        echo "Please install the following manually:"
        echo "  - GTK 3 development libraries"
        echo "  - WebKit2GTK 4.1"
        echo "  - Build essentials (gcc, make, pkg-config)"
        echo "  - Google Chrome"
        echo "  - ChromeDriver"
        exit 1
        ;;
esac

echo ""
echo "=============================================="
echo "📋 Verifying Installation"
echo "=============================================="
echo ""

# Verify installations
echo "Checking installed packages..."

# Check Chrome
if command -v google-chrome &> /dev/null; then
    CHROME_VER=$(google-chrome --version)
    echo "✅ Chrome: $CHROME_VER"
else
    echo "❌ Chrome: Not found"
fi

# Check ChromeDriver
if command -v chromedriver &> /dev/null; then
    CHROMEDRIVER_VER=$(chromedriver --version)
    echo "✅ ChromeDriver: $CHROMEDRIVER_VER"
else
    echo "❌ ChromeDriver: Not found"
fi

# Check pkg-config
if command -v pkg-config &> /dev/null; then
    PKG_CONFIG_VER=$(pkg-config --version)
    echo "✅ pkg-config: $PKG_CONFIG_VER"
else
    echo "❌ pkg-config: Not found"
fi

# Check GTK libraries
echo ""
echo "Checking GTK libraries..."
if pkg-config --exists gtk+-3.0; then
    GTK_VER=$(pkg-config --modversion gtk+-3.0)
    echo "✅ GTK 3: $GTK_VER"
else
    echo "❌ GTK 3: Not found"
fi

if pkg-config --exists webkit2gtk-4.1; then
    WEBKIT_VER=$(pkg-config --modversion webkit2gtk-4.1)
    echo "✅ WebKit2GTK 4.1: $WEBKIT_VER"
else
    echo "❌ WebKit2GTK 4.1: Not found"
fi

if pkg-config --exists glib-2.0; then
    GLIB_VER=$(pkg-config --modversion glib-2.0)
    echo "✅ GLib: $GLIB_VER"
else
    echo "❌ GLib: Not found"
fi

echo ""
echo "=============================================="
echo "🦀 Rust Installation"
echo "=============================================="
echo ""

# Check if Rust is installed for the actual user
if sudo -u $ACTUAL_USER bash -c 'command -v rustc &> /dev/null'; then
    RUST_VER=$(sudo -u $ACTUAL_USER rustc --version)
    echo "✅ Rust is already installed: $RUST_VER"
else
    echo "📦 Rust is not installed. Installing for user: $ACTUAL_USER"
    echo ""

    # Install Rust as the actual user (not root)
    sudo -u $ACTUAL_USER bash -c 'curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'

    # Source cargo env for the user
    echo "✅ Rust installed. Please run: source ~/.cargo/env"
fi

echo ""
echo "=============================================="
echo "✅ Setup Complete!"
echo "=============================================="
echo ""
echo "All system dependencies have been installed."
echo ""
echo "📝 Next steps:"
echo ""
echo "1. If Rust was just installed, reload your shell:"
echo "   source ~/.cargo/env"
echo ""
echo "2. Verify your setup:"
echo "   cargo --version"
echo "   google-chrome --version"
echo "   chromedriver --version"
echo ""
echo "3. Build the project:"
echo "   cd $ACTUAL_HOME/robert"
echo "   cargo build"
echo ""
echo "4. Run tests:"
echo "   cargo test"
echo ""
echo "5. Run validation tests:"
echo "   cargo test --package robert-webdriver validation"
echo ""
echo "6. Run clippy:"
echo "   cargo clippy --workspace --all-targets -- -D warnings"
echo ""
echo "🎉 You're ready to develop!"
echo ""
