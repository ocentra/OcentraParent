# LAN Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `lan-plan`
> Doc: `LAN Plan Health Report`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This file records documentation health and consistency checks for the plan. It is generated from the existing docs and should be updated manually when the plan state is cleaned further.

## Status sources

- Short README: `README.md`
- Preserved full README: `README_FULL_ORIGINAL.md`
- Current snapshot: `current-lan-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 25 active rows, with `23` and `25` still open follow-on workpacks
- Current proof roots: Slice A, B1, B2, and current `01-25` row proof roots under `output/lan-plan-proof/`

## Consistency warnings

- No high-level checklist/workpack contradiction detected by the generated health check. Still verify the assigned workpack and checklist rows before DONE/PR_READY.
- B1 is local proof-regeneration only and ends in `not-ready-for-product-ready-household-lan-claim`.
- B2 is test-category truth only; placeholder test folders do not count as integration/e2e/security/performance/load coverage.
- Rust owns canonical LAN contracts, shapes, business logic, read models, runtime truth, and proof truth. Remaining TS LAN package surfaces are migration residue or UI-only and are not ownership signals.
- Real organized test folders/crates only; inline source-owned tests, placeholder directories, and mock-only coverage do not count as closure.
- Workpacks `21-25` are active LAN follow-on scope. Rows `21`, `22`, and `24`
  now have their own completion proof; rows `23` and `25` remain open and
  still need manual/physical proof before broader completion claims.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.
- Do not claim READY from single-machine proof as multi-device household proof.
- Do not claim READY from `lan-domain` unit tests as integration/e2e/security/performance/load coverage.
- Do not claim READY from schema/contract proof as packet/runtime proof.
- Do not claim READY from source matrix proof as physical LAN discovery proof.
- Do not claim READY from portal rendering as LAN truth proof.
- Do not claim READY from B1/B2 local proof as signed hello/heartbeat, service/runtime, portal, physical household, router/firewall, Android/mobile, or relay proof.
- Do not claim READY from follow-on workpacks unless each included row has its
  own row truth, organized tests where applicable, and current proof artifacts.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
schema/contract proof
weak LAN source evidence
packet/runtime discovery
service/read-model proof
portal projection
signed child hello/heartbeat proof
household assignment/revocation/audit
physical two-device/router/firewall proof
remote relay/cloud proof
rollout gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
real second-device household proof
router/firewall reachability proof
real signed child hello and heartbeat artifacts
replay/restart/event-stream proof completion
portal and downstream consumer proof artifacts
Android/mobile-controller proof where the plan still keeps those claims
integration/e2e/security/performance/load categories beyond unit tests
```

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes household LAN discovery, trust, peer identity, local transport, provider mesh bridge, and LAN proof work here.
- Scope split: LAN topology, authenticated local peers, discovery, lease/claim, degraded/offline behavior, and transport proof stay here. AI job semantics, eventing local bus semantics, account/device trust, remote relay, portal UI, and enforcement outcomes stay in their owning plans unless named.
- Minimum read set: assigned workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and only the bridge docs named by the workpack.
- Test/proof decision: require authN/authZ, replay, token/lease lifecycle, clock skew, peer spoofing, partial outage, retry storm, rate limit, connection exhaustion, and two-device/manual topology proof where touched.
- DONE blocker: no LAN row may claim household execution until proof shows authenticated peer discovery, custody boundaries, duplicate prevention, failure handling, and no sensitive transfer outside the allowed contract.

## PR-ready rule

The whole plan is PR-ready only when the selected rollout/proof gate consumes or
blocks every still-open authoritative `01-25` proof path that applies to the
current state and updates `PLAN_STATE.md`.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, no-claim language, and remaining open workpacks listed.
