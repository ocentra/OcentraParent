# WP33 Proof Gates Fixtures Rollout And PR Gate

## Purpose

Define final test fixtures, proof packs, Playwright/manual proof, docs updates,
and merge blockers for tracking work.

## Source Inputs

- `docs/plans/tracking-plan/v0-5-location-test-blueprint.md`
- `docs/plans/tracking-plan/implementation-checklist.md`
- `docs/plans/tracking-plan/pasted-content-coverage-audit.md`
- `.ocentra-ai/rules/ocentra-parent-validation.mdc`

## Target State

No tracking implementation can report `DONE` or PR-ready without proof packs,
validation commands, docs/checklist updates, and explicit known gaps.

## Tests And Proof

Proof root: `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`

- `00-source-snapshot.md`
- `01-contract-proof.log`
- `02-platform-permission-proof.md`
- `03-runtime-location-evidence.json`
- `04-device-status-proof.json`
- `05-geofence-transition-proof.json`
- `06-expected-place-proof.json`
- `07-nearby-place-proof.json`
- `08-ai-analysis-proof.json`
- `09-policy-alert-proof.json`
- `10-journal-sqlite-proof.json`
- `11-ui-snapshots/`
- `12-playwright-proof.log`
- `13-security-negative-proof.log`
- `14-retention-delete-proof.json`
- `15-manual-platform-proof.md`
- `16-validation-commands.log`

## Merge Blockers

- LAN/IP displayed as GPS.
- Location missing accuracy/source/timestamp/freshness.
- Stale displayed as live.
- Nearby POI displayed as exact place with low accuracy.
- AI triggers notification without policy decision.
- Critical alert suppressed by generic exception.
- Parent acknowledgement ignored.
- Retention delete fails.
- Remote sync runs by default.
- Remote AI runs by default.
- Background tracking claimed without Android/iOS proof.

## AI Worker Checklist

- [ ] Run the smallest useful validation while working.
- [ ] Run requested focused tests before handoff.
- [ ] Update feature docs and capability checklist when proof changes.
- [ ] Include touched files, validation, product-doc updates, known gaps, and
      platform proof state in `DONE`.
- [ ] Do not mark product-complete from planning-only docs.

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

- docs/plans/tracking-plan/workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
