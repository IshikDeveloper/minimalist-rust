// browser_core.rs - Fixed version with proper structure
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Clone)]
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
    flash_handler: FlashHandler,
    memory_limit_mb: usize,
}

pub struct FlashHandler {
    plugin_path: Option<PathBuf>,
    use_ruffle: bool,
    enabled: bool,
}

impl FlashHandler {
    pub fn new() -> Self {
        let plugin_path = Self::find_flash_plugin();
        Self {
            use_ruffle: plugin_path.is_none(),
            plugin_path,
            enabled: false,
        }
    }
    
    fn find_flash_plugin() -> Option<PathBuf> {
        let paths = [
            "plugins/pepflashplayer32_32_0_0_465.dll",
            "plugins/pepflashplayer.dll",
            "plugins/libpepflashplayer.so",
        ];
        
        for path in &paths {
            let path = PathBuf::from(path);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }
    
    pub fn get_injection_script(&self) -> String {
        if !self.enabled {
            return String::new();
        }
        
        if self.plugin_path.is_some() {
            // Native Flash support
            r#"
            // Enable Flash plugin
            window.__flashEnabled = true;
            console.log('Flash Player enabled');
            "#.to_string()
        } else if self.use_ruffle {
            // Ruffle fallback
            r#"
            // Load Ruffle emulator
            const script = document.createElement('script');
            script.src = 'https://unpkg.com/@ruffle-rs/ruffle';
            document.head.appendChild(script);
            console.log('Ruffle Flash emulator loaded');
            "#.to_string()
        } else {
            String::new()
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
            return ("minimalist://newtab".to_string(), "minimalist://newtab".to_string());
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
        
        // Check if it's a domain
        if self.looks_like_domain(trimmed) {
            let url = format!("https://{}", trimmed);
            return (url, trimmed.to_string());
        }
        
        // Default to search
        let search = format!("https://www.google.com/search?q={}", 
                           urlencoding::encode(trimmed));
        (search.clone(), search)
    }
    
    fn looks_like_domain(&self, input: &str) -> bool {
        // Simple domain check
        input.contains('.') 
            && !input.contains(' ') 
            && input.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == ':')
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
        
        "New Tab".to_string()
    }
    
    pub fn toggle_booster_mode(&mut self) {
        self.booster_mode = !self.booster_mode;
    }
    
    pub fn get_booster_script(&self) -> &str {
        if self.booster_mode {
            r#"
            // Memory optimization script
            (function() {
                // Periodic garbage collection
                setInterval(() => {
                    if (window.gc) window.gc();
                }, 30000);
                
                // Clear caches
                if ('caches' in window) {
                    caches.keys().then(names => {
                        names.forEach(name => caches.delete(name));
                    });
                }
                
                // Reduce image memory
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
    
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            total_mb: self.get_process_memory(),
            tab_count: self.tabs.len(),
            booster_active: self.booster_mode,
            limit_mb: self.memory_limit_mb,
        }
    }
    
    fn get_process_memory(&self) -> usize {
        #[cfg(target_os = "windows")]
        {
            use winapi::um::psapi::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
            use winapi::um::processthreadsapi::GetCurrentProcess;
            
            unsafe {
                let mut pmc = PROCESS_MEMORY_COUNTERS::default();
                let cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
                
                if GetProcessMemoryInfo(GetCurrentProcess(), &mut pmc, cb) != 0 {
                    return (pmc.WorkingSetSize / 1_048_576) as usize;
                }
            }
        }
        
        // Default fallback
        100
    }
}

#[derive(Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_mb: usize,
    pub tab_count: usize,
    pub booster_active: bool,
    pub limit_mb: usize,
}