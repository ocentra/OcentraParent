# WP29 Missing-Device Mode

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP29 Missing-Device Mode`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Purpose

Provide last-known location, battery, connectivity, stale/offline, pending upload, contact action, and prominent UI state when a parent marks a device missing.

## Central schema boundary

```text
schema-domain owns missing-device mode contracts that cross runtime, portal, policy, notification, custody, or proof boundaries.
tracking-core may evaluate last-known/stale/offline state from canonical inputs.
tracking-domain may provide helper/proof adapters only.
platform/runtime owners own actual device behavior and OS lost-mode integrations.
```

## Source Inputs

- `docs/device-location-tracking-capability-guide.md`
- `docs/tracking-control-settings-inventory.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`

## Target State

Missing-device mode shows last known plus status without claiming current location for powered-off/offline devices.

## Required proof fields

```text
canonical_schema_owner_state
last_known_state
battery_state
connectivity_state
pending_upload_state
contact_action_state
stale_offline_state
current_location_claim_state
remote_sync_state
provider_state
os_lost_mode_state
audit_state
portal_state
no_current_location_claim
no_product_ready_claim
no_claim
```

## Tests And Proof

Proof root: `output/tracking-plan-proof/29-missing-device-mode/`

- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `11-ui-snapshots/`
- `20-missing-device-hosted-ui-proof.json`
- `12-playwright-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Prioritize last known, battery, connectivity, and last contact.
- [ ] Avoid current-location claims when stale/offline.
- [ ] Add parent action/audit state.
- [ ] Test powered-off/offline copy and UI.
- [ ] Keep remote sync optional and explicit.

## Where We Are

This workpack now has focused proof under the proof root below. Runtime, platform, and provider behavior is not claimed beyond the proof state recorded in `proof.json`, `03-runtime-location-evidence.json`, `04-device-status-proof.json`, `11-ui-snapshots/missing-device-ui-state-matrix.json`, and the implementation checklist. Hosted parent route proof renders last-known-only, offline/powered-off, contact-requested, and manual-required rows without claiming current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode API execution, authority, production workers, or product readiness.

## Manual-Required Gaps

- Platform, provider, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Hosted route rendering is now proved only for the read-only missing-device rows; runtime execution and platform behavior remain manual-required.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Validation commands and results: missing-device proof and hosted route proof commands passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/29-missing-device-mode/`.
- [ ] Product doc/checklist updates: owning feature doc, product capability checklist, implementation checklist, and this workpack doc updated.
- [ ] Known gaps/manual-required states: runtime execution, Android/iOS physical-device proof, provider delivery, remote sync runtime, OS lost-mode APIs, notification delivery, production proof, and product-ready behavior remain proof-gated.
