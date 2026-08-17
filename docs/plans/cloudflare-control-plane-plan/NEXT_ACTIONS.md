# Next Actions

## Current slice

- Current slice: `Cloudflare WP06 durable account-authority adapter/caller source`. Independent review accepts WP01's bounded scaffold and Account WP08's sealed `v0.7` source contract as implementation-phase inputs only. WP06 must now replace the uncalled provider-subject store boundary with the real durable adapter/caller without inventing provider or household authority. Tests, migration execution, proof, deployment, and runtime-ready claims remain deferred. #604 stays closed without merge.
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`
- Final-tree scoped validation (2026-08-04): PR #608 merged to `main` as `5af4a1a92` after fresh full CI passed its product, security, and platform jobs. The merged local commands `npm --prefix infra/cloudflare run test:local-dev-workflow` (12 focused tests), `npm --prefix infra/cloudflare run lint`, `npm --prefix infra/cloudflare run proof:local-dev`, and `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-proof.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` pass. The proof command retains its result only through the canonical redacted NDJSON artifact under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/runs/<generated-run-id>`; it does not retain a raw stdout summary. This is local WP07 validation evidence only, not workpack closure. The successor is proof-only from current source after the real Wrangler/Workers-types dependency-resolution gap is cleared; it must not reuse or rebase PR #604.

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 01 | WP01 module scaffold | validation / bounded scaffold source accepted | `infra/cloudflare/package.json`, Wrangler configs, Worker entrypoint, env and route surfaces | Preserve the reviewed scaffold boundary. Do not reopen source unless WP06 exposes a concrete missing seam. Tests/proof remain a later phase. | Implementation-phase input only; no runtime, deployment, provider, or DONE claim. |
| 06 | WP06 account-identity durable adapter/caller | implementation-phase authorized; source packet next | `infra/cloudflare/src/storage/account-identity-store.ts`, both Wrangler configs, isolated migration, and the narrow Worker-owned caller | Implement the durable adapter/caller against Account WP08 `v0.7`; reject caller-supplied authority, inactive/revoked/mismatched bindings, missing migration, and unavailable provider verification. Do not run the later test/proof phase yet. | Real source path consumes the canonical contract and is reachable only through a fail-closed Worker-owned boundary; no runtime/deployment completion claim. |
| 08 | WP08 account-identity runner/integration proof | blocked on Cloudflare WP06 and the WP01 module dependency environment | `infra/cloudflare/scripts/test-runner.ts`, `src/generated/billing-contracts.ts`, and selected integration surface | After WP06 proof and a non-empty `npm --prefix infra/cloudflare ls wrangler @cloudflare/workers-types` result, use the module script `npm --prefix infra/cloudflare run test:integration` to retain the storage-facing runner/proof result or a new exact blocker. Do not revive obsolete `packages/billing-domain/src/*` imports. | Cloudflare WP08 maps the WP06 storage assertions and command result to retained proof for Account WP06; no account-contract ownership or runtime-ready claim. |
| 07 | WP07 local dev/seed proof-only successor | proof phase deferred | current `infra/cloudflare` WP07 source and proof root | Run only after the current source-completion chain reaches its test/proof phase. Do not revive PR #604 or the stale private billing-domain import blocker. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |
| 00-12 except WP01/WP06/WP07/WP08 | Selected Cloudflare workpack | source-present / retained-proof-absent | selected workpack's first-touch surface | Install and reconcile the selected workpack's declared dependencies, then rerun its focused validation and retain the resulting bundle. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
