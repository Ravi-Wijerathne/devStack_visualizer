use crate::analyzer::{DependencyGraph, GraphData};
use crate::graph;
use crate::language_detector::{self, ProjectStack};
use crate::parser::rust_parser::RustParser;
use crate::parser::{FileAnalysis, LanguageParser};
use crate::scanner;
use rayon::prelude::*;
use serde::Serialize;
use std::path::Path;

/// Full analysis result returned to the frontend
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    pub stack: ProjectStack,
    pub files_parsed: usize,
    pub total_nodes: usize,
    pub total_edges: usize,
    pub circular_dependencies: Vec<(String, String)>,
    pub graph_data: GraphData,
    pub file_analyses: Vec<FileAnalysis>,
}

/// Complexity report for a single file
#[derive(Debug, Clone, Serialize)]
pub struct ComplexityReport {
    pub file: String,
    pub complexity: String,
    pub functions_count: usize,
    pub structs_count: usize,
    pub score: usize,
}

/// Layer detection result
#[derive(Debug, Clone, Serialize)]
pub struct LayerInfo {
    pub controllers: Vec<String>,
    pub services: Vec<String>,
    pub models: Vec<String>,
    pub others: Vec<String>,
}

/// Analyze a project directory and return the full result
#[tauri::command]
pub fn analyze_project(path: String) -> Result<AnalysisResult, String> {
    let project_root = Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid project path '{}': {}", path, e))?;

    // Scan project files
    let project_files =
        scanner::scan_project(&project_root).map_err(|e| format!("Scan error: {}", e))?;

    if project_files.is_empty() {
        return Err(format!(
            "No source files found in {}",
            project_root.display()
        ));
    }

    // Detect stack
    let stack = language_detector::detect_stack(&project_files, &project_root);

    // Parse source files (Rust only for now, parallel)
    let rust_parser = RustParser::new();
    let analyses: Vec<FileAnalysis> = project_files
        .rust_files
        .par_iter()
        .filter_map(|file| match rust_parser.parse(file) {
            Ok(analysis) => Some(analysis),
            Err(_) => None,
        })
        .collect();

    // Build dependency graph
    let dep_graph = DependencyGraph::build(&analyses, &project_root)
        .map_err(|e| format!("Graph build error: {}", e))?;

    let summary = dep_graph.summary();
    let graph_data = dep_graph.to_graph_data(&analyses);

    Ok(AnalysisResult {
        stack,
        files_parsed: analyses.len(),
        total_nodes: summary.total_nodes,
        total_edges: summary.total_edges,
        circular_dependencies: summary.circular_dependencies,
        graph_data,
        file_analyses: analyses,
    })
}

/// Get details for a specific file
#[tauri::command]
pub fn get_file_details(path: String) -> Result<FileAnalysis, String> {
    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err(format!("File not found: {}", path));
    }

    let ext = file_path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match ext {
        "rs" => {
            let parser = RustParser::new();
            parser
                .parse(file_path)
                .map_err(|e| format!("Parse error: {}", e))
        }
        _ => Err(format!("Unsupported file type: .{}", ext)),
    }
}

/// Export the dependency graph to a file (DOT → Graphviz render)
#[tauri::command]
pub fn export_graph(path: String, format: String, output_path: String) -> Result<String, String> {
    let project_root = Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid project path: {}", e))?;

    // Re-analyze to get graph data
    let project_files =
        scanner::scan_project(&project_root).map_err(|e| format!("Scan error: {}", e))?;

    let rust_parser = RustParser::new();
    let analyses: Vec<FileAnalysis> = project_files
        .rust_files
        .par_iter()
        .filter_map(|file| rust_parser.parse(file).ok())
        .collect();

    let dep_graph = DependencyGraph::build(&analyses, &project_root)
        .map_err(|e| format!("Graph build error: {}", e))?;

    // Generate DOT
    let dot_content =
        graph::dot_generator::generate_dot(&dep_graph).map_err(|e| format!("DOT error: {}", e))?;

    let dot_path = project_root.join("architecture.dot");
    graph::dot_generator::save_dot(&dot_content, &dot_path)
        .map_err(|e| format!("Save DOT error: {}", e))?;

    // If Graphviz is available, render
    if graph::renderer::is_graphviz_available() {
        let fmt = graph::renderer::OutputFormat::from_str(&format)
            .map_err(|e| format!("Format error: {}", e))?;

        let out = if output_path.is_empty() {
            project_root.join(format!("architecture.{}", fmt.extension()))
        } else {
            Path::new(&output_path).to_path_buf()
        };

        graph::renderer::render_dot(&dot_path, &out, fmt)
            .map_err(|e| format!("Render error: {}", e))?;

        Ok(format!("Exported to {}", out.display()))
    } else {
        Ok(format!(
            "DOT file saved to {}. Install Graphviz to render to {}.",
            dot_path.display(),
            format
        ))
    }
}

/// Detect the technology stack of a project
#[tauri::command]
pub fn detect_stack(path: String) -> Result<ProjectStack, String> {
    let project_root = Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;

    let project_files =
        scanner::scan_project(&project_root).map_err(|e| format!("Scan error: {}", e))?;

    Ok(language_detector::detect_stack(
        &project_files,
        &project_root,
    ))
}

/// Get complexity reports for all files in a project
#[tauri::command]
pub fn get_complexity(path: String) -> Result<Vec<ComplexityReport>, String> {
    let project_root = Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;

    let project_files =
        scanner::scan_project(&project_root).map_err(|e| format!("Scan error: {}", e))?;

    let rust_parser = RustParser::new();
    let analyses: Vec<FileAnalysis> = project_files
        .rust_files
        .par_iter()
        .filter_map(|file| rust_parser.parse(file).ok())
        .collect();

    Ok(analyses
        .iter()
        .map(|a| ComplexityReport {
            file: a
                .file
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            complexity: a.complexity_label().to_string(),
            functions_count: a.functions.len(),
            structs_count: a.structs.len(),
            score: a.complexity_score(),
        })
        .collect())
}

/// Detect architecture layers (MVC / Clean Architecture)
#[tauri::command]
pub fn detect_layers(path: String) -> Result<LayerInfo, String> {
    let project_root = Path::new(&path)
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;

    let project_files =
        scanner::scan_project(&project_root).map_err(|e| format!("Scan error: {}", e))?;

    let rust_parser = RustParser::new();
    let analyses: Vec<FileAnalysis> = project_files
        .rust_files
        .par_iter()
        .filter_map(|file| rust_parser.parse(file).ok())
        .collect();

    let dep_graph = DependencyGraph::build(&analyses, &project_root)
        .map_err(|e| format!("Graph build error: {}", e))?;

    let mut layers = LayerInfo {
        controllers: Vec::new(),
        services: Vec::new(),
        models: Vec::new(),
        others: Vec::new(),
    };

    for node_idx in dep_graph.graph.node_indices() {
        let label = dep_graph.graph[node_idx].clone();
        let lower = label.to_lowercase();

        if lower.contains("controller") || lower.contains("handler") || lower.contains("route") {
            layers.controllers.push(label);
        } else if lower.contains("service") || lower.contains("usecase") {
            layers.services.push(label);
        } else if lower.contains("model") || lower.contains("entity") || lower.contains("schema")
        {
            layers.models.push(label);
        } else {
            layers.others.push(label);
        }
    }

    Ok(layers)
}
