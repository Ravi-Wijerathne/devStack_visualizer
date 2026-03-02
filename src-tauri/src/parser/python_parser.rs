use super::{FileAnalysis, LanguageParser};
use anyhow::{Context, Result};
use std::path::Path;

/// Python source file parser using regex-based extraction
pub struct PythonParser {
    _private: (),
}

impl PythonParser {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for PythonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for PythonParser {
    fn parse(&self, path: &Path) -> Result<FileAnalysis> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let mut analysis = FileAnalysis::new(path.to_path_buf());

        for line in source.lines() {
            let trimmed = line.trim();

            // Skip comments and empty lines
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // import module / import module as alias
            if trimmed.starts_with("import ") && !trimmed.starts_with("import (") {
                let rest = trimmed.strip_prefix("import ").unwrap().trim();
                // Handle "import a, b, c"
                for part in rest.split(',') {
                    let module = part.split(" as ").next().unwrap_or(part).trim();
                    if !module.is_empty() {
                        analysis.imports.push(module.to_string());
                    }
                }
            }
            // from module import ...
            else if trimmed.starts_with("from ") {
                if let Some(module) = extract_from_import(trimmed) {
                    analysis.imports.push(module);
                }
            }
            // def function_name(...)
            else if trimmed.starts_with("def ") {
                if let Some(name) = extract_python_func_name(trimmed) {
                    analysis.functions.push(name);
                }
            }
            // class ClassName(...)
            else if trimmed.starts_with("class ") {
                if let Some(name) = extract_python_class_name(trimmed) {
                    analysis.structs.push(name);
                }
            }
            // Decorators that wrap functions/classes — handled by the next def/class line
        }

        Ok(analysis)
    }
}

/// Extract module name from "from module import ..."
fn extract_from_import(line: &str) -> Option<String> {
    let rest = line.strip_prefix("from ")?.trim();
    let module = rest.split_whitespace().next()?;
    if module == "." || module == ".." {
        // Relative import — keep as-is
        Some(module.to_string())
    } else {
        Some(module.to_string())
    }
}

/// Extract function name from "def func_name(...):"
fn extract_python_func_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("def ")?.trim();
    let name = rest.split('(').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Extract class name from "class ClassName(...):" or "class ClassName:"
fn extract_python_class_name(line: &str) -> Option<String> {
    let rest = line.strip_prefix("class ")?.trim();
    let name = rest.split(|c: char| c == '(' || c == ':').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
