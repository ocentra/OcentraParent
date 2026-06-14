# Parent Cloudflare Module Spec

Purpose: define the minimum repo-local `infra/cloudflare/` scaffold that Parent needs before consumer plans can claim runtime work.

## Module contract

- Module path: `infra/cloudflare/`
- Ownership: `cloudflare-control-plane-plan`
- Consumer plans: payment, portal, support/admin, setup, future entitlement and remote-control consumers
- Current state allowed in this pass: scaffold-only or manual-required
- Current state not allowed in this pass: fake runtime claims, fake proof, copied game-only code

## Required files

| Path | Required state in this pass |
| --- | --- |
| `infra/cloudflare/package.json` | Exists with script contract; dependencies may remain manual-required. |
| `infra/cloudflare/wrangler.toml` | Exists with development bindings and placeholders only. |
| `infra/cloudflare/wrangler.production.toml` | Exists with production names and placeholders only. |
| `infra/cloudflare/.dev.vars.example` | Exists with placeholder values only. |
| `infra/cloudflare/src/index.ts` | Exists and fails safe; no payment runtime claim yet. |
| `infra/cloudflare/src/env.ts` | Exists with binding interface. |
| `infra/cloudflare/src/routes.ts` | Exists with route manifest skeleton. |
| `infra/cloudflare/src/auth/` | Exists with verifier interface or exact blocker. |
| `infra/cloudflare/src/handlers/` and `src/flows/` | Exist as scaffold directories; no copied game handlers. |
| `infra/cloudflare/src/durable-objects/`, `src/queues/`, `src/storage/`, `src/providers/`, `src/security/`, `src/observability/` | Exist as scaffold directories. |
| `infra/cloudflare/scripts/` | Exists with placeholder runner and seed scripts or explicit blockers. |
| `infra/cloudflare/tests/...` | Exists with placeholder docs that map required test files and blockers. |
| `infra/cloudflare/docs/...` | Exists and points back to this plan for route truth. |

## Required scripts

The module script contract is:

```json
{
  "dev": "wrangler dev --local",
  "dev:remote": "wrangler dev",
  "login": "wrangler login",
  "seed:local": "tsx scripts/seed-local.ts",
  "seed:products:local": "tsx scripts/seed-products-local.ts",
  "seed:referrals:local": "tsx scripts/seed-referrals-local.ts",
  "seed:test-accounts:local": "tsx scripts/seed-test-accounts-local.ts",
  "test": "tsx scripts/test-runner.ts",
  "test:unit": "tsx scripts/test-runner.ts --type=unit",
  "test:integration": "tsx scripts/test-runner.ts --type=integration",
  "test:e2e": "tsx scripts/test-runner.ts --type=e2e",
  "test:contract": "tsx scripts/test-runner.ts --type=contract",
  "test:security": "tsx scripts/test-runner.ts --type=security",
  "test:property": "tsx scripts/test-runner.ts --type=property",
  "test:fuzz": "tsx scripts/test-runner.ts --type=fuzz",
  "test:all-cloudflare": "npm run test:unit && npm run test:integration && npm run test:e2e && npm run test:contract && npm run test:security && npm run test:property && npm run test:fuzz",
  "deploy:dev": "wrangler deploy --env development",
  "deploy": "wrangler deploy --config wrangler.production.toml --env production"
}
```

If a script cannot run yet, the script file or the plan docs must return an exact manual-required blocker instead of implying success.

## No-claim boundaries

- Do not claim payment runtime logic from this scaffold.
- Do not claim shared auth-provider choice until the account plan decides it.
- Do not claim real queue, D1, or DO behavior without proof.
- Do not claim the module is type-checked, deployed, or tested from scaffold existence alone.
