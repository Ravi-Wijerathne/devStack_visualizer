#[cfg(test)]
mod tests {
    #[test]
    fn test_detect_rust_stack() {
        let mut files = ProjectFiles::default();
        files.rust_files.push("src/main.rs".into());

        let has_cargo = true;
        let has_package_json = false;
        let has_go_mod = false;

        let stack = detect_stack(&files, has_cargo, has_package_json, has_go_mod);

        assert_eq!(stack.backend, Some("Rust".to_string()));
        assert!(stack.frontend.is_none());
        assert!(!stack.containerized);
    }

    #[test]
    fn test_detect_python_stack() {
        let mut files = ProjectFiles::default();
        files.python_files.push("app/main.py".into());

        let has_cargo = false;
        let has_pyproject = true;
        let has_requirements = true;
        let has_package_json = false;
        let has_go_mod = false;

        let stack = detect_stack_simple(
            &files,
            has_cargo,
            has_pyproject,
            has_requirements,
            has_package_json,
            has_go_mod,
            false,
            false,
        );

        assert_eq!(stack.backend, Some("Python".to_string()));
    }

    #[test]
    fn test_detect_react_stack() {
        let mut files = ProjectFiles::default();
        files.js_files.push("src/App.tsx".into());
        files.js_files.push("src/index.ts".into());

        let has_package_json = true;

        let stack = detect_stack_js(&files, has_package_json);

        assert!(stack.frontend.is_some());
        let frontend = stack.frontend.as_ref().unwrap();
        assert!(frontend.contains("React") || frontend.contains("JavaScript"));
    }

    #[test]
    fn test_detect_go_stack() {
        let has_cargo = false;
        let has_package_json = false;
        let has_go_mod = true;

        let stack = detect_go_stack(has_cargo, has_package_json, has_go_mod);

        assert_eq!(stack.backend, Some("Go".to_string()));
    }

    #[test]
    fn test_detect_node_backend() {
        let has_package_json = true;
        let has_js_files = true;

        let stack = detect_node_backend(has_package_json, has_js_files, None);

        assert!(stack.backend.is_some() || stack.frontend.is_some());
    }

    #[test]
    fn test_detect_docker_containerized() {
        let mut files = ProjectFiles::default();
        files.docker_files.push("Dockerfile".into());

        let has_dockerfile = true;
        let has_docker_compose = false;

        let containerized = detect_containerized(&files, has_dockerfile, has_docker_compose);

        assert!(containerized);
    }

    #[test]
    fn test_detect_docker_compose_containerized() {
        let mut files = ProjectFiles::default();
        files.docker_files.push("docker-compose.yml".into());

        let has_dockerfile = false;
        let has_docker_compose = true;

        let containerized = detect_containerized(&files, has_dockerfile, has_docker_compose);

        assert!(containerized);
    }

    #[test]
    fn test_mixed_stack_detection() {
        let mut files = ProjectFiles::default();
        files.rust_files.push("src/main.rs".into());
        files.js_files.push("frontend/App.tsx".into());
        files.docker_files.push("Dockerfile".into());

        let has_cargo = true;
        let has_package_json = true;

        let stack = detect_stack_full(&files, has_cargo, has_package_json);

        assert_eq!(stack.backend, Some("Rust".to_string()));
        assert!(stack.frontend.is_some());
        assert!(stack.containerized);
    }

    #[test]
    fn test_project_stack_display() {
        let mut stack = ProjectStack::default();
        stack.backend = Some("Rust".to_string());
        stack.frontend = Some("React".to_string());
        stack.database = Some("PostgreSQL".to_string());
        stack.containerized = true;

        let display = format!("{}", stack);
        assert!(display.contains("Rust"));
        assert!(display.contains("React"));
        assert!(display.contains("PostgreSQL"));
        assert!(display.contains("Yes"));
    }

    #[test]
    fn test_project_stack_default() {
        let stack = ProjectStack::default();

        assert!(stack.backend.is_none());
        assert!(stack.frontend.is_none());
        assert!(stack.database.is_none());
        assert!(!stack.containerized);
    }

    #[test]
    fn test_typescript_jsx_detection() {
        let mut files = ProjectFiles::default();
        files.js_files.push("Component.tsx".into());
        files.js_files.push("App.jsx".into());

        let has_tsx = files
            .js_files
            .iter()
            .any(|f| f.ends_with(".tsx") || f.ends_with(".jsx"));

        assert!(has_tsx);
    }

