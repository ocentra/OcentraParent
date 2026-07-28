# Local Dev And Seeding Model

Purpose: define the first local worker workflow before payment runtime work starts.

## Local development contract

- Root command: `npm run dev:cloudflare`
- Module command: `npm --prefix infra/cloudflare run dev`
- Mode: `wrangler dev --local`
- Expected local caller: parent portal or local smoke tooling
- Origin: `http://localhost:3000`
- Auth adapter mode during local work: `account-auth-adapter-manual-required`

### Current local start truth

- The standalone local workflow is preflight-ready and import-check passed, while its runtime boot remains explicitly unproven because that command does not start Wrangler or issue a health request.
- Verified probe:
  - `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts`
  - Result: `preflightStatus = ready`, `importCheckStatus = passed`, `runtimeBootStatus = unproven`
- Separate runtime evidence:
  - `npm --prefix infra/cloudflare run test:integration`
  - Result: 61/61, including the real Wrangler local runtime suite at 10/10 with a bounded `/health` request.
- Keep these claims separate. The import-only report proves the entrypoint preflight; the integration harness proves its own bounded local Worker run. Neither proves a production deployment or payment readiness.

## Seed commands

- `npm --prefix infra/cloudflare run seed:local`
- `npm --prefix infra/cloudflare run seed:products:local`
- `npm --prefix infra/cloudflare run seed:referrals:local`
- `npm --prefix infra/cloudflare run seed:test-accounts:local`

### Current seed truth

- The seed command family is explicit and runnable in this checkout.
- Verified probe:
  - `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  - Result: 4/4; `seed.status = runnable`; all six families have positive counts; five correlated/redacted workflow milestones are flushed to and asserted from a test-owned NDJSON store.
- Do not describe the billing fixtures as payment-ready; they are local seed fixtures only.

## Required fixture families

- `pricing-catalog`
  - source: `seed:products:local`
  - current state: populated with `3` items in the runnable local seed probe
- `parent-test-accounts`
  - source: `seed:local`
  - current state: populated with `4` items in the runnable local seed probe
- `support-admin-test-accounts`
  - source: `seed:test-accounts:local`
  - current state: populated with `4` items in the runnable local seed probe
- `referral-test-graph`
  - source: `seed:referrals:local`
  - current state: populated with `2` items in the runnable local seed probe
- `webhook-payload-fixtures`
  - source: `infra/cloudflare/tests/fuzz/provider-webhook-payload.fuzz.test.ts`, `infra/cloudflare/tests/integration/worker-runtime-real.test.ts`
  - current state: explicit test-fixture-backed family with `5` items, not a seed placeholder
- `queue-replay-fixtures`
  - source: `infra/cloudflare/tests/property/billing-idempotency.property.test.ts`
  - current state: explicit test-fixture-backed family with `2` items, not a seed placeholder

## Structured proof boundary

- Owner: `infra/cloudflare/scripts/local-dev-workflow.ts`
- Store: the existing logging-domain bridge and test-log NDJSON writer, rooted in a test-owned temporary directory.
- Required persisted fields: run id, correlation id, owner, boundary, result, no-claim reason, and redaction state.
- Required milestones: workflow command accepted, import preflight accepted, seed fixtures consumed, teardown contract accepted, and proof chain stored.
- Teardown removes the temporary proof store only after flush and persisted-row assertions complete.
- Durable branch proof is the retained receipt at `docs/proof/cloudflare-control-plane-plan/07-local-dev-seeding-and-fixtures.md`; the raw `output/cloudflare-control-plane-plan-proof/07-local-dev-seeding-and-fixtures/` directory is absent/ephemeral and does not contain durable evidence.

## Manual-required states

- If local D1 or Queue emulation is not wired yet, record the exact blocker.
- If account auth adapter is unresolved, local auth may use a stub only with a clear `manual-required` note.
- If the worker-runtime harness creates `infra/cloudflare/.dev.vars`, teardown may remove only the harness-created file; do not delete a pre-existing developer file.
- If the worker-runtime harness uses `--persist-to <temp-dir>`, teardown must remove that temp directory after the local worker stops.
- WP11 production deployment proof and WP12 payment handoff proof remain separate and absent until their owning workpacks produce them.
