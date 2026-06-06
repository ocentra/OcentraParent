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
- `18-ai-provider-routing-custody-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [x] Keep child-device local as default safety path.
- [x] Require explicit parent approval for remote mode.
- [x] Preserve provider unavailable/degraded state.
- [x] Prevent assistant-only policy writes.
- [x] Cite evidence and custody in all AI contexts.

## Where We Are

This workpack has focused contract proof from `codex/tracking-plan-full-scope`
under the proof root below. This continuation adds
`node scripts/test/tracking-ai-provider-routing-proof.mjs`, which proves a
parent-domain route matrix with child-local as the only default safety path,
parent-approved remote as the only remote-data route, explicit degraded/
unavailable/disabled rows, assistant preview-only no-write/no-enforcement
boundaries, and evidence/custody refs on every AI context. Runtime, platform,
model execution, provider delivery, production, and UI behavior is not claimed
beyond the proof state recorded in `proof-summary.json` and the implementation
checklist.

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
- packages/parent-domain/src/tracking-ai-provider-routing-proof.ts
- packages/parent-domain/tests/tracking-ai-provider-routing-proof.test.ts
- scripts/test/tracking-ai-provider-routing-proof.mjs
- `output/tracking-plan-proof/24-ai-provider-routing/`
- `test-results/tracking-ai-provider-routing-proof/`
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
- [x] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [x] Touched files: WP24 proof source, tests, proof harness, feature doc,
      implementation checklist, WP24/WP33 docs, and generated proof artifacts.
- [x] Validation commands and results:
      `node scripts/test/tracking-ai-provider-routing-proof.mjs` passed
      locally.
- [x] Proof artifacts under
      `output/tracking-plan-proof/24-ai-provider-routing/` and
      `test-results/tracking-ai-provider-routing-proof/`.
- [x] Product doc/checklist updates: owning tracking feature doc, tracking
      implementation checklist, WP24, and WP33 updated; central product
      checklist row remains unchanged because this does not upgrade product
      support beyond the existing local proof tier.
- [x] Known gaps/manual-required states: model execution, provider delivery,
      child-device runtime, rendered assistant/provider UI, authority,
      enforcement, production behavior, and physical-device proof remain
      unclaimed.
