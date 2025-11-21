// main.rs - Fixed version using wry properly
use std::path::PathBuf;
use wry::{
    application::{
        event::{Event, StartCause, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    },
    webview::WebViewBuilder,
};
use log::info;

mod browser_core;
mod assets;

use browser_core::BrowserCore;

fn main() -> wry::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Minimalist Browser v1.0.0");

    setup_browser_directories().expect("Failed to setup directories");
    check_flash_installation();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Minimalist Browser")
        .with_inner_size(wry::application::dpi::LogicalSize::new(1280.0, 850.0))
        .build(&event_loop)?;

    let _webview = WebViewBuilder::new(window)?
        .with_url("https://www.google.com")?
        .build()?;

    let mut browser = BrowserCore::new();
    browser.create_tab("minimalist://newtab");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Browser initialized");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                info!("Closing browser");
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

fn check_flash_installation() {
    let flash_path = PathBuf::from("plugins/pepflashplayer32_32_0_0_465.dll");
    if !flash_path.exists() {
        info!("Flash plugin not found. Browser will use Ruffle emulation.");
    } else {
        info!("Flash plugin detected at: {:?}", flash_path);
    }
}