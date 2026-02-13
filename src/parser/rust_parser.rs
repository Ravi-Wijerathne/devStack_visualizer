use super::{FileAnalysis, LanguageParser};
use anyhow::{Context, Result};
use std::path::Path;
use tree_sitter::Parser;

/// Rust source file parser using tree-sitter
pub struct RustParser {
    _private: (), // prevent direct construction to enforce use of new()
}

impl RustParser {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for RustParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for RustParser {
    fn parse(&self, path: &Path) -> Result<FileAnalysis> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let mut parser = Parser::new();
        let language = tree_sitter_rust::LANGUAGE;
        parser
            .set_language(&language.into())
            .context("Failed to set Rust grammar for tree-sitter")?;

        let tree = parser
            .parse(&source, None)
            .context("Failed to parse Rust source")?;

        let root = tree.root_node();
        let mut analysis = FileAnalysis::new(path.to_path_buf());

        visit_node(root, &source, &mut analysis);

        Ok(analysis)
    }
}

/// Recursively visit AST nodes and extract relevant information
fn visit_node(node: tree_sitter::Node, source: &str, analysis: &mut FileAnalysis) {
    match node.kind() {
        "use_declaration" => {
            if let Some(import_text) = extract_node_text(node, source) {
                // Clean the import: remove "use " prefix and ";" suffix
                let cleaned = import_text
                    .trim_start_matches("use ")
                    .trim_end_matches(';')
                    .trim()
                    .to_string();
                analysis.imports.push(cleaned);
            }
        }
        "function_item" => {
            if let Some(name) = find_child_by_field(node, "name", source) {
                analysis.functions.push(name);
            }
        }
        "struct_item" => {
            if let Some(name) = find_child_by_field(node, "name", source) {
                analysis.structs.push(name);
            }
        }
        "impl_item" => {
            // Also capture methods inside impl blocks
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "declaration_list" {
                        for j in 0..child.child_count() {
                            if let Some(item) = child.child(j) {
                                if item.kind() == "function_item" {
                                    if let Some(name) = find_child_by_field(item, "name", source) {
                                        analysis.functions.push(name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "enum_item" => {
            if let Some(name) = find_child_by_field(node, "name", source) {
                analysis.structs.push(name); // treat enums like structs for graphing
            }
        }
        "mod_item" => {
            // Module declaration — treat as an import-like relationship
            if let Some(name) = find_child_by_field(node, "name", source) {
                analysis.imports.push(format!("mod {}", name));
            }
        }
        _ => {}
    }

    // Recurse into children (skip impl_item since we handle it above)
    if node.kind() != "impl_item" {
        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                visit_node(child, source, analysis);
            }
        }
    }
}

/// Extract the full text of a node
fn extract_node_text(node: tree_sitter::Node, source: &str) -> Option<String> {
    let start = node.start_byte();
    let end = node.end_byte();
    if end <= source.len() {
        Some(source[start..end].to_string())
    } else {
        None
    }
}

/// Find a named child field and extract its text
fn find_child_by_field(
    node: tree_sitter::Node,
    field_name: &str,
    source: &str,
) -> Option<String> {
    node.child_by_field_name(field_name)
        .and_then(|child| extract_node_text(child, source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn parse_rust_source(code: &str) -> FileAnalysis {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(code.as_bytes()).unwrap();
        let parser = RustParser::new();
        parser.parse(file.path()).unwrap()
    }

    #[test]
    fn test_parse_use_declaration() {
        let analysis = parse_rust_source("use std::collections::HashMap;");
        assert_eq!(analysis.imports, vec!["std::collections::HashMap"]);
    }

    #[test]
    fn test_parse_function() {
        let analysis = parse_rust_source("fn hello() { }");
        assert_eq!(analysis.functions, vec!["hello"]);
    }

    #[test]
    fn test_parse_struct() {
        let analysis = parse_rust_source("struct Foo { x: i32 }");
        assert_eq!(analysis.structs, vec!["Foo"]);
    }

    #[test]
    fn test_parse_combined() {
        let code = r#"
use std::io;
use anyhow::Result;

struct Config {
    path: String,
}

fn main() {
    println!("hello");
}

fn helper() -> i32 {
    42
}
"#;
        let analysis = parse_rust_source(code);
        assert_eq!(analysis.imports.len(), 2);
        assert_eq!(analysis.structs, vec!["Config"]);
        assert_eq!(analysis.functions, vec!["main", "helper"]);
    }
}
