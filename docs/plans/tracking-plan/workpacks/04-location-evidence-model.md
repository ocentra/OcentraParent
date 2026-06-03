# WP04 Location Evidence Model

## Purpose

Model reported device location with source, accuracy, time, freshness, custody,
retention, permission, confidence, and reason codes.

## Source Inputs

- `docs/expectations/location-geofence.md`
- `docs/device-location-tracking-capability-guide.md`
- `docs/plans/tracking-plan/v0-5-location-tracking-full-scope-plan.md`

## Target State

Location evidence distinguishes precise samples, approximate samples, last-known
samples, manual check-ins, LAN/Wi-Fi/IP hints, unavailable states, and adapter
errors without overclaiming.

## Tests And Proof

Proof root: `output/tracking-plan-proof/04-location-evidence-model/`

- `01-contract-proof.log`
- `03-runtime-location-evidence.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## Merge Blockers

- GPS-like source without accuracy.
- Hint-only source promoted to precise location.
- Stale evidence displayed as live.

## AI Worker Checklist

- [ ] Require source, observed/collected time, freshness, custody, retention,
      permission, confidence, and reason codes.
- [ ] Preserve `hint_only` states.
- [ ] Add negative tests for LAN/IP/Wi-Fi overclaiming.
- [ ] Cite proof artifacts before checking off implementation status.

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

- docs/plans/tracking-plan/workpacks/04-location-evidence-model.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/04-location-evidence-model/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch.
- [ ] Touched files.
- [ ] Validation commands and results.
- [ ] Proof artifacts under `output/tracking-plan-proof/04-location-evidence-model/`.
- [ ] Product doc/checklist updates or reason none were needed.
- [ ] Known gaps/manual-required states.
