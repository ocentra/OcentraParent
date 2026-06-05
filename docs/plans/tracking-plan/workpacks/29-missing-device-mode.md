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

- [x] Prioritize last known, battery, connectivity, and last contact.
- [x] Avoid current-location claims when stale/offline.
- [x] Add parent action/audit state.
- [x] Test powered-off/offline copy and UI.
- [x] Keep remote sync optional and explicit.

## Where We Are

This workpack now has focused parent-domain proof from `codex/tracking-missing-device-mode-proof` under the proof root below. Runtime, platform, provider, and portal UI behavior is not claimed beyond the proof state recorded in `proof.json`, `03-runtime-location-evidence.json`, `04-device-status-proof.json`, `11-ui-snapshots/missing-device-ui-state-matrix.json`, and the implementation checklist.

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

- [x] Workpack id and branch: `codex/tracking-missing-device-mode-proof`.
- [x] Touched files: `packages/parent-domain/src/tracking-missing-device-mode-proof.ts`, `packages/parent-domain/tests/tracking-missing-device-mode-proof.test.ts`, `scripts/test/tracking-missing-device-mode-proof.mjs`, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `cmd /c node scripts/test/tracking-missing-device-mode-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/29-missing-device-mode/`.
- [x] Product doc/checklist updates: owning feature doc, product capability checklist, implementation checklist, and this workpack doc updated.
- [x] Known gaps/manual-required states: portal runtime UI/screenshots, child-device delivery/runtime execution, Android/iOS physical-device proof, provider delivery, remote sync runtime, OS lost-mode APIs, notification delivery, and production proof remain proof-gated.
