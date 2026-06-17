# LAN-domain Test Surface

Current truth on this branch/worktree:

- Real LAN tests currently live only under `tests/unit`.
- `tests/unit` currently contains `18` real `*.test.ts` files.
- Other directories under `tests/**` are placeholder scaffolds only.
- Placeholder scaffolds do not count as integration, contract, e2e, property,
  security, observability, release, or load coverage.

Current placeholder-only category roots:

- `ai-safety`
- `chaos`
- `clock-skew`
- `concurrency`
- `consumer-driven`
- `contract`
- `differential`
- `e2e`
- `human-misuse`
- `integration`
- `invariant`
- `load`
- `migration`
- `monitoring`
- `mutation`
- `observability`
- `property-based`
- `quality`
- `release`
- `security`

Rule for future LAN test work:

- Add a category only when it contains real test files.
- Do not treat empty folders or `.gitkeep` scaffolds as coverage.
- Keep LAN test claims inside major top-level categories that actually contain
  executable tests.
