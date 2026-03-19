use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::config::NestForgeWebConfig;
use crate::openapi::spec::OpenApiSpec;

pub struct SwaggerUi {
    spec: OpenApiSpec,
}

impl SwaggerUi {
    pub fn new(spec: OpenApiSpec) -> Self {
        Self { spec }
    }

    pub fn into_router(self, config: NestForgeWebConfig) -> Router {
        let spec = Arc::new(self.spec);

        Router::new()
            .route("/docs", get(docs_handler))
            .route("/docs/openapi.json", get(openapi_json_handler))
            .route("/docs/openapi.yaml", get(openapi_yaml_handler))
            .with_state(spec)
    }
}

async fn docs_handler() -> Html<String> {
    Html(SWAGGER_HTML.to_string())
}

async fn openapi_json_handler(State(spec): State<Arc<OpenApiSpec>>) -> impl IntoResponse {
    let json = serde_json::to_string_pretty(&*spec).unwrap_or_default();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(json)
        .unwrap_or_else(|_| Response::builder().status(500).body(String::new()).unwrap())
}

async fn openapi_yaml_handler(State(spec): State<Arc<OpenApiSpec>>) -> impl IntoResponse {
    let yaml = serde_yaml::to_string(&*spec).unwrap_or_default();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/x-yaml")
        .body(yaml)
        .unwrap_or_else(|_| Response::builder().status(500).body(String::new()).unwrap())
}

static SWAGGER_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NestForge Web - API Documentation</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css">
    <style>
        body {
            margin: 0;
            padding: 0;
        }
    </style>
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {
            SwaggerUIBundle({
                url: "/docs/openapi.json",
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "StandaloneLayout",
                deepLinking: true,
                showExtensions: true,
                showInternals: true,
            });
        };
    </script>
</body>
</html>"#;

pub struct Redoc {
    spec: OpenApiSpec,
}

impl Redoc {
    pub fn new(spec: OpenApiSpec) -> Self {
        Self { spec }
    }

    pub fn into_router(self, config: NestForgeWebConfig) -> Router {
        let spec = Arc::new(self.spec);

        Router::new()
            .route("/redoc", get(redoc_handler))
            .route("/docs/openapi.json", get(openapi_json_handler))
            .with_state(spec)
    }
}

async fn redoc_handler() -> Html<String> {
    Html(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NestForge Web - API Documentation</title>
    <link rel="stylesheet" href="https://unpkg.com/redoc@latest/bundles/redoc.standalone.css">
</head>
<body>
    <redoc spec-url="/docs/openapi.json"></redoc>
    <script src="https://unpkg.com/redoc@latest/bundles/redoc.standalone.js"></script>
</body>
</html>"#
    ))
}
