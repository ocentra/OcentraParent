# 24 Enforcement Handoff Guard

<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `24 Enforcement Handoff Guard`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Target State

No enforcement from raw pixels or raw AI text; dry-run and manual-required guards are implemented.

## Current State

The domain-level enforcement handoff guard proof now exists in
`@ocentra-parent/parent-domain` and writes
`output/screen-plan-proof/screen-ai-enforcement-handoff-guard/proof-summary.json`.
It proves a screen-derived policy decision can create only a guarded dry-run or
manual-required handoff payload when summary, local-AI result, parent-rule,
confidence, and audit refs are present. It does not claim adapter execution or
broad browser/network/mobile enforcement.

## Checklist

- [ ] Define enforcement handoff payload.
- [ ] Include summary ref.
- [ ] Include parent policy rule.
- [ ] Include confidence/unknown state.
- [ ] Block raw model text/pixel handoff.
- [ ] Add audit event.

## Proof

- `packages/parent-domain/tests/screen-ai-enforcement-handoff-guard-proof.test.ts`
  shows AI output alone cannot enforce: the input must include a dry-run policy
  decision that has not already been handed off, an enabled parent policy rule,
  summary/local-AI/audit evidence refs already present on the decision, and
  `rawPixelsIncluded:false`, `rawModelTextIncluded:false`,
  `rawScreenshotRetained:false`, and `localAiAuthorityClaimed:false`.
- `scripts/test/screen-ai-enforcement-handoff-guard-proof.mjs` builds the
  payload from the real schema, writes proof artifacts, and confirms the
  payload carries refs and an audit event only.
