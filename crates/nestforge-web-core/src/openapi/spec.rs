use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    pub openapi: String,
    pub info: InfoObject,
    pub servers: Vec<ServerObject>,
    pub paths: HashMap<String, PathItem>,
    pub components: ComponentsObject,
    #[serde(default)]
    pub tags: Vec<TagObject>,
}

impl Default for OpenApiSpec {
    fn default() -> Self {
        Self {
            openapi: "3.1.0".to_string(),
            info: InfoObject::default(),
            servers: vec![ServerObject {
                url: "http://localhost:3000".to_string(),
                description: Some("Development server".to_string()),
            }],
            paths: HashMap::new(),
            components: ComponentsObject::default(),
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoObject {
    pub title: String,
    pub description: Option<String>,
    pub version: String,
    pub contact: Option<ContactObject>,
    pub license: Option<LicenseObject>,
}

impl Default for InfoObject {
    fn default() -> Self {
        Self {
            title: "NestForge Web API".to_string(),
            description: Some("API documentation".to_string()),
            version: "1.0.0".to_string(),
            contact: None,
            license: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerObject {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactObject {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseObject {
    pub name: String,
    pub identifier: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagObject {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PathItem {
    #[serde(default)]
    pub get: Option<OperationObject>,
    #[serde(default)]
    pub post: Option<OperationObject>,
    #[serde(default)]
    pub put: Option<OperationObject>,
    #[serde(default)]
    pub patch: Option<OperationObject>,
    #[serde(default)]
    pub delete: Option<OperationObject>,
    #[serde(default)]
    pub options: Option<OperationObject>,
    #[serde(default)]
    pub head: Option<OperationObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationObject {
    pub operation_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub parameters: Vec<ParameterObject>,
    pub request_body: Option<RequestBodyObject>,
    pub responses: HashMap<String, ResponseObject>,
    #[serde(default)]
    pub deprecated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterObject {
    pub name: String,
    #[serde(rename = "in")]
    pub location: ParameterLocation,
    pub required: Option<bool>,
    pub description: Option<String>,
    pub schema: SchemaObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ParameterLocation {
    Query,
    Path,
    Header,
    Cookie,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBodyObject {
    pub description: Option<String>,
    pub required: bool,
    pub content: HashMap<String, MediaTypeObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseObject {
    pub description: String,
    #[serde(default)]
    pub content: HashMap<String, MediaTypeObject>,
    #[serde(default)]
    pub headers: HashMap<String, HeaderObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaTypeObject {
    pub schema: Option<SchemaObject>,
    #[serde(default)]
    pub example: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderObject {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub schema: SchemaObject,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaObject {
    #[serde(rename = "type")]
    pub schema_type: Option<String>,
    pub format: Option<String>,
    pub properties: Option<HashMap<String, SchemaObject>>,
    pub items: Option<Box<SchemaObject>>,
    pub required: Option<Vec<String>>,
    pub description: Option<String>,
    pub example: Option<serde_json::Value>,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub deprecated: bool,
    pub enum_values: Option<Vec<serde_json::Value>>,
    pub default: Option<serde_json::Value>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl SchemaObject {
    pub fn string() -> Self {
        Self {
            schema_type: Some("string".to_string()),
            format: None,
            properties: None,
            items: None,
            required: None,
            description: None,
            example: None,
            nullable: false,
            deprecated: false,
            enum_values: None,
            default: None,
            minimum: None,
            maximum: None,
        }
    }

    pub fn integer() -> Self {
        Self {
            schema_type: Some("integer".to_string()),
            format: Some("int32".to_string()),
            ..Default::default()
        }
    }

    pub fn number() -> Self {
        Self {
            schema_type: Some("number".to_string()),
            ..Default::default()
        }
    }

    pub fn boolean() -> Self {
        Self {
            schema_type: Some("boolean".to_string()),
            ..Default::default()
        }
    }

    pub fn array(items: SchemaObject) -> Self {
        Self {
            schema_type: Some("array".to_string()),
            items: Some(Box::new(items)),
            ..Default::default()
        }
    }

    pub fn object(properties: HashMap<String, SchemaObject>) -> Self {
        Self {
            schema_type: Some("object".to_string()),
            properties: Some(properties),
            ..Default::default()
        }
    }

    pub fn with_example(mut self, example: serde_json::Value) -> Self {
        self.example = Some(example);
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }
}

impl Default for SchemaObject {
    fn default() -> Self {
        Self {
            schema_type: None,
            format: None,
            properties: None,
            items: None,
            required: None,
            description: None,
            example: None,
            nullable: false,
            deprecated: false,
            enum_values: None,
            default: None,
            minimum: None,
            maximum: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComponentsObject {
    pub schemas: HashMap<String, SchemaObject>,
    #[serde(default)]
    pub security_schemes: HashMap<String, SecuritySchemeObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySchemeObject {
    #[serde(rename = "type")]
    pub scheme_type: String,
    pub description: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "in")]
    pub location: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
}

impl SecuritySchemeObject {
    pub fn bearer() -> Self {
        Self {
            scheme_type: "http".to_string(),
            description: Some("Bearer token authentication".to_string()),
            name: None,
            location: None,
            scheme: Some("bearer".to_string()),
            bearer_format: Some("JWT".to_string()),
        }
    }

    pub fn api_key(name: &str, location: &str) -> Self {
        Self {
            scheme_type: "apiKey".to_string(),
            description: Some("API key authentication".to_string()),
            name: Some(name.to_string()),
            location: Some(location.to_string()),
            scheme: None,
            bearer_format: None,
        }
    }
}
