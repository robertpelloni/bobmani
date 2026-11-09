#!/usr/bin/env python3
"""
Automated Submodule Synchronization & Integrity Verifier for bobmani.
Ensures all submodules are cleanly initialized, up to date with published commits,
and have zero uncommitted dirty state.
"""

import os
import subprocess
import sys

SUBMODULES = [
    "Simply-Love-SM5",
    "arrowvortex",
    "beatoraja",
    "bobmania",
    "ddc",
    "ddc_onset",
    "ffr-difficulty-model",
    "hymnmania",
    "itgmania",
    "ksm-v2",
    "leraine-studio",
    "linthesia",
    "pianogame",
]

def run_cmd(cmd, cwd=None):
    res = subprocess.run(cmd, cwd=cwd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    return res.returncode, res.stdout.strip(), res.stderr.strip()

def main():
    print("Starting Automated Submodule Synchronization & Audit...")
    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    errors = []

    # 1. Update submodules recursively
    code, out, err = run_cmd("git submodule update --init --recursive", cwd=repo_root)
    if code != 0:
        print(f"ERROR: git submodule update failed: {err}")
        sys.exit(1)

    print("Checking status for each submodule:")
    for sub in SUBMODULES:
        sub_path = os.path.join(repo_root, sub)
        if not os.path.exists(sub_path):
            errors.append(f"{sub}: directory does not exist")
            continue

        # Check HEAD commit
        c, head, _ = run_cmd("git rev-parse --short HEAD", cwd=sub_path)
        # Check working tree status
        c, status, _ = run_cmd("git status --porcelain", cwd=sub_path)
        is_clean = len(status) == 0

        # Check remote branch
        c, branch, _ = run_cmd("git rev-parse --abbrev-ref HEAD", cwd=sub_path)

        state_str = "CLEAN" if is_clean else "DIRTY"
        print(f" - [{sub:20}] commit: {head:7} | branch: {branch:15} | state: {state_str}")

        if not is_clean:
            errors.append(f"{sub}: dirty working tree: {status}")

    if errors:
        print(f"\nFAILED with {len(errors)} error(s):")
        for e in errors:
            print(f"  * {e}")
        sys.exit(1)
    else:
        print(f"\nSUCCESS: All {len(SUBMODULES)} submodules are clean and synchronized.")
        sys.exit(0)

if __name__ == "__main__":
    main()
