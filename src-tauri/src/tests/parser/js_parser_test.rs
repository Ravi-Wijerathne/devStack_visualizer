#[cfg(test)]
mod tests {
    #[test]
    fn test_js_parser_es6_imports() {
        let source = r#"
import React from 'react';
import { useState, useEffect } from 'react';
import MyComponent from './MyComponent';
import utils from '../utils';
import "polyfills";
"#;
        let imports = extract_es6_imports(source);
        assert_eq!(imports.len(), 5);
        assert!(imports.contains(&"'react'".to_string()));
        assert!(imports.contains(&"'./MyComponent'".to_string()));
        assert!(imports.contains(&"'../utils'".to_string()));
    }

    #[test]
    fn test_js_parser_commonjs_require() {
        let source = r#"
const fs = require('fs');
const path = require('path');
const { readFile } require('./helpers');
"#;
        let imports = extract_require_imports(source);
        assert!(imports.contains(&"'fs'".to_string()));
        assert!(imports.contains(&"'path'".to_string()));
    }

    #[test]
    fn test_js_parser_function_declarations() {
        let source = r#"
function main() {
    return 0;
}

async function fetchData(url) {
    return fetch(url);
}

export function helper() {
    console.log('help');
}

export default function App() {
    return <div />;
}

export async function processItems(items) {
    return items.map(process);
}
"#;
        let functions = extract_js_functions(source);
        assert_eq!(functions.len(), 5);
        assert!(functions.contains(&"main".to_string()));
        assert!(functions.contains(&"fetchData".to_string()));
        assert!(functions.contains(&"helper".to_string()));
        assert!(functions.contains(&"App".to_string()));
        assert!(functions.contains(&"processItems".to_string()));
    }

    #[test]
    fn test_js_parser_arrow_functions() {
        let source = r#"
const add = (a, b) => a + b;

const fetchUser = async (id) => {
    return await api.getUser(id);
};

const multiply = function(x, y) {
    return x * y;
};

export const formatDate = (date) => {
    return date.toISOString();
};

export let validateInput = (input) => {
    return input.length > 0;
};
"#;
        let functions = extract_const_functions(source);
        assert!(functions.contains(&"add".to_string()));
        assert!(functions.contains(&"fetchUser".to_string()));
        assert!(functions.contains(&"multiply".to_string()));
        assert!(functions.contains(&"formatDate".to_string()));
        assert!(functions.contains(&"validateInput".to_string()));
    }

    #[test]
    fn test_js_parser_class_declarations() {
        let source = r#"
class User {
    constructor(name) {
        this.name = name;
    }
}

export class Controller {
    handleRequest(req) {
        return req;
    }
}

export default class App {
    render() {
        return null;
    }
}

class Animal {
    speak() {
        console.log('...');
    }
}
"#;
        let classes = extract_js_classes(source);
        assert_eq!(classes.len(), 4);
        assert!(classes.contains(&"User".to_string()));
        assert!(classes.contains(&"Controller".to_string()));
        assert!(classes.contains(&"App".to_string()));
        assert!(classes.contains(&"Animal".to_string()));
    }

    #[test]
    fn test_js_parser_typescript_interfaces() {
        let source = r#"
interface User {
    id: number;
    name: string;
}

export interface Config {
    apiUrl: string;
    timeout: number;
}

type Status = 'pending' | 'active' | 'done';

export type Result<T> = {
    data: T;
    error: null;
} | {
    data: null;
    error: Error;
};
"#;
        let types = extract_ts_types(source);
        assert_eq!(types.len(), 4);
        assert!(types.contains(&"User".to_string()));
        assert!(types.contains(&"Config".to_string()));
        assert!(types.contains(&"Status".to_string()));
        assert!(types.contains(&"Result".to_string()));
    }

    #[test]
    fn test_js_parser_skips_comments() {
        let source = r#"
// This is a single-line comment
const x = 1; // inline comment

/*
 * Block comment
 */

const y = 2;

export function commented() {
    // commented code
    return 0;
}
"#;
        let functions = extract_js_functions(source);
        let imports = extract_es6_imports(source);
        assert!(imports.is_empty());
        assert!(functions.contains(&"commented".to_string()));
    }

