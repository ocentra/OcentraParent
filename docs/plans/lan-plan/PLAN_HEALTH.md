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
- Workpacks indexed: 20

## Consistency warnings

- No high-level checklist/workpack contradiction detected by the generated health check. Still verify the assigned workpack and checklist rows before DONE/PR_READY.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes household LAN discovery, trust, peer identity, local transport, provider mesh bridge, and LAN proof work here.
- Scope split: LAN topology, authenticated local peers, discovery, lease/claim, degraded/offline behavior, and transport proof stay here. AI job semantics, eventing local bus semantics, and enforcement outcomes stay in their owning plans unless named.
- Minimum read set: assigned workpack, exact checklist row, `TEST_PROOF_EXPECTATIONS.md`, and only the bridge docs named by the workpack.
- Test/proof decision: require authN/authZ, replay, token/lease lifecycle, clock skew, peer spoofing, partial outage, retry storm, rate limit, connection exhaustion, and two-device/manual topology proof where touched.
- DONE blocker: no LAN row may claim household execution until proof shows authenticated peer discovery, custody boundaries, duplicate prevention, failure handling, and no raw sensitive transfer outside the allowed contract.
