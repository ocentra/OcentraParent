<!-- agent-capsule -->

> Plan: `data-custody-storage-plan`
> Workpack: `WP-data-custody-storage-plan-10-restore-orchestration-and-producer-handoffs`
> Status: routed source work only; implementation, tests, proof, and PR readiness are open.

# WP10 Restore Orchestration And Producer Handoffs

## Intent

Close the missing pure orchestration boundary between bundle preflight and the
data-class owners that perform durable restore, migration, and rollback. The
WP05 parent-runtime owner persists the restore/migration ledger, mounts the
executor/rollback seam, and reconciles restarts; this downstream route derives
safe plans and producer handoffs without duplicating that owner.

## Scope and ownership

In scope:

- preflight, apply, migration, rollback, and idempotency plan derivation for
  the WP05 parent-runtime ledger;
- orchestration of data-class producer/consumer handoffs with explicit owner
  identity, version, household, and tombstone context;
- Account authority/confirmation and session/device freshness checks at the
  orchestration boundary;
- monotonic state transitions, retry, partial restore, and manual-required
  outcomes consumed by the parent-runtime restart reconciler;
- no-resurrection enforcement for deleted/tombstoned data.

Out of scope:

- parent-local/provider byte storage (WP09);
- data-class mutation logic, child-runtime filesystem ownership, or event-bus
  implementation;
- durable restore/migration ledgers, restart reconciliation, executor/rollback
  mounting, or Eventing/outbox composition (owned by WP05 parent-runtime-core);
- Account authority implementation, Device Trust key material, provider SDKs,
  portal/desktop UI, or proof publication.

## Reviewed planned source and test roots

- Pure orchestration owner: `crates/storage-custody-core/src/restore_orchestration_and_producer_handoffs.rs`.
- Expected test owner: `crates/storage-custody-core/tests/unit/restore_orchestration_and_producer_handoffs.rs`.

Both paths are intentionally absent at this routing checkpoint. They declare
the downstream pure orchestration boundary; this workpack Markdown cannot
satisfy the implementation requirement, and no placeholder source is
accepted. Durable ledgers, restart reconciliation, and executor/rollback
mounting are owned by the WP05 `crates/parent-runtime-core` route listed in
`05-export-import-backup-recovery.md`.

## Required handoffs and dependencies

- Data WP02/WP03/WP04/WP05 provide key, provider, retention/tombstone, and bundle
  contracts and states. WP05 is the base layer; its later composition mount is
  not a WP10 dependency.
- Account WP05 provides current authority/capability/session/lease composition;
  Account WP08 provides the trusted authority/confirmation contract where
  required.
- Conditional handoffs only: if the selected implementation requires shared
  durable ordering/replay primitives, route them to the exact Eventing owner;
  if an action requires independent trusted-device readiness, route that input
  to the exact Device Trust owner. WP10 returns owner-derived outcomes to the
  WP05 base and later WP11 composition; it does not depend on WP11 or WP09.
- Data-class owners must expose typed producer/consumer handoffs and return
  owner-derived outcomes; WP10 records/coordinates them but does not fabricate
  them.

## Ownership correction (2026-08-18)

WP10 is a downstream pure storage-custody orchestration route from the WP05
base layer. The WP05 `parent-runtime-core` owner persists the operation ledger
and composes the real Eventing journal/outbox seam; the later WP11 route mounts
the opaque executor, Account/key, provider, and producer ports. No
caller-supplied authority/integrity boolean, provider SDK, filesystem adapter,
or fake producer result is permitted here.

## Acceptance criteria

- Preflight is non-mutating and binds bundle, household, authority, key,
  tombstone, schema, and migration context before apply.
- Apply, migration, rollback, and partial outcomes are durable, monotonic,
  replay-safe, and tied to an opaque operation/idempotency reference.
- Confirmation is trusted, authority-bound, expiry/replay checked, and cannot
  be supplied as a caller-selected boolean or authority selector.
- Producer handoffs identify the owning data class and return an inspectable
  result before a receipt advances; absent/failed owners remain blocked or
  manual-required.
- Tombstones and retention expiry cannot be bypassed by restore or migration.
- Crash/restart recovery resumes or safely quarantines work without duplicate
  mutation, false success, or resurrection.

## Expected tests and proof (deferred until source wave is complete)

- non-mutating preflight and wrong-household/key/authority negatives;
- confirmation expiry, replay, wrong-operation, and wrong-household negatives;
- monotonic apply/rollback/idempotency transitions under retry and restart;
- partial producer failure and owner-missing/manual-required outcomes;
- tombstone/no-resurrection and migration rollback cases;
- receipt provenance and redaction/no-fake-success checks.

Expected proof root: `output/data-custody-storage-plan-proof/10-restore-orchestration-and-producer-handoffs/`.

## No-claim boundary

This route does not claim any data-class writer, provider/local filesystem
implementation, Account authority implementation, Device Trust readiness,
portal/desktop behavior, proof, or plan completion until those owners deliver
their own accepted source and validation evidence.
