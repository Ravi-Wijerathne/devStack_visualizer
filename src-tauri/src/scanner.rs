use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Directories to skip during scanning
const SKIP_DIRS: &[&str] = &["target", "node_modules", ".git", "__pycache__", "venv", ".venv", "dist"];

/// Maximum file size to parse (1 MB)
const MAX_FILE_SIZE: u64 = 1_048_576;

/// Collected project files organized by language
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ProjectFiles {
    pub rust_files: Vec<PathBuf>,
    pub python_files: Vec<PathBuf>,
    pub js_files: Vec<PathBuf>,
    pub config_files: Vec<PathBuf>,
    pub docker_files: Vec<PathBuf>,
}

impl ProjectFiles {
    /// Total number of source files found
    pub fn total_source_files(&self) -> usize {
        self.rust_files.len() + self.python_files.len() + self.js_files.len()
    }

    /// Check if any source files were found
    pub fn is_empty(&self) -> bool {
        self.total_source_files() == 0
    }
}

/// Scan the project directory and collect files by language
pub fn scan_project(root: &Path) -> Result<ProjectFiles> {
    let root = root
        .canonicalize()
        .with_context(|| format!("Failed to resolve path: {}", root.display()))?;

    let mut files = ProjectFiles::default();

    for entry in WalkDir::new(&root)
        .follow_links(true)
        .into_iter()
        .filter_entry(|e| !is_skipped_dir(e))
    {
        let entry = entry.with_context(|| "Error reading directory entry")?;

        if !entry.file_type().is_file() {
            continue;
        }

        // Skip files that are too large
        if let Ok(metadata) = entry.metadata() {
            if metadata.len() > MAX_FILE_SIZE {
                continue;
            }
        }

        let path = entry.path().to_path_buf();
        let file_name = entry.file_name().to_string_lossy().to_string();

        match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => files.rust_files.push(path),
            Some("py") => files.python_files.push(path),
            Some("js") | Some("ts") | Some("jsx") | Some("tsx") => files.js_files.push(path),
            Some("toml") | Some("json") | Some("yaml") | Some("yml") | Some("txt") => {
                files.config_files.push(path);
            }
            _ => {
                if file_name == "Dockerfile" || file_name.starts_with("Dockerfile.") {
                    files.docker_files.push(path);
                } else if file_name == "docker-compose.yml" || file_name == "docker-compose.yaml" {
                    files.docker_files.push(path);
                }
            }
        }
    }

    Ok(files)
}

/// Check if a directory entry should be skipped
fn is_skipped_dir(entry: &walkdir::DirEntry) -> bool {
    if entry.file_type().is_dir() {
        if let Some(name) = entry.file_name().to_str() {
            return SKIP_DIRS.contains(&name);
        }
    }
    false
}
