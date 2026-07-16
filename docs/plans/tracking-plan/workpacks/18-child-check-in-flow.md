# WP18 Child Check-In Flow

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP18 Child Check-In Flow`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Provide a safe parent-requested and child-initiated check-in flow with optional
location sample, no-shame copy, audit refs, and escalation handoff.

## Source Inputs

- `docs/expectations/notifications.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`

## Target State

Child check-ins support safe/help/share-location/call-parent responses. Unclear
or missing check-ins remain rule-based escalation inputs, not accusations.

## Tests And Proof

Proof root: `output/tracking-plan-proof/18-child-check-in-flow/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Use calm child-safe copy.
- [ ] Support optional current location sample where permission allows.
- [ ] Audit every prompt and response.
- [ ] Resolve or update alert state from child response.
- [ ] Test unresolved check-in escalation by rule only.

## Where We Are

This workpack has P0 contract proof and P1 fixture proof for child check-in
response and expiry resolution from `codex/tracking-plan-full-scope` under the
proof root below. Child-device UI, delivery, timeout escalation wiring, and
physical device behavior are not claimed beyond the proof state recorded in
`proof-summary.json`, `09-policy-alert-proof.json`, and the implementation
checklist.
Child check-in timeout escalation proof now maps existing child check-in
requests and responses through the runtime resolver into waiting, safe response,
help response escalation, call-parent escalation, and expired timeout escalation
rows through
`node scripts/test/tracking-child-check-in-timeout-escalation-proof.mjs`. It
preserves request refs, response refs, alert refs, evidence refs, policy refs,
prompt/response audit refs, parent action refs, timeout refs, optional
location-sample request state, attached response location-evidence refs, alert
outcome projection, and rule-only timeout escalation basis without claiming
child-device delivery/runtime execution, rendered child UI, provider delivery,
notification receipt runtime, live location sample runtime, physical-device
proof, authority proof, production timeout workers, or adapter dispatch. Calm
child-safe copy is separately rendered in the hosted child check-in card under
WP30; WP18 does not claim child-device UI delivery.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/18-child-check-in-flow.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/18-child-check-in-flow/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Child-device UI, delivery/runtime execution, platform, provider delivery,
  notification receipt runtime, live location sample runtime, physical-device,
  authority, production timeout worker, and adapter-dispatch claims remain
  manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: parent tracking policy runtime, tests, proof script,
      tracking plan docs, checklist, and this workpack doc.
- [ ] Validation commands and results:
      `node scripts/test/tracking-plan-runtime-proof.mjs` passed.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/18-child-check-in-flow/`, including
      `09-policy-alert-proof.json`.
- [ ] Product doc/checklist updates: tracking plan/checklist updated; central
      product capability checklist reconciliation remains pending while that
      shared doc is actively locked outside this lane.
- [ ] Known gaps/manual-required states: child-device UI, delivery, timeout
      escalation wiring, Android/iOS physical proof, provider delivery, and
      notifications remain proof-gated as applicable.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain child check-in timeout escalation
      proof/test, proof script, owning tracking feature doc, implementation
      checklist, this workpack doc, WP33 proof-gate doc, generated WP18/WP33
      proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-child-check-in-timeout-escalation-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/18-child-check-in-flow/31-child-check-in-timeout-escalation-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/31-child-check-in-timeout-escalation-proof.json`,
      and
      `test-results/tracking-child-check-in-timeout-escalation-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [ ] Known gaps/manual-required states: child-device delivery/runtime
      execution, rendered child UI, provider delivery, notification receipt
      runtime, live location sample runtime, Android/iOS physical proof,
      authority, production timeout workers, adapter dispatch, and
      product-ready child check-in behavior remain proof-gated.
- [ ] Workpack id and branch:
      `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: parent-domain child check-in timeout proof model/test,
      proof harness, owning tracking feature doc, implementation checklist,
      this workpack doc, WP33 proof-gate doc, regenerated WP18/WP33/test-result
      proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-child-check-in-timeout-escalation-proof.mjs`
      passed after adding optional location-sample, prompt/response audit,
      alert-outcome, and rule-only escalation assertions.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/18-child-check-in-flow/31-child-check-in-timeout-escalation-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/31-child-check-in-timeout-escalation-proof.json`,
      `test-results/tracking-child-check-in-timeout-escalation-proof/proof.json`,
      and
      `test-results/tracking-child-check-in-timeout-escalation-proof/tracking-child-check-in-timeout-read-model.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack doc, and WP33 proof-gate doc updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta.
- [ ] Known gaps/manual-required states: this remains P1 fixture proof.
      Child-device delivery/runtime execution, rendered child-device UI,
      provider delivery, notification receipt runtime, live location sample
      runtime, Android/iOS physical proof, authority, production timeout
      workers, adapter dispatch, and product-ready child check-in behavior
      remain proof-gated.
