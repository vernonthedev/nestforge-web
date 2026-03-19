#![allow(clippy::unwrap_used)]

#[cfg(test)]
mod tests {
    use nfw_core::hmr::{
        reload::{HotReloader, ModuleReloader, ModuleState},
        watcher::{ChangedFile, FileChangeKind, FileWatcher, HmrMessage, HmrServer},
    };
    use nfw_core::routing::{Route, RouteMethod};
    use std::path::PathBuf;

    #[test]
    fn test_module_reloader_new() {
        let reloader = ModuleReloader::new();
        let state = reloader.get_state("test_module");
        assert!(state.is_none());
    }

    #[test]
    fn test_module_reloader_mark_loading() {
        let mut reloader = ModuleReloader::new();
        reloader.mark_loading("test_module");

        let state = reloader.get_state("test_module");
        assert!(matches!(state, Some(ModuleState::Reloading)));
    }

    #[test]
    fn test_module_reloader_mark_loaded() {
        let mut reloader = ModuleReloader::new();
        reloader.mark_loaded("test_module");

        let state = reloader.get_state("test_module");
        assert!(matches!(state, Some(ModuleState::Loaded)));
    }

    #[test]
    fn test_module_reloader_mark_failed() {
        let mut reloader = ModuleReloader::new();
        reloader.mark_failed("test_module", "Error message".to_string());

        let state = reloader.get_state("test_module");
        assert!(matches!(state, Some(ModuleState::Failed(_))));

        if let Some(ModuleState::Failed(msg)) = state {
            assert_eq!(msg, "Error message");
        }
    }

    #[test]
    fn test_hot_reloader_new() {
        let routes = vec![];
        let _reloader = HotReloader::new(routes);
    }

    #[test]
    fn test_hot_reloader_get_dependencies() {
        let routes = vec![Route {
            path: "/users".to_string(),
            method: RouteMethod::Get,
            file_path: "src/app/users/page.tsx".to_string(),
            handler_name: "usersPage".to_string(),
            segments: vec![],
        }];

        let reloader = HotReloader::new(routes);
        let deps = reloader.get_module_dependencies("users");

        assert!(!deps.is_empty());
        assert!(deps[0].contains("users"));
    }

    #[test]
    fn test_hot_reloader_generate_vite_config() {
        let routes = vec![];
        let reloader = HotReloader::new(routes);
        let config = reloader.generate_vite_config();

        assert!(config.contains("vite"));
        assert!(config.contains("hmrPort"));
        assert!(config.contains("hotReload"));
    }

    #[test]
    fn test_changed_file_creation() {
        let file = ChangedFile {
            path: PathBuf::from("src/app/page.tsx"),
            kind: FileChangeKind::Created,
        };

        assert_eq!(file.kind, FileChangeKind::Created);
    }

    #[test]
    fn test_file_change_kind_variants() {
        assert_eq!(FileChangeKind::Created, FileChangeKind::Created);
        assert_eq!(FileChangeKind::Modified, FileChangeKind::Modified);
        assert_eq!(FileChangeKind::Deleted, FileChangeKind::Deleted);
        assert_eq!(FileChangeKind::Renamed, FileChangeKind::Renamed);
    }

    #[test]
    fn test_hmr_message_variants() {
        let msg = HmrMessage::FullReload {
            reason: "Config change".to_string(),
        };
        assert!(matches!(msg, HmrMessage::FullReload { .. }));

        let msg = HmrMessage::Error {
            message: "Test error".to_string(),
        };
        assert!(matches!(msg, HmrMessage::Error { .. }));

        let msg = HmrMessage::Connected {
            client_id: "123".to_string(),
        };
        assert!(matches!(msg, HmrMessage::Connected { .. }));

        let files = vec![ChangedFile {
            path: PathBuf::from("test.ts"),
            kind: FileChangeKind::Modified,
        }];
        let msg = HmrMessage::Reload { files };
        assert!(matches!(msg, HmrMessage::Reload { .. }));

        let msg = HmrMessage::ModuleUpdate {
            module: "test".to_string(),
            code: "code".to_string(),
        };
        assert!(matches!(msg, HmrMessage::ModuleUpdate { .. }));
    }

    #[test]
    fn test_hmr_message_clone() {
        let msg1 = HmrMessage::FullReload {
            reason: "test".to_string(),
        };
        let msg2 = msg1.clone();

        assert!(matches!(msg2, HmrMessage::FullReload { .. }));
    }

    #[test]
    fn test_hmr_message_debug() {
        let msg = HmrMessage::Error {
            message: "error".to_string(),
        };

        let debug_str = format!("{:?}", msg);
        assert!(debug_str.contains("Error"));
    }

    #[test]
    fn test_file_watcher_creation() {
        let watcher = FileWatcher::new(100);
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_hmr_server_creation() {
        let server = HmrServer::new(3001);
        assert!(server.is_ok());
        assert_eq!(server.unwrap().port, 3001);
    }

    #[test]
    fn test_module_state_default() {
        let state = ModuleState::Loaded;
        assert!(matches!(state, ModuleState::Loaded));
    }
}
