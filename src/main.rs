mod analyzer;
mod cli;
mod graph;
mod language_detector;
mod output;
mod parser;
mod scanner;

use anyhow::{Context, Result};
use parser::rust_parser::RustParser;
use parser::LanguageParser;
use rayon::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let cli = cli::parse_args();

    match cli.command {
        cli::Commands::Analyze {
            path,
            output,
            languages: _languages,
            verbose,
            summary,
            graph,
            complexity,
            json,
            detect_layers,
        } => {
            run_analysis(
                &path,
                &output,
                verbose,
                summary,
                graph,
                complexity,
                json,
                detect_layers,
            )?;
        }
    }

    Ok(())
}

fn run_analysis(
    path: &Path,
    output_format: &str,
    verbose: bool,
    summary: bool,
    graph: bool,
    complexity: bool,
    json: bool,
    detect_layers: bool,
) -> Result<()> {
    // Resolve the project path
    let project_root = path
        .canonicalize()
        .with_context(|| format!("Invalid project path: {}", path.display()))?;

    if verbose {
        println!("Scanning project at: {}", project_root.display());
    }

    // Phase 2: Scan project files
    let project_files = scanner::scan_project(&project_root)?;

    if verbose {
        println!(
            "Found {} Rust, {} Python, {} JS/TS files",
            project_files.rust_files.len(),
            project_files.python_files.len(),
            project_files.js_files.len()
        );
    }

    if project_files.is_empty() {
        println!("No source files found in {}", project_root.display());
        return Ok(());
    }

    // Phase 3: Detect language/stack
    let stack = language_detector::detect_stack(&project_files, &project_root);

    if verbose {
        print!("{}", stack);
    }

    // Phase 4: Parse source files using tree-sitter (Rust only for now)
    let rust_parser = RustParser::new();
    let analyses: Vec<_> = project_files
        .rust_files
        .par_iter()
        .filter_map(|file| {
            if verbose {
                println!("  Parsing: {}", file.display());
            }
            match rust_parser.parse(file) {
                Ok(analysis) => Some(analysis),
                Err(e) => {
                    eprintln!("  Warning: Failed to parse {}: {}", file.display(), e);
                    None
                }
            }
        })
        .collect();

    // Phase 5: Build dependency graph
    let dep_graph = analyzer::DependencyGraph::build(&analyses, &project_root)?;
    let dep_summary = dep_graph.summary();

    // Output based on selected mode
    if json {
        let analysis_output = output::AnalysisOutput {
            stack: stack.clone(),
            files_parsed: analyses.len(),
            dependency_summary: dep_summary.clone(),
            file_analyses: analyses.clone(),
        };
        output::print_json(&analysis_output);
        return Ok(());
    }

    // Always show summary unless only graph mode is requested
    if summary || (!graph && !complexity) {
        output::print_summary(&stack, &dep_summary);
    }

    if complexity {
        output::print_complexity_report(&analyses);
    }

    // Phase 6 & 7: Generate graph output
    if graph || (!summary && !complexity) {
        let dot_content = if detect_layers {
            graph::dot_generator::generate_dot_with_layers(&dep_graph)?
        } else {
            graph::dot_generator::generate_dot(&dep_graph)?
        };

        let dot_path = project_root.join("architecture.dot");
        graph::dot_generator::save_dot(&dot_content, &dot_path)?;
        println!("DOT file saved: {}", dot_path.display());

        // Try to render with Graphviz
        if graph::renderer::is_graphviz_available() {
            let format = graph::renderer::OutputFormat::from_str(output_format)?;
            let output_file =
                project_root.join(format!("architecture.{}", format.extension()));
            graph::renderer::render_dot(&dot_path, &output_file, format)?;
            println!("Architecture diagram saved: {}", output_file.display());
        } else {
            println!(
                "\nNote: Graphviz is not installed. Only the DOT file was generated."
            );
            println!("Install Graphviz to render PNG/SVG/PDF: https://graphviz.org/download/");
        }
    }

    Ok(())
}
