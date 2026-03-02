use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Supported output formats for Graphviz rendering
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Png,
    Svg,
    Pdf,
}

impl OutputFormat {
    /// Parse format from a string
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "png" => Ok(OutputFormat::Png),
            "svg" => Ok(OutputFormat::Svg),
            "pdf" => Ok(OutputFormat::Pdf),
            other => anyhow::bail!("Unsupported output format: '{}'. Use png, svg, or pdf.", other),
        }
    }

    /// Get the Graphviz -T flag value
    fn as_flag(&self) -> &str {
        match self {
            OutputFormat::Png => "png",
            OutputFormat::Svg => "svg",
            OutputFormat::Pdf => "pdf",
        }
    }

    /// Get the file extension
    pub fn extension(&self) -> &str {
        self.as_flag()
    }
}

/// Render a DOT file to the specified format using Graphviz `dot` command
pub fn render_dot(dot_path: &Path, output_path: &Path, format: OutputFormat) -> Result<()> {
    let format_flag = format!("-T{}", format.as_flag());

    let status = Command::new("dot")
        .args([
            &format_flag,
            &dot_path.to_string_lossy().to_string(),
            "-o",
            &output_path.to_string_lossy().to_string(),
        ])
        .status()
        .context(
            "Failed to run 'dot' command. Is Graphviz installed?\n\
             Install it from: https://graphviz.org/download/",
        )?;

    if !status.success() {
        anyhow::bail!(
            "Graphviz 'dot' command failed with exit code: {:?}",
            status.code()
        );
    }

    Ok(())
}

/// Check if Graphviz is available on the system
pub fn is_graphviz_available() -> bool {
    Command::new("dot")
        .arg("-V")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}
