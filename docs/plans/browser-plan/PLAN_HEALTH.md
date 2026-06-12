# Browser Plan Health Report

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `Browser Plan Health Report`
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
- Current snapshot: `current-browser-snapshot.md`
- Implementation checklist present: true
- Workpacks indexed: 24

## Consistency warnings

- No high-level checklist/workpack contradiction detected by the generated health check. Still verify the assigned workpack and checklist rows before DONE/PR_READY.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes browser inventory, managed profile, active tab, URL, browser intervention, browser settings, browser UI, and browser AI evidence here.
- Scope split: browser facts stay separate from native app/game, decrypted network payloads, screen content analysis, and enforcement execution. Those plans are read only when the assigned workpack names the handoff.
- Minimum read set: one workpack, exact checklist row, browser source index only for ownership ambiguity, and `TEST_PROOF_EXPECTATIONS.md` for test/proof selection.
- Test/proof decision: require managed/unmanaged profile, custody/redaction, URL normalization, redirect/URL-hijack, origin/header/security, authZ, rollback, idempotency, rate-limit, and UI screenshot/log proof where touched.
- DONE blocker: no browser claim may move unless proof distinguishes installed/running browser state, managed profile custody, active tab evidence, policy intent, and actual intervention authority.
