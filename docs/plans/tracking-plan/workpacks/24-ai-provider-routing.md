# WP24 AI Provider Routing

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `WP24 AI Provider Routing`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

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
- `19-ai-stored-ref-consumer-proof.json`
- `13-security-negative-proof.log`
- `16-validation-commands.log`

## AI Worker Checklist

- [ ] Keep child-device local as default safety path.
- [ ] Require explicit parent approval for remote mode.
- [ ] Preserve provider unavailable/degraded state.
- [ ] Prevent assistant-only policy writes.
- [ ] Cite evidence and custody in all AI contexts.
- [ ] Require stored journal/read-model refs before AI report/policy consumer
      use.

## AI Boundary Ownership

Tracking owns only location-specific AI request/result contracts and evidence
handoff tests. Provider selection, provider mesh, work lease/claim internals,
model quality, prompt tuning, prompt-injection model behavior, temperature
behavior, and summarizer accuracy belong to the AI lane. Tracking tests must
prove accepted AI output is evidence-only and cannot publish policy,
enforcement, notification, live-mode, or escalation events.

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
This continuation also adds
`node scripts/test/tracking-ai-stored-ref-consumer-proof.mjs`, which proves that
AI parent-report, policy-drill-in, and metadata-fallback consumer contexts cite
the existing provider-route proof plus stored journal and read-model row refs
before AI report/policy use. It keeps model execution, assistant policy writes,
assistant enforcement, provider delivery, child-device runtime, authority,
production behavior, and product-ready claims false.

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
- packages/parent-domain/src/tracking-ai-stored-ref-consumer-proof.ts
- packages/parent-domain/tests/tracking-ai-provider-routing-proof.test.ts
- packages/parent-domain/tests/tracking-ai-stored-ref-consumer-proof.test.ts
- scripts/test/tracking-ai-provider-routing-proof.mjs
- scripts/test/tracking-ai-stored-ref-consumer-proof.mjs
- `output/tracking-plan-proof/24-ai-provider-routing/`
- `test-results/tracking-ai-provider-routing-proof/`
- `test-results/tracking-ai-stored-ref-consumer-proof/`
- Implementation paths listed by the worker before editing.

## Manual-Required Gaps

- Platform, provider, UI, retention, or runtime claims remain manual-required until the assigned proof artifacts exist.
- Any unsupported platform or provider failure must surface as degraded/manual-required state, not as a silent success.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch: `codex/tracking-plan-full-scope`.
- [ ] Touched files: tracking contract files, proof script, product docs, checklist, and this workpack doc.
- [ ] Validation commands and results: `node scripts/test/tracking-plan-contract-proof.mjs` passed.
- [ ] Proof artifacts under `output/tracking-plan-proof/24-ai-provider-routing/`.
- [ ] Product doc/checklist updates: owning feature doc, feature list, capability checklist, implementation checklist, tracking snapshot, and package READMEs updated.
- [ ] Known gaps/manual-required states: Android/iOS, precise desktop, provider delivery, runtime engines, retention/delete/export, Rust journal/SQLite, notifications, and UI remain proof-gated as applicable.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: WP24 proof source, tests, proof harness, feature doc,
      implementation checklist, WP24/WP33 docs, and generated proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-ai-provider-routing-proof.mjs` passed
      locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/24-ai-provider-routing/` and
      `test-results/tracking-ai-provider-routing-proof/`.
- [ ] Product doc/checklist updates: owning tracking feature doc, tracking
      implementation checklist, WP24, and WP33 updated; central product
      checklist row remains unchanged because this does not upgrade product
      support beyond the existing local proof tier.
- [ ] Known gaps/manual-required states: model execution, provider delivery,
      child-device runtime, rendered assistant/provider UI, authority,
      enforcement, production behavior, and physical-device proof remain
      unclaimed.
- [ ] Workpack id and branch: `codex/tracking-plan-full-continuation-a`.
- [ ] Touched files: WP24/WP32 AI stored-ref consumer proof source, tests,
      proof harness, feature doc, implementation checklist, WP24/WP32 docs,
      central product checklist row, and generated proof artifacts.
- [ ] Validation commands and results:
      `node scripts/test/tracking-ai-stored-ref-consumer-proof.mjs` passed
      locally.
- [ ] Proof artifacts under
      `output/tracking-plan-proof/24-ai-provider-routing/19-ai-stored-ref-consumer-proof.json`,
      `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json`,
      `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/30-ai-stored-ref-consumer-proof.json`,
      and `test-results/tracking-ai-stored-ref-consumer-proof/`.
- [ ] Product doc/checklist updates: owning tracking feature doc,
      implementation checklist, WP24, WP32, and central product capability row
      updated.
- [ ] Known gaps/manual-required states: AI model execution, assistant policy
      writes, assistant enforcement, provider delivery, child-device runtime,
      notification receipt, authority, production behavior, physical-device
      proof, and product-ready tracking remain unclaimed.
