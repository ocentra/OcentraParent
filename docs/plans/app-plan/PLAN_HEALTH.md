<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-plan`
> Doc: `PLAN_HEALTH.md`
> Kind: plan health and route audit.
> Read when: Only for broad status, route-quality, staleness, or PR_READY claims.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Health changes require PLAN_STATE.md and relevant route indexes to stay aligned.

<!-- /agent-capsule -->

# Native Apps Plan Health Report

## Route Health

- Plan route files present: yes
- Workpacks indexed: 95
- Implementation completion claimed: no

## Agent Route Walkthrough

- Landing decision: root AGENTS.md -> docs/agent/TASK_ROUTER.md -> docs/PLAN_INDEX.md or docs/FEATURE_ROUTE_INDEX.md selects this plan for native app identity, installed inventory, process/runtime, foreground app evidence, app-only policy targets, app catalog/settings, and legacy app-plan reconciliation.
- Minimum read set: AGENTS.md, PLAN_STATE.md, NEXT_ACTIONS.md, WORKPACK_INDEX.md, one workpack, TEST_PROOF_EXPECTATIONS.md.
- Stop rule: adjacent plans and source trees stay closed until the selected workpack names an exact handoff.
- Test/proof decision: use local expected proof inventory first; escalate to global validation matrix only when touched risk is broader.
- DONE blocker: this plan cannot claim implementation or product completion without proof artifacts, checklist rows, and feature status sync.

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `app-plan`.
- Ownership path: this plan is coordinated via `app-plan/AGENTS.md`, `app-plan/PLAN_STATE.md`, and `app-plan/NEXT_ACTIONS.md` plus selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `app-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan�s AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and the assigned plan workpacks.
