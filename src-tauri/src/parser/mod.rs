pub mod js_parser;
pub mod python_parser;
pub mod rust_parser;

use anyhow::Result;
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Extracted analysis data from a single source file
#[derive(Debug, Clone, Serialize)]
pub struct FileAnalysis {
    pub file: PathBuf,
    pub imports: Vec<String>,
    pub functions: Vec<String>,
    pub structs: Vec<String>,
}

impl FileAnalysis {
    pub fn new(file: PathBuf) -> Self {
        Self {
            file,
            imports: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
        }
    }

    /// A simple complexity score based on the number of definitions
    pub fn complexity_score(&self) -> usize {
        self.functions.len() + self.structs.len()
    }

    /// Human-readable complexity label
    pub fn complexity_label(&self) -> &str {
        match self.complexity_score() {
            0..=2 => "Low",
            3..=6 => "Medium",
            _ => "High",
        }
    }
}

/// Trait that all language parsers must implement
pub trait LanguageParser: Send + Sync {
    fn parse(&self, path: &Path) -> Result<FileAnalysis>;
}
