# Current LAN Snapshot - 2026-06-17

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `Current LAN Snapshot - 2026-06-17`
> Kind: current executable truth snapshot.
> Read when: status or claim routing is needed before opening a workpack.
> Stop rule: this snapshot does not authorize frozen follow-on work.
> Proves: current LAN model, current Slice A/B1/B2 evidence, and current open gaps only.
> Does not prove: physical household readiness or sibling plan completion.
> Proof rule: every supportable claim below points either to current source or to a current proof artifact from the active LAN proof roots.

<!-- /agent-capsule -->

## Authoritative Model

- Authoritative workpacks: `01-20`
- Frozen follow-on only: `21-25`
- Current evidence roots:
  - `output/lan-plan-proof/00-plan-model-reconciliation/`
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/`
  - `output/lan-plan-proof/02-lan-b2-test-truth-repair/`

## Current Source Truth

- `packages/lan-domain` is the current TypeScript source owner for executable LAN work.
- `packages/parent-domain/src/lan-*` is legacy compatibility or stale reference surface only.
- `packages/agent-protocol-domain` carries typed LAN protocol/browser-runtime adapters.
- `crates/agent-service` carries the service-backed source matrix and household device spine/runtime state.
- Portal LAN proof surfaces remain consumers of service-backed truth, not a parallel LAN truth source.

## Evidence That Exists Now

- `packages/lan-domain/src/lan-pairing.ts` repaired to an explicit export surface
- `packages/lan-domain/src/v0-9-production-discovery-household-proof.ts` repaired to use direct local imports
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`
- `packages/lan-domain/tests/README.md`
- `output/lan-plan-proof/02-lan-b2-test-truth-repair/00-b2-test-truth-note.md`

Validated commands:

- `npx vitest run tests/unit/v0-9-production-discovery-household-proof.test.ts`
- `npm run test --workspace @ocentra-parent/lan-domain`
- `npm run build --workspace @ocentra-parent/lan-domain`
- `npm run lint:architecture -- --files packages/lan-domain`
- `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`
- `node scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`
- `node scripts/test/v0-9-production-discovery-household-proof.mjs`
- `node scripts/test/v0-9-household-lan-proof-readiness.mjs`
- `packages/lan-domain :: cmd /c npx vitest run tests/unit`

## Claims Supportable Now

- the LAN package export/contract surface is green for the current scoped package
- the current executable source-matrix/read-model model is still `01-20`
- regenerated source-matrix and signed-discovery relay proof artifacts exist for the local `B1` slice
- production discovery states and readiness non-claim are machine-checked for the current local proof slice
- only `packages/lan-domain/tests/unit` currently contains real LAN test files
- placeholder LAN test folders do not count as integration, contract, e2e, property, security, or other higher-category coverage
- weak/manual network evidence remains fenced from child-agent identity claims
- signed hello/heartbeat is not overclaimed as implemented proof
- stale `parent-domain` ownership and stale proof references are no longer valid current-plan truth

## Claims Not Supportable Now

- regenerated portal screenshot proof
- physical two-device household LAN proof
- router/firewall reachability proof
- real signed child hello/heartbeat artifacts
- full replayable LAN event proof
- first-run household/setup/account UI completion

## Immediate Next Slice

After `B2`, the next slice is `lan-c1-protocol-service-truth-repair` unless a
new contradiction is found inside `packages/lan-domain/tests/**`.
