use serde::{Deserialize, Serialize};

use super::scanner::RouteSegment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub path: String,
    pub method: RouteMethod,
    pub file_path: String,
    pub handler_name: String,
    #[serde(default)]
    pub segments: Vec<RouteSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RouteMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

impl RouteMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            RouteMethod::Get => "GET",
            RouteMethod::Post => "POST",
            RouteMethod::Put => "PUT",
            RouteMethod::Delete => "DELETE",
            RouteMethod::Patch => "PATCH",
            RouteMethod::Options => "OPTIONS",
            RouteMethod::Head => "HEAD",
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} -> {}",
            self.method.as_str(),
            self.path,
            self.file_path
        )
    }
}
