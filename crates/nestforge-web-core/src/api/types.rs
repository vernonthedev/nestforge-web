use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T = serde_json::Value> {
    pub data: T,
    #[serde(default)]
    pub meta: ResponseMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResponseMeta {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub total: Option<u64>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: u32,
    pub page_size: u32,
    pub total: u64,
    pub total_pages: u32,
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, page: u32, page_size: u32, total: u64) -> Self {
        let total_pages = (total as f64 / page_size as f64).ceil() as u32;
        Self {
            items,
            page,
            page_size,
            total,
            total_pages,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    pub fn new(code: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            message: message.to_string(),
            details: None,
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::new("BAD_REQUEST", message)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new("NOT_FOUND", message)
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::new("UNAUTHORIZED", message)
    }

    pub fn internal(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", message)
    }
}
