#![allow(clippy::unwrap_used)]

#[cfg(test)]
mod tests {
    use nestforge_web_core::config::NestForgeWebConfig;

    #[test]
    fn test_config_default_values() {
        std::env::remove_var("PORT");
        std::env::remove_var("HTTP_PORT");
        std::env::remove_var("HOST");
        std::env::remove_var("APP_NAME");
        std::env::remove_var("APP_DIR");
        std::env::remove_var("BACKEND_DIR");

        let config = NestForgeWebConfig::default();

        assert_eq!(config.port, 3000);
        assert_eq!(config.http_port, 3001);
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.app_name, "nestforge-app");
        assert_eq!(config.app_dir, "src/app");
        assert_eq!(config.backend_dir, "src/backend");
    }

    #[test]
    fn test_config_from_env() {
        std::env::set_var("PORT", "8080");
        std::env::set_var("HTTP_PORT", "8081");
        std::env::set_var("HOST", "0.0.0.0");
        std::env::set_var("APP_NAME", "test-app");
        std::env::set_var("APP_DIR", "custom/app");
        std::env::set_var("BACKEND_DIR", "custom/backend");

        let config = NestForgeWebConfig::from_env();

        assert_eq!(config.port, 8080);
        assert_eq!(config.http_port, 8081);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.app_name, "test-app");
        assert_eq!(config.app_dir, "custom/app");
        assert_eq!(config.backend_dir, "custom/backend");

        // Cleanup
        std::env::remove_var("PORT");
        std::env::remove_var("HTTP_PORT");
        std::env::remove_var("HOST");
        std::env::remove_var("APP_NAME");
        std::env::remove_var("APP_DIR");
        std::env::remove_var("BACKEND_DIR");
    }

    #[test]
    fn test_config_with_port() {
        std::env::remove_var("PORT");

        let config = NestForgeWebConfig::default().with_port(9000);
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn test_config_with_host() {
        let config = NestForgeWebConfig::default().with_host("192.168.1.1");
        assert_eq!(config.host, "192.168.1.1");
    }

    #[test]
    fn test_config_clone() {
        let config = NestForgeWebConfig::default();
        let cloned = config.clone();

        assert_eq!(config.port, cloned.port);
        assert_eq!(config.http_port, cloned.http_port);
        assert_eq!(config.host, cloned.host);
        assert_eq!(config.app_name, cloned.app_name);
    }

    #[test]
    fn test_config_invalid_port_fallback() {
        std::env::set_var("PORT", "invalid");

        let config = NestForgeWebConfig::default();
        assert_eq!(config.port, 3000); // Should fallback to default

        std::env::remove_var("PORT");
    }

    #[test]
    fn test_config_invalid_http_port_fallback() {
        std::env::set_var("HTTP_PORT", "invalid");

        let config = NestForgeWebConfig::default();
        assert_eq!(config.http_port, 3001); // Should fallback to default

        std::env::remove_var("HTTP_PORT");
    }
}
