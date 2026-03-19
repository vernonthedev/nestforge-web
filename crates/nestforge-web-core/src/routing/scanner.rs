use std::path::{Path, PathBuf};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::routing::{Route, RouteMethod};

static DYNAMIC_SEGMENT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\[(.+?)\]$").unwrap()
});

static CATCH_ALL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\[\.\.\.(.+?)\]$").unwrap()
});

static OPTIONAL_CATCH_ALL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\[\[(.+?)\]\]$").unwrap()
});

#[derive(Debug, Clone)]
pub enum RouteSegment {
    Static(String),
    Dynamic(String),
    CatchAll(String),
    OptionalCatchAll(String),
}

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
        
        self.scan_directory(base_path, &mut routes, "", &Vec::new())?;
        
        tracing::info!("Scanned {} routes", routes.len());
        for route in &routes {
            tracing::debug!("Found route: {}", route);
        }
        
        Ok(routes)
    }

    fn scan_directory(
        &self,
        path: &Path,
        routes: &mut Vec<Route>,
        prefix: &str,
        segments: &[RouteSegment],
    ) -> anyhow::Result<()> {
        if !path.is_dir() {
            return Ok(());
        }

        let mut entries: Vec<_> = std::fs::read_dir(path)?.collect::<Result<_, _>>()?;
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        let mut new_segments = segments.to_vec();

        for entry in entries {
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_path.is_dir() {
                if file_name == "api" {
                    self.scan_api_directory(&file_path, routes, prefix, &new_segments)?;
                } else if file_name.starts_with('_') || file_name.starts_with('.') {
                    continue;
                } else {
                    let segment = self.parse_segment(&file_name);
                    if let Some(seg) = &segment {
                        new_segments.push(seg.clone());
                    }
                    
                    let new_prefix = if prefix.is_empty() {
                        format!("/{}", file_name)
                    } else {
                        format!("{}/{}", prefix, file_name)
                    };
                    self.scan_directory(&file_path, routes, &new_prefix, &new_segments)?;
                    
                    if segment.is_some() {
                        new_segments.pop();
                    }
                }
            } else if let Some(ext) = file_path.extension() {
                if ext == "tsx" || ext == "ts" || ext == "jsx" || ext == "js" {
                    if let Some(stem) = file_path.file_stem() {
                        let stem_str = stem.to_string_lossy();
                        
                        if stem_str == "page" {
                            let route_path = self.build_path(prefix, segments);
                            routes.push(Route {
                                path: route_path,
                                method: RouteMethod::Get,
                                file_path: file_path.to_string_lossy().to_string(),
                                handler_name: format!("{}Page", self.sanitize_name(prefix)),
                                segments: segments.to_vec(),
                            });
                        } else if stem_str == "layout" {
                        } else if stem_str == "route" {
                            self.parse_route_file(&file_path, routes, prefix, segments)?;
                        } else if stem_str == "loading" || stem_str == "error" || stem_str == "not-found" {
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn scan_api_directory(
        &self,
        path: &Path,
        routes: &mut Vec<Route>,
        prefix: &str,
        segments: &[RouteSegment],
    ) -> anyhow::Result<()> {
        let api_prefix = if prefix.is_empty() { "api".to_string() } else { format!("{}/api", prefix) };
        
        if !path.is_dir() {
            return Ok(());
        }

        let mut entries: Vec<_> = std::fs::read_dir(path)?.collect::<Result<_, _>>()?;
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        let mut new_segments = segments.to_vec();

        for entry in entries {
            let file_path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            if file_path.is_dir() {
                let segment = self.parse_segment(&file_name);
                if let Some(seg) = &segment {
                    new_segments.push(seg.clone());
                }
                
                let new_prefix = format!("{}/{}", api_prefix, file_name);
                self.scan_api_directory(&file_path, routes, &new_prefix, &new_segments)?;
                
                if segment.is_some() {
                    new_segments.pop();
                }
            } else if let Some(ext) = file_path.extension() {
                if ext == "ts" || ext == "js" {
                    if let Some(stem) = file_path.file_stem() {
                        if stem == "route" {
                            self.parse_route_file(&file_path, routes, &api_prefix, segments)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn parse_route_file(
        &self,
        path: &Path,
        routes: &mut Vec<Route>,
        prefix: &str,
        segments: &[RouteSegment],
    ) -> anyhow::Result<()> {
        let content = std::fs::read_to_string(path)?;
        let route_path = self.build_path(prefix, segments);
        
        let methods = [
            ("GET", RouteMethod::Get),
            ("POST", RouteMethod::Post),
            ("PUT", RouteMethod::Put),
            ("DELETE", RouteMethod::Delete),
            ("PATCH", RouteMethod::Patch),
            ("OPTIONS", RouteMethod::Options),
            ("HEAD", RouteMethod::Head),
        ];
        
        for (method_name, method) in methods {
            if content.contains(&format!("export async function {}", method_name)) || 
               content.contains(&format!("export function {}", method_name)) ||
               (content.contains("export") && content.contains(method_name) && 
                (content.contains("Request") || content.contains("Response"))) {
                routes.push(Route {
                    path: route_path.clone(),
                    method,
                    file_path: path.to_string_lossy().to_string(),
                    handler_name: format!("{}Handler", method_name.to_lowercase()),
                    segments: segments.to_vec(),
                });
            }
        }
        
        if routes.is_empty() && content.contains("export") {
            routes.push(Route {
                path: route_path,
                method: RouteMethod::Get,
                file_path: path.to_string_lossy().to_string(),
                handler_name: "routeHandler".to_string(),
                segments: segments.to_vec(),
            });
        }
        
        Ok(())
    }

    fn parse_segment(&self, name: &str) -> Option<RouteSegment> {
        if name.starts_with('[') && name.ends_with(']') {
            if name.starts_with("[[") && name.ends_with("]]") {
                let inner = &name[2..name.len()-2];
                Some(RouteSegment::OptionalCatchAll(inner.to_string()))
            } else if name.starts_with("...") {
                let inner = &name[3..name.len()-1];
                Some(RouteSegment::CatchAll(inner.to_string()))
            } else {
                let inner = &name[1..name.len()-1];
                Some(RouteSegment::Dynamic(inner.to_string()))
            }
        } else {
            Some(RouteSegment::Static(name.to_string()))
        }
    }

    fn build_path(&self, prefix: &str, segments: &[RouteSegment]) -> String {
        let mut path = if prefix.is_empty() { String::from("/") } else { prefix.to_string() };
        
        for segment in segments {
            match segment {
                RouteSegment::Static(s) => path = format!("{}/{}", path, s),
                RouteSegment::Dynamic(s) => path = format!("{}/{{{}}}", path, s),
                RouteSegment::CatchAll(s) => path = format!("{}/{{{}}}", path, s),
                RouteSegment::OptionalCatchAll(s) => path = format!("{}/{{{}}}", path, s),
            }
        }
        
        if path != "/" && path.ends_with('/') {
            path.pop();
        }
        
        path
    }

    fn sanitize_name(&self, path: &str) -> String {
        path.replace('/', "_")
            .replace('-', "_")
            .replace('[', "")
            .replace(']', "")
            .replace("...", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_segment() {
        let scanner = RouteScanner::new(".");
        assert!(matches!(scanner.parse_segment("[id]"), Some(RouteSegment::Dynamic(_))));
        assert!(matches!(scanner.parse_segment("[slug]"), Some(RouteSegment::Dynamic(_))));
    }

    #[test]
    fn test_catch_all_segment() {
        let scanner = RouteScanner::new(".");
        assert!(matches!(scanner.parse_segment("[...slug]"), Some(RouteSegment::CatchAll(_))));
    }

    #[test]
    fn test_optional_catch_all_segment() {
        let scanner = RouteScanner::new(".");
        assert!(matches!(scanner.parse_segment("[[...slug]]"), Some(RouteSegment::OptionalCatchAll(_))));
    }

    #[test]
    fn test_build_path() {
        let scanner = RouteScanner::new(".");
        let segments = vec![
            RouteSegment::Static("users".to_string()),
            RouteSegment::Dynamic("id".to_string()),
        ];
        assert_eq!(scanner.build_path("", &segments), "/users/{id}");
    }
}
