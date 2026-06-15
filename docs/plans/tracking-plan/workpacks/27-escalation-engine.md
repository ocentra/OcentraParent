# WP27 Escalation Engine

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP27 Escalation Engine`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Implement rule-based escalation for unacknowledged parent alerts, missing child
check-ins, offline-after-alert, critical place, and left-expected-place states.

## Source Inputs

- `docs/expectations/notifications.md`
- `docs/expectations/policy.md`
- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`

## Target State

Escalation chains are configured by parent policy, acknowledgement-aware,
provider-minimized, multi-guardian capable, and never auto-contact emergency
services in MVP.

## Tests And Proof

Proof root: `output/tracking-plan-proof/27-escalation-engine/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `10-escalation-runtime-readiness-blocker-proof.json`
- `11-escalation-runtime-artifact-gate-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test warning acknowledgement/check-in resolution, urgent second
      guardian, and critical multi-channel manual readiness.
- [ ] Parent acknowledgement cancels escalation where configured.
- [ ] Child check-in resolves pending state where configured.
- [ ] AI cannot schedule escalation directly.
- [ ] No emergency auto-contact in MVP.

## Where We Are

This workpack now has P1 fixture-simulation proof from
`codex/tracking-escalation-readiness-proof` under the proof root below. The
proof derives escalation readiness rows from the existing tracking policy read
model, covers parent acknowledgement cancellation, child check-in resolution,
urgent second-guardian manual readiness, critical multi-channel manual
readiness, manual-required, and unavailable states, and rejects AI direct
scheduling plus emergency auto-contact overclaims. This continuation also adds
`node scripts/test/tracking-escalation-runtime-readiness-blocker-proof.mjs`,
which consumes the escalation readiness proof and provider-runtime blocker
proof to write `10-escalation-runtime-readiness-blocker-proof.json` under this
proof root plus the WP33 companion gate. Runtime workers, platform adapters,
provider delivery/receipt runtime, parent notification history runtime,
child-device delivery, durable escalation storage, physical-device proof,
authority proof, emergency auto-contact policy, and production quiet-hours
timers remain unclaimed until real runtime artifacts exist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/27-escalation-engine.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/27-escalation-engine/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-escalation-readiness-proof`.
- [ ] Touched files: parent-domain escalation readiness proof contract, focused
      test, proof script, feature docs, implementation checklist, workpack doc,
      proof output, and queued capability-checklist doc delta.
- [ ] Validation commands and results:
      `node scripts/test/tracking-escalation-readiness-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/27-escalation-engine/`
      and `test-results/tracking-escalation-readiness-proof/`.
- [ ] Product doc/checklist updates: owning feature docs, implementation
      checklist, this workpack doc, and capability-checklist delta queued while
      the central checklist remains sequenced through hub locks.
- [ ] Known gaps/manual-required states: provider delivery, receipt ingestion,
      credentials, cloud routing, parent notification UI/history/preferences,
      child-device delivery, physical-device proof, production escalation
      workers, production quiet-hours timers, durable storage, emergency
      auto-contact, Android/iOS physical proof, and full runtime execution
      remain proof-gated.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: escalation runtime blocker proof model/test, focused proof
      script, owning tracking feature doc, implementation checklist, WP27,
      WP33, generated WP27/WP33 proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-escalation-runtime-readiness-blocker-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/27-escalation-engine/10-escalation-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/53-escalation-runtime-readiness-blocker-proof.json`,
      `output/tracking-plan-proof/tracking-escalation-runtime-readiness-blocker-proof/proof.json`,
      and
      `test-results/tracking-escalation-runtime-readiness-blocker-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP27, and WP33 updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: production escalation workers,
      production quiet-hours timers, provider delivery/receipt runtime,
      provider credentials, parent notification history runtime,
      child-device delivery, durable escalation storage, physical-device proof,
      authority proof, emergency auto-contact policy, and product-ready
      escalation remain proof-gated until real runtime artifacts exist.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: escalation runtime artifact gate proof/test, focused proof
      script, product-readiness closure proof model, closure harness, owning
      tracking feature doc, implementation checklist, WP27, WP33, generated
      WP27/WP33 proof artifacts, and hub doc delta queue.
- [ ] Validation commands and results:
      `node scripts/test/tracking-escalation-runtime-artifact-gate-proof.mjs`
      passed; `node scripts/test/tracking-product-readiness-closure-proof.mjs`
      passed.
- [ ] Proof artifacts:
      `output/tracking-plan-proof/27-escalation-engine/11-escalation-runtime-artifact-gate-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/62-escalation-runtime-artifact-gate-proof.json`,
      `output/tracking-plan-proof/tracking-escalation-runtime-artifact-gate-proof/proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json`,
      `output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json`,
      `test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json`,
      and `test-results/tracking-product-readiness-closure-proof/proof.json`.
- [ ] Product doc/checklist updates: owning feature doc, implementation
      checklist, WP27, and WP33 updated. Central
      `docs/product-capability-checklist.md` update is queued through the hub
      doc delta instead of editing the shared checklist directly.
- [ ] Known gaps/manual-required states: production escalation workers,
      production quiet-hours timers, provider delivery/receipt runtime,
      parent notification history runtime, child-device delivery, durable
      escalation storage, emergency auto-contact policy, physical-device proof,
      authority proof, and product-ready escalation remain proof-gated; this
      gate only classifies the runtime artifact inventory.
