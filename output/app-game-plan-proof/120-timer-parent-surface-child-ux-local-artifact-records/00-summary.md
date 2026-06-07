# WP120 Timer Parent-Surface Child UX Local Artifact Records

## Summary

WP120 adds structured child UX local artifact records to the shared native app
plus native game timer parent-surface read model. The records are derived from
ready app/game action-result rows that already have both child reason and child
status references.

## Implementation

- Rust protocol now includes
  `AppGameTimerParentSurfaceChildUxLocalArtifactRecord`.
- Agent service derives record rows from app/game approval action results and
  keeps raw target executable/package values out of the record.
- Agent-protocol-domain validates the structured record and rejects delivery,
  adapter, platform, or raw-private-source overclaims.
- Portal-domain renders artifact record source ids and target domains in the
  timer parent-surface summary.

## No-Claim Boundary

- Child delivery remains unclaimed.
- Notification delivery remains unclaimed.
- Adapter dispatch remains unclaimed.
- Broad blocking and platform enforcement remain unclaimed.
- Raw private source rows remain excluded.
- Durable service export and package export remain unclaimed.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns that checklist churn.
