use std::path::{Path, PathBuf};

use crate::routing::Route;

pub struct RouteScanner {
    base_path: PathBuf,
}

impl RouteScanner {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    pub async fn scan(&self) -> anyhow::Result<Vec<Route>> {
        let mut routes = Vec::new();
        let base_path = Path::new(&self.base_path);
        
        if !base_path.exists() {
            tracing::info!("App directory does not exist, creating: {}", self.base_path);
            std::fs::create_dir_all(base_path)?;
        }
        
        self.scan_directory(base_path, &mut routes, "")?;
        
        tracing::info!("Scanned {} routes", routes.len());
        for route in &routes {
            tracing::debug!("Found route: {}", route);
        }
        
        Ok(routes)
    }

    fn scan_directory(&self, path: &Path, routes: &mut Vec<Route>, prefix: &str) -> anyhow::Result<()> {
        if !path.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_dir() {
                let dir_name = entry.file_name().to_string_lossy();
                
                if dir_name == "api" {
                    self.scan_api_directory(&file_path, routes, prefix)?;
                } else if dir_name.starts_with('_') || dir_name.starts_with('.') {
                    continue;
                } else {
                    let new_prefix = if prefix.is_empty() {
                        format!("/{}", dir_name)
                    } else {
                        format!("{}/{}", prefix, dir_name)
                    };
                    self.scan_directory(&file_path, routes, &new_prefix)?;
                }
            } else if let Some(ext) = file_path.extension() {
                if ext == "tsx" || ext == "ts" || ext == "jsx" || ext == "js" {
                    if let Some(stem) = file_path.file_stem() {
                        let stem_str = stem.to_string_lossy();
                        
                        if stem_str == "page" {
                            routes.push(Route {
                                path: if prefix.is_empty() { "/".to_string() } else { prefix.clone() },
                                method: crate::routing::RouteMethod::Get,
                                file_path: file_path.to_string_lossy().to_string(),
                                handler_name: format!("{}Page", prefix.replace('/', "_")),
                            });
                        } else if stem_str == "layout" {
                        } else if stem_str == "route" {
                            self.parse_route_file(&file_path, routes, prefix)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn scan_api_directory(&self, path: &Path, routes: &mut Vec<Route>, prefix: &str) -> anyhow::Result<()> {
        let api_prefix = if prefix.is_empty() { "/api".to_string() } else { format!("{}/api", prefix) };
        
        if !path.is_dir() {
            return Ok(());
        }

        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_path = entry.path();

            if file_path.is_dir() {
                let dir_name = entry.file_name().to_string_lossy();
                let new_prefix = format!("{}/{}", api_prefix, dir_name);
                self.scan_api_directory(&file_path, routes, &new_prefix)?;
            } else if let Some(ext) = file_path.extension() {
                if ext == "ts" || ext == "js" {
                    if let Some(stem) = file_path.file_stem() {
                        if stem == "route" {
                            self.parse_route_file(&file_path, routes, &api_prefix)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_route_file(&self, path: &Path, routes: &mut Vec<Route>, prefix: &str) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let methods = ["GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD"];
        
        for method in methods {
            if content.contains(&format!("export async function {}", method)) || 
               content.contains(&format!("export const {} ", method)) ||
               content.contains(&format!("export {} ", method)) {
                let route_path = if prefix.is_empty() { "/".to_string() } else { prefix.clone() };
                routes.push(Route {
                    path: route_path,
                    method: match method {
                        "GET" => crate::routing::RouteMethod::Get,
                        "POST" => crate::routing::RouteMethod::Post,
                        "PUT" => crate::routing::RouteMethod::Put,
                        "DELETE" => crate::routing::RouteMethod::Delete,
                        "PATCH" => crate::routing::RouteMethod::Patch,
                        "OPTIONS" => crate::routing::RouteMethod::Options,
                        "HEAD" => crate::routing::RouteMethod::Head,
                        _ => crate::routing::RouteMethod::Get,
                    },
                    file_path: path.to_string_lossy().to_string(),
                    handler_name: format!("{}Handler", method.to_lowercase()),
                });
            }
        }
        
        Ok(())
    }
}
