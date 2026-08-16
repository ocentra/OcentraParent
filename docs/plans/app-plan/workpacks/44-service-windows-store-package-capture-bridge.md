# WP44 - Service Windows Store Package Capture Bridge

## Scope

Cross-record the shared app/game WP44 service Windows packaged-app manifest
capture bridge for the native app plan.

## Expected Outcome

- Native app plan records service-captured Windows Store/UWP package inventory through the shared app/game bridge.
- Rows remain inventory-only and read-model-visible.
- Runtime, foreground, policy, adapter execution, broad blocking, and platform support remain out of scope.

## Proof

Proof is shared with:

```text
output/app-game-plan-proof/44-service-windows-store-package-capture-bridge
```

Native app cross-record proof lives in:

```text
output/app-plan-proof/44-service-windows-store-package-capture-bridge
```

## No-Claim Boundaries

This is service-captured Store/UWP inventory evidence only. It does not add
registry crawling, portal UI, policy consumption, adapter execution, broad app
blocking, or platform support claims.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because this
does not move native app product status without portal, policy, and adapter
proof.

## Execution Detail

Minimum context:

- `docs/plans/app-plan/workpacks/43-live-windows-store-package-source.md`
- `docs/plans/app-game-plan/workpacks/44-service-windows-store-package-capture-bridge.md`

Expected tests/proof names:

- `app-plan.wp44.service-store-package-capture`
- `app-plan.wp44.inventory-read-model`
- `app-plan.wp44.no-install-approval-claim`
- `app-plan.wp44.no-policy-or-adapter-claim`

Failure conditions:

- Store package capture is used as proof of purchase interception, approval flow, usage, or enforcement.
