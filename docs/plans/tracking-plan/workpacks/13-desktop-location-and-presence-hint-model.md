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
- `04-desktop-presence-read-model.json`
- `13-security-negative-proof.log`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`
- `proof.json`

## AI Worker Checklist

- [x] Add no-GPS guards for LAN/IP/Wi-Fi.
- [x] Represent manual check-in separately.
- [x] Label desktop precise location as manual-required until proved.
- [x] Include stale/offline/missing-device state.
- [x] Do not use LAN pairing as physical presence proof.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope` under the proof root below. Runtime, platform, provider, and UI behavior is not claimed beyond the proof state recorded in `proof-summary.json` and the implementation checklist.

Focused parent-domain desktop presence proof now also exists on
`codex/tracking-desktop-presence-hint-proof` through
`node scripts/test/tracking-desktop-presence-hint-proof.mjs`. It builds and
tests a read model that keeps LAN, Wi-Fi, and IP rows hint-only, separates
manual check-in, marks desktop OS location as manual-required until runtime
proof exists, includes stale/offline and missing-device states, and explicitly
rejects GPS, precise-location, physical-presence, LAN-pairing physical proof, OS
location runtime, physical-device, and product-ready desktop tracking claims.

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

- [x] Workpack id and branch: `codex/tracking-desktop-presence-hint-proof`.
- [x] Touched files: parent-domain desktop presence contract/test, focused proof
      script, feature doc, implementation checklist, workpack doc, and WP13 proof
      artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-desktop-presence-hint-proof.mjs` passed.
- [x] Proof artifacts under
      `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`
      and `test-results/tracking-desktop-presence-hint-proof/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, and this workpack updated; product-capability checklist not
      touched because another lane currently owns that file and this slice did
      not change product-ready status.
- [x] Known gaps/manual-required states: desktop OS precise location runtime,
      physical-device proof, provider delivery, portal/runtime UI completion,
      and product-ready desktop tracking remain unclaimed.
- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/13-desktop-location-and-presence-hint-model/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
