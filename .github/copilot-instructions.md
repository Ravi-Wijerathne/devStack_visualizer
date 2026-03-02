# Copilot Instructions — DevStack Visualizer

You are working on **DevStack Visualizer**, a Rust + Tauri desktop GUI application that analyzes project codebases and generates interactive architecture diagrams.

## Key Reference

Always read and follow the full project specification in `PROJECT_SPEC.md` at the workspace root before making any changes.

## Project Summary

- **Language:** Rust (backend / core logic) + TypeScript/React (frontend UI)
- **Framework:** Tauri v2 (Rust backend + system webview frontend)
- **Purpose:** Desktop GUI application that scans a project directory, parses source files using Tree-Sitter, builds a dependency graph, and renders interactive architecture diagrams in-app. Previously a CLI-only tool, now upgraded to a full GUI experience.
- **Key Crates (Rust/Backend):** `tauri` (desktop app framework), `walkdir` (scanner), `tree-sitter` (AST parsing), `petgraph` (graph), `anyhow` (errors), `serde`/`serde_json` (serialization), `rayon` (parallelism).
- **Key Packages (Frontend):** `react`, `typescript`, `d3` or `react-flow` (interactive graph rendering), `@tauri-apps/api` (Rust ↔ JS bridge).

## Project Structure

```
src-tauri/
├── src/
│   ├── main.rs                  # Tauri entry point
│   ├── commands.rs              # Tauri command handlers (IPC bridge)
│   ├── scanner.rs               # File system scanner (walkdir)
│   ├── language_detector.rs     # Smart language/stack detection
│   ├── parser/
│   │   ├── mod.rs               # LanguageParser trait + module exports
│   │   ├── rust_parser.rs       # Rust AST parser
│   │   ├── python_parser.rs     # Python AST parser
│   │   └── js_parser.rs         # JS/TS AST parser
│   ├── analyzer.rs              # Dependency analyzer (petgraph)
│   ├── graph/
│   │   ├── mod.rs               # Graph module exports
│   │   ├── dot_generator.rs     # DOT format generator
│   │   └── renderer.rs          # Graphviz renderer (PNG/SVG/PDF export)
│   └── output.rs                # Output formatting / serialization
├── Cargo.toml                   # Rust dependencies
└── tauri.conf.json              # Tauri app configuration

src/                             # Frontend (React + TypeScript)
├── App.tsx                      # Main application component
├── main.tsx                     # React entry point
├── components/
│   ├── GraphView.tsx            # Interactive dependency graph (zoomable, pannable)
│   ├── Sidebar.tsx              # File detail sidebar (imports, structs, functions)
│   ├── Toolbar.tsx              # Actions: open project, export, settings
│   ├── ProjectPicker.tsx        # Folder picker dialog
│   ├── SettingsPanel.tsx        # Preferences & configuration
│   └── ExportDialog.tsx         # Export graph as PNG/SVG/PDF
├── hooks/
│   └── useTauriCommands.ts      # React hooks for Tauri IPC calls
├── styles/
│   └── ...                      # CSS / Tailwind styles
└── types/
    └── index.ts                 # Shared TypeScript types

package.json
tsconfig.json
vite.config.ts                   # Vite bundler config for frontend
```

## Coding Guidelines

### General

1. **Always refer to `PROJECT_SPEC.md`** for detailed specs, structs, traits, and phase descriptions before implementing or modifying any module.
2. **Use `anyhow::Result`** for error handling in all Rust code.
3. **Parser extensibility:** All language parsers must implement the `LanguageParser` trait.
4. **Skip directories:** `target/`, `node_modules/`, `.git/`, `__pycache__/`, `dist/`.
5. **Parallel parsing** with `rayon` where applicable.
6. **Update the Status Tracker** table in `PROJECT_SPEC.md` when completing a phase.
7. When adding a new language parser, follow the existing pattern in `rust_parser.rs`.
8. Use `petgraph::Graph<String, ()>` for the dependency graph (nodes = files/modules, edges = imports).
9. DOT output must be valid Graphviz syntax.
10. Support export formats: PNG, SVG, PDF.

### Tauri / GUI Specific

11. **Tauri commands** are the IPC bridge between frontend and backend. Define them in `commands.rs` using `#[tauri::command]` and register in `main.rs`.
12. All data passed between Rust and the frontend must be **serializable** (`serde::Serialize` / `serde::Deserialize`).
13. Use `@tauri-apps/api` on the frontend to invoke Rust commands — never call Rust directly.
14. **Frontend state management:** Keep analysis results in React state; the Rust backend is stateless per-request.
15. **File dialogs:** Use Tauri's native dialog API (`tauri::api::dialog`) for the project folder picker.
16. **Real-time analysis:** Use Tauri events or file watcher (`notify` crate) to push updates when project files change.
17. **Graph rendering:** Render the dependency graph interactively in the browser (d3.js / react-flow), not via Graphviz. Graphviz/DOT is only used for export.
18. Keep the frontend responsive — run heavy analysis work in Rust (backend) and stream results via Tauri commands/events.

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
