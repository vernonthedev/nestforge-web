#[cfg(test)]
mod tests {
    use nestforge_web_core::routing::{Route, RouteMethod, RouteScanner, RouteSegment};

    #[test]
    fn test_route_method_as_str() {
        assert_eq!(RouteMethod::Get.as_str(), "GET");
        assert_eq!(RouteMethod::Post.as_str(), "POST");
        assert_eq!(RouteMethod::Put.as_str(), "PUT");
        assert_eq!(RouteMethod::Delete.as_str(), "DELETE");
        assert_eq!(RouteMethod::Patch.as_str(), "PATCH");
        assert_eq!(RouteMethod::Options.as_str(), "OPTIONS");
        assert_eq!(RouteMethod::Head.as_str(), "HEAD");
    }

    #[test]
    fn test_route_display() {
        let route = Route {
            path: "/users".to_string(),
            method: RouteMethod::Get,
            file_path: "src/app/users/page.tsx".to_string(),
            handler_name: "usersPage".to_string(),
            segments: vec![],
        };

        let display = format!("{}", route);
        assert!(display.contains("GET"));
        assert!(display.contains("/users"));
        assert!(display.contains("src/app/users/page.tsx"));
    }

    #[test]
    fn test_route_segment_dynamic() {
        let scanner = RouteScanner::new(".");
        let segment = scanner.parse_segment("[id]");

        assert!(matches!(segment, Some(RouteSegment::Dynamic(_))));
        if let Some(RouteSegment::Dynamic(name)) = segment {
            assert_eq!(name, "id");
        }
    }

    #[test]
    fn test_route_segment_catch_all() {
        let scanner = RouteScanner::new(".");
        let segment = scanner.parse_segment("[...slug]");

        assert!(matches!(segment, Some(RouteSegment::CatchAll(_))));
        if let Some(RouteSegment::CatchAll(name)) = segment {
            assert_eq!(name, "slug");
        }
    }

    #[test]
    fn test_route_segment_optional_catch_all() {
        let scanner = RouteScanner::new(".");
        let segment = scanner.parse_segment("[[...path]]");

        assert!(matches!(segment, Some(RouteSegment::OptionalCatchAll(_))));
        if let Some(RouteSegment::OptionalCatchAll(name)) = segment {
            assert_eq!(name, "path");
        }
    }

    #[test]
    fn test_route_segment_static() {
        let scanner = RouteScanner::new(".");
        let segment = scanner.parse_segment("users");

        assert!(matches!(segment, Some(RouteSegment::Static(_))));
        if let Some(RouteSegment::Static(name)) = segment {
            assert_eq!(name, "users");
        }
    }

    #[test]
    fn test_build_path_static() {
        let scanner = RouteScanner::new(".");
        let segments = vec![
            RouteSegment::Static("users".to_string()),
            RouteSegment::Static("123".to_string()),
        ];

        let path = scanner.build_path("", &segments);
        assert_eq!(path, "/users/123");
    }

    #[test]
    fn test_build_path_dynamic() {
        let scanner = RouteScanner::new(".");
        let segments = vec![
            RouteSegment::Static("users".to_string()),
            RouteSegment::Dynamic("id".to_string()),
        ];

        let path = scanner.build_path("", &segments);
        assert_eq!(path, "/users/{id}");
    }

    #[test]
    fn test_build_path_catch_all() {
        let scanner = RouteScanner::new(".");
        let segments = vec![
            RouteSegment::Static("docs".to_string()),
            RouteSegment::CatchAll("rest".to_string()),
        ];

        let path = scanner.build_path("", &segments);
        assert_eq!(path, "/docs/{rest}");
    }

    #[test]
    fn test_build_path_with_prefix() {
        let scanner = RouteScanner::new(".");
        let segments = vec![RouteSegment::Dynamic("id".to_string())];

        let path = scanner.build_path("/api/users", &segments);
        assert_eq!(path, "/api/users/{id}");
    }

    #[test]
    fn test_build_path_root() {
        let scanner = RouteScanner::new(".");
        let path = scanner.build_path("", &[]);
        assert_eq!(path, "/");
    }

    #[test]
    fn test_sanitize_name() {
        let scanner = RouteScanner::new(".");

        assert_eq!(scanner.sanitize_name("/users"), "users");
        assert_eq!(scanner.sanitize_name("/users/[id]"), "usersid");
        assert_eq!(scanner.sanitize_name("/api/[...slug]"), "apislug");
    }

    #[test]
    fn test_route_segment_edge_cases() {
        let scanner = RouteScanner::new(".");

        // Empty string should return None
        let segment = scanner.parse_segment("");
        assert!(segment.is_none());

        // Just brackets
        let segment = scanner.parse_segment("[]");
        assert!(segment.is_some());
    }
}
