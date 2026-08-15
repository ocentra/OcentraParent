# WP49 - Category/Risk Policy Routing

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP49 - Category/Risk Policy Routing`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Route category, risk, and game-context candidates into the Rust-owned app/game
policy target compiler boundary without treating those candidates as source
truth or enforcement authority.

This workpack proves Rust-owned category/risk routing can carry active
category proof, confidence, candidate source, target kind, policy action, and
supporting evidence refs into soft/manual compiler inputs.

It does not add live classifier/provider execution, service runtime policy
evaluation, portal UI, notifications, child request delivery, adapter execution,
broad blocking, or platform support.

## Implementation

- Add a Rust-owned category/risk policy-routing contract in `ocentra-app-game-core`.
- Map native app categories, risk candidates, native game categories, and game
  context signals to matching app/game policy target kinds.
- Require active category proof and supporting evidence refs before a route is
  compile-ready.
- Require local AI category routes to cite an AI digest ref.
- Keep manual-review candidates manual-required.
- Preserve the no-adapter boundary with `adapterDispatchState:
not-dispatched`.

## Current Status - Phase 1 and Phase 2 Complete; Phase 3 Open

The Rust-owned routing implementation landed at `14ef6fbf4`. It maps seven
category/risk/game-context candidate families to WP19 compiler target kinds,
requires active category proof and supporting evidence, binds local-AI digest
refs, rejects invalid confidence and stale/missing proof, and converts hard
actions into manual compiler inputs while preserving `not-dispatched`.

Checked-in contract tests cover the complete mapping matrix and all named
negative/no-claim boundaries. Focused validation is green:

- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`
- `cargo test -p ocentra-app-game-core --test contract` (`52 passed`)
- focused Enforcer `architecture-policy`, `source-shape`, `required-tests`,
  `no-test-doubles`, and `no-naked-domain-strings`

Phase 3 retained proof and plan-level precommit/CI remain open. Runtime/service
consumption remains outside this workpack.

## Expected Focused Validation

- `cargo clippy -p ocentra-app-game-core --all-targets -- -D warnings`
- `cargo test -p ocentra-app-game-core --test contract category_risk_policy_routing`
- focused Enforcer routing for the exact Rust source and contract test files
- `npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/49-category-risk-policy-routing
```

## No-Claim Boundaries

- Category and risk candidates remain policy inputs, not final decisions.
- Risk candidates cannot request hard adapter actions.
- Local AI category routes require digest refs and cannot dispatch adapters.
- Stale category proof is rejected before compile-ready routing.
- Runtime service consumption, portal rendering, child UX, notifications,
  platform adapters, and broad blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` remains unchanged. WP49 moves category
quality and policy-routing proof forward at the Rust compiler boundary, but
product status should not move until runtime service policy consumption,
portal category/risk UI, live classifier/provider execution, notification/child
request UX, adapter execution, and platform proof are complete.
