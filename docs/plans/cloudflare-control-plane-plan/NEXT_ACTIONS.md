# Next Actions

## Current slice

- Current slice: `09. Portal-to-worker smoke`
- Current owner: `cloudflare-control-plane-plan`
- Current status: `pending`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 00 | Games infra parity extraction | done | `GAMES_INFRA_PARITY_MAP.md` | Keep the parity map aligned with future module changes; do not re-import stripped game-only surfaces. | Parent parity map exists and stays reduction-honest. |
| 01 | Cloudflare module scaffold | scaffolded / proof-open | `infra/cloudflare/package.json` | Keep the module tree honest while real runtime handlers are added. | `infra/cloudflare/` exists with scripts, docs, and source placeholders. |
| 02 | Wrangler env and bindings | scaffolded / proof-open | `infra/cloudflare/wrangler.toml` | Replace placeholder IDs and vars only when secret custody and env promotion are proven. | Bindings, secret names, and env docs agree. |
| 03 | Worker entrypoint runtime guards | scaffolded / proof-open | `infra/cloudflare/src/index.ts` | Replace placeholder responses with real handlers without weakening fail-fast guards. | Env validation, CORS, request-size, kill-switch, and safe error behavior remain explicit. |
| 04 | Route manifest and domain contracts | scaffolded / proof-open | `infra/cloudflare/src/routes.ts` | Bind the manifest to domain-owned request/response contracts. | No raw route strings are required outside the manifest model. |
| 05 | Auth, admin, and support boundary | scaffolded / proof-open | `infra/cloudflare/src/auth/verifier.ts` | Replace adapter placeholders after account-plan auth decisions land. | Private, admin, support, webhook, and queue trust states are explicit. |
| 06 | Storage and coordination bindings | scaffolded / proof-open | `infra/cloudflare/src/env.ts` | Wire real DO, D1, KV, Queue, and optional R2 bindings. | Serialized writes, read models, retry, and audit storage each have an owner. |
| 07 | Local dev, seeding, and fixtures | scaffolded / manual-required | `LOCAL_DEV_AND_SEEDING_MODEL.md` | Replace placeholder seed blockers with real fixture and teardown flows. | Local start/seed/teardown path is explicit. |
| 08 | Test runner and test pyramid | scaffolded / proof-open | `infra/cloudflare/scripts/test-runner.ts` | Replace the exact blocker runner with real suite execution and logs. | Test families, blocker states, and proof paths are explicit. |
| 09 | Portal-to-worker smoke | pending | `docs/proof/cloudflare-control-plane-plan/wp09-portal-to-worker-e2e-smoke/` | Define first consumer smoke and its no-claim boundary. | Parent portal consumer can point at the module contract without secret leakage. |
| 10 | Security, property, fuzz, observability | pending | `TESTING_STRATEGY.md` | Reduce games security/load tooling to the Parent-required baseline. | Security/property/fuzz/observability expectations are explicit and scoped. |
| 11 | Deployment and promotion | pending | `DEPLOYMENT_MODEL.md` | Define dev-to-production promotion and rollback shape. | Deployment docs, wrangler files, and secret custody agree. |
| 12 | Payment handoff gate | pending | `docs/proof/cloudflare-control-plane-plan/wp12-payment-plan-handoff-gate/` | Prove what payment may assume and what remains blocked. | Payment WP00 can point at a real Cloudflare handoff artifact. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
