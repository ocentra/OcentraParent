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

- Interactive local start is import-check passed / runtime boot unproven in this checkout; it does not accept traffic yet.
- Verified probe:
  - `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts`
  - Result: `start.status = runnable`, `importCheckStatus = passed`, `runtimeBootStatus = unproven`
- Do not treat the import-only local probe as payment readiness; it only proves the local dev path is callable from this checkout and does not prove Wrangler boot or request health.

## Seed commands

- `npm --prefix infra/cloudflare run seed:local`
- `npm --prefix infra/cloudflare run seed:products:local`
- `npm --prefix infra/cloudflare run seed:referrals:local`
- `npm --prefix infra/cloudflare run seed:test-accounts:local`

### Current seed truth

- The seed command family is explicit and runnable in this checkout.
- Verified probe:
  - `node --import tsx --test infra/cloudflare/tests/integration/local-dev-seeding-workflow.test.ts`
  - Result: `seed.status = runnable`
- Do not describe the billing fixtures as payment-ready; they are local seed fixtures only.

## Required fixture families

- `pricing-catalog`
  - source: `seed:products:local`
  - current state: populated in the runnable local seed probe
- `parent-test-accounts`
  - source: `seed:local`
  - current state: populated in the runnable local seed probe
- `support-admin-test-accounts`
  - source: `seed:test-accounts:local`
  - current state: populated in the runnable local seed probe
- `referral-test-graph`
  - source: `seed:referrals:local`
  - current state: populated in the runnable local seed probe
- `webhook-payload-fixtures`
  - source: `infra/cloudflare/tests/fuzz/provider-webhook-payload.fuzz.test.ts`, `infra/cloudflare/tests/integration/worker-runtime-real.test.ts`
  - current state: explicit test-fixture-backed family, not a seed placeholder
- `queue-replay-fixtures`
  - source: `infra/cloudflare/tests/property/billing-idempotency.property.test.ts`
  - current state: explicit test-fixture-backed family, not a seed placeholder

## Manual-required states

- If local D1 or Queue emulation is not wired yet, record the exact blocker.
- If account auth adapter is unresolved, local auth may use a stub only with a clear `manual-required` note.
- If the worker-runtime harness creates `infra/cloudflare/.dev.vars`, teardown may remove only the harness-created file; do not delete a pre-existing developer file.
- If the worker-runtime harness uses `--persist-to <temp-dir>`, teardown must remove that temp directory after the local worker stops.
