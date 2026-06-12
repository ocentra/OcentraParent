# WP108 Timer Service Read API Response Consumer Parent-Surface Handoff

Native app cross-record for the shared app/game WP108 slice.

## Scope

Consume WP107 native app response-consumer handoff rows and record the
parent-surface proof still required before parent-visible app surface
consumption can be claimed.

## Non-Goals

- No package export or public manifest claim.
- No service runtime, response-consumer implementation, parent-surface runtime,
  portal UI, protocol, Rust mirror, adapter dispatch, child delivery, broad app
  blocking, platform enforcement, or raw private source-row claim.

## Done Signal

The shared WP108 proof pack is cross-recorded in the native app checklist and
keeps app-specific parent-surface runtime and rendering claims false.

## Execution Detail

Minimum context:

- `docs/plans/app-plan/WORKPACK_INDEX.md`
- `docs/plans/app-game-plan/workpacks/108-timer-service-read-api-response-consumer-parent-surface-handoff.md`
- `docs/plans/portal-ux-household-surfaces-plan/AGENTS.md`

Owner boundary:

- This workpack hands off response-consumer proof requirements to parent-surface owners.
- It does not prove parent rendering, service runtime readiness, package export, or adapter execution.
- Portal UX owns visual consumption; app/app-game service owners own source response readiness.

Required output:

- Which response fields are ready for parent-surface consumption.
- Which parent-surface states remain missing.
- Which proof artifact closes the handoff and which artifact remains open.

Expected tests/proof names:

- `app-plan.wp108.response-consumer-handoff`
- `app-plan.wp108.parent-surface-missing-proof`
- `app-plan.wp108.no-rendering-claim`
- `app-plan.wp108.cross-plan-checklist-sync`

Failure conditions:

- A future worker treats a handoff row as rendered portal UI proof.
- Parent-visible readiness is claimed without screenshot/state proof and service data proof.
