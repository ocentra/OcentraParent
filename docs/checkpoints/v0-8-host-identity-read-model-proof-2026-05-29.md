# V0.8 Host Identity Read-Model Proof

Date: 2026-05-29
Owner lane: codex-b
Branch: codex/v0-8-host-identity-read-model-proof

## Scope

This checkpoint adds Rust-facing protocol and service read-model proof for host identity evidence readiness after the process/package identity bridge.

Captured read-model rows:

- installed app inventory
- process lineage
- executable identity
- package identity
- publisher/signature evidence
- inventory/process link
- unsupported identity
- rollback readiness
- audit custody

## Product Truth

This slice is not broad app blocking, network/domain blocking, managed-browser exact URL control, unmanaged browser exact evidence, real process termination, Android child enforcement, iOS enforcement, admin hardening, anti-tamper, or rollback enforcement.

Every host identity row is typed as manual-required, unavailable, or not-claimed. `safeForBroadAppBlocking` remains false for all rows until real Windows host evidence, apply, rollback, and audit artifacts exist.

## Counts

- Read-model entries: 9
- Manual-required rows: 7
- Unavailable rows: 1
- Not-claimed rows: 1
- Safe-for-broad-app-blocking rows: 0

Evidence classes:

- inventory: 2
- process: 1
- executable: 1
- package: 2
- publisher-signature: 1
- rollback: 1
- audit: 1

## Focused Proof

Required command:

```powershell
node scripts/test/v0-8-host-identity-read-model-proof.mjs
```

The proof command runs the Rust protocol tests, Rust service read-model tests, and pre-AI proof matrix validation, then writes:

```text
test-results/v0-8-host-identity-read-model-proof/proof.json
```

The pre-AI matrix registration for this specific claim is deferred because another worker currently owns the `docs/expectations/pre-ai-proof-matrix.json` lock. This checkpoint and proof artifact are ready for matrix registration when that lock clears.

## Manual Gaps Before Claim Upgrade

- Run real Windows host inventory, process lineage, executable identity, package identity, publisher/signature, and inventory/process join proof.
- Preserve unsupported, permission-limited, unknown, unpackaged, unsigned, invalid-signature, and stale evidence as typed states.
- Run block apply and rollback proof for the same package or executable identity before broad app rollback can upgrade.
- Record real service audit custody with identity evidence refs, policy decision, adapter result or manual-required state, fallback, and audit event ids.
