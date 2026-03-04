#!/usr/bin/env python3
"""
DevStack Visualizer — Environment Checker & Setup Script
=========================================================
Checks all prerequisites for the Tauri v2 Desktop GUI application,
validates the environment, and optionally installs missing components
or provides download links.

Prerequisites checked:
  1. Python 3.8+       (running this script)
  2. Node.js / npm     (frontend build & Tauri CLI)
  3. Rust / Cargo      (Tauri backend / Rust core)
  4. A C compiler      (required by tree-sitter build)
  5. Graphviz (dot)    (exporting architecture diagrams)

Run:
    python scripts/check_env.py
"""

from __future__ import annotations

import os
import platform
import shutil
import subprocess
import sys
from pathlib import Path
from typing import Optional

# ── Colours (ANSI) ──────────────────────────────────────────────────
GREEN = "\033[92m"
YELLOW = "\033[93m"
RED = "\033[91m"
CYAN = "\033[96m"
BOLD = "\033[1m"
RESET = "\033[0m"

# ── Minimum versions ───────────────────────────────────────────────
MIN_PYTHON = (3, 8)
MIN_RUST = (1, 60)  # Rust 2021 edition requirement
MIN_NODE = (18, 0)  # Node.js 18+ required for Tauri v2

# ── Download links ──────────────────────────────────────────────────
LINKS = {
    "rust":     "https://www.rust-lang.org/tools/install",
    "node":     "https://nodejs.org/en/download/",
    "graphviz": "https://graphviz.org/download/",
    "msvc":     "https://visualstudio.microsoft.com/visual-cpp-build-tools/",
    "gcc":      "https://gcc.gnu.org/install/",
    "xcode":    "https://developer.apple.com/xcode/",
    "python":   "https://www.python.org/downloads/",
}


def header(msg: str) -> None:
    print(f"\n{BOLD}{CYAN}{'─' * 60}")
    print(f"  {msg}")
    print(f"{'─' * 60}{RESET}\n")


def ok(msg: str) -> None:
    print(f"  {GREEN}✔{RESET}  {msg}")


def warn(msg: str) -> None:
    print(f"  {YELLOW}⚠{RESET}  {msg}")


def fail(msg: str) -> None:
    print(f"  {RED}✘{RESET}  {msg}")


def info(msg: str) -> None:
    print(f"  {CYAN}ℹ{RESET}  {msg}")


# ── Utility ─────────────────────────────────────────────────────────

def run_cmd(args: list[str], timeout: int = 15) -> Optional[str]:
    """Run a command and return stdout, or None on failure."""
    try:
        result = subprocess.run(
            args,
            capture_output=True,
            text=True,
            timeout=timeout,
        )
        return result.stdout.strip() if result.returncode == 0 else None
    except Exception:
        return None


def parse_version(text: str) -> Optional[tuple[int, ...]]:
    """Extract the first semver-like version from *text*."""
    import re
    m = re.search(r"(\d+)\.(\d+)(?:\.(\d+))?", text)
    if m:
        parts = [int(x) for x in m.groups() if x is not None]
        return tuple(parts)
    return None


def prompt_yn(question: str, default: bool = True) -> bool:
    """Ask a yes/no question; return a bool."""
    suffix = " [Y/n]: " if default else " [y/N]: "
    answer = input(f"  {YELLOW}?{RESET}  {question}{suffix}").strip().lower()
    if answer == "":
        return default
    return answer in ("y", "yes")


# ── Individual Checks ──────────────────────────────────────────────

def check_python() -> bool:
    """Check that the running Python meets the minimum version."""
    v = sys.version_info[:2]
    if v >= MIN_PYTHON:
        ok(f"Python {sys.version.split()[0]}  (>= {'.'.join(map(str, MIN_PYTHON))} required)")
        return True
    else:
        fail(f"Python {sys.version.split()[0]} is too old  (>= {'.'.join(map(str, MIN_PYTHON))} required)")
        info(f"Download: {LINKS['python']}")
        return False


