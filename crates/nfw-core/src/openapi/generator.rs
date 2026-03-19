use std::collections::HashMap;
use std::path::Path;

use crate::openapi::spec::{
    MediaTypeObject, OpenApiSpec, OperationObject, ParameterLocation, ParameterObject,
    RequestBodyObject, ResponseObject, SchemaObject, ServerObject, TagObject,
};
use crate::routing::Route;

pub struct OpenApiGenerator {
    spec: OpenApiSpec,
}

impl OpenApiGenerator {
    pub fn new(title: &str, version: &str) -> Self {
        Self {
            spec: OpenApiSpec {
                info: crate::openapi::spec::InfoObject {
                    title: title.to_string(),
                    description: None,
                    version: version.to_string(),
                    contact: None,
                    license: None,
                },
                ..Default::default()
            },
        }
    }

    pub fn with_server(mut self, url: &str, description: Option<&str>) -> Self {
        self.spec.servers.push(ServerObject {
            url: url.to_string(),
            description: description.map(|s| s.to_string()),
        });
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.spec.info.description = Some(description.to_string());
        self
    }

    pub fn with_contact(mut self, name: &str, email: &str, url: Option<&str>) -> Self {
        self.spec.info.contact = Some(crate::openapi::spec::ContactObject {
            name: Some(name.to_string()),
            email: Some(email.to_string()),
            url: url.map(|s| s.to_string()),
        });
        self
    }

    pub fn add_tag(&mut self, name: &str, description: Option<&str>) {
        self.spec.tags.push(TagObject {
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
        });
    }

    pub fn add_schema(&mut self, name: &str, schema: SchemaObject) {
        self.spec
            .components
            .schemas
            .insert(name.to_string(), schema);
    }

    pub fn add_route(&mut self, route: &Route) {
        let path = route.path.clone();
        let operation = self.create_operation(route);

        let path_item = self.spec.paths.entry(path).or_default();

        match route.method.as_str() {
            "GET" => path_item.get = Some(operation),
            "POST" => path_item.post = Some(operation),
            "PUT" => path_item.put = Some(operation),
            "PATCH" => path_item.patch = Some(operation),
            "DELETE" => path_item.delete = Some(operation),
            "OPTIONS" => path_item.options = Some(operation),
            "HEAD" => path_item.head = Some(operation),
            _ => {}
        }
    }

    pub fn add_routes(&mut self, routes: &[Route]) {
        for route in routes {
            self.add_route(route);
        }
    }

    fn create_operation(&self, route: &Route) -> OperationObject {
        let mut params = Vec::new();

        for segment in &route.segments {
            match segment {
                crate::routing::RouteSegment::Dynamic(name) => {
                    params.push(ParameterObject {
                        name: name.clone(),
                        location: ParameterLocation::Path,
                        required: Some(true),
                        description: Some(format!("Dynamic parameter: {}", name)),
                        schema: SchemaObject::string(),
                    });
                }
                crate::routing::RouteSegment::CatchAll(name) => {
                    params.push(ParameterObject {
                        name: name.clone(),
                        location: ParameterLocation::Path,
                        required: Some(true),
                        description: Some(format!("Catch-all parameter: {}", name)),
                        schema: SchemaObject::string(),
                    });
                }
                crate::routing::RouteSegment::OptionalCatchAll(name) => {
                    params.push(ParameterObject {
                        name: name.clone(),
                        location: ParameterLocation::Path,
                        required: Some(false),
                        description: Some(format!("Optional catch-all: {}", name)),
                        schema: SchemaObject::string(),
                    });
                }
                crate::routing::RouteSegment::Static(_) => {}
            }
        }

        OperationObject {
            operation_id: Some(route.handler_name.clone()),
            summary: Some(format!("{} {}", route.method.as_str(), route.path)),
            description: None,
            tags: vec![self.extract_tag(&route.path)],
            parameters: params,
            request_body: None,
            responses: self.default_responses(),
            deprecated: false,
        }
    }

