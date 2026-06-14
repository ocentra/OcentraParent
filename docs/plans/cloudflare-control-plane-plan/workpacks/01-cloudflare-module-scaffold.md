# Workpack 01: Cloudflare Module Scaffold

## Goal

Create the repo-local `infra/cloudflare/` scaffold and file tree without overclaiming runtime readiness.

## First-touch surface

- `infra/cloudflare/package.json`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `infra/cloudflare/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `docs/proof/cloudflare-control-plane-plan/wp01-cloudflare-module-scaffold/`

## Acceptance

- The module tree exists.
- Package scripts are explicit.
- Source and test placeholders are marked scaffold-only.

## Proof IDs

- `cloudflare-control.module-exists`
- `cloudflare-control.package-scripts`

## Validation

- Docs or scaffold validation: `npm run format:check`

## Negative cases

- Reject fake “implemented” claims from empty files.

## Failure conditions

- Do not claim runtime code or tests from scaffold existence alone.
