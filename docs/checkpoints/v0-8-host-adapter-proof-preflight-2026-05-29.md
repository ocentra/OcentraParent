<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Host Adapter Proof Preflight Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 Host Adapter Proof Preflight Checkpoint

Date: 2026-05-29
Worker: codex-b
Branch: codex/v0-8-host-adapter-proof-preflight

## Scope

This checkpoint adds a typed manual proof preflight for host adapter claims. It does not implement broad app blocking, network/domain blocking, managed-browser exact URL enforcement, unmanaged browser exact evidence, admin hardening, anti-tamper, rollback, Android child enforcement, or iOS child enforcement.

## Preflight Gates

- Process/package identity: broad app blocking requires OS-approved package identity, executable/publisher evidence, installed app inventory, process lineage, block apply, rollback, and audit evidence.
- Host network filter: network/domain blocking requires a host network filter or DNS/VPN adapter proof with metadata-only custody and rollback evidence.
- Managed browser boundary: managed-browser commands and exact URL control require managed session, bridge state, active document/tab evidence, command result, and audit evidence.
- Explicit browser integration: unmanaged browser process/window/network evidence remains process-only and cannot prove exact URL, active tab, title, download source, page text, HTTPS content, or intent.
- Rollback/anti-tamper: admin hardening, service install state, tamper attempt, rollback token/result, bypass resistance, and audit custody must be recorded before product claims upgrade.

## Local Proof Command

```powershell
node scripts/test/v0-8-host-adapter-proof-preflight.mjs
```

The command builds contracts, runs the focused parent-domain preflight test, verifies proof-matrix registration, and writes:

```text
test-results/v0-8-host-adapter-proof-preflight/proof.json
```

## Required Manual Evidence Before Claim Upgrade

- Commit SHA, Windows build, service version, parent/child ids, and policy decision ids.
- Rust service logs and audit ids for each broad app/domain/browser command.
- OS-approved app/package identity and rollback artifacts.
- Network filter or DNS/VPN adapter apply/rollback artifacts with no decrypted payload capture.
- Managed browser active document and exact URL enforcement artifacts.
- Admin hardening, anti-tamper, rollback, and bypass-resistance artifacts.
