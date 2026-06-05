# WP27 Escalation Engine

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
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Test warning repeat, urgent second guardian, and critical multi-channel
      only when configured.
- [ ] Parent acknowledgement cancels escalation where configured.
- [ ] Child check-in resolves pending state where configured.
- [ ] AI cannot schedule escalation directly.
- [ ] No emergency auto-contact in MVP.
- [x] Prove acknowledgement/check-in escalation boundaries through the P1
      runtime proof gate without claiming provider delivery, emergency
      automation, or child-device runtime.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. It now also has P1 parent-domain runtime proof from
`codex/tracking-policy-escalation-runtime-proof` through
`npm run test:tracking-plan-policy-escalation-runtime-proof`, which writes
`09-policy-alert-proof.json`, `13-security-negative-proof.log`,
`16-validation-commands.log`, and
`test-results/tracking-plan-policy-escalation-runtime-proof/proof.json`.
Runtime proof currently covers warning acknowledgement suppression, critical
alert visibility, safe child check-in resolution, and expired-child-check-in
policy escalation. Provider delivery, emergency-contact automation,
child-device runtime, background-location, physical-device, and full UI behavior
are not claimed beyond the proof state recorded in the implementation checklist.

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
- packages/parent-domain/src/tracking-policy-escalation-runtime-proof.ts
- packages/parent-domain/tests/tracking-policy-escalation-runtime-proof.test.ts
- scripts/test/tracking-plan-policy-escalation-runtime-proof.mjs
- `output/tracking-plan-proof/27-escalation-engine/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.
- Provider notification delivery, emergency-contact automation,
  child-device runtime prompts, background-location behavior, physical
  Android/iOS behavior, and AI final-authority claims remain unclaimed.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope` contract proof;
      `codex/tracking-policy-escalation-runtime-proof` P1 runtime proof.
- [x] Touched files: tracking contract files, P1 runtime proof module, focused
      proof test, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results:
      `node scripts/test/tracking-plan-contract-proof.mjs` passed for the
      contract proof; `npm run test:tracking-plan-policy-escalation-runtime-proof`
      passed for the P1 runtime proof.
- [x] Proof artifacts under `output/tracking-plan-proof/27-escalation-engine/`
      and `test-results/tracking-plan-policy-escalation-runtime-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack, and hub checklist delta updated. Package README
      delta is noted in the hub handoff because `packages/parent-domain/readme.md`
      is currently locked by another lane.
- [x] Known gaps/manual-required states: Android/iOS physical behavior, provider
      delivery, emergency-contact automation, child-device runtime, background
      location, notifications, and full UI remain proof-gated as applicable.
