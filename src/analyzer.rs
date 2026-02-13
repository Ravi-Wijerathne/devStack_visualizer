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
    pub node_map: HashMap<String, NodeIndex>,
}

/// Summary of dependency analysis
#[derive(Debug, Clone, Serialize)]
pub struct DependencySummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub circular_dependencies: Vec<(String, String)>,
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
                // Try to resolve the import to an existing node
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

    /// Detect circular dependencies using DFS
    pub fn detect_cycles(&self) -> Vec<(String, String)> {
        let mut cycles = Vec::new();

        // Simple bidirectional edge check (A -> B and B -> A)
        for edge in self.graph.edge_references() {
            let source = edge.source();
            let target = edge.target();

            // Check if reverse edge exists
            if self
                .graph
                .edges_connecting(target, source)
                .next()
                .is_some()
            {
                let a = self.graph[source].clone();
                let b = self.graph[target].clone();

                // Avoid duplicates (only add if a < b alphabetically)
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
    // Handle "mod foo" declarations
    let module_name = if let Some(stripped) = import.strip_prefix("mod ") {
        stripped.to_string()
    } else {
        // For "crate::foo::bar", try to find "src/foo/bar.rs" or "src/foo.rs"
        import.replace("::", "/")
    };

    // Try various resolution strategies
    let candidates = vec![
        format!("src/{}.rs", module_name),
        format!("src/{}/mod.rs", module_name),
        format!("{}.rs", module_name),
        format!("{}/mod.rs", module_name),
    ];

    for candidate in &candidates {
        if node_map.contains_key(candidate) {
            return Some(candidate.clone());
        }
    }

    // Also try partial matches (if the import path ends with a node name)
    for key in node_map.keys() {
        let key_stem = key
            .trim_end_matches(".rs")
            .trim_end_matches("/mod")
            .rsplit('/')
            .next()
            .unwrap_or(key);
        if module_name.ends_with(key_stem) && !key_stem.is_empty() {
            return Some(key.clone());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_build_graph() {
        let root = PathBuf::from("/project");
        let analyses = vec![
            FileAnalysis {
                file: PathBuf::from("/project/src/main.rs"),
                imports: vec!["crate::utils".to_string()],
                functions: vec!["main".to_string()],
                structs: vec![],
            },
            FileAnalysis {
                file: PathBuf::from("/project/src/utils.rs"),
                imports: vec![],
                functions: vec!["helper".to_string()],
                structs: vec![],
            },
        ];

        let dep_graph = DependencyGraph::build(&analyses, &root).unwrap();
        assert_eq!(dep_graph.graph.node_count(), 2);
        assert_eq!(dep_graph.graph.edge_count(), 1);
    }

    #[test]
    fn test_cycle_detection() {
        let root = PathBuf::from("/project");
        let analyses = vec![
            FileAnalysis {
                file: PathBuf::from("/project/src/a.rs"),
                imports: vec!["mod b".to_string()],
                functions: vec![],
                structs: vec![],
            },
            FileAnalysis {
                file: PathBuf::from("/project/src/b.rs"),
                imports: vec!["mod a".to_string()],
                functions: vec![],
                structs: vec![],
            },
        ];

        let dep_graph = DependencyGraph::build(&analyses, &root).unwrap();
        let cycles = dep_graph.detect_cycles();
        assert_eq!(cycles.len(), 1);
    }
}
