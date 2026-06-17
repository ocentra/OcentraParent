# policy-control-plane-plan Instruction

## Verdict

`partial`. Core TS/Rust policy contracts are real; WP01/WP07/WP08 locally strong; WP03/WP04 proof bundles and WP06 route truth are not closed; WP02/WP05 depend on UI/assistant/product surfaces.

## Assign first

`policy-wp06-route-proof-truth-repair`:

- repair `PROOF_INDEX.md`, `PROOF_AND_TEST_INVENTORY.md`, stale `06-route-sync-proof.md`, and missing `PLAN_PROOF_MANIFEST.md`;
- make checked/open statuses match actual proof root.

## Then

1. `policy-wp03-compiler-proof-bundle`.
2. `policy-wp04-delivery-ack-audit-proof-bundle`.
3. `policy-wp02-wp05-rendered-ui-assistant-contracts` only after portal/assistant dependencies are assigned.
4. Move policy-counted Rust agent-protocol tests from `src` into crate `tests/` categories before final closure.

## Coordinate with

- `portal-ux-household-surfaces-plan` for authoring/preview UI.
- `ai-plan` / parent assistant surfaces for ask-parent overrides.
- `v0-8-enforcement-control-plan` for dispatch/enforcement consumers.

## Do not

- Do not call the whole plan done from WP01/WP07/WP08.
- Do not use overbroad portal workspace failures as policy proof.
- Do not hide agent-protocol-domain barrel debt under policy file-specific passes.
