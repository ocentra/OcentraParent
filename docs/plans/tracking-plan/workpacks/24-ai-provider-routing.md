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

- [x] Keep child-device local as default safety path.
- [x] Require explicit parent approval for remote mode.
- [x] Preserve provider unavailable/degraded state.
- [x] Prevent assistant-only policy writes.
- [x] Cite evidence and custody in all AI contexts.

## Where We Are

This workpack now has focused contract proof from
`codex/tracking-plan-full-scope` plus P1 provider-readiness proof from
`codex/tracking-ai-provider-readiness-proof`. The current route proof covers
child-device local AI, parent-device local AI, local-LAN family hub,
parent-approved remote, metadata-only, no-AI, unavailable, and degraded states
without claiming live model execution, real provider delivery, policy authority,
or enforcement.

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
- packages/activity-domain/src/tracking-ai-provider-routing.ts
- packages/activity-domain/tests/tracking-ai-provider-routing.test.ts
- scripts/test/tracking-plan-ai-provider-readiness-proof.mjs
- `output/tracking-plan-proof/24-ai-provider-routing/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, live-provider, UI, retention, and runtime execution claims remain
  manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch: `codex/tracking-ai-provider-readiness-proof`.
- [x] Touched files:
      `packages/activity-domain/src/tracking-ai-provider-routing.ts`;
      `packages/activity-domain/src/tracking.ts`;
      `packages/activity-domain/tests/tracking-ai-provider-routing.test.ts`;
      `packages/activity-domain/package.json`;
      `packages/activity-domain/README.md`;
      `scripts/test/tracking-plan-ai-provider-readiness-proof.mjs`;
      `package.json`; tracking feature doc; implementation checklist; and this
      workpack doc.
- [x] Validation commands and results:
      `cmd /c npm run test:tracking-plan-ai-provider-readiness-proof` passed,
      including activity-domain build, 1 test file / 7 tests, and
      `lint:schema-boundaries` with pre-existing source-shape warnings.
- [x] Proof artifacts under `output/tracking-plan-proof/24-ai-provider-routing/`.
- [x] Product doc/checklist updates: owning feature doc, implementation
      checklist, this workpack, activity-domain README, and central product
      checklist delta queued through hub.
- [x] Known gaps/manual-required states: live model execution, real family hub
      runtime/discovery, parent-approved remote adapter and approval UI,
      Android/iOS physical behavior, provider delivery, notifications,
      authority, enforcement, and production proof remain proof-gated.
