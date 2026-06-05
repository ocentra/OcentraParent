# WP25 Policy Compiler For Tracking Rules

## Purpose

Compile location, geofence, expected-place, nearby-place, stale/offline,
battery, check-in, and escalation targets into parent policy decisions.

## Source Inputs

- `docs/expectations/policy.md`
- `docs/expectations/location-geofence.md`
- `docs/plans/tracking-plan/v0-5-location-ai-safety-analysis-plan.md`

## Target State

Policy decisions cite evidence, rule refs, schedule refs, AI candidate refs
when used, capability state, exception state, and audit refs.

## Tests And Proof

Proof root: `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`

- `01-contract-proof.log`
- `09-policy-alert-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Parent policy is final action authority.
- [ ] Compile observe, notify, ask child, ask parent ack, live tracking,
      escalate, critical alert, suppress, and manual-required.
- [ ] Add deterministic conflict tests.
- [ ] Prevent AI-only alert/escalation.
- [ ] Preserve dry-run/preview where applicable.
- [x] Prove parent-policy authority and AI non-authority through the P1 runtime
      proof gate without claiming provider delivery or child-device runtime.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. It now also has P1 parent-domain runtime proof from
`codex/tracking-policy-escalation-runtime-proof` through
`npm run test:tracking-plan-policy-escalation-runtime-proof`, which writes
`09-policy-alert-proof.json`, `13-security-negative-proof.log`,
`16-validation-commands.log`, and
`test-results/tracking-plan-policy-escalation-runtime-proof/proof.json`.
Runtime proof currently covers parent-policy final authority, AI non-authority,
warning acknowledgement suppression, critical alert visibility, safe child
check-in resolution, and expired-child-check-in policy escalation. Platform,
provider delivery, emergency-contact automation, child-device runtime,
background-location, physical-device, and full UI behavior are not claimed
beyond the proof state recorded in the implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/25-policy-compiler-for-tracking-rules.md
- docs/plans/tracking-plan/implementation-checklist.md
- packages/parent-domain/src/tracking-policy-escalation-runtime-proof.ts
- packages/parent-domain/tests/tracking-policy-escalation-runtime-proof.test.ts
- scripts/test/tracking-plan-policy-escalation-runtime-proof.mjs
- `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`
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
- [x] Proof artifacts under `output/tracking-plan-proof/25-policy-compiler-for-tracking-rules/`
      and `test-results/tracking-plan-policy-escalation-runtime-proof/proof.json`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack, and hub checklist delta updated. Package README
      delta is noted in the hub handoff because `packages/parent-domain/readme.md`
      is currently locked by another lane.
- [x] Known gaps/manual-required states: Android/iOS physical behavior, provider
      delivery, emergency-contact automation, child-device runtime, background
      location, notifications, and full UI remain proof-gated as applicable.
