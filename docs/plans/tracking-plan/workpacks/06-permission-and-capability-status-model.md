# WP06 Permission And Capability Status Model

## Purpose

Represent permission, service, background, accuracy, platform, and proof state
before any strict tracking control is shown as available.

## Source Inputs

- `docs/expectations/platforms.md`
- `docs/expectations/location-geofence.md`
- `docs/tracking-control-settings-inventory.md`

## Target State

Capability state covers foreground-only, background-ready, approximate-only,
permission-required, background-permission-required, service-disabled,
manual-required, platform-unsupported, offline-last-known-only,
battery-throttled, unavailable, and adapter-error states.

## Tests And Proof

Proof root: `output/tracking-plan-proof/06-permission-and-capability-status-model/`

- `01-contract-proof.log`
- `02-platform-permission-proof.md`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Model Android, iOS, desktop, managed-device, and unsupported states.
- [ ] Show capability state next to strict controls.
- [ ] Prevent UI/rules from enabling unsupported behavior silently.
- [ ] Keep scaffold-only support separate from product support.

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

- docs/plans/tracking-plan/workpacks/06-permission-and-capability-status-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/06-permission-and-capability-status-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/06-permission-and-capability-status-model/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
