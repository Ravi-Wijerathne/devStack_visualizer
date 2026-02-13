use crate::scanner::ProjectFiles;
use serde::Serialize;
use std::path::Path;

/// Detected project technology stack
#[derive(Debug, Default, Clone, Serialize)]
pub struct ProjectStack {
    pub backend: Option<String>,
    pub frontend: Option<String>,
    pub database: Option<String>,
    pub containerized: bool,
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

/// Detect the project technology stack from scanned files
pub fn detect_stack(project_files: &ProjectFiles, root: &Path) -> ProjectStack {
    let mut stack = ProjectStack::default();

    // Backend detection
    if !project_files.rust_files.is_empty() || root.join("Cargo.toml").exists() {
        stack.backend = Some("Rust".to_string());
    } else if !project_files.python_files.is_empty()
        || root.join("requirements.txt").exists()
        || root.join("pyproject.toml").exists()
    {
        stack.backend = Some("Python".to_string());
    }

    // Frontend detection
    let has_package_json = root.join("package.json").exists();
    let has_jsx_tsx = project_files
        .js_files
        .iter()
        .any(|f| matches!(f.extension().and_then(|e| e.to_str()), Some("jsx") | Some("tsx")));

    if has_jsx_tsx {
        stack.frontend = Some("React".to_string());
    } else if has_package_json && !project_files.js_files.is_empty() {
        stack.frontend = Some("Node.js / JavaScript".to_string());
    } else if !project_files.js_files.is_empty() {
        stack.frontend = Some("JavaScript / TypeScript".to_string());
    }

    // If we detected JS as frontend but no backend, and there are signs of a Node backend
    if stack.backend.is_none() && has_package_json && !project_files.js_files.is_empty() {
        stack.backend = Some("Node.js".to_string());
    }

    // Go detection
    if root.join("go.mod").exists() {
        stack.backend = Some("Go".to_string());
    }

    // Docker / containerization detection
    if !project_files.docker_files.is_empty()
        || root.join("Dockerfile").exists()
        || root.join("docker-compose.yml").exists()
        || root.join("docker-compose.yaml").exists()
    {
        stack.containerized = true;
    }

    stack
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_rust_project() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]").unwrap();

        let files = ProjectFiles {
            rust_files: vec![tmp.path().join("main.rs")],
            ..Default::default()
        };

        let stack = detect_stack(&files, tmp.path());
        assert_eq!(stack.backend.as_deref(), Some("Rust"));
    }

    #[test]
    fn test_detect_containerized() {
        let tmp = tempfile::tempdir().unwrap();

        let files = ProjectFiles {
            docker_files: vec![tmp.path().join("Dockerfile")],
            ..Default::default()
        };

        let stack = detect_stack(&files, tmp.path());
        assert!(stack.containerized);
    }
}
