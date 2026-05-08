#!/usr/bin/env python3
"""Yank or unyank a published version across all TAPIS SDK crates.

Examples:
  python3 .github/skills/sdk-gen/scripts/yank_published_sdks.py 0.3.0 --dry-run
  python3 .github/skills/sdk-gen/scripts/yank_published_sdks.py 0.3.0
  python3 .github/skills/sdk-gen/scripts/yank_published_sdks.py 0.3.0 --undo
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Iterable, List


def run(cmd: List[str], cwd: Path, dry_run: bool = False) -> None:
    print(f"$ {' '.join(cmd)}")
    if dry_run:
        return
    subprocess.run(cmd, cwd=str(cwd), check=True)


def parse_package_name(manifest: Path) -> str:
    text = manifest.read_text()
    in_package = False
    for line in text.splitlines():
        if line.strip() == "[package]":
            in_package = True
            continue
        if in_package and line.startswith("[") and line.strip() != "[package]":
            in_package = False
        if in_package:
            match = re.match(r'^name\s*=\s*"([^"]+)"\s*$', line.strip())
            if match:
                return match.group(1)
    raise RuntimeError(f"Could not parse package name in {manifest}")


def parse_workspace_members(root_manifest: Path) -> List[str]:
    text = root_manifest.read_text()
    match = re.search(r"(?ms)^\[workspace\]\n(.*?)(?=^\[|\Z)", text)
    if not match:
        raise RuntimeError(f"Could not parse [workspace] in {root_manifest}")

    members = re.findall(r'"([^"]+)"', match.group(1))
    if not members:
        raise RuntimeError(f"No workspace members found in {root_manifest}")
    return members


def iter_crate_dirs(repo_root: Path) -> Iterable[Path]:
    root_manifest = repo_root / "Cargo.toml"
    yield repo_root
    for member in parse_workspace_members(root_manifest):
        crate_dir = repo_root / member
        manifest = crate_dir / "Cargo.toml"
        if not manifest.exists():
            raise RuntimeError(f"Missing manifest for workspace member: {manifest}")
        yield crate_dir


def build_yank_command(version: str, undo: bool) -> List[str]:
    cmd = ["cargo", "yank", "--version", version]
    if undo:
        cmd.append("--undo")
    return cmd


def run_yank_sequence(
    repo_root: Path,
    version: str,
    dry_run: bool,
    list_only: bool,
    undo: bool,
    continue_on_error: bool,
) -> int:
    action = "unyanking" if undo else "yanking"
    failures: List[str] = []

    for crate_dir in iter_crate_dirs(repo_root):
        manifest = crate_dir / "Cargo.toml"
        package_name = parse_package_name(manifest)
        if list_only:
            print(f"{package_name} v{version} ({crate_dir})")
            continue

        print(f"{action} {package_name} v{version}")
        try:
            run(build_yank_command(version, undo), cwd=crate_dir, dry_run=dry_run)
        except subprocess.CalledProcessError:
            failures.append(package_name)
            if not continue_on_error:
                raise

    if failures:
        print(
            f"{action} completed with failures: {', '.join(failures)}",
            file=sys.stderr,
        )
        return 1

    if not list_only:
        print(f"Completed {action} TAPIS SDK crates for version {version}.")
    return 0


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Yank or unyank a published version across all TAPIS SDK crates"
    )
    parser.add_argument(
        "version",
        help="Published crate version to yank or unyank across the TAPIS SDK crates",
    )
    parser.add_argument(
        "--undo",
        action="store_true",
        help="Undo a previous yank instead of yanking the version",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print cargo yank commands without executing them",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="List the crates that would be targeted for the specified version",
    )
    parser.add_argument(
        "--continue-on-error",
        action="store_true",
        help="Continue yanking remaining crates if one yank command fails",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    script_dir = Path(__file__).resolve().parent
    repo_root = script_dir.parents[3]

    if not args.dry_run and not args.list and not os.getenv("CARGO_REGISTRY_TOKEN"):
        raise RuntimeError(
            "CARGO_REGISTRY_TOKEN is not set. Export it before yanking, e.g.:\n"
            "  export CARGO_REGISTRY_TOKEN=<your_crates_io_token>"
        )

    return run_yank_sequence(
        repo_root=repo_root,
        version=args.version,
        dry_run=args.dry_run,
        list_only=args.list,
        undo=args.undo,
        continue_on_error=args.continue_on_error,
    )


if __name__ == "__main__":
    raise SystemExit(main())
