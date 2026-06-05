# 48 Portal Source Freshness Surface

## Scope

Render the backend `sourceStatusRows` from the app-use and games read models in
the existing parent App/Game Sessions dashboard surface.

This workpack proves that the portal can display source row counts, fresh source
counts, source-kind capability state, latest observed timestamps, and evidence
ref counts from service-backed read-model payloads without adding backend
contracts or parsing raw evidence vectors.

It does not add a new backend source contract, policy consumption, adapter
execution, broad blocking, provider delivery, or cross-platform support claims.

## Implementation

- Carry nested `sourceStatusRows` from app-use and games read-model rows into the
  dashboard intent.
- Render source row and fresh source counts through the existing App/Game
  Sessions metric grid.
- Render source-kind capability/timestamp/evidence-ref summaries through the
  existing evidence drawer rows.
- Keep the dedicated SVG panel layout unchanged because another active lane owns
  `ParentPortalSvgSurface.tsx`.

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
output/app-game-plan-proof/48-portal-source-freshness-surface
```

## No-Claim Boundaries

- Source freshness rows summarize already-stored service evidence only.
- Portal rendering does not prove policy evaluator consumption, source adapter
  execution, broad app/game blocking, provider delivery, or platform support.
- Inventory source rows remain inventory-only and cannot become runtime or
  foreground usage without matching service evidence.
- Launcher source rows remain launcher evidence unless child-game proof is
  separately present.

## Product Doc Decision

`docs/product-capability-checklist.md` is updated because App/Game Sessions now
has parent-visible portal source freshness rendering proof. The capability
remains in progress because policy consumption, live provider quality, runtime
evaluation, production notifications, broad blocking, and cross-platform proof
remain open.
