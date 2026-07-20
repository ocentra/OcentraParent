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

## Status

- `blocked / proof-required`
- Proof root: expected output path `output/cloudflare-control-plane-plan-proof/11-deployment-and-environment-promotion/`

## Execution truth

- Development and production config are separate and keep explicit non-wildcard origins.
- Secret values remain out of repo.
- Both scoped deploy commands exit 0 at the explicit `--dry-run` boundary and intentionally perform no remote publish.
- Post-deploy `/health`, `/public/pricing`, and `/auth/billing/status` smoke are therefore still blocked.
- Rollback stays explicit but manual-required because no deploy artifact or published version was produced.

## Exact blocker set

- Dry-run command execution is green against source base `4573ab436` on 2026-07-20; deployment and promotion proof remain absent:
  - `npm --prefix infra/cloudflare run deploy:dev -- --dry-run`
  - `npm --prefix infra/cloudflare run deploy -- --dry-run`
- Each command completed local bundling, emitted the existing missing matching `[env.*]` warning, and stopped without publishing.
- Placeholder-backed resource identifiers remain in both configs.
- `AUTH_ADAPTER_MODE = "account-auth-adapter-manual-required"` remains active in both configs.
- The current deploy scripts emit `--env` warnings because they do not point at matching `[env.*]` sections in the selected config file.
- No promoted endpoint exists, so post-deploy smoke and rollback execution remain blocked rather than green.

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
