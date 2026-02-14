# DevStack Visualizer

A Rust CLI tool that analyzes project codebases and generates architecture diagrams using Tree-Sitter and Graphviz.

## Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Graphviz](https://graphviz.org/download/) (for graph rendering)

## Setup

```bash
git clone https://github.com/Ravi-Wijerathne/devStack_visualizer.git
cd devStack_visualizer
cargo build --release
```

The binary will be at `target/release/devstack`.

## Usage

```bash
devstack analyze <PATH> [OPTIONS]
```

Or run directly with cargo:

```bash
cargo run -- analyze <PATH> [OPTIONS]
```

### Options

| Flag | Description |
|------|-------------|
| `-o, --output <FORMAT>` | Output format: `png`, `svg`, `pdf` (default: `png`) |
| `-l, --languages <LIST>` | Filter by languages: `rust,python,js` |
| `-v, --verbose` | Enable verbose logging |
| `--summary` | Show stack summary only |
| `--graph` | Generate architecture graph |
| `--complexity` | Show complexity report |
| `--json` | JSON output |
| `--detect-layers` | Detect MVC / Clean Architecture layers |

### Examples

```bash
# Analyze current directory
devstack analyze .

# Verbose output with graph generation
devstack analyze . --verbose --graph

# Filter by language, output as SVG
devstack analyze ./my-project -o svg -l rust

# JSON summary with layer detection
devstack analyze . --json --summary --detect-layers
```

## License

MIT
