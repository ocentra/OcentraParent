# WP42 - Service Windows Inventory Capture Bridge

## Scope

Cross-record the shared app/game WP42 service Windows shortcut inventory capture
bridge for the native app plan.

## Implementation

- Reuses the shared app/game service activity-capture bridge.
- Appends bounded live Windows shortcut inventory journal events into the
  encrypted journal and ActivityStore path.
- Projects native app/game inventory-only rows through existing app-use/games
  read models without runtime, foreground, policy, or adapter claims.

## Proof

Proof is shared with:

```text
output/app-game-plan-proof/42-service-windows-inventory-capture-bridge
```

Native app cross-record proof lives in:

```text
output/app-plan-proof/42-service-windows-inventory-capture-bridge
```

## No-Claim Boundaries

This is service-captured inventory evidence only. It does not add registry
crawling, Store package enumeration, portal UI, policy consumption, adapter
execution, broad app blocking, or platform support claims.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because this
does not move native app product status without portal, policy, and adapter
proof.