    fn extract_tag(&self, path: &str) -> String {
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        segments
            .first()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "default".to_string())
    }

    fn default_responses(&self) -> HashMap<String, ResponseObject> {
        let mut responses = HashMap::new();

        responses.insert(
            "200".to_string(),
            ResponseObject {
                description: "Successful response".to_string(),
                content: {
                    let mut content = HashMap::new();
                    content.insert(
                        "application/json".to_string(),
                        MediaTypeObject {
                            schema: Some(SchemaObject::object(HashMap::new())),
                            example: None,
                        },
                    );
                    content
                },
                headers: HashMap::new(),
            },
        );

        responses.insert(
            "400".to_string(),
            ResponseObject {
                description: "Bad request".to_string(),
                content: HashMap::new(),
                headers: HashMap::new(),
            },
        );

        responses.insert(
            "404".to_string(),
            ResponseObject {
                description: "Not found".to_string(),
                content: HashMap::new(),
                headers: HashMap::new(),
            },
        );

        responses
    }

    pub fn add_request_body(&mut self, path: &str, method: &str, schema_name: &str) {
        if let Some(path_item) = self.spec.paths.get_mut(path) {
            let operation = match method.to_uppercase().as_str() {
                "GET" => &mut path_item.get,
                "POST" => &mut path_item.post,
                "PUT" => &mut path_item.put,
                "PATCH" => &mut path_item.patch,
                "DELETE" => &mut path_item.delete,
                _ => return,
            };

            if let Some(op) = operation {
                let mut content = HashMap::new();
                content.insert(
                    "application/json".to_string(),
                    MediaTypeObject {
                        schema: Some(SchemaObject::object(
                            self.spec
                                .components
                                .schemas
                                .get(schema_name)
                                .cloned()
                                .unwrap_or_default()
                                .properties
                                .unwrap_or_default(),
                        )),
                        example: None,
                    },
                );

                op.request_body = Some(RequestBodyObject {
                    description: Some(format!("Request body for {}", schema_name)),
                    required: true,
                    content,
                });
            }
        }
    }

    pub fn add_response(
        &mut self,
        path: &str,
        method: &str,
        status: &str,
        schema_name: &str,
        description: &str,
    ) {
        if let Some(path_item) = self.spec.paths.get_mut(path) {
            let operation = match method.to_uppercase().as_str() {
                "GET" => &mut path_item.get,
                "POST" => &mut path_item.post,
                "PUT" => &mut path_item.put,
                "PATCH" => &mut path_item.patch,
                "DELETE" => &mut path_item.delete,
                _ => return,
            };

            if let Some(op) = operation {
                let schema = self.spec.components.schemas.get(schema_name).cloned();

                let mut content = HashMap::new();
                if let Some(ref s) = schema {
                    content.insert(
                        "application/json".to_string(),
                        MediaTypeObject {
                            schema: Some(s.clone()),
                            example: None,
                        },
                    );
                }

                op.responses.insert(
                    status.to_string(),
                    ResponseObject {
                        description: description.to_string(),
                        content,
                        headers: HashMap::new(),
                    },
                );
            }
        }
    }

    pub fn build(self) -> OpenApiSpec {
        self.spec
    }

    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string_pretty(&self.spec)
            .map_err(|e| anyhow::anyhow!("Failed to serialize OpenAPI spec: {}", e))
    }

    pub fn to_yaml(&self) -> anyhow::Result<String> {
        serde_yaml::to_string(&self.spec)
            .map_err(|e| anyhow::anyhow!("Failed to serialize OpenAPI spec to YAML: {}", e))
    }

    pub fn write_json(&self, output_path: &Path) -> anyhow::Result<()> {
        let json = self.to_json()?;
        std::fs::write(output_path, json)?;
        tracing::info!("Written OpenAPI spec to {}", output_path.display());
        Ok(())
    }

    pub fn write_yaml(&self, output_path: &Path) -> anyhow::Result<()> {
        let yaml = self.to_yaml()?;
        std::fs::write(output_path, yaml)?;
        tracing::info!("Written OpenAPI spec to {}", output_path.display());
        Ok(())
    }
}

pub struct RouteToOpenApiConverter {
    generator: OpenApiGenerator,
}

impl RouteToOpenApiConverter {
    pub fn from_routes(title: &str, version: &str, routes: &[Route]) -> Self {
        let mut generator = OpenApiGenerator::new(title, version);

        for route in routes {
            generator.add_route(route);
        }

        Self { generator }
    }

    pub fn convert(mut self) -> OpenApiSpec {
        for (name, schema) in self.infer_schemas() {
            self.generator.add_schema(&name, schema);
        }

        self.generator.build()
    }

    fn infer_schemas(&self) -> Vec<(String, SchemaObject)> {
        vec![(
            "Error".to_string(),
            SchemaObject::object(
                vec![
                    ("code".to_string(), SchemaObject::string()),
                    ("message".to_string(), SchemaObject::string()),
                ]
                .into_iter()
                .collect(),
            ),
        )]
    }
}
