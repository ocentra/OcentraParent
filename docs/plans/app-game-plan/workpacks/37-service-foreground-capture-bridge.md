# WP37 - Service Foreground Capture Bridge

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
