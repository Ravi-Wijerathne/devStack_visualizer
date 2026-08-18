use crate::parser::FileAnalysis;
use anyhow::Result;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// The dependency graph built from file analysis results
#[derive(Debug)]
pub struct DependencyGraph {
    pub graph: DiGraph<String, ()>,
    #[allow(dead_code)]
    pub node_map: HashMap<String, NodeIndex>,
}

/// Summary of dependency analysis
#[derive(Debug, Clone, Serialize)]
pub struct DependencySummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub circular_dependencies: Vec<(String, String)>,
}

/// A single node in the graph for frontend consumption
#[derive(Debug, Clone, Serialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub file_path: String,
    pub node_type: String, // "rust", "python", "js", "other"
    pub complexity: String,
    pub functions_count: usize,
    pub structs_count: usize,
}

/// A single edge in the graph for frontend consumption
#[derive(Debug, Clone, Serialize)]
pub struct GraphEdge {
    pub source: String,
    pub target: String,
    pub is_circular: bool,
}

/// Complete graph data for the frontend
#[derive(Debug, Clone, Serialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl DependencyGraph {
    /// Build a dependency graph from a list of file analyses
    pub fn build(analyses: &[FileAnalysis], project_root: &Path) -> Result<Self> {
        let mut graph = DiGraph::<String, ()>::new();
        let mut node_map = HashMap::new();

        // First pass: add all files as nodes
        for analysis in analyses {
            let label = make_label(&analysis.file, project_root);
            if !node_map.contains_key(&label) {
                let idx = graph.add_node(label.clone());
                node_map.insert(label, idx);
            }
        }

        // Second pass: add edges based on imports
        for analysis in analyses {
            let source_label = make_label(&analysis.file, project_root);
            let source_idx = node_map[&source_label];

            for import in &analysis.imports {
                if let Some(target_label) = resolve_import(import, &node_map) {
                    let target_idx = node_map[&target_label];
                    if source_idx != target_idx {
                        graph.add_edge(source_idx, target_idx, ());
                    }
                }
            }
        }

        Ok(DependencyGraph { graph, node_map })
    }

    /// Get a summary of the dependency graph
    pub fn summary(&self) -> DependencySummary {
        DependencySummary {
            total_nodes: self.graph.node_count(),
            total_edges: self.graph.edge_count(),
            circular_dependencies: self.detect_cycles(),
        }
    }

    /// Convert to frontend-friendly graph data
    pub fn to_graph_data(&self, analyses: &[FileAnalysis]) -> GraphData {
        let cycles = self.detect_cycles();

        // Collect all filenames to detect duplicates — use parent/file for disambiguation
        let all_labels: Vec<String> = self
            .graph
            .node_indices()
            .map(|idx| {
                let path = &self.graph[idx];
                path.rsplit('/').next().unwrap_or(path).to_string()
            })
            .collect();
        let mut filename_counts: HashMap<String, usize> = HashMap::new();
        for name in &all_labels {
            *filename_counts.entry(name.clone()).or_insert(0) += 1;
        }

        let nodes: Vec<GraphNode> = self
            .graph
            .node_indices()
            .map(|idx| {
                let label = &self.graph[idx];
                let analysis = analyses.iter().find(|a| {
                    let a_label = a.file.to_string_lossy().replace('\\', "/");
                    a_label.ends_with(label.as_str())
                });

                let node_type = if label.ends_with(".rs") {
                    "rust"
                } else if label.ends_with(".py") {
                    "python"
                } else if label.ends_with(".js")
                    || label.ends_with(".ts")
                    || label.ends_with(".jsx")
                    || label.ends_with(".tsx")
                {
                    "js"
                } else {
                    "other"
                };

                // Use parent/filename when filename is ambiguous
                let filename = label.rsplit('/').next().unwrap_or(label).to_string();
                let display_label = if filename_counts.get(&filename).copied().unwrap_or(0) > 1 {
                    // Show last two path segments (e.g. "parser/mod.rs")
                    let parts: Vec<&str> = label.rsplitn(3, '/').collect();
                    if parts.len() >= 2 {
                        format!("{}/{}", parts[1], parts[0])
                    } else {
                        filename
                    }
                } else {
                    filename
                };

                GraphNode {
                    id: label.clone(),
                    label: display_label,
                    file_path: label.clone(),
                    node_type: node_type.to_string(),
                    complexity: analysis
                        .map(|a| a.complexity_label().to_string())
                        .unwrap_or_else(|| "Unknown".to_string()),
                    functions_count: analysis.map(|a| a.functions.len()).unwrap_or(0),
                    structs_count: analysis.map(|a| a.structs.len()).unwrap_or(0),
                }
            })
            .collect();

        let edges: Vec<GraphEdge> = self
            .graph
            .edge_references()
            .map(|edge| {
                let source = self.graph[edge.source()].clone();
                let target = self.graph[edge.target()].clone();
                let is_circular = cycles.iter().any(|(a, b)| {
                    (a == &source && b == &target) || (a == &target && b == &source)
                });
                GraphEdge {
                    source,
                    target,
                    is_circular,
                }
            })
            .collect();

        GraphData { nodes, edges }
    }

    /// Detect circular dependencies using DFS
    pub fn detect_cycles(&self) -> Vec<(String, String)> {
        let mut cycles = Vec::new();

        for edge in self.graph.edge_references() {
            let source = edge.source();
            let target = edge.target();

            if self
                .graph
                .edges_connecting(target, source)
                .next()
                .is_some()
            {
                let a = self.graph[source].clone();
                let b = self.graph[target].clone();

                if a < b {
                    cycles.push((a, b));
                }
            }
        }

        cycles
    }
}

