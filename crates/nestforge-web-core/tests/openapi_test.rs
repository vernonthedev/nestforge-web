#[cfg(test)]
mod tests {
    use nestforge_web_core::openapi::{
        MediaTypeObject, OpenApiGenerator, OperationObject, ParameterLocation, ParameterObject,
        ResponseObject, SchemaObject,
    };
    use nestforge_web_core::routing::{Route, RouteMethod, RouteSegment};
    use std::collections::HashMap;

    #[test]
    fn test_openapi_spec_default() {
        let spec = nestforge_web_core::openapi::spec::OpenApiSpec::default();

        assert_eq!(spec.openapi, "3.1.0");
        assert_eq!(spec.info.title, "NestForge Web API");
        assert_eq!(spec.info.version, "1.0.0");
    }

    #[test]
    fn test_openapi_generator_new() {
        let generator = OpenApiGenerator::new("Test API", "1.0.0");

        let json = generator.to_json().unwrap();
        assert!(json.contains("Test API"));
        assert!(json.contains("1.0.0"));
    }

    #[test]
    fn test_openapi_generator_with_server() {
        let generator = OpenApiGenerator::new("API", "1.0")
            .with_server("https://api.example.com", Some("Production"));

        let json = generator.to_json().unwrap();
        assert!(json.contains("https://api.example.com"));
        assert!(json.contains("Production"));
    }

    #[test]
    fn test_openapi_generator_with_description() {
        let generator = OpenApiGenerator::new("API", "1.0").with_description("A test API");

        let json = generator.to_json().unwrap();
        assert!(json.contains("A test API"));
    }

    #[test]
    fn test_openapi_generator_add_route() {
        let mut generator = OpenApiGenerator::new("API", "1.0");

        let route = Route {
            path: "/users".to_string(),
            method: RouteMethod::Get,
            file_path: "users/page.tsx".to_string(),
            handler_name: "getUsers".to_string(),
            segments: vec![],
        };

        generator.add_route(&route);

        let spec = generator.build();
        assert!(spec.paths.contains_key("/users"));
    }

    #[test]
    fn test_openapi_generator_add_routes() {
        let mut generator = OpenApiGenerator::new("API", "1.0");

        let routes = vec![
            Route {
                path: "/users".to_string(),
                method: RouteMethod::Get,
                file_path: "users/page.tsx".to_string(),
                handler_name: "getUsers".to_string(),
                segments: vec![],
            },
            Route {
                path: "/posts".to_string(),
                method: RouteMethod::Get,
                file_path: "posts/page.tsx".to_string(),
                handler_name: "getPosts".to_string(),
                segments: vec![],
            },
        ];

        generator.add_routes(&routes);

        let spec = generator.build();
        assert!(spec.paths.contains_key("/users"));
        assert!(spec.paths.contains_key("/posts"));
    }

    #[test]
    fn test_openapi_generator_dynamic_route() {
        let mut generator = OpenApiGenerator::new("API", "1.0");

        let route = Route {
            path: "/users/{id}".to_string(),
            method: RouteMethod::Get,
            file_path: "users/[id]/page.tsx".to_string(),
            handler_name: "getUser".to_string(),
            segments: vec![RouteSegment::Dynamic("id".to_string())],
        };

        generator.add_route(&route);

        let spec = generator.build();
        assert!(spec.paths.contains_key("/users/{id}"));

        if let Some(path_item) = spec.paths.get("/users/{id}") {
            if let Some(op) = &path_item.get {
                assert!(!op.parameters.is_empty());
                let param = &op.parameters[0];
                assert_eq!(param.name, "id");
                assert!(matches!(param.location, ParameterLocation::Path));
            }
        }
    }

    #[test]
    fn test_schema_object_string() {
        let schema = SchemaObject::string();

        assert_eq!(schema.schema_type, Some("string".to_string()));
    }

    #[test]
    fn test_schema_object_integer() {
        let schema = SchemaObject::integer();

        assert_eq!(schema.schema_type, Some("integer".to_string()));
        assert_eq!(schema.format, Some("int32".to_string()));
    }

