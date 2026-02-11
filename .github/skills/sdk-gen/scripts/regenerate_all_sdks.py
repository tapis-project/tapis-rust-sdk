#!/usr/bin/env python3
"""End-to-end TAPIS SDK regeneration orchestrator.

Workflow:
1) Generate selected services from OpenAPI registry (optional)
2) Apply known generator fixes
3) Generate wrappers + basic examples for each service crate
4) Update parent crate workspace/dependencies/re-exports
5) Build workspace and verify wrapper coverage
6) Optionally run version bump script as final step
"""

from __future__ import annotations

import argparse
import json
import re
import socket
import subprocess
import sys
from pathlib import Path
from typing import Iterable, List
from urllib.parse import urlparse


def run(cmd: List[str], cwd: Path, dry_run: bool = False) -> None:
    print(f"$ {' '.join(cmd)}")
    if dry_run:
        return
    subprocess.run(cmd, cwd=str(cwd), check=True)


def pascal_from_kebab(name: str) -> str:
    return "".join(part.capitalize() for part in name.split("-"))


def pascal_from_snake(name: str) -> str:
    return "".join(part.capitalize() for part in name.split("_"))


def split_top_level_params(params: str) -> List[str]:
    parts: List[str] = []
    current: List[str] = []
    depth = {"<": 0, "(": 0, "[": 0, "{": 0}
    closes = {">": "<", ")": "(", "]": "[", "}": "{"}

    for ch in params:
        if ch in depth:
            depth[ch] += 1
        elif ch in closes:
            opener = closes[ch]
            depth[opener] = max(0, depth[opener] - 1)

        if ch == "," and all(v == 0 for v in depth.values()):
            token = "".join(current).strip()
            if token:
                parts.append(token)
            current = []
        else:
            current.append(ch)

    tail = "".join(current).strip()
    if tail:
        parts.append(tail)

    return parts


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
            m = re.match(r'^version\s*=\s*"([^"]+)"\s*$', line.strip())
            if m:
                return m.group(1)
    raise RuntimeError(f"Could not parse package version in {manifest}")


def replace_section(text: str, section: str, new_body: str) -> str:
    pattern = re.compile(rf"(?ms)^\[{re.escape(section)}\]\n.*?(?=^\[|\Z)")
    repl = f"[{section}]\n{new_body}\n"
    if pattern.search(text):
        return pattern.sub(repl, text, count=1)
    if not text.endswith("\n"):
        text += "\n"
    return text + "\n" + repl


def ensure_attr(lib_text: str, attr: str) -> str:
    if attr in lib_text:
        return lib_text
    anchor = "#![allow(clippy::too_many_arguments)]"
    if anchor in lib_text:
        return lib_text.replace(anchor, anchor + "\n" + attr, 1)
    return attr + "\n" + lib_text


def update_service_manifest(manifest: Path, dry_run: bool) -> None:
    text = manifest.read_text()
    original = text

    def with_stream(match: re.Match[str]) -> str:
        block = match.group(0)
        if '"stream"' in block:
            return block
        return block.replace("] }", ', "stream"] }')

    text = re.sub(
        r"reqwest\s*=\s*\{[^{}]*features\s*=\s*\[[^\]]*\][^{}]*\}",
        with_stream,
        text,
        flags=re.M,
    )

    if not re.search(r'^http\s*=\s*"\^1\.0"\s*$', text, flags=re.M):
        text = replace_section(
            text,
            "dependencies",
            _append_line_to_section_body(_section_body(text, "dependencies"), 'http = "^1.0"'),
        )

    dev_body = _section_body(text, "dev-dependencies")
    if dev_body is None:
        text = replace_section(
            text,
            "dev-dependencies",
            'tokio = { version = "^1.0", features = ["full"] }',
        )
    elif not re.search(r'^tokio\s*=\s*\{[^\n]*features\s*=\s*\["full"\][^\n]*\}\s*$', dev_body, flags=re.M):
        text = replace_section(
            text,
            "dev-dependencies",
            _append_line_to_section_body(dev_body, 'tokio = { version = "^1.0", features = ["full"] }'),
        )

    if text != original:
        print(f"updated manifest: {manifest}")
        if not dry_run:
            manifest.write_text(text)


def _section_body(text: str, section: str) -> str | None:
    pattern = re.compile(rf"(?ms)^\[{re.escape(section)}\]\n(.*?)(?=^\[|\Z)")
    m = pattern.search(text)
    if not m:
        return None
    return m.group(1).rstrip("\n")


