# Workpack 07: Rollout Proof And Route Gate

## Status

`in progress / limited retention lifecycle proven`

## Goal

Define the proof and route gate required before any data-custody claim can be treated as product-current, while carrying missing roots and upstream blockers explicitly instead of flattening them into readiness.

## Ownership boundary

```text
WP07 aggregates data-custody-storage-plan proof roots only.
Adjacent plans own their own implementation and may be referenced only by typed handoff proof.
WP07 cannot convert blockers, manual-required rows, or one proof family into broad readiness.
```

## Execution truth

- A clean checkout does not contain the ignored `output/` proof root that older
  WP07 checklist rows cited. Those rows cannot be used as current aggregate
  evidence.
- This packet now proves one real retention lifecycle: a Rust-owned expired
  custody action is journaled idempotently, persisted through the child-runtime
  durable tombstone outbox, survives reopen, and remains pending until explicit
  acknowledgement. A typed non-delete action is rejected before it can create a
  tombstone intent.
- The lifecycle proof is deliberately narrower than the rollout gate. It does
  not establish WP01/WP02/WP03/WP05/WP06/WP08 aggregate acceptance, provider
  execution, portal application, or plan-wide readiness.

## Required rollout artifact fields

- `rollout_id`
- `accepted_proof_roots`
- `missing_proof_roots`
- `carried_blockers`
- `manual_required_gaps`
- `adjacent_handoff_refs`
- `product_claims_allowed`
- `product_claims_blocked`
- `privacy_language_review_state`
- `route_index_sync_state`
- `feature_route_sync_state`
- `no_claim`

## Proof pack outcome

- Accepted proof roots:
  - None for aggregate route status until durable, reviewable proof artifacts
    are available from a clean checkout.
- Missing proof roots:
  - WP01, WP02, WP03, WP04, WP05, WP06, and WP08 aggregate inputs remain
    unaccepted by this gate.
- Carried blockers:
  - The output proof-root retention/publication model is unresolved: `output/`
    is ignored, so a fresh clone cannot audit the older cited artifact paths.
  - Provider, portal, AI, notification, Cloudflare, account, and device-trust
    runtime evidence remains owned by adjacent plans.

## Current code proof

```text
storage-custody action event
  -> child runtime durable tombstone intent (atomic outbox write)
  -> idempotent NDJSON event journal append
  -> process reopen recovery
  -> explicit terminal acknowledgement compacts the row to a minimal
     terminal idempotency marker (the marker is retained for replay protection)
```

Focused test owners:

- `crates/storage-custody-core/tests/unit/retention_delete_tombstone_store.rs`
- `crates/child-runtime/tests/unit/runtime_gate.rs`

## Validation expectations for this packet

- Proof-root existence and file inventory checks
- Upstream validation-log inspection for carried blockers
- Route/index truth review for `PLAN_STATE.md`, `WORKPACK_INDEX.md`, `CHECKLIST_INDEX.md`, and `PROOF_INDEX.md`
- Focused doc/proof hygiene checks on the touched WP07 files

## Required proof files

- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/00-rollout-proof-pack.md`
- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/01-route-index-sync-proof.md`
- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/02-privacy-language-review-proof.md`
- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/03-manual-required-gap-register.md`
- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/04-adjacent-handoff-proof.md`
- `output/data-custody-storage-plan-proof/07-rollout-proof-and-route-gate/16-validation-commands.log`

## No-claim boundary

- WP07 does not claim plan-wide custody readiness.
- WP07 does not claim provider runtime, portal runtime, AI runtime, notification runtime, Cloudflare runtime, account authority, or device-trust readiness.
- WP07 does not upgrade missing or blocked upstream proof roots into accepted readiness.

## Failure conditions

- PR_READY without negative privacy or security tests.
- Product docs claiming no data theft without explicit data classes, storage locations, and encryption or custody proof.
- Route or proof indexes drifting away from the selected workpack set.
- WP07 claims readiness while upstream proof roots are absent and not carried as explicit blockers.
- WP07 claims sync, export, restore, delete, report/query, assistant, or settings readiness from the wrong proof family.
- Adjacent plan completion is inferred instead of referenced through a typed handoff and no-claim boundary.
