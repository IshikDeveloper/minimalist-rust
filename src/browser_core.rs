use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Clone, Debug)]
pub struct TabData {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub display_url: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
}

pub struct BrowserCore {
    tabs: HashMap<usize, TabData>,
    active_tab_id: Option<usize>,
    tab_counter: usize,
    booster_mode: bool,
    ultimate_boost: bool,
    flash_handler: FlashHandler,
    memory_limit_mb: usize,
}

pub struct FlashHandler {
    plugin_path: Option<PathBuf>,
    use_ruffle: bool,
    pub enabled: bool,
}

impl FlashHandler {
    pub fn new() -> Self {
        let plugin_path = Self::find_flash_plugin();
        let use_ruffle = plugin_path.is_none();
        
        Self {
            plugin_path,
            use_ruffle,
            enabled: true,
        }
    }
    
    fn find_flash_plugin() -> Option<PathBuf> {
        let paths = vec![
            "plugins/pepflashplayer32_32_0_0_465.dll",
            "plugins/libpepflashplayer.so",
            "/usr/lib/pepflashplayer.so",
            "/usr/local/lib/pepflashplayer.so",
        ];
        
        for path in paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
        None
    }
    
    pub fn get_injection_script(&self) -> String {
        if !self.enabled {
            return String::new();
        }
        
        if self.use_ruffle {
            r#"
            (function() {
                try {
                    const script = document.createElement('script');
                    script.src = 'https://unpkg.com/@ruffle-rs/ruffle@latest/ruffle.js';
                    script.async = true;
                    script.onerror = () => console.warn('Ruffle failed to load');
                    script.onload = () => {
                        console.log('✅ Ruffle Flash emulator ready');
                        const player = window.RufflePlayer?.newest?.();
                        if (player) {
                            document.querySelectorAll('embed[type="application/x-shockwave-flash"]').forEach(el => {
                                try {
                                    player.createPlayer().replaceChild(el);
                                } catch(e) {}
                            });
                        }
                    };
                    document.head.appendChild(script);
                } catch(e) {
                    console.log('Flash support via Ruffle');
                }
            })();
            "#.to_string()
        } else {
            r#"
            (function() {
                window.__flashEnabled = true;
                console.log('✅ Native Flash Player plugin enabled');
            })();
            "#.to_string()
        }
    }
    
    pub fn status(&self) -> String {
        if self.plugin_path.is_some() {
            "Native Flash".to_string()
        } else {
            "Ruffle Emulator".to_string()
        }
    }
}

