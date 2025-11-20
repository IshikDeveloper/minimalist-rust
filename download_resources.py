#!/usr/bin/env python3
"""
Resource downloader for Minimalist Browser
Downloads Flash, Ruffle, and other required resources
"""

import os
import sys
import urllib.request
import zipfile
import tarfile
import platform

RESOURCES = {
    'flash_windows': {
        'url': 'https://fpdownload.macromedia.com/pub/flashplayer/updaters/32/flashplayer_32_sa.exe',
        'path': 'plugins/pepflashplayer32_32_0_0_465.dll',
        'extract': False
    },
    'flash_linux': {
        'url': 'https://archive.org/download/flashplayerarchive/pub/flashplayer/installers/archive/fp_32.0.0.465_archive/flashplayer32_0r0_465_linux.x86_64.tar.gz',
        'path': 'plugins/libpepflashplayer.so',
        'extract': True
    },
    'ruffle': {
        'url': 'https://github.com/ruffle-rs/ruffle/releases/latest/download/ruffle_web_demo.zip',
        'path': 'assets/ruffle',
        'extract': True
    }
}

def download_file(url, dest):
    """Download a file with progress indicator"""
    print(f"Downloading {url}...")
    try:
        with urllib.request.urlopen(url) as response:
            total_size = int(response.headers.get('Content-Length', 0))
            downloaded = 0
            chunk_size = 8192
            
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            
            with open(dest, 'wb') as f:
                while True:
                    chunk = response.read(chunk_size)
                    if not chunk:
                        break
                    f.write(chunk)
                    downloaded += len(chunk)
                    if total_size > 0:
                        percent = (downloaded / total_size) * 100
                        print(f"Progress: {percent:.1f}%", end='\r')
            
            print(f"\nDownloaded to {dest}")
            return True
    except Exception as e:
        print(f"Download failed: {e}")
        return False

def extract_archive(archive_path, extract_to):
    """Extract zip or tar.gz archives"""
    print(f"Extracting {archive_path}...")
    try:
        if archive_path.endswith('.zip'):
            with zipfile.ZipFile(archive_path, 'r') as z:
                z.extractall(extract_to)
        elif archive_path.endswith('.tar.gz'):
            with tarfile.open(archive_path, 'r:gz') as t:
                t.extractall(extract_to)
        print(f"Extracted to {extract_to}")
        os.remove(archive_path)
    except Exception as e:
        print(f"Extraction failed: {e}")

def main():
    print("=================================")
    print("Minimalist Browser Resource Setup")
    print("=================================")
    
    system = platform.system().lower()
    
    # Download Flash based on platform
    if system == 'windows':
        resource = RESOURCES['flash_windows']
    elif system == 'linux':
        resource = RESOURCES['flash_linux']
    else:
        print(f"Flash not available for {system}")
        resource = None
    
    if resource:
        temp_file = 'temp_flash_download'
        if download_file(resource['url'], temp_file):
            if resource['extract']:
                extract_archive(temp_file, 'plugins/')
            else:
                os.rename(temp_file, resource['path'])
    
    # Download Ruffle
    ruffle = RESOURCES['ruffle']
    if download_file(ruffle['url'], 'ruffle.zip'):
        extract_archive('ruffle.zip', ruffle['path'])
    
    print("\nSetup complete!")
    print("Important links for manual downloads:")
    print("- Flash Archive: https://archive.org/details/flashplayerarchive")
    print("- Ruffle Releases: https://github.com/ruffle-rs/ruffle/releases")
    print("- WebView2: https://developer.microsoft.com/en-us/microsoft-edge/webview2/")

if __name__ == "__main__":
    main()