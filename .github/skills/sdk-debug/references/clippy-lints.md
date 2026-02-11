# Clippy Lint Notes for Generated SDKs

Generated code frequently triggers style lints that are not functional regressions.

Guidance:
- Prioritize compile correctness and API parity first.
- Avoid broad behavior-changing refactors in generated files.
- If warnings come from generated naming patterns, prefer targeted `#![allow(...)]` at crate root over editing many generated files.
