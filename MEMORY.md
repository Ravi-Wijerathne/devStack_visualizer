# DevStack Visualizer — Agent Memory

> This file is a persistent memory log for the AI coding agent. It tracks all changes, decisions, and context across sessions. Most recent entries appear first.

---

## Change Log

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
