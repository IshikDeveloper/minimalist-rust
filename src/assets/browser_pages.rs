// browser_pages.rs
// All internal browser pages with consistent minimalist design

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
        
        pages.insert("history", BrowserPage {
            title: "History",
            content: HISTORY_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("downloads", BrowserPage {
            title: "Downloads",
            content: DOWNLOADS_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("about", BrowserPage {
            title: "About Minimalist",
            content: ABOUT_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("error", BrowserPage {
            title: "Error",
            content: ERROR_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("flash", BrowserPage {
            title: "Flash Test",
            content: FLASH_TEST_HTML,
            content_type: "text/html; charset=utf-8",
        });
        
        pages.insert("memory", BrowserPage {
            title: "Memory Statistics",
            content: MEMORY_STATS_HTML,
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

// Enhanced New Tab Page with your provided HTML
const NEW_TAB_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width,initial-scale=1" />
  <title>New Tab</title>
  <meta name="description" content="Minimalist Browser New Tab Page" />
  <style>
    :root{
      --bg:#0f1720;
      --card:#0b1220;
      --muted:#9aa6b2;
      --accent:#4fd1c5;
      --glass: rgba(255,255,255,0.03);
      color-scheme: dark;
      font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", Arial;
    }
    *{box-sizing:border-box}
    html,body{height:100%;margin:0;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3}
    .wrap{max-width:1100px;margin:32px auto;padding:20px}

    header{display:flex;align-items:center;justify-content:space-between;gap:16px}
    .left{display:flex;align-items:center;gap:16px}
    .brand{font-weight:700;font-size:18px;letter-spacing:0.6px}
    .time{font-weight:600}
    .date{color:var(--muted);font-size:13px}

    .search-wrap{margin-top:20px;display:flex;gap:8px;align-items:center}
    .search{flex:1;display:flex;align-items:center;background:var(--glass);border-radius:12px;padding:10px 12px;backdrop-filter:blur(4px)}
    .search input{flex:1;border:0;background:transparent;color:inherit;font-size:16px;outline:none}
    .engine{border-radius:10px;padding:8px 10px;background:rgba(255,255,255,0.04);border:1px solid rgba(255,255,255,0.02);cursor:pointer}
    .engine select{background:transparent;border:0;color:inherit;font-weight:600;outline:none;cursor:pointer}

    .tiles{margin-top:22px;display:grid;grid-template-columns:repeat(auto-fill,minmax(140px,1fr));gap:12px}
    .tile{background:var(--card);padding:12px;border-radius:10px;display:flex;align-items:center;gap:10px;cursor:pointer;user-select:none;transition:transform 0.2s,background 0.2s}
    .tile:hover{background:#0d1420;transform:translateY(-2px)}
    .favicon{width:36px;height:36px;border-radius:8px;background:linear-gradient(135deg,#1b2430,#16202a);display:flex;align-items:center;justify-content:center;font-weight:700}
    .tile .meta{flex:1}
    .tile .title{font-weight:600;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
    .tile .url{font-size:12px;color:var(--muted);white-space:nowrap;overflow:hidden;text-overflow:ellipsis}

    .controls{display:flex;gap:8px;align-items:center}
    .btn{background:transparent;border:1px solid rgba(255,255,255,0.04);padding:6px 8px;border-radius:8px;cursor:pointer;font-size:13px;color:#e6eef3;transition:background 0.2s}
    .btn:hover{background:rgba(255,255,255,0.05)}

    .modal{position:fixed;inset:0;display:none;align-items:center;justify-content:center;backdrop-filter:blur(8px);background:rgba(0,0,0,0.5)}
    .modal.show{display:flex}
    .modal .card{background:#07121a;padding:18px;border-radius:10px;width:360px;box-shadow:0 10px 30px rgba(2,6,23,0.7)}
    .modal label{font-size:13px;color:var(--muted);display:block;margin-top:8px}
    .modal input{width:100%;padding:8px;margin-top:6px;border-radius:8px;border:1px solid rgba(255,255,255,0.03);background:transparent;color:inherit}
    .modal .row{display:flex;gap:8px;margin-top:12px}

    footer{margin-top:28px;color:var(--muted);font-size:13px;text-align:center}
    
    .stats{display:flex;gap:16px;margin-top:16px;padding:12px;background:var(--glass);border-radius:8px}
    .stat{flex:1;text-align:center}
    .stat-value{font-size:20px;font-weight:600;color:var(--accent)}
    .stat-label{font-size:12px;color:var(--muted);margin-top:4px}

    @media (max-width:520px){.wrap{padding:12px;margin:12px}.brand{font-size:15px}.tiles{grid-template-columns:repeat(2,1fr)}}
  </style>
</head>
<body>
  <div class="wrap">
    <header>
      <div class="left">
        <div class="brand">Minimalist</div>
        <div>
          <div class="time" id="time">--:--</div>
          <div class="date" id="date">Loading date...</div>
        </div>
      </div>
      <div class="controls">
        <button class="btn" onclick="window.location='minimalist://settings'">Settings</button>
        <button class="btn" id="editBtn">Edit Shortcuts</button>
        <button class="btn" id="resetBtn">Reset</button>
      </div>
    </header>

    <div class="search-wrap">
      <div class="search" id="searchBox">
        <input id="searchInput" placeholder="Search the web or type a URL (Ctrl+K)" autocomplete="off" />
        <button class="engine" id="engineBtn" title="Choose search engine">
          <select id="engineSelect">
            <option value="https://www.google.com/search?q=">Google</option>
            <option value="https://duckduckgo.com/?q=">DuckDuckGo</option>
            <option value="https://www.bing.com/search?q=">Bing</option>
            <option value="https://www.yandex.com/search/?text=">Yandex</option>
            <option value="https://search.brave.com/search?q=">Brave</option>
          </select>
        </button>
      </div>
      <button class="btn" id="goBtn">Go</button>
    </div>

    <div class="stats">
      <div class="stat">
        <div class="stat-value" id="memUsage">--</div>
        <div class="stat-label">Memory Usage</div>
      </div>
      <div class="stat">
        <div class="stat-value" id="tabCount">--</div>
        <div class="stat-label">Open Tabs</div>
      </div>
      <div class="stat">
        <div class="stat-value" id="boosterStatus">OFF</div>
        <div class="stat-label">Booster Mode</div>
      </div>
    </div>

    <section class="tiles" id="tiles"></section>

    <footer>
      Minimalist Browser • Double‑click a tile to edit • 
      <a href="minimalist://about" style="color:var(--muted)">About</a> • 
      <a href="minimalist://memory" style="color:var(--muted)">Memory Stats</a>
    </footer>
  </div>

  <div class="modal" id="modal">
    <div class="card">
      <div style="font-weight:700">Edit Shortcut</div>
      <label>Title</label>
      <input id="modalTitle" />
      <label>URL (include https://)</label>
      <input id="modalUrl" />
      <div class="row">
        <button class="btn" id="saveModal">Save</button>
        <button class="btn" id="cancelModal">Cancel</button>
      </div>
    </div>
  </div>

  <script>
    const DEFAULT_TILES = [
      {title:'YouTube',url:'https://www.youtube.com'},
      {title:'Gmail',url:'https://mail.google.com'},
      {title:'GitHub',url:'https://github.com'},
      {title:'Reddit',url:'https://www.reddit.com'},
      {title:'Stack Overflow',url:'https://stackoverflow.com'},
      {title:'Drive',url:'https://drive.google.com'},
      {title:'Twitter',url:'https://twitter.com'},
      {title:'Wikipedia',url:'https://en.wikipedia.org'},
      {title:'Discord',url:'https://discord.com'},
      {title:'News',url:'https://news.google.com'}
    ];

    function loadTiles(){
      try{const raw=localStorage.getItem('minimalist.tiles');if(raw)return JSON.parse(raw)}catch(e){}
      return DEFAULT_TILES.slice();
    }
    function saveTiles(t){localStorage.setItem('minimalist.tiles',JSON.stringify(t))}

    const tilesEl=document.getElementById('tiles');
    let tiles = loadTiles();

    function renderTiles(){tilesEl.innerHTML='';tiles.forEach((t,idx)=>{
      const el = document.createElement('div');el.className='tile';el.dataset.idx=idx;
      el.innerHTML = `
        <div class="favicon">${(t.title||'').slice(0,1).toUpperCase()}</div>
        <div class="meta"><div class="title">${escapeHtml(t.title)}</div><div class="url">${shortenUrl(t.url)}</div></div>
      `;
      el.addEventListener('click',()=>{ openUrl(t.url) });
      el.addEventListener('dblclick',(ev)=>{ ev.stopPropagation(); openEditor(idx) });
      tilesEl.appendChild(el);
    })}

    function openUrl(url){ if(!url) return; let final=url; if(!/^https?:\/\//i.test(final)) final='https://'+final; window.location.href = final }

    function shortenUrl(u){try{const url=new URL(u);return url.hostname.replace('www.','')}catch(e){return u}}
    function escapeHtml(s){return String(s).replace(/[&<>\"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":"&#39;"}[c]))}

    const modal=document.getElementById('modal');
    const modalTitle=document.getElementById('modalTitle');
    const modalUrl=document.getElementById('modalUrl');
    let editIndex=null;

    function openEditor(i){editIndex=i;const t=tiles[i];modalTitle.value=t.title;modalUrl.value=t.url;modal.classList.add('show')}
    document.getElementById('saveModal').addEventListener('click',()=>{
      const title=modalTitle.value.trim();const url=modalUrl.value.trim();
      if(editIndex===null)return;tiles[editIndex]={title:title||'Untitled',url:url||''};saveTiles(tiles);renderTiles();closeModal();
    })
    document.getElementById('cancelModal').addEventListener('click',closeModal);
    function closeModal(){modal.classList.remove('show');editIndex=null}

    document.getElementById('editBtn').addEventListener('click',()=>{openEditor(0)})
    document.getElementById('resetBtn').addEventListener('click',()=>{if(confirm('Reset shortcuts to defaults?')){tiles=DEFAULT_TILES.slice();saveTiles(tiles);renderTiles()}})

    const engineSelect=document.getElementById('engineSelect');
    const searchInput=document.getElementById('searchInput');
    document.getElementById('goBtn').addEventListener('click',doSearch);
    function doSearch(){let q=searchInput.value.trim();if(!q) return; const engine=engineSelect.value; if(/^https?:\/\//i.test(q)||/^minimalist:\/\//i.test(q)) return openUrl(q); location.href = engine + encodeURIComponent(q)}

    document.addEventListener('keydown',(e)=>{
      if((e.ctrlKey||e.metaKey) && e.key.toLowerCase()==='k'){e.preventDefault();searchInput.focus();searchInput.select();}
      if(e.key==='Enter' && document.activeElement===searchInput){doSearch()}
    })

    searchInput.addEventListener('keydown', (e)=>{ if(e.key==='/' && document.activeElement!==searchInput){ e.preventDefault(); searchInput.focus(); }})

    function updateTime(){const d=new Date();const hh=String(d.getHours()).padStart(2,'0');const mm=String(d.getMinutes()).padStart(2,'0');document.getElementById('time').textContent=`${hh}:${mm}`;document.getElementById('date').textContent=d.toLocaleDateString()}
    setInterval(updateTime,1000*30);updateTime();

    // Browser stats via IPC
    function updateStats(){
      if(window.chrome&&window.chrome.webview){
        window.chrome.webview.postMessage({cmd:'get_stats'});
        window.chrome.webview.addEventListener('message',(e)=>{
          if(e.data.type==='stats'){
            document.getElementById('memUsage').textContent=e.data.memory||'--';
            document.getElementById('tabCount').textContent=e.data.tabs||'--';
            document.getElementById('boosterStatus').textContent=e.data.booster?'ON':'OFF';
          }
        });
      }
    }
    updateStats();
    setInterval(updateStats,5000);

    renderTiles();
  </script>
</body>
</html>"#;

// Settings Page
const SETTINGS_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Settings - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5}
    *{box-sizing:border-box;margin:0;padding:0}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px}
    .container{max-width:800px;margin:0 auto}
    h1{font-size:28px;margin-bottom:32px;font-weight:700}
    .section{background:var(--card);border-radius:12px;padding:24px;margin-bottom:16px}
    h2{font-size:18px;margin-bottom:16px;color:var(--accent)}
    .setting{display:flex;align-items:center;justify-content:space-between;padding:12px 0;border-bottom:1px solid rgba(255,255,255,0.05)}
    .setting:last-child{border:none}
    .setting-label{display:flex;flex-direction:column;flex:1}
    .setting-title{font-weight:600;margin-bottom:4px}
    .setting-desc{font-size:13px;color:var(--muted)}
    .toggle{width:48px;height:24px;background:rgba(255,255,255,0.1);border-radius:12px;position:relative;cursor:pointer;transition:background 0.3s}
    .toggle.active{background:var(--accent)}
    .toggle-knob{position:absolute;width:20px;height:20px;background:white;border-radius:50%;top:2px;left:2px;transition:transform 0.3s}
    .toggle.active .toggle-knob{transform:translateX(24px)}
    select,input[type="number"]{background:transparent;border:1px solid rgba(255,255,255,0.1);color:inherit;padding:8px;border-radius:6px}
    button{background:var(--accent);color:var(--bg);border:none;padding:10px 20px;border-radius:8px;font-weight:600;cursor:pointer;transition:opacity 0.2s}
    button:hover{opacity:0.9}
    .danger{background:#e53935}
    .back-link{color:var(--muted);text-decoration:none;display:inline-block;margin-bottom:20px}
    .back-link:hover{color:#e6eef3}
  </style>
</head>
<body>
  <div class="container">
    <a href="minimalist://newtab" class="back-link">← Back to New Tab</a>
    <h1>Settings</h1>
    
    <div class="section">
      <h2>Performance</h2>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Booster Mode</div>
          <div class="setting-desc">Aggressive memory optimization and reduced features</div>
        </div>
        <div class="toggle" id="boosterToggle" onclick="toggleSetting('booster')">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Memory Limit</div>
          <div class="setting-desc">Maximum RAM usage per tab (MB)</div>
        </div>
        <input type="number" id="memLimit" value="512" min="128" max="2048">
      </div>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Lazy Loading</div>
          <div class="setting-desc">Load images only when visible</div>
        </div>
        <div class="toggle active" id="lazyToggle" onclick="toggleSetting('lazy')">
          <div class="toggle-knob"></div>
        </div>
      </div>
    </div>

    <div class="section">
      <h2>Flash Support</h2>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Enable Flash</div>
          <div class="setting-desc">Use Flash Player 32.0.0.465 or Ruffle emulator</div>
        </div>
        <div class="toggle" id="flashToggle" onclick="toggleSetting('flash')">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Flash Mode</div>
          <div class="setting-desc">Choose Flash implementation</div>
        </div>
        <select id="flashMode">
          <option value="native">Native Plugin (if available)</option>
          <option value="ruffle">Ruffle Emulator</option>
        </select>
      </div>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Click to Activate</div>
          <div class="setting-desc">Require user interaction to run Flash content</div>
        </div>
        <div class="toggle active" id="flashClickToggle" onclick="toggleSetting('flashClick')">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div class="setting">
        <button onclick="window.location='minimalist://flash'">Test Flash →</button>
      </div>
    </div>

    <div class="section">
      <h2>Privacy</h2>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Do Not Track</div>
          <div class="setting-desc">Send DNT header with requests</div>
        </div>
        <div class="toggle active" id="dntToggle" onclick="toggleSetting('dnt')">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Block Third-Party Cookies</div>
          <div class="setting-desc">Prevent tracking cookies</div>
        </div>
        <div class="toggle" id="cookieToggle" onclick="toggleSetting('cookies')">
          <div class="toggle-knob"></div>
        </div>
      </div>
      <div class="setting">
        <button class="danger" onclick="clearData()">Clear Browsing Data</button>
      </div>
    </div>

    <div class="section">
      <h2>Appearance</h2>
      <div class="setting">
        <div class="setting-label">
          <div class="setting-title">Theme</div>
          <div class="setting-desc">Visual appearance</div>
        </div>
        <select id="theme">
          <option value="dark">Dark (Default)</option>
          <option value="darker">Darker</option>
          <option value="oled">OLED Black</option>
        </select>
      </div>
    </div>
  </div>

  <script>
    function toggleSetting(id) {
      const toggle = document.getElementById(id + 'Toggle');
      toggle.classList.toggle('active');
      saveSetting(id, toggle.classList.contains('active'));
    }
    
    function saveSetting(key, value) {
      localStorage.setItem('minimalist.settings.' + key, JSON.stringify(value));
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'update_setting', key, value});
      }
    }
    
    function clearData() {
      if (confirm('Clear all browsing data? This cannot be undone.')) {
        localStorage.clear();
        sessionStorage.clear();
        if (window.chrome?.webview) {
          window.chrome.webview.postMessage({cmd: 'clear_all_data'});
        }
        alert('Data cleared. Restarting browser...');
        window.location = 'minimalist://newtab';
      }
    }
    
    // Load saved settings
    document.addEventListener('DOMContentLoaded', () => {
      const settings = ['booster', 'lazy', 'flash', 'flashClick', 'dnt', 'cookies'];
      settings.forEach(s => {
        const saved = localStorage.getItem('minimalist.settings.' + s);
        if (saved !== null) {
          const toggle = document.getElementById(s + 'Toggle');
          if (toggle) {
            if (JSON.parse(saved)) {
              toggle.classList.add('active');
            } else {
              toggle.classList.remove('active');
            }
          }
        }
      });
    });
  </script>
</body>
</html>"#;

// History Page
const HISTORY_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>History - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5}
    *{box-sizing:border-box;margin:0;padding:0}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px}
    .container{max-width:900px;margin:0 auto}
    h1{font-size:28px;margin-bottom:24px}
    .search-bar{background:rgba(255,255,255,0.05);border-radius:8px;padding:12px;margin-bottom:24px;display:flex;gap:8px}
    .search-bar input{flex:1;background:transparent;border:none;color:inherit;outline:none;font-size:16px}
    .history-item{background:var(--card);border-radius:8px;padding:16px;margin-bottom:8px;display:flex;align-items:center;gap:12px;cursor:pointer;transition:background 0.2s}
    .history-item:hover{background:#0d1420}
    .favicon{width:32px;height:32px;border-radius:6px;background:linear-gradient(135deg,#1b2430,#16202a);display:flex;align-items:center;justify-content:center;font-weight:700}
    .history-content{flex:1}
    .history-title{font-weight:600;margin-bottom:4px}
    .history-url{font-size:13px;color:var(--muted)}
    .history-time{font-size:12px;color:var(--muted)}
    .empty{text-align:center;color:var(--muted);padding:60px 0}
    button{background:var(--accent);color:var(--bg);border:none;padding:8px 16px;border-radius:6px;cursor:pointer;font-weight:600}
    .danger{background:#e53935}
  </style>
</head>
<body>
  <div class="container">
    <h1>Browsing History</h1>
    
    <div class="search-bar">
      <input type="text" placeholder="Search history..." id="searchInput">
      <button class="danger" onclick="clearHistory()">Clear All</button>
    </div>
    
    <div id="historyList">
      <div class="empty">No browsing history yet</div>
    </div>
  </div>

  <script>
    let historyData = [];

    function loadHistory() {
      // Request history from browser via IPC
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'get_history'});
        window.chrome.webview.addEventListener('message', (e) => {
          if (e.data.type === 'history') {
            historyData = e.data.items || [];
            renderHistory();
          }
        });
      } else {
        // Fallback to localStorage
        const saved = localStorage.getItem('minimalist.history');
        historyData = saved ? JSON.parse(saved) : [];
        renderHistory();
      }
    }

    function renderHistory(filter = '') {
      const list = document.getElementById('historyList');
      const filtered = historyData.filter(item => 
        item.title.toLowerCase().includes(filter.toLowerCase()) ||
        item.url.toLowerCase().includes(filter.toLowerCase())
      );

      if (filtered.length === 0) {
        list.innerHTML = '<div class="empty">No matching history items</div>';
        return;
      }

      list.innerHTML = filtered.map(item => `
        <div class="history-item" onclick="window.location='${item.url}'">
          <div class="favicon">${item.title[0].toUpperCase()}</div>
          <div class="history-content">
            <div class="history-title">${escapeHtml(item.title)}</div>
            <div class="history-url">${item.url}</div>
          </div>
          <div class="history-time">${formatTime(item.timestamp)}</div>
        </div>
      `).join('');
    }

    function clearHistory() {
      if (confirm('Clear all browsing history?')) {
        historyData = [];
        localStorage.removeItem('minimalist.history');
        if (window.chrome?.webview) {
          window.chrome.webview.postMessage({cmd: 'clear_history'});
        }
        renderHistory();
      }
    }

    function escapeHtml(text) {
      const div = document.createElement('div');
      div.textContent = text;
      return div.innerHTML;
    }

    function formatTime(timestamp) {
      const date = new Date(timestamp);
      const now = new Date();
      const diff = now - date;
      
      if (diff < 3600000) return Math.floor(diff / 60000) + ' min ago';
      if (diff < 86400000) return Math.floor(diff / 3600000) + ' hours ago';
      return date.toLocaleDateString();
    }

    document.getElementById('searchInput').addEventListener('input', (e) => {
      renderHistory(e.target.value);
    });

    loadHistory();
  </script>
</body>
</html>"#;

// Memory Statistics Page
const MEMORY_STATS_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Memory Statistics - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5;--danger:#e53935;--success:#4caf50}
    *{box-sizing:border-box;margin:0;padding:0}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px}
    .container{max-width:1000px;margin:0 auto}
    h1{font-size:28px;margin-bottom:32px}
    .stats-grid{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:16px;margin-bottom:32px}
    .stat-card{background:var(--card);border-radius:12px;padding:20px}
    .stat-title{font-size:13px;color:var(--muted);margin-bottom:8px;text-transform:uppercase;letter-spacing:0.5px}
    .stat-value{font-size:32px;font-weight:700;color:var(--accent)}
    .stat-unit{font-size:14px;color:var(--muted);margin-left:4px}
    .progress-bar{height:4px;background:rgba(255,255,255,0.1);border-radius:2px;margin-top:12px;overflow:hidden}
    .progress-fill{height:100%;background:var(--accent);transition:width 0.3s}
    .danger .progress-fill{background:var(--danger)}
    .success .progress-fill{background:var(--success)}
    .tab-list{background:var(--card);border-radius:12px;padding:20px}
    .tab-item{display:flex;justify-content:space-between;padding:12px 0;border-bottom:1px solid rgba(255,255,255,0.05)}
    .tab-item:last-child{border:none}
    .tab-name{font-weight:600}
    .tab-memory{color:var(--muted)}
    button{background:var(--accent);color:var(--bg);border:none;padding:10px 20px;border-radius:8px;font-weight:600;cursor:pointer;margin-top:16px}
    .actions{display:flex;gap:12px;margin-top:24px}
    .refresh-icon{animation:spin 1s linear infinite}
    @keyframes spin{from{transform:rotate(0deg)}to{transform:rotate(360deg)}}
  </style>
</head>
<body>
  <div class="container">
    <h1>Memory Statistics</h1>
    
    <div class="stats-grid">
      <div class="stat-card">
        <div class="stat-title">Total Memory Usage</div>
        <div>
          <span class="stat-value" id="totalMem">--</span>
          <span class="stat-unit">MB</span>
        </div>
        <div class="progress-bar" id="totalProgress">
          <div class="progress-fill" style="width:0%"></div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-title">Browser Process</div>
        <div>
          <span class="stat-value" id="browserMem">--</span>
          <span class="stat-unit">MB</span>
        </div>
        <div class="progress-bar">
          <div class="progress-fill" style="width:0%" id="browserProgress"></div>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-title">Active Tabs</div>
        <div>
          <span class="stat-value" id="tabCount">--</span>
          <span class="stat-unit">tabs</span>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-title">Average per Tab</div>
        <div>
          <span class="stat-value" id="avgMem">--</span>
          <span class="stat-unit">MB</span>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-title">Booster Mode</div>
        <div>
          <span class="stat-value" id="boosterStatus">--</span>
        </div>
      </div>
      
      <div class="stat-card">
        <div class="stat-title">Memory Saved</div>
        <div>
          <span class="stat-value" id="savedMem">--</span>
          <span class="stat-unit">MB</span>
        </div>
      </div>
    </div>
    
    <div class="tab-list">
      <h2 style="margin-bottom:16px">Tab Memory Usage</h2>
      <div id="tabList">
        <div style="color:var(--muted);text-align:center;padding:20px">Loading tab information...</div>
      </div>
    </div>
    
    <div class="actions">
      <button onclick="collectGarbage()">Force Garbage Collection</button>
      <button onclick="refreshStats()" id="refreshBtn">
        <span id="refreshIcon">↻</span> Refresh
      </button>
      <button onclick="window.location='minimalist://settings'">Settings</button>
    </div>
  </div>

  <script>
    function updateStats() {
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'get_memory_stats'});
        window.chrome.webview.addEventListener('message', (e) => {
          if (e.data.type === 'memory_stats') {
            const stats = e.data;
            
            // Update values
            document.getElementById('totalMem').textContent = Math.round(stats.total / 1048576);
            document.getElementById('browserMem').textContent = Math.round(stats.browser / 1048576);
            document.getElementById('tabCount').textContent = stats.tabs.length;
            document.getElementById('avgMem').textContent = Math.round(stats.average / 1048576);
            document.getElementById('boosterStatus').textContent = stats.booster ? 'ON' : 'OFF';
            document.getElementById('savedMem').textContent = Math.round(stats.saved / 1048576);
            
            // Update progress bars
            const totalPercent = Math.min((stats.total / stats.limit) * 100, 100);
            document.getElementById('totalProgress').className = 
              totalPercent > 80 ? 'progress-bar danger' : 
              totalPercent < 50 ? 'progress-bar success' : 'progress-bar';
            document.querySelector('#totalProgress .progress-fill').style.width = totalPercent + '%';
            
            // Update tab list
            const tabList = document.getElementById('tabList');
            if (stats.tabs.length > 0) {
              tabList.innerHTML = stats.tabs.map(tab => `
                <div class="tab-item">
                  <div class="tab-name">${escapeHtml(tab.title)}</div>
                  <div class="tab-memory">${Math.round(tab.memory / 1048576)} MB</div>
                </div>
              `).join('');
            }
          }
        });
      }
    }

    function collectGarbage() {
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'force_gc'});
        setTimeout(refreshStats, 1000);
      }
    }

    function refreshStats() {
      const icon = document.getElementById('refreshIcon');
      icon.classList.add('refresh-icon');
      updateStats();
      setTimeout(() => icon.classList.remove('refresh-icon'), 1000);
    }

    function escapeHtml(text) {
      const div = document.createElement('div');
      div.textContent = text;
      return div.innerHTML;
    }

    updateStats();
    setInterval(updateStats, 5000);
  </script>
</body>
</html>"#;

// Flash Test Page
const FLASH_TEST_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Flash Test - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5;--success:#4caf50;--danger:#e53935}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px}
    .container{max-width:800px;margin:0 auto}
    h1{font-size:28px;margin-bottom:32px}
    .test-card{background:var(--card);border-radius:12px;padding:24px;margin-bottom:16px}
    .status{display:flex;align-items:center;gap:12px;margin-bottom:20px}
    .status-icon{width:12px;height:12px;border-radius:50%}
    .status-icon.success{background:var(--success)}
    .status-icon.danger{background:var(--danger)}
    .status-icon.warning{background:#ff9800}
    .flash-container{background:#000;border-radius:8px;padding:20px;min-height:400px;display:flex;align-items:center;justify-content:center;color:#666}
    button{background:var(--accent);color:var(--bg);border:none;padding:10px 20px;border-radius:8px;font-weight:600;cursor:pointer;margin-right:8px}
    .info{background:rgba(255,255,255,0.05);border-radius:8px;padding:16px;margin-top:16px}
    .info h3{margin-bottom:8px}
    .info p{color:var(--muted);font-size:14px;line-height:1.5}
  </style>
</head>
<body>
  <div class="container">
    <h1>Flash Player Test</h1>
    
    <div class="test-card">
      <div class="status">
        <div class="status-icon" id="statusIcon"></div>
        <span id="statusText">Checking Flash support...</span>
      </div>
      
      <div class="flash-container" id="flashContainer">
        <div id="flashContent">Flash content will appear here</div>
      </div>
      
      <div style="margin-top:16px">
        <button onclick="testFlash()">Test Flash</button>
        <button onclick="testRuffle()">Test Ruffle</button>
        <button onclick="loadSWF()">Load SWF File</button>
      </div>
    </div>
    
    <div class="info">
      <h3>Flash Information</h3>
      <p id="flashInfo">Detecting Flash configuration...</p>
    </div>
    
    <div class="info">
      <h3>Test Results</h3>
      <div id="testResults">
        <p>• Flash Plugin: <span id="pluginStatus">Checking...</span></p>
        <p>• Ruffle Emulator: <span id="ruffleStatus">Checking...</span></p>
        <p>• Click-to-Activate: <span id="ctaStatus">Checking...</span></p>
      </div>
    </div>
  </div>

  <script>
    function checkFlashSupport() {
      const hasFlashPlugin = navigator.plugins && navigator.plugins['Shockwave Flash'];
      const hasRuffle = typeof window.RufflePlayer !== 'undefined';
      
      const statusIcon = document.getElementById('statusIcon');
      const statusText = document.getElementById('statusText');
      
      if (hasFlashPlugin || hasRuffle) {
        statusIcon.className = 'status-icon success';
        statusText.textContent = hasFlashPlugin ? 
          'Flash Player 32.0.0.465 detected' : 
          'Ruffle Flash emulator available';
      } else {
        statusIcon.className = 'status-icon danger';
        statusText.textContent = 'No Flash support detected';
      }
      
      // Update detailed info
      document.getElementById('pluginStatus').textContent = 
        hasFlashPlugin ? '✓ Available' : '✗ Not found';
      document.getElementById('ruffleStatus').textContent = 
        hasRuffle ? '✓ Available' : '✗ Not loaded';
      document.getElementById('ctaStatus').textContent = 
        window.__flashClickToActivate ? 'Enabled' : 'Disabled';
      
      if (hasFlashPlugin) {
        const plugin = navigator.plugins['Shockwave Flash'];
        document.getElementById('flashInfo').innerHTML = `
          <strong>Flash Version:</strong> ${plugin.description}<br>
          <strong>Filename:</strong> ${plugin.filename}<br>
          <strong>Mode:</strong> Native PPAPI Plugin
        `;
      } else if (hasRuffle) {
        document.getElementById('flashInfo').innerHTML = `
          <strong>Emulator:</strong> Ruffle WebAssembly<br>
          <strong>Version:</strong> Latest<br>
          <strong>Mode:</strong> Flash Emulation
        `;
      }
    }

    function testFlash() {
      const container = document.getElementById('flashContent');
      container.innerHTML = `
        <object width="550" height="400" 
                data="https://www.adobe.com/devnet/flash/samples/drawing_1/ball.swf"
                type="application/x-shockwave-flash">
          <param name="movie" value="ball.swf">
          <param name="quality" value="high">
          <param name="bgcolor" value="#000000">
          <embed src="ball.swf" quality="high" bgcolor="#000000"
                 width="550" height="400"
                 type="application/x-shockwave-flash">
        </object>
      `;
    }

    function testRuffle() {
      const container = document.getElementById('flashContent');
      const player = window.RufflePlayer?.newest();
      if (player) {
        const ruffle = player.createPlayer();
        ruffle.style.width = '550px';
        ruffle.style.height = '400px';
        container.innerHTML = '';
        container.appendChild(ruffle);
        ruffle.load('https://www.newgrounds.com/portal/view/59593');
      } else {
        container.innerHTML = '<div style="color:#e53935">Ruffle not available</div>';
      }
    }

    function loadSWF() {
      const url = prompt('Enter SWF URL:');
      if (url) {
        const container = document.getElementById('flashContent');
        container.innerHTML = `
          <embed src="${url}" width="550" height="400" 
                 type="application/x-shockwave-flash"
                 allowscriptaccess="always" 
                 allowfullscreen="true">
        `;
      }
    }

    checkFlashSupport();
  </script>
</body>
</html>"#;

// About Page
const ABOUT_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>About - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px;display:flex;align-items:center;justify-content:center}
    .about-card{background:var(--card);border-radius:16px;padding:48px;max-width:500px;text-align:center}
    .logo{font-size:48px;font-weight:900;letter-spacing:1px;margin-bottom:16px;background:linear-gradient(135deg,var(--accent),#80deea);-webkit-background-clip:text;-webkit-text-fill-color:transparent}
    .version{color:var(--muted);margin-bottom:32px}
    .description{line-height:1.6;margin-bottom:32px;color:#cfd8dc}
    .stats{display:grid;grid-template-columns:1fr 1fr;gap:24px;margin:32px 0}
    .stat{text-align:center}
    .stat-value{font-size:24px;font-weight:700;color:var(--accent)}
    .stat-label{font-size:12px;color:var(--muted);margin-top:4px}
    .links{display:flex;gap:16px;justify-content:center;margin-top:32px}
    a{color:var(--accent);text-decoration:none}
    a:hover{text-decoration:underline}
  </style>
</head>
<body>
  <div class="about-card">
    <div class="logo">Minimalist</div>
    <div class="version">Version 1.0.0 • Rust Edition</div>
    
    <div class="description">
      A lightweight, memory-efficient web browser built with Rust. 
      Featuring Flash support through native plugins or Ruffle emulation, 
      aggressive memory optimization, and a clean, minimalist interface.
    </div>
    
    <div class="stats">
      <div class="stat">
        <div class="stat-value" id="buildDate">2024</div>
        <div class="stat-label">Build Year</div>
      </div>
      <div class="stat">
        <div class="stat-value">Rust</div>
        <div class="stat-label">Language</div>
      </div>
      <div class="stat">
        <div class="stat-value">WebView2</div>
        <div class="stat-label">Engine</div>
      </div>
      <div class="stat">
        <div class="stat-value" id="memUsage">< 100MB</div>
        <div class="stat-label">Base Memory</div>
      </div>
    </div>
    
    <div class="features" style="text-align:left;background:rgba(255,255,255,0.03);border-radius:8px;padding:16px;margin:24px 0">
      <strong style="display:block;margin-bottom:12px">Features:</strong>
      <div style="color:var(--muted);font-size:14px;line-height:1.8">
        • Flash Player 32.0.0.465 support<br>
        • Ruffle WebAssembly fallback<br>
        • Memory usage under 100MB idle<br>
        • Custom minimalist:// protocol<br>
        • Tab isolation and management<br>
        • Aggressive garbage collection<br>
        • Dark theme optimized UI
      </div>
    </div>
    
    <div class="links">
      <a href="minimalist://newtab">New Tab</a>
      <a href="minimalist://settings">Settings</a>
      <a href="minimalist://memory">Memory Stats</a>
    </div>
  </div>
</body>
</html>"#;

// Error Page
const ERROR_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Error - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--accent:#4fd1c5;--danger:#e53935}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;display:flex;align-items:center;justify-content:center;padding:40px}
    .error-container{text-align:center;max-width:500px}
    .error-icon{font-size:72px;margin-bottom:24px}
    h1{font-size:32px;margin-bottom:16px}
    p{color:#9aa6b2;line-height:1.6;margin-bottom:32px}
    button{background:var(--accent);color:var(--bg);border:none;padding:12px 24px;border-radius:8px;font-weight:600;cursor:pointer;margin:0 8px}
    button:hover{opacity:0.9}
  </style>
</head>
<body>
  <div class="error-container">
    <div class="error-icon">⚠️</div>
    <h1>Page Not Found</h1>
    <p>The page you're looking for couldn't be loaded. It might have been moved, deleted, or the URL might be incorrect.</p>
    <div>
      <button onclick="history.back()">Go Back</button>
      <button onclick="window.location='minimalist://newtab'">New Tab</button>
    </div>
  </div>
</body>
</html>"#;

// Downloads page
const DOWNLOADS_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Downloads - Minimalist Browser</title>
  <style>
    :root{--bg:#0f1720;--card:#0b1220;--muted:#9aa6b2;--accent:#4fd1c5}
    body{font-family:system-ui;background:linear-gradient(180deg,var(--bg),#071017);color:#e6eef3;min-height:100vh;padding:40px}
    .container{max-width:900px;margin:0 auto}
    h1{font-size:28px;margin-bottom:24px}
    .download-item{background:var(--card);border-radius:8px;padding:16px;margin-bottom:8px;display:flex;align-items:center;gap:12px}
    .download-icon{width:40px;height:40px;border-radius:6px;background:linear-gradient(135deg,#1b2430,#16202a);display:flex;align-items:center;justify-content:center}
    .download-info{flex:1}
    .download-name{font-weight:600;margin-bottom:4px}
    .download-details{font-size:13px;color:var(--muted)}
    .download-progress{height:4px;background:rgba(255,255,255,0.1);border-radius:2px;margin-top:8px}
    .download-progress-fill{height:100%;background:var(--accent);border-radius:2px;transition:width 0.3s}
    .empty{text-align:center;color:var(--muted);padding:60px 0}
    button{background:transparent;border:1px solid rgba(255,255,255,0.1);color:inherit;padding:6px 12px;border-radius:6px;cursor:pointer;font-size:13px}
    button:hover{background:rgba(255,255,255,0.05)}
  </style>
</head>
<body>
  <div class="container">
    <h1>Downloads</h1>
    <div id="downloadsList">
      <div class="empty">No downloads yet</div>
    </div>
  </div>
  
  <script>
    function loadDownloads() {
      // This would connect to the browser's download manager
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'get_downloads'});
        window.chrome.webview.addEventListener('message', (e) => {
          if (e.data.type === 'downloads') {
            renderDownloads(e.data.items || []);
          }
        });
      }
    }
    
    function renderDownloads(downloads) {
      const list = document.getElementById('downloadsList');
      if (downloads.length === 0) {
        list.innerHTML = '<div class="empty">No downloads yet</div>';
        return;
      }
      
      list.innerHTML = downloads.map(item => `
        <div class="download-item">
          <div class="download-icon">📁</div>
          <div class="download-info">
            <div class="download-name">${item.name}</div>
            <div class="download-details">${item.size} • ${item.date}</div>
            ${item.progress < 100 ? `
              <div class="download-progress">
                <div class="download-progress-fill" style="width:${item.progress}%"></div>
              </div>
            ` : ''}
          </div>
          <button onclick="openFile('${item.path}')">Open</button>
        </div>
      `).join('');
    }
    
    function openFile(path) {
      if (window.chrome?.webview) {
        window.chrome.webview.postMessage({cmd: 'open_file', path});
      }
    }
    
    loadDownloads();
  </script>
</body>
</html>"#;