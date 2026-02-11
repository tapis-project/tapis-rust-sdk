# Wrapper Testing Guide

## Build-level checks

```bash
cargo build --all-targets
```

## Coverage parity checks

```bash
rg '^pub async fn ' src/apis/*_api.rs | wc -l
rg '^[[:space:]]*pub async fn ' src/client.rs | wc -l
```

## Smoke test pattern

- Create a client instance
- Call one read-only endpoint per sub-client
- Ensure auth header behavior matches service expectations
