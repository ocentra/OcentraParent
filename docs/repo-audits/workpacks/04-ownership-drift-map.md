# WP04 Ownership Drift Map

## Objective

Find code and tests that landed in broad frontage packages or wrong crates instead of the narrow owner.

## Scope

Inspect broad aggregators first:

- `packages/parent-domain`
- `packages/portal-domain`
- `packages/agent-protocol-domain`
- `crates/agent-core`
- `crates/agent-protocol`

Then compare against narrow owner packages/crates:

- `packages/family-domain`
- `packages/setup-domain`
- `packages/data-custody-domain`
- `packages/policy-domain`
- `packages/tracking-domain`
- `packages/browser-domain`
- `packages/network-domain`
- `packages/app-game-domain`
- focused Rust crates such as `tracking-core`, `network-core`, `storage-custody-core`, `policy-control-core`, `child-runtime`, and `ocentra-eventing`.

## Classification

| Class | Meaning |
| --- | --- |
| owner | Source owns product/domain behavior. |
| adapter | Source translates between owner and caller. |
| frontage | Parent/portal/read-model display surface only. |
| misplaced | Source should move or be rewritten around an owner package/crate. |
| duplicate | Same behavior exists in more than one owner. |

## Output table

| File/surface | Current location | Class | Preferred owner | Reason | Action |
| --- | --- | --- | --- | --- | --- |

## Starting rules

- `parent-domain` should not become child runtime authority.
- `portal-domain` and `apps/portal` should not own runtime truth.
- `agent-protocol-domain` and `agent-protocol` should own transport/protocol shapes, not product logic.
- `agent-core` should own runtime composition/adapters; reusable engines should live in focused crates where practical.

## Acceptance

- Move candidates are listed before any source move.
- Adapter/frontage files are not counted as domain completion.
- Each plan report can be checked against the owner map.

## Failure conditions

- Moving source just to fix compile without preserving owner truth.
- Creating new broad frontage files for narrow-domain behavior.
- Treating portal screenshots as runtime ownership proof.
