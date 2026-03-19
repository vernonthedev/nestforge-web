use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageProps {
    pub params: HashMap<String, String>,
    pub search_params: HashMap<String, Vec<String>>,
}

impl Default for PageProps {
    fn default() -> Self {
        Self {
            params: HashMap::new(),
            search_params: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub layout: Option<String>,
}

impl Default for PageMetadata {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            layout: None,
        }
    }
}
