// browser_pages.rs - Fixed version with proper string escaping
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref BROWSER_PAGES: HashMap<&'static str, BrowserPage> = {
        let mut pages = HashMap::new();
        
        pages.insert("newtab", BrowserPage {
            title: "New Tab",
            content: NEW_TAB_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("settings", BrowserPage {
            title: "Settings",
            content: SETTINGS_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("memory", BrowserPage {
            title: "Memory Statistics",
            content: MEMORY_STATS_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("flash", BrowserPage {
            title: "Flash Test",
            content: FLASH_TEST_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("error", BrowserPage {
            title: "Error",
            content: ERROR_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages
    };
}

pub struct BrowserPage {
    pub title: &'static str,
    pub content: &'static str,
    pub content_type: &'static str,
}

// Minimal New Tab Page
const NEW_TAB_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>New Tab</title>
  <style>
    body {
      margin: 0;
      padding: 20px;
      background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
      color: #e6eef3;
      font-family: system-ui, -apple-system, "Segoe UI", Roboto;
    }
    .container { max-width: 1100px; margin: 0 auto; }
    h1 { font-size: 28px; margin: 0 0 24px 0; }
    .stats {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 16px;
      margin-bottom: 32px;
    }
    .stat-card {
      background: rgba(11, 18, 32, 0.8);
      border-radius: 12px;
      padding: 20px;
      text-align: center;
    }
    .stat-value { font-size: 24px; font-weight: 700; color: #4fd1c5; }
    .stat-label { font-size: 12px; color: #9aa6b2; margin-top: 8px; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Minimalist Browser</h1>
    <div class="stats">
      <div class="stat-card">
        <div class="stat-value" id="memUsage">--</div>
        <div class="stat-label">Memory Usage</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" id="tabCount">1</div>
        <div class="stat-label">Open Tabs</div>
      </div>
      <div class="stat-card">
        <div class="stat-value" id="boosterStatus">OFF</div>
        <div class="stat-label">Booster Mode</div>
      </div>
    </div>
  </div>
</body>
</html>"#;

const SETTINGS_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings</title>
  <style>
    body {
      margin: 0;
      padding: 40px;
      background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
      color: #e6eef3;
      font-family: system-ui;
    }
    .container { max-width: 800px; margin: 0 auto; }
    h1 { font-size: 28px; margin-bottom: 32px; }
    .section {
      background: rgba(11, 18, 32, 0.8);
      border-radius: 12px;
      padding: 24px;
      margin-bottom: 16px;
    }
    h2 { font-size: 18px; margin-top: 0; color: #4fd1c5; }
    .setting {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 12px 0;
      border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    }
    .setting:last-child { border: none; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Settings</h1>
    <div class="section">
      <h2>Performance</h2>
      <div class="setting">
        <span>Booster Mode</span>
        <input type="checkbox">
      </div>
    </div>
  </div>
</body>
</html>"#;

const MEMORY_STATS_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Memory Stats</title>
  <style>
    body {
      margin: 0;
      padding: 40px;
      background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
      color: #e6eef3;
      font-family: system-ui;
    }
    .container { max-width: 1000px; margin: 0 auto; }
    h1 { font-size: 28px; margin-bottom: 32px; }
    .stats-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
      gap: 16px;
    }
    .stat-card {
      background: rgba(11, 18, 32, 0.8);
      border-radius: 12px;
      padding: 20px;
    }
    .stat-title { font-size: 13px; color: #9aa6b2; margin-bottom: 12px; }
    .stat-value { font-size: 32px; font-weight: 700; color: #4fd1c5; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Memory Statistics</h1>
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-title">Total Memory</div>
        <div class="stat-value">-- MB</div>
      </div>
      <div class="stat-card">
        <div class="stat-title">Tab Count</div>
        <div class="stat-value">--</div>
      </div>
    </div>
  </div>
</body>
</html>"#;

const FLASH_TEST_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Flash Test</title>
  <style>
    body {
      margin: 0;
      padding: 40px;
      background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
      color: #e6eef3;
      font-family: system-ui;
    }
    .container { max-width: 800px; margin: 0 auto; }
    h1 { font-size: 28px; margin-bottom: 32px; }
    .info {
      background: rgba(11, 18, 32, 0.8);
      border-radius: 12px;
      padding: 24px;
    }
    button {
      background: #4fd1c5;
      color: #0f1720;
      border: none;
      padding: 10px 20px;
      border-radius: 8px;
      cursor: pointer;
      font-weight: 600;
      margin-top: 16px;
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Flash Player Test</h1>
    <div class="info">
      <p>Flash support: <strong>Enabled via Ruffle</strong></p>
      <button onclick="alert('Flash test - Ruffle emulator ready')">Test Flash</button>
    </div>
  </div>
</body>
</html>"#;

const ERROR_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Error</title>
  <style>
    body {
      margin: 0;
      padding: 40px;
      background: linear-gradient(135deg, #0f1720 0%, #071017 100%);
      color: #e6eef3;
      font-family: system-ui;
      display: flex;
      align-items: center;
      justify-content: center;
      min-height: 100vh;
    }
    .container { text-align: center; max-width: 500px; }
    h1 { font-size: 32px; margin-bottom: 16px; }
    p { color: #9aa6b2; line-height: 1.6; }
  </style>
</head>
<body>
  <div class="container">
    <h1>Page Not Found</h1>
    <p>The page you're looking for couldn't be loaded.</p>
  </div>
</body>
</html>"#;