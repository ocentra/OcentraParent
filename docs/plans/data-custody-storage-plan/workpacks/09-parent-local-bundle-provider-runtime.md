<!-- agent-capsule -->

> Plan: `data-custody-storage-plan`
> Workpack: `WP-data-custody-storage-plan-09-parent-local-bundle-provider-runtime`
> Status: planned source work; Account WP05 base and WP05A owner-coordinator source, implementation, tests, proof, and PR readiness are open.

# WP09 Parent Local Bundle Provider Runtime

## Intent

Close the missing parent-local/provider-neutral byte-custody boundary for
encrypted data bundles. This downstream route owns pure byte-custody
decisions, verification, atomic-operation planning, and opaque adapter ports.
The WP05 parent-runtime owner owns durable scheduler/job state and restart
reconciliation; this workpack does not duplicate that ledger or invent a cloud
provider or move data-class authority into the storage layer.

## Scope and ownership

In scope:

- parent-local encrypted bundle byte persistence and retrieval;
- cryptographic byte-level hash/signature verification before acceptance;
- atomic write, replace, recovery, and corruption quarantine semantics;
- manual and scheduled backup operation planning, retry, and idempotency
  decisions consumed by the parent-runtime durable job owner;
- a provider-neutral adapter boundary that returns opaque custody status and
  never exposes readable child payloads;
- explicit no-provider, no-fallback, and manual-required states.

Out of scope:

- cloud SDK/OAuth/provider implementation or provider credentials;
- Account household/device authority or Device Trust key ownership;
- data-class mutation, restore/apply/rollback orchestration, or receipt minting;
- durable scheduler/job persistence, restore/migration ledgers, restart
  reconciliation, or executor mounting (owned by WP05 parent-runtime-core);
- portal/desktop UI and proof artifact publication.

## Reviewed planned source and test roots

- Pure decision/port owner: `crates/storage-custody-core/src/parent_local_bundle_provider_runtime.rs`.
- Expected test owner: `crates/storage-custody-core/tests/unit/parent_local_bundle_provider_runtime.rs`.

Both paths are intentionally absent at this routing checkpoint. They declare
the downstream pure decision/port boundary; this workpack Markdown cannot
satisfy the implementation requirement, and no placeholder source is
accepted. Durable scheduler/job persistence and restart reconciliation are
owned by the WP05 `crates/parent-runtime-core` route listed in
`05-export-import-backup-recovery.md`.

## Required handoffs and dependencies

- Data WP02 supplies key-custody and decrypt-scope decisions.
- Data WP03 supplies parent-owned provider/sync state and encryption-before-upload
  boundaries.
- Data WP04 supplies retention/delete/tombstone ordering and no-resurrection
  constraints.
- Data WP05 supplies bundle manifest/preflight/integrity contracts.
- Data WP05 base parent-runtime-core supplies the durable scheduler/job ledger,
  restore/migration ledger, and restart reconciliation seam; WP09 returns
  owner-bound provider operation outcomes to that base and does not persist a
  second ledger or depend on the later WP11 composition mount.
- Account WP05 remains the authorization consumer and base Account handoff.
  Account WP05A supplies the durable multi-owner coordinator outcome for
  ParentOwner/step-up-bound data actions and the typed Data handoff when a
  provider operation requires authority. Its remote-view/remote-control
  capability and controller-lease reservations are outside this Data route.
  WP05A consumes the existing WP08-owned transaction-scoped Account
  repository/read/CAS seam; WP09 does not duplicate Account authority or
  create a second reservation ledger.
- The coordinator outcome remains jointly dependent on Account, Device Trust,
  Parent Step-Up, and Protected Custody WP01 owner boundaries; missing
  protected admission or any owner result remains blocked/manual-required.
- Conditional handoffs only: if the selected implementation requires a
  trusted-device signer/readiness input, route that input to the exact Device
  Trust owner; if it requires a shared durable event/retry primitive, route
  that primitive to the exact Eventing owner. No exact owner workpack is
  selected by the current live-code audit, so neither plan is a current hard
  dependency and no graph edge is asserted here.

## Acceptance criteria

- Bytes are encrypted before persistence or provider handoff and are never
  accepted solely because a manifest says they are valid.
- Hash/signature verification binds the exact bundle bytes, household, source,
  version, and key/signing context; mismatch quarantines without mutation.
- Writes use an atomic commit/recovery protocol and do not publish partial
  bundles after interruption.
- Manual and scheduled jobs persist retry/restart/idempotency state without
  duplicating a bundle or silently changing retention/tombstone state.
- Provider-neutral adapters expose only opaque references/status and preserve
  manual-required state when no supported provider exists.
- No runtime path accepts authority selectors, keys, or provider identities
  from request/JSON data.

## Ownership correction (2026-08-18)

WP09 is a downstream pure storage/adapter-port route from the WP05 base layer
and the Account WP05A multi-owner data-action handoff.
The durable parent scheduler/job lifecycle, restart reconciliation, and
Eventing/outbox composition are WP05 responsibilities. No provider SDK, OAuth
flow, filesystem adapter, caller-supplied authority, duplicate ledger, or
dependency on the later WP11 composition mount is permitted here.

## Expected tests and proof (deferred until source wave is complete)

- exact byte hash/signature mismatch and wrong-household/key negatives;
- atomic interruption/restart and partial-write recovery;
- retry/idempotency and duplicate-job suppression;
- encrypted-before-persistence/provider boundary;
- unsupported provider/manual-required and provider disconnect states;
- redaction of payloads, keys, provider identifiers, and local paths.

Expected proof root: `output/data-custody-storage-plan-proof/09-parent-local-bundle-provider-runtime/`.

## No-claim boundary

This route does not claim provider SDK execution, Account authority, Device
Trust readiness, data-class restore/apply, portal rendering, or production
custody completion until the owning source, expected tests, focused gates, and
proof are accepted.
