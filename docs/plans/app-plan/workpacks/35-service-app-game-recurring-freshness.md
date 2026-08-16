# 35 Service App/Game Recurring Freshness

## Target State

The native app plan cross-records shared app/game WP35: agent-service startup
capture repeats bounded live native app/game runtime capture on a protocol-owned
cadence and keeps the existing journal/store/read-model path fresh without
claiming foreground, policy, portal, or adapter completion.

## Scope

- Reuse the shared app/game recurring capture cadence.
- Prove two service capture cycles append two live native app/game runtime rows.
- Prove foreground state remains not-claimed.
- Keep foreground source proof, portal freshness polish, policy consumption, and
  adapter execution as explicit gaps.

## Tests And Proof

- `cargo test -p ocentra-parent-agent-service activity_capture`
- `output/app-plan-proof/35-service-app-game-recurring-freshness`

## Done Signal

Native app runtime evidence now has recurring bounded service freshness through
the existing activity-capture path. Product status remains in progress until
foreground capture, portal source status, policy consumption, and platform
action proof are added.

## Execution Detail

Minimum context:

- `docs/plans/app-plan/workpacks/34-service-capture-app-game-live-process-bridge.md`
- `docs/plans/app-game-plan/workpacks/35-service-app-game-recurring-freshness.md`
- `docs/plans/policy-control-plane-plan/AGENTS.md`

Owner boundary:

- This workpack proves repeated capture cadence and freshness state.
- It does not prove user foreground activity, child policy effect, or parent visible readiness.

Required output:

- Freshness cadence expectation.
- Stale/missing capture state.
- Proof that repeated rows do not duplicate product claims.
- Handoff to source-gated policy preview.

Expected tests/proof names:

- `app-plan.wp35.recurring-capture`
- `app-plan.wp35.stale-source-state`
- `app-plan.wp35.no-foreground-claim`
- `app-plan.wp35.policy-preview-handoff`

Failure conditions:

- Fresh runtime rows are treated as active usage, policy enforcement, or UI completion.
