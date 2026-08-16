# Current LAN Snapshot - 2026-06-28 Truth Sync

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `Current LAN Snapshot - 2026-06-17`
> Kind: current executable truth snapshot.
> Read when: status or claim routing is needed before opening a workpack.
> Stop rule: this snapshot does not authorize broad cross-plan widening; `21-25` remain active LAN follow-on scope, with `23` and `25` currently open.
> Proves: current LAN model, current Slice A/B1/B2 evidence, and current open gaps only.
> Does not prove: physical household readiness or sibling plan completion.
> Proof rule: every supportable claim below points either to current source or to a current proof artifact from the active LAN proof roots.

<!-- /agent-capsule -->

## Authoritative Model

- Authoritative workpacks: `01-25`
- Active open follow-on workpacks: `23`, `25`
- Current evidence roots:
  - `output/lan-plan-proof/00-plan-model-reconciliation/`
  - `output/lan-plan-proof/01-lan-b1-proof-regeneration/`
  - `output/lan-plan-proof/02-lan-b2-test-truth-repair/`

## Current Source Truth

- `crates/schema`, `crates/lan-core`, and `crates/agent-protocol` are the Rust-owned contract and protocol boundary for executable LAN work.
- `crates/agent-service` and `crates/parent-runtime-core` carry the service-backed source matrix, household device spine, route snapshots, and runtime truth.
- `packages/lan-domain`, `packages/agent-protocol-domain`, and `packages/parent-domain/src/lan-*` are historical or migration-era TS surfaces only. They are not authoritative contract owners or runtime owners.
- Portal LAN proof surfaces remain consumers of Rust-backed truth, not a parallel LAN truth source.
- Organized Rust crate test groups and explicit portal presentation tests are the forward test surface. Inline source-owned tests, placeholder trees, fake tests, and mock-only closure do not count.

## Historical Generated Artifact Paths

- None of the generated `output/` or `test-results/` paths below exists in the
  current clean checkout. They are historical/expected Phase 3 destinations,
  not current retained proof. Rust-owned LAN source and organized tests remain
  the Phase 1 truth boundary.
- `output/lan-plan-proof/00-plan-model-reconciliation/01-lan-domain-validation.log`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/01-lan-source-matrix-plan-completion-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/02-lan-signed-discovery-relay-spine-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/03-production-discovery-household-proof.json`
- `output/lan-plan-proof/01-lan-b1-proof-regeneration/04-household-lan-proof-readiness.json`
- historical migration note only, not an active proof or ownership surface:
  `packages/lan-domain/tests/README.md`
- `output/lan-plan-proof/02-lan-b2-test-truth-repair/00-b2-test-truth-note.md`

Historical validation command inventory (current existence rechecked
2026-08-15):

- `cargo check -p ocentra-lan-core --tests`
- `cargo check -p ocentra-parent-agent-service --tests`
- `cargo check -p ocentra-parent-runtime-core --tests`
- missing and must be restored or replaced:
  `node scripts/test/v0-9-lan-source-matrix-plan-completion.mjs`
- missing and must be restored or replaced:
  `node scripts/test/v0-9-lan-signed-discovery-relay-spine.mjs`
- missing and must be restored or replaced:
  `node scripts/test/v0-9-production-discovery-household-proof.mjs`
- missing and must be restored or replaced:
  `node scripts/test/v0-9-household-lan-proof-readiness.mjs`

## Claims Supportable Now

- the Rust-owned LAN contract/read-model boundary is the current executable truth target
- all 25 workpacks now have reviewed code/test ownership; 22 have bounded
  Phase 1 code/expected tests written, while `16`, `20`, and `25` retain exact
  code/test gaps
- historical generated source-matrix and signed-discovery artifacts do not
  prove the two absent aggregate verifier programs exist in the current tree
- no generated LAN proof is freshly reverified in this checkout
- tests must live in real organized test folders and crates; placeholder folders, inline source-owned tests, and fake/mock coverage claims do not count as higher-category proof
- weak/manual network evidence remains fenced from child-agent identity claims
- signed hello/heartbeat is not overclaimed as implemented proof
- stale TS package ownership and stale proof references are no longer valid current-plan truth

## Claims Not Supportable Now

- regenerated portal screenshot proof
- physical two-device household LAN proof
- router/firewall reachability proof
- real signed child hello/heartbeat artifacts
- full replayable LAN event proof
- first-run household/setup/account UI completion

## Immediate Next Slice

After the 2026-06-28 truth-sync, the next open work remains the explicit
partial/manual rows: `16`, `18`, `19`, `20`, `23`, `25`, plus packet/manual
tails for locally code-complete discovery rows. That does not park `21-25`;
those follow-on rows remain active scope and must stay aligned to Rust-first
ownership, organized tests where applicable, and real proof.
