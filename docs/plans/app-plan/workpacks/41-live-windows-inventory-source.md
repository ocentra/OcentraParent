# WP41 - Live Windows Inventory Source

## Scope

Cross-record the shared app/game WP41 core live Windows shortcut inventory
source for the native app plan.

## Expected Outcome

- Native app plan records bounded Windows shortcut inventory evidence from the shared app/game source.
- Rows are inventory-only and privacy-safe.
- Path-derived references remain hashed/opaque in proof and downstream docs.
- Runtime, foreground, policy, and adapter authority remain out of scope.

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

## Execution Detail

Minimum context:

- `docs/plans/app-game-plan/workpacks/41-live-windows-inventory-source.md`
- `docs/plans/data-custody-storage-plan/AGENTS.md` only if retention/export is touched.

Expected tests/proof names:

- `app-plan.wp41.windows-shortcut-inventory`
- `app-plan.wp41.opaque-source-refs`
- `app-plan.wp41.inventory-not-runtime`
- `app-plan.wp41.no-product-status-move`

Failure conditions:

- Inventory presence is treated as usage, foreground, category, or enforceable app policy.
