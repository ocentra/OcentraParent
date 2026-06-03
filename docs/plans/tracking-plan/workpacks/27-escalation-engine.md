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

## Where We Are

This workpack is planning-only until its implementation branch produces the proof root below. Existing source docs describe the intended capability, but runtime/product-complete behavior is not claimed yet.

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

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/27-escalation-engine/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
