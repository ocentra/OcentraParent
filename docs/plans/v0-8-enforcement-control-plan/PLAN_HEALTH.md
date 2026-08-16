# V0.8 Enforcement Control Plan Health

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `V0.8 Enforcement Control Plan Health Report`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Health: audit-open. Route docs and proof routing are now aligned, but the plan
remains open because 14 workpacks still lack runtime or proof closure.

Known risks: AI/UI/evidence treated as authority, broad app blocking without
narrow adapter proof, managed-browser exact-URL overclaim, network blocking
overclaim, approval/expiry drift, missing audit, read-model/UI drift,
heartbeat mistaken for anti-tamper, and fake-green rollout claims.

## Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and
  proof expectations for `v0-8-enforcement-control-plan`.
- Ownership path: this plan is coordinated via `AGENTS.md`, `PLAN_STATE.md`,
  `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, the selected
  workpack, and `PROOF_INDEX.md`.

## State

- Current state: route docs now carry explicit ownership doctrine,
  workpack-family routing, enforcement E2E proof tiers, and a cleaned proof
  index. This is docs-only cleanup and does not close product/runtime gaps.
- Proof status: checked slices exist for WP01, WP02, WP03, WP07, WP09, and
  WP18 under `docs/proof/v0-8-enforcement-control-plan/` and
  `output/v0-8-enforcement-control-plan-proof/`.
- Current blockers: WP04, WP05, WP06, WP08, WP10, WP11, WP12, WP13, WP14,
  WP15, WP16, WP17, WP19, and WP20 remain open.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update proof links under `docs/proof/v0-8-enforcement-control-plan/` and the
  selected deterministic proof root.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and
  `PROOF_INDEX.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner or handoff boundaries are unclear.
- Do not claim READY from evidence capture, UI rendering, AI output, or focused
  contract passes alone.
- Do not claim READY from managed-browser proof as exact-URL control.
- Do not claim READY from network visibility as network blocking.
- Do not claim READY from heartbeat/install visibility as anti-tamper proof.
- Do not claim READY from one checked workpack while the selected assigned
  workpack or its prerequisite family remains open.

## Known healthy boundaries

This plan intentionally separates:

```text
policy authority
evidence refs
adapter capability
execution state
rollback and recovery
approval and override
audit and journal
service read model
portal visibility
integrity and non-claim state
rollout proof gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
WP04 owned-process narrow execution proof
WP05 app/game session evidence handoff
WP06 managed browser session control
WP08 network/domain report-only boundary
WP10 parent approval and override
WP11 audit and journal events
WP12 child-facing status and reasons
WP13 service read models and API
WP14 portal control state consumption
WP15 integrity heartbeat and permission loss
WP16 tamper/uninstall non-claim design
WP17 cross-platform unavailable states
WP19 Playwright and UI proof
WP20 rollout docs and CI/PR gate
```

## Decision routes and failure controls

- Decision route: follow `AGENTS.md`, the selected workpack path,
  `WORKPACK_FAMILIES.md` when needed, and the proof matrix referenced here.
- Failure controls: do not claim completion when handoff routes are missing,
  checklist/workpack states diverge, proof artifacts are absent, or known risks
  remain unmitigated with no explicit deferral.
- Platform proof boundary: real iOS/macOS proof is an external-platform
  constraint on this Windows host. Windows proof is expected where relevant.

## Proof mapping

- Required proof before READY: explicit artifact files under
  `output/v0-8-enforcement-control-plan-proof/`, matching notes under
  `docs/proof/v0-8-enforcement-control-plan/`, focused validation logs, and
  cross-plan handoff notes when a selected workpack names them.
- At minimum, align `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`,
  `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and the
  selected workpack before READY.

## PR-ready rule

The whole plan is PR-ready only when the open execution, approval, audit,
read-model, integrity, and rollout workpacks are closed with proof artifacts or
explicit carried blockers and WP20 consumes the final status honestly.

A partial PR may be ready only when one selected workpack is closed with proof
artifacts, validation logs, negative cases, no-claim language, and the
remaining open workpacks are listed explicitly.
