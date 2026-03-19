use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecursiveMode, Event, EventKind};
use std::time::Duration;

pub struct FileWatcher {
    watcher: notify::RecommendedWatcher,
    watched_paths: Vec<PathBuf>,
    debounce_ms: u64,
}

impl FileWatcher {
    pub fn new(debounce_ms: u64) -> anyhow::Result<Self> {
        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            match res {
                Ok(event) => {
                    tracing::debug!("File event: {:?}", event);
                }
                Err(e) => {
                    tracing::error!("Watch error: {:?}", e);
                }
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

    pub fn watch_pattern(&mut self, path: impl AsRef<Path>, pattern: &str) -> anyhow::Result<()> {
        self.watch(path)?;
        Ok(())
    }

    pub fn unwatch(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let path = path.as_ref();
        self.watcher.unwatch(path)?;
        self.watched_paths.retain(|p| p != path);
        Ok(())
    }

    pub fn process_events<F>(&mut self, mut callback: F) -> anyhow::Result<()>
    where
        F: FnMut(Vec<ChangedFile>) -> Vec<ChangedFile>,
    {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
    pub path: PathBuf,
    pub kind: FileChangeKind,
}

#[derive(Debug, Clone, PartialEq)]
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
            EventKind::Other => FileChangeKind::Renamed,
            _ => FileChangeKind::Modified,
        }
    }
}

pub struct HmrServer {
    pub port: u16,
    file_watcher: FileWatcher,
    connections: Arc<RwLock<Vec<tokio::sync::mpsc::Sender<HmrMessage>>>>,
}

impl HmrServer {
    pub fn new(port: u16) -> anyhow::Result<Self> {
        Ok(Self {
            port,
            file_watcher: FileWatcher::new(100)?,
            connections: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub fn watch_dir(&mut self, dir: impl AsRef<Path>) -> anyhow::Result<()> {
        self.file_watcher.watch(dir)?;
        Ok(())
    }

    pub async fn start(&self) -> anyhow::Result<()> {
        use axum::{routing::get, Router};
        
        let connections = self.connections.clone();
        
        let app = Router::new()
            .route("/__hmr", get(hmr websocket))
            .with_state(connections);

        let addr = format!("127.0.0.1:{}", self.port);
        tracing::info!("HMR server running on ws://{}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }

    pub async fn notify_change(&self, files: Vec<ChangedFile>) {
        let message = HmrMessage::Reload { files };
        let mut connections = self.connections.write().await;
        
        connections.retain(|tx| {
            tx.try_send(message.clone()).is_ok()
        });
    }

    pub async fn broadcast_message(&self, message: HmrMessage) {
        let mut connections = self.connections.write().await;
        
        connections.retain(|tx| {
            tx.try_send(message.clone()).is_ok()
        });
    }
}

#[derive(Debug, Clone)]
pub enum HmrMessage {
    Reload { files: Vec<ChangedFile> },
    FullReload { reason: String },
    ModuleUpdate { module: String, code: String },
    Error { message: String },
    Connected { client_id: String },
}

async fn hmr_websocket(
    axum::extract::ws::WebSocket upgrade,
    axum::extract::State(connections): axum::extract::State<Arc<RwLock<Vec<tokio::sync::mpsc::Sender<HmrMessage>>>>>,
) {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    let client_id = uuid::Uuid::new_v4().to_string();
    
    {
        let mut conns = connections.write().await;
        conns.push(tx.clone());
    }
    
    let (mut sender, mut receiver) = upgrade.await.into_web_socket().split();
    
    let _ = sender.send(axum::extract::ws::Message::Text(
        serde_json::json!({
            "type": "connected",
            "clientId": client_id
        }).to_string()
    ).into()).await;
    
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap_or_default();
            let _ = sender.send(axum::extract::ws::Message::Text(json).into()).await;
        }
    });
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
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const ws = new WebSocket(`${protocol}//${window.location.host}/__hmr`);
    
    ws.onopen = () => {
        console.log('[HMR] Connected to dev server');
    };
    
    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        
        switch (data.type) {
            case 'reload':
                console.log('[HMR] Reloading modules:', data.files);
                if (import.meta.hot) {
                    import.meta.hot.invalidate();
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
    
    ws.onclose = () => {
        console.log('[HMR] Disconnected, reconnecting...');
        setTimeout(() => {
            window.location.reload();
        }, 1000);
    };
    
    window.__hmrClient = ws;
})();
"#.to_string()
    }
}
