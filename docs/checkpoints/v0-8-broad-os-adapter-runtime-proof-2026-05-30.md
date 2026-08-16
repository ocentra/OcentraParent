<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 Broad OS Adapter Runtime Proof Checkpoint
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# V0.8 Broad OS Adapter Runtime Proof Checkpoint

- Branch: `codex/v0-8-broad-os-adapter-runtime-proof`
- Worker: `codex-b`
- Date: 2026-05-30
- Proof command: `node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`
- Proof artifact: `test-results/v0-8-broad-os-adapter-runtime-proof/proof.json`

## Scope

This checkpoint adds the final non-visual V0.8 broad app/domain/browser OS-adapter runtime proof pass. It composes the existing broad OS adapter proof, browser/domain adapter proof, OS adapter manual artifact gates, and product proof read models into one parent-domain read model.

Implemented-boundary entries are limited to Windows owned-process pid/name plus app timer runtime behavior and Windows managed-browser session intervention. Those boundaries remain explicitly separate from global app blocking, network/domain blocking, managed exact URL enforcement, and unmanaged browser exact evidence.

## Manual, Unavailable, And Not-Claimed States

- Windows broad installed-app runtime remains `manual-required`.
- Windows network/domain runtime remains `manual-required`.
- Windows managed-browser exact URL runtime remains `manual-required`.
- Windows unmanaged browser exact evidence remains `not-claimed`.
- Linux host runtime remains `unavailable`.
- macOS host runtime remains `manual-required`.
- Android privileged mobile runtime remains `manual-required`.
- iOS privileged mobile runtime remains `manual-required`.

## Non-Claims

This checkpoint does not prove global installed-app blocking, host network/domain blocking, managed active-tab exact URL enforcement, unmanaged browser URL/title/page/download/HTTPS/intent evidence, Linux or macOS host enforcement, Android device-owner or managed-profile support, Android VPN/DNS or package lifecycle support, iOS Family Controls or DeviceActivity support, signing, TestFlight, stores, or physical device behavior.

## Validation

Expected focused validation:

- `cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-8-broad-os-adapter-runtime-proof`
- `cmd /c node --check scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`
- `cmd /c node scripts/test/v0-8-broad-os-adapter-runtime-proof.mjs`
- `cmd /c npm run test:pre-ai-proof`
- `cmd /c npm run lint:schema-boundaries`
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cmd /c npm run lanes:guard -- --owner codex`
- `cmd /c npm run hub:guard`
- `cmd /c npm run validate`