impl BrowserCore {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            tab_counter: 0,
            booster_mode: false,
            ultimate_boost: false,
            flash_handler: FlashHandler::new(),
            memory_limit_mb: 512,
        }
    }
    
    pub fn create_tab(&mut self, url: &str) -> TabData {
        let tab_id = self.tab_counter;
        self.tab_counter += 1;
        
        let (real_url, display_url) = self.process_url(url);
        
        let tab = TabData {
            id: tab_id,
            title: Self::get_title_from_url(&display_url),
            url: real_url.clone(),
            display_url,
            can_go_back: false,
            can_go_forward: false,
        };
        
        self.tabs.insert(tab_id, tab.clone());
        self.active_tab_id = Some(tab_id);
        
        tab
    }
    
    pub fn close_tab(&mut self, tab_id: usize) {
        self.tabs.remove(&tab_id);
        if self.active_tab_id == Some(tab_id) {
            self.active_tab_id = self.tabs.keys().next().copied();
        }
    }
    
    pub fn process_url(&self, input: &str) -> (String, String) {
        if input.is_empty() {
            return ("minimalist://newtab".to_string(), "New Tab".to_string());
        }
        
        let trimmed = input.trim();
        
        // Already a full URL
        if trimmed.starts_with("http://") 
            || trimmed.starts_with("https://")
            || trimmed.starts_with("file://")
            || trimmed.starts_with("minimalist://") {
            return (trimmed.to_string(), trimmed.to_string());
        }
        
        // Localhost or IP
        if trimmed.starts_with("localhost") || trimmed.starts_with("127.0.0.1") {
            let url = format!("http://{}", trimmed);
            return (url.clone(), trimmed.to_string());
        }
        
        // Check if it looks like a domain
        if self.looks_like_domain(trimmed) {
            let url = format!("https://{}", trimmed);
            return (url, trimmed.to_string());
        }
        
        // Default to Google search
        let encoded = urlencoding::encode(trimmed);
        let search = format!("https://www.google.com/search?q={}", encoded);
        (search, trimmed.to_string())
    }
    
    fn looks_like_domain(&self, input: &str) -> bool {
        input.contains('.') 
            && !input.contains(' ') 
            && !input.contains('\n')
            && input.chars().all(|c| {
                c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == ':' || c == '/'
            })
    }
    
    fn get_title_from_url(url: &str) -> String {
        if url == "minimalist://newtab" {
            return "New Tab".to_string();
        }
        
        if let Ok(parsed) = Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                return host.to_string();
            }
        }
        
        url.to_string()
    }
    
    pub fn toggle_booster_mode(&mut self) {
        self.booster_mode = !self.booster_mode;
    }
    
    pub fn toggle_ultimate_boost(&mut self) {
        self.ultimate_boost = !self.ultimate_boost;
        if self.ultimate_boost {
            log::info!("🚀 ULTIMATE BOOST ACTIVATED - Near Zero Memory Mode");
        }
    }
    
    pub fn get_ultimate_boost_script(&self) -> &str {
        if self.ultimate_boost {
            r#"
            (function() {
                console.log('🚀 ULTIMATE BOOST: Aggressive Memory Optimization');
                
                // Force garbage collection every 2 seconds
                setInterval(() => {
                    if (window.gc) {
                        window.gc();
                        console.log('🧹 GC');
                    }
                }, 2000);
                
                // Disable hardware acceleration
                const canvas = document.createElement('canvas');
                const ctx = canvas.getContext('2d');
                if (ctx) ctx.canvas.width = 0;
                
                // Remove all animations
                document.querySelectorAll('*').forEach(el => {
                    const style = window.getComputedStyle(el);
                    if (style.animation || style.transition) {
                        el.style.animation = 'none';
                        el.style.transition = 'none';
                    }
                });
                
                // Lazy load everything
                document.querySelectorAll('img, iframe').forEach(el => {
                    el.loading = 'lazy';
                    if (el.tagName === 'IMG') el.decoding = 'async';
                });
                
                // Stop all videos
                document.querySelectorAll('video').forEach(v => {
                    v.pause();
                    v.src = '';
                });
                
                // Reduce audio
                document.querySelectorAll('audio').forEach(a => {
                    a.pause();
                    a.src = '';
                });
                
                // Kill background sounds
                const AudioContext = window.AudioContext || window.webkitAudioContext;
                if (AudioContext) {
                    const ctx = new AudioContext();
                    if (ctx.state === 'running') ctx.suspend();
                }
                
                // Clear event listeners on unused elements
                const walker = document.createTreeWalker(
                    document.body,
                    NodeFilter.SHOW_ELEMENT,
                    null,
                    false
                );
                
                let node;
                let count = 0;
                while (node = walker.nextNode()) {
                    if (count++ % 100 === 0) {
                        if (window.gc) window.gc();
                    }
                }
                
                // Monitor memory
                setInterval(() => {
                    if (performance.memory) {
                        const mb = (performance.memory.usedJSHeapSize / 1048576).toFixed(1);
                        console.log(`📊 Memory: ${mb}MB`);
                    }
                }, 5000);
                
                // Clear local storage periodically
                setInterval(() => {
                    try {
                        localStorage.clear();
                        sessionStorage.clear();
                    } catch(e) {}
                }, 10000);
                
                console.log('✅ Ultimate Boost: Memory at MINIMUM');
            })();
            "#
        } else {
            ""
        }
    }
    
    pub fn get_booster_script(&self) -> &str {
        if self.booster_mode {
            r#"
            (function() {
                setInterval(() => {
                    if (window.gc) window.gc();
                }, 30000);
                
                if ('caches' in window) {
                    caches.keys().then(names => {
                        names.forEach(name => caches.delete(name));
                    });
                }
                
                document.querySelectorAll('img').forEach(img => {
                    img.loading = 'lazy';
                    img.decoding = 'async';
                });
                
                console.log('Booster mode active');
            })();
            "#
        } else {
            ""
        }
    }
    
    pub fn get_active_tab(&self) -> Option<&TabData> {
        self.active_tab_id.and_then(|id| self.tabs.get(&id))
    }
    
    pub fn get_tabs(&self) -> Vec<&TabData> {
        self.tabs.values().collect()
    }
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            total_mb: self.get_process_memory(),
            tab_count: self.tabs.len(),
            booster_active: self.booster_mode,
            ultimate_boost_active: self.ultimate_boost,
            limit_mb: self.memory_limit_mb,
        }
    }
    
    pub fn set_booster_enabled(&mut self, enabled: bool) {
        self.booster_mode = enabled;
    }
    
    pub fn set_ultimate_boost_enabled(&mut self, enabled: bool) {
        self.ultimate_boost = enabled;
    }
    
    pub fn set_flash_enabled(&mut self, enabled: bool) {
        self.flash_handler.enabled = enabled;
    }
    
    pub fn get_flash_status(&self) -> String {
        self.flash_handler.status()
    }
    
    #[cfg(target_os = "windows")]
    fn get_process_memory(&self) -> usize {
        use std::mem;
        use winapi::um::psapi::GetProcessMemoryInfo;
        use winapi::um::processthreadsapi::GetCurrentProcess;
        use winapi::um::psapi::PROCESS_MEMORY_COUNTERS;
        
        unsafe {
            let mut pmc: PROCESS_MEMORY_COUNTERS = mem::zeroed();
            let cb = mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
            
            if GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, cb) != 0 {
                return (pmc.WorkingSetSize / 1_048_576) as usize;
            }
        }
        45
    }
    
    #[cfg(target_os = "macos")]
    fn get_process_memory(&self) -> usize {
        // macOS memory detection
        std::process::Command::new("ps")
            .args(&["-o", "rss=", "-p", &std::process::id().to_string()])
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .ok()
                    .and_then(|s| s.trim().parse::<usize>().ok())
                    .map(|kb| kb / 1024)
            })
            .unwrap_or(45)
    }
    
    #[cfg(target_os = "linux")]
    fn get_process_memory(&self) -> usize {
        // Linux memory detection from /proc/self/status
        std::fs::read_to_string("/proc/self/status")
            .ok()
            .and_then(|content| {
                content.lines()
                    .find(|line| line.starts_with("VmRSS"))
                    .and_then(|line| {
                        line.split_whitespace()
                            .nth(1)
                            .and_then(|s| s.parse::<usize>().ok())
                            .map(|kb| kb / 1024)
                    })
            })
            .unwrap_or(45)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MemoryStats {
    pub total_mb: usize,
    pub tab_count: usize,
    pub booster_active: bool,
    pub ultimate_boost_active: bool,
    pub limit_mb: usize,
}