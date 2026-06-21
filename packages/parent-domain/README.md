# @ocentra-parent/parent-domain

`@ocentra-parent/parent-domain` is no longer the canonical owner for broad
parent/family schema contracts.

## Live ownership

- Behavioral/runtime code that still belongs locally in this package.
- The current live public subpath is
  `@ocentra-parent/parent-domain/parent-owned-local-export-runtime-executor`.

## Canonical schema ownership

Shared parent schema/value/read-model/proof contracts belong in
`@ocentra-parent/schema-domain`.

Current parent-owned local export schema ownership is centralized in:

- `packages/schema-domain/src/parent-owned-local-export-runtime.ts`
- `packages/schema-domain/src/parent-owned-local-export-runtime-values.ts`

The executor in this package consumes those centralized schema surfaces and
implements only the local filesystem/export runtime behavior.

## Package exports

- `./parent-owned-local-export-runtime-executor`: live behavioral export.

## Non-ownership boundary

This package does not own shared billing, network, enforcement, policy, or
other parent schema contracts for the current central-schema round. Those
contracts should stay in their canonical domain packages, usually
`@ocentra-parent/schema-domain`.
