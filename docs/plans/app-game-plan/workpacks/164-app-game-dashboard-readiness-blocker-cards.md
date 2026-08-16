# WP164 - App/game dashboard readiness blocker cards

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP164 - App/game dashboard readiness blocker cards`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Convert the service-backed app/game boundary counts and row states already
visible in the App/Game Sessions dashboard into parent-readable readiness
blocker cards.

## Implementation

- Dashboard metrics now include aggregate `Readiness blockers` count.
- The existing dashboard evidence drawer now surfaces blocker rows before raw
  source/boundary summaries:
  - missing approval action result for rows with approval authority but no
    approval action result;
  - AI classifier evidence-only review rows;
  - manual-required capability rows with adapter dispatch explicitly not
    claimed;
  - unknown approval review rows.
- Focused portal tests prove the blocker cards remain visible alongside the
  existing source and boundary summaries while raw executable path refs stay out
  of the parent-visible intent.

## No-Claim Boundary

This is parent dashboard readiness visibility only. It does not add policy
execution, policy persistence, provider delivery, adapter dispatch, broad
blocking, platform enforcement, raw private source rows, raw target values, or
private diagnostics. The E-A-owned SVG renderer is intentionally untouched; it
already renders dashboard metrics and evidence rows from the intent.

The central product capability checklist remains untouched while another lane
owns that file.

## Validation

See
`output/app-game-plan-proof/164-app-game-dashboard-readiness-blocker-cards/10-validation-commands.log`.
