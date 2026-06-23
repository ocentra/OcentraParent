# Workpack 07: Rollout Proof and Route Gate

Goal: define proof required before data custody claims are product-current.

Context to read:

- `docs/plans/data-custody-storage-plan/PLAN_STATE.md`
- `docs/plans/data-custody-storage-plan/CHECKLIST_INDEX.md`
- `docs/plans/data-custody-storage-plan/PROOF_INDEX.md`
- `docs/plans/data-custody-storage-plan/TEST_PROOF_EXPECTATIONS.md`
- `docs/plans/data-custody-storage-plan/WORKPACK_FAMILIES.md`
- `docs/PLAN_INDEX.md`
- `docs/FEATURE_ROUTE_INDEX.md`
- `docs/agent/PR_DONE_FLOW.md`

## Ownership boundary

```text
WP07 aggregates data-custody-storage-plan proof roots only.
Adjacent plans own their own implementation and may be referenced only by typed handoff proof.
WP07 cannot convert blockers, manual-required rows, or one proof family into broad readiness.
```

## Required proof pack

- Data custody matrix.
- Key and platform custody model.
- Parent-owned cloud sync matrix.
- Retention, delete, and tombstone proof.
- Export, import, backup, and restore proof.
- Report, query, notification, and assistant custody proof.
- Parent storage settings UI proof.
- Route and index sync.
- Adjacent handoff proof when a sibling plan is named.
- Manual-required gap register.

## Required rollout artifact fields

The rollout artifact must name, at minimum:

```text
rollout_id
accepted_proof_roots
missing_proof_roots
carried_blockers
manual_required_gaps
adjacent_handoff_refs
product_claims_allowed
product_claims_blocked
privacy_language_review_state
route_index_sync_state
feature_route_sync_state
no_claim
```

These are proof-routing fields, not implementation code prescriptions.

## Validation expectations

- Unit and contract tests for schema, export/import, retention, tombstone, and query behavior.
- Integration tests for sync, delete, and replay boundaries when implementation exists.
- Security tests for authZ, encryption, replay, and redaction.
- UI proof for state cards, restore preview, delete and disconnect flow, and claim-safe copy.
- Observability proof for logs, metrics, traces, alerts, and support diagnostics where selected.

## Expected proof names

- `data-custody.rollout.pr-gate`
- `data-custody.source.acceptance-route`
- `data-custody.observability.redaction`
- `data-custody.rollout.route-sync`
- `data-custody.rollout.adjacent-handoff-proof`
- `data-custody.rollout.manual-required-gap-register`

## Required proof files

```text
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/00-rollout-proof-pack.md
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/01-route-index-sync-proof.md
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/02-privacy-language-review-proof.md
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/03-manual-required-gap-register.md
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/04-adjacent-handoff-proof.md
output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/16-validation-commands.log
```

Failure conditions:

- PR_READY without negative privacy or security tests.
- Product docs claiming no data theft without explicit data classes, storage locations, and encryption or custody proof.
- Route or proof indexes drifting away from the selected workpack set.
- WP07 claims readiness while upstream proof roots are absent and not carried as explicit blockers.
- WP07 claims sync, export, restore, delete, report/query, assistant, or settings readiness from the wrong proof family.
- Adjacent plan completion is inferred instead of referenced through a typed handoff and no-claim boundary.
