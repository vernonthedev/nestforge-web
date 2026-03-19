use axum::{
    extract::{Path as AxumPath, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;

use crate::config::NestForgeWebConfig;
use crate::routing::RouteScanner;
use crate::server::renderer::{PageProps, Renderer};

#[derive(Clone)]
pub struct AppState {
    pub config: NestForgeWebConfig,
    pub routes: Vec<crate::routing::Route>,
    pub renderer: Renderer,
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
        let renderer = Renderer::new(std::path::PathBuf::from(&self.config.app_dir));

        let state = Arc::new(AppState {
            config: self.config.clone(),
            routes: routes.clone(),
            renderer,
        });

        let mut app = Router::new()
            .route("/health", get(health_handler))
            .with_state(state.clone());

        for route in routes {
            if route.method == crate::routing::RouteMethod::Get {
                let route_path = route.path.clone();
                let page_path = route.file_path.clone();
                
                let state_clone = state.clone();
                app = app.route(
                    &route_path,
                    get(move |path: AxumPath<String>, query: Query<HashMap<String, String>>| {
                        let state = state_clone.clone();
                        async move {
                            page_handler(state, path.0, query.0).await
                        }
                    }),
                );
            }
        }

        if !app.routes().iter().any(|r| r.path == "/") {
            app = app.route("/", get(root_handler));
        }

        Ok(app)
    }

    pub async fn listen(&self) -> anyhow::Result<()> {
        let app = self.build().await?;
        let addr = format!("{}:{}", self.config.host, self.config.port);
        
        tracing::info!("Starting NestForge Web server on {}", addr);
        tracing::info!("Server ready at http://{}", addr);
        
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

async fn health_handler() -> &'static str {
    "healthy"
}

async fn page_handler(
    state: Arc<AppState>,
    path: String,
    search_params: HashMap<String, String>,
) -> Response {
    let route_path = if path.is_empty() || path == "/" {
        "/".to_string()
    } else {
        format!("/{}", path)
    };

    let route = state.routes.iter().find(|r| {
        r.method == crate::routing::RouteMethod::Get && r.path == route_path
    });

    match route {
        Some(route) => {
            let props = PageProps {
                params: HashMap::new(),
                search_params: search_params.into_iter().map(|(k, v)| (k, vec![v])).collect(),
            };

            match state.renderer.render(&route.file_path, props).await {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    tracing::error!("Failed to render page: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Render error: {}", e)).into_response()
                }
            }
        }
        None => {
            (StatusCode::NOT_FOUND, "Page not found").into_response()
        }
    }
}

async fn root_handler(state: State<Arc<AppState>>) -> Response {
    let page_props = PageProps::default();

    match state.renderer.get_layout_for_path("/") {
        Some(layout) => {
            match state.renderer.render_with_layout("/page.tsx", page_props, &layout).await {
                Ok(html) => Html(html).into_response(),
                Err(_) => Html(get_default_html("Welcome to NestForge Web")).into_response(),
            }
        }
        None => {
            let default_page = format!("{}/page.tsx", state.config.app_dir);
            match state.renderer.render(&default_page, page_props).await {
                Ok(html) => Html(html).into_response(),
                Err(_) => Html(get_default_html("Welcome to NestForge Web")).into_response(),
            }
        }
    }
}

fn get_default_html(title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <main>
        <h1>{}</h1>
        <p>Start building your fullstack application</p>
    </main>
</body>
</html>"#,
        title, title
    )
}

pub async fn start_dev_server(config: NestForgeWebConfig) -> anyhow::Result<()> {
    let app = NestForgeWebApp::new(config);
    app.listen().await
}

pub async fn build_for_production(app_dir: &str) -> anyhow::Result<()> {
    tracing::info!("Building NestForge Web application...");
    
    let dist_dir = format!("{}/.next", app_dir);
    std::fs::create_dir_all(&dist_dir)?;
    
    tracing::info!("Build complete. Output: {}", dist_dir);
    Ok(())
}
