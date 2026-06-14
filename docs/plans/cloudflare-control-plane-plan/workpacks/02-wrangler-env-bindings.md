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
- `docs/proof/cloudflare-control-plane-plan/wp02-wrangler-env-bindings/`

## Acceptance

- Development and production bindings are explicit.
- Secret names appear only as placeholders.
- No wildcard production origin claim exists.

## Proof IDs

- `cloudflare-control.wrangler-dev-config`
- `cloudflare-control.wrangler-prod-config`
- `cloudflare-control.dev-vars-example`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject real secret values in repo.
- Reject production wildcard origins.

## Failure conditions

- Do not claim deployability without real binding setup proof.
