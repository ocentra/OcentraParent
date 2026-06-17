# WP07 Orphaned Legacy Surface Inventory

## Objective

Find old, weakly-owned, pre-eventing, transitional, or shadow source paths that still exist in the repo and can mislead plan workers.

This is a structural audit workpack. It does not move code by itself.

## Scope

Inspect broad or historically overloaded surfaces first:

- `packages/parent-domain/src/**`
- `packages/portal-domain/src/**`
- `packages/agent-protocol-domain/src/**`
- `crates/agent-core/src/**`
- `crates/agent-service/src/**`
- `crates/agent-protocol/src/**`
- `scripts/test/**`
- plan proof/index docs that still reference old owners

## Classification

| Class | Meaning |
| --- | --- |
| active owner | Current source of truth for behavior. |
| adapter/frontage | Allowed bridge, display, or transport surface. |
| legacy shim | Kept only for compatibility; must not drive proof or ownership. |
| orphan | No current consumer or proof route justifies the file. |
| pre-eventing shadow | Old local implementation parallel to the eventing/runtime model. |
| stale proof wrapper | Script or doc still pointing at old owner/test path. |

## Output table

| File / surface | Class | Current consumer | Preferred owner | Risk | Action |
| --- | --- | --- | --- | --- | --- |

## Acceptance

- Parent-domain and portal-domain shadow exports are listed explicitly.
- Old proof wrappers that target wrong owner packages are listed.
- Pre-eventing or pre-runtime parallel implementations are identified before DRY extraction.
- Each action is one of: keep, convert to adapter, move, delete, or block pending owner decision.

## Failure conditions

- Treating old source existence as proof of ownership.
- Moving code without a current consumer map.
- Deleting code before ownership and proof replacement are clear.
