# TAPIS Rust SDK Agent Skills

This directory contains modular agent skills for working with TAPIS Rust SDKs. Each skill follows the [agentskills.io specification](https://agentskills.io/specification) for maximum compatibility with AI agents.

## Available Skills

### [sdk-gen](./sdk-gen/SKILL.md)
**Generate base Rust SDK from OpenAPI specifications**

Use this skill to:
- Generate Rust code from OpenAPI specs (YAML/JSON)
- Set up initial project structure
- Create API modules and models
- Configure dependencies

**When to use:** Starting a new SDK or regenerating after API changes

---

### [sdk-wrapper](./sdk-wrapper/SKILL.md)
**Create high-level client wrapper with ergonomic API**

Use this skill to:
- Build `TapisClient` wrapper with sub-clients
- Implement global JWT authentication
- Ensure 100% API coverage
- Provide clean, ergonomic interface

**When to use:** After generating base SDK, to create production-ready client

---

### [sdk-debug](./sdk-debug/SKILL.md)
**Fix compilation errors and remove warnings**

Use this skill to:
- Diagnose compilation errors
- Resolve dependency conflicts
- Fix clippy warnings
- Handle generator quirks

**When to use:** When SDK fails to compile or has many warnings

---

## Workflow

The typical workflow uses all three skills in sequence:

```
1. sdk-gen    → Generate base SDK from OpenAPI spec
2. sdk-wrapper → Create high-level client wrapper
3. sdk-debug   → Fix any compilation issues
```

## Format

All skills follow the [agentskills.io specification](https://agentskills.io/specification):

```yaml
---
name: skill-name
description: What the skill does and when to use it
license: Apache-2.0
---

# Markdown content with step-by-step instructions
```

### Name Constraints
- Lowercase with hyphens only
- 1-64 characters
- No consecutive hyphens
- No leading/trailing hyphens

### Description Constraints
- 1-1024 characters
- Should describe both WHAT the skill does and WHEN to use it

## Example Usage

```bash
# 1. Generate SDK
openapi-generator-cli generate -i spec.yaml -g rust -o ./tapis-service

# 2. Create wrapper (follow sdk-wrapper/SKILL.md)
cd tapis-service
# ... implement client.rs following guide

# 3. Fix issues (follow sdk-debug/SKILL.md)
cargo build
cargo clippy --fix
cargo fmt
```

## References

- **Documentation:** `../docs/` - Examples and guides
- **Specification:** https://agentskills.io/specification
- **TAPIS API:** https://tapis.readthedocs.io/

## License

Apache-2.0