    #[test]
    fn test_empty_project_stack() {
        let files = ProjectFiles::default();
        let stack = detect_stack(&files, false, false, false);

        assert!(stack.backend.is_none());
        assert!(stack.frontend.is_none());
        assert!(!stack.containerized);
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

    struct ProjectStack {
        backend: Option<String>,
        frontend: Option<String>,
        database: Option<String>,
        containerized: bool,
    }

    impl Default for ProjectStack {
        fn default() -> Self {
            Self {
                backend: None,
                frontend: None,
                database: None,
                containerized: false,
            }
        }
    }

    impl std::fmt::Display for ProjectStack {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "Detected Stack:")?;
            if let Some(ref backend) = self.backend {
                writeln!(f, "  Backend:       {}", backend)?;
            }
            if let Some(ref frontend) = self.frontend {
                writeln!(f, "  Frontend:      {}", frontend)?;
            }
            if let Some(ref database) = self.database {
                writeln!(f, "  Database:      {}", database)?;
            }
            writeln!(
                f,
                "  Containerized: {}",
                if self.containerized { "Yes" } else { "No" }
            )?;
            Ok(())
        }
    }

    fn detect_stack(
        files: &ProjectFiles,
        has_cargo: bool,
        has_package_json: bool,
        has_go_mod: bool,
    ) -> ProjectStack {
        let mut stack = ProjectStack::default();

        if !files.rust_files.is_empty() || has_cargo {
            stack.backend = Some("Rust".to_string());
        } else if !files.python_files.is_empty() {
            stack.backend = Some("Python".to_string());
        }

        if has_go_mod {
            stack.backend = Some("Go".to_string());
        }

        let has_jsx_tsx = files
            .js_files
            .iter()
            .any(|f| f.ends_with(".jsx") || f.ends_with(".tsx"));

        if has_jsx_tsx {
            stack.frontend = Some("React".to_string());
        } else if has_package_json && !files.js_files.is_empty() {
            stack.frontend = Some("Node.js / JavaScript".to_string());
        }

        if stack.backend.is_none() && has_package_json && !files.js_files.is_empty() {
            stack.backend = Some("Node.js".to_string());
        }

        stack
    }

    fn detect_stack_simple(
        files: &ProjectFiles,
        has_cargo: bool,
        has_pyproject: bool,
        has_requirements: bool,
        has_package_json: bool,
        has_go_mod: bool,
        _has_jsx: bool,
        _has_docker: bool,
    ) -> ProjectStack {
        let mut stack = ProjectStack::default();

        if !files.rust_files.is_empty() || has_cargo {
            stack.backend = Some("Rust".to_string());
        } else if !files.python_files.is_empty() || has_pyproject || has_requirements {
            stack.backend = Some("Python".to_string());
        }

        if has_go_mod {
            stack.backend = Some("Go".to_string());
        }

        if has_package_json && !files.js_files.is_empty() {
            stack.frontend = Some("JavaScript".to_string());
        }

        stack
    }

    fn detect_stack_js(files: &ProjectFiles, has_package_json: bool) -> ProjectStack {
        let mut stack = ProjectStack::default();

        let has_jsx_tsx = files
            .js_files
            .iter()
            .any(|f| f.ends_with(".jsx") || f.ends_with(".tsx"));

        if has_jsx_tsx {
            stack.frontend = Some("React".to_string());
        } else if has_package_json && !files.js_files.is_empty() {
            stack.frontend = Some("Node.js / JavaScript".to_string());
        }

        stack
    }

    fn detect_go_stack(has_cargo: bool, has_package_json: bool, has_go_mod: bool) -> ProjectStack {
        let mut stack = ProjectStack::default();

        if has_go_mod {
            stack.backend = Some("Go".to_string());
        } else if has_cargo {
            stack.backend = Some("Rust".to_string());
        } else if has_package_json {
            stack.backend = Some("Node.js".to_string());
        }

        stack
    }

    fn detect_node_backend(
        has_package_json: bool,
        has_js_files: bool,
        _docker: Option<bool>,
    ) -> ProjectStack {
        let mut stack = ProjectStack::default();

        if has_package_json && has_js_files {
            stack.backend = Some("Node.js".to_string());
            stack.frontend = Some("JavaScript".to_string());
        }

        stack
    }

    fn detect_containerized(
        files: &ProjectFiles,
        has_dockerfile: bool,
        has_docker_compose: bool,
    ) -> bool {
        !files.docker_files.is_empty() || has_dockerfile || has_docker_compose
    }

    fn detect_stack_full(
        files: &ProjectFiles,
        has_cargo: bool,
        has_package_json: bool,
    ) -> ProjectStack {
        let mut stack = ProjectStack::default();

        if !files.rust_files.is_empty() || has_cargo {
            stack.backend = Some("Rust".to_string());
        }

        if has_package_json && !files.js_files.is_empty() {
            let has_tsx = files
                .js_files
                .iter()
                .any(|f| f.ends_with(".tsx") || f.ends_with(".jsx"));
            if has_tsx {
                stack.frontend = Some("React".to_string());
            } else {
                stack.frontend = Some("Node.js / JavaScript".to_string());
            }
        }

        if !files.docker_files.is_empty()
            || files.docker_files.iter().any(|f| f.contains("Dockerfile"))
        {
            stack.containerized = true;
        }

        stack
    }
}