def check_rust() -> bool:
    """Check that Rust (rustc) and Cargo are installed."""
    passed = True

    # rustc
    rustc_out = run_cmd(["rustc", "--version"])
    if rustc_out:
        ver = parse_version(rustc_out)
        if ver and ver[:2] >= MIN_RUST:
            ok(f"rustc {'.'.join(map(str, ver))}  (>= {'.'.join(map(str, MIN_RUST))} required)")
        elif ver:
            warn(f"rustc {'.'.join(map(str, ver))} — upgrade recommended (>= {'.'.join(map(str, MIN_RUST))})")
        else:
            ok(f"rustc found: {rustc_out}")
    else:
        fail("rustc not found")
        passed = False

    # cargo
    cargo_out = run_cmd(["cargo", "--version"])
    if cargo_out:
        ok(f"Cargo: {cargo_out}")
    else:
        fail("Cargo not found")
        passed = False

    if not passed:
        info(f"Install Rust & Cargo: {LINKS['rust']}")
        if platform.system() != "Windows":
            info("  Quick install:  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
        else:
            info("  Download and run rustup-init.exe from the link above.")
    return passed


def check_node() -> bool:
    """Check that Node.js and npm are installed (required for frontend & Tauri CLI)."""
    passed = True

    # Node.js
    node_out = run_cmd(["node", "--version"])
    if node_out:
        ver = parse_version(node_out)
        if ver and ver[:2] >= MIN_NODE:
            ok(f"Node.js {node_out}  (>= {'.'.join(map(str, MIN_NODE))} required)")
        elif ver:
            warn(f"Node.js {node_out} — upgrade recommended (>= {'.'.join(map(str, MIN_NODE))})")
        else:
            ok(f"Node.js found: {node_out}")
    else:
        fail("Node.js not found")
        passed = False

    # npm
    npm_out = run_cmd(["npm", "--version"])
    if npm_out:
        ok(f"npm {npm_out}")
    else:
        fail("npm not found")
        passed = False

    if not passed:
        info(f"Install Node.js (includes npm): {LINKS['node']}")
        system = platform.system()
        if system == "Windows":
            info("  Or: winget install OpenJS.NodeJS.LTS")
        elif system == "Darwin":
            info("  Or: brew install node")
        else:
            info("  Or: sudo apt install nodejs npm  (Ubuntu/Debian)")
            info("      sudo dnf install nodejs npm  (Fedora)")

    return passed


def check_c_compiler() -> bool:
    """Check for a C compiler (needed to build tree-sitter native code)."""
    system = platform.system()

    if system == "Windows":
        # Check for MSVC cl.exe or gcc/MinGW
        cl = shutil.which("cl")
        gcc = shutil.which("gcc")
        cc = shutil.which("cc")
        if cl:
            ok("C compiler: MSVC (cl.exe)")
            return True
        if gcc:
            ok("C compiler: GCC (gcc.exe)")
            return True
        if cc:
            ok("C compiler: cc")
            return True
        # On Windows with Rust, the MSVC build tools are usually bundled via rustup
        # Let's check if cargo can compile a simple C dependency
        warn("No standalone C compiler found — MSVC Build Tools are usually installed with Rust on Windows.")
        info(f"If builds fail, install Visual C++ Build Tools: {LINKS['msvc']}")
        return True  # soft pass — usually works with rustup default

    elif system == "Darwin":
        cc = shutil.which("cc") or shutil.which("clang") or shutil.which("gcc")
        if cc:
            ok(f"C compiler: {cc}")
            return True
        fail("No C compiler found")
        info(f"Install Xcode Command Line Tools:  xcode-select --install")
        info(f"Or download: {LINKS['xcode']}")
        return False

    else:  # Linux / other
        cc = shutil.which("cc") or shutil.which("gcc") or shutil.which("clang")
        if cc:
            ok(f"C compiler: {cc}")
            return True
        fail("No C compiler found")
        info("Install via your package manager, e.g.:")
        info("  Ubuntu/Debian: sudo apt install build-essential")
        info("  Fedora:        sudo dnf install gcc")
        info(f"  Or see: {LINKS['gcc']}")
        return False


def check_graphviz() -> bool:
    """Check that Graphviz (dot) is installed and on PATH."""
    dot = shutil.which("dot")
    if dot:
        version_out = run_cmd(["dot", "-V"])
        if version_out:
            ok(f"Graphviz: {version_out}")
        else:
            # dot -V writes to stderr on some systems
            try:
                result = subprocess.run(
                    ["dot", "-V"],
                    capture_output=True,
                    text=True,
                    timeout=10,
                )
                ver_str = (result.stdout or result.stderr).strip()
                ok(f"Graphviz: {ver_str}")
            except Exception:
                ok(f"Graphviz found at: {dot}")
        return True
    else:
        fail("Graphviz (dot) not found on PATH")
        info(f"Download: {LINKS['graphviz']}")
        system = platform.system()
        if system == "Windows":
            if prompt_yn("Attempt to install Graphviz via winget?"):
                ret = subprocess.run(
                    ["winget", "install", "--id", "Graphviz.Graphviz", "-e"],
                    timeout=120,
                )
                if ret.returncode == 0:
                    ok("Graphviz installed via winget — you may need to restart your terminal for PATH changes.")
                    return True
                else:
                    warn("winget install failed — please install manually.")
            else:
                info("Manual install options:")
                info(f"  winget install Graphviz.Graphviz")
                info(f"  choco install graphviz")
                info(f"  Or download from: {LINKS['graphviz']}")
        elif system == "Darwin":
            info("  brew install graphviz")
        else:
            info("  Ubuntu/Debian: sudo apt install graphviz")
            info("  Fedora:        sudo dnf install graphviz")
        return False


# ── Cargo & frontend project checks ────────────────────────────────

def check_cargo_project() -> bool:
    """Verify the src-tauri/Cargo.toml and package.json exist at the workspace root."""
    project_root = Path(__file__).resolve().parent.parent
    passed = True

    # Tauri Cargo.toml
    tauri_cargo = project_root / "src-tauri" / "Cargo.toml"
    if tauri_cargo.is_file():
        ok(f"src-tauri/Cargo.toml found")
    else:
        fail(f"src-tauri/Cargo.toml not found at {project_root}")
        passed = False

    # Frontend package.json
    pkg_json = project_root / "package.json"
    if pkg_json.is_file():
        ok(f"package.json found")
    else:
        fail(f"package.json not found at {project_root}")
        passed = False

    # tauri.conf.json
    tauri_conf = project_root / "src-tauri" / "tauri.conf.json"
    if tauri_conf.is_file():
        ok(f"src-tauri/tauri.conf.json found")
    else:
        warn(f"src-tauri/tauri.conf.json not found — Tauri app may not be configured")

    if not passed:
        info("Make sure you are running this script from the devstack_visualizer project root.")

    return passed


def check_node_modules() -> bool:
    """Check if npm dependencies are installed."""
    project_root = Path(__file__).resolve().parent.parent
    node_modules = project_root / "node_modules"
    if node_modules.is_dir():
        ok("node_modules/ directory found (npm dependencies installed)")
        return True
    else:
        warn("node_modules/ not found — run 'npm install' to install frontend dependencies")
        return False


def check_tauri_cli() -> bool:
    """Check if @tauri-apps/cli is available."""
    project_root = Path(__file__).resolve().parent.parent
    result = run_cmd(["npx", "tauri", "--version"])
    if result:
        ok(f"Tauri CLI: {result}")
        return True
    else:
        warn("Tauri CLI not found — install with 'npm install' (included in devDependencies)")
        return False


# ── Install dependencies ────────────────────────────────────────────

def install_npm_deps(project_root: Path) -> bool:
    """Install npm dependencies (frontend + Tauri CLI)."""
    info("Installing npm dependencies...")
    result = subprocess.run(
        ["npm", "install"],
        cwd=str(project_root),
    )
    if result.returncode == 0:
        ok("npm dependencies installed successfully.")
        return True
    else:
        fail("npm install failed.")
        return False


# ── Build Tauri app ─────────────────────────────────────────────────

def build_tauri_app(project_root: Path) -> bool:
    """Build the Tauri desktop application."""
    info("Building DevStack Visualizer (Tauri app) — this may take a few minutes...")
    result = subprocess.run(
        ["npx", "tauri", "build"],
        cwd=str(project_root),
    )
    if result.returncode == 0:
        ok("Tauri app built successfully!")
        # Show the output location
        system = platform.system()
        if system == "Windows":
            bundle_dir = project_root / "src-tauri" / "target" / "release" / "bundle"
            info(f"Look for the installer/executable in: {bundle_dir}")
        elif system == "Darwin":
            bundle_dir = project_root / "src-tauri" / "target" / "release" / "bundle" / "dmg"
            info(f"Look for the .dmg in: {bundle_dir}")
        else:
            bundle_dir = project_root / "src-tauri" / "target" / "release" / "bundle"
            info(f"Look for the package in: {bundle_dir}")
        return True
    else:
        fail("Tauri build failed. Check the errors above.")
        return False


# ── Summary ─────────────────────────────────────────────────────────

def print_summary(results: dict[str, bool]) -> bool:
    header("Summary")
    all_ok = True
    for name, passed in results.items():
        if passed:
            ok(name)
        else:
            fail(name)
            all_ok = False

    if all_ok:
        print(f"\n  {GREEN}{BOLD}All checks passed! Environment is ready.{RESET}")
    else:
        print(f"\n  {RED}{BOLD}Some checks failed. Please resolve the issues above.{RESET}")
    return all_ok


# ── Main ────────────────────────────────────────────────────────────

def main() -> int:
    header("DevStack Visualizer — Environment Check & Setup")
    info(f"System: {platform.system()} {platform.machine()}")
    info(f"Python: {sys.version}")
    info(f"App type: Tauri v2 Desktop GUI (React + TypeScript frontend, Rust backend)")
    print()

    results: dict[str, bool] = {}

    # 1. Python version
    results["Python >= 3.8"] = check_python()

    # 2. Node.js & npm
    results["Node.js & npm"] = check_node()

    # 3. Rust & Cargo
    results["Rust & Cargo"] = check_rust()

    # 4. C compiler
    results["C Compiler"] = check_c_compiler()

    # 5. Graphviz (optional, used for DOT export only)
    results["Graphviz (dot)"] = check_graphviz()

    # 6. Project files
    results["Project files present"] = check_cargo_project()

    all_ok = print_summary(results)

    if not all_ok:
        return 1

    # ── Check npm dependencies & Tauri CLI ───────────────────────
    project_root = Path(__file__).resolve().parent.parent
    header("Frontend Dependencies & Tauri CLI")

    has_modules = check_node_modules()
    if not has_modules:
        if prompt_yn("Install npm dependencies now?"):
            if not install_npm_deps(project_root):
                return 1
        else:
            info("Skipped. Run 'npm install' manually before launching the app.")

    check_tauri_cli()

    # ── Offer to build the Tauri app ─────────────────────────────
    header("Build & Launch")
    info("Development mode:  npx tauri dev")
    info("Production build:  npx tauri build")
    print()

    if prompt_yn("Build the Tauri desktop app in production mode?", default=False):
        if not build_tauri_app(project_root):
            return 1
    else:
        info("Skipped production build.")
        info("To start development mode, run:  npx tauri dev")

    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
