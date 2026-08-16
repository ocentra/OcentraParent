# browser-plan Instruction

## Verdict

`partial`. One WP01 repair is coherent; the plan-level closure audit still fails and most proof roots are missing.

## Assign first

`browser-wp01-foundation-cleanup`:

- finish `packages/browser-domain` re-export cleanup;
- keep WP01 proof root coherent;
- rerun full browser-domain tests/typecheck and scoped architecture.

## Then

1. `browser-wp03-05-inventory-platform-proof`: inventory/platform proof and required artifact repair.
2. `browser-managed-runtime-chain`: WP06-WP14 managed profile/runtime/intervention proof.
3. `browser-policy-intervention-enforcement`: WP15-WP21, after enforcement contract boundaries are ready.

## Coordinate with

- `v0-8-enforcement-control-plan` for final WP19/WP20 enforcement claims.
- `policy-control-plane-plan` for browser policy compiler/preview.
- `network-plan` and `app-game-plan` where browser evidence feeds broader surfaces.

## Do not

- Do not count browser-domain empty category folders as coverage.
- Do not ship raw browser-profile artifacts as final proof-safe evidence.
- Do not claim unmanaged exact URL control or Apple Screen Time proof without external artifacts.
