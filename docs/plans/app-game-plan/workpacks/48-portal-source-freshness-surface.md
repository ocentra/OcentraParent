# 48 Portal Source Freshness Surface

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `48 Portal Source Freshness Surface`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

## Live Code/Test Re-Audit - 2026-08-24

- The production path now exists: service-backed `sourceStatusRows` flow through
  `app-game-dashboard-intent.ts`, the split source-panel intent, and the mounted
  `ParentPortalSvgSurface.tsx` into the App/Game Sessions route.
- The rendered surface exposes source row/fresh counts plus capability,
  timestamp, and evidence summaries. It does not mint policy or source
  authority.
- The named `apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`
  is not checked in. The existing Playwright route scaffold asserts only source
  headings/count labels, not empty/stale/degraded behavior.
- Production source is present; expected-test writing, focused execution,
  proof, checklist acceptance, READY, and DONE remain open.

## Graph ownership correction — 2026-08-25

WP48 owns the dashboard source-row seam in
`vendor/ocentra-parent-core-ui/AppPages/ParentPortal/app-game-dashboard-intent.ts`
and the shared focused test packet
`apps/portal/tests/activity-ui-app-game-dashboard-intent.test.ts`. WP63 is a
consumer of that seam (`WP63 -> WP48`) and owns only the source-panel intent.
The bridge, live-activity state, activity intent, SVG surface, portal surface,
route, and route-scaffold roots are not WP48/WP63 implementation claims; the
broader mounted surface remains with the existing App/Game dashboard owner.
This is graph/docs routing only; the focused test and completion gates remain
open.
