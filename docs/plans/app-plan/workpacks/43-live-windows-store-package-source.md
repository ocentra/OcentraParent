# WP43 - Live Windows Store Package Source

## Scope

Cross-record the shared app/game WP43 live Windows packaged-app manifest source
for the native app plan.

## Implementation

- Reuses the shared app/game `agent-core` source module.
- Parses bounded `AppxManifest.xml` evidence into native app/game store-package
  inventory rows.
- Converts rows into journal events with hashed source refs and without
  runtime, foreground, policy, or adapter claims.

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
