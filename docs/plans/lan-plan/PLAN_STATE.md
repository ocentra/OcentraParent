# LAN Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan State`
> Kind: current executable status and open gaps.
> Read when: first, before opening workpacks or proof paths.
> Stop rule: do not widen into frozen follow-on workpacks from here.
> Proves: current plan model, current slice status, and next execution route only.
> Does not prove: final completion of open workpacks, physical household proof, or sibling plan completion.
> Proof rule: any status claim here must point at an existing artifact or an explicit open/manual-required gap.

<!-- /agent-capsule -->

## Current State

- Plan state: active
- Authoritative execution model: `01-20`
- Frozen follow-on only: `21-25`
- Current completed reconciliation slice: `Slice A`
- Slice A evidence root: `output/lan-plan-proof/00-plan-model-reconciliation/`

## Slice A Status

`Slice A` is green as of 2026-06-17 for the scope that was actually assigned:

- LAN package export/ownership repair in `packages/lan-domain`
- focused proof-schema test repair
- full `@ocentra-parent/lan-domain` test pass
- full `@ocentra-parent/lan-domain` build pass
- full `packages/lan-domain` architecture pass
- plan truth-sync for the authoritative `01-20` model
- honest proof-root bootstrap for this reconciliation slice

Exact evidence:

- `output/lan-plan-proof/00-plan-model-reconciliation/00-source-snapshot.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/00-plan-model-reconciliation/02-plan-truth-sync.md`
- `output/lan-plan-proof/00-plan-model-reconciliation/03-missing-proof-inventory.md`

## Executable Truth

- `packages/lan-domain` is the current TypeScript source owner for executable `lan-plan` work.
- `packages/parent-domain/src/lan-*` is not the authoritative owner for current completion claims.
- The current LAN source-matrix/read-model model covers workpacks `01-20`, not `21-25`.
- Portal LAN proof still depends on source/service-backed truth; portal does not own the LAN truth model.

## Open Execution Buckets

- Locally executable next: `01-16`, `19`, `20`
- Open implementation but still local-slice work: `05`, `06`, `07`, `08`, `09`, `11`, `17`
- Mixed local plus physical/manual final gates: `15`, `16`, `18`, `19`, `20`
- Frozen and out of current model: `21-25`

## Remaining Gaps For Real Completion

- real second-device household proof
- router/firewall reachability proof
- real signed child hello/heartbeat artifacts
- replay/restart/event-stream proof completion
- regenerated LAN source-matrix and portal proof artifacts
- Android/mobile-controller proof where the plan still keeps those claims

Household/setup/account first-run UX is not part of the current authoritative model. That work remains frozen in `21-25`.

## Next Slice

Next approved slice after this file is `B1`: local LAN proof regeneration for the authoritative `01-20` model only.
