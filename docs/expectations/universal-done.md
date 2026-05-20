# Universal Done Definition

A feature is done only when every relevant expectation is true.

## Required

- Contracts exist before runtime consumers depend on them.
- TypeScript runtime validation uses Effect Schema.
- Rust protocol structs mirror shared contract shapes when crossing the Rust boundary.
- Source of truth is clear.
- Runtime apps do not own protocol strings, route ids, field names, event names, policy ids, or display text literals.
- No naked domain strings are introduced.
- No test doubles are introduced.
- Tests prove behavior with real parsers, real storage, real service boundaries, or real UI automation.
- Failure paths are specified and tested where feasible.
- Dev logs or status surfaces expose enough information to debug without guessing.
- Docs are updated when a feature changes product behavior, architecture, release behavior, or platform claims.
- Local validation appropriate to the change passes.

## Merge Readiness

Before PR merge, the feature must have:

- Focused local tests for changed packages/crates.
- Real integration or E2E tests for changed runtime behavior.
- Full local validation and build unless the change is explicitly docs-only and the repo gate already confirms docs-only scope.
- CI green on the PR branch.

## Not Done

A feature is not done if it only compiles, only renders a UI shell, only updates docs, only adds a contract without using it where the requested behavior needs runtime proof, or only demonstrates behavior through fake data.
