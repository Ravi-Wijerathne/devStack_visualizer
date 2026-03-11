#!/usr/bin/env python3
"""
DevStack Visualizer — Launcher Script
=======================================
Quickly launches the Tauri v2 Desktop GUI application in development
or production mode. Installs npm dependencies if needed.

Usage:
    python scripts/run.py              # Launch in development mode (default)
    python scripts/run.py dev          # Launch in development mode
    python scripts/run.py build        # Build production app
    python scripts/run.py --help       # Show help

Examples:
    python scripts/run.py              # Start Tauri dev server + hot reload
    python scripts/run.py dev          # Same as above
    python scripts/run.py build        # Build production installer/bundle
"""

from __future__ import annotations

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
PACKAGE_JSON = PROJECT_ROOT / "package.json"
TAURI_CONF = PROJECT_ROOT / "src-tauri" / "tauri.conf.json"
NODE_MODULES = PROJECT_ROOT / "node_modules"
IS_WINDOWS = platform.system() == "Windows"


# ── Prerequisite checks ────────────────────────────────────────────

def check_prerequisites() -> bool:
    """Fast sanity check — verify Node.js, npm, and Cargo are reachable."""
    errors: list[str] = []

    # Node.js
    try:
        result = subprocess.run(
            ["node", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
        )
        if result.returncode == 0:
            ok(f"Node.js {result.stdout.strip()}")
        else:
            errors.append("Node.js check failed.")
    except FileNotFoundError:
        errors.append("Node.js is not installed or not on PATH.")
    except Exception as e:
        errors.append(f"Error checking Node.js: {e}")

    # npm
    try:
        result = subprocess.run(
            ["npm", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
            shell=IS_WINDOWS,
        )
        if result.returncode == 0:
            ok(f"npm {result.stdout.strip()}")
        else:
            errors.append("npm check failed.")
    except FileNotFoundError:
        errors.append("npm is not installed or not on PATH.")
    except Exception as e:
        errors.append(f"Error checking npm: {e}")

    # Cargo
    try:
        result = subprocess.run(
            ["cargo", "--version"],
            capture_output=True,
            text=True,
            timeout=10,
        )
        if result.returncode == 0:
            ok(f"Cargo {result.stdout.strip()}")
        else:
            errors.append("Cargo check failed.")
    except FileNotFoundError:
        errors.append("Rust/Cargo is not installed or not on PATH.")
    except Exception as e:
        errors.append(f"Error checking Cargo: {e}")

    # Graphviz (optional — warn only)
    try:
        subprocess.run(["dot", "-V"], capture_output=True, timeout=10, shell=IS_WINDOWS)
    except FileNotFoundError:
        warn("Graphviz (dot) not found — DOT export will produce files only, no PNG/SVG rendering.")
    except Exception:
        pass

    if errors:
        for e in errors:
            fail(e)
        info("Run 'python scripts/check_env.py' for detailed diagnostics & setup.")
        return False
    return True


# ── npm install ─────────────────────────────────────────────────────

def ensure_npm_deps() -> bool:
    """Install npm dependencies if node_modules is missing."""
    if NODE_MODULES.is_dir():
        ok("npm dependencies already installed")
        return True

    info("node_modules/ not found — installing npm dependencies...")
    result = subprocess.run(
        ["npm", "install"],
        cwd=str(PROJECT_ROOT),
        shell=IS_WINDOWS,
    )
    if result.returncode == 0:
        ok("npm dependencies installed successfully.")
        return True
    else:
        fail("npm install failed. Fix errors above and try again.")
        return False


# ── Launch modes ────────────────────────────────────────────────────

def launch_dev() -> int:
    """Launch the Tauri app in development mode with hot reload."""
    info("Starting DevStack Visualizer in development mode...")
    info("This will start the Vite dev server and the Tauri window.\n")
    print(f"{BOLD}{'─' * 60}{RESET}")
    result = subprocess.run(
        ["npx", "tauri", "dev"],
        cwd=str(PROJECT_ROOT),
        shell=IS_WINDOWS,
    )
    print(f"{BOLD}{'─' * 60}{RESET}")
    return result.returncode


def build_production() -> int:
    """Build the Tauri app for production (creates installer/bundle)."""
    info("Building DevStack Visualizer for production...")
    info("This will compile the Rust backend and bundle the app.\n")
    print(f"{BOLD}{'─' * 60}{RESET}")
    result = subprocess.run(
        ["npx", "tauri", "build"],
        cwd=str(PROJECT_ROOT),
        shell=IS_WINDOWS,
    )
    print(f"{BOLD}{'─' * 60}{RESET}")

    if result.returncode == 0:
        print()
        ok("Production build complete!")
        bundle_dir = PROJECT_ROOT / "src-tauri" / "target" / "release" / "bundle"
        if bundle_dir.is_dir():
            info(f"Find the installer/executable in: {bundle_dir}")
            # List bundle subdirectories
            for child in sorted(bundle_dir.iterdir()):
                if child.is_dir():
                    info(f"  → {child.name}/")
    return result.returncode


# ── Usage ───────────────────────────────────────────────────────────

def print_usage() -> None:
    """Show quick usage help."""
    print(f"""
{BOLD}{CYAN}DevStack Visualizer — Launcher{RESET}
{CYAN}Tauri v2 Desktop GUI Application{RESET}

{BOLD}Usage:{RESET}
    python scripts/run.py [command]

{BOLD}Commands:{RESET}
    dev       Launch in development mode with hot reload (default)
    build     Build production installer/bundle

{BOLD}Examples:{RESET}
    python scripts/run.py              Start dev server + Tauri window
    python scripts/run.py dev          Same as above
    python scripts/run.py build        Build production app

{BOLD}Other scripts:{RESET}
    python scripts/check_env.py        Check prerequisites & setup environment

{BOLD}Direct npm/Tauri commands:{RESET}
    npm install                        Install frontend dependencies
    npx tauri dev                      Start development mode
    npx tauri build                    Build production app
""")


# ── Main ────────────────────────────────────────────────────────────

def main() -> int:
    print(f"\n{BOLD}{CYAN}DevStack Visualizer — Launcher{RESET}")
    print(f"{CYAN}Tauri v2 Desktop GUI Application{RESET}\n")

    # Validate project root
    if not PACKAGE_JSON.is_file():
        fail(f"package.json not found at {PROJECT_ROOT}")
        info("Run this script from the devstack_visualizer project directory.")
        return 1

    if not TAURI_CONF.is_file():
        fail(f"src-tauri/tauri.conf.json not found at {PROJECT_ROOT}")
        info("Run this script from the devstack_visualizer project directory.")
        return 1

    # Parse command
    command = "dev"  # default
    if len(sys.argv) >= 2:
        arg = sys.argv[1].lower().strip("-")
        if arg in ("h", "help"):
            print_usage()
            return 0
        elif arg in ("dev", "start", "run"):
            command = "dev"
        elif arg in ("build", "release", "prod", "production"):
            command = "build"
        else:
            fail(f"Unknown command: {sys.argv[1]}")
            print_usage()
            return 1

    # Quick prerequisite check
    if not check_prerequisites():
        return 1

    # Ensure npm dependencies are installed
    if not ensure_npm_deps():
        return 1

    print()

    # Execute the chosen command
    if command == "dev":
        return launch_dev()
    elif command == "build":
        return build_production()
    else:
        print_usage()
        return 0


if __name__ == "__main__":
    sys.exit(main())
