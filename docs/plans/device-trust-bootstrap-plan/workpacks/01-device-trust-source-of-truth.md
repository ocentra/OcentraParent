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

## Current audit state

- A runtime-security proof now exists at `output/device-trust-bootstrap-plan-proof/01-device-trust-source-of-truth/00-runtime-security-proof.md` for the narrow Rust parent-presence custody slice.
- `crates/family-identity-core` now owns an explicit caller-path SQLite repository for issued and consumed parent-presence challenges. Existing stores are validated before initialization across exact columns, nullability, primary keys, strict/rowid shape, unique indexes, the named nonce-integrity index, private receipt sequence, and receipt foreign-key target/delete behavior. Challenge refs and nonce refs are durable unique identities; consumption uses `BEGIN IMMEDIATE`; public receipt refs are stored 256-bit OS-random opaque capabilities while the database sequence stays private.
- Focused real-process tests prove concurrent consume contention, durable replay rejection after restart, and concurrent different-challenge issuance against one nonce yielding exactly one issue plus one duplicate-nonce rejection.
- Custody tests reject corrupt databases without recreation, malformed schemas without silent repair or byte changes, relative or missing-parent paths, read-only files, and final/ancestor symbolic substitution. On Windows both links must be created before rejection counts as proof; link-creation denial fails the test and remains an external platform constraint.
- This is still a partial WP01 result. It does not prove the broader device-trust lifecycle, platform key sealing, backup/export/restore, passkey ceremony, phone approval, recovery, entitlement binding, revocation integration, or platform-wide path guarantees.

## Negative cases

- Copied binaries do not create trust.
- Login success does not imply device trust.
- Revoked trust cannot be revived by stale local state.
- A nonce cannot be issued under two challenge refs, including across processes or restart.
- A consumed challenge cannot be replayed after process restart.
- Corrupt or substituted custody paths fail closed without deleting or recreating the caller's data.
