# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 00 | Games infra parity extraction | done | `GAMES_INFRA_PARITY_MAP.md` | Keep the parity map aligned with future module changes; do not re-import stripped game-only surfaces. | Parent parity map exists and stays reduction-honest. |
| 01 | Cloudflare module scaffold | implemented / proof-open | `infra/cloudflare/package.json` | Keep the module tree honest and stop counting scaffold directories as implementation. | `infra/cloudflare/` source shape and docs agree on what is real versus placeholder. |
| 02 | Wrangler env and bindings | implemented / proof-open | `infra/cloudflare/wrangler.toml` | Keep placeholder IDs honest while proving secret custody and environment promotion separately. | Bindings, secret names, and env docs agree. |
| 03 | Worker entrypoint runtime guards | implemented / proof-open | `infra/cloudflare/src/index.ts` | Keep fail-fast guards green and restore the scoped integration baseline. | Env validation, CORS, request-size, kill-switch, and safe error behavior remain explicit and tested. |
| 04 | Route manifest and domain contracts | implemented / proof-open | `infra/cloudflare/src/routes.ts` | Keep the manifest synchronized with domain-owned request/response contracts. | No raw route strings are required outside the manifest model. |
| 05 | Auth, admin, and support boundary | partial / proof-open | `infra/cloudflare/src/auth/verifier.ts` | Keep the real boundary honest while waiting on upstream account and trusted-device authority decisions. | Private, admin, support, webhook, and queue trust states are explicit. |
| 06 | Storage and coordination bindings | implemented / proof-open | `infra/cloudflare/src/env.ts` | Prove DO, D1, KV, Queue, and optional R2 ownership with queue and observability depth. | Serialized writes, read models, retry, and audit storage each have an owner. |
| 07 | Local dev, seeding, and fixtures | implemented / proof-open | `LOCAL_DEV_AND_SEEDING_MODEL.md` | Capture local start, seed, and teardown proof from the existing workflow. | Local start/seed/teardown path is explicit and command-backed. |
| 08 | Test runner and test pyramid | implemented / proof-open | `infra/cloudflare/scripts/test-runner.ts` | Keep every scoped test family green and record command output under `output/...`. | Test families, exact assertions, blocker states, and proof paths are explicit and command-backed. |
| 09 | Portal-to-worker smoke | implemented / proof-open | `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/` | Capture the existing smoke in a real proof bundle and keep it redaction-safe. | Parent portal consumer can point at the module contract without secret leakage. |
| 10 | Security, property, fuzz, observability | partial / proof-open | `TESTING_STRATEGY.md` | Deepen the remaining security, property, fuzz, and queue-observability cases without faking coverage. | Security/property/fuzz/observability expectations are explicit and scoped, and execution produces real command output or exact blockers. |
| 11 | Deployment and promotion | pending | `DEPLOYMENT_MODEL.md` | Capture dev-to-production promotion and rollback proof against real environments. | Deployment docs, wrangler files, and secret custody agree. |
| 12 | Payment handoff gate | blocked / proof-missing | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/` | Prove what payment may assume and what remains blocked. | Payment WP00 can point at a real Cloudflare handoff artifact. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
