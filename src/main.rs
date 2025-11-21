use std::sync::{Arc, Mutex};
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
        dpi::LogicalSize,
    },
    webview::WebViewBuilder,
};
use log::info;

mod browser_core;
mod assets;

use browser_core::BrowserCore;

const SHELL_HTML: &str = include_str!("./shell.html");

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

    let _webview = WebViewBuilder::new(window)?
        .with_html(SHELL_HTML)?
        .with_custom_protocol("minimalist".into(), move |request| {
            let path = request.uri().path();
            let _browser = browser_clone.lock().unwrap();
            
            let content = match path {
                "/newtab" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("newtab") {
                        page.content.as_bytes().to_vec()
                    } else {
                        b"Page not found".to_vec()
                    }
                }
                "/settings" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("settings") {
                        page.content.as_bytes().to_vec()
                    } else {
                        b"Page not found".to_vec()
                    }
                }
                "/memory" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("memory") {
                        page.content.as_bytes().to_vec()
                    } else {
                        b"Page not found".to_vec()
                    }
                }
                "/flash" => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("flash") {
                        page.content.as_bytes().to_vec()
                    } else {
                        b"Page not found".to_vec()
                    }
                }
                _ => {
                    if let Some(page) = assets::browser_pages::BROWSER_PAGES.get("error") {
                        page.content.as_bytes().to_vec()
                    } else {
                        b"Page not found".to_vec()
                    }
                }
            };
            
            wry::http::Response::builder()
                .header("Content-Type", "text/html; charset=utf-8")
                .body(content.into())
                .unwrap()
        })
        .with_devtools(true)
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