    #[test]
    fn test_js_parser_skips_block_comments() {
        let source = r#"
/* 
   Multi-line comment block
   contains code-like text
   import foo from 'bar'
   function fake() {}
*/

export function real() {
    return true;
}
"#;
        let functions = extract_js_functions(source);
        assert_eq!(functions.len(), 1);
        assert!(functions.contains(&"real".to_string()));
    }

    #[test]
    fn test_js_parser_mixed_imports() {
        let source = r#"
import React, { useState } from 'react';
import ReactDOM from 'react-dom';
import './styles.css';
const _ = require('lodash');
const moment = require('moment');
import type { Config } from './types';
"#;
        let es6_imports = extract_es6_imports(source);
        let require_imports = extract_require_imports(source);
        assert_eq!(es6_imports.len(), 4);
        assert_eq!(require_imports.len(), 2);
    }

    #[test]
    fn test_js_parser_typescript_generic_functions() {
        let source = r#"
function identity<T>(arg: T): T {
    return arg;
}

async function fetch<T>(url: string): Promise<T> {
    const response = await fetch(url);
    return response.json();
}
"#;
        let functions = extract_js_functions(source);
        assert_eq!(functions.len(), 2);
        assert!(functions.contains(&"identity".to_string()));
        assert!(functions.contains(&"fetch".to_string()));
    }

    fn extract_es6_imports(source: &str) -> Vec<String> {
        let mut imports = Vec::new();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

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

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if trimmed.starts_with("import ") {
                if let Some(module) = extract_es6_import(trimmed) {
                    imports.push(module);
                }
            }
        }
        imports
    }

    fn extract_require_imports(source: &str) -> Vec<String> {
        let mut imports = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.contains("require(") {
                if let Some(module) = extract_require(trimmed) {
                    imports.push(module);
                }
            }
        }
        imports
    }

    fn extract_es6_import(line: &str) -> Option<String> {
        let from_idx = line.find(" from ")?;
        let rest = line[from_idx + 6..].trim();
        extract_string_literal(rest).or_else(|| {
            let rest = line.strip_prefix("import ")?.trim();
            extract_string_literal(rest)
        })
    }

    fn extract_require(line: &str) -> Option<String> {
        let req_idx = line.find("require(")?;
        let rest = &line[req_idx + 8..];
        extract_string_literal(rest)
    }

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
        Some(rest[..end].to_string())
    }

    fn extract_js_functions(source: &str) -> Vec<String> {
        let mut functions = Vec::new();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

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

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if let Some(name) = extract_js_function(trimmed) {
                functions.push(name);
            }
        }
        functions
    }

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

        let name = rest
            .split(|c: char| c == '(' || c == '<' || c == ' ')
            .next()?
            .trim();
        if name.is_empty() || name == "*" {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn extract_const_functions(source: &str) -> Vec<String> {
        let mut functions = Vec::new();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

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

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if let Some(name) = extract_const_function(trimmed) {
                functions.push(name);
            }
        }
        functions
    }

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

        let has_arrow = rest.contains("=>") || rest.contains("function");
        if !has_arrow {
            return None;
        }

        let name = rest
            .split(|c: char| c == ' ' || c == ':' || c == '=')
            .next()?
            .trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn extract_js_classes(source: &str) -> Vec<String> {
        let mut classes = Vec::new();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

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

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if let Some(name) = extract_js_class(trimmed) {
                classes.push(name);
            }
        }
        classes
    }

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

        let name = rest
            .split(|c: char| c == ' ' || c == '{' || c == '<')
            .next()?
            .trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn extract_ts_types(source: &str) -> Vec<String> {
        let mut types = Vec::new();
        let mut in_block_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

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

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            if let Some(name) = extract_ts_type(trimmed) {
                types.push(name);
            }
        }
        types
    }

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

        let name = rest
            .split(|c: char| c == ' ' || c == '{' || c == '<' || c == '=')
            .next()?
            .trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }
}
