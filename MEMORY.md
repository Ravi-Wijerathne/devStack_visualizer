# DevStack Visualizer — Agent Memory

> This file is a persistent memory log for the AI coding agent. It tracks all changes, decisions, and context across sessions. Most recent entries appear first.

---

## Change Log

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
| 2026-02-13 | Use `MEMORY.md` at workspace root as agent memory | Simple, version-controlled, human-readable log that persists across sessions |

---

## Project Notes

- **Language:** Rust
- **Spec:** See `PROJECT_SPEC.md` for full specification and status tracker.
- **Structure:** See `.github/copilot-instructions.md` for coding guidelines and project layout.
