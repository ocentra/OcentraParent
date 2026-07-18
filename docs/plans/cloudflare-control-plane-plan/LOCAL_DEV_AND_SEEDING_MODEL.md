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

- Interactive local start is explicit but currently blocked.
- Blocked runtime dependencies:
  - the workspace bootstrap is incomplete in this checkout: `node --import tsx infra/cloudflare/scripts/local-dev-workflow.ts` fails immediately with `ERR_MODULE_NOT_FOUND: Cannot find package 'tsx' imported from C:\Users\sujan\.codex\worktrees\4e8a\OcentraParent\`
  - the local-start rerun has not yet been executed successfully in this checkout against the live generated billing-contract source surface
- Do not claim local-start success until those focused reruns and bootstrap steps are evidenced here.

## Seed commands

- `npm --prefix infra/cloudflare run seed:local`
- `npm --prefix infra/cloudflare run seed:products:local`
- `npm --prefix infra/cloudflare run seed:referrals:local`
- `npm --prefix infra/cloudflare run seed:test-accounts:local`

### Current seed truth

- The seed command family is explicit but currently blocked because the workspace bootstrap fails before the seed commands can execute in this checkout and the seed/bootstrap reruns have not been executed here against the live generated billing-contract source surface.
- Do not describe the billing fixtures as populated while those seed commands are blocked.

## Required fixture families

- `pricing-catalog`
  - source: `seed:products:local`
  - current state: blocked until the workspace bootstrap and local seed/bootstrap reruns are evidenced here
- `parent-test-accounts`
  - source: `seed:local`
  - current state: blocked until the workspace bootstrap and local seed/bootstrap reruns are evidenced here
- `support-admin-test-accounts`
  - source: `seed:test-accounts:local`
  - current state: blocked until the workspace bootstrap and local seed/bootstrap reruns are evidenced here
- `referral-test-graph`
  - source: `seed:referrals:local`
  - current state: blocked until the workspace bootstrap and local seed/bootstrap reruns are evidenced here
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
