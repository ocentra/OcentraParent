# WP74 Native App Source Freshness Policy Consumption

Cross-recorded from
`docs/plans/app-game-plan/workpacks/74-source-freshness-policy-consumption.md`.

This proof keeps native app policy readiness separate from native game policy
readiness while using the same low-level app/game `sourceStatusRows` evidence
spine. Native app policy compile is allowed only when inventory, runtime, and
foreground rows are fresh and evidence-backed. Native game rows additionally
require launcher evidence.

No portal UI, child UX, adapter dispatch, broad app blocking, platform
hard-control support, raw private source rows, or child-device mutation is
claimed by this slice.
