# WP42 - Service Windows Inventory Capture Bridge

## Scope

Cross-record the shared app/game WP42 service Windows shortcut inventory capture
bridge for the native app plan.

## Expected Outcome

- Native app plan records service-captured Windows shortcut inventory evidence through the shared app/game bridge.
- Rows remain inventory-only and can be queried through existing read-model paths.
- Runtime, foreground, policy, adapter execution, broad blocking, and platform support are not claimed.

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

## Execution Detail

Minimum context:

- `docs/plans/app-plan/workpacks/41-live-windows-inventory-source.md`
- `docs/plans/app-game-plan/workpacks/42-service-windows-inventory-capture-bridge.md`

Expected tests/proof names:

- `app-plan.wp42.service-inventory-capture`
- `app-plan.wp42.inventory-read-model`
- `app-plan.wp42.no-runtime-claim`
- `app-plan.wp42.no-policy-claim`

Failure conditions:

- Service-captured inventory is used as proof of active usage, policy readiness, or parent UI completion.
