# WP16 Expected-Place Schedule Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP16 Expected-Place Schedule Engine`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
The hosted parent route now renders those expected-place alert/check-in/
suppression/manual rows as read-only parent action readiness UI through
`npm run test:tracking-plan-hosted-ui-proof`. The hosted proof writes
`30-expected-place-alert-policy-hosted-ui-proof.json` while keeping alert
delivery runtime, provider delivery, receipt ingestion, child-device runtime,
physical-device proof, authority proof, production workers, adapter dispatch,
and product-ready expected-place behavior unclaimed.
Schedule-engine holiday-mode/trip-exception suppression now exists in
`packages/tracking-domain` and `crates/tracking-core`, with surfaced
exception state/audit refs on the TypeScript decision contract plus crate-owned
unit coverage in `packages/tracking-domain/tests/unit/tracking.test.ts` and
`crates/tracking-core/tests/unit/expected_place.rs`. The focused TypeScript
owner test passes; focused Rust execution is still blocked in this checkout
before `tracking-core` runs by the unrelated `policy-control-core` import
failure.
`packages/tracking-domain` now also keeps explicit `ruleId`,
`distanceToleranceMeters`, `lateGraceSeconds`, and `earlyExitGraceSeconds`
citations on expected-place decision payloads instead of dropping that schedule
metadata at the runtime helper boundary, and the package-local unit suite now
covers the low-accuracy ambiguous path plus a DST-spanning absolute-window case
in the same WP16 owner bucket.
`crates/tracking-core` and `crates/agent-protocol` now mirror that citation
payload on the Rust event side: the expected-place evaluated event keeps the
existing `expectedPlaceRef` plus explicit `distanceToleranceMeters`,
`lateGraceSeconds`, `earlyExitGraceSeconds`, and `exceptionState`, and
`cargo test -p ocentra-parent-agent-protocol --test contract
tracking_expected_place_state_evaluated_event` now proves the serialized
contract/idempotency shape. Focused `tracking-core` execution remains blocked in
this checkout before the target crate runs by the same unrelated
`policy-control-core` unresolved-import failure.

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

- Proof-refreshed exception/holiday integration, rendered UI, platform claims,
  delivery runtime, child-device runtime, physical-device proof, authority
  proof, production workers, and adapter dispatch remain manual-required until
  the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract/runtime files, proof scripts, tracking
      plan docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/16-expected-place-schedule-engine/`,
      including `06-expected-place-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [ ] Known gaps/manual-required states: alert policy integration,
      exception/holiday integration, Android/iOS physical proof, notifications,
      and UI remain proof-gated as applicable.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain expected-place alert policy proof/test,
      proof script, owning tracking feature doc, implementation checklist,
      this workpack doc, WP33 proof-gate doc, generated WP16/WP33 proof
      artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-expected-place-alert-policy-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/16-expected-place-schedule-engine/29-expected-place-alert-policy-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/29-expected-place-alert-policy-proof.json`,
      `output/tracking-plan-proof/tracking-expected-place-alert-policy-proof/proof.json`,
      and `test-results/tracking-expected-place-alert-policy-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [ ] Known gaps/manual-required states: rendered parent UI, alert delivery
      runtime, provider delivery, notification receipt runtime, child-device
      runtime, physical-device proof, authority proof, production workers,
      adapter dispatch, exception/holiday integration, and product-ready
      expected-place behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: hosted parent action readiness proof model, portal route
      renderer/tests, hosted Playwright proof spec, hosted proof script,
      portal/text/domain constants, owning tracking feature doc, implementation
      checklist, WP16, WP17, WP30, WP33, and generated hosted proof artifacts.
- [ ] Validation commands and results: pending final hosted proof refresh after
      focused text-domain, portal-domain, and portal tracking-status tests
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/16-expected-place-schedule-engine/30-expected-place-alert-policy-hosted-ui-proof.json`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/23-parent-action-readiness-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/36-parent-action-readiness-hosted-ui-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP16, WP17, WP30, and WP33 updated. Central
      `docs/product-capability-checklist.md` remains hub-sequenced because E-B
      owns that lock.
- [ ] Known gaps/manual-required states: hosted parent action readiness is
      read-only rendering proof only; alert delivery runtime, provider delivery,
      notification receipt runtime, child-device runtime, physical-device
      proof, authority proof, production workers, adapter dispatch, and
      product-ready expected-place behavior remain proof-gated.
