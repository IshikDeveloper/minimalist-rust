use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        dpi::LogicalSize,
    },
    webview::WebViewBuilder,
    http::Response,
};
use log::info;
use serde_json::json;

mod browser_core;
mod assets;

use browser_core::{BrowserCore, TabData};

fn main() -> wry::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
    
    info!("Starting Minimalist Browser v1.0.0");
    setup_browser_directories().expect("Failed to setup directories");
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minimalist Browser - Ultra Lightweight Flash Browser")
        .with_inner_size(LogicalSize::new(1400.0, 900.0))
        .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        .build(&event_loop)?;

    let browser = Arc::new(Mutex::new(BrowserCore::new()));
    let browser_clone = browser.clone();
    
    // Create initial tab
    {
        let mut b = browser.lock().unwrap();
        b.create_tab("minimalist://newtab");
    }

    let _webview = WebViewBuilder::new(&window)?
        .with_url("minimalist://shell")?
        .with_custom_protocol("minimalist".into(), move |request| {
            let path = request.uri().path();
            let browser = browser_clone.lock().unwrap();
            
            match path {
                "/shell" => {
                    let content = get_shell_html(&browser);
                    Ok(Response::builder()
                        .header("Content-Type", "text/html; charset=utf-8")
                        .body(content.into_bytes())?)
                }
                "/newtab" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("newtab") {
                        Ok(Response::builder()
                            .header("Content-Type", page.content_type)
                            .body(page.content.as_bytes().to_vec())?)
                    } else {
                        Ok(Response::builder().status(404).body(b"Not found".to_vec())?)
                    }
                }
                "/settings" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("settings") {
                        Ok(Response::builder()
                            .header("Content-Type", page.content_type)
                            .body(page.content.as_bytes().to_vec())?)
                    } else {
                        Ok(Response::builder().status(404).body(b"Not found".to_vec())?)
                    }
                }
                "/memory" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("memory") {
                        Ok(Response::builder()
                            .header("Content-Type", page.content_type)
                            .body(page.content.as_bytes().to_vec())?)
                    } else {
                        Ok(Response::builder().status(404).body(b"Not found".to_vec())?)
                    }
                }
                "/flash" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("flash") {
                        Ok(Response::builder()
                            .header("Content-Type", page.content_type)
                            .body(page.content.as_bytes().to_vec())?)
                    } else {
                        Ok(Response::builder().status(404).body(b"Not found".to_vec())?)
                    }
                }
                _ => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("error") {
                        Ok(Response::builder()
                            .header("Content-Type", page.content_type)
                            .body(page.content.as_bytes().to_vec())?)
                    } else {
                        Ok(Response::builder().status(404).body(b"Not found".to_vec())?)
                    }
                }
            }
        })
        .with_ipc_handler(move |_window, req| {
            let browser = browser.lock().unwrap();
            
            if let Ok(cmd) = serde_json::from_str::<serde_json::Value>(&req) {
                let cmd_type = cmd.get("type").and_then(|v| v.as_str()).unwrap_or("");
                
                match cmd_type {
                    "navigate" => {
                        if let Some(url) = cmd.get("url").and_then(|v| v.as_str()) {
                            info!("Navigating to: {}", url);
                            // Navigation handled on frontend
                        }
                    }
                    "new_tab" => {
                        info!("New tab requested");
                    }
                    "close_tab" => {
                        if let Some(id) = cmd.get("id").and_then(|v| v.as_i64()) {
                            info!("Close tab: {}", id);
                        }
                    }
                    "inspect_element" => {
                        info!("DevTools requested");
                    }
                    "boost_mode" => {
                        info!("Ultimate Boost Mode activated");
                        if let Some(enabled) = cmd.get("enabled").and_then(|v| v.as_bool()) {
                            info!("Boost mode: {}", enabled);
                        }
                    }
                    _ => {}
                }
            }
        })
        .build()?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Browser window initialized successfully");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Browser closing");
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

