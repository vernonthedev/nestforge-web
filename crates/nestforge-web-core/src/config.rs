use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestForgeWebConfig {
    pub app_name: String,
    pub app_dir: String,
    pub nestforge_dir: String,
    pub port: u16,
    pub host: String,
}

impl Default for NestForgeWebConfig {
    fn default() -> Self {
        Self {
            app_name: "nestforge-app".to_string(),
            app_dir: "src/app".to_string(),
            nestforge_dir: "src/nestforge".to_string(),
            port: 3000,
            host: "127.0.0.1".to_string(),
        }
    }
}
