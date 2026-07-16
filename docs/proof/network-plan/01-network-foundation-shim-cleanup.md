# 01 Network Foundation Shim Cleanup

## Scope

- `packages/parent-domain/src/network-flow.ts`
- `packages/parent-domain/src/network-contracts.ts`
- documentation/proof routing for `WP01` and `WP08`

## Intended Outcome

- retire the dead non-catalog parent-domain network frontage files because they are unpublished and have no live in-repo consumers
- keep the canonical network proof root current for this slice
- retire the stale parent-domain `./network-control-catalog` compatibility surface instead of inventing a replacement API

## Exact Decision And Resolution

- canonical control-catalog ownership already lives in `@ocentra-parent/network-domain`:
  - `./network-control-catalog`
  - `./network-control-catalog-data`
  - `./network-control-catalog-metadata`
  - `./network-control-catalog-schema`
- repo search found no live in-repo consumers of `@ocentra-parent/parent-domain/network-control-catalog`
- the parent-domain compatibility surface was therefore retired by:
  - removing `./network-control-catalog` from `packages/parent-domain/package.json`
  - deleting the four one-line parent-domain control-catalog shim files
- this slice still does not widen into a new public API design for the control-catalog surface

## Artifact Root

```text
output/network-plan-proof/01-network-foundation-shim-cleanup/
```

## Validation Plan

- touched-file architecture gate on the changed parent-domain shim files
- `@ocentra-parent/network-domain` focused unit tests
- `@ocentra-parent/agent-protocol-domain` `network-runtime-events.test.ts`

## Current Result

- `packages/parent-domain/src/network-flow.ts` and `packages/parent-domain/src/network-contracts.ts` were retired because `@ocentra-parent/parent-domain` does not publish `./network-flow` or `./network-contracts`, and no live in-repo consumers were found for those parent-domain paths.
- `packages/parent-domain/package.json` no longer publishes `./network-control-catalog`.
- the parent-domain control-catalog shim family was deleted instead of rewritten into another compatibility layer.
- `@ocentra-parent/network-domain` remains the only published owner for the `network-flow`, `network-contracts`, and control-catalog subpaths.
- `@ocentra-parent/network-domain` focused unit tests pass.
- `@ocentra-parent/parent-domain` direct typecheck passes after retiring the dead frontage files.
- the wider `@ocentra-parent/parent-domain` `lint:exec` run is blocked by unrelated pre-existing lint in `packages/parent-domain/tests/logging/parent-domain-logger-consumer.test.ts`.
- no remaining contradiction exists inside the exact parent-domain network frontage boundary for this slice.

## Result Rule

This slice is now honest because:

1. the unpublished parent-domain `network-flow` and `network-contracts` frontage files were retired instead of being preserved as dead compatibility surfaces,
2. the stale `./network-control-catalog` parent-domain compatibility surface was retired rather than replaced, and
3. the slice stayed inside the exact parent-domain TypeScript boundary without widening into Rust, portal, or platform work.
