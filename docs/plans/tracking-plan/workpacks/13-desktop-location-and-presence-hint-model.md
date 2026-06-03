# WP13 Desktop Location And Presence Hint Model

## Purpose

Represent Windows/macOS/Linux location, Wi-Fi, LAN, IP, manual check-in, and
missing-device states without GPS overclaim.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- `docs/plans/lan-plan`

## Target State

Desktop OS location samples are source/accuracy labeled. LAN, Wi-Fi, and IP are
hint-only unless precise OS location proof exists.

## Tests And Proof

Proof root: `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`

- `03-runtime-location-evidence.json`
- `13-security-negative-proof.log`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Add no-GPS guards for LAN/IP/Wi-Fi.
- [ ] Represent manual check-in separately.
- [ ] Label desktop precise location as manual-required until proved.
- [ ] Include stale/offline/missing-device state.
- [ ] Do not use LAN pairing as physical presence proof.

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

- docs/plans/tracking-plan/workpacks/13-desktop-location-and-presence-hint-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
