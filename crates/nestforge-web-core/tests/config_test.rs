#![allow(clippy::unwrap_used)]

#[cfg(test)]
mod tests {
    use nestforge_web_core::config::NestForgeWebConfig;

    #[test]
    fn test_config_default_values() {
        std::env::set_var("PORT", "3000");
        std::env::set_var("HTTP_PORT", "3001");
        std::env::set_var("HOST", "127.0.0.1");
        std::env::set_var("APP_NAME", "nestforge-app");
        std::env::set_var("APP_DIR", "src/app");
        std::env::set_var("BACKEND_DIR", "src/backend");

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
    }

    #[test]
    fn test_config_with_port() {
        std::env::set_var("PORT", "3000");

        let config = NestForgeWebConfig::default().with_port(9000);
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn test_config_with_host() {
        std::env::set_var("HOST", "127.0.0.1");

        let config = NestForgeWebConfig::default().with_host("192.168.1.1");
        assert_eq!(config.host, "192.168.1.1");
    }

    #[test]
    fn test_config_clone() {
        std::env::set_var("PORT", "3000");
        std::env::set_var("HTTP_PORT", "3001");
        std::env::set_var("HOST", "127.0.0.1");

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
    }

    #[test]
    fn test_config_invalid_http_port_fallback() {
        std::env::set_var("HTTP_PORT", "invalid");

        let config = NestForgeWebConfig::default();
        assert_eq!(config.http_port, 3001); // Should fallback to default
    }
}
