use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// DevStack Visualizer — Analyze project codebases and generate architecture diagrams
#[derive(Parser, Debug)]
#[command(name = "devstack", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Analyze a project directory
    Analyze {
        /// Path to the project to analyze
        #[arg(value_name = "PATH")]
        path: PathBuf,

        /// Output format: png, svg, pdf
        #[arg(short, long, default_value = "png")]
        output: String,

        /// Filter by languages (comma-separated): rust, python, js
        #[arg(short, long, value_delimiter = ',')]
        languages: Option<Vec<String>>,

        /// Enable verbose logging
        #[arg(short, long, default_value_t = false)]
        verbose: bool,

        /// Show stack summary only
        #[arg(long, default_value_t = false)]
        summary: bool,

        /// Generate architecture graph output
        #[arg(long, default_value_t = false)]
        graph: bool,

        /// Show complexity report
        #[arg(long, default_value_t = false)]
        complexity: bool,

        /// Machine-readable JSON output
        #[arg(long, default_value_t = false)]
        json: bool,

        /// Detect MVC / Clean Architecture layers
        #[arg(long, default_value_t = false)]
        detect_layers: bool,
    },
}

/// Parse CLI arguments and return the Cli struct
pub fn parse_args() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_analyze() {
        let cli = Cli::parse_from(["devstack", "analyze", "./my-project"]);
        match cli.command {
            Commands::Analyze { ref path, .. } => {
                assert_eq!(path, &PathBuf::from("./my-project"));
            }
        }
    }

    #[test]
    fn test_cli_parse_with_options() {
        let cli = Cli::parse_from([
            "devstack",
            "analyze",
            "./project",
            "--output",
            "svg",
            "--languages",
            "rust,python",
            "--verbose",
            "--summary",
            "--graph",
            "--complexity",
            "--json",
            "--detect-layers",
        ]);
        match cli.command {
            Commands::Analyze {
                ref path,
                ref output,
                ref languages,
                verbose,
                summary,
                graph,
                complexity,
                json,
                detect_layers,
            } => {
                assert_eq!(path, &PathBuf::from("./project"));
                assert_eq!(output, "svg");
                assert_eq!(
                    languages.as_ref().unwrap(),
                    &vec!["rust".to_string(), "python".to_string()]
                );
                assert!(verbose);
                assert!(summary);
                assert!(graph);
                assert!(complexity);
                assert!(json);
                assert!(detect_layers);
            }
        }
    }
}
