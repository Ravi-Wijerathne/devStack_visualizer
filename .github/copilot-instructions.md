# Copilot Instructions — DevStack Visualizer

You are working on **DevStack Visualizer**, a Rust CLI tool that analyzes project codebases and generates architecture diagrams.

## Key Reference

Always read and follow the full project specification in `PROJECT_SPEC.md` at the workspace root before making any changes.

## Project Summary

- **Language:** Rust
- **Purpose:** CLI tool that scans a project directory, parses source files using Tree-Sitter, builds a dependency graph, and renders architecture diagrams via Graphviz.
- **Key Crates:** `clap` (CLI), `walkdir` (scanner), `tree-sitter` (AST parsing), `petgraph` (graph), `anyhow` (errors), `serde`/`serde_json` (serialization), `rayon` (parallelism).

## Project Structure

```
src/
├── main.rs                  # Entry point
├── cli.rs                   # CLI argument parsing (clap v4 derive)
├── scanner.rs               # File system scanner (walkdir)
├── language_detector.rs     # Smart language/stack detection
├── parser/
│   ├── mod.rs               # LanguageParser trait + module exports
│   ├── rust_parser.rs       # Rust AST parser
│   ├── python_parser.rs     # Python AST parser
│   └── js_parser.rs         # JS/TS AST parser
├── analyzer.rs              # Dependency analyzer (petgraph)
├── graph/
│   ├── mod.rs               # Graph module exports
│   ├── dot_generator.rs     # DOT format generator
│   └── renderer.rs          # Graphviz renderer (PNG/SVG/PDF)
└── output.rs                # Output formatting
```

## Coding Guidelines

1. **Always refer to `PROJECT_SPEC.md`** for detailed specs, structs, traits, and phase descriptions before implementing or modifying any module.
2. **Use `anyhow::Result`** for error handling throughout.
3. **Parser extensibility:** All language parsers must implement the `LanguageParser` trait.
4. **Skip directories:** `target/`, `node_modules/`, `.git/`, `__pycache__/`.
5. **Parallel parsing** with `rayon` where applicable.
6. **Update the Status Tracker** table in `PROJECT_SPEC.md` when completing a phase.
7. When adding a new language parser, follow the existing pattern in `rust_parser.rs`.
8. Use `petgraph::Graph<String, ()>` for the dependency graph (nodes = files/modules, edges = imports).
9. DOT output must be valid Graphviz syntax.
10. Support output formats: PNG, SVG, PDF via `std::process::Command` calling `dot`.

## Current Implementation Status

Check the **Status Tracker** section at the bottom of `PROJECT_SPEC.md` for the latest progress.

Always update the tracker when you complete a phase or add a new feature!

## Agent Memory

Copilot must maintain a persistent memory file at **`MEMORY.md`** in the workspace root. This file serves as a living log and context memory for the AI agent across sessions.

### Rules

1. **Read `MEMORY.md` at the start of every session** to recall prior context, decisions, and progress.
2. **Update `MEMORY.md` after every meaningful change** to the project. This includes but is not limited to:
   - New features implemented or modified
   - Bug fixes and what caused them
   - Architectural decisions and their rationale
   - Files created, renamed, or deleted
   - Dependencies added or removed
   - Refactors and why they were done
   - Any user preferences or recurring instructions observed
3. **Format entries as a chronological log** with timestamps, keeping the most recent entries at the top.
4. Each entry should include: date, a short summary of the change, and affected files/modules.
5. Keep the file concise — summarize older entries if it grows too long, but never delete important architectural decisions.
6. The memory file must **never be added to `.gitignore`** — it is part of the project knowledge base.
