# WP163 - App/game dashboard boundary count visibility

## Scope

Expose existing service-backed app/game boundary row counts in the main
App/Game Sessions dashboard intent, not only in the dedicated policy-readiness
route.

## Implementation

- The dashboard row model now carries evidence-claim, identity, approval
  authority, approval action-result, platform authority, AI classifier, and
  aggregate boundary row counts from each app-use/game read-model row.
- Dashboard metrics now include aggregate `Boundary rows` and `AI classifier`
  counts.
- The existing evidence-drawer list now includes parent-visible boundary
  summaries for rows with boundary evidence, including approval
  authority/action-result and AI classifier counts.
- Focused portal tests prove the dashboard exposes those counts while retaining
  raw executable path redaction, launcher-only boundaries, unknown review rows,
  and manual-required rows.

## No-Claim Boundary

This is parent dashboard visibility only. It does not add policy execution,
policy persistence, provider delivery, adapter dispatch, broad blocking,
platform enforcement, raw private source rows, raw target values, or private
diagnostics. The E-A-owned SVG renderer is intentionally untouched; it already
renders dashboard metrics and evidence rows from the intent.

The central product capability checklist remains untouched while another lane
owns that file.

## Validation

See
`output/app-game-plan-proof/163-app-game-dashboard-boundary-count-visibility/10-validation-commands.log`.
