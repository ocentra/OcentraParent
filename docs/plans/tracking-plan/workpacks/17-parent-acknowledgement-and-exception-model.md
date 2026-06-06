# WP17 Parent Acknowledgement And Exception Model

## Purpose

Make parent acknowledgement, false alarms, holidays, trips, with-parent state,
exceptions, expiry, and still-alert categories first-class.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/notifications.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Parents can acknowledge safe, mark expected, create exception, call child,
request check-in, start live tracking, escalate, or mark false alarm with audit
refs and expiry.

## Tests And Proof

Proof root: `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Model acknowledgement actions and exception scopes.
- [ ] Preserve still-alert-for critical categories.
- [ ] Test holiday, school closed, child with parent, false alarm, and expiry.
- [ ] Ensure acknowledgement cancels escalation where configured.
- [ ] Do not allow generic exceptions to suppress configured critical alerts.

## Where We Are

This workpack has P0 contract proof and P1 fixture proof for parent
acknowledgement impact evaluation from `codex/tracking-plan-full-scope` under
the proof root below. Alert delivery, portal acknowledgement UI, and physical
device behavior are not claimed beyond the proof state recorded in
`proof-summary.json`, `09-policy-alert-proof.json`, and the implementation
checklist.
Parent acknowledgement action readiness proof now maps existing tracking alerts,
acknowledgements, expected/holiday/trip exceptions, false-alarm rows, child
check-in decisions, and escalation/manual-review rows into parent action
readiness rows through
`node scripts/test/tracking-parent-acknowledgement-action-readiness-proof.mjs`.
It preserves evidence refs, policy decision refs, acknowledgement refs,
escalation refs, audit refs, exception expiry refs, critical-alert visibility,
and UI surface refs without claiming rendered portal acknowledgement UI, live
service mutation, provider delivery, notification receipt runtime,
child-device runtime, physical-device proof, authority proof, production
workers, or adapter dispatch.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/17-parent-acknowledgement-and-exception-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Alert delivery, rendered portal acknowledgement UI, live service mutation,
  platform, provider, child-device runtime, physical-device, authority,
  production worker, and adapter-dispatch claims remain manual-required until
  the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: parent tracking policy runtime, tests, proof script,
      tracking plan docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`,
      including `09-policy-alert-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [x] Known gaps/manual-required states: alert delivery, portal
      acknowledgement UI, Android/iOS physical proof, provider delivery, and
      notifications remain proof-gated as applicable.
- [x] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: parent-domain parent acknowledgement action readiness
      proof/test, proof script, owning tracking feature doc, implementation
      checklist, this workpack doc, WP33 proof-gate doc, generated WP17/WP33
      proof artifacts, and hub doc delta queue.
- [x] Validation commands and results:
      `node scripts/test/tracking-parent-acknowledgement-action-readiness-proof.mjs`
      passed.
- [x] Proof artifacts:
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/30-parent-acknowledgement-action-readiness-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/30-parent-acknowledgement-action-readiness-proof.json`,
      and
      `test-results/tracking-parent-acknowledgement-action-readiness-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [x] Known gaps/manual-required states: rendered portal acknowledgement UI,
      live service mutation, alert/provider delivery, notification receipt
      runtime, child-device runtime, Android/iOS physical proof, authority,
      production workers, adapter dispatch, and product-ready parent action
      behavior remain proof-gated.
