# WP31 Platform Extension Checklists And Proof Routing

## Purpose

Route Android, iOS, desktop, managed-device, store/privacy, and manual platform
proof without bloating base contracts.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-platform-deep-dive.md`
- `docs/expectations/platforms.md`
- `docs/expectations/platform-deliverables.md`

## Target State

Each platform extension has a proof checklist, manual-required state, output
path, and no-claim rule before product status changes.

## Tests And Proof

Proof root: `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`

- `02-platform-permission-proof.md`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Maintain Android extension rows.
- [ ] Maintain iOS extension rows.
- [ ] Maintain desktop extension rows.
- [ ] Add managed-device proof only when real enrollment/control exists.
- [ ] Keep CI/package proof separate from real device capability proof.

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

- docs/plans/tracking-plan/workpacks/31-platform-extension-checklists-and-proof-routing.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
