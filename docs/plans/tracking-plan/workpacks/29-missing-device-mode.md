# WP29 Missing-Device Mode

## Purpose

Provide last-known location, battery, connectivity, stale/offline, pending
upload, contact action, and prominent UI state when a parent marks a device
missing.

## Source Inputs

- `docs/device-location-tracking-capability-guide.md`
- `docs/tracking-control-settings-inventory.md`
- `docs/plans/tracking-plan/ui-ux-requirements-guide.md`

## Target State

Missing-device mode shows last known plus status without claiming current
location for powered-off/offline devices.

## Tests And Proof

Proof root: `output/tracking-plan-proof/29-missing-device-mode/`

- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Prioritize last known, battery, connectivity, and last contact.
- [ ] Avoid current-location claims when stale/offline.
- [ ] Add parent action/audit state.
- [ ] Test powered-off/offline copy and UI.
- [ ] Keep remote sync optional and explicit.

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

- docs/plans/tracking-plan/workpacks/29-missing-device-mode.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/29-missing-device-mode/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/29-missing-device-mode/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
