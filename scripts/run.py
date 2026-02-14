#!/usr/bin/env python3
"""
DevStack Visualizer — Launcher Script
=======================================
Quickly builds (if needed) and launches the devstack CLI tool.

Usage:
    python scripts/run.py <path-to-project> [OPTIONS]

Examples:
    python scripts/run.py .                         # Analyse current project
    python scripts/run.py ../my-app --output svg    # SVG output
    python scripts/run.py . --graph --verbose       # Verbose graph mode
    python scripts/run.py . --help                  # Show devstack help

If the release binary doesn't exist yet, it builds it automatically.
"""

from __future__ import annotations

import os
import platform
import subprocess
import sys
from pathlib import Path

# ── Colours ─────────────────────────────────────────────────────────
GREEN = "\033[92m"
YELLOW = "\033[93m"
RED = "\033[91m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"


def info(msg: str) -> None:
    print(f"  {CYAN}ℹ{RESET}  {msg}")


def ok(msg: str) -> None:
    print(f"  {GREEN}✔{RESET}  {msg}")


def fail(msg: str) -> None:
    print(f"  {RED}✘{RESET}  {msg}")


def warn(msg: str) -> None:
    print(f"  {YELLOW}⚠{RESET}  {msg}")


# ── Paths ───────────────────────────────────────────────────────────
PROJECT_ROOT = Path(__file__).resolve().parent.parent
CARGO_TOML = PROJECT_ROOT / "Cargo.toml"
IS_WINDOWS = platform.system() == "Windows"
EXE_NAME = "devstack.exe" if IS_WINDOWS else "devstack"
RELEASE_BIN = PROJECT_ROOT / "target" / "release" / EXE_NAME
DEBUG_BIN = PROJECT_ROOT / "target" / "debug" / EXE_NAME


# ── Quick prerequisite sanity check ─────────────────────────────────

def quick_check() -> bool:
    """Fast sanity check — just verify cargo and dot are reachable."""
    errors: list[str] = []

    # Cargo
    try:
        subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            timeout=10,
        )
    except FileNotFoundError:
        errors.append("Rust/Cargo is not installed or not on PATH.")
    except Exception as e:
        errors.append(f"Error checking Cargo: {e}")

    # Graphviz (optional but warn)
    try:
        subprocess.run(
            ["dot", "-V"],
            capture_output=True,
            timeout=10,
        )
    except FileNotFoundError:
        warn("Graphviz (dot) not found — graph rendering will produce DOT files only.")
    except Exception:
        pass

    if errors:
        for e in errors:
            fail(e)
        info("Run 'python scripts/check_env.py' for detailed diagnostics & setup.")
        return False
    return True


# ── Build ───────────────────────────────────────────────────────────

def find_or_build_binary(release: bool = True) -> Path | None:
    """Return the path to the devstack binary, building first if necessary."""
    target_bin = RELEASE_BIN if release else DEBUG_BIN
    profile = "release" if release else "dev"

    if target_bin.is_file():
        # Check if sources are newer than binary (simple mtime check)
        src_dir = PROJECT_ROOT / "src"
        newest_src = max(
            (f.stat().st_mtime for f in src_dir.rglob("*") if f.is_file()),
            default=0,
        )
        if target_bin.stat().st_mtime >= newest_src:
            ok(f"Binary up-to-date: {target_bin}")
            return target_bin
        else:
            info("Source files changed — rebuilding...")

    info(f"Building devstack ({profile} profile)...")
    build_cmd = ["cargo", "build"]
    if release:
        build_cmd.append("--release")

    result = subprocess.run(build_cmd, cwd=str(PROJECT_ROOT))
    if result.returncode != 0:
        fail("Build failed. Fix compilation errors and try again.")
        info("Tip: run 'python scripts/check_env.py' to verify your environment.")
        return None

    if target_bin.is_file():
        ok(f"Build successful: {target_bin}")
        return target_bin
    else:
        fail(f"Binary not found after build: {target_bin}")
        return None


# ── Run ─────────────────────────────────────────────────────────────

def run_devstack(binary: Path, args: list[str]) -> int:
    """Execute the devstack binary with the given arguments."""
    cmd = [str(binary)] + args
    info(f"Running: {' '.join(cmd)}\n")
    print(f"{BOLD}{'─' * 60}{RESET}")
    result = subprocess.run(cmd)
    print(f"{BOLD}{'─' * 60}{RESET}")
    return result.returncode


def print_usage() -> None:
    """Show quick usage help."""
    print(f"""
{BOLD}{CYAN}DevStack Visualizer — Launcher{RESET}

{BOLD}Usage:{RESET}
    python scripts/run.py <project-path> [options]

{BOLD}Examples:{RESET}
    python scripts/run.py .                              Analyse current project
    python scripts/run.py ../my-app --output svg         SVG output
    python scripts/run.py . --graph --verbose            Verbose graph mode
    python scripts/run.py . --complexity --detect-layers Full analysis
    python scripts/run.py . --json                       JSON output
    python scripts/run.py . --summary                    Quick summary

{BOLD}Options are passed directly to devstack.{RESET}
Run 'python scripts/run.py . --help' to see all devstack options.

{BOLD}Scripts:{RESET}
    python scripts/check_env.py    Check prerequisites & setup environment
    python scripts/run.py          Build & launch devstack (this script)
""")


# ── Main ────────────────────────────────────────────────────────────

def main() -> int:
    print(f"\n{BOLD}{CYAN}DevStack Visualizer — Launcher{RESET}\n")

    # Validate project root
    if not CARGO_TOML.is_file():
        fail(f"Cargo.toml not found at {PROJECT_ROOT}")
        info("Run this script from the devstack_visualizer project directory.")
        return 1

    # Show usage if no arguments
    if len(sys.argv) < 2:
        print_usage()
        # Default: analyse the current project directory with sensible defaults
        if not _prompt_yn("Run devstack on the current project directory?"):
            return 0
        user_args = ["analyze", str(PROJECT_ROOT), "--graph", "--verbose"]
    elif sys.argv[1] in ("-h", "--help"):
        print_usage()
        return 0
    else:
        # Build the argument list
        # If the first arg is a path (doesn't start with -), prepend "analyze"
        user_args = list(sys.argv[1:])
        if user_args and not user_args[0].startswith("-"):
            # Assume it's a path → insert "analyze" subcommand
            user_args = ["analyze"] + user_args
        elif user_args and user_args[0] == "analyze":
            pass  # user already typed "analyze"
        else:
            # All flags — assume they want to analyse current dir
            user_args = ["analyze", "."] + user_args

    # Quick sanity check
    if not quick_check():
        return 1

    # Build (if needed)
    binary = find_or_build_binary(release=True)
    if binary is None:
        return 1

    # Launch
    return run_devstack(binary, user_args)


def _prompt_yn(question: str, default: bool = True) -> bool:
    suffix = " [Y/n]: " if default else " [y/N]: "
    answer = input(f"  {YELLOW}?{RESET}  {question}{suffix}").strip().lower()
    if answer == "":
        return default
    return answer in ("y", "yes")


if __name__ == "__main__":
    sys.exit(main())
