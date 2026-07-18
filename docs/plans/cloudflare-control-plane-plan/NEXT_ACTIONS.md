# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 00 | Games infra parity extraction | blocked / proof-required | `GAMES_INFRA_PARITY_MAP.md` | Clear the repo-wide `npm run format:check` drift or carve a narrower accepted docs-validation gate, then rerun the WP00 formatting check. | Parent parity map is present in docs, formatting validation reruns clean, and the docs-validation proof is recorded. |
| 01 | Cloudflare module scaffold | blocked / proof-required | `infra/cloudflare/package.json` | Keep the scaffold aligned with the checked-in billing-contract sidecar and current output inventory; preserve the current green execution record and close the proof-artifact gap. | `infra/cloudflare/` source shape and docs agree on what is real versus placeholder, scoped module lint passes, and the proof artifact is recorded. |
| 02 | Wrangler env and bindings | blocked / proof-required | `infra/cloudflare/wrangler.toml` | Keep wrangler/env docs aligned with the checked-in billing-contract sidecar and current output inventory; preserve the current green execution record and close the proof-artifact gap. | Explicit dev/prod bindings and placeholder-only secret custody remain documented, and the required unit plus lint reruns complete clean. |
| 03 | Worker entrypoint runtime guards | blocked / proof-required | `infra/cloudflare/src/index.ts` | The worker entrypoint already consumes the checked-in billing-contract sidecar; preserve the current green execution record while keeping fetch-path guard proof explicit. | Env validation, CORS, request-size, kill-switch, and safe error behavior are explicit in source, and the scoped unit plus integration reruns prove the worker entrypoint on the current surface. |
| 04 | Route manifest and domain contracts | blocked / proof-required | `infra/cloudflare/src/routes.ts` | Keep the route-manifest packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record while keeping route and auth exception proof explicit. | The manifest and auth model remain documented, and the scoped unit, property, contract, and integration reruns prove live route and dispatch coverage. |
| 05 | Auth, admin, and support boundary | blocked / proof-required | `infra/cloudflare/src/auth/verifier.ts` | Keep the auth/admin/support packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record while keeping the owned packet proof explicit. | Private, admin, support, webhook, and queue trust states are explicit, and the unit, security, and integration reruns prove the owned packet on a booted worker. |
| 06 | Storage and coordination bindings | blocked / proof-required | `infra/cloudflare/src/env.ts` | Keep the storage and coordination packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record while keeping storage proof explicit. | DO/D1/KV/Queue/R2 ownership stays explicit, and the required unit, integration, and property reruns prove the owned storage packet. |
| 07 | Local dev, seeding, and fixtures | blocked / proof-required | `LOCAL_DEV_AND_SEEDING_MODEL.md` | Keep the local-dev and seed packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record and keep the proof-artifact gap explicit. | The local dev and seed model remains explicit, and the owned workflow probe plus integration test rerun clean through local start and seed. |
| 08 | Test runner and test pyramid | blocked / proof-required | `infra/cloudflare/scripts/test-runner.ts` | Keep the test runner aligned with the checked-in billing-contract sidecar; preserve the current green execution record and keep the proof-artifact gap explicit. | Test families, exact assertions, blocker states, and proof paths are explicit, and the narrowed required-family runner plus module lint rerun clean under the current gate surface. |
| 09 | Portal-to-worker smoke | blocked / proof-required | `output/cloudflare-control-plane-plan-proof/09-portal-to-worker-e2e-smoke/` | Keep the portal-to-worker smoke aligned with the checked-in billing-contract sidecar; preserve the current green execution record and keep the proof-artifact gap explicit. | Parent portal consumer expectations remain documented, and the owned e2e family reaches `/auth/billing/status` through a booted worker. |
| 10 | Security, property, fuzz, observability | blocked / proof-required | `TESTING_STRATEGY.md` | Keep the security, property, fuzz, and observability packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record and keep the proof-artifact gap explicit. | Security/property/fuzz/observability expectations stay explicit, and the required families rerun clean without widening into excluded Cloudflare tests. |
| 11 | Deployment and promotion | blocked / proof-required | `DEPLOYMENT_MODEL.md` | Keep the deployment and promotion packet aligned with the checked-in billing-contract sidecar; preserve the current green execution record and keep the proof-artifact gap explicit. | Deployment model notes remain explicit, and deploy plus post-deploy smoke rerun with real resource identifiers and out-of-repo secrets in place. |
| 12 | Payment handoff gate | blocked / proof-required | `output/cloudflare-control-plane-plan-proof/12-payment-plan-handoff-gate/` | Keep historical WP01-WP11 references distinct from the current checkout output inventory; the current gap is the absent proof artifact plus placeholder deployment custody and missing downstream payment acknowledgment. The defined expected output path is present in docs, but the generated proof is absent/not produced in this checkout. | The handoff proof is generated, validated, consumed downstream, and the carried blockers are resolved or explicitly accepted. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
