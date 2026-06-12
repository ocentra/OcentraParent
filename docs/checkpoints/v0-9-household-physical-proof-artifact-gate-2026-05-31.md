<!-- agent-capsule -->

> Agent Capsule
> Doc: V0.9 Household Physical Proof Artifact Gate
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# V0.9 Household Physical Proof Artifact Gate

Date: 2026-05-31
Branch: `codex/v0-9-household-physical-proof-artifact-gate`

## Scope

This checkpoint adds a non-visual artifact gate for the remaining physical
household LAN proof requirement. It composes the existing V0.9 household
discovery/mobile-controller proof into a typed read model that names the manual
evidence bundle required before the product can claim physical two-device
household LAN readiness.

The gate covers:

- two physical household hosts on the same LAN
- router/subnet and child-service reachability evidence
- OS firewall or local-network permission evidence
- controller origin allowlist evidence
- selected-device route recovery after restart
- controller/observer route health
- revoked, stale, and offline route rejection artifacts
- real parent mobile controller package evidence
- manual evidence custody status

## Proof Boundary

The proof remains honest about what is not proved:

- physical household LAN readiness is still `manual-required`
- real router/firewall traversal is not claimed
- real Android or iOS parent mobile write authority is not claimed
- cloud relay remains `not-implemented`
- visual selector/mobile-controller UX remains out of this worker slice

## Validation Target

Focused validation for this slice:

```powershell
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- v0-9-household-physical-proof-artifact-gate
cmd /c node --check scripts/test/v0-9-household-physical-proof-artifact-gate.mjs
cmd /c node scripts/test/v0-9-household-physical-proof-artifact-gate.mjs
```

Expected proof output:

```text
test-results/v0-9-household-physical-proof-artifact-gate/proof.json
```

The proof matrix was intentionally left untouched in this slice because another
active worker owns `docs/expectations/pre-ai-proof-matrix.json`; this artifact
gate is validated directly by its focused contract test and harness.