def _append_line_to_section_body(body: str | None, line: str) -> str:
    body = (body or "").strip("\n")
    if not body:
        return line
    if re.search(rf"(?m)^{re.escape(line)}\s*$", body):
        return body
    return body + "\n" + line


def fix_empty_enum_defaults(crate_dir: Path, dry_run: bool) -> None:
    models_dir = crate_dir / "src" / "models"
    if not models_dir.exists():
        return

    enum_block = re.compile(
        r"\n///[^\n]*\n#\[derive[^\n]*\]\npub enum (?P<name>[A-Za-z0-9_]+) \{\n\}\n\nimpl Default for (?P=name) \{\n\s*fn default\(\) -> (?P=name) \{\n\s*Self::\n\s*\}\n\}\n",
        flags=re.M,
    )

    for path in models_dir.glob("*.rs"):
        text = path.read_text()
        original = text

        matches = list(enum_block.finditer(text))
        for m in matches:
            enum_name = m.group("name")
            text = re.sub(
                rf"Option<Option<\s*{re.escape(enum_name)}\s*>>",
                "Option<Option<serde_json::Value>>",
                text,
            )
            text = re.sub(
                rf"Option<\s*{re.escape(enum_name)}\s*>",
                "Option<serde_json::Value>",
                text,
            )
            text = re.sub(
                rf":\s*{re.escape(enum_name)}\s*,",
                ": serde_json::Value,",
                text,
            )

        text = enum_block.sub("\n", text)
        # Recovery path for previously partially-patched files where the enum was removed
        # but the field type still references Result.
        if "pub enum Result" not in text:
            text = text.replace("Option<Option<Result>>", "Option<Option<serde_json::Value>>")
            text = text.replace("Option<Result>", "Option<serde_json::Value>")

        if text != original:
            print(f"patched empty enum default: {path}")
            if not dry_run:
                path.write_text(text)


def fix_invalid_serde_json_paths(crate_dir: Path, dry_run: bool) -> None:
    targets: List[Path] = []
    apis_dir = crate_dir / "src" / "apis"
    if apis_dir.exists():
        targets.extend(sorted(apis_dir.glob("*.rs")))
    client_file = crate_dir / "src" / "client.rs"
    if client_file.exists():
        targets.append(client_file)

    for path in targets:
        text = path.read_text()
        fixed = text.replace("models::serde_json::Value", "serde_json::Value")
        if fixed != text:
            print(f"patched invalid serde_json path: {path}")
            if not dry_run:
                path.write_text(fixed)


def ensure_header_byte_range_display(crate_dir: Path, dry_run: bool) -> None:
    path = crate_dir / "src" / "models" / "header_byte_range.rs"
    if not path.exists():
        return

    text = path.read_text()
    if "impl std::fmt::Display for HeaderByteRange" in text:
        return

    snippet = """
impl std::fmt::Display for HeaderByteRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min = self.min.map(|v| v.to_string()).unwrap_or_default();
        let max = self.max.map(|v| v.to_string()).unwrap_or_default();
        write!(f, "bytes={}-{}", min, max)
    }
}
"""
    print(f"patched HeaderByteRange display: {path}")
    if not dry_run:
        path.write_text(text.rstrip() + "\n\n" + snippet.strip() + "\n")


