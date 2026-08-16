# WP37 - Service Foreground Capture Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP37 - Service Foreground Capture Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Scope

Wire the WP36 app/game foreground source into the existing bounded
`agent-service` activity-capture journal/store path.

This workpack does not add portal UI, foreground transition subscriptions,
policy consumption, adapter execution, broad blocking, or platform support
claims.

## Implementation

- Export the app/game live foreground journal-event helper from `agent-core`.
- Append the optional foreground event during service activity capture when the
  active-window source is available.
- Keep unsupported or unavailable foreground capture as no event, not as a false
  support claim.
- Update service capture tests to prove the read model can return optional
  foreground rows while preserving no-content boundaries.

## Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `cargo test -p ocentra-parent-agent-core foreground`
- `git diff --check`

Proof artifacts live in:

```text
output/app-game-plan-proof/37-service-foreground-capture-bridge
```

## No-Claim Boundaries

- Foreground evidence is not raw content.
- Optional service capture is not a foreground transition subscription.
- Service read-model exposure is not portal freshness polish.
- No policy decision, adapter execution, broad app blocking, or platform support
  claim is added.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP37 moves
foreground evidence into bounded service capture only; product status should not
move until portal, policy, adapter, and platform proof exist.
