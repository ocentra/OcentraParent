<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `PLAN_STATE.md`
> Kind: plan state and current gap summary.
> Read when: After this plan is selected and before opening workpacks.
> Stop rule: Continue only through `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the selected workpack.
> Proves: current routed plan state; code completion still requires the mapped source/tests and later validation.
> Proof rule: Keep this file aligned with `CODE_AUDIT.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and the engineering graph.

<!-- /agent-capsule -->

# Native Apps Plan State

## Current state

- Plan status: **Phase 1 audited; implementation incomplete**.
- Audit date: 2026-08-15.
- Authoritative code/test inventory: [CODE_AUDIT.md](CODE_AUDIT.md).
- Graph coverage: **95/95 reviewed workpack maps**.
- Bounded Phase 1: **80 complete, 15 incomplete**.
- Phase 2 focused tests/Enforcer: not run for this plan-wide audit.
- Phase 3 proof: not regenerated.

The 15 source/test-writing gaps are WP15, WP16, WP17, WP18, WP19, WP20,
WP26, WP48, WP49, WP62, WP63, WP64, WP65, and
WP102.

## Current ownership

```text
app-core
  app-only observation/runtime decision boundary

app-game-core
  shared app/game projections and handoff models; many are pure models

agent-protocol
  shared wire DTOs and authority/readiness/adapter contracts

agent-core + agent-service
  real Windows acquisition, recurring capture, journal/SQLite projection,
  read models/events, and scoped owned-process time-limit execution

schema + schema-domain
  Rust-generated cross-boundary TypeScript contracts

parent-runtime-core + portal
  parent-visible projections; never OS observation or enforcement authority
```

`packages/activity-domain`, `packages/parent-domain`,
`packages/agent-protocol-domain`, and `packages/text-domain` are not tracked
implementation owners in this checkout. Their paths in older workpacks are
migration history.

## Product truth

The Windows evidence spine is real through service capture and local
journal/SQLite read models. The product is not release-ready because durable
review/risk state, the native-app policy compiler/runtime composition, child
request UX, notification delivery/history, and complete parent
inventory/freshness surfaces are not written.

WP74-WP101 are mostly typed, tested no-claim projections. Their production
call-site inventory does not show live agent-service composition, so they must
not be promoted as runtime policy/timer support.

## Next implementation frontier

1. WP18/WP49 compiler and category/risk routing.
2. WP16/WP17 durable unknown/new-app review and live risk candidates.
3. WP19/WP20 time-budget runtime composition and child UX.
4. WP62-WP65 notification preference/status pipeline.
5. WP15/WP48/WP63 parent inventory/freshness UI.
6. WP26 load/performance harnesses.
7. WP102 implement or explicitly merge/retire into WP103.

## Completion boundary

Do not use this Phase 1 audit to mark the plan DONE. After code/test-writing
gaps close, run focused Phase 2 tests and Enforcer by touched risk surface.
Only then regenerate Phase 3 proof from a clean checkout and reconcile product
acceptance.
