# DevStack Visualizer — Agent Memory

> This file is a persistent memory log for the AI coding agent. It tracks all changes, decisions, and context across sessions. Most recent entries appear first.

---

## Change Log

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
