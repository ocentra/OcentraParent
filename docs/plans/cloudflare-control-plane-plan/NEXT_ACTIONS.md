# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 00 | Games infra parity extraction | blocked / proof-required | `GAMES_INFRA_PARITY_MAP.md` | Clear the repo-wide `npm run format:check` drift or carve a narrower accepted docs-validation gate, then rerun the WP00 formatting check. | Parent parity map is present in docs, formatting validation reruns clean, and the docs-validation proof is recorded. |
| 01 | Cloudflare module scaffold | blocked / proof-required | `infra/cloudflare/package.json` | Keep the scaffold aligned with the checked-in billing-contract sidecar, rerun the exact WP01 gate, and retain a current-head receipt. | `infra/cloudflare/` source shape and docs agree on what is real versus placeholder, scoped module lint passes, and the receipt is reviewed. |
| 02 | Wrangler env and bindings | blocked / proof-required | `infra/cloudflare/wrangler.toml` | Rerun the exact WP02 binding/env family and retain environment-safe evidence; do not treat placeholder identifiers as readiness. | Explicit dev/prod bindings and placeholder-only secret custody remain documented, and the required unit plus lint reruns complete clean. |
| 03 | Worker entrypoint runtime guards | blocked / proof-required | `infra/cloudflare/src/index.ts` | Map and rerun the exact WP03 guard assertions at current head, then retain negative-path evidence. | Env validation, CORS, request-size, kill-switch, and safe error behavior are explicit in source, and the scoped unit plus integration reruns prove the worker entrypoint on the current surface. |
| 04 | Route manifest and domain contracts | blocked / proof-required | `infra/cloudflare/src/routes.ts` | Map and rerun the exact WP04 route/auth assertions at current head, then retain a receipt without widening into payment semantics. | The manifest and auth model remain documented, and the scoped unit, property, contract, and integration reruns prove live route and dispatch coverage. |
| 05 | Auth, admin, and support boundary | blocked / proof-required | `infra/cloudflare/src/auth/verifier.ts` | Rerun the WP05 local boundary family, retain exact unavailable/manual-required states, and keep production account/trust inputs blocked. | Private, admin, support, webhook, and queue trust states are explicit, and the unit, security, and integration reruns prove only the owned local packet. |
| 06 | Storage and coordination bindings | blocked / proof-required | `infra/cloudflare/src/env.ts` | Rerun the exact WP06 storage/queue family and retain retry/dead-letter/operations evidence. | DO/D1/KV/Queue/R2 ownership stays explicit, and the required unit, integration, and property reruns prove the owned storage packet. |
| 07 | Local dev, seeding, and fixtures | source-present / retained-receipt-required | `LOCAL_DEV_AND_SEEDING_MODEL.md` | Regenerate the focused WP07 gates at current head and retain a compact receipt without tracking generated `output/` files. Keep production deployment, WP12 handoff, downstream payment acceptance, and manual authority states open. | A current-head receipt is reviewed without promoting local preflight, seed, structured-log, or bounded Wrangler evidence into production, WP12, or payment readiness. |
| 08 | Test runner and test pyramid | blocked / proof-required | `infra/cloudflare/scripts/test-runner.ts` | Rerun every WP08 family, map exact assertions, and retain a current-head receipt; the WP12 subset is insufficient. | Test families, exact assertions, blocker states, and proof paths are explicit, and the required-family runner plus module lint rerun clean. |
| 09 | Portal-to-worker smoke | blocked / proof-required | `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/` | Rerun the exact WP09 e2e flow through a booted worker and retain a portal-smoke receipt. | Parent portal consumer expectations remain documented, and the owned e2e family reaches `/auth/billing/status` through a booted worker. |
| 10 | Security, property, fuzz, observability | blocked / proof-required | `TESTING_STRATEGY.md` | Rerun security, property, fuzz, integration, and observability assertions at current head and retain negative evidence. | Security/property/fuzz/observability expectations stay explicit, and the required families rerun clean without widening into excluded Cloudflare tests. |
| 11 | Deployment and promotion | blocked / proof-required | `DEPLOYMENT_MODEL.md` | Run deploy/promotion/rollback only with real resources and out-of-repo secrets, then retain environment-safe receipts. | Deployment model notes remain explicit, and deploy plus post-deploy smoke and rollback rerun with real resource identifiers. |
| 12 | Payment handoff gate | blocked / retained-receipt-present / downstream-ack-required | `docs/proof/cloudflare-control-plane-plan/12-payment-plan-handoff-gate.md`; raw `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/` | Keep historical WP01-WP11 references distinct from current accepted proof. The retained receipt records current local gates and exact blockers, while account/trust/provider/storage/portal/deployment/custody and downstream payment acknowledgment remain open. | Payment consumes and acknowledges an accepted handoff only after required upstream receipts exist and carried blockers are resolved or explicitly accepted without converting blocked state into readiness. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
