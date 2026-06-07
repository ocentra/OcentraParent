# WP121 Timer Parent-Surface Child UX Local Outbox Bridge

## Summary

WP121 adds a parent-domain local outbox bridge for the unified native app plus
native game child UX path. It consumes schema-validated child UX local artifact
records and emits parent-owned notification local-outbox JSONL records for
deliverable child UX states.

## Implementation

- Added `app-game-child-facing-ux-local-outbox-bridge`.
- Reused the existing `NotificationLocalOutboxRecordSchema`.
- Linked deliverable child UX artifacts to `queued-local` in-app outbox records.
- Kept manual-required and unavailable artifacts blocked out of JSONL records.
- Added focused contract tests for linked, blocked, and overclaim rejection
  behavior.

## No-Claim Boundary

- Child delivery runtime remains unclaimed.
- Provider delivery and receipt ingestion remain unclaimed.
- Scheduler runtime remains unclaimed.
- Parent notification UI remains unclaimed.
- Adapter dispatch remains unclaimed.
- Platform enforcement and broad blocking remain unclaimed.
- Raw private source rows and private diagnostics remain excluded.

## Product Docs

- Updated `docs/features/app-game-control.md`.
- Updated `docs/plans/app-game-plan/implementation-checklist.md`.
- Updated `docs/plans/app-game-plan/workpacks/README.md`.
- Did not update `docs/product-capability-checklist.md` because another lane
  owns that checklist churn.
- Did not update `packages/parent-domain/package.json` because another lane owns
  that package export lock.
