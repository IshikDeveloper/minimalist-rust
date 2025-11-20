// main.rs - Application entry point
use std::env;
use std::path::PathBuf;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use log::info;

mod browser_core;
mod main_window;
mod assets;

use browser_core::BrowserCore;
use main_window::MinimalistWindow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("Starting Minimalist Browser v1.0.0");

    // Set up paths
    setup_browser_directories()?;
    
    // Check for Flash support
    check_flash_installation();

    // Create event loop
    let event_loop = EventLoop::new();
    let mut browser = BrowserCore::new();
    let window = MinimalistWindow::new(&event_loop);

    // Run the browser
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("Shutting down browser...");
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    browser.handle_keyboard_input(input, window.get_modifiers());
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // Update UI
                window.window.request_redraw();
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
        info!("Flash plugin not found. Run setup script to download.");
    } else {
        info!("Flash plugin detected at: {:?}", flash_path);
    }
}