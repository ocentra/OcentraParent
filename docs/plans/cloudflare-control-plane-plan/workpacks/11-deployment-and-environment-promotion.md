# Workpack 11: Deployment And Environment Promotion

## Goal

Define deployment commands, environment promotion, and rollback expectations for the shared worker module.

## First-touch surface

- `infra/cloudflare/wrangler.production.toml`
- `DEPLOYMENT_MODEL.md`

## Read inputs

- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)
- [STORAGE_BINDING_MODEL.md](../STORAGE_BINDING_MODEL.md)

## Output files

- `infra/cloudflare/wrangler.production.toml`
- [DEPLOYMENT_MODEL.md](../DEPLOYMENT_MODEL.md)
- `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/`

## Acceptance

- Dev and prod config are separate.
- Secrets remain out of repo.
- Rollback path is explicit.

## Proof IDs

- `cloudflare-control.wrangler-prod-config`

## Validation

- `npm --prefix infra/cloudflare run deploy:dev`
- `npm --prefix infra/cloudflare run deploy`
- post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke in the promoted environment

## Negative cases

- Reject production wildcard origins.
- Reject deploy claims without rollback notes.

## Failure conditions

- Do not treat deploy commands as deployment proof.
