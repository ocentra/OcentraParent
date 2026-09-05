# Workpack 07: Rollout Proof And Route Gate

## Status

`blocked / production custody source reachable; Account WP04/WP05 composition, tests, and aggregate acceptance open`

## Goal

Define the proof and route gate required before any data-custody claim can be treated as product-current, while carrying missing roots and upstream blockers explicitly instead of flattening them into readiness.

## Ownership boundary

```text
WP07 aggregates data-custody-storage-plan proof roots only.
Adjacent plans own their own implementation and may be referenced only by typed handoff proof.
WP07 cannot convert blockers, manual-required rows, or one proof family into broad readiness.
Trusted authority composition is hard-blocked on Account WP04's durable correlated export/delete handoff and Account WP05's current household/member/device/session authorization composer.
```

## Execution truth

- A clean checkout does not contain the ignored `output/` proof root that older
  WP07 checklist rows cited. Those rows cannot be used as current aggregate
  evidence.
- The integrated source now contains a real internal service command path:
  `ChildAgentIngress::submit_storage_custody_action` queues
  `ChildAgentCommand::PublishStorageCustody`, dispatch remains behind the
  dynamic Device Trust readiness gate, and `ChildStorageCustodyRuntime::execute`
  records durable effect/journal/tombstone state. Startup recovery replays
  pending typed rows before readiness and terminal acknowledgement remains
  delivery-owned.
- Default service composition deliberately installs a manual-required custody
  authority. No Account/family-owned trusted authority adapter or external
  upstream production caller currently supplies the opaque authority handle,
  so the executable path remains fail-closed rather than pretending readiness.
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
trusted custody authority + StorageCustodyExecutionRequest
  -> ChildAgentIngress::submit_storage_custody_action
  -> ChildAgentCommand::PublishStorageCustody
  -> dynamic Device Trust readiness validation
  -> ChildStorageCustodyRuntime::execute
  -> durable effect ledger + idempotent NDJSON journal
  -> child-runtime tombstone intent for the delete/expiry path
  -> startup recovery and explicit delivery-owned terminal acknowledgement
```

Expected-test wave debt:

- `crates/storage-custody-core/tests/unit/retention_delete_tombstone_store.rs`
  still imports the deleted core store and must move to the child-runtime owner
  or be rewritten through its public flow.
- `crates/child-runtime/tests/unit/runtime_gate.rs` also imports the deleted
  core module and must be migrated.
- `crates/child-runtime/tests/unit/runtime_gate_tombstone_recovery.rs` remains
  the focused restart/recovery owner, but no current test run is claimed here.

The concrete child-service startup now invokes `journal.recover()` and
`ChildRuntimeTombstoneEventFlow::recover_pending()` before readiness in
`crates/child-runtime/src/service.rs`. This source wiring is not restart proof,
aggregate route acceptance, or a claim that all downstream delivery paths are
complete.

## Production reachability audit (2026-08-17, source checkpoint `7a1e1c389`)

The shipped binary `crates/child-runtime/src/bin/ocentra-child-agent-service.rs`
calls `run_child_agent_service()`. Initialization reaches the durable journal
and `RetentionDeleteTombstoneStore`, then invokes `journal.recover()` and
`ChildRuntimeTombstoneEventFlow::recover_pending()` before readiness. This is a
real fail-closed recovery path.

Production source now reaches the custody runtime through the child-service
ingress/command/dispatch path and records durable effect state before the
delete/tombstone flow. The unresolved production boundary is authority and
external composition, not another synthetic publication helper:
`initialize_with_paths` always installs
`ChildStorageCustodyAuthorityHandle::manual_required()`, the trusted handle
constructor is crate-private, and no production Account/family adapter or
external caller supplies it. This is intentionally fail-closed. The next source
packet must consume an owning-plan authority without accepting selectors,
identity, generation, or authority claims from request/JSON input.

No tests, builds, proof generation, precommit, CI, or PR were run for this
source checkpoint.

Graph dependencies:

- `WP-account-identity-family-plan-04-invites-recovery-lifecycle`
- `WP-account-identity-family-plan-05-device-ownership-authz`

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
