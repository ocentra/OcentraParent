# WP48 - Portal Source Freshness Surface

## Scope

Cross-record the shared app/game WP48 portal source freshness surface for the
native app plan.

The shared App/Game Sessions dashboard now renders service-backed native
app/game `sourceStatusRows` as source row counts, fresh source counts, and
source-kind evidence drawer summaries.

Code-pass status: the existing intent and dashboard render path is now mapped
to its actual vendor portal owners; validation and proof remain deferred.

## Implementation

- Consume app-use and games read-model `sourceStatusRows` in the portal
  dashboard intent.
- Surface source row count and fresh source count in the existing dashboard
  metrics.
- Surface source-kind capability, latest observed timestamp, and evidence ref
  counts in the existing evidence drawer.
- Do not change backend contracts, policy consumption, adapter execution, broad
  blocking, provider delivery, or platform support.

## Proof

- `cmd /c npm exec --workspace @ocentra-parent/portal -- vitest run tests/activity-ui-app-game-dashboard-intent.test.ts`
- `node scripts/test/app-game-source-freshness-portal-proof.mjs`
- `cmd /c npm run test:e2e --workspace @ocentra-parent/portal`
- `cmd /c npm run format:check`
- `cmd /c npm run lint:schema-boundaries`
- `git diff --check`
- `cmd /c npm run lanes:guard`
- `cmd /c npm run hub:guard`

Proof artifacts live in:

```text
output/app-plan-proof/48-portal-source-freshness-surface
```

## No-Claim Boundaries

- This is portal rendering proof for existing read-model data.
- It does not prove native app policy runtime consumption, provider execution,
  app blocking, installer/store integration, or cross-platform adapter support.

## Product Doc Decision

`docs/product-capability-checklist.md` is updated through the shared app/game
capability row. The native app plan remains in progress until policy runtime,
provider delivery, broad blocking, and platform support are proved.
