# 01 Network Foundation Shim Cleanup

## Scope

- `packages/parent-domain/src/network-flow.ts`
- `packages/parent-domain/src/network-contracts.ts`
- documentation/proof routing for `WP01` and `WP08`

## Intended Outcome

- remove banned `export *` barrel syntax from the non-catalog parent-domain network shim files
- restore the canonical network proof root for this slice
- stop cleanly if the remaining contradiction is only the public `./network-control-catalog` surface

## Exact Remaining Contradiction

- `packages/parent-domain/package.json` still publishes `./network-control-catalog`
- the remaining parent-domain control-catalog shim family is still barrel-based:
  - `packages/parent-domain/src/network-control-catalog.ts`
  - `packages/parent-domain/src/network-control-catalog-data.ts`
  - `packages/parent-domain/src/network-control-catalog-metadata.ts`
  - `packages/parent-domain/src/network-control-catalog-schema.ts`
- this slice does not widen into a new public API design for the control-catalog surface

## Artifact Root

```text
output/network-plan-proof/01-network-foundation-shim-cleanup/
```

## Validation Plan

- touched-file architecture gate on the changed parent-domain shim files
- `@ocentra-parent/network-domain` focused unit tests
- `@ocentra-parent/agent-protocol-domain` `network-runtime-events.test.ts`

## Current Result

- `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` now pass the focused architecture gate.
- `@ocentra-parent/network-domain` focused unit tests pass.
- `@ocentra-parent/parent-domain` direct typecheck passes.
- the wider `@ocentra-parent/parent-domain` `lint:exec` run is blocked by unrelated pre-existing lint in `packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts`.
- the remaining control-catalog shim family still fails the focused architecture gate, which keeps the slice stopped on the single public-export contradiction described above.

## Result Rule

This slice is honest if either:

1. the non-catalog parent-domain network shims are converted off barrel syntax and the proof root is restored, or
2. the slice stops on the single `./network-control-catalog` public-export decision without widening into Rust, portal, or platform work.
