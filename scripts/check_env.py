#!/usr/bin/env python3
"""
DevStack Visualizer — Environment Checker & Setup Script
=========================================================
Checks all prerequisites, validates the environment, and optionally
installs missing components or provides download links.

Prerequisites checked:
  1. Python 3.8+       (running this script)
  2. Rust / Cargo      (building the project)
  3. A C compiler      (required by tree-sitter build)
  4. Graphviz (dot)    (rendering architecture diagrams)

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

# ── Download links ──────────────────────────────────────────────────
LINKS = {
    "rust":     "https://www.rust-lang.org/tools/install",
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


# ── Cargo project check ────────────────────────────────────────────

def check_cargo_project() -> bool:
    """Verify the Cargo.toml exists at the workspace root."""
    project_root = Path(__file__).resolve().parent.parent
    cargo_toml = project_root / "Cargo.toml"
    if cargo_toml.is_file():
        ok(f"Cargo.toml found at {project_root}")
        return True
    else:
        fail(f"Cargo.toml not found at {project_root}")
        info("Make sure you are running this script from the devstack_visualizer project.")
        return False


# ── Build binary ───────────────────────────────────────────────────

def build_release(project_root: Path) -> Optional[Path]:
    """Build the devstack binary in release mode and return its path."""
    info("Building devstack in release mode (this may take a minute)...")
    result = subprocess.run(
        ["cargo", "build", "--release"],
        cwd=str(project_root),
    )
    if result.returncode != 0:
        fail("Cargo build failed.")
        return None

    system = platform.system()
    exe_name = "devstack.exe" if system == "Windows" else "devstack"
    binary = project_root / "target" / "release" / exe_name
    if binary.is_file():
        ok(f"Binary built: {binary}")
        return binary
    else:
        fail(f"Expected binary not found at: {binary}")
        return None


# ── PATH setup ─────────────────────────────────────────────────────

def setup_path(binary_dir: Path) -> None:
    """Add the binary directory to the user's PATH (if not already present)."""
    binary_dir_str = str(binary_dir)
    current_path = os.environ.get("PATH", "")

    if binary_dir_str.lower() in current_path.lower():
        ok(f"PATH already contains {binary_dir_str}")
        return

    system = platform.system()

    if system == "Windows":
        if prompt_yn(f"Add '{binary_dir_str}' to your user PATH?"):
            try:
                # Use PowerShell to modify the user-level PATH persistently
                ps_cmd = (
                    f'$old = [Environment]::GetEnvironmentVariable("PATH", "User");'
                    f'if ($old -notlike "*{binary_dir_str}*") {{'
                    f'  [Environment]::SetEnvironmentVariable("PATH", "$old;{binary_dir_str}", "User");'
                    f'  Write-Host "Done"'
                    f'}}'
                )
                ret = subprocess.run(
                    ["powershell", "-NoProfile", "-Command", ps_cmd],
                    capture_output=True,
                    text=True,
                    timeout=30,
                )
                if ret.returncode == 0:
                    ok(f"Added to user PATH. Restart your terminal for changes to take effect.")
                else:
                    warn("Could not modify PATH automatically.")
                    info(f"Manually add to PATH: {binary_dir_str}")
            except Exception as e:
                warn(f"PATH setup error: {e}")
                info(f"Manually add to PATH: {binary_dir_str}")
        else:
            info(f"Skipped. To add manually, add this to your PATH: {binary_dir_str}")

    elif system == "Darwin" or system == "Linux":
        shell = os.environ.get("SHELL", "/bin/bash")
        if "zsh" in shell:
            rc_file = Path.home() / ".zshrc"
        elif "fish" in shell:
            rc_file = Path.home() / ".config" / "fish" / "config.fish"
        else:
            rc_file = Path.home() / ".bashrc"

        export_line = f'\nexport PATH="{binary_dir_str}:$PATH"\n'
        if rc_file.exists() and binary_dir_str in rc_file.read_text():
            ok(f"PATH already configured in {rc_file}")
            return

        if prompt_yn(f"Add '{binary_dir_str}' to {rc_file}?"):
            with open(rc_file, "a") as f:
                f.write(export_line)
            ok(f"Added to {rc_file}. Run 'source {rc_file}' or restart your terminal.")
        else:
            info(f"Skipped. Add manually: export PATH=\"{binary_dir_str}:$PATH\"")


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
    print()

    results: dict[str, bool] = {}

    # 1. Python version
    results["Python >= 3.8"] = check_python()

    # 2. Rust & Cargo
    results["Rust & Cargo"] = check_rust()

    # 3. C compiler
    results["C Compiler"] = check_c_compiler()

    # 4. Graphviz
    results["Graphviz (dot)"] = check_graphviz()

    # 5. Cargo project
    results["Cargo.toml present"] = check_cargo_project()

    all_ok = print_summary(results)

    if not all_ok:
        return 1

    # ── Offer to build & install ─────────────────────────────────
    project_root = Path(__file__).resolve().parent.parent
    header("Build & Install")

    if prompt_yn("Build the devstack binary in release mode?"):
        binary = build_release(project_root)
        if binary:
            setup_path(binary.parent)
            print()
            info("You can now run:  devstack analyze <path>")
        else:
            return 1
    else:
        info("Skipped build. Run 'cargo build --release' manually when ready.")

    print()
    return 0


if __name__ == "__main__":
    sys.exit(main())
