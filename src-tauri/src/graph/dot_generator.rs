use crate::analyzer::DependencyGraph;
use anyhow::Result;
use petgraph::visit::EdgeRef;
use std::fmt::Write;
use std::path::Path;

/// Generate a DOT format string from the dependency graph
pub fn generate_dot(dep_graph: &DependencyGraph) -> Result<String> {
    let mut dot = String::new();

    writeln!(dot, "digraph DevStackArchitecture {{")?;
    writeln!(dot, "    rankdir=LR;")?;
    writeln!(dot, "    node [shape=box, style=filled, fillcolor=\"#e8f4fd\", fontname=\"Helvetica\"];")?;
    writeln!(dot, "    edge [color=\"#4a90d9\"];")?;
    writeln!(dot)?;

    for node_idx in dep_graph.graph.node_indices() {
        let label = &dep_graph.graph[node_idx];
        let escaped = label.replace('"', "\\\"");
        writeln!(dot, "    \"{}\" [label=\"{}\"];", escaped, escaped)?;
    }

    writeln!(dot)?;

    for edge in dep_graph.graph.edge_references() {
        let source = &dep_graph.graph[edge.source()];
        let target = &dep_graph.graph[edge.target()];
        writeln!(
            dot,
            "    \"{}\" -> \"{}\";",
            source.replace('"', "\\\""),
            target.replace('"', "\\\"")
        )?;
    }

    writeln!(dot, "}}")?;

    Ok(dot)
}

/// Generate DOT with layer subgraphs (MVC / Clean Architecture detection)
pub fn generate_dot_with_layers(dep_graph: &DependencyGraph) -> Result<String> {
    let mut dot = String::new();

    writeln!(dot, "digraph DevStackArchitecture {{")?;
    writeln!(dot, "    rankdir=LR;")?;
    writeln!(dot, "    node [shape=box, style=filled, fontname=\"Helvetica\"];")?;
    writeln!(dot, "    edge [color=\"#4a90d9\"];")?;
    writeln!(dot)?;

    let mut controllers = Vec::new();
    let mut services = Vec::new();
    let mut models = Vec::new();
    let mut others = Vec::new();

    for node_idx in dep_graph.graph.node_indices() {
        let label = dep_graph.graph[node_idx].to_lowercase();
        if label.contains("controller") || label.contains("handler") || label.contains("route") {
            controllers.push(node_idx);
        } else if label.contains("service") || label.contains("usecase") {
            services.push(node_idx);
        } else if label.contains("model") || label.contains("entity") || label.contains("schema") {
            models.push(node_idx);
        } else {
            others.push(node_idx);
        }
    }

    if !controllers.is_empty() {
        writeln!(dot, "    subgraph cluster_controller {{")?;
        writeln!(dot, "        label=\"Controllers\";")?;
        writeln!(dot, "        style=filled;")?;
        writeln!(dot, "        color=\"#d4edda\";")?;
        for idx in &controllers {
            let label = &dep_graph.graph[*idx];
            writeln!(dot, "        \"{}\" [fillcolor=\"#c3e6cb\"];", label.replace('"', "\\\""))?;
        }
        writeln!(dot, "    }}")?;
    }

    if !services.is_empty() {
        writeln!(dot, "    subgraph cluster_service {{")?;
        writeln!(dot, "        label=\"Services\";")?;
        writeln!(dot, "        style=filled;")?;
        writeln!(dot, "        color=\"#cce5ff\";")?;
        for idx in &services {
            let label = &dep_graph.graph[*idx];
            writeln!(dot, "        \"{}\" [fillcolor=\"#b8daff\"];", label.replace('"', "\\\""))?;
        }
        writeln!(dot, "    }}")?;
    }

    if !models.is_empty() {
        writeln!(dot, "    subgraph cluster_model {{")?;
        writeln!(dot, "        label=\"Models\";")?;
        writeln!(dot, "        style=filled;")?;
        writeln!(dot, "        color=\"#fff3cd\";")?;
        for idx in &models {
            let label = &dep_graph.graph[*idx];
            writeln!(dot, "        \"{}\" [fillcolor=\"#ffeeba\"];", label.replace('"', "\\\""))?;
        }
        writeln!(dot, "    }}")?;
    }

    for idx in &others {
        let label = &dep_graph.graph[*idx];
        writeln!(dot, "    \"{}\" [fillcolor=\"#e8f4fd\"];", label.replace('"', "\\\""))?;
    }

    writeln!(dot)?;

    for edge in dep_graph.graph.edge_references() {
        let source = &dep_graph.graph[edge.source()];
        let target = &dep_graph.graph[edge.target()];
        writeln!(
            dot,
            "    \"{}\" -> \"{}\";",
            source.replace('"', "\\\""),
            target.replace('"', "\\\"")
        )?;
    }

    writeln!(dot, "}}")?;

    Ok(dot)
}

/// Save DOT content to a file
pub fn save_dot(dot_content: &str, output_path: &Path) -> Result<()> {
    std::fs::write(output_path, dot_content)?;
    Ok(())
}
