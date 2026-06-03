# WP24 AI Provider Routing

## Purpose

Route location AI analysis through child-local, parent-local, family AI hub,
parent-approved remote, metadata-only, unavailable, and no-AI modes without
breaking custody rules.

## Source Inputs

- `docs/features/local-ai-safety-evaluator.md`
- `docs/features/parent-assistant-actions.md`
- `docs/expectations/ai.md`
- `docs/expectations/data-custody.md`

## Target State

Provider routing is capability-gated, custody-labeled, explicit about remote
disabled state, and unable to override local policy.

## Tests And Proof

Proof root: `output/tracking-plan-proof/24-ai-provider-routing/`

- `01-contract-proof.log`
- `08-ai-analysis-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Keep child-device local as default safety path.
- [ ] Require explicit parent approval for remote mode.
- [ ] Preserve provider unavailable/degraded state.
- [ ] Prevent assistant-only policy writes.
- [ ] Cite evidence and custody in all AI contexts.

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

- docs/plans/tracking-plan/workpacks/24-ai-provider-routing.md
- docs/plans/tracking-plan/implementation-checklist.md
- `output/tracking-plan-proof/24-ai-provider-routing/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [x] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [x] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [x] Proof artifacts under `output/tracking-plan-proof/24-ai-provider-routing/`.
- [x] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [x] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
