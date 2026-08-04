# Next Actions

## Current slice

- Current slice: `CFCP-A truth-sync and proof-root canonicalization`; #608 local-proof hardening merged, but WP01 now owns the prerequisite `infra/cloudflare/package.json` Wrangler/Workers-types dependency reconciliation before the proof-only WP07 successor. Account authority follows the separate route Account WP08 Rust contract -> Cloudflare WP06 D1/DO/KV binding/migration -> Cloudflare WP08 runner/integration proof -> Account WP06 aggregate gate. #604 is closed without merge as superseded/conflicting and its branch/evidence are preserved only for audit.
- Current owner: `cloudflare-control-plane-plan`
- Current status: `in_progress`
- Final-tree scoped validation (2026-08-04): PR #608 merged to `main` as `5af4a1a92` after fresh full CI passed its product, security, and platform jobs. The merged local commands `npm --prefix infra/cloudflare run test:local-dev-workflow` (12 focused tests), `npm --prefix infra/cloudflare run lint`, `npm --prefix infra/cloudflare run proof:local-dev`, and `npm run lint:architecture -- --files infra/cloudflare/scripts/local-dev-proof.ts infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts` pass. The proof command retains its result only through the canonical redacted NDJSON artifact under `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/runs/<generated-run-id>`; it does not retain a raw stdout summary. This is local WP07 validation evidence only, not workpack closure. The successor is proof-only from current source after the real Wrangler/Workers-types dependency-resolution gap is cleared; it must not reuse or rebase PR #604.

## Ordered queue

| Order | Slice | Status | First-touch surface | Next action | Exit gate |
| --- | --- | --- | --- | --- | --- |
| 01 | WP01 module dependency reconciliation | blocked / package-scope prerequisite | `infra/cloudflare/package.json` and the resolved module dependency graph | Reconcile the declared Wrangler and `@cloudflare/workers-types` versions/peer requirements in WP01's package scope. Record the selected resolution and restore the module's focused local validation environment before touching WP07 proof. | The WP01 package dependency graph is resolvable and its focused module validation can run; this is not payment/runtime/deployment closure. |
| 06 | WP06 account-identity storage binding/migration | blocked on Account WP08 contract and WP01 environment | `infra/cloudflare/src/env.ts` plus selected D1 migration/adapter surfaces | Consume the Account WP08 canonical Rust handoff; define the D1/DO/KV boundary, adapter, and migration proof without re-owning account authority. Use `npm --prefix infra/cloudflare exec -- wrangler d1 migrations apply <account-identity-d1-database> --local` only after the binding/migration exists. | The exact binding/migration proof or an exact blocker is retained for Cloudflare WP08 and Account WP06; no account/runtime/deployment closure. |
| 08 | WP08 account-identity runner/integration proof | blocked on Cloudflare WP06 | `infra/cloudflare/scripts/test-runner.ts` and selected integration surface | Use the Cloudflare module test scripts, including `npm --prefix infra/cloudflare run test:integration`, to retain the storage-facing runner/proof result or exact blocker. | Cloudflare WP08 maps the WP06 storage assertions and command result to retained proof for Account WP06; no account-contract ownership or runtime-ready claim. |
| 07 | WP07 local dev/seed proof-only successor | blocked on WP01 prerequisite | current `infra/cloudflare` WP07 source and proof root | After WP01 dependency reconciliation, run the proof-only successor from current source and retain the focused bundle. Do not revive PR #604 or the stale private billing-domain import blocker. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |
| 00-12 except WP01/WP07 | Selected Cloudflare workpack | source-present / retained-proof-absent | selected workpack's first-touch surface | Install and reconcile the selected workpack's declared dependencies, then rerun its focused validation and retain the resulting bundle. | The selected workpack has reproducible focused results, retained positive/negative/teardown evidence, and an explicit no-claim boundary. |

## Working rules

- Move exactly one row to `in_progress` when implementation starts.
- Do not start payment runtime slices while row 12 still lacks proof.
- Keep scaffold placeholders honest: `exists` is not the same as `validated`.
- Do not invent, shrink, or merge away test scope outside
  `REQUIRED_TEST_ASSERTION_MATRIX.md`.
- Keep `PLAN_EXECUTION_SCORECARD.md` aligned with real module and proof state.
