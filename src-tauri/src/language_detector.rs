use crate::scanner::ProjectFiles;
use serde::Serialize;
use std::fs;
use std::path::Path;

const UTILITY_FILE_THRESHOLD: usize = 5;

#[derive(Debug, Clone, Serialize)]
pub struct SecondaryLanguage {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ProjectStack {
    pub backend: Option<String>,
    pub frontend: Option<String>,
    pub database: Option<String>,
    pub containerized: bool,
    #[serde(default)]
    pub secondary_languages: Vec<SecondaryLanguage>,
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
        if !self.secondary_languages.is_empty() {
            let utils: Vec<&str> = self
                .secondary_languages
                .iter()
                .map(|s| s.name.as_str())
                .collect();
            writeln!(f, "  Utilities:     {}", utils.join(", "))?;
        }
        writeln!(
            f,
            "  Containerized: {}",
            if self.containerized { "Yes" } else { "No" }
        )?;
        Ok(())
    }
}

fn has_next_config(root: &Path) -> bool {
    root.join("next.config.js").exists()
        || root.join("next.config.mjs").exists()
        || root.join("next.config.ts").exists()
}

fn has_next_pages_dir(root: &Path) -> bool {
    root.join("pages").exists()
}

fn has_next_app_dir(root: &Path) -> bool {
    root.join("app").exists()
}

fn has_next_in_package_json(root: &Path) -> bool {
    let package_json_path = root.join("package.json");
    if let Ok(content) = fs::read_to_string(&package_json_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = json.get("dependencies").or(json.get("devDependencies")) {
                if let Some(obj) = deps.as_object() {
                    return obj.contains_key("next");
                }
            }
        }
    }
    false
}

pub fn is_nextjs_project(root: &Path) -> bool {
    has_next_config(root)
        || has_next_pages_dir(root)
        || has_next_app_dir(root)
        || has_next_in_package_json(root)
}

pub fn has_api_routes(root: &Path) -> bool {
    root.join("app/api").exists()
        || root.join("pages/api").exists()
        || root.join("app").join("api").exists()
        || root.join("pages").join("api").exists()
}

/// Detect the project technology stack from scanned files
pub fn detect_stack(project_files: &ProjectFiles, root: &Path) -> ProjectStack {
    let mut stack = ProjectStack::default();

    let is_nextjs = is_nextjs_project(root);
    let has_nextjs_api = has_api_routes(root);

    let has_package_json = root.join("package.json").exists();
    let has_jsx_tsx = project_files.js_files.iter().any(|f| {
        matches!(
            f.extension().and_then(|e| e.to_str()),
            Some("jsx") | Some("tsx")
        )
    });

    let python_count = project_files.python_files.len();
    let has_python_backend = python_count >= UTILITY_FILE_THRESHOLD
        || root.join("requirements.txt").exists()
        || root.join("pyproject.toml").exists();

    if is_nextjs {
        if has_nextjs_api {
            stack.frontend = Some("Next.js".to_string());
            stack.backend = Some("Next.js (API Routes)".to_string());
        } else {
            stack.frontend = Some("Next.js".to_string());
            if stack.backend.is_none() && has_package_json {
                stack.backend = Some("Node.js".to_string());
            }
        }
    } else if has_jsx_tsx {
        stack.frontend = Some("React".to_string());
    }

    if stack.frontend.is_none() {
        if has_package_json && !project_files.js_files.is_empty() {
            stack.frontend = Some("Node.js / JavaScript".to_string());
        } else if !project_files.js_files.is_empty() {
            stack.frontend = Some("JavaScript / TypeScript".to_string());
        }
    }

    if stack.backend.is_none() {
        if !project_files.rust_files.is_empty() || root.join("Cargo.toml").exists() {
            stack.backend = Some("Rust".to_string());
        } else if has_python_backend {
            stack.backend = Some("Python".to_string());
        } else if has_package_json && !project_files.js_files.is_empty() {
            stack.backend = Some("Node.js".to_string());
        }
    }

    if stack.backend.is_none() && root.join("go.mod").exists() {
        stack.backend = Some("Go".to_string());
    }

    if has_python_backend && stack.backend.as_deref() != Some("Python") && is_nextjs {
        stack.secondary_languages.push(SecondaryLanguage {
            name: "Python".to_string(),
            description: "utility scripts".to_string(),
        });
    }

    if !project_files.rust_files.is_empty()
        && stack.backend.as_deref() != Some("Rust")
        && project_files.rust_files.len() < UTILITY_FILE_THRESHOLD
        && !root.join("Cargo.toml").exists()
    {
        stack.secondary_languages.push(SecondaryLanguage {
            name: "Rust".to_string(),
            description: "utility components".to_string(),
        });
    }

    if !project_files.docker_files.is_empty()
        || root.join("Dockerfile").exists()
        || root.join("docker-compose.yml").exists()
        || root.join("docker-compose.yaml").exists()
    {
        stack.containerized = true;
    }

    stack
}
