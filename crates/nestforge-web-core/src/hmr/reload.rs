use crate::routing::Route;

pub struct ModuleReloader {
    cache: std::collections::HashMap<String, ModuleState>,
}

#[derive(Debug, Clone)]
pub enum ModuleState {
    Loaded,
    Reloading,
    Failed(String),
}

impl ModuleReloader {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn mark_loading(&mut self, module_path: &str) {
        self.cache
            .insert(module_path.to_string(), ModuleState::Reloading);
    }

    pub fn mark_loaded(&mut self, module_path: &str) {
        self.cache
            .insert(module_path.to_string(), ModuleState::Loaded);
    }

    pub fn mark_failed(&mut self, module_path: &str, error: String) {
        self.cache
            .insert(module_path.to_string(), ModuleState::Failed(error));
    }

    pub fn get_state(&self, module_path: &str) -> Option<&ModuleState> {
        self.cache.get(module_path)
    }
}

impl Default for ModuleReloader {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HotReloader {
    reloader: ModuleReloader,
    routes: Vec<Route>,
}

impl HotReloader {
    pub fn new(routes: Vec<Route>) -> Self {
        Self {
            reloader: ModuleReloader::new(),
            routes,
        }
    }

    pub fn reload_page(&self, page_path: &str) -> anyhow::Result<String> {
        let route = self.routes.iter().find(|r| r.path == page_path);

        match route {
            Some(route) => {
                tracing::info!("Hot reloading page: {}", route.file_path);
                Ok(format!("Reloaded: {}", route.file_path))
            }
            None => {
                anyhow::bail!("Page not found: {}", page_path)
            }
        }
    }

    pub fn get_module_dependencies(&self, module_path: &str) -> Vec<String> {
        let mut deps = Vec::new();

        for route in &self.routes {
            if route.file_path.contains(module_path) {
                deps.push(route.file_path.clone());
            }
        }

        deps
    }

    pub fn generate_vite_config(&self) -> String {
        r#"
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import nestforgeWeb from 'nestforge-web/vite';

export default defineConfig({
    plugins: [
        react(),
        nestforgeWeb({
            hotReload: true,
            hmrPort: 3001,
        }),
    ],
    server: {
        port: 3000,
    },
});
"#
        .to_string()
    }
}
