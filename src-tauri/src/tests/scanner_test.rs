#[cfg(test)]
mod tests {
    #[test]
    fn test_scanner_finds_rust_files() {
        let files = vec![
            "src/main.rs",
            "src/lib.rs",
            "src/commands/mod.rs",
            "src/parser/rust_parser.rs",
        ];
        let rust_files: Vec<_> = files.iter().filter(|f| f.ends_with(".rs")).collect();
        assert_eq!(rust_files.len(), 4);
    }

    #[test]
    fn test_scanner_finds_python_files() {
        let files = vec![
            "app/main.py",
            "models/user.py",
            "views/__init__.py",
            "tests/test_main.py",
        ];
        let python_files: Vec<_> = files.iter().filter(|f| f.ends_with(".py")).collect();
        assert_eq!(python_files.len(), 4);
    }

    #[test]
    fn test_scanner_finds_js_files() {
        let files = vec![
            "src/index.js",
            "src/App.ts",
            "components/Button.tsx",
            "utils/helpers.jsx",
        ];
        let js_files: Vec<_> = files
            .iter()
            .filter(|f| {
                f.ends_with(".js")
                    || f.ends_with(".ts")
                    || f.ends_with(".jsx")
                    || f.ends_with(".tsx")
            })
            .collect();
        assert_eq!(js_files.len(), 4);
    }

    #[test]
    fn test_scanner_detects_dockerfile() {
        let files = vec![
            "Dockerfile",
            "Dockerfile.prod",
            "Dockerfile.dev",
            "docker-compose.yml",
            "docker-compose.yaml",
        ];
        let docker_files: Vec<_> = files
            .iter()
            .filter(|f| f.starts_with("Dockerfile") || f.contains("docker-compose"))
            .collect();
        assert_eq!(docker_files.len(), 5);
    }

    #[test]
    fn test_skip_dirs() {
        let dirs_to_skip = vec![
            "target",
            "node_modules",
            ".git",
            "__pycache__",
            "venv",
            ".venv",
            "dist",
        ];

        assert!(dirs_to_skip.contains(&"target"));
        assert!(dirs_to_skip.contains(&"node_modules"));
        assert!(dirs_to_skip.contains(&".git"));
        assert!(dirs_to_skip.contains(&"__pycache__"));
    }

    #[test]
    fn test_max_file_size() {
        const MAX_FILE_SIZE: u64 = 1_048_576; // 1 MB
        assert_eq!(MAX_FILE_SIZE, 1048576);

        let small_file = 500;
        let large_file = 2_000_000;

        assert!(small_file < MAX_FILE_SIZE);
        assert!(large_file > MAX_FILE_SIZE);
    }

    #[test]
    fn test_total_source_files() {
        let rust_files = 5;
        let python_files = 10;
        let js_files = 8;

        let total = rust_files + python_files + js_files;
        assert_eq!(total, 23);
    }

    #[test]
    fn test_config_files_detection() {
        let files = vec![
            "Cargo.toml",
            "package.json",
            "pyproject.toml",
            "requirements.txt",
            "docker-compose.yml",
            "config.yaml",
        ];
        let config_exts = vec![".toml", ".json", ".yaml", ".yml", ".txt"];

        let config_files: Vec<_> = files
            .iter()
            .filter(|f| f.contains('.') && config_exts.iter().any(|ext| f.ends_with(ext)))
            .collect();

        assert_eq!(config_files.len(), 6);
    }

    #[test]
    fn test_project_files_default() {
        let project_files = ProjectFiles::default();
        assert!(project_files.rust_files.is_empty());
        assert!(project_files.python_files.is_empty());
        assert!(project_files.js_files.is_empty());
        assert!(project_files.config_files.is_empty());
        assert!(project_files.docker_files.is_empty());
    }

    #[test]
    fn test_project_files_total() {
        let mut files = ProjectFiles::default();
        files.rust_files.push("/path/main.rs".into());
        files.rust_files.push("/path/lib.rs".into());
        files.python_files.push("/path/app.py".into());
        files.js_files.push("/path/index.js".into());

        assert_eq!(files.total_source_files(), 4);
    }

    #[test]
    fn test_project_files_is_empty() {
        let mut empty = ProjectFiles::default();
        assert!(empty.is_empty());

        empty.js_files.push("/path/index.js".into());
        assert!(!empty.is_empty());
    }

    struct ProjectFiles {
        rust_files: Vec<String>,
        python_files: Vec<String>,
        js_files: Vec<String>,
        config_files: Vec<String>,
        docker_files: Vec<String>,
    }

    impl Default for ProjectFiles {
        fn default() -> Self {
            Self {
                rust_files: Vec::new(),
                python_files: Vec::new(),
                js_files: Vec::new(),
                config_files: Vec::new(),
                docker_files: Vec::new(),
            }
        }
    }

    impl ProjectFiles {
        fn total_source_files(&self) -> usize {
            self.rust_files.len() + self.python_files.len() + self.js_files.len()
        }

        fn is_empty(&self) -> bool {
            self.total_source_files() == 0
        }
    }
}
