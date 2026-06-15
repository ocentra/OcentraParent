# WP17 Parent Acknowledgement And Exception Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP17 Parent Acknowledgement And Exception Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
The hosted parent route now renders those acknowledgement, exception,
false-alarm, child check-in request, and escalation rows as read-only parent
action readiness UI through `npm run test:tracking-plan-hosted-ui-proof`. The
hosted proof writes `31-parent-acknowledgement-action-hosted-ui-proof.json`
while keeping live service mutation, alert/provider delivery, receipt
ingestion, child-device runtime, physical-device proof, authority proof,
production workers, adapter dispatch, and product-ready parent action behavior
unclaimed.

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

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: parent tracking policy runtime, tests, proof script,
      tracking plan docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/`,
      including `09-policy-alert-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [ ] Known gaps/manual-required states: alert delivery, portal
      acknowledgement UI, Android/iOS physical proof, provider delivery, and
      notifications remain proof-gated as applicable.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain parent acknowledgement action readiness
      proof/test, proof script, owning tracking feature doc, implementation
      checklist, this workpack doc, WP33 proof-gate doc, generated WP17/WP33
      proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-parent-acknowledgement-action-readiness-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/30-parent-acknowledgement-action-readiness-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/30-parent-acknowledgement-action-readiness-proof.json`,
      and
      `test-results/tracking-parent-acknowledgement-action-readiness-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [ ] Known gaps/manual-required states: rendered portal acknowledgement UI,
      live service mutation, alert/provider delivery, notification receipt
      runtime, child-device runtime, Android/iOS physical proof, authority,
      production workers, adapter dispatch, and product-ready parent action
      behavior remain proof-gated.
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
      `output/tracking-plan-proof/17-parent-acknowledgement-and-exception-model/31-parent-acknowledgement-action-hosted-ui-proof.json`,
      `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/23-parent-action-readiness-hosted-ui-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/36-parent-action-readiness-hosted-ui-proof.json`,
      and `test-results/tracking-plan-hosted-ui-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP16, WP17, WP30, and WP33 updated. Central
      `docs/product-capability-checklist.md` remains hub-sequenced because E-B
      owns that lock.
- [ ] Known gaps/manual-required states: hosted parent action readiness is
      read-only rendering proof only; live service mutation, alert/provider
      delivery, receipt ingestion, child-device runtime, Android/iOS physical
      proof, authority, production workers, adapter dispatch, and product-ready
      parent action behavior remain proof-gated.