def generate_client(crate_dir: Path, service: str, wrapper_name: str, dry_run: bool) -> None:
    api_dir = crate_dir / "src" / "apis"
    modules = sorted(p.stem.replace("_api", "") for p in api_dir.glob("*_api.rs"))

    imports = ", ".join(f"{m}_api" for m in modules)
    out: List[str] = [
        f"use crate::apis::{{configuration, Error, {imports}}};",
        "use crate::models;",
        "use http::header::{HeaderMap, HeaderValue};",
        "use std::sync::Arc;",
        "",
        "#[derive(Clone)]",
        f"pub struct {wrapper_name} {{",
        "    config: Arc<configuration::Configuration>,",
    ]

    for module in modules:
        out.append(f"    pub {module}: {pascal_from_snake(module)}Client,")

    out.extend(["}", "", f"impl {wrapper_name} {{"])

    if service == "authenticator":
        out.extend(
            [
                "    pub fn new(base_url: &str, jwt_token: Option<&str>) -> Result<Self, Box<dyn std::error::Error>> {",
                "        let mut headers = HeaderMap::new();",
                "        if let Some(token) = jwt_token {",
                "            headers.insert(\"X-Tapis-Token\", HeaderValue::from_str(token)?);",
                "        }",
            ]
        )
    else:
        out.extend(
            [
                "    pub fn new(base_url: &str, jwt_token: &str) -> Result<Self, Box<dyn std::error::Error>> {",
                "        let mut headers = HeaderMap::new();",
                "        headers.insert(\"X-Tapis-Token\", HeaderValue::from_str(jwt_token)?);",
            ]
        )

    out.extend(
        [
            "",
            "        let client = reqwest::Client::builder()",
            "            .default_headers(headers)",
            "            .build()?;",
            "",
            "        let mut config = configuration::Configuration::default();",
            "        config.base_path = base_url.to_string();",
            "        config.client = client;",
            "",
            "        let config = Arc::new(config);",
            "",
            "        Ok(Self {",
            "            config: config.clone(),",
        ]
    )

    for module in modules:
        cname = pascal_from_snake(module) + "Client"
        out.append(f"            {module}: {cname} {{ config: config.clone() }},")

    out.extend(
        [
            "        })",
            "    }",
            "",
            "    pub fn config(&self) -> &configuration::Configuration {",
            "        &self.config",
            "    }",
            "}",
            "",
        ]
    )

    sig_re = re.compile(r"^pub async fn\s+([A-Za-z0-9_]+)\((.*)\)\s*->\s*(.*)\s*\{$")

    for module in modules:
        cname = pascal_from_snake(module) + "Client"
        out.extend(["#[derive(Clone)]", f"pub struct {cname} {{", "    config: Arc<configuration::Configuration>,", "}", "", f"impl {cname} {{"])

        for line in (api_dir / f"{module}_api.rs").read_text().splitlines():
            if not line.startswith("pub async fn "):
                continue
            m = sig_re.match(line.strip())
            if not m:
                continue

            fn_name = m.group(1)
            params = m.group(2).strip()
            ret = m.group(3).strip()
            ret = re.sub(r"\bError<([A-Za-z0-9_]+)>", rf"Error<{module}_api::\1>", ret)

            pitems = split_top_level_params(params)
            non_cfg = [p for p in pitems if not p.strip().startswith("configuration:")]
            sig_params = "&self" + (", " + ", ".join(non_cfg) if non_cfg else "")

            arg_names: List[str] = []
            for p in non_cfg:
                lhs = p.split(":", 1)[0].strip()
                if lhs:
                    arg_names.append(lhs)

            invoke = f"{module}_api::{fn_name}(&self.config"
            if arg_names:
                invoke += ", " + ", ".join(arg_names)
            invoke += ").await"

            out.append(f"    pub async fn {fn_name}({sig_params}) -> {ret} {{")
            out.append(f"        {invoke}")
            out.append("    }")
            out.append("")

        out.extend(["}", ""])

    target = crate_dir / "src" / "client.rs"
    print(f"generated wrapper: {target}")
    if not dry_run:
        target.write_text("\n".join(out) + "\n")


def update_service_lib(crate_dir: Path, wrapper_name: str, dry_run: bool) -> None:
    lib_path = crate_dir / "src" / "lib.rs"
    text = lib_path.read_text()

    text = ensure_attr(text, "#![allow(non_snake_case)]")
    text = ensure_attr(text, "#![allow(deprecated)]")

    lines = text.splitlines()
    lines = [
        ln
        for ln in lines
        if ln.strip() != "pub mod client;"
        and not re.match(r"^pub use client::[A-Za-z0-9_]+;\s*$", ln.strip())
    ]
    lines.append("pub mod client;")
    lines.append(f"pub use client::{wrapper_name};")

    updated = "\n".join(lines) + "\n"
    if updated != lib_path.read_text():
        print(f"updated lib exports: {lib_path}")
        if not dry_run:
            lib_path.write_text(updated)


def generate_basic_example(crate_dir: Path, service: str, crate_name: str, wrapper_name: str, dry_run: bool) -> None:
    examples_dir = crate_dir / "examples"
    target = examples_dir / f"{service}_basic_example.rs"
    examples_dir.mkdir(parents=True, exist_ok=True)

    if service == "authenticator":
        text = f'''use {crate_name}::{wrapper_name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let client = {wrapper_name}::new(&base_url, None)?;
    println!("Initialized {wrapper_name} against {{}}", client.config().base_path);

    Ok(())
}}
'''
    else:
        text = f'''use {crate_name}::{wrapper_name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {{
    let jwt_token = std::env::var("TAPIS_TOKEN")
        .expect("TAPIS_TOKEN environment variable must be set");
    let base_url = std::env::var("TAPIS_BASE_URL")
        .unwrap_or_else(|_| "https://dev.develop.tapis.io/v3".to_string());

    let client = {wrapper_name}::new(&base_url, &jwt_token)?;
    println!("Initialized {wrapper_name} against {{}}", client.config().base_path);

    Ok(())
}}
'''

    print(f"generated example: {target}")
    if not dry_run:
        target.write_text(text)


