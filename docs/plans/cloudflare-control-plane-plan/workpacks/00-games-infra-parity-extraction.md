# Workpack 00: Games Infra Parity Extraction

## Goal

Extract the reusable games Cloudflare control-plane pattern and reduce it to Parent-only needs.

## First-touch surface

- `GAMES_INFRA_PARITY_MAP.md`

## Read inputs

- [PLAN_STATE.md](../PLAN_STATE.md)
- [GAMES_INFRA_PARITY_MAP.md](../GAMES_INFRA_PARITY_MAP.md)
- [PARENT_CLOUDFLARE_MODULE_SPEC.md](../PARENT_CLOUDFLARE_MODULE_SPEC.md)

## Output files

- [GAMES_INFRA_PARITY_MAP.md](../GAMES_INFRA_PARITY_MAP.md)
- [SOURCE_SURFACE_STATUS_MATRIX.md](../SOURCE_SURFACE_STATUS_MATRIX.md)
- `output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/`

## Acceptance

- Keep/adapt/strip decisions are explicit.
- Parent-required bindings, tests, and scripts are reduced cleanly.
- No game-only runtime concern leaks into Parent.

## Proof IDs

- `cloudflare-control.module-exists`

## Validation

- Docs validation: `npm run format:check`

## Negative cases

- Reject copied game economy or Solana assumptions.
- Reject asset/archive/storage expansion beyond Parent needs.

## Failure conditions

- Do not let this workpack imply payment runtime readiness.
