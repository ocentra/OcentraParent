<!-- agent-capsule -->

> Agent Capsule
> Doc: Parent Mobile Controller Observer Handoff Proof - 2026-05-30
> Kind: historical checkpoint/proof documentation; read only when CHECKPOINT_INDEX or PROOF_INDEX names it.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Parent Mobile Controller Observer Handoff Proof - 2026-05-30

## Scope

This checkpoint records the non-visual parent mobile controller/observer
handoff proof for the D lane.

Covered:

- observer controller-lease visibility without parent mobile write authority
- controller takeover request, denial, degradation, and release states
- selected route and provider handoff truth
- degraded or unavailable LAN AI provider handoff from parent mobile
- disabled phone-local model execution
- explicit non-claims for cloud relay, mobile parity, child mobile agent
  behavior, Android device-owner behavior, iOS Family Controls, signing,
  stores, and entitlements

Not covered:

- C-owned UI rendering, layout, scratch, or vendor visual paths
- real Android or iOS active-controller authority
- remote-control or cloud-relay runtime behavior
- physical household LAN/provider readiness on two devices
- phone-local model execution
- child mobile agent behavior, Android device-owner mode, or iOS Family
  Controls entitlement proof

## Proof Command

```powershell
cmd /c npm run test:parent-mobile-controller-observer-handoff
```

Expected output proof:

```text
test-results/parent-mobile-controller-observer-handoff-proof/proof.json
```

## Source Proofs

- `test-results/parent-mobile-service-bridge-proof/proof.json`
- `test-results/v0-9-production-lan-mobile-controller-proof/proof.json`
- `test-results/v0-9-mobile-controller-discovery-runtime-proof/proof.json`
- `test-results/v0-9-prod-discovery-provider-selection-proof/proof.json`

## Touched Files

- `packages/parent-domain/src/parent-mobile-controller-observer-handoff-runtime.ts`
- `packages/parent-domain/tests/parent-mobile-controller-observer-handoff-runtime.test.ts`
- `scripts/test/parent-mobile-controller-observer-handoff-proof.mjs`
- `package.json`
- `packages/parent-domain/package.json`
- `docs/checkpoints/parent-mobile-controller-observer-handoff-proof-2026-05-30.md`

## Runtime Truth

- Parent mobile may observe controller lease and selected route state.
- Parent mobile controller takeover remains denied or manual-required until
  real package, device, and platform authority proof exists.
- LAN AI handoff remains degraded or unavailable when provider evidence is not
  product-ready.
- Cloud relay is not implemented and is not a fallback for LAN/mobile handoff.
- Parent mobile does not run a phone-local model by default.

## Validation

Run before PR-ready handoff:

```powershell
git diff --check
git diff --check origin/main...HEAD
cmd /c node --check scripts/test/parent-mobile-controller-observer-handoff-proof.mjs
cmd /c npm run build --workspace @ocentra-parent/parent-domain
cmd /c npm run test --workspace @ocentra-parent/parent-domain -- tests/parent-mobile-controller-observer-handoff-runtime.test.ts
cmd /c npm run lint:exec --workspace @ocentra-parent/parent-domain
cmd /c npm run build:contracts
cmd /c npm run lint:schema-boundaries
cmd /c npm run format:check
cmd /c npm run test:parent-mobile-controller-observer-handoff
cmd /c npm run lanes:guard -- --owner codex
cmd /c npm run hub:guard
```