/// Create a short label for a file path (relative to project root)
fn make_label(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Try to resolve an import string to a known node in the graph
fn resolve_import(import: &str, node_map: &HashMap<String, NodeIndex>) -> Option<String> {
    let module_name = if let Some(stripped) = import.strip_prefix("mod ") {
        stripped.to_string()
    } else {
        import.replace("::", "/")
    };

    // Convert Python dotted imports to path (e.g. "app.models" -> "app/models")
    let module_path = module_name.replace('.', "/");

    // Build candidates for all supported languages
    let mut candidates = vec![
        // Rust
        format!("src/{}.rs", module_path),
        format!("src/{}/mod.rs", module_path),
        format!("{}.rs", module_path),
        format!("{}/mod.rs", module_path),
        // Python
        format!("{}.py", module_path),
        format!("{}/__init__.py", module_path),
        format!("src/{}.py", module_path),
        // JS/TS
        format!("{}.js", module_path),
        format!("{}.ts", module_path),
        format!("{}.jsx", module_path),
        format!("{}.tsx", module_path),
        format!("{}/index.js", module_path),
        format!("{}/index.ts", module_path),
        format!("{}/index.tsx", module_path),
        format!("src/{}.js", module_path),
        format!("src/{}.ts", module_path),
        format!("src/{}.tsx", module_path),
    ];

    // For relative JS/TS imports (starting with ./), strip the prefix
    if let Some(stripped) = module_path.strip_prefix("./") {
        candidates.push(format!("{}.js", stripped));
        candidates.push(format!("{}.ts", stripped));
        candidates.push(format!("{}.jsx", stripped));
        candidates.push(format!("{}.tsx", stripped));
        candidates.push(format!("{}/index.js", stripped));
        candidates.push(format!("{}/index.ts", stripped));
        candidates.push(format!("{}/index.tsx", stripped));
        candidates.push(format!("src/{}.js", stripped));
        candidates.push(format!("src/{}.ts", stripped));
        candidates.push(format!("src/{}.tsx", stripped));
    }

    for candidate in &candidates {
        if node_map.contains_key(candidate) {
            return Some(candidate.clone());
        }
    }

    // Fuzzy match: compare the last path segment(s)
    for key in node_map.keys() {
        let key_stem = key
            .trim_end_matches(".rs")
            .trim_end_matches(".py")
            .trim_end_matches(".js")
            .trim_end_matches(".ts")
            .trim_end_matches(".jsx")
            .trim_end_matches(".tsx")
            .trim_end_matches("/mod")
            .trim_end_matches("/index")
            .trim_end_matches("/__init__")
            .rsplit('/')
            .next()
            .unwrap_or(key);
        let import_stem = module_path
            .rsplit('/')
            .next()
            .unwrap_or(&module_path);
        if !key_stem.is_empty() && !import_stem.is_empty() && key_stem == import_stem {
            return Some(key.clone());
        }
    }

    None
}
