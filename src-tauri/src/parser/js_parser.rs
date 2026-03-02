use super::{FileAnalysis, LanguageParser};
use anyhow::{Context, Result};
use std::path::Path;

/// JavaScript/TypeScript source file parser using regex-based extraction
pub struct JsParser {
    _private: (),
}

impl JsParser {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Default for JsParser {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageParser for JsParser {
    fn parse(&self, path: &Path) -> Result<FileAnalysis> {
        let source = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let mut analysis = FileAnalysis::new(path.to_path_buf());
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

            // Handle block comments
            if in_block_comment {
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }
            if trimmed.starts_with("/*") {
                in_block_comment = true;
                if trimmed.contains("*/") {
                    in_block_comment = false;
                }
                continue;
            }

            // Skip single-line comments and empty lines
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            // ES6 imports: import ... from '...'
            if trimmed.starts_with("import ") {
                if let Some(module) = extract_es6_import(trimmed) {
                    analysis.imports.push(module);
                }
            }
            // CommonJS require: const x = require('...')
            else if trimmed.contains("require(") {
                if let Some(module) = extract_require(trimmed) {
                    analysis.imports.push(module);
                }
            }

            // Function declarations: function name(...) / async function name(...)
            if let Some(name) = extract_js_function(trimmed) {
                analysis.functions.push(name);
            }
            // Arrow/const functions: const name = (...) => / const name = function
            else if let Some(name) = extract_const_function(trimmed) {
                analysis.functions.push(name);
            }

            // Class declarations: class Name { / class Name extends ...
            if let Some(name) = extract_js_class(trimmed) {
                analysis.structs.push(name);
            }
            // Interface/type declarations (TypeScript)
            else if let Some(name) = extract_ts_type(trimmed) {
                analysis.structs.push(name);
            }
        }

        Ok(analysis)
    }
}

/// Extract module path from ES6 import statement
fn extract_es6_import(line: &str) -> Option<String> {
    // import ... from 'module' or import ... from "module"
    let from_idx = line.find(" from ")?;
    let rest = line[from_idx + 6..].trim();
    extract_string_literal(rest)
        .or_else(|| {
            // import 'module' (side-effect import)
            let rest = line.strip_prefix("import ")?.trim();
            extract_string_literal(rest)
        })
}

/// Extract module path from require('module') or require("module")
fn extract_require(line: &str) -> Option<String> {
    let req_idx = line.find("require(")?;
    let rest = &line[req_idx + 8..];
    extract_string_literal(rest)
}

/// Extract a string literal from text starting with ' or "
fn extract_string_literal(text: &str) -> Option<String> {
    let text = text.trim().trim_end_matches(';').trim();
    let (quote, rest) = if text.starts_with('\'') {
        ('\'', &text[1..])
    } else if text.starts_with('"') {
        ('"', &text[1..])
    } else {
        return None;
    };
    let end = rest.find(quote)?;
    let module = &rest[..end];
    if module.is_empty() {
        None
    } else {
        Some(module.to_string())
    }
}

/// Extract function name from "function name(..." or "async function name(..."
fn extract_js_function(line: &str) -> Option<String> {
    let rest = if line.starts_with("function ") {
        line.strip_prefix("function ")?.trim()
    } else if line.starts_with("async function ") {
        line.strip_prefix("async function ")?.trim()
    } else if line.starts_with("export function ") {
        line.strip_prefix("export function ")?.trim()
    } else if line.starts_with("export default function ") {
        line.strip_prefix("export default function ")?.trim()
    } else if line.starts_with("export async function ") {
        line.strip_prefix("export async function ")?.trim()
    } else {
        return None;
    };

    let name = rest.split(|c: char| c == '(' || c == '<' || c == ' ').next()?.trim();
    if name.is_empty() || name == "*" {
        None
    } else {
        Some(name.to_string())
    }
}

/// Extract function name from const/let/var name = (...) => or = function
fn extract_const_function(line: &str) -> Option<String> {
    let rest = if line.starts_with("const ") {
        line.strip_prefix("const ")?
    } else if line.starts_with("let ") {
        line.strip_prefix("let ")?
    } else if line.starts_with("var ") {
        line.strip_prefix("var ")?
    } else if line.starts_with("export const ") {
        line.strip_prefix("export const ")?
    } else if line.starts_with("export let ") {
        line.strip_prefix("export let ")?
    } else {
        return None;
    };

    // Check if this is a function assignment
    let has_arrow = rest.contains("=>") || rest.contains("function");
    if !has_arrow {
        return None;
    }

    let name = rest.split(|c: char| c == ' ' || c == ':' || c == '=').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Extract class name from "class Name" / "export class Name"
fn extract_js_class(line: &str) -> Option<String> {
    let rest = if line.starts_with("class ") {
        line.strip_prefix("class ")?
    } else if line.starts_with("export class ") {
        line.strip_prefix("export class ")?
    } else if line.starts_with("export default class ") {
        line.strip_prefix("export default class ")?
    } else {
        return None;
    };

    let name = rest.split(|c: char| c == ' ' || c == '{' || c == '<').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

/// Extract TypeScript interface/type name
fn extract_ts_type(line: &str) -> Option<String> {
    let rest = if line.starts_with("interface ") {
        line.strip_prefix("interface ")?
    } else if line.starts_with("export interface ") {
        line.strip_prefix("export interface ")?
    } else if line.starts_with("type ") && line.contains('=') {
        line.strip_prefix("type ")?
    } else if line.starts_with("export type ") && line.contains('=') {
        line.strip_prefix("export type ")?
    } else {
        return None;
    };

    let name = rest.split(|c: char| c == ' ' || c == '{' || c == '<' || c == '=').next()?.trim();
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}
