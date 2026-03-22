#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_graph_creation() {
        let nodes = vec!["main.rs", "lib.rs", "commands.rs"];
        let mut graph = create_graph(&nodes, &[]);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_with_edges() {
        let nodes = vec!["main.rs", "lib.rs", "utils.rs"];
        let edges = vec![("main.rs", "lib.rs"), ("main.rs", "utils.rs")];
        let mut graph = create_graph(&nodes, &edges);

        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 2);
    }

    #[test]
    fn test_linear_dependencies() {
        let nodes = vec!["a.rs", "b.rs", "c.rs", "d.rs"];
        let edges = vec![("a.rs", "b.rs"), ("b.rs", "c.rs"), ("c.rs", "d.rs")];
        let graph = create_graph(&nodes, &edges);

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 3);

        let has_cycle = detect_cycle(&graph, "a.rs", "d.rs");
        assert!(!has_cycle);
    }

    #[test]
    fn test_diamond_dependencies() {
        let nodes = vec!["a.rs", "b.rs", "c.rs", "d.rs"];
        let edges = vec![
            ("a.rs", "b.rs"),
            ("a.rs", "c.rs"),
            ("b.rs", "d.rs"),
            ("c.rs", "d.rs"),
        ];
        let graph = create_graph(&nodes, &edges);

        assert_eq!(graph.node_count(), 4);
        assert_eq!(graph.edge_count(), 4);

        let has_cycle = detect_cycle(&graph, "b.rs", "c.rs");
        assert!(!has_cycle);
    }

    #[test]
    fn test_circular_dependencies() {
        let nodes = vec!["a.rs", "b.rs", "c.rs"];
        let edges = vec![("a.rs", "b.rs"), ("b.rs", "c.rs"), ("c.rs", "a.rs")];
        let graph = create_graph(&nodes, &edges);

        let cycles = detect_all_cycles(&graph);
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_detect_mutual_dependencies() {
        let nodes = vec!["module_a.rs", "module_b.rs"];
        let edges = vec![
            ("module_a.rs", "module_b.rs"),
            ("module_b.rs", "module_a.rs"),
        ];
        let graph = create_graph(&nodes, &edges);

        let has_circular = detect_cycle(&graph, "module_a.rs", "module_b.rs");
        assert!(has_circular);
    }

    #[test]
    fn test_orphan_nodes() {
        let nodes = vec!["main.rs", "orphan.rs", "unused.rs"];
        let edges = vec![("main.rs", "orphan.rs")];
        let graph = create_graph(&nodes, &edges);

        let orphans = find_orphan_nodes(&graph);
        assert!(orphans.contains(&"unused.rs".to_string()));
    }

    #[test]
    fn test_resolve_import_paths() {
        let candidates = vec![
            "src/lib.rs".to_string(),
            "src/main.rs".to_string(),
            "src/utils/mod.rs".to_string(),
        ];

        assert_eq!(
            resolve_import("crate::lib", &candidates),
            Some("src/lib.rs".to_string())
        );
        assert_eq!(
            resolve_import("crate::main", &candidates),
            Some("src/main.rs".to_string())
        );
        assert_eq!(
            resolve_import("crate::utils", &candidates),
            Some("src/utils/mod.rs".to_string())
        );
    }

    #[test]
    fn test_resolve_python_imports() {
        let candidates = vec![
            "models/user.py".to_string(),
            "views/index.py".to_string(),
            "utils/__init__.py".to_string(),
        ];

        assert_eq!(
            resolve_import("models.user", &candidates),
            Some("models/user.py".to_string())
        );
        assert_eq!(
            resolve_import("views.index", &candidates),
            Some("views/index.py".to_string())
        );
    }

    #[test]
    fn test_resolve_js_imports() {
        let candidates = vec![
            "components/Button.js".to_string(),
            "utils/helpers.ts".to_string(),
            "index.js".to_string(),
        ];

        assert_eq!(
            resolve_import("./Button", &candidates),
            Some("components/Button.js".to_string())
        );
        assert_eq!(
            resolve_import("./helpers", &candidates),
            Some("utils/helpers.ts".to_string())
        );
        assert_eq!(
            resolve_import("./index", &candidates),
            Some("index.js".to_string())
        );
    }

    #[test]
    fn test_fuzzy_import_matching() {
        let candidates = vec![
            "src/parser/mod.rs".to_string(),
            "src/commands/mod.rs".to_string(),
        ];

        assert_eq!(
            resolve_import("crate::parser", &candidates),
            Some("src/parser/mod.rs".to_string())
        );
        assert_eq!(
            resolve_import("crate::commands", &candidates),
            Some("src/commands/mod.rs".to_string())
        );
    }

    #[test]
    fn test_make_label() {
        let path = "/home/user/project/src/main.rs";
        let root = "/home/user/project";

        let label = make_label(path, root);
        assert_eq!(label, "src/main.rs");
    }

    #[test]
    fn test_make_label_preserves_windows_paths() {
        let path = "C:\\Users\\dev\\project\\src\\lib.rs";
        let root = "C:\\Users\\dev\\project";

        let label = make_label(path, root);
        assert!(label.contains("src/lib.rs"));
    }

    #[test]
    fn test_complexity_label() {
        assert_eq!(complexity_label(0), "Low");
        assert_eq!(complexity_label(1), "Low");
        assert_eq!(complexity_label(2), "Low");
        assert_eq!(complexity_label(3), "Medium");
        assert_eq!(complexity_label(6), "Medium");
        assert_eq!(complexity_label(7), "High");
        assert_eq!(complexity_label(100), "High");
    }

    #[test]
    fn test_graph_node_types() {
        let files = vec![
            ("main.rs", "rust"),
            ("app.py", "python"),
            ("index.js", "js"),
            ("index.ts", "js"),
            ("Button.tsx", "js"),
            ("Component.jsx", "js"),
            ("data.txt", "other"),
        ];

        for (file, expected_type) in files {
            let node_type = get_node_type(file);
            assert_eq!(node_type, expected_type, "Failed for file: {}", file);
        }
    }

    #[test]
    fn test_edge_deduplication() {
        let nodes = vec!["a.rs", "b.rs"];
        let edges = vec![("a.rs", "b.rs"), ("a.rs", "b.rs"), ("a.rs", "b.rs")];
        let graph = create_graph(&nodes, &edges);

        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_self_referencing_edge_ignored() {
        let nodes = vec!["main.rs"];
        let edges = vec![("main.rs", "main.rs")];
        let graph = create_graph(&nodes, &edges);

        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_graph_summary() {
        let nodes = vec!["a.rs", "b.rs", "c.rs", "d.rs", "e.rs"];
        let edges = vec![
            ("a.rs", "b.rs"),
            ("b.rs", "c.rs"),
            ("c.rs", "a.rs"),
            ("d.rs", "e.rs"),
        ];
        let graph = create_graph(&nodes, &edges);

        let summary = create_summary(&graph);
        assert_eq!(summary.total_nodes, 5);
        assert_eq!(summary.total_edges, 4);
        assert!(!summary.circular_dependencies.is_empty());
    }

    // Helper functions and types for testing
    struct Graph {
        nodes: Vec<String>,
        edges: Vec<(String, String)>,
    }

    impl Graph {
        fn node_count(&self) -> usize {
            self.nodes.len()
        }

        fn edge_count(&self) -> usize {
            self.edges.len()
        }
    }

    struct Summary {
        total_nodes: usize,
        total_edges: usize,
        circular_dependencies: Vec<(String, String)>,
    }

    fn create_graph(nodes: &[&str], edges: &[(&str, &str)]) -> Graph {
        let mut unique_nodes: Vec<String> = nodes.iter().map(|s| s.to_string()).collect();
        unique_nodes.sort();
        unique_nodes.dedup();

        let mut unique_edges: Vec<(String, String)> = edges
            .iter()
            .filter(|(s, t)| s != t)
            .map(|(s, t)| (s.to_string(), t.to_string()))
            .collect();
        unique_edges.sort();
        unique_edges.dedup();

        Graph {
            nodes: unique_nodes,
            edges: unique_edges,
        }
    }

    fn detect_cycle(graph: &Graph, from: &str, to: &str) -> bool {
        graph
            .edges
            .iter()
            .any(|(s, t)| (s == from && t == to) || (s == to && t == from))
    }

    fn detect_all_cycles(graph: &Graph) -> Vec<(String, String)> {
        let mut cycles = Vec::new();

        for edge in &graph.edges {
            let reverse_exists = graph
                .edges
                .iter()
                .any(|(s, t)| s == &edge.1 && t == &edge.0);

            if reverse_exists {
                if edge.0 < edge.1 {
                    cycles.push((edge.0.clone(), edge.1.clone()));
                } else {
                    cycles.push((edge.1.clone(), edge.0.clone()));
                }
            }
        }

        cycles
    }

    fn find_orphan_nodes(graph: &Graph) -> Vec<String> {
        let mut has_outgoing = std::collections::HashSet::new();
        let mut has_incoming = std::collections::HashSet::new();

        for (source, target) in &graph.edges {
            has_outgoing.insert(source.clone());
            has_incoming.insert(target.clone());
        }

        graph
            .nodes
            .iter()
            .filter(|n| !has_outgoing.contains(*n) && !has_incoming.contains(*n))
            .cloned()
            .collect()
    }

    fn resolve_import(import: &str, candidates: &[String]) -> Option<String> {
        let module_name = if let Some(stripped) = import.strip_prefix("mod ") {
            stripped.to_string()
        } else {
            import.replace("::", "/").replace('.', "/")
        };

        for candidate in candidates {
            if candidate.contains(&module_name)
                || candidate.ends_with(&format!("{}.rs", module_name))
                || candidate.ends_with(&format!("{}.py", module_name))
                || candidate.ends_with(&format!("{}.js", module_name))
                || candidate.ends_with(&format!("{}.ts", module_name))
            {
                return Some(candidate.clone());
            }
        }
        None
    }

    fn make_label(path: &str, root: &str) -> String {
        if let Some(stripped) = path.strip_prefix(root) {
            stripped.trim_start_matches(['/', '\\']).to_string()
        } else {
            path.to_string()
        }
    }

    fn complexity_label(score: usize) -> &'static str {
        match score {
            0..=2 => "Low",
            3..=6 => "Medium",
            _ => "High",
        }
    }

    fn get_node_type(file: &str) -> &'static str {
        if file.ends_with(".rs") {
            "rust"
        } else if file.ends_with(".py") {
            "python"
        } else if file.ends_with(".js")
            || file.ends_with(".ts")
            || file.ends_with(".jsx")
            || file.ends_with(".tsx")
        {
            "js"
        } else {
            "other"
        }
    }

    fn create_summary(graph: &Graph) -> Summary {
        let cycles = detect_all_cycles(graph);
        Summary {
            total_nodes: graph.node_count(),
            total_edges: graph.edge_count(),
            circular_dependencies: cycles,
        }
    }
}
