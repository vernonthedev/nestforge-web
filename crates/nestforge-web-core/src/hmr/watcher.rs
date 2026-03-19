use notify::{Event, EventKind, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    watched_paths: Vec<PathBuf>,
    debounce_ms: u64,
}

impl FileWatcher {
    pub fn new(debounce_ms: u64) -> anyhow::Result<Self> {
        let watcher =
            notify::recommended_watcher(move |res: Result<Event, notify::Error>| match res {
                Ok(event) => {
                    tracing::debug!("File event: {:?}", event);
                }
                Err(e) => {
                    tracing::error!("Watch error: {:?}", e);
                }
            })?;

        Ok(Self {
            watcher,
            watched_paths: Vec::new(),
            debounce_ms,
        })
    }

    pub fn watch(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref().to_path_buf();
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        self.watcher.watch(&path, RecursiveMode::Recursive)?;
        self.watched_paths.push(path);
        Ok(())
    }

    pub fn unwatch(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        self.watcher.unwatch(path)?;
        self.watched_paths.retain(|p| p != path);
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedFile {
    pub path: PathBuf,
    pub kind: FileChangeKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileChangeKind {
    Created,
    Modified,
    Deleted,
    Renamed,
}

impl From<EventKind> for FileChangeKind {
    fn from(kind: EventKind) -> Self {
        match kind {
            EventKind::Create(_) => FileChangeKind::Created,
            EventKind::Modify(_) => FileChangeKind::Modified,
            EventKind::Remove(_) => FileChangeKind::Deleted,
            _ => FileChangeKind::Modified,
        }
    }
}

pub struct HmrServer {
    pub port: u16,
    connections: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<HmrMessage>>>>,
}

impl HmrServer {
    pub fn new(port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            port,
            connections: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn notify_change(&self, files: Vec<ChangedFile>) {
        let message = HmrMessage::Reload { files };
        let connections = self.connections.write().await;

        for tx in connections.iter() {
            let _ = tx.try_send(message.clone());
        }
    }

    pub async fn broadcast_message(&self, message: HmrMessage) {
        let connections = self.connections.write().await;

        for tx in connections.iter() {
            let _ = tx.try_send(message.clone());
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HmrMessage {
    Reload { files: Vec<ChangedFile> },
    FullReload { reason: String },
    ModuleUpdate { module: String, code: String },
    Error { message: String },
    Connected { client_id: String },
}

pub struct HmrClient {
    ws_url: String,
}

impl HmrClient {
    pub fn new(ws_url: &str) -> Self {
        Self {
            ws_url: ws_url.to_string(),
        }
    }

    pub fn generate_script() -> String {
        r#"
(function() {
    var ws = null;
    var reconnectAttempts = 0;
    var maxReconnectAttempts = 10;
    
    function connect() {
        var protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        ws = new WebSocket(protocol + '//' + window.location.host + '/__hmr');
        
        ws.onopen = function() {
            console.log('[HMR] Connected to dev server');
            reconnectAttempts = 0;
        };
        
        ws.onmessage = function(event) {
            var data = JSON.parse(event.data);
            
            switch (data.type) {
                case 'reload':
                    console.log('[HMR] Reloading modules:', data.files);
                    if (window.__nestforgeWebHot) {
                        window.__nestforgeWebHot.invalidate();
                    } else {
                        window.location.reload();
                    }
                    break;
                case 'fullReload':
                    console.log('[HMR] Full reload:', data.reason);
                    window.location.reload();
                    break;
                case 'moduleUpdate':
                    console.log('[HMR] Module updated:', data.module);
                    break;
                case 'error':
                    console.error('[HMR] Error:', data.message);
                    break;
            }
        };
        
        ws.onclose = function() {
            console.log('[HMR] Disconnected');
            if (reconnectAttempts < maxReconnectAttempts) {
                reconnectAttempts++;
                setTimeout(connect, 1000 * reconnectAttempts);
            }
        };
    }
    
    connect();
    window.__hmrClient = ws;
})();
"#
        .to_string()
    }
}
