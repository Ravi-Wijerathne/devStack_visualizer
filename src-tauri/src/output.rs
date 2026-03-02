use crate::analyzer::DependencySummary;
use crate::language_detector::ProjectStack;
use crate::parser::FileAnalysis;
use serde::Serialize;

/// Complete analysis result for JSON/IPC output
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisOutput {
    pub stack: ProjectStack,
    pub files_parsed: usize,
    pub dependency_summary: DependencySummary,
    pub file_analyses: Vec<FileAnalysis>,
}
