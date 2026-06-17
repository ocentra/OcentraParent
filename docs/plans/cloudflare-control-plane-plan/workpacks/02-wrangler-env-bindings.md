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
