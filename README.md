# DevStack Visualizer

A Rust + Tauri v2 desktop application that analyzes project codebases and generates interactive architecture diagrams using Tree-Sitter and Graphviz.

## Features

- **Interactive dependency graph** — zoomable, pannable, clickable nodes (React Flow)
- **Multi-language support** — Rust, Python, JavaScript/TypeScript
- **Smart stack detection** — identifies backend, frontend, database, and containerization from marker files
- **File detail sidebar** — view imports, functions, structs, and complexity per file
- **Graph export** — PNG, SVG, PDF via Graphviz
- **Circular dependency detection** — highlights cycles in the dependency graph
- **Code complexity scoring** — per-file complexity metrics
- **Architecture layer detection** — recognizes MVC / Clean Architecture patterns

## Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) (18+)
- [Graphviz](https://graphviz.org/download/) (for PNG/SVG/PDF export)

## Setup

```bash
git clone https://github.com/Ravi-Wijerathne/devStack_visualizer.git
cd devStack_visualizer
npm install
```

## Development

```bash
npx tauri dev
```

## Build

```bash
npx tauri build
```

The installer/executable will be in `src-tauri/target/release/`.

## Helper Scripts

Python utility scripts are included in `scripts/` to simplify environment setup and launching:

### Environment Check & Setup

```bash
python scripts/check_env.py
```

Validates all prerequisites (Python, Node.js, Rust, C compiler, Graphviz), checks project files (`src-tauri/Cargo.toml`, `package.json`, `tauri.conf.json`), installs npm dependencies if missing, and optionally builds the production app.

### Quick Launcher

```bash
python scripts/run.py          # Start in development mode (default)
python scripts/run.py dev      # Same as above
python scripts/run.py build    # Build production installer/bundle
```

Performs prerequisite checks, auto-installs npm dependencies if needed, and launches the Tauri app in development or production mode.

## Usage

1. Launch the app (`npx tauri dev` or `python scripts/run.py`)
2. Click **Open Project** to select a project folder
3. The dependency graph renders automatically — zoom, pan, and click nodes
4. Click a node to view file details in the sidebar
5. Use **Export** to save the graph as PNG, SVG, or PDF

## Tech Stack

| Layer    | Technology |
|----------|------------|
| Backend  | Rust, Tauri v2, Tree-Sitter, petgraph, rayon |
| Frontend | React, TypeScript, React Flow, Tailwind CSS |
| Build    | Vite, Cargo |

## License

MIT