def check_wrapper_coverage(crate_dir: Path) -> tuple[int, int]:
    api_total = 0
    for api_file in (crate_dir / "src" / "apis").glob("*_api.rs"):
        api_total += sum(1 for line in api_file.read_text().splitlines() if line.startswith("pub async fn "))

    client_file = crate_dir / "src" / "client.rs"
    wrapper_total = sum(1 for line in client_file.read_text().splitlines() if line.lstrip().startswith("pub async fn "))

    return api_total, wrapper_total


def update_parent_cargo(repo_root: Path, services: List[str], dry_run: bool) -> None:
    root_manifest = repo_root / "Cargo.toml"
    text = root_manifest.read_text()

    dep_lines = []
    for svc in services:
        svc_dir = repo_root / f"tapis-{svc}"
        version = parse_package_version(svc_dir / "Cargo.toml")
        dep_key = f"tapis-{svc}"
        pkg_name = f"tapis_{svc.replace('-', '_')}"
        dep_lines.append(
            f'{dep_key} = {{ package = "{pkg_name}", version = "{version}", path = "./tapis-{svc}" }}'
        )

    ws_members = [f'    "tapis-{svc}",' for svc in services]

    text = replace_section(text, "dependencies", "\n".join(dep_lines))
    text = replace_section(text, "workspace", "members = [\n" + "\n".join(ws_members) + "\n]")

    print(f"updated parent manifest: {root_manifest}")
    if not dry_run:
        root_manifest.write_text(text)


def update_parent_lib(repo_root: Path, services: List[str], dry_run: bool) -> None:
    path = repo_root / "src" / "lib.rs"
    lines = ["//! Tapis SDK - Unified client library for Tapis services", ""]

    for svc in services:
        module = svc.replace("-", "_")
        pkg = f"tapis_{svc.replace('-', '_')}"
        display = " ".join(part.capitalize() for part in svc.split("-"))
        lines.extend(
            [
                f"/// Tapis {display} service client",
                f"pub mod {module} {{",
                f"    pub use {pkg}::*;",
                "}",
                "",
            ]
        )

    text = "\n".join(lines).rstrip() + "\n"
    print(f"updated parent lib: {path}")
    if not dry_run:
        path.write_text(text)


def service_crate_name(crate_dir: Path) -> str:
    text = (crate_dir / "Cargo.toml").read_text()
    m = re.search(r'^name\s*=\s*"([^"]+)"\s*$', text, flags=re.M)
    if not m:
        raise RuntimeError(f"Could not parse package name in {crate_dir / 'Cargo.toml'}")
    return m.group(1)


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Regenerate TAPIS SDKs with wrappers/examples and parent wiring")
    p.add_argument("--env", default="prod", help="OpenAPI environment key (prod/staging/dev)")
    p.add_argument(
        "--services",
        default="all",
        help="Comma-separated services to process, or 'all' (default)",
    )
    p.add_argument("--skip-generate", action="store_true", help="Skip OpenAPI generation step")
    p.add_argument(
        "--skip-network-precheck",
        action="store_true",
        help="Skip DNS reachability checks for OpenAPI spec hosts before generation",
    )
    p.add_argument(
        "--allow-branch-switch",
        action="store_true",
        help="Allow generation script to checkout env-mapped branch (default is no branch switch)",
    )
    p.add_argument("--skip-build", action="store_true", help="Skip final cargo build")
    p.add_argument("--skip-bump", action="store_true", help="Skip final version bump")
    p.add_argument("--dry-run", action="store_true", help="Print actions without writing files or running commands")
    p.add_argument(
        "--continue-on-generate-error",
        action="store_true",
        help="Continue processing other services when generation fails",
    )
    return p.parse_args()


def load_services(specs_json: Path, env_key: str, selected: str) -> List[str]:
    data = json.loads(specs_json.read_text())
    if env_key not in data:
        raise RuntimeError(f"Environment '{env_key}' not found in {specs_json}")

    available = sorted(data[env_key].keys())
    if selected == "all":
        return available

    requested = [s.strip() for s in selected.split(",") if s.strip()]
    missing = [s for s in requested if s not in available]
    if missing:
        raise RuntimeError(f"Requested services not in {env_key}: {', '.join(missing)}")
    return requested


