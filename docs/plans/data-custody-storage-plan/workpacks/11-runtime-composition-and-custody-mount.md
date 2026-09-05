<!-- agent-capsule -->

> Plan: `data-custody-storage-plan`
> Workpack: `WP-data-custody-storage-plan-11-runtime-composition-and-custody-mount`
> Status: planned dependency-blocked source work; WP09/WP10 source, Account WP05 base and WP05A owner-coordinator source, production composition, tests, proof, and PR readiness are open.

# WP11 Runtime Composition And Custody Mount

## Intent

Define the explicit parent-runtime composition boundary that mounts the WP05
base ledgers and manual-required gates to dependency-owned custody capabilities.
This workpack is the only route for composing those owners; it does not move
their authority, persistence, or producer logic into `parent-runtime-core`.

## Scope and ownership

In scope:

- parent-runtime-core composition of the WP05 base scheduler, restore/migration
  ledgers, reconciliation, rollback/executor seams, and Eventing/outbox gate;
- private, opaque mount seams for the Account WP05A multi-owner coordinator
  outcome for `ExportDeleteData`/`ImportRestoreData`-style ParentOwner and
  step-up-bound actions plus the typed Data handoff, the WP05 base authority
  transaction/CAS handoff, key/import custody, sealed producer-artifact
  custody, WP09 provider operation capability, and WP10 owner-derived
  outcomes; remote-view/remote-control capability and controller-lease
  reservations are not claimed by this Data route;
- fail-closed `manual-required` or blocked outcomes when any dependency-owned
  capability is absent, stale, revoked, ambiguous, or not current;
- the expected integration boundary and its no-claim test/proof obligations.

Out of scope:

- Account authority transaction/CAS implementation or authority minting;
- key/import custody, provider byte custody, producer artifact custody, or
  data-class mutation implementation;
- WP09 provider-neutral byte decisions or WP10 producer-handoff orchestration;
- publicizing private traits, handles, or constructors;
- production caller wiring, proof publication, or runtime-completeness claims.

## Reviewed planned source and test roots

- Composition owner: `crates/parent-runtime-core/src/data_custody_runtime_composition.rs`.
- Mount owner: `crates/parent-runtime-core/src/data_custody_runtime_composition_mount.rs`.
- Expected integration test: `crates/parent-runtime-core/tests/integration/data_custody_runtime_composition.rs`.

These roots are intentionally absent at this routing checkpoint. They are
planned ownership and expected-test obligations only; this workpack does not
authorize placeholder source, public trait exports, or test-only completion.

## Required handoffs and dependency gates

The composition root remains blocked until all of the following are available
as typed, owner-derived, non-forgeable capabilities:

- Account WP05A's true durable multi-owner coordinator/recovery and opaque
  Account/Device Trust/Parent Step-Up/Protected Custody outcome for the Data
  actions and typed Data handoff, consuming the existing WP08 Account
  transaction-scoped repository/read/CAS seam;
- Account WP05's base authority transaction/CAS handoff and parent-runtime
  ledgers;
- Data key/import custody owner, including current key/decrypt and import
  integrity custody state;
- the producer-owned sealed artifact-custody handoff bound to the WP05 job or
  restore operation;
- WP09's provider operation capability and opaque operation outcome;
- WP10's producer-handoff outcome, including partial/manual-required states.

WP05 is the base dependency for its schema, pure decisions, durable ledgers,
reconciliation, Eventing/outbox composition, and manual-required gates. WP09
and WP10 consume that base independently. Account WP05A is a direct reviewed-
implementation dependency for its owner-coordinator outcome while retaining
the WP05 base edge. This workpack consumes WP05, WP05A, WP09, and WP10; neither
WP09 nor WP10 may depend on this composition workpack.

## Acceptance criteria

- Composition preserves WP05's durable state and never creates a second ledger.
- Every external capability is owner-resolved and opaque; request/JSON input
  cannot select authority, keys, integrity, provider, or producer outcomes.
- Missing, stale, revoked, ambiguous, or unavailable handoffs remain blocked or
  manual-required, with no synthetic success or fabricated receipt.
- Private traits and mount constructors remain private to their owning Rust
  boundary until the dependency owners provide a reviewed implementation.
- The expected integration test covers the composition gate and negative
  handoff states before any runtime-completeness claim is considered.

## Expected tests and proof (deferred)

The expected integration test is the planned root above. It must cover
authority/CAS currentness, key/import custody, producer artifact custody,
provider operation capability, WP10 outcomes, restart/reconciliation binding,
manual-required degradation, and no-fake-success/no-resurrection boundaries.
No tests, proof artifacts, focused gates, or runtime completion are claimed by
this routing packet.

Expected proof root:
`output/data-custody-storage-plan-proof/11-runtime-composition-and-custody-mount/`.

## No-claim boundary

This route does not claim Account authority, key custody, provider execution,
producer execution, data-class mutation, a production caller, runtime
completeness, proof, PR readiness, or plan completion.
