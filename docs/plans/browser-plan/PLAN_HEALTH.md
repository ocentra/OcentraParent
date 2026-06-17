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

This file records documentation health and consistency checks for the plan. It
is generated from the existing docs and should be updated manually when the
plan state is cleaned further.

## Status sources

- Short README: `README.md`
- Preserved full README: `README_FULL_ORIGINAL.md`
- Current snapshot: `current-browser-snapshot.md`
- Implementation checklist present: true
- Numbered workpacks indexed: 24

## Consistency warnings

- A high-level checklist/workpack contradiction exists: the numbered workpack
  files still contain open checkbox rows even though older generated summaries
  marked them as checked.
- The expected plan-local proof roots under
  `output/browser-plan-proof/<workpack-file-stem>/` are absent in this
  checkout.
- The browser-plan docs still require localized cleanup where older generated
  summaries or legacy ownership notes conflict with the audited source/test
  truth.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update the relevant checklist row/section.
- Update `PLAN_STATE.md`/`NEXT_ACTIONS.md` if the current state changed.
- Update feature/product docs if a product claim, gap, or proof changed.
- Do not use a stale checked row to override an open assigned workpack or hub
  instruction.

## Agent Route Walkthrough

- Landing decision: root `AGENTS.md` routes browser inventory, managed profile,
  active tab, URL, browser intervention, browser settings, browser UI, and
  browser AI evidence here.
- Scope split: browser facts stay separate from native app/game, decrypted
  network payloads, screen content analysis, and enforcement execution. Those
  plans are read only when the assigned workpack names the handoff.
- Minimum read set: one workpack, exact checklist row, browser source index
  only for ownership ambiguity, and `TEST_PROOF_EXPECTATIONS.md` for
  test/proof selection.
- Test/proof decision: require managed/unmanaged profile, custody/redaction,
  URL normalization, redirect/URL-hijack, origin/header/security, authZ,
  rollback, idempotency, rate-limit, and UI screenshot/log proof where touched.
- DONE blocker: no browser claim may move unless proof distinguishes
  installed/running browser state, managed profile custody, active tab
  evidence, policy intent, and actual intervention authority.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and
  proof expectations for `browser-plan`.
- Ownership path: this plan is coordinated via `browser-plan/AGENTS.md`,
  `browser-plan/PLAN_STATE.md`, and `browser-plan/NEXT_ACTIONS.md` plus
  selected workpack files.

### State

- Current state: route and schema hygiene are present, but documentation truth
  is not yet fully aligned with actual workpack/proof state, and
  implementation/closure proof remains incomplete.
- Current action: reconcile status, ownership, and proof-routing docs before
  any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected
  workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing,
  checklist/workpack states diverge, or known risks remain unmitigated with no
  explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows,
  proof artifacts named in this file, and cross-plan handoff notes in
  AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`,
  `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
