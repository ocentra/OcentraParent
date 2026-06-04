# WP37 - Service Foreground Capture Bridge

## Scope

Cross-record the shared app/game WP37 service foreground capture bridge for the
native app plan.

The native app plan gains service-backed optional foreground row capture through
the shared app/game read-model path, without claiming product-complete native app
foreground control.

## Implementation

- Reuse the shared app/game active-window foreground source.
- Append the optional foreground journal event in `agent-service` bounded
  activity capture.
- Preserve no-content, no-policy, no-adapter, and unsupported-platform
  boundaries.

## Proof

Proof artifacts live in:

```text
output/app-plan-proof/37-service-foreground-capture-bridge
```

The authoritative implementation proof is the shared app/game workpack:

```text
docs/plans/app-game-plan/workpacks/37-service-foreground-capture-bridge.md
output/app-game-plan-proof/37-service-foreground-capture-bridge
```

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. Native app
product status remains blocked on portal, policy, adapter, and platform proof.
