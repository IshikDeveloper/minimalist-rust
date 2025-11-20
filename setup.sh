#!/bin/bash
# Minimalist Browser Setup Script for Linux/macOS

echo "==================================="
echo "Minimalist Browser Setup"
echo "==================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Create necessary directories
echo -e "${GREEN}Creating directories...${NC}"
mkdir -p browser_data/cache browser_data/storage plugins assets

# Install Rust if not present
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Installing Rust...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
fi

# Download Flash Plugin (from Internet Archive)
echo -e "${GREEN}Downloading Flash Player...${NC}"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux Flash Player
    wget -O flash_player.tar.gz "https://archive.org/download/flashplayerarchive/pub/flashplayer/installers/archive/fp_32.0.0.465_archive/flashplayer32_0r0_465_linux.x86_64.tar.gz" 2>/dev/null || \
    echo -e "${YELLOW}Flash download failed. You can manually download from: https://archive.org/details/flashplayerarchive${NC}"
    
    if [ -f flash_player.tar.gz ]; then
        tar -xzf flash_player.tar.gz -C plugins/ libpepflashplayer.so 2>/dev/null || true
        rm flash_player.tar.gz
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS Flash Player
    echo -e "${YELLOW}Please download Flash manually for macOS from:${NC}"
    echo "https://archive.org/details/flashplayerarchive"
fi

# Download Ruffle as backup (WebAssembly Flash emulator)
echo -e "${GREEN}Downloading Ruffle WebAssembly...${NC}"
wget -O assets/ruffle.js "https://unpkg.com/@ruffle-rs/ruffle@latest/ruffle.js" 2>/dev/null || \
echo -e "${YELLOW}Ruffle download failed. Browser will use CDN fallback.${NC}"

# Install system dependencies
echo -e "${GREEN}Installing system dependencies...${NC}"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    # Linux dependencies
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y libwebkit2gtk-4.0-dev libgtk-3-dev
    elif command -v dnf &> /dev/null; then
        sudo dnf install -y webkit2gtk3-devel gtk3-devel
    elif command -v pacman &> /dev/null; then
        sudo pacman -S --noconfirm webkit2gtk gtk3
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    # macOS dependencies
    if command -v brew &> /dev/null; then
        brew install --cask webview2
    else
        echo -e "${YELLOW}Please install Homebrew first: https://brew.sh${NC}"
    fi
fi

# Build the browser
echo -e "${GREEN}Building Minimalist Browser...${NC}"
cargo build --release --profile minimal 2>/dev/null || cargo build --release

echo -e "${GREEN}Setup complete!${NC}"
echo -e "Run the browser with: ${YELLOW}./target/release/minimalist-browser${NC}"