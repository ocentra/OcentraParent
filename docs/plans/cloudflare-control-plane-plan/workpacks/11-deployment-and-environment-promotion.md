# Workpack 11: Deployment And Environment Promotion

> **2026-07-28 correction:** The later missing-private-billing-import blocker text is historical. `infra/cloudflare` now imports module-local generated billing contracts. This workpack remains open because it has no tracked proof bundle; rerun after installing dependencies and record the actual result.

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

## Status

- `blocked / proof-present`
- Proof root: `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/`

## Execution truth

- Development and production config are separate and keep explicit non-wildcard origins.
- Secret values remain out of repo.
- Both scoped deploy dry-runs block before any remote publish step.
- Post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke are therefore still blocked.
- Rollback stays explicit but manual-required because no deploy artifact or published version was produced.

## Exact blocker set

- Missing billing-domain runtime boundary modules:
  - `packages/billing-domain/src/billing-checkout-portal-boundary.js`
  - `packages/billing-domain/src/billing-referral-boundary.js`
  - `packages/billing-domain/src/billing-support-admin-api-boundary.js`
  - `packages/billing-domain/src/billing-account-runtime-boundary.js`
  - `packages/billing-domain/src/billing-support-admin-runtime-boundary.js`
- Placeholder-backed resource identifiers remain in both configs.
- `AUTH_ADAPTER_MODE = "account-auth-adapter-manual-required"` remains active in both configs.
- The current deploy scripts emit `--env` warnings because they do not point at matching `[env.*]` sections in the selected config file.

## Validations run

- `cmd /c npm --prefix infra/cloudflare run deploy:dev -- --dry-run`
- `cmd /c npm --prefix infra/cloudflare run deploy -- --dry-run`
- PowerShell config gate for explicit non-wildcard origin, placeholder IDs, and manual-required markers

## No-claim boundary

- This workpack does not prove development deployment.
- This workpack does not prove production deployment.
- This workpack does not prove environment promotion.
- This workpack does not prove rollback execution.
- This workpack does not prove payment readiness, portal readiness, or account authority.

## Failure conditions

- Reject production wildcard origins.
- Reject deploy claims without rollback notes.
- Do not treat deploy commands as deployment proof.
