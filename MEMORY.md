# DevStack Visualizer — Agent Memory

> This file is a persistent memory log for the AI coding agent. It tracks all changes, decisions, and context across sessions. Most recent entries appear first.

---

## Change Log

### 2026-03-02 (Session 5)

- **Fixed GraphView not rendering nodes — stale React state bug**
  - **Root cause:** `useNodesState(initialNodes)` and `useEdgesState(initialEdges)` only use their argument on the **first render** (like `useState`). When `graphData` arrived asynchronously after analysis, the `useMemo` recalculated new nodes/edges but the state hooks never picked up the updated values — resulting in empty/scattered nodes.
  - **Fix:** Added `useEffect` hooks that call `setNodes(initialNodes)` and `setEdges(initialEdges)` whenever the memoized values change, keeping React Flow state in sync with incoming graph data. Also imported `useEffect` from React.
  - **Affected files:** `src/components/GraphView.tsx`, `MEMORY.md`

- **Fixed ambiguous node labels (multiple "mod.rs" nodes indistinguishable)**
  - **Root cause:** `to_graph_data()` in `analyzer.rs` used only the filename (`label.rsplit('/').next()`) as the display label. Projects with multiple `mod.rs` files in different directories all showed "mod.rs".
  - **Fix:** Pre-count filename occurrences; when a filename appears more than once, display `parent/filename` (e.g. `parser/mod.rs`, `graph/mod.rs`) instead of just the filename.
  - **Affected files:** `src-tauri/src/analyzer.rs`, `MEMORY.md`

### 2026-03-02 (Session 4)

- **Added Python and JS/TS parsers — Fixed zero-results bug**
  - **Root cause:** `analyze_project` only parsed Rust files (`project_files.rust_files`). When opening a Python+React project, there were no `.rs` files, so all stats were 0.
  - **Created `python_parser.rs`:** Regex-based parser extracting `import`/`from` imports, `def` functions, and `class` declarations from Python files.
  - **Created `js_parser.rs`:** Regex-based parser extracting ES6 imports, `require()` calls, `function`/`const => ` function declarations, `class`/`interface`/`type` definitions from JS/TS files.
  - **Updated `parser/mod.rs`:** Added `pub mod python_parser;` and `pub mod js_parser;`.
  - **Updated `commands.rs`:** All 4 commands (`analyze_project`, `export_graph`, `get_complexity`, `detect_layers`) now parse Rust + Python + JS/TS files. `get_file_details` now handles `.py`, `.js`, `.ts`, `.jsx`, `.tsx` extensions.
  - **Updated `analyzer.rs` `resolve_import`:** Added candidates for Python (`.py`, `__init__.py`), JS/TS (`.js`, `.ts`, `.jsx`, `.tsx`, `index.js/ts/tsx`), and relative imports (`./`). Fuzzy matching now strips all language extensions.
  - **Build verified:** Compiles cleanly (3 pre-existing warnings, no errors).
  - **Affected files:** `src-tauri/src/parser/python_parser.rs` (new), `src-tauri/src/parser/js_parser.rs` (new), `src-tauri/src/parser/mod.rs`, `src-tauri/src/commands.rs`, `src-tauri/src/analyzer.rs`, `MEMORY.md`

### 2026-03-02 (Session 3)

- **Fixed Tauri app launch issues**
  - Added `beforeDevCommand` and `beforeBuildCommand` to `tauri.conf.json` so Tauri auto-starts the Vite dev server.
  - Fixed `plugins.dialog` config — removed invalid map `{ "open": true, "save": true }` (dialog plugin expects unit/empty config, not a map). This was causing a runtime panic: `PluginInitialization("dialog", "invalid type: map, expected unit")`.
  - Removed `plugins.shell` config similarly (replaced `plugins` with empty `{}`).
  - **Build verified:** TypeScript compiles cleanly (`tsc --noEmit`), Rust compiles with 3 minor warnings (unused code), app launches successfully with `npx tauri dev`.
  - **Affected files:** `src-tauri/tauri.conf.json`, `MEMORY.md`

### 2026-03-02 (Session 2)

