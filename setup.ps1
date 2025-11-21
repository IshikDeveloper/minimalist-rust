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
$flash_dll = "plugins\pepflashplayer32_32_0_0_465.dll"

if (!(Test-Path $flash_dll)) {
    try {
        # Try direct DLL download from Archive.org
        $dll_url = "https://archive.org/download/flashplayerarchivedversions2/99/fp_11.7.700.242_archive.zip/fp_11.7.700.242_archive%2F11_7_r700_242%2Fflashplayer_11_7r700_242_win.exe"
        Write-Host "Attempting to download Flash from: $dll_url" -ForegroundColor Yellow
        Invoke-WebRequest -Uri $dll_url -OutFile "flash_installer.exe" -ErrorAction Stop
        Write-Host "Flash installer downloaded. You'll need to extract the DLL manually." -ForegroundColor Yellow
        Write-Host "Or use: https://archive.org/details/flashplayerarchive" -ForegroundColor Yellow
    }
    catch {
        Write-Host "Flash download failed. Please manually download from:" -ForegroundColor Yellow
        Write-Host "https://archive.org/download/flashplayerarchive/pub/flashplayer/installers/archive/fp_32.0.0.465_archive/" -ForegroundColor Yellow
        Write-Host "" -ForegroundColor Yellow
        Write-Host "Options:" -ForegroundColor Yellow
        Write-Host "1. flashplayer32_0r0_465_win.exe (installer)" -ForegroundColor Yellow
        Write-Host "2. Or directly: pepflashplayer32_32_0_0_465.dll" -ForegroundColor Yellow
        Write-Host "" -ForegroundColor Yellow
        Write-Host "Place the DLL in: $flash_dll" -ForegroundColor Yellow
        Write-Host "The browser will use Ruffle emulator as fallback." -ForegroundColor Yellow
    }
}
else {
    Write-Host "Flash Player already present at $flash_dll" -ForegroundColor Green
}

# Download Ruffle
Write-Host "Downloading Ruffle WebAssembly..." -ForegroundColor Green
try {
    Invoke-WebRequest -Uri "https://unpkg.com/@ruffle-rs/ruffle@latest/ruffle.js" -OutFile "assets\ruffle.js"
    Write-Host "Ruffle downloaded successfully!" -ForegroundColor Green
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
Write-Host "" -ForegroundColor Cyan
Write-Host "Note: Flash is optional. The browser works with Ruffle emulation." -ForegroundColor Cyan