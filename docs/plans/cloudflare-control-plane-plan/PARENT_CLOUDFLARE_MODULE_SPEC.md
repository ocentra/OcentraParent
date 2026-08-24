# Parent Cloudflare Module Spec

Purpose: define the current repo-local `infra/cloudflare/` module surface and
the no-claim boundaries that still gate true completion.

## Module contract

- Module path: `infra/cloudflare/`
- Ownership: `cloudflare-control-plane-plan`
- Consumer plans: payment, portal, support/admin, setup, future entitlement and remote-control consumers
- Current state allowed in this pass: real runtime present, proof-open, and
  dependency-gated auth or deploy closure
- Current state not allowed in this pass: fake runtime claims, fake proof,
  copied game-only code, or empty-folder optics counted as coverage

## Required files

| Path | Required state in this pass |
| --- | --- |
| `infra/cloudflare/package.json` | Exists with real scoped dev, seed, test, deploy, and lint scripts. |
| `infra/cloudflare/wrangler.toml` | Exists with explicit development bindings; resource IDs remain placeholder-backed until deployment proof lands. |
| `infra/cloudflare/wrangler.production.toml` | Exists with explicit production bindings; promotion proof remains open. |
| `infra/cloudflare/.dev.vars.example` | Exists with concrete secret and signing key names only; no real values. |
| `infra/cloudflare/src/index.ts` | Exists as the current concentrated worker runtime, not a stub. |
| `infra/cloudflare/src/env.ts` | Exists with real env and binding validation. |
| `infra/cloudflare/src/routes.ts` | Exists with a real route manifest and auth metadata. |
| `infra/cloudflare/src/auth/` | Exists with a real verifier boundary; upstream account and trusted-device authority remain open dependencies. |
| `infra/cloudflare/src/handlers/` and `src/flows/` | Exist mostly as scaffold directories; they do not count as implementation while runtime remains concentrated elsewhere. |
| `infra/cloudflare/src/durable-objects/`, `src/queues/`, `src/storage/`, `src/providers/`, `src/security/`, `src/observability/` | Exist, but several remain `README.md` placeholders rather than first-class code modules. |
| `infra/cloudflare/scripts/` | Exists with real runner and seed scripts. |
| `infra/cloudflare/tests/...` | Exists with real unit, integration, e2e, contract, security, property, and fuzz files. |
| `infra/cloudflare/docs/...` | Exists but must stay synchronized with actual module truth and `output/...` proof routing. |

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
  "deploy:dev": "npm run contracts:build && wrangler deploy",
  "deploy": "npm run contracts:build && wrangler deploy --config wrangler.production.toml"
}
```

If a script cannot run yet, the script file or the plan docs must return an exact manual-required blocker instead of implying success.

## No-claim boundaries

- Do not claim payment handoff from source presence alone.
- Do not claim shared auth-provider choice until the account plan decides it.
- Do not claim real queue, D1, Durable Object, or deploy readiness without proof.
- Do not claim the module is type-checked, deployed, or tested from scaffold existence alone.