- **Implemented Tauri v2 GUI — Full scaffolding, migration, and frontend**
  - **Phase 1 — Tauri Scaffolding:** Created `src-tauri/` directory with `Cargo.toml`, `build.rs`, `tauri.conf.json`, `capabilities/default.json`, and placeholder icon.
  - **Phase 1b — IPC Bridge:** Created `commands.rs` with 6 Tauri commands: `analyze_project`, `get_file_details`, `export_graph`, `detect_stack`, `get_complexity`, `detect_layers`. All registered in `lib.rs`.
  - **Rust Migration:** Moved all Rust modules (`scanner.rs`, `language_detector.rs`, `parser/`, `analyzer.rs`, `graph/`, `output.rs`) from `src/` to `src-tauri/src/`. Removed `cli.rs` (replaced by GUI). Added `GraphData`, `GraphNode`, `GraphEdge` structs to `analyzer.rs` for frontend consumption.
  - **Frontend Setup:** Created `package.json`, `tsconfig.json`, `vite.config.ts`, `tailwind.config.js`, `postcss.config.js`, `index.html`.
  - **Frontend Components:** Created all 6 React components:
    - `ProjectPicker.tsx` — Welcome screen with folder picker button
    - `Toolbar.tsx` — Top bar with Open, Refresh, Export, Settings buttons + project stats
    - `GraphView.tsx` — Interactive dependency graph using `@xyflow/react` (React Flow) with zoom, pan, clickable nodes, color-coded by language/complexity
    - `Sidebar.tsx` — File details panel (imports, functions, structs) + project overview
    - `SettingsPanel.tsx` — Modal with language filter, layout direction, theme options
    - `ExportDialog.tsx` — Modal for PNG/SVG/PDF export
  - **Hooks & Types:** Created `useTauriCommands.ts` hook (wraps all 6 IPC calls with loading/error state), `types/index.ts` (TypeScript interfaces matching Rust structs).
  - **Root Cargo.toml:** Converted to workspace config pointing to `src-tauri/`.
  - **Old CLI files removed** from `src/` (replaced by React/TS frontend).
  - **`.gitignore` updated** — added `/dist/`, `/src-tauri/target/`.
  - **Build verified:** Frontend builds (`vite build` → `dist/`), Rust backend compiles (`cargo check` passes).
  - **Affected files:** All `src-tauri/src/*.rs`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, `src-tauri/build.rs`, `src-tauri/capabilities/default.json`, `src/*.tsx`, `src/hooks/*.ts`, `src/types/*.ts`, `src/styles/*.css`, `package.json`, `tsconfig.json`, `vite.config.ts`, `tailwind.config.js`, `postcss.config.js`, `index.html`, `Cargo.toml`, `.gitignore`, `PROJECT_SPEC.md`, `MEMORY.md`

### 2026-03-02

- **Upgraded project from CLI-only to Tauri v2 Desktop GUI**
  - **Decision:** Replace CLI with a full GUI using Tauri v2 (Rust backend + React/TypeScript frontend). CLI mode (`clap` / `cli.rs`) will be removed entirely.
  - **GUI framework:** Tauri v2 chosen for lightweight native feel with web-based frontend.
  - **GUI features planned:** Interactive dependency graph (react-flow/d3), project folder picker, file detail sidebar, settings panel, export dialog (PNG/SVG/PDF), real-time file watching.
  - **Updated `.github/copilot-instructions.md`** — rewrote project summary, structure, and coding guidelines for Tauri + React. Added Tauri-specific guidelines (commands, IPC, serialization, events).
  - **Updated `PROJECT_SPEC.md`** — new architecture diagram, updated project structure (`src-tauri/` + `src/` frontend), added Tauri + frontend dependencies, replaced Phase 1 (CLI Setup) with Tauri scaffolding, added Phases 8–12 for frontend components and file watcher, updated status tracker.
  - **Existing Rust modules** (scanner, parser, analyzer, graph, etc.) are marked for migration into `src-tauri/src/`.
  - **Affected files:** `.github/copilot-instructions.md`, `PROJECT_SPEC.md`, `MEMORY.md`

### 2026-02-14

- **Created automation Python scripts** (`scripts/` directory)
  - `scripts/check_env.py` — Prerequisites checker & environment setup script. Validates Python ≥ 3.8, Rust/Cargo, C compiler (tree-sitter build dep), and Graphviz. Offers to install missing tools or provides download links. Builds release binary and adds `target/release` to user PATH.
  - `scripts/run.py` — Launcher script. Quick sanity check, auto-builds if sources changed, and launches devstack with user-provided arguments. Supports smart argument parsing (auto-prepends `analyze` subcommand).
  - **Affected files:** `scripts/check_env.py`, `scripts/run.py`, `MEMORY.md`

### 2026-02-13

- **Added agent memory system**
  - Created `MEMORY.md` (this file) at workspace root to serve as persistent AI agent memory.
  - Updated `.github/copilot-instructions.md` with the "Agent Memory" section — instructions for Copilot to read and update this file on every session and after every meaningful change.
  - **Affected files:** `.github/copilot-instructions.md`, `MEMORY.md`

---

## Key Architectural Decisions

| Date       | Decision | Rationale |
|------------|----------|-----------|
| 2026-03-02 | Upgrade from CLI to Tauri v2 GUI (React + TypeScript frontend) | Full desktop GUI experience with interactive graph visualization; CLI replaced entirely |
| 2026-03-02 | Use react-flow / d3.js for in-app graph rendering | Interactive, zoomable, pannable graphs — Graphviz only used for export |
| 2026-03-02 | Add `notify` crate for real-time file watching | Automatically re-analyze when project files change |
| 2026-02-13 | Use `MEMORY.md` at workspace root as agent memory | Simple, version-controlled, human-readable log that persists across sessions |

---

## Project Notes

- **Language:** Rust (backend) + TypeScript/React (frontend)
- **Framework:** Tauri v2
- **Spec:** See `PROJECT_SPEC.md` for full specification and status tracker.
- **Structure:** See `.github/copilot-instructions.md` for coding guidelines and project layout.
- **Migration needed:** Existing Rust code in `src/` must be moved to `src-tauri/src/`, with `cli.rs` removed and `commands.rs` added as the IPC bridge.
