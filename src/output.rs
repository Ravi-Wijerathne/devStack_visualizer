use crate::analyzer::DependencySummary;
use crate::language_detector::ProjectStack;
use crate::parser::FileAnalysis;
use serde::Serialize;

/// Complete analysis result for JSON output
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisOutput {
    pub stack: ProjectStack,
    pub files_parsed: usize,
    pub dependency_summary: DependencySummary,
    pub file_analyses: Vec<FileAnalysis>,
}

/// Print a human-readable summary of the analysis
pub fn print_summary(stack: &ProjectStack, dep_summary: &DependencySummary) {
    println!("\n========================================");
    println!("  DevStack Visualizer — Analysis Report");
    println!("========================================\n");
    print!("{}", stack);
    println!();
    println!("  Files Parsed:   {}", dep_summary.total_nodes);
    println!("  Dependencies:   {} edges", dep_summary.total_edges);

    if dep_summary.circular_dependencies.is_empty() {
        println!("  Circular Deps:  None detected");
    } else {
        println!(
            "  Circular Deps:  {} detected",
            dep_summary.circular_dependencies.len()
        );
        for (a, b) in &dep_summary.circular_dependencies {
            println!("    Warning: {} <-> {}", a, b);
        }
    }
    println!();
}

/// Print a file-level complexity report
pub fn print_complexity_report(analyses: &[FileAnalysis]) {
    println!("Complexity Report:");
    println!("------------------");
    for analysis in analyses {
        let label = analysis
            .file
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        println!(
            "  {} — Complexity: {} (functions: {}, structs: {})",
            label,
            analysis.complexity_label(),
            analysis.functions.len(),
            analysis.structs.len()
        );
    }
    println!();
}

/// Print analysis results as JSON
pub fn print_json(output: &AnalysisOutput) {
    match serde_json::to_string_pretty(output) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing to JSON: {}", e),
    }
}
