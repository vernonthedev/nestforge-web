#![allow(clippy::unwrap_used)]

#[cfg(test)]
mod tests {
    use nestforge_web_core::api::{ApiError, ApiGenerator, FieldSpec, PaginatedResponse, TypeSpec};
    use nestforge_web_core::routing::{Route, RouteMethod};

    #[test]
    fn test_api_error_new() {
        let error = ApiError::new("TEST_ERROR", "Test message");

        assert_eq!(error.code, "TEST_ERROR");
        assert_eq!(error.message, "Test message");
        assert!(error.details.is_none());
    }

    #[test]
    fn test_api_error_with_details() {
        let error =
            ApiError::new("TEST", "msg").with_details(serde_json::json!({"field": "value"}));

        assert!(error.details.is_some());
    }

    #[test]
    fn test_api_error_convenience_methods() {
        let bad_request = ApiError::bad_request("Invalid input");
        assert_eq!(bad_request.code, "BAD_REQUEST");

        let not_found = ApiError::not_found("User not found");
        assert_eq!(not_found.code, "NOT_FOUND");

        let unauthorized = ApiError::unauthorized("Invalid token");
        assert_eq!(unauthorized.code, "UNAUTHORIZED");

        let internal = ApiError::internal("Server error");
        assert_eq!(internal.code, "INTERNAL_ERROR");
    }

    #[test]
    fn test_api_error_display() {
        let error = ApiError::new("CODE", "message");
        let display = format!("{}", error);
        assert!(display.contains("CODE"));
        assert!(display.contains("message"));
    }

    #[test]
    fn test_paginated_response_new() {
        let items = vec![1, 2, 3, 4, 5];
        let response = PaginatedResponse::new(items, 1, 2, 10);

        assert_eq!(response.items.len(), 5);
        assert_eq!(response.page, 1);
        assert_eq!(response.page_size, 2);
        assert_eq!(response.total, 10);
        assert_eq!(response.total_pages, 5);
    }

    #[test]
    fn test_paginated_response_has_next() {
        let response = PaginatedResponse::new(vec![1, 2], 1, 2, 10);
        assert!(response.has_next());

        let last_page = PaginatedResponse::new(vec![1, 2], 5, 2, 10);
        assert!(!last_page.has_next());
    }

    #[test]
    fn test_paginated_response_has_prev() {
        let response = PaginatedResponse::new(vec![1, 2], 1, 2, 10);
        assert!(!response.has_prev());

        let response = PaginatedResponse::new(vec![1, 2], 2, 2, 10);
        assert!(response.has_prev());
    }

    #[test]
    fn test_type_spec_creation() {
        let spec = TypeSpec {
            name: "User".to_string(),
            fields: vec![
                FieldSpec {
                    name: "id".to_string(),
                    type_name: "string".to_string(),
                    is_optional: false,
                    is_array: false,
                },
                FieldSpec {
                    name: "name".to_string(),
                    type_name: "string".to_string(),
                    is_optional: true,
                    is_array: false,
                },
            ],
            is_enum: false,
            enum_variants: vec![],
        };

        assert_eq!(spec.name, "User");
        assert_eq!(spec.fields.len(), 2);
        assert!(!spec.is_enum);
    }

    #[test]
    fn test_enum_type_spec() {
        let spec = TypeSpec {
            name: "Status".to_string(),
            fields: vec![],
            is_enum: true,
            enum_variants: vec![
                "Active".to_string(),
                "Inactive".to_string(),
                "Pending".to_string(),
            ],
        };

        assert!(spec.is_enum);
        assert_eq!(spec.enum_variants.len(), 3);
    }

    #[test]
    fn test_api_generator_new() {
        let routes = vec![
            Route {
                path: "/users".to_string(),
                method: RouteMethod::Get,
                file_path: "src/app/users/route.ts".to_string(),
                handler_name: "getUsers".to_string(),
                segments: vec![],
            },
            Route {
                path: "/users".to_string(),
                method: RouteMethod::Post,
                file_path: "src/app/users/route.ts".to_string(),
                handler_name: "createUser".to_string(),
                segments: vec![],
            },
        ];

        let generator = ApiGenerator::new(routes);
        let endpoints = generator.get_endpoint_specs();

        assert_eq!(endpoints.len(), 2);
    }

    #[test]
    fn test_generate_typescript_types() {
        let routes = vec![];
        let mut generator = ApiGenerator::new(routes);

        generator.add_type(TypeSpec {
            name: "User".to_string(),
            fields: vec![FieldSpec {
                name: "id".to_string(),
                type_name: "string".to_string(),
                is_optional: false,
                is_array: false,
            }],
            is_enum: false,
            enum_variants: vec![],
        });

        let output = generator.generate_typescript_types();
        assert!(output.contains("export interface User"));
        assert!(output.contains("id: string"));
    }

    #[test]
    fn test_generate_typescript_client() {
        let routes = vec![Route {
            path: "/users".to_string(),
            method: RouteMethod::Get,
            file_path: "src/app/users/route.ts".to_string(),
            handler_name: "getUsers".to_string(),
            segments: vec![],
        }];

        let generator = ApiGenerator::new(routes);
        let output = generator.generate_typescript_client();

        assert!(output.contains("class ApiClient"));
        assert!(output.contains("request<"));
    }

    #[test]
    fn test_generate_rust_client() {
        let routes = vec![Route {
            path: "/api/users".to_string(),
            method: RouteMethod::Post,
            file_path: "src/app/users/route.ts".to_string(),
            handler_name: "createUser".to_string(),
            segments: vec![],
        }];

        let generator = ApiGenerator::new(routes);
        let output = generator.generate_rust_client();

        assert!(output.contains("pub struct ApiClient"));
        assert!(output.contains("reqwest"));
    }

    #[test]
    fn test_endpoint_specs_from_routes() {
        let routes = vec![
            Route {
                path: "/posts".to_string(),
                method: RouteMethod::Get,
                file_path: "posts/route.ts".to_string(),
                handler_name: "getPosts".to_string(),
                segments: vec![],
            },
            Route {
                path: "/posts".to_string(),
                method: RouteMethod::Post,
                file_path: "posts/route.ts".to_string(),
                handler_name: "createPost".to_string(),
                segments: vec![],
            },
            Route {
                path: "/posts".to_string(),
                method: RouteMethod::Delete,
                file_path: "posts/route.ts".to_string(),
                handler_name: "deletePost".to_string(),
                segments: vec![],
            },
        ];

        let generator = ApiGenerator::new(routes);
        let endpoints = generator.get_endpoint_specs();

        assert_eq!(endpoints.len(), 3);
        assert!(endpoints.iter().all(|e| e.path == "/posts"));
    }
}
