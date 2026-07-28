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
- `device_trust_ref` generation is opaque, CSPRNG-backed, and input-independent. Sealing remains manual-required because no specifically authorized high-risk device-trust sealing action exists; low-risk actions are never promoted.
- Parent-presence decisions are transactionally enqueued beside custody state and delivered fail-closed into an `ocentra-eventing` hash-chained NDJSON journal. Visible tests cover accepted and replay outcomes, correlation and redaction, real delivery failure, restart recovery, and idempotent re-delivery. This workpack does not claim subscriber delivery, a broader event-bus runtime, or broader device-trust lifecycle completion.
- A durable SQLite `DeviceTrustRegistry` now accepts only a consumed, action-bound parent-presence verification paired with an opaque household authorization grant. It records pair requests as `pending-sealing`, preserves the parent/account/family/device binding, journals the decision in the same transaction, rejects cross-family ownership conflicts, and does not expose a transition to `trusted` without the later platform-key sealing receipt boundary.
- The public household authority entrypoint intentionally remains fail-closed because this crate has no authenticated household/member/device state adapter. That means the registry is a tested storage and authorization boundary, not a connected runtime trust lifecycle or a device-trust completion claim. Its registry mutation scenarios currently run as crate-private invariant tests; the visible integration test proves the public no-adapter and unbound-target rejection boundary.
- This is still a partial, unchecked WP01 result. It does not prove the broader device-trust lifecycle, platform key sealing, backup/export/restore, passkey ceremony, phone approval, recovery, entitlement binding, revocation integration, Unix production custody, or platform-wide path guarantees.

## Negative cases

- Copied binaries do not create trust.
- Login success does not imply device trust.
- Revoked trust cannot be revived by stale local state.
- A nonce cannot be issued under two challenge refs, including across processes or restart.
- A consumed challenge cannot be replayed after process restart.
- Corrupt or substituted custody paths fail closed without deleting or recreating the caller's data.
- Unsupported production custody fails unavailable before accepting challenge custody.
- Missing high-risk sealing authority produces manual-required and no device trust reference.
