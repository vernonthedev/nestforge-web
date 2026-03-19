use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct TypedRequest<T> {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Option<T>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl<T: Serialize> TypedRequest<T> {
    pub fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_string(),
            path: path.to_string(),
            headers: HashMap::new(),
            body: None,
            params: HashMap::new(),
            query: HashMap::new(),
        }
    }

    pub fn with_body(mut self, body: T) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_query(mut self, key: &str, value: &str) -> Self {
        self.query.insert(key.to_string(), value.to_string());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedResponse<T = serde_json::Value> {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<T>,
}

impl<T: for<'de> Deserialize<'de>> TypedResponse<T> {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }
}

pub struct ApiClientBuilder {
    base_url: String,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

impl ApiClientBuilder {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            headers: HashMap::new(),
            timeout_secs: None,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    pub fn build(self) -> RuntimeApiClient {
        RuntimeApiClient {
            base_url: self.base_url,
            headers: self.headers,
            timeout_secs: self.timeout_secs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeApiClient {
    base_url: String,
    headers: HashMap<String, String>,
    timeout_secs: Option<u64>,
}

impl RuntimeApiClient {
    pub fn builder(base_url: &str) -> ApiClientBuilder {
        ApiClientBuilder::new(base_url)
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> anyhow::Result<T> {
        self.request("GET", path, None::<()>).await
    }

    pub async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(&self, path: &str, body: &B) -> anyhow::Result<T> {
        self.request("POST", path, Some(body)).await
    }

    pub async fn put<B: Serialize, T: for<'de> Deserialize<'de>>(&self, path: &str, body: &B) -> anyhow::Result<T> {
        self.request("PUT", path, Some(body)).await
    }

    pub async fn delete<T: for<'de> Deserialize<'de>>(&self, path: &str) -> anyhow::Result<T> {
        self.request("DELETE", path, None::<()>).await
    }

    pub async fn patch<B: Serialize, T: for<'de> Deserialize<'de>>(&self, path: &str, body: &B) -> anyhow::Result<T> {
        self.request("PATCH", path, Some(body)).await
    }

    async fn request<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);

        let client = reqwest::Client::new();
        let mut req = client.request(
            reqwest::Method::from_bytes(method.as_bytes())?,
            &url,
        );

        for (key, value) in &self.headers {
            req = req.header(key, value);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        if let Some(timeout) = self.timeout_secs {
            req = req.timeout(std::time::Duration::from_secs(timeout));
        }

        let response = req.send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("Request failed with {}: {}", status, text);
        }

        let json = response.json::<T>().await?;
        Ok(json)
    }
}
