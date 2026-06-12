<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.8 OS Adapter Manual Artifact Gates
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.8 OS Adapter Manual Artifact Gates

Date: 2026-05-30
Branch: `codex/v0-8-os-adapter-manual-artifact-gates`

## Scope

This checkpoint records the non-visual V0.8 OS-adapter manual artifact gates proof. It adds a parent-domain read model and focused harness that keeps privileged app, network, browser, desktop OS, Android, and iOS support honest until target-specific artifacts exist.

## Proved Boundaries

- Windows broad installed-app blocking, process/package identity, owned-process terminate, parent cancel/override, network/domain filter apply plus rollback, managed-browser exact URL, restart recovery, audit custody, service permission, and package lifecycle are manual-required artifact gates.
- Unmanaged browser exact URL, title, page, download, HTTPS content, and intent evidence remain not-claimed without explicit browser integration.
- Linux adapter gates are unavailable in this proof; macOS gates remain manual-required.
- Android UsageStats, accessibility, VPN/DNS, device-owner, managed-profile, and package lifecycle gates remain mobile-artifact-required.
- iOS Family Controls, DeviceActivity, Screen Time, Network Extension, background execution, signing, and TestFlight gates remain mobile-artifact-required.

## Non-Claims

- No product-ready broad app blocking.
- No host network/domain blocking.
- No managed browser exact active-tab URL enforcement.
- No unmanaged browser exact URL/title/page/download evidence.
- No Linux or macOS host adapter support upgrade.
- No Android privileged device-owner, managed-profile, VPN/DNS, accessibility, UsageStats, or package lifecycle support upgrade.
- No iOS Family Controls, DeviceActivity, Screen Time, Network Extension, background execution, signing, or TestFlight support upgrade.

## Validation

Expected focused validation:

```powershell
cmd /c npm run build:contracts
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-8-os-adapter-manual-artifact-gates
cmd /c node scripts/test/v0-8-os-adapter-manual-artifact-gates.mjs
cmd /c npm run test:pre-ai-proof
cmd /c npm run lint:schema-boundaries
cmd /c npm run lanes:guard -- --owner codex
cmd /c npm run hub:guard
git diff --check origin/main...HEAD
```
