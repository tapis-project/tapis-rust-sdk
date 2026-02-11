# Wrapper Architecture Patterns

## Core Pattern

- One top-level `Tapis<Service>` wrapper
- One sub-client per `*_api.rs` module
- Shared `Arc<configuration::Configuration>`

## Method Pattern

Wrapper method should:
1. Mirror generated API signature (excluding `configuration` argument)
2. Delegate directly to generated API method
3. Preserve exact success/error types

## Auth Pattern

Set `X-Tapis-Token` once in constructor via reqwest default headers.

Use optional token only for services with public endpoints.
