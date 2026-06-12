# Workpack 07: Rollout Proof and Route Gate

Goal: define proof required before data custody claims are product-current.

Required proof pack:

- Data custody matrix.
- Key/encryption custody decision.
- Parent-owned cloud sync matrix.
- Retention/delete/tombstone proof.
- Export/import/backup/restore proof.
- Report/query/notification custody proof.
- Privacy language review.
- Route/index sync.

Validation expectations:

- Unit and contract tests for schema, export/import, retention, tombstone, and query behavior.
- Integration tests for sync/deletion/replay boundaries when implementation exists.
- Security tests for authZ, encryption, secret leakage, replay, and redaction.
- Observability proof for logs, metrics, traces, alerts, and support diagnostics.

Failure conditions:

- PR_READY without negative privacy/security tests.
- Product docs claiming no data theft without explicit data classes, storage locations, and encryption/custody proof.

## Research Gate

This rollout gate cannot be closed from docs alone. The assigned agent must first inspect existing eventing, logging, portal report, local storage, sync/export, and cloud-provider code/docs. Any unresolved encryption, provider sync, retention, or parent-owned-storage decision must be discussed with Sujan before changing product status.

## Required Route Updates

- `docs/PLAN_INDEX.md` must route data custody work here before `eventing-plan` when the task is retention, export, sync, encryption, delete, cloud, or query custody.
- `docs/FEATURE_ROUTE_INDEX.md` must keep `evidence-store-query` and `reports-notifications-sync` tied to this plan for custody claims.
- Adjacent plans may be referenced only for implementation handoffs; they do not prove custody completion.

## Minimum DONE Report

The report must name:

- data classes touched.
- source-of-truth owner.
- encryption/key decision.
- retention/delete behavior.
- export/import behavior.
- cloud sync provider state.
- proof artifacts.
- skipped risks and Sujan decisions still required.
