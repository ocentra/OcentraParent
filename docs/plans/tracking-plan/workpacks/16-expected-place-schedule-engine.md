# WP16 Expected-Place Schedule Engine

## Purpose

Evaluate where the child device is expected to be based on recurring schedules,
calendar events, temporary windows, geofences, place categories, and routes.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Expected-place decisions cite schedule refs, expected-place rule refs, tolerance,
grace, evidence refs, and exception state.

## Tests And Proof

Proof root: `output/tracking-plan-proof/16-expected-place-schedule-engine/`

- `06-expected-place-proof.json`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test school, home-at-night, activity, calendar, and temporary trip rules.
- [ ] Test early/late/exit grace and distance tolerance.
- [ ] Test holiday exception suppression.
- [ ] Test low accuracy maps to check-in/ambiguous, not accusation.
- [ ] Keep parent policy as final action authority.

## Where We Are

This workpack has P0 contract proof and P1 fixture proof for deterministic
expected-place decision evaluation from `codex/tracking-plan-full-scope` under
the proof root below. Alert policy integration, exception/holiday integration,
platform behavior, and UI behavior are not claimed beyond the proof state
recorded in `proof-summary.json`, `06-expected-place-proof.json`, and the
implementation checklist.
Expected-place alert policy proof now maps expected-place policy decisions into
parent alert, child check-in, suppression, and manual-required UI-readiness
rows through `node scripts/test/tracking-expected-place-alert-policy-proof.mjs`.
It preserves schedule rule refs, policy decision refs, alert intent refs,
evidence refs, reason refs, audit refs, and UI surface refs without claiming
rendered parent UI, alert delivery runtime, provider delivery, notification
receipt runtime, child-device runtime, physical-device proof, authority proof,
production workers, or adapter dispatch.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/16-expected-place-schedule-engine.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/16-expected-place-schedule-engine/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Exception/holiday integration, rendered UI, platform claims, delivery
  runtime, child-device runtime, physical-device proof, authority proof,
  production workers, and adapter dispatch remain manual-required until the
  assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract/runtime files, proof scripts, tracking
      plan docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/16-expected-place-schedule-engine/`,
      including `06-expected-place-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [x] Known gaps/manual-required states: alert policy integration,
      exception/holiday integration, Android/iOS physical proof, notifications,
      and UI remain proof-gated as applicable.
- [x] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: parent-domain expected-place alert policy proof/test,
      proof script, owning tracking feature doc, implementation checklist,
      this workpack doc, WP33 proof-gate doc, generated WP16/WP33 proof
      artifacts, and hub doc delta queue.
- [x] Validation commands and results:
      `node scripts/test/tracking-expected-place-alert-policy-proof.mjs`
      passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/29-expected-place-alert-policy-proof.json`,
      `output/tracking-plan-proof/tracking-expected-place-alert-policy-proof/proof.json`,
      and `test-results/tracking-expected-place-alert-policy-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [x] Known gaps/manual-required states: rendered parent UI, alert delivery
      runtime, provider delivery, notification receipt runtime, child-device
      runtime, physical-device proof, authority proof, production workers,
      adapter dispatch, exception/holiday integration, and product-ready
      expected-place behavior remain proof-gated.
