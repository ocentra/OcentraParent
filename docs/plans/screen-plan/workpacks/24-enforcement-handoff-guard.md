# 24 Enforcement Handoff Guard

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

- [x] Define enforcement handoff payload.
- [x] Include summary ref.
- [x] Include parent policy rule.
- [x] Include confidence/unknown state.
- [x] Block raw model text/pixel handoff.
- [x] Add audit event.

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
