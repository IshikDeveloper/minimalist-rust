# Minimalist Browser Setup Script for Windows
Write-Host "===================================" -ForegroundColor Cyan
Write-Host "Minimalist Browser Setup" -ForegroundColor Cyan
Write-Host "===================================" -ForegroundColor Cyan

# Create directories
Write-Host "Creating directories..." -ForegroundColor Green
New-Item -ItemType Directory -Force -Path browser_data\cache | Out-Null
New-Item -ItemType Directory -Force -Path browser_data\storage | Out-Null
New-Item -ItemType Directory -Force -Path plugins | Out-Null
New-Item -ItemType Directory -Force -Path assets | Out-Null

# Check for Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Rust..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "rustup-init.exe"
    .\rustup-init.exe -y
    Remove-Item rustup-init.exe
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

# Download WebView2 Runtime
Write-Host "Checking WebView2 Runtime..." -ForegroundColor Green
$webview2_url = "https://go.microsoft.com/fwlink/p/?LinkId=2124703"
if (!(Test-Path "HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}")) {
    Write-Host "Downloading WebView2 Runtime..." -ForegroundColor Yellow
    Invoke-WebRequest -Uri $webview2_url -OutFile "MicrosoftEdgeWebview2Setup.exe"
    Start-Process -FilePath "MicrosoftEdgeWebview2Setup.exe" -ArgumentList "/silent /install" -Wait
    Remove-Item MicrosoftEdgeWebview2Setup.exe
}

# Download Flash Player
Write-Host "Downloading Flash Player 32.0.0.465..." -ForegroundColor Green
$flash_url = "https://archive.org/download/flashplayerarchive/pub/flashplayer/installers/archive/fp_32.0.0.465_archive/flashplayer32_0r0_465_win.exe"
$flash_dll = "plugins\pepflashplayer32_32_0_0_465.dll"

if (!(Test-Path $flash_dll)) {
    try {
        # Alternative: Direct DLL download (if available)
        $dll_url = "https://archive.org/download/flashplayer_old/pepflashplayer32_32_0_0_465.dll"
        Invoke-WebRequest -Uri $dll_url -OutFile $flash_dll -ErrorAction Stop
        Write-Host "Flash Player downloaded successfully!" -ForegroundColor Green
    }
    catch {
        Write-Host "Flash download failed. Please manually download from:" -ForegroundColor Yellow
        Write-Host "https://archive.org/details/flashplayerarchive" -ForegroundColor Yellow
        Write-Host "Look for: flashplayer32_0r0_465_win.exe" -ForegroundColor Yellow
    }
}

# Download Ruffle
Write-Host "Downloading Ruffle WebAssembly..." -ForegroundColor Green
try {
    Invoke-WebRequest -Uri "https://unpkg.com/@ruffle-rs/ruffle@latest/ruffle.js" -OutFile "assets\ruffle.js"
}
catch {
    Write-Host "Ruffle download failed. Browser will use CDN fallback." -ForegroundColor Yellow
}

# Build the browser
Write-Host "Building Minimalist Browser..." -ForegroundColor Green
cargo build --release

Write-Host "===================================" -ForegroundColor Cyan
Write-Host "Setup complete!" -ForegroundColor Green
Write-Host "Run: .\target\release\minimalist-browser.exe" -ForegroundColor Yellow
Write-Host "===================================" -ForegroundColor Cyan