    #[test]
    fn test_schema_object_boolean() {
        let schema = SchemaObject::boolean();

        assert_eq!(schema.schema_type, Some("boolean".to_string()));
    }

    #[test]
    fn test_schema_object_array() {
        let schema = SchemaObject::array(SchemaObject::string());

        assert_eq!(schema.schema_type, Some("array".to_string()));
        assert!(schema.items.is_some());
    }

    #[test]
    fn test_schema_object_object() {
        let mut props = HashMap::new();
        props.insert("id".to_string(), SchemaObject::string());
        props.insert("name".to_string(), SchemaObject::string());

        let schema = SchemaObject::object(props);

        assert_eq!(schema.schema_type, Some("object".to_string()));
        assert!(schema.properties.is_some());
        if let Some(props) = &schema.properties {
            assert!(props.contains_key("id"));
            assert!(props.contains_key("name"));
        }
    }

    #[test]
    fn test_schema_object_with_example() {
        let schema = SchemaObject::string()
            .with_example(serde_json::json!("test@example.com"))
            .with_description("User email address");

        assert!(schema.example.is_some());
        assert!(schema.description.is_some());
    }

    #[test]
    fn test_schema_object_with_format() {
        let schema = SchemaObject::string().with_format("email");

        assert_eq!(schema.format, Some("email".to_string()));
    }

    #[test]
    fn test_parameter_object() {
        let param = ParameterObject {
            name: "id".to_string(),
            location: ParameterLocation::Path,
            required: Some(true),
            description: Some("User ID".to_string()),
            schema: SchemaObject::string(),
        };

        assert_eq!(param.name, "id");
        assert!(matches!(param.location, ParameterLocation::Path));
        assert!(param.required.unwrap());
    }

    #[test]
    fn test_response_object() {
        let mut content = HashMap::new();
        content.insert(
            "application/json".to_string(),
            MediaTypeObject {
                schema: Some(SchemaObject::object(HashMap::new())),
                example: None,
            },
        );

        let response = ResponseObject {
            description: "Success".to_string(),
            content,
            headers: HashMap::new(),
        };

        assert_eq!(response.description, "Success");
        assert!(response.content.contains_key("application/json"));
    }

    #[test]
    fn test_openapi_to_json() {
        let generator = OpenApiGenerator::new("API", "1.0");
        let json = generator.to_json().unwrap();

        // Verify it's valid JSON
        let _: serde_json::Value = serde_json::from_str(&json).unwrap();
    }

    #[test]
    fn test_openapi_to_yaml() {
        let generator = OpenApiGenerator::new("API", "1.0");
        let yaml = generator.to_yaml().unwrap();

        assert!(yaml.contains("openapi:"));
        assert!(yaml.contains("3.1.0"));
    }

    #[test]
    fn test_operation_object() {
        let params = vec![ParameterObject {
            name: "id".to_string(),
            location: ParameterLocation::Path,
            required: Some(true),
            description: None,
            schema: SchemaObject::integer(),
        }];

        let operation = OperationObject {
            operation_id: Some("getUser".to_string()),
            summary: Some("Get a user".to_string()),
            description: None,
            tags: vec!["users".to_string()],
            parameters: params,
            request_body: None,
            responses: HashMap::new(),
            deprecated: false,
        };

        assert_eq!(operation.operation_id, Some("getUser".to_string()));
        assert!(operation.parameters.len() == 1);
    }

    #[test]
    fn test_security_scheme_bearer() {
        let scheme = nestforge_web_core::openapi::spec::SecuritySchemeObject::bearer();

        assert_eq!(scheme.scheme_type, "http");
        assert_eq!(scheme.scheme, Some("bearer".to_string()));
        assert_eq!(scheme.bearer_format, Some("JWT".to_string()));
    }

    #[test]
    fn test_security_scheme_api_key() {
        let scheme =
            nestforge_web_core::openapi::spec::SecuritySchemeObject::api_key("X-API-Key", "header");

        assert_eq!(scheme.scheme_type, "apiKey");
        assert_eq!(scheme.name, Some("X-API-Key".to_string()));
        assert_eq!(scheme.location, Some("header".to_string()));
    }
}
