#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn test_python_parser_simple_import() {
        let source = r#"
import os
import sys
from typing import List
from collections import defaultdict
"#;
        let imports = extract_python_imports(source);
        assert!(imports.contains(&"os".to_string()));
        assert!(imports.contains(&"sys".to_string()));
        assert!(imports.contains(&"typing".to_string()));
        assert!(imports.contains(&"collections".to_string()));
    }

    #[test]
    fn test_python_parser_from_import() {
        let source = r#"
from django.http import HttpRequest
from .models import User
from ..utils.helpers import format_date
"#;
        let imports = extract_python_imports(source);
        assert!(imports.contains(&"django.http".to_string()));
        assert!(imports.contains(&".".to_string()));
        assert!(imports.contains(&"..".to_string()));
    }

    #[test]
    fn test_python_parser_functions() {
        let source = r#"
def main():
    pass

def calculate_sum(a, b):
    return a + b

async def fetch_data(url):
    response = await client.get(url)
    return response
"#;
        let functions = extract_python_functions(source);
        assert_eq!(functions.len(), 3);
        assert!(functions.contains(&"main".to_string()));
        assert!(functions.contains(&"calculate_sum".to_string()));
        assert!(functions.contains(&"fetch_data".to_string()));
    }

    #[test]
    fn test_python_parser_classes() {
        let source = r#"
class User:
    pass

class Animal:
    def __init__(self, name):
        self.name = name

class Dog(Animal):
    def bark(self):
        print("Woof!")
"#;
        let classes = extract_python_classes(source);
        assert_eq!(classes.len(), 3);
        assert!(classes.contains(&"User".to_string()));
        assert!(classes.contains(&"Animal".to_string()));
        assert!(classes.contains(&"Dog".to_string()));
    }

    #[test]
    fn test_python_parser_skips_comments() {
        let source = r#"
# This is a comment
import os  # inline comment
"""
Docstring comment
"""
def func():
    pass
"#;
        let imports = extract_python_imports(source);
        let functions = extract_python_functions(source);
        assert!(imports.contains(&"os".to_string()));
        assert_eq!(imports.len(), 1);
        assert_eq!(functions.len(), 1);
    }

    #[test]
    fn test_python_parser_multiline_import() {
        let source = r#"
from os.path import (
    join,
    split,
    exists
)
"#;
        let imports = extract_python_imports(source);
        assert!(imports.contains(&"os.path".to_string()));
    }

    #[test]
    fn test_python_parser_import_with_alias() {
        let source = r#"
import pandas as pd
import numpy as np
from sqlalchemy import create_engine as ce
"#;
        let imports = extract_python_imports(source);
        assert!(imports.contains(&"pandas".to_string()));
        assert!(imports.contains(&"numpy".to_string()));
        assert!(imports.contains(&"sqlalchemy".to_string()));
    }

    #[test]
    fn test_python_parser_complex_class_definitions() {
        let source = r#"
class MyClass(BaseClass, Protocol):
    pass

class GenericClass(Generic[T]):
    pass

class ABCClass(metaclass=ABCMeta):
    pass
"#;
        let classes = extract_python_classes(source);
        assert_eq!(classes.len(), 3);
        assert!(classes.contains(&"MyClass".to_string()));
        assert!(classes.contains(&"GenericClass".to_string()));
        assert!(classes.contains(&"ABCClass".to_string()));
    }

    #[test]
    fn test_complexity_score() {
        let source = r#"
def func1(): pass
def func2(): pass
def func3(): pass
class Class1: pass
class Class2: pass
"#;
        let score = calculate_complexity_score(source);
        assert_eq!(score, 5);
    }

    fn extract_python_imports(source: &str) -> Vec<String> {
        let mut imports = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if trimmed.starts_with("import ") && !trimmed.starts_with("import (") {
                let rest = trimmed.strip_prefix("import ").unwrap().trim();
                for part in rest.split(',') {
                    let module = part.split(" as ").next().unwrap_or(part).trim();
                    if !module.is_empty() {
                        imports.push(module.to_string());
                    }
                }
            } else if trimmed.starts_with("from ") {
                if let Some(module) = extract_from_import(trimmed) {
                    imports.push(module);
                }
            }
        }
        imports
    }

    fn extract_python_functions(source: &str) -> Vec<String> {
        let mut functions = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("def ") {
                if let Some(name) = extract_python_func_name(trimmed) {
                    functions.push(name);
                }
            }
        }
        functions
    }

    fn extract_python_classes(source: &str) -> Vec<String> {
        let mut classes = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("class ") {
                if let Some(name) = extract_python_class_name(trimmed) {
                    classes.push(name);
                }
            }
        }
        classes
    }

    fn extract_from_import(line: &str) -> Option<String> {
        let rest = line.strip_prefix("from ")?.trim();
        let module = rest.split_whitespace().next()?;
        Some(module.to_string())
    }

    fn extract_python_func_name(line: &str) -> Option<String> {
        let rest = line.strip_prefix("def ")?.trim();
        let name = rest.split('(').next()?.trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn extract_python_class_name(line: &str) -> Option<String> {
        let rest = line.strip_prefix("class ")?.trim();
        let name = rest.split(|c: char| c == '(' || c == ':').next()?.trim();
        if name.is_empty() {
            None
        } else {
            Some(name.to_string())
        }
    }

    fn calculate_complexity_score(source: &str) -> usize {
        let functions = extract_python_functions(source);
        let classes = extract_python_classes(source);
        functions.len() + classes.len()
    }
}
