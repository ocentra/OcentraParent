# WP162 - App/game policy readiness parent-surface detail rows

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP162 - App/game policy readiness parent-surface detail rows`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Make the existing App/Game Sessions policy-readiness route more useful to a
parent by rendering the service-backed readiness count fields and explicit
row-level blocker reasons.

## Implementation

- `portal-domain` policy-readiness intent now includes evidence-claim,
  identity, approval authority, approval action-result, platform authority, and
  AI classifier row counts from the service-backed read model.
- Each readiness row now includes a parent-visible reason that distinguishes
  ready, manual-required, and missing states.
- The focused portal test proves zero-count approval action and AI classifier
  rows stay visible, the AI classifier manual-required row is explicit, absent
  evidence refs show `Not reported`, and adapter dispatch remains `Not
claimed`.

## No-Claim Boundary

This is parent-surface visibility only. It does not add policy execution,
policy persistence, provider delivery, adapter dispatch, broad blocking,
platform enforcement, raw private source rows, raw target values, or private
diagnostics. The central product capability checklist remains untouched while
another lane owns that file.

## Validation

See
`output/app-game-plan-proof/162-app-game-policy-readiness-parent-surface-detail-rows/10-validation-commands.log`.
