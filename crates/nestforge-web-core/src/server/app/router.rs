use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::config::NestForgeWebConfig;
use crate::routing::RouteScanner;

#[derive(Clone)]
pub struct AppState {
    pub config: NestForgeWebConfig,
    pub routes: Vec<crate::routing::Route>,
}

pub struct NestForgeWebApp {
    config: NestForgeWebConfig,
}

impl NestForgeWebApp {
    pub fn new(config: NestForgeWebConfig) -> Self {
        Self { config }
    }

    pub async fn build(&self) -> anyhow::Result<Router> {
        let scanner = RouteScanner::new(&self.config.app_dir);
        let routes = scanner.scan().await?;

        let state = Arc::new(AppState {
            config: self.config.clone(),
            routes,
        });

        let app = Router::new()
            .route("/", get(handler))
            .route("/health", get(health_handler))
            .with_state(state);

        Ok(app)
    }

    pub async fn listen(&self) -> anyhow::Result<()> {
        let app = self.build().await?;
        let addr = format!("{}:{}", self.config.host, self.config.port);
        
        tracing::info!("Starting NestForge Web server on {}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

async fn handler() -> &'static str {
    "NestForge Web"
}

async fn health_handler() -> &'static str {
    "healthy"
}
