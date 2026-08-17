<!-- agent-capsule -->

> Plan: `data-custody-storage-plan`
> Workpack: `WP-data-custody-storage-plan-10-restore-orchestration-and-producer-handoffs`
> Status: routed source work only; implementation, tests, proof, and PR readiness are open.

# WP10 Restore Orchestration And Producer Handoffs

## Intent

Close the missing orchestration boundary between bundle preflight and the
data-class owners that perform durable restore, migration, and rollback.
Restore orchestration coordinates; it does not become a data-class producer
and cannot mint success receipts for work it did not observe.

## Scope and ownership

In scope:

- durable preflight, apply, migration, rollback, and idempotency receipts;
- orchestration of data-class producer/consumer handoffs with explicit owner
  identity, version, household, and tombstone context;
- Account authority/confirmation and session/device freshness checks at the
  orchestration boundary;
- crash/restart recovery, monotonic state transitions, retry, partial restore,
  and manual-required outcomes;
- no-resurrection enforcement for deleted/tombstoned data.

Out of scope:

- parent-local/provider byte storage (WP09);
- data-class mutation logic, child-runtime filesystem ownership, or event-bus
  implementation;
- Account authority implementation, Device Trust key material, provider SDKs,
  portal/desktop UI, or proof publication.

## Required handoffs and dependencies

- Data WP02/WP03/WP04/WP05 provide key, provider, retention/tombstone, and bundle
  contracts and states.
- Account WP05 provides current authority/capability/session/lease composition;
  Account WP08 provides the trusted authority/confirmation contract where
  required.
- Eventing supplies durable ordering/replay primitives; Device Trust supplies
  independent trusted-device readiness where the action requires it.
- Data-class owners must expose typed producer/consumer handoffs and return
  owner-derived outcomes; WP10 records/coordinates them but does not fabricate
  them.

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

