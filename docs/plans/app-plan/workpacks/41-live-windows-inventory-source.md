# WP41 - Live Windows Inventory Source

## Scope

Cross-record the shared app/game WP41 core live Windows shortcut inventory
source for the native app plan.

## Implementation

- Reuses shared app/game `agent-core` source helpers.
- Maps bounded Windows Start Menu shortcut scans into native app/game
  inventory-only rows.
- Hashes path-derived source and desktop-entry refs before journal projection.

## Proof

Proof is shared with:

```text
output/app-game-plan-proof/41-live-windows-inventory-source
```

Native app cross-record proof lives in:

```text
output/app-plan-proof/41-live-windows-inventory-source
```

## No-Claim Boundaries

This is core inventory evidence only. It does not add registry crawling, Store
package enumeration, service capture, portal UI, policy consumption, adapter
execution, broad app blocking, or platform support claims.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because this
does not move native app product status without service, portal, policy, and
adapter proof.
