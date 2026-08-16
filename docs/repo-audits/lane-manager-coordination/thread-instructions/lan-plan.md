# lan-plan Instruction

## Verdict

`partial`. Slice A reconciliation is green only for the narrow scope. Many implementation/proof rows remain open.

## Assign first

`lan-b1-proof-regeneration`:

- regenerate LAN proof for authoritative 01-20 scope;
- run named `v0-9` proof scripts;
- produce current `output/lan-plan-proof/*` artifacts;
- keep 21-25 frozen and out of completion claims.

## Then

1. `lan-b2-test-truth-repair`: stop counting empty category folders; add real tests where needed.
2. `lan-c1-protocol-service-truth-repair`: fix Rust/protocol/service LAN proof paths.
3. Open implementation cluster: 05/06/07/08/09/11/17.
4. Runtime/physical proof cluster: 15/16/18/19/20.

## Coordinate with

- `eventing-plan` for WP10 authority wording.
- `portal-ux-household-surfaces-plan` for LAN consumer read-model proof.
- `remote-access-plan` for route/transport only later.

## Do not

- Do not count empty `packages/lan-domain` test folders.
- Do not claim second-device/physical/router proof without artifacts.
- Do not reopen frozen 21-25 as completion blockers.