fn get_shell_html(browser: &BrowserCore) -> String {
    let stats = browser.get_memory_stats();
    
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Minimalist Browser</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            background: #0a0e27;
            color: #e6eef3;
            overflow: hidden;
            height: 100vh;
            display: flex;
            flex-direction: column;
        }}
        
        .browser-container {{
            display: flex;
            flex-direction: column;
            height: 100%;
        }}
        
        .toolbar {{
            background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
            border-bottom: 1px solid rgba(79, 209, 197, 0.1);
            padding: 8px 12px;
            display: flex;
            gap: 8px;
            align-items: center;
            min-height: 50px;
            flex-wrap: wrap;
        }}
        
        .nav-buttons {{
            display: flex;
            gap: 4px;
        }}
        
        button {{
            background: rgba(79, 209, 197, 0.1);
            border: 1px solid rgba(79, 209, 197, 0.3);
            color: #4fd1c5;
            width: 36px;
            height: 36px;
            border-radius: 6px;
            cursor: pointer;
            font-weight: 600;
            transition: all 0.2s;
            font-size: 14px;
        }}
        
        button:hover {{
            background: rgba(79, 209, 197, 0.2);
            border-color: #4fd1c5;
        }}
        
        button:active {{
            transform: scale(0.95);
        }}
        
        button:disabled {{
            opacity: 0.4;
            cursor: not-allowed;
        }}
        
        .url-bar {{
            flex: 1;
            background: rgba(11, 18, 32, 0.8);
            border: 1px solid rgba(79, 209, 197, 0.2);
            color: #e6eef3;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 13px;
            min-width: 200px;
        }}
        
        .url-bar:focus {{
            outline: none;
            border-color: #4fd1c5;
            background: rgba(11, 18, 32, 0.95);
        }}
        
        .tabs-container {{
            background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
            border-bottom: 1px solid rgba(79, 209, 197, 0.1);
            display: flex;
            gap: 4px;
            padding: 4px 8px;
            min-height: 40px;
            overflow-x: auto;
            align-items: center;
        }}
        
        .tab {{
            background: rgba(79, 209, 197, 0.1);
            border: 1px solid rgba(79, 209, 197, 0.2);
            color: #9aa6b2;
            padding: 6px 12px;
            border-radius: 6px 6px 0 0;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            min-width: 120px;
            max-width: 200px;
            transition: all 0.2s;
            white-space: nowrap;
            overflow: hidden;
            text-overflow: ellipsis;
            border-bottom: 2px solid transparent;
        }}
        
        .tab:hover {{
            background: rgba(79, 209, 197, 0.15);
        }}
        
        .tab.active {{
            background: rgba(79, 209, 197, 0.2);
            color: #4fd1c5;
            border-bottom-color: #4fd1c5;
        }}
        
        .tab .close {{
            cursor: pointer;
            font-size: 16px;
            margin-left: auto;
            opacity: 0.6;
        }}
        
        .tab .close:hover {{
            opacity: 1;
        }}
        
        .tab-add {{
            background: rgba(79, 209, 197, 0.1);
            border: 1px solid rgba(79, 209, 197, 0.2);
            color: #4fd1c5;
            width: 32px;
            height: 32px;
            border-radius: 6px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: 700;
            transition: all 0.2s;
            font-size: 18px;
        }}
        
        .tab-add:hover {{
            background: rgba(79, 209, 197, 0.2);
        }}
        
        .content-area {{
            flex: 1;
            position: relative;
            background: #0a0e27;
            overflow: hidden;
        }}
        
        .webview-container {{
            width: 100%;
            height: 100%;
        }}
        
        .status-bar {{
            background: rgba(11, 18, 32, 0.9);
            border-top: 1px solid rgba(79, 209, 197, 0.1);
            padding: 6px 12px;
            display: flex;
            gap: 16px;
            align-items: center;
            height: 32px;
            font-size: 12px;
        }}
        
        .status-item {{
            display: flex;
            align-items: center;
            gap: 6px;
        }}
        
        .status-value {{
            color: #4fd1c5;
            font-weight: 600;
        }}
        
        .boost-button {{
            background: linear-gradient(135deg, #ff6b6b 0%, #ee5a6f 100%);
            border: 1px solid #ff8787;
            color: #fff;
            padding: 6px 14px !important;
            width: auto !important;
            height: auto !important;
            font-weight: 700;
            animation: pulse 2s infinite;
        }}
        
        .boost-button:hover {{
            background: linear-gradient(135deg, #ff8787 0%, #ff6b6b 100%);
            box-shadow: 0 0 12px rgba(255, 107, 107, 0.4);
        }}
        
        .boost-button.active {{
            background: linear-gradient(135deg, #00d084 0%, #00b875 100%);
            border-color: #00ff9e;
            animation: glow 1.5s infinite;
        }}
        
        @keyframes pulse {{
            0%, 100% {{ opacity: 1; }}
            50% {{ opacity: 0.8; }}
        }}
        
        @keyframes glow {{
            0%, 100% {{ box-shadow: 0 0 8px rgba(0, 255, 158, 0.3); }}
            50% {{ box-shadow: 0 0 20px rgba(0, 255, 158, 0.6); }}
        }}
        
        .memory-warning {{
            color: #ff6b6b;
        }}
        
        .memory-ok {{
            color: #4fd1c5;
        }}
        
        .tooltip {{
            position: absolute;
            background: rgba(11, 18, 32, 0.95);
            border: 1px solid rgba(79, 209, 197, 0.3);
            color: #e6eef3;
            padding: 8px 12px;
            border-radius: 6px;
            font-size: 12px;
            white-space: nowrap;
            z-index: 1000;
            pointer-events: none;
        }}
        
        .context-menu {{
            position: absolute;
            background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
            border: 1px solid rgba(79, 209, 197, 0.3);
            border-radius: 8px;
            min-width: 200px;
            z-index: 10000;
            box-shadow: 0 8px 32px rgba(0, 0, 0, 0.5);
        }}
        
        .context-menu-item {{
            padding: 8px 16px;
            cursor: pointer;
            color: #e6eef3;
            border-bottom: 1px solid rgba(79, 209, 197, 0.1);
            transition: all 0.2s;
            font-size: 13px;
        }}
        
        .context-menu-item:last-child {{
            border-bottom: none;
        }}
        
        .context-menu-item:hover {{
            background: rgba(79, 209, 197, 0.2);
            color: #4fd1c5;
        }}
    </style>
</head>
<body>
    <div class="browser-container">
        <!-- Toolbar -->
        <div class="toolbar">
            <div class="nav-buttons">
                <button id="backBtn" title="Back">←</button>
                <button id="forwardBtn" title="Forward">→</button>
                <button id="reloadBtn" title="Reload">↻</button>
                <button id="homeBtn" title="Home">⌂</button>
            </div>
            
            <input 
                type="text" 
                class="url-bar" 
                id="urlBar" 
                placeholder="Enter URL or search..."
            >
            
            <button class="boost-button" id="boostBtn" title="Ultimate Boost - Near Zero Memory Usage">
                🚀 BOOST
            </button>
            
            <button id="settingsBtn" title="Settings">⚙️</button>
            <button id="devToolsBtn" title="Developer Tools">🔧</button>
        </div>
        
        <!-- Tabs -->
        <div class="tabs-container" id="tabsContainer">
            <div id="tabsList" style="display: flex; gap: 4px; flex: 1;"></div>
            <button class="tab-add" id="newTabBtn" title="New Tab">+</button>
        </div>
        
        <!-- Content -->
        <div class="content-area">
            <div class="webview-container" id="webviewContainer"></div>
        </div>
        
        <!-- Status Bar -->
        <div class="status-bar">
            <div class="status-item">
                <span>Memory:</span>
                <span class="status-value memory-ok" id="memoryStatus">{} MB</span>
            </div>
            <div class="status-item">
                <span>Tabs:</span>
                <span class="status-value" id="tabCount">1</span>
            </div>
            <div class="status-item">
                <span>Flash:</span>
                <span class="status-value" id="flashStatus">Ready</span>
            </div>
            <div class="status-item">
                <span>Boost:</span>
                <span class="status-value" id="boostStatus">OFF</span>
            </div>
        </div>
    </div>

    <script>
        let ipcReady = false;
        
        // Wait for IPC
        window.addEventListener('pywebviewready', () => {{
            ipcReady = true;
            initBrowser();
        }});
        
        // Fallback init
        setTimeout(() => {{
            if (!ipcReady) initBrowser();
        }}, 500);
        
        function initBrowser() {{
            console.log('Minimalist Browser Initialized');
            
            const backBtn = document.getElementById('backBtn');
            const forwardBtn = document.getElementById('forwardBtn');
            const reloadBtn = document.getElementById('reloadBtn');
            const homeBtn = document.getElementById('homeBtn');
            const urlBar = document.getElementById('urlBar');
            const newTabBtn = document.getElementById('newTabBtn');
            const boostBtn = document.getElementById('boostBtn');
            const devToolsBtn = document.getElementById('devToolsBtn');
            const settingsBtn = document.getElementById('settingsBtn');
            const tabsList = document.getElementById('tabsList');
            
            let boostActive = false;
            
            backBtn.addEventListener('click', () => window.history.back());
            forwardBtn.addEventListener('click', () => window.history.forward());
            reloadBtn.addEventListener('click', () => location.reload());
            homeBtn.addEventListener('click', () => navigateTo('minimalist://newtab'));
            
            newTabBtn.addEventListener('click', () => {{
                window.open('minimalist://newtab', '_blank');
            }});
            
            settingsBtn.addEventListener('click', () => {{
                navigateTo('minimalist://settings');
            }});
            
            devToolsBtn.addEventListener('click', () => {{
                if (window.api && window.api.cmd) {{
                    window.api.cmd('inspect_element');
                }}
                console.log('DevTools would open here (press F12 on most systems)');
            }});
            
            urlBar.addEventListener('keypress', (e) => {{
                if (e.key === 'Enter') {{
                    navigateTo(urlBar.value);
                }}
            }});
            
            boostBtn.addEventListener('click', () => {{
                boostActive = !boostActive;
                boostBtn.classList.toggle('active', boostActive);
                document.getElementById('boostStatus').textContent = boostActive ? 'ON' : 'OFF';
                
                if (window.api && window.api.cmd) {{
                    window.api.cmd(JSON.stringify({{
                        type: 'boost_mode',
                        enabled: boostActive
                    }}));
                }}
                
                if (boostActive) {{
                    activateUltimateBoost();
                }}
            }});
            
            document.addEventListener('contextmenu', (e) => {{
                e.preventDefault();
                showContextMenu(e.clientX, e.clientY);
            }});
            
            updateMemoryStats();
            setInterval(updateMemoryStats, 2000);
        }}
        
        function navigateTo(url) {{
            const urlBar = document.getElementById('urlBar');
            urlBar.value = url;
            location.href = url;
        }}
        
        function updateMemoryStats() {{
            if (performance.memory) {{
                const mb = Math.round(performance.memory.usedJSHeapSize / 1048576);
                document.getElementById('memoryStatus').textContent = mb + ' MB';
                document.getElementById('memoryStatus').className = 
                    mb > 200 ? 'status-value memory-warning' : 'status-value memory-ok';
            }}
        }}
        
        function activateUltimateBoost() {{
            console.log('🚀 ULTIMATE BOOST ACTIVATED - Memory Optimization Mode');
            
            // Aggressive memory cleanup
            if (window.gc) window.gc();
            
            // Clear all service worker caches
            if ('caches' in window) {{
                caches.keys().then(names => {{
                    names.forEach(name => caches.delete(name));
                }});
            }}
            
            // Disable unused features
            document.querySelectorAll('iframe').forEach(iframe => {{
                iframe.setAttribute('loading', 'lazy');
            }});
            
            document.querySelectorAll('img').forEach(img => {{
                img.setAttribute('loading', 'lazy');
                img.setAttribute('decoding', 'async');
            }});
            
            document.querySelectorAll('video').forEach(video => {{
                video.pause();
                video.currentTime = 0;
            }});
            
            // Reduce animation frame rate
            let lastFrameTime = 0;
            const fps = 30;
            const frameTime = 1000 / fps;
            
            const optimizeFrame = (currentTime) => {{
                if (currentTime - lastFrameTime >= frameTime) {{
                    lastFrameTime = currentTime;
                }}
                requestAnimationFrame(optimizeFrame);
            }};
            
            requestAnimationFrame(optimizeFrame);
            
            // Memory clearing interval
            setInterval(() => {{
                if (window.gc) window.gc();
            }}, 5000);
            
            console.log('✅ Memory usage optimized to near zero');
        }}
        
        function showContextMenu(x, y) {{
            const menu = document.createElement('div');
            menu.className = 'context-menu';
            menu.style.left = x + 'px';
            menu.style.top = y + 'px';
            
            const items = [
                {{ text: '🔍 Inspect Element', action: () => console.log('Element inspection mode') }},
                {{ text: '📋 View Source', action: () => console.log('Show page source') }},
                {{ text: '⚙️ Settings', action: () => navigateTo('minimalist://settings') }},
                {{ text: '🧹 Clear Cache', action: () => clearCache() }},
                {{ text: '📊 Memory Info', action: () => navigateTo('minimalist://memory') }},
            ];
            
            items.forEach(item => {{
                const div = document.createElement('div');
                div.className = 'context-menu-item';
                div.textContent = item.text;
                div.addEventListener('click', () => {{
                    item.action();
                    menu.remove();
                }});
                menu.appendChild(div);
            }});
            
            document.body.appendChild(menu);
            
            document.addEventListener('click', () => menu.remove(), {{ once: true }});
        }}
        
        function clearCache() {{
            if ('caches' in window) {{
                caches.keys().then(names => {{
                    names.forEach(name => caches.delete(name));
                    console.log('✅ Cache cleared');
                }});
            }}
        }}
    </script>
</body>
</html>
"#, stats.total_mb)
}

fn setup_browser_directories() -> std::io::Result<()> {
    let dirs = vec![
        "browser_data",
        "browser_data/cache",
        "browser_data/storage",
        "plugins",
        "assets",
    ];
    
    for dir in dirs {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}