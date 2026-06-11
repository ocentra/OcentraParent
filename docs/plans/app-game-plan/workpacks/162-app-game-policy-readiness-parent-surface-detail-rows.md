# WP162 - App/game policy readiness parent-surface detail rows

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
