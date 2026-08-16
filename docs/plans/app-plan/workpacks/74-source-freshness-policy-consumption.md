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

## Execution Detail

Minimum context:

- `docs/plans/app-plan/AGENTS.md`
- `docs/plans/app-game-plan/workpacks/74-source-freshness-policy-consumption.md`
- `docs/plans/policy-control-plane-plan/AGENTS.md`
- `docs/features/policy-schedules-approvals.md`

Owner boundary:

- This workpack only records when native app source freshness is sufficient for policy preview consumption.
- Policy source truth belongs to `policy-control-plane-plan`.
- Runtime enforcement belongs to `v0-8-enforcement-control-plan` and domain adapter plans.

Required output:

- Freshness prerequisites for inventory/runtime/foreground rows.
- Native app versus native game difference.
- Manual-required state for missing source evidence.
- Handoff to policy preview and enforcement owners.

Expected tests/proof names:

- `app-plan.wp74.freshness-gate`
- `app-plan.wp74.missing-source-manual-required`
- `app-plan.wp74.native-app-not-game`
- `app-plan.wp74.no-enforcement-claim`

Failure conditions:

- Policy compile proceeds from stale or private-only source rows.
- This workpack claims child mutation, adapter dispatch, broad blocking, or portal UI completion.
