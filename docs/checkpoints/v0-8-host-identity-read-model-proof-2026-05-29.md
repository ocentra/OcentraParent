<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Host Identity Read-Model Proof
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

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

The pre-AI matrix now registers this claim and checkpoint scenario as `v0-8-host-identity-read-model-proof`. The proof command validates that matrix registration through `npm run test:pre-ai-proof`.

## Manual Gaps Before Claim Upgrade

- Run real Windows host inventory, process lineage, executable identity, package identity, publisher/signature, and inventory/process join proof.
- Preserve unsupported, permission-limited, unknown, unpackaged, unsigned, invalid-signature, and stale evidence as typed states.
- Run block apply and rollback proof for the same package or executable identity before broad app rollback can upgrade.
- Record real service audit custody with identity evidence refs, policy decision, adapter result or manual-required state, fallback, and audit event ids.
