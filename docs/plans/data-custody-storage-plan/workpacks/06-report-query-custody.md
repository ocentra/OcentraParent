# Workpack 06: Report Query Custody

Goal: define reports and queries as derived views over governed evidence, not a second data truth.

Context to read:

- `docs/plans/data-custody-storage-plan/EVENT_MODEL.md`
- `docs/plans/data-custody-storage-plan/DATA_CLASSIFICATION.md`
- `docs/features/reports-notifications-sync.md`
- `docs/features/evidence-store-query.md`
- `docs/expectations/notifications.md`
- `docs/expectations/evidence-storage.md`

In scope:

- Report source references, query cursors, pagination, citations, redaction, retention, and export/delete behavior.
- Notification payload boundaries.
- Assistant and report Q&A evidence references.
- Query performance and abuse limits.
- Derived materialization rules, cache invalidation, delete and tombstone effects, and cross-device stale or conflict states.

Out of scope:

- Event transport mechanics owned by `eventing-plan`.
- Portal rendering owned by `portal-ux-household-surfaces-plan`.
- Creating a second uncontrolled report database outside custody rules.

Acceptance:

- Derived views cite allowed source data only.
- Notification payloads are redacted to the chosen custody boundary.
- Query pagination and cursor behavior are stable.
- Stale or conflicting sync state is explicit and claim-safe.

Required query and report states:

- `derivedFresh`
- `derivedStale`
- `partiallyRedacted`
- `deletedSource`
- `syncConflict`
- `cursorExpired`
- `rateLimited`

Expected proof names:

- `data-custody.report.derived-source-matrix`
- `data-custody.report.deleted-expired-no-leak`
- `data-custody.query.cursor-pagination-stability`
- `data-custody.query.rate-limit-abuse`
- `data-custody.notification.payload-allow-deny`
- `data-custody.assistant.allowed-citation-proof`
- `data-custody.report.stale-conflict-state`

Failure conditions:

- Reports duplicate sensitive evidence outside retention control.
- Notifications leak child activity details beyond the chosen custody boundary.
- Assistant or report answers cite evidence the parent role cannot access.
- Query cache returns deleted, expired, wrong-household, or wrong-child data.

