// main_window.rs - Fixed version with proper imports and integration
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
        dpi::LogicalSize,
        keyboard::{KeyCode, ModifiersState},
    },
    webview::{WebView, WebViewBuilder},
};

pub struct MinimalistWindow {
    pub window: Window,
    pub webview: WebView,
    pub tabs: Vec<TabData>,
    pub active_tab: usize,
    pub booster_active: bool,
    pub url_bar_text: String,
}

pub struct TabData {
    pub id: usize,
    pub title: String,
    pub url: String,
}

impl MinimalistWindow {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, Box<dyn std::error::Error>> {
        // Create window
        let window = WindowBuilder::new()
            .with_title("Minimalist Browser")
            .with_inner_size(LogicalSize::new(1280, 850))
            .with_min_inner_size(LogicalSize::new(600, 400))
            .with_decorations(false)
            .with_transparent(true)
            .build(event_loop)?;
        
        // Create webview
        let webview = WebViewBuilder::new(window.clone())?
            .with_url("minimalist://newtab")?
            .with_custom_protocol("minimalist".into(), |request| {
                // Handle minimalist:// protocol
                let path = request.uri().path();
                let (content, mime) = match path {
                    "/newtab" => (include_str!("../assets/newtab.html"), "text/html"),
                    "/settings" => (include_str!("../assets/settings.html"), "text/html"),
                    _ => (include_str!("../assets/404.html"), "text/html"),
                };
                
                Ok(wry::http::Response::builder()
                    .header("Content-Type", mime)
                    .body(content.as_bytes().to_vec())?)
            })
            .build()?;
        
        Ok(Self {
            window,
            webview,
            tabs: vec![TabData {
                id: 0,
                title: "New Tab".to_string(),
                url: "minimalist://newtab".to_string(),
            }],
            active_tab: 0,
            booster_active: false,
            url_bar_text: String::new(),
        })
    }
    
    pub fn navigate(&self, url: &str) {
        self.webview.load_url(url);
    }
    
    pub fn toggle_booster(&mut self) {
        self.booster_active = !self.booster_active;
        
        if self.booster_active {
            // Apply memory optimizations
            self.webview.eval(r#"
                if (window.gc) window.gc();
                console.log('Booster mode activated');
            "#).ok();
        }
    }
}