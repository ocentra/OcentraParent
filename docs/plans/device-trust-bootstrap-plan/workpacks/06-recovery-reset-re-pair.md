# Workpack 06: Recovery Reset Re-Pair

Purpose: define encrypted recovery bundles, reset, revoke, and re-pair flows.

## Owns

- Encrypted recovery bundle shape.
- Household-bound restore semantics.
- Parent-authorized reset and re-pair.
- Revocation preservation during restore.

## Exit condition

- Recovery is not the same as account login.
- Wrong-household and wrong-key restores fail.
- Re-pair after reset is explicit and audited.

## Proof target

- `output/device-trust-bootstrap-plan-proof/06-recovery-reset-re-pair/`

## Current audit state

- No proof root currently exists on disk for this workpack.
- Recovery authorization and handoff rules exist in `packages/family-domain`, but encrypted recovery bundle handling and re-pair runtime proof are still missing.
- The storage restore boundary blocks the legacy confirmation-only entry point.
  The current crate has no owner-bound durable cursor token, so its dead apply
  seam is unconditionally blocked; a caller-held `ImportBundleContext` cannot
  authorize side effects. Bundle encryption, key custody, revocation
  preservation, re-pair ownership, and runtime proof remain open.

## Accepted source checkpoint — 2026-08-17

The accepted continuation integrated through `68717b5b7` removes the
caller-minted restore-authorization path and keeps restore state behind the
verified-parent and unavailable-by-default executor boundaries. This closes a
source-authority defect only. Encrypted bundle/key custody, a real executor and
composition caller, expected tests, focused execution, and proof remain open.

## Source repair candidate — 2026-08-18 (tests open)

The WP06 source wave is pushed at `4ad484197` on
`codex/device-trust-wp06-source-wave`, based on `31e4a7c55`; it is a candidate
source packet, not a completion or acceptance claim:

- The public `repair_with_new_installation` generation-only seam was removed.
  Lifecycle revoke/reset remains durable owner code, but no parent-authorized
  re-pair transition producer or startup caller exists.
- The recovery bundle builder now returns `EncryptionCustodyUnavailable` and
  cannot mark caller-provided payload references as encrypted. No platform
  key/envelope custody owner is present.
- Import preflight accepts currentness only when a non-empty bundle cursor
  exactly matches the storage-owner current cursor. Missing or mismatched
  currentness is a `TombstoneConflict` with tombstones not preserved. Because
  the current context is only a caller-held snapshot, the apply seam is now
  unconditionally blocked and cannot invoke an executor; an owner-bound
  cursor token must be reread and consumed at apply time before this can open.
- `RestoreApplyRequest`, restore application, and migration readiness remain
  non-public; the parent-authority/custom-executor path was removed. No
  external caller can submit serde-shaped preflight data as restore authority,
  and the blocked result does not copy caller-provided custody facts.

The actual source has no legal producer for the import context or verified
parent authority, no encrypted key custody, durable current revocation/tombstone
owner, authorized re-pair ceremony, real restore executor, or shipped
parent/child recovery caller. Expected tests and proof remain open. The five
storage recovery files overlap Data WP05 candidate `3def622df`; rebase and
semantic reconciliation after Data acceptance are required before integration.

## Negative cases

- Corrupted bundles fail closed.
- Partial restore cannot silently create a new trust root.
- Revoked trust cannot be resurrected by recovery.
