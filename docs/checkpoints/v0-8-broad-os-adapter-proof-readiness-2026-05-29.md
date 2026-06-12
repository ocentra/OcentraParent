<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Broad OS Adapter Proof Readiness Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 Broad OS Adapter Proof Readiness Checkpoint

Date: 2026-05-29
Worker: codex-b
Branch: codex/v0-8-broad-os-adapter-proof-readiness

## Scope

This checkpoint adds contract-first readiness proof for broad OS adapter claims. It does not make broad app blocking, network/domain blocking, managed-browser exact URL control, unmanaged browser exact evidence, admin hardening, anti-tamper, rollback, Android child enforcement, or iOS child enforcement product-ready.

## Proof Boundary

- Implemented: owned-process pid/name termination, app time-limit lifecycle, and unmanaged-browser process-only boundary where host support exists.
- Manual required: broad app blocking, network/domain blocking, managed-browser service-command enforcement, managed-browser exact URL control, admin hardening, anti-tamper, and rollback.
- Not claimed: unmanaged browser exact URL, active tab, title, download source, page text, HTTPS content, or intent evidence.
- Unavailable: unsupported hosts must keep explicit unavailable states instead of silently upgrading claims.

## Local Proof Command

```powershell
node scripts/test/v0-8-broad-os-adapter-proof-readiness.mjs
```

The proof command builds contracts, runs focused parent-domain and Rust readiness/service tests, validates the proof matrix entry, and writes:

```text
test-results/v0-8-broad-os-adapter-proof-readiness/proof.json
```

## Required Claim Upgrade Artifacts

- OS-approved app/package identity, installed-app block, rollback, and bypass-resistance proof.
- Host network filter proof with domain block apply and rollback evidence.
- Managed browser active-tab and exact URL enforcement artifacts.
- Admin hardening and anti-tamper proof.
- Android device-owner/package lifecycle and iOS Family Controls entitlement/device artifacts before mobile child claims change.
