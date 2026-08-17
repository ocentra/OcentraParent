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
- This remains a partial, unchecked WP01 foundation. It does not prove the broader device-trust lifecycle, platform key sealing, backup/export/restore, passkey ceremony, phone approval, recovery, entitlement binding, revocation integration, Unix production custody, or platform-wide path guarantees. The workpack is READY only for the bounded production implementation route below; its required tests, validation, and proof remain open.

## LAN WP26 dependency routing

WP01 is READY as the next legal production-code route for the missing persistent
trusted-device and signer-key registration source that LAN WP26 must consume.
The route belongs in the family-identity trust owner and must expose current
registration, revocation, and authority-generation state; WP26 must not create
that state or call a LAN-local substitute. This routing note is not a WP01
completion claim: the existing partial lifecycle remains open until a shipped
owner and its required tests/validation are present. WP03 depends on this
source before it can authorize the one-time `RegisterLanSignerAnchor` step.

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

## Negative cases

- Copied binaries do not create trust.
- Login success does not imply device trust.
- Revoked trust cannot be revived by stale local state.
- A nonce cannot be issued under two challenge refs, including across processes or restart.
- A consumed challenge cannot be replayed after process restart.
- Corrupt or substituted custody paths fail closed without deleting or recreating the caller's data.
- Unsupported production custody fails unavailable before accepting challenge custody.
- Missing high-risk sealing authority produces manual-required and no device trust reference.
