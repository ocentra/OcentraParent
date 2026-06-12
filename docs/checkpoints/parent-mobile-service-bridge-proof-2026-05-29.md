<!-- agent-capsule -->

> Agent Capsule
> Doc: Parent Mobile Service Bridge Proof - 2026-05-29
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Parent Mobile Service Bridge Proof - 2026-05-29

## Scope

This checkpoint records the D-owned parent mobile service bridge proof slice.

The slice adds a typed parent-domain contract for the current parent mobile bridge boundary:

- local service connection state
- LAN service connection state
- cloud relay non-implementation
- mobile package bridge state
- observer read-only behavior
- controller takeover manual-required authority
- degraded or unavailable LAN AI provider submission
- package and service launch gaps
- disabled phone-local model execution

It does not implement C UI rendering, vendor visuals, Rust mobile runtime wiring, physical household LAN proof, cloud relay, Android child-agent behavior, iOS Family Controls, signing, stores, or phone-local model execution.

## Proof Command

```powershell
cmd /c npm run test:parent-mobile-service-bridge
```

The command writes:

```text
test-results/parent-mobile-service-bridge-proof/proof.json
```

## Source Proofs

The proof harness composes these existing non-browser proof artifacts:

- `test-results/parent-mobile-shell-runtime-proof/proof.json`
- `test-results/v0-9-production-lan-mobile-controller-proof/proof.json`
- `test-results/v0-9-mobile-controller-observer-runtime-proof/proof.json`

## Contract Files

- `packages/parent-domain/src/parent-mobile-service-bridge-runtime.ts`
- `packages/parent-domain/tests/parent-mobile-service-bridge-runtime.test.ts`
- `scripts/test/parent-mobile-service-bridge-proof.mjs`

## Known Integration Gap

`docs/expectations/pre-ai-proof-matrix.json` was not edited in this slice because another active worker lane currently owns that shared matrix. The proof output records matrix registration as deferred; integration can register the new proof command after the active matrix owner clears or merges.

## Claims Preserved

- Parent mobile remains observer or controller-candidate only until real Android or iOS package/device authority proof exists.
- Observer mobile surfaces reject policy writes and approval decisions.
- Controller takeover stays manual-required.
- LAN AI submission stays degraded or unavailable and uses provider state, not a phone-local model.
- Cloud relay remains not implemented.
- Package/service launch gaps stay explicit for mobile service behavior, signing, stores, notification permission, foreground/background execution, and controller authority.
