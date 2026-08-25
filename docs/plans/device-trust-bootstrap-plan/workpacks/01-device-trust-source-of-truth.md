# Workpack 01: Device Trust Source of Truth

Purpose: define trust ownership, trust states, bootstrap lifecycle, and cross-plan boundaries.

## Owns

- Trust vocabulary.
- Parent vs child authority.
- Bootstrap, revoke, reset, and re-pair transitions.
- Relationship to login, billing entitlement, remote access, and policy delivery.

## Exit condition

- The trust state machine is explicit and typed.
- Login is separate from trust.
- Child devices never own the trust root.
- Adjacent plan boundaries are written down in one place.

## Proof target

- `output/device-trust-bootstrap-plan-proof/01-*`
- Route-sync note at `output/device-trust-bootstrap-plan-proof/09-cross-plan-route-gate/06-route-sync-proof.md`.
- Both paths are local generated evidence only and must remain untracked.

## Current audit state

- No generated proof file is committed. The narrow Rust parent-presence custody slice is reviewable through its source, visible crate tests, and current validation runs.
- `crates/family-identity-core` owns an explicit caller-path SQLite repository for issued and consumed parent-presence challenges. Existing stores are validated before initialization across an exact object allowlist plus columns, nullability, primary keys, strict/rowid shape, unique indexes, the named nonce-integrity index, private receipt sequence, and receipt foreign-key target/delete behavior. Triggers, views, extra tables, and virtual-table shadow objects are rejected without byte changes. Challenge refs and nonce refs are durable unique identities; public receipt refs are 256-bit OS-random opaque capabilities while the database sequence stays private.
- Focused real-process tests prove concurrent consume contention, durable replay rejection after restart, and concurrent different-challenge issuance against one nonce yielding exactly one issue plus one duplicate-nonce rejection.
- First creation is initialized privately and published atomically without overwrite; concurrency, stale unpublished artifacts, and restart are exercised by visible tests.
- Production custody fails unavailable before path creation on every platform until a trusted custody provider can exclude same-user challenge-store writers. Windows final-file/ancestor handle checks and owner-private creation tests run only through an explicit debug-only seam and are not production custody proof.
- The lifecycle authority sidecar now serializes process writers through a sibling lock, reloads the current map before each generation update, and persists with an atomic replacement plus file/parent synchronization. Corrupt, missing-after-database, lock, and persistence failures remain unavailable rather than being treated as trusted.
- `device_trust_ref` generation is opaque, CSPRNG-backed, and input-independent. Sealing remains manual-required because no specifically authorized high-risk device-trust sealing action exists; low-risk actions are never promoted.
- Parent-presence decisions are transactionally enqueued beside custody state and delivered fail-closed into an `ocentra-eventing` hash-chained NDJSON journal. Visible tests cover accepted and replay outcomes, correlation and redaction, real delivery failure, restart recovery, and idempotent re-delivery. This workpack does not claim subscriber delivery, a broader event-bus runtime, or broader device-trust lifecycle completion.
- The attempted private runtime-fence participant integrated through
  `f5974c795` was rejected and withdrawn. Its same-user-writable SQLite rows and
  recomputable unkeyed digest allowed fabricated committed outcomes; it also
  lacked an explicit existing-database migration and bounded retention.
- This remains a partial, unchecked WP01 foundation/source. It does not prove the broader device-trust lifecycle, platform key sealing, backup/export/restore, passkey ceremony, phone approval, recovery, entitlement binding, revocation integration, Unix production custody, or platform-wide path guarantees. It is not a shipped authority or production-caller route; its required tests, validation, authority bridge, and proof remain open.

## LAN WP26 dependency routing

WP01 is a foundation/source route, not a READY authority or production-caller
route. The family-identity trust owner exposes the durable current registration,
revocation, and authority-generation state that downstream owners must consume;
WP26 must not create that state or call a LAN-local substitute. This routing
note is not a WP01 completion claim: the partial lifecycle remains open until a
shipped authority issuer, production composition, expected tests, validation,
and proof exist. WP03 owns the one-time `RegisterLanSignerAnchor` ceremony and
must not be made to depend on a LAN consumer in return.

## Multi-owner effect-fence handoff

WP01 owns the future Device Trust participant for Account WP05A. No accepted
participant source currently exists. A replacement must prepare, commit, abort,
and recover an action-bound reservation against current trusted-device/signer
binding, generation, and revocation state while using protected Device-owned
receipt custody that excludes same-user writers. Account may not copy Device
Trust currentness or replay truth.

