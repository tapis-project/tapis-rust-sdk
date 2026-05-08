#!/usr/bin/env python3
"""Update README version snippets from Cargo manifest versions."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
from pathlib import Path


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


def parse_package_version(manifest: Path) -> str:
    text = manifest.read_text()
    in_package = False
    for line in text.splitlines():
        if line.strip() == "[package]":
            in_package = True
            continue
        if in_package and line.startswith("[") and line.strip() != "[package]":
            in_package = False
        if in_package:
            match = re.match(r'^version\s*=\s*"([^"]+)"\s*$', line.strip())
            if match:
                return match.group(1)
    raise RuntimeError(f"Could not parse package version in {manifest}")


def workspace_manifests(root_manifest: Path) -> list[Path]:
    metadata = subprocess.run(
        [
            "cargo",
            "metadata",
            "--manifest-path",
            str(root_manifest),
            "--no-deps",
            "--format-version",
            "1",
        ],
        check=True,
        capture_output=True,
        text=True,
    )
    data = json.loads(metadata.stdout)
    manifests = sorted({Path(pkg["manifest_path"]) for pkg in data["packages"]})
    if not manifests:
        raise RuntimeError(f"No manifests found via cargo metadata for {root_manifest}")
    return manifests


def replace_pattern(text: str, pattern: str, repl: str) -> str:
    return re.sub(pattern, repl, text, flags=re.M)


def update_root_readme(readme_path: Path, version: str, dry_run: bool) -> bool:
    text = readme_path.read_text()
    updated = text

    updated = replace_pattern(
        updated,
        r"Current release line:\s*`[^`]+`",
        f"Current release line: `{version}`",
    )
    updated = replace_pattern(
        updated,
        r'^(tapis-[A-Za-z0-9_-]+\s*=\s*)"[^"]+"$',
        rf'\1"{version}"',
    )

    if updated == text:
        return False

    print(f"updated README versions: {readme_path}")
    if not dry_run:
        readme_path.write_text(updated)
    return True


def update_service_readme(
    readme_path: Path, package_name: str, version: str, dry_run: bool
) -> bool:
    text = readme_path.read_text()
    updated = text

    updated = replace_pattern(
        updated,
        r"^- Package version:\s*.*$",
        f"- Package version: {version}",
    )
    updated = replace_pattern(
        updated,
        rf'^({re.escape(package_name)}\s*=\s*)"[^"]+"$',
        rf'\1"{version}"',
    )

    if updated == text:
        return False

    print(f"updated README versions: {readme_path}")
    if not dry_run:
        readme_path.write_text(updated)
    return True


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Update TAPIS README version snippets from Cargo manifests"
    )
    parser.add_argument(
        "--version",
        help="Override the manifest versions and rewrite all managed README snippets to this version",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="Print which README files would be updated without modifying them",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()

    script_dir = Path(__file__).resolve().parent
    repo_root = script_dir.parents[3]
    root_manifest = repo_root / "Cargo.toml"
    root_readme = repo_root / "README.md"

    manifests = workspace_manifests(root_manifest)
    root_version = args.version or parse_package_version(root_manifest)

    changed = False
    if root_readme.exists():
        changed = update_root_readme(root_readme, root_version, args.dry_run) or changed

    for manifest in manifests:
        crate_dir = manifest.parent
        if crate_dir == repo_root:
            continue
        readme_path = crate_dir / "README.md"
        if not readme_path.exists():
            continue
        package_name = parse_package_name(manifest)
        version = args.version or parse_package_version(manifest)
        changed = (
            update_service_readme(readme_path, package_name, version, args.dry_run)
            or changed
        )

    if args.dry_run and not changed:
        print("Dry run complete. No README changes needed.")
    elif args.dry_run:
        print("Dry run complete. No files changed.")
    elif not changed:
        print("README versions already up to date.")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
