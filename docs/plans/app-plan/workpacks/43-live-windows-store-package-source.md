# WP43 - Live Windows Store Package Source

## Scope

Cross-record the shared app/game WP43 live Windows packaged-app manifest source
for the native app plan.

## Expected Outcome

- Native app plan records bounded Windows packaged-app inventory evidence from the shared app/game source.
- Store/UWP rows are inventory-only.
- Source references stay hashed/opaque.
- Runtime, foreground, policy, adapter execution, broad blocking, and platform support remain out of scope.

## Proof

Proof is shared with:

```text
output/app-game-plan-proof/43-live-windows-store-package-source
```

Native app cross-record proof lives in:

```text
output/app-plan-proof/43-live-windows-store-package-source
```

## No-Claim Boundaries

This is core packaged-app inventory evidence only. It does not add registry
crawling, service capture, portal UI, policy consumption, adapter execution,
broad app blocking, or platform support claims.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because this
does not move native app product status without service, portal, policy, and
adapter proof.

## Execution Detail

Minimum context:

- `docs/plans/app-game-plan/workpacks/43-live-windows-store-package-source.md`
- `docs/features/app-install-purchase-approval.md` only if store approval is touched.

Expected tests/proof names:

- `app-plan.wp43.store-package-inventory`
- `app-plan.wp43.opaque-source-refs`
- `app-plan.wp43.no-install-approval-claim`
- `app-plan.wp43.no-runtime-claim`

Failure conditions:

- Store package inventory is treated as install/purchase interception, usage proof, or platform enforcement.
