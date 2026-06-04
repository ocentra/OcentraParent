# WP42 - Service Windows Inventory Capture Bridge

## Scope

Wire the WP41 live Windows shortcut inventory source into the existing
agent-service activity-capture journal/store path.

This workpack proves that bounded Start Menu shortcut inventory events can be
captured by the service and projected into the existing app/game read model as
inventory-only rows.

It does not add registry crawling, Store package enumeration, portal UI, source
freshness rows, policy consumption, adapter execution, broad blocking, or
platform support claims.

## Implementation

- Export the core live Windows inventory journal-event helper for service use.
- Add a protocol-owned bounded inventory capture limit.
- Extend the service app/game activity-capture event list with live inventory
  events.
- Add a deterministic service test that injects a temporary shortcut root and
  proves encrypted journal replay plus SQLite read-model projection.
- Keep environment-dependent default Windows shortcut counts bounded instead of
  treating read-model row limits as event counts.

## Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `cargo test -p ocentra-parent-agent-service app_game`
- `cargo test -p ocentra-parent-agent-core live_inventory`
- `cargo test -p ocentra-parent-agent-core app_game`
- `cargo test -p ocentra-parent-agent-protocol app_game`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-game-plan-proof/42-service-windows-inventory-capture-bridge
```

## No-Claim Boundaries

- Service capture stores shortcut inventory as inventory evidence only.
- Shortcut inventory does not prove runtime use, foreground use, content
  knowledge, registry crawling, Store package enumeration, portal UI, policy
  decisions, adapter execution, broad app blocking, or platform support.
- Raw shortcut paths remain hashed before becoming source or desktop-entry refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP42 moves
service capture proof forward, but product status should not move until portal
freshness, policy consumption, live platform proof, and adapter boundaries are
finished.
