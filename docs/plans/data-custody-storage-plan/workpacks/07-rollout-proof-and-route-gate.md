# Workpack 07: Rollout Proof and Route Gate

Goal: define proof required before data custody claims are product-current.

Context to read:

- `docs/plans/data-custody-storage-plan/PLAN_STATE.md`
- `docs/plans/data-custody-storage-plan/CHECKLIST_INDEX.md`
- `docs/plans/data-custody-storage-plan/PROOF_INDEX.md`
- `docs/plans/data-custody-storage-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/PLAN_INDEX.md`
- `docs/FEATURE_ROUTE_INDEX.md`
- `docs/agent/PR_DONE_FLOW.md`

Required proof pack:

- Data custody matrix.
- Key and platform custody model.
- Parent-owned cloud sync matrix.
- Retention, delete, and tombstone proof.
- Export, import, backup, and restore proof.
- Report, query, notification, and assistant custody proof.
- Parent storage settings UI proof.
- Route and index sync.

Validation expectations:

- Unit and contract tests for schema, export/import, retention, tombstone, and query behavior.
- Integration tests for sync, delete, and replay boundaries when implementation exists.
- Security tests for authZ, encryption, secret leakage, replay, and redaction.
- UI proof for state cards, restore preview, delete and disconnect flow, and claim-safe copy.
- Observability proof for logs, metrics, traces, alerts, and support diagnostics.

Expected proof names:

- `data-custody.rollout.pr-gate`
- `data-custody.source.acceptance-route`
- `data-custody.observability.redaction`
- `data-custody.rollout.route-sync`

Failure conditions:

- PR_READY without negative privacy or security tests.
- Product docs claiming no data theft without explicit data classes, storage locations, and encryption or custody proof.
- Route or proof indexes drifting away from the selected workpack set.

