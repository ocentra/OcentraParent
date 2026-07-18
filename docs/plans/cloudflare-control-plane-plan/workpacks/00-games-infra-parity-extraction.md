# Workpack 00: Games Infra Parity Extraction

## Goal

Extract the reusable games Cloudflare control-plane pattern and reduce it to Parent-only needs.

## Current status

`blocked / proof-required`

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

## Execution truth

- The parity extraction is docs/proof-only in this packet.
- `GAMES_INFRA_PARITY_MAP.md` now carries exact keep/adapt/strip decisions for the reusable games Cloudflare module pattern.
- `SOURCE_SURFACE_STATUS_MATRIX.md` now records the parity map and parent module spec as real source-of-truth surfaces instead of leaving WP00 implicit.
- The proof root `output/cloudflare-control-plane-plan-proof/00-games-infra-parity-extraction/` is an expected output path in the plan, but no tracked proof root exists in this checkout.
- The required docs validation is currently blocked outside WP00 because `npm run format:check` fails on repo-wide Prettier drift across 1010 files, including many untouched `apps/portal`, `packages/schema-domain`, `scripts/test`, and `vendor` paths.

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

## No-claim boundary

- This workpack does not claim Cloudflare runtime readiness, account authority, trusted-device authority, deployment readiness, or payment readiness.
- This workpack does not claim the reduced Parent module tree is fully implemented from docs alone.
- This workpack only records that the games module pattern reduction target is Parent-safe and keeps the carried non-goals explicit.
- This workpack is not validation-green while the repo-wide docs formatting gate remains red outside the packet.
