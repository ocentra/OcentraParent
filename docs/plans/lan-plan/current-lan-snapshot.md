# Current LAN Snapshot - 2026-06-17

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `Current LAN Snapshot - 2026-06-17`
> Kind: current executable truth snapshot.
> Read when: status or claim routing is needed before opening a workpack.
> Stop rule: this snapshot does not authorize frozen follow-on work.
> Proves: current LAN model, current Slice A evidence, and current open gaps only.
> Does not prove: regenerated B1 proof, physical household readiness, or sibling plan completion.
> Proof rule: every supportable claim below points either to current source or to a current proof artifact from `Slice A`.

<!-- /agent-capsule -->

## Authoritative Model

- Authoritative workpacks: `01-20`
- Frozen follow-on only: `21-25`
- Current evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`

## Current Source Truth

- `packages/lan-domain` is the current TypeScript source owner for executable LAN work.
- `packages/parent-domain/src/lan-*` is legacy compatibility or stale reference surface only.
- `packages/agent-protocol-domain` carries typed LAN protocol/browser-runtime adapters.
- `crates/agent-service` carries the service-backed source matrix and household device spine/runtime state.
- Portal LAN proof surfaces remain consumers of service-backed truth, not a parallel LAN truth source.

## Slice A Evidence That Exists Now

- `packages/lan-domain/src/lan-pairing.ts` repaired to an explicit export surface
- `packages/lan-domain/src/v0-9-production-discovery-household-proof.ts` repaired to use direct local imports
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`

Validated commands:

- `npx vitest run tests/unit/v0-9-production-discovery-household-proof.test.ts`
- `npm run test --workspace @ocentra-parent/lan-domain`
- `npm run build --workspace @ocentra-parent/lan-domain`
- `npm run lint:architecture -- --files packages/lan-domain`

## Claims Supportable Now

- the LAN package export/contract surface is green for the current scoped package
- the current executable source-matrix/read-model model is still `01-20`
- weak/manual network evidence remains fenced from child-agent identity claims
- signed hello/heartbeat is not overclaimed as implemented proof
- stale `parent-domain` ownership and stale proof references are no longer valid current-plan truth

## Claims Not Supportable Now

- regenerated source-matrix proof artifacts
- regenerated portal screenshot proof
- physical two-device household LAN proof
- router/firewall reachability proof
- real signed child hello/heartbeat artifacts
- full replayable LAN event proof
- first-run household/setup/account UI completion

## Immediate Next Slice

`B1` can start after this slice. `B1` is limited to local LAN proof regeneration for the authoritative `01-20` model and should not widen into frozen `21-25`.
