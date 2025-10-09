# Linux Development Setup

This guide explains how to set up your Linux development environment for the Robert browser automation project.

## Quick Setup (Recommended)

We provide an automated setup script that installs all required dependencies:

```bash
# Download and run the setup script
sudo ./setup-linux-dev.sh
```

**What it installs:**
- ✅ GTK 3 and WebKit2GTK libraries (for Tauri)
- ✅ Build essentials (gcc, make, pkg-config)
- ✅ Google Chrome (stable)
- ✅ ChromeDriver (matching Chrome version)
- ✅ Rust toolchain (if not already installed)

**Supported distributions:**
- Ubuntu / Debian / Linux Mint / Pop!_OS
- Fedora / RHEL / CentOS / Rocky Linux / AlmaLinux
- Arch Linux / Manjaro

## Manual Setup

If you prefer to install dependencies manually:

### Ubuntu / Debian / Linux Mint

```bash
# Update package lists
sudo apt-get update

# Install GTK dependencies for Tauri
sudo apt-get install -y \
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

# Install build essentials
sudo apt-get install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  curl \
  wget \
  git

# Install Google Chrome
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
sudo apt-get install -y ./google-chrome-stable_current_amd64.deb
rm google-chrome-stable_current_amd64.deb

# Install ChromeDriver (matching Chrome version)
CHROME_VERSION=$(google-chrome --version | awk '{print $3}' | cut -d. -f1)
CHROMEDRIVER_VERSION=$(curl -s "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_$CHROME_VERSION")
wget "https://storage.googleapis.com/chrome-for-testing-public/$CHROMEDRIVER_VERSION/linux64/chromedriver-linux64.zip"
unzip chromedriver-linux64.zip
sudo mv chromedriver-linux64/chromedriver /usr/local/bin/
sudo chmod +x /usr/local/bin/chromedriver
rm -rf chromedriver-linux64.zip chromedriver-linux64

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Fedora / RHEL / CentOS

```bash
# Update packages
sudo dnf check-update

# Install GTK dependencies for Tauri
sudo dnf install -y \
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

# Install build essentials
sudo dnf install -y \
  gcc \
  gcc-c++ \
  make \
  pkgconfig \
  openssl-devel \
  curl \
  wget \
  git

# Install Google Chrome
sudo tee /etc/yum.repos.d/google-chrome.repo <<EOF
[google-chrome]
name=google-chrome
baseurl=http://dl.google.com/linux/chrome/rpm/stable/x86_64
enabled=1
gpgcheck=1
gpgkey=https://dl.google.com/linux/linux_signing_key.pub
EOF
sudo dnf install -y google-chrome-stable

# Install ChromeDriver (same as Ubuntu above)
# ... (see Ubuntu section)

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Arch Linux / Manjaro

```bash
# Update packages
sudo pacman -Sy

# Install GTK dependencies for Tauri
sudo pacman -S --needed \
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

# Install build essentials
sudo pacman -S --needed \
  base-devel \
  pkgconf \
  openssl \
  curl \
  wget \
  git

# Install Google Chrome (via AUR)
yay -S google-chrome

# Install ChromeDriver (same as Ubuntu above)
# ... (see Ubuntu section)

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Verification

After installation, verify everything is set up correctly:

```bash
# Check Rust
cargo --version
rustc --version

# Check Chrome
google-chrome --version

# Check ChromeDriver
chromedriver --version

# Check GTK libraries
pkg-config --modversion gtk+-3.0
pkg-config --modversion webkit2gtk-4.1
pkg-config --modversion glib-2.0

# Check build tools
gcc --version
make --version
pkg-config --version
```

## Build and Test

Once setup is complete:

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run validation tests only
cargo test --package robert-webdriver validation

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings

# Build the Tauri app
cargo build --package robert-app
```

## Troubleshooting

### "glib-2.0 not found" error

```bash
# Install missing GTK library
sudo apt-get install libglib2.0-dev  # Ubuntu/Debian
sudo dnf install glib2-devel         # Fedora/RHEL
sudo pacman -S glib2                 # Arch
```

### "webkit2gtk-4.1 not found" error

```bash
# Install WebKit2GTK
sudo apt-get install libwebkit2gtk-4.1-dev  # Ubuntu/Debian
sudo dnf install webkit2gtk4.1-devel        # Fedora/RHEL
sudo pacman -S webkit2gtk-4.1               # Arch
```

### ChromeDriver version mismatch

```bash
# Update ChromeDriver to match Chrome
google-chrome --version  # Check Chrome version
# Download matching ChromeDriver from:
# https://googlechromelabs.github.io/chrome-for-testing/
```

### Build without Tauri

If you only want to work on the validation library (without the Tauri GUI):

```bash
# Build without Tauri app
cargo build --workspace --exclude robert-app

# Test without Tauri
cargo test --workspace --exclude robert-app

# Lint without Tauri
cargo clippy --workspace --exclude robert-app --all-targets -- -D warnings
```

This works without installing GTK libraries!

## What Each Dependency Does

| Dependency | Purpose |
|------------|---------|
| **GTK 3** | GUI toolkit for Tauri |
| **WebKit2GTK** | Web rendering engine for Tauri |
| **GLib** | Core library for GTK |
| **Cairo** | 2D graphics library |
| **Pango** | Text rendering |
| **GDK-Pixbuf** | Image loading |
| **ATK** | Accessibility toolkit |
| **libsoup** | HTTP client library |
| **Google Chrome** | Browser for testing |
| **ChromeDriver** | Browser automation driver |
| **Rust** | Programming language |
| **pkg-config** | Library configuration tool |

## Minimal Setup (Validation Only)

If you only need to work on the validation feature (no Tauri GUI):

```bash
# Install only Rust and Chrome
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Chrome (see distribution-specific commands above)

# Build and test validation
cargo build --package robert-webdriver
cargo test --package robert-webdriver validation
```

**No GTK dependencies needed!** ✅

## CI/CD Note

GitHub Actions workflows automatically install all dependencies. See:
- `.github/workflows/ci.yml`
- `.github/workflows/e2e-test.yml`

## Support

If you encounter issues:
1. Check the troubleshooting section above
2. Review `BUILD_STATUS.md` for detailed build information
3. Check `FIX_CARGO_XLINT.md` for common build errors
4. Open an issue on GitHub

---

**Quick Start:**
```bash
sudo ./setup-linux-dev.sh
source ~/.cargo/env
cargo build
cargo test
```

**That's it!** 🎉
