# Workpack 06: Report Query Custody

Goal: define reports and queries as derived views over governed evidence, not a second data truth.

Context to read:

- `docs/features/reports-notifications-sync.md`
- `docs/features/evidence-store-query.md`
- `docs/expectations/notifications.md`
- `docs/expectations/evidence-storage.md`

In scope:

- Report source references, query cursors, pagination, citations, redaction, retention, and export/delete behavior.
- Notification payload boundaries.
- Assistant/report Q&A evidence references.
- Query performance and abuse limits.
- Derived materialization rules, cache invalidation, delete/tombstone effects, and cross-device stale/conflict states.

Out of scope:

- Event transport mechanics owned by `eventing-plan`.
- Portal rendering owned by `portal-ux-household-surfaces-plan`.
- Creating a second uncontrolled report database outside custody rules.

Decision tree:

| If the assignment touches... | Route                                                                  |
| ---------------------------- | ---------------------------------------------------------------------- |
| Report/query source truth    | this workpack and WP01 custody source of truth                         |
| Notification payloads        | reports/notifications feature plus notification owning plan            |
| Assistant Q&A over reports   | ai-plan for answer generation, this workpack for evidence refs/custody |
| Portal report UI             | portal-ux-household-surfaces-plan                                      |
| Delete/export interaction    | WP04 and WP05 before report claim changes                              |

Required query/report states:

- `derivedFresh`: report derived from current allowed evidence.
- `derivedStale`: report may be shown with age/source warning and no live claim.
- `partiallyRedacted`: some evidence hidden due to retention, role, or custody boundary.
- `deletedSource`: report row remains only as allowed minimal audit/citation shell.
- `syncConflict`: provider/local state conflict prevents final report claim.
- `cursorExpired`: query cursor cannot continue safely.
- `rateLimited`: abuse/rate guard blocks query.

Decisions required:

- What report data is materialized versus derived.
- How citations point to evidence after retention/delete.
- What notification payloads may leave the device/cloud boundary.
- How queries behave during provider sync conflict or partial outage.
- Which report summaries can be shared remotely and which require local parent-owned storage.

Expected artifacts:

- Report/query custody matrix.
- Citation and redaction rules.
- Notification payload allow/deny list.
- Query abuse and pagination proof plan.
- Materialized-view invalidation and cache freshness rules.
- Assistant/report Q&A evidence reference rules.

Expected proof:

- Deleted/expired evidence does not leak through reports.
- Notification payload redaction.
- Query pagination/cursor stability.
- Assistant answer cites only allowed evidence.
- Rate limit/abuse proof.
- Stale/partial sync report state proof.
- Exported report respects retention/delete proof.

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
- Assistant/report answers cite evidence the parent role cannot access.
- Query cache returns deleted, expired, wrong-household, or wrong-child data.
