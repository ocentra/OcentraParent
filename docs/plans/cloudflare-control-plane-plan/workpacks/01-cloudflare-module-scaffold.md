# Workpack 01: Cloudflare Module Scaffold

## Goal

Keep the repo-local `infra/cloudflare/` module shape honest without overclaiming runtime completion.

## First-touch surface

- `infra/cloudflare/package.json`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)

## Output files

- `infra/cloudflare/`
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/01-cloudflare-module-scaffold/`

## Acceptance

- The module tree exists.
- Package scripts are explicit.
- The concentrated runtime surface versus placeholder subdirectories is explicit.

## Proof IDs

- `cloudflare-control.module-exists`
- `cloudflare-control.package-scripts`

## Validation

- Scoped validation: `npm --prefix infra/cloudflare run lint`
- Architecture validation: `npm run lint:architecture -- --files infra/cloudflare`

## Negative cases

- Reject fake `implemented` claims from empty files or README-only directories.

## Failure conditions

- Do not claim module completion from directory existence alone.
