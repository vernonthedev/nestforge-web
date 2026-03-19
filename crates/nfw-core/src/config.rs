use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestForgeWebConfig {
    pub app_name: String,
    pub app_dir: String,
    pub backend_dir: String,
    pub port: u16,
    pub http_port: u16,
    pub host: String,
}

impl Default for NestForgeWebConfig {
    fn default() -> Self {
        Self {
            app_name: std::env::var("APP_NAME").unwrap_or_else(|_| "nestforge-app".to_string()),
            app_dir: std::env::var("APP_DIR").unwrap_or_else(|_| "src/app".to_string()),
            backend_dir: std::env::var("BACKEND_DIR").unwrap_or_else(|_| "src/backend".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            http_port: std::env::var("HTTP_PORT")
                .unwrap_or_else(|_| "3001".to_string())
                .parse()
                .unwrap_or(3001),
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        }
    }
}

impl NestForgeWebConfig {
    pub fn from_env() -> Self {
        Self::default()
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
}
