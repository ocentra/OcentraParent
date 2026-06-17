# app-plan Instruction

## Verdict

`partial / routing repair`. The plan is stale and overlaps `app-game-plan`; do not implement broad runtime here before ownership is repaired.

## Assign first

`app-plan-truth-repair`:

- update docs/source index to real owners;
- delegate shared app/game runtime surfaces to `app-game-plan` where appropriate;
- identify app-only surfaces that remain genuinely separate;
- remove stale proof/root claims.

## Then

- `app-ownership-and-architecture-cleanup`: fix parent-domain app shims and architecture debt only after path ownership is locked.
- `app-rust-test-rehome`: move app-owned Rust tests from `src` into crate `tests/` categories.

## Coordinate with

- `app-game-plan` as first predecessor.
- `policy-control-plane-plan`, `eventing-plan`, and `v0-8-enforcement-control-plan` for policy/runtime/enforcement chain.

## Do not

- Do not duplicate app-game runtime work.
- Do not close app-plan from app-game proof roots.
- Do not write new app behavior into broad `parent-domain` surfaces.
