# 22 Current State And Gap Map

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `22 Current State And Gap Map`
> Kind: assigned active workpack; read only when this exact workpack is selected.
> Read when: Only when this exact workpack is explicitly selected from `WORKPACK_INDEX.md`.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack's own proof rows and tests support the claim.
> Proves: only this workpack's current gap-map boundary and progress explicitly recorded here.
> Does not prove: current completion of sibling workpacks or broad LAN readiness.
> Proof rule: Rewrite any stale TS-first file targets before using this file for execution claims.

<!-- /agent-capsule -->

Sources: [folder README](../README.md), [feature doc](../../../features/family-setup-device-roles.md),
[family setup expectations](../../../expectations/family-setup.md),
[LAN pairing expectations](../../../expectations/lan-pairing.md).

## Active scope status

This workpack is part of the authoritative `01-25` LAN execution model. It is
locally complete for the current gap-map truth-sync slice.

Historical TS-domain file targets from older copies of this draft are stale.
Current direction for this workpack is:

- Rust owns the shared route/source enums, read-model contracts, and runtime
  behavior.
- TS remains presentation only and may consume generated bridge artifacts at
  the UI edge.
- Any path list that still points to TS contract ownership must be rewritten
  before implementation starts.

## Where We Are

The current LAN spine already exposes a service-backed add-device read model
with:

- trusted-registry and route-custody device rows
- signed discovery / relay / stale / offline labels
- LAN source-matrix rows for workpacks and discovery sources
- parent decision fields such as assign, rename, ignore, restore, trust, and
  revoke

This locally complete workpack captures the current family-setup-oriented gap
map around household grouping, controller leases, observer permissions,
recovery UX, and route/source label closure.

## Where We Want To Be

Map every remaining gap to a Rust-owned boundary and classify it as:

- implement now
- deferred to later workpack
- manual-required

A future worker should be able to read this file and know which Rust boundary
owns the next move without being sent back to TS contract ownership or a portal
execution path.

## Gap inventory

The current verified gaps for this workpack are now:

1. physical two-device household LAN proof for route/revoke/re-pair behavior
2. router/firewall/manual network proof beyond local single-host validation
3. broader passive DHCP, WS-Discovery, and active WSD live proof
4. broader service-probe, weighted-classification, and install-eligibility
   proof beyond the current bounded Rust slices
5. replay/restart/event-stream proof and broader downstream consumer proof

## Owning Rust boundaries

The likely Rust-first owners are:

- the Rust persistence boundary for household, lease, and permission state
- the Rust read-model boundary for household and LAN device presentation state
- the Rust shared schema or bridge/schema boundary for route/source enums
- supporting presentation checks only after the Rust-owned read-model proof is
  already honest

Do not revive any older `packages/parent-domain` contract-first path from this
workpack.

## Tests And Proof

- Current WP22 closure is a truth-sync/gap-map slice, not a new DB or runtime
  implementation slice.
- Supporting Rust validation is the current LAN row proof set: `21` contract
  proof, `23` route/rejection proof, `24` presentation proof, and the focused
  LAN runtime checks recorded in `25`.
- Deferred DB/migration/lease runtime implementation remains outside WP22 until
  a future selected workpack names that exact owner and proof path.
- Test proof must live in real organized Rust crate test folders or explicit UI
  presentation checks that stay supplementary. Inline source-owned tests,
  placeholder directories, `.gitkeep` trees, fake coverage, or mock-only
  readiness do not count.
- Proof artifact: `output/lan-plan-proof/22-current-state-and-gap-map/01-local-validation.md`

## AI Worker Checklist

- [x] Confirm WP22 is the assigned active workpack.
- [x] Rewrite any stale TS-first ownership or file-target language before code
      moves.
- [x] Confirm WP01 Rust-owned contracts exist before runtime implementation.
- [x] Existing db/read-model layout inspected; no duplicate household truth
      created alongside existing LAN slot tables.
- [x] SQLite migration work remains deferred because this WP22 pass did not add
      a DB/migration implementation surface.
- [x] Read-model/runtime truth is covered by the selected Rust proof rows
      referenced in the proof artifact.
- [x] Route/source enum ownership remains Rust-owned before UI projection.
- [x] Deferred items and manual-required items remain recorded honestly.

## Manual-Required Gaps

Physical two-device LAN proof and broader router/firewall topology proof cannot
be automated in CI here and remain separate from this gap map unless a later
proof packet proves them.
