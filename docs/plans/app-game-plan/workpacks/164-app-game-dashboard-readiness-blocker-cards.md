# WP164 - App/game dashboard readiness blocker cards

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