def precheck_spec_hosts(specs_json: Path, env_key: str, services: List[str]) -> None:
    data = json.loads(specs_json.read_text())
    env_specs = data.get(env_key, {})
    hosts = set()
    for svc in services:
        spec_url = env_specs.get(svc)
        if not spec_url:
            continue
        host = urlparse(spec_url).hostname
        if host:
            hosts.add(host)

    unresolved: List[str] = []
    for host in sorted(hosts):
        try:
            socket.getaddrinfo(host, None)
        except socket.gaierror:
            unresolved.append(host)

    if unresolved:
        raise RuntimeError(
            "DNS lookup failed for OpenAPI spec host(s): "
            + ", ".join(unresolved)
            + ". Check network access (or use --skip-generate / --skip-network-precheck)."
        )


def main() -> int:
    args = parse_args()

    script_dir = Path(__file__).resolve().parent
    repo_root = script_dir.parents[3]
    specs_json = repo_root / ".github" / "skills" / "sdk-gen" / "references" / "OpenAPI_specs.json"
    generate_script = repo_root / ".github" / "skills" / "sdk-gen" / "scripts" / "generate_rust_sdk.sh"
    bump_script = repo_root / ".github" / "skills" / "sdk-parent" / "scripts" / "bump_minor_version.sh"

    all_env_services = load_services(specs_json, args.env, "all")
    services = load_services(specs_json, args.env, args.services)
    print(f"Services ({args.env}): {', '.join(services)}")

    failed_generation: List[str] = []
    if not args.skip_generate:
        if not args.skip_network_precheck and not args.dry_run:
            precheck_spec_hosts(specs_json, args.env, services)
        for svc in services:
            gen_cmd = ["bash", str(generate_script)]
            if not args.allow_branch_switch:
                gen_cmd.append("--no-branch-switch")
            gen_cmd.extend([args.env, svc])
            try:
                run(gen_cmd, cwd=repo_root, dry_run=args.dry_run)
            except subprocess.CalledProcessError:
                failed_generation.append(svc)
                if not args.continue_on_generate_error:
                    raise

    if failed_generation:
        print("Generation failures:", ", ".join(failed_generation), file=sys.stderr)

    processed_services = [s for s in services if s not in failed_generation]
    if failed_generation and not processed_services and not args.skip_generate:
        raise RuntimeError("All generation steps failed. Aborting before build/version bump.")

    for svc in processed_services:
        crate_dir = repo_root / f"tapis-{svc}"
        if not crate_dir.exists():
            raise RuntimeError(f"Missing crate directory: {crate_dir}")

        wrapper = f"Tapis{pascal_from_kebab(svc)}"
        manifest = crate_dir / "Cargo.toml"

        update_service_manifest(manifest, args.dry_run)
        fix_empty_enum_defaults(crate_dir, args.dry_run)
        fix_invalid_serde_json_paths(crate_dir, args.dry_run)
        ensure_header_byte_range_display(crate_dir, args.dry_run)
        generate_client(crate_dir, svc, wrapper, args.dry_run)
        update_service_lib(crate_dir, wrapper, args.dry_run)

        crate_name = service_crate_name(crate_dir)
        generate_basic_example(crate_dir, svc, crate_name, wrapper, args.dry_run)

    parent_services = [svc for svc in all_env_services if (repo_root / f"tapis-{svc}").exists()]
    if parent_services:
        update_parent_cargo(repo_root, parent_services, args.dry_run)
        update_parent_lib(repo_root, parent_services, args.dry_run)

    if not args.dry_run:
        for svc in processed_services:
            crate_dir = repo_root / f"tapis-{svc}"
            generated, wrapped = check_wrapper_coverage(crate_dir)
            if generated != wrapped:
                raise RuntimeError(
                    f"Wrapper mismatch for tapis-{svc}: generated={generated}, wrapped={wrapped}"
                )
            print(f"wrapper parity ok: tapis-{svc} ({generated})")

    if not args.skip_build:
        run(["cargo", "build", "--workspace", "--all-targets"], cwd=repo_root, dry_run=args.dry_run)

    if not args.skip_bump and not (failed_generation and not args.skip_generate):
        run(["bash", str(bump_script)], cwd=repo_root, dry_run=args.dry_run)
        if not args.skip_build:
            run(["cargo", "build", "--workspace", "--all-targets"], cwd=repo_root, dry_run=args.dry_run)
    elif failed_generation and not args.skip_generate:
        print("Skipping version bump due generation failures.", file=sys.stderr)

    print("Automation workflow completed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
