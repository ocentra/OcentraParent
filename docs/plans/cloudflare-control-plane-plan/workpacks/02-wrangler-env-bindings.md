# Workpack 02: Wrangler Env Bindings

## Goal

Define development and production Wrangler config, binding names, and secret custody.

## First-touch surface

- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`
- `infra/cloudflare/.dev.vars.example`

## Read inputs

- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)
- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)

## Output files

- `infra/cloudflare/wrangler.toml`
- `infra/cloudflare/wrangler.production.toml`
- `infra/cloudflare/.dev.vars.example`
- `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/`

## Acceptance

- Development and production bindings are explicit.
- Secret names appear only as placeholders.
- No wildcard production origin claim exists.

## Proof IDs

- `cloudflare-control.wrangler-dev-config`
- `cloudflare-control.wrangler-prod-config`
- `cloudflare-control.dev-vars-example`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run test:unit`
- Scoped validation: `npm --prefix infra/cloudflare run lint`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare`

## Negative cases

- Reject real secret values in repo.
- Reject production wildcard origins.

## Failure conditions

- Do not claim deployability without real binding setup proof.

## Completion

- Status: blocked / proof-required for WP02 only; no deploy-ready, runtime-ready, or payment-ready claim is made.
- Proof root: `output/cloudflare-control-plane-plan-proof/02-wrangler-env-bindings/`
- Owned config surface: `infra/cloudflare/wrangler.toml`, `infra/cloudflare/wrangler.production.toml`, `infra/cloudflare/.dev.vars.example`
- Owned test surface: `infra/cloudflare/tests/unit/env-bindings.test.ts`

## What is actually proved

- Development and production Wrangler configs both declare the expected D1, KV, queue, optional R2, analytics, and Durable Object binding names explicitly.
- Production origin and CORS origin remain explicit as `https://parent.ocentra.com`; no wildcard production origin claim exists.
- Checked-in secret-bearing values in `.dev.vars.example` remain placeholders only; no real secret values are kept in repo.
- Auth and entitlement refs in checked-in Wrangler config remain explicit `manual-required` placeholders rather than accidental readiness claims.

## Blocked truth

- `npm --prefix infra/cloudflare run test:unit` remains proof-required until the current green execution record is captured into the output proof bundle.
- `npm --prefix infra/cloudflare run lint` passed at head `c121ba5eb`; the broader rerun set still has not been refreshed here, but the current Cloudflare module lint is not a blocker.
- Those failures are outside the owned WP02 wrangler/dev-vars surface, so they are carried as blockers rather than fixed here.

## Proof artifacts

- `00-scope-summary.md`
- `01-negative-case-proof.md`
- `02-rollback-or-teardown-proof.md`
- `16-validation-commands.log`

## Focused validations

- `node --import tsx --test infra/cloudflare/tests/unit/env-bindings.test.ts`
- `npm --prefix infra/cloudflare run test:unit` blocked before worker boot
- `npm --prefix infra/cloudflare run lint` blocked in broader Cloudflare sources outside WP02
- `npm run lint:architecture -- --files infra/cloudflare`

## No-claim boundary

- No claim is made that the checked-in Wrangler files are deployable as-is.
- No claim is made that placeholder D1/KV identifiers or manual-required refs prove environment readiness.
- No claim is made that the Cloudflare worker boots successfully in this worktree.
