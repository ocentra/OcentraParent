# WP05 Device Status Model

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP05 Device Status Model`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Model heartbeat, battery, charging, connectivity, pending upload, stale/offline,
and degraded states that shape tracking reliability.

## Source Inputs

- `docs/tracking-control-settings-inventory.md`
- `docs/device-location-tracking-capability-guide.md`
- `docs/expectations/location-geofence.md`

## Target State

Device status evidence can explain why tracking is live, stale, throttled,
offline, pending upload, service-disabled, or unavailable.

## Tests And Proof

Proof root: `output/tracking-plan-proof/05-device-status-model/`

- `01-contract-proof.log`
- `04-device-status-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Cover last heartbeat, last location sample, last parent sync, and pending
      upload count.
- [ ] Cover battery percentage, charging, low-power, radio, and connectivity.
- [ ] Keep degraded reasons explicit.
- [ ] Do not let stale/offline become live location.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.

## Where We Want To Be

This workpack can be assigned independently, implemented against the owning domain boundaries, validated with real contracts or platform proof, and reported without leaving unclear tracking claims behind.

## Scope

- Re-read only the source inputs listed in this workpack and the shared source index.
- Update the owned contracts, runtime paths, UI paths, or docs named by the implementation worker before editing.
- Keep no-claim boundaries explicit for LAN/IP hints, nearby-place ambiguity, AI evidence, background behavior, and remote sync.
- Update the feature doc and product capability checklist only when status, proof, or gaps change.

## Touched Paths

- docs/plans/tracking-plan/workpacks/05-device-status-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/05-device-status-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [ ] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/05-device-status-model/`.
- [ ] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
