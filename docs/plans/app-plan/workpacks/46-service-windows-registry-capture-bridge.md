# WP46 - Service Windows Registry Capture Bridge

## Scope

Cross-record the shared app/game WP46 service capture bridge for native app
inventory.

This workpack proves that bounded Windows Uninstall registry inventory events
can be captured by the Rust service and projected into the existing native app
read-model path as inventory-only rows.

It does not add portal UI, source freshness rows, policy consumption, adapter
execution, broad blocking, or platform support claims.

## Implementation

- Reuse the shared app/game registry inventory source from WP45.
- Extend the service activity-capture path to append registry inventory events.
- Add deterministic service proof with a temporary `.reg` export root.
- Keep all source refs and executable/install/uninstall path refs opaque.

## Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `cargo test -p ocentra-parent-agent-core registry_inventory`
- `cargo test -p ocentra-parent-agent-core app_game`
- `cargo test -p ocentra-parent-agent-protocol app_game`
- `cargo fmt --all --check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/46-service-windows-registry-capture-bridge
```

## No-Claim Boundaries

- Registry service capture remains inventory evidence only.
- Registry evidence does not prove app usage, foreground state, content
  knowledge, policy decisions, adapter execution, broad app blocking, or
  platform support.
- Raw registry keys and install/uninstall paths remain hashed before becoming
  refs.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP46 moves
service capture proof forward, but product status should not move until portal
freshness, policy consumption, live platform proof, and adapter boundaries are
finished.