The planned source root remains
`crates/family-identity-core/src/device_trust_runtime_fence_participant.rs` with
private adjacent helpers. The expected test root
`crates/family-identity-core/tests/unit/device_trust_runtime_fence_participant.rs`
is absent. Protected receipt/key custody, startup/schema-migration ownership,
bounded retention/archive policy, a production Account WP05A caller, tests,
proof, runtime reachability, READY, and DONE are all missing.

## Downstream bridge order

The current legal order is explicit and non-circular:

1. Account Identity WP08 defines the canonical household/child/device/pairing
   binding; Cloudflare WP06 persists and resolves it from a provider-gated
   production caller. Neither packet may redefine Device Trust authority.
2. Device Trust WP03 consumes WP01 plus those Account/Cloudflare handoffs to
   own parent step-up, target resolution, signer registration authorization,
   signature verification, durable sign-count ownership, and one-time nonce
   consumption. Typed receipts and request identifiers are not ceremony
   authority.
3. The selected WP02 parent-runtime/platform sealing route composes sealing,
   lifecycle, and revocation with the current binding. It remains
   manual-required without a real ceremony issuer and platform owner.
4. LAN WP26 and child/runtime consumers run only after WP03 and consume the
   current binding/revocation state; they do not mint, register, or infer
   signer authority from pairing or transport evidence.

## Accepted source consolidation — 2026-08-17

The accepted Device Trust continuation is retained at source head `914d06b6a`
and integrated through `68717b5b7`. In addition to the earlier repository
packet, the owner now re-resolves the current device/signer binding and no
public household signer/verifier mint path remains. Current reviewed owner
paths include:

- `crates/family-identity-core/src/device_trust_signer_registration.rs`
- `crates/family-identity-core/src/device_trust_signer_registration_schema.rs`
- `crates/family-identity-core/src/device_trust_signer_registration_current_authority.rs`
- `crates/family-identity-core/src/device_trust_current_binding.rs`
- `crates/family-identity-core/src/device_trust_lifecycle.rs`
- `crates/family-identity-core/src/device_trust_lifecycle_revocation.rs`
- `crates/family-identity-core/src/household_authority_proof.rs`
- `crates/family-identity-core/src/lib.rs`

An independent source review found no P0/P1 findings and accepts these paths as
reviewed implementation evidence only. Existing databases are validated
without silent schema repair, first-open creation remains concurrency-tolerant,
SQLite uses a busy timeout, and raw enrollment/revoke/reset mutation entrypoints
are crate-private. Focused source-format, architecture, Enforcer, diff, and
guard checks passed. Expected-test migration, functional validation, proof,
production platform/passkey callers, repo-wide Enforcer/architecture acceptance, platform
custody, broader lifecycle composition, and DONE remain open.

## Runtime-fence source withdrawal — 2026-08-24

A later independent review rejected the integrated 18-path participant packet.
Private Rust types did not make its persisted authority owner-authenticated:
same-user SQLite writers could recompute the plain digest and fabricate a
committed row. Existing databases had no explicit migration for the new table,
and the unbounded ledger was scanned in full on every repository open. The
participant/helper files and lifecycle schema hook are therefore removed.

Protected Capability Custody WP01 (or another dependency-owned protected
Device receipt provider), versioned migration, bounded retention/archive
semantics, the Account WP05A caller, expected participant and migrated lifecycle
tests, functional execution, proof, precommit, CI, and PR remain open.

The quarantined remote residual `origin/codex/device-wp01-fence-participant-aug23`
at `5f0280f711` is rejected/superseded evidence: its tip reintroduces the
withdrawn participant even though its commits are patch-equivalent to canonical
history. It is not an accepted implementation or test root.

## Negative cases

- Copied binaries do not create trust.
- Login success does not imply device trust.
- Revoked trust cannot be revived by stale local state.
- A nonce cannot be issued under two challenge refs, including across processes or restart.
- A consumed challenge cannot be replayed after process restart.
- Corrupt or substituted custody paths fail closed without deleting or recreating the caller's data.
- Unsupported production custody fails unavailable before accepting challenge custody.
- Missing high-risk sealing authority produces manual-required and no device trust reference.
- Prepared participant state after restart is uncertain and cannot be promoted
  into a reconstructed commit handle.
- A stale target, signer, lifecycle generation, authority digest, or corrupted
  committed row cannot recover or commit an effect.
