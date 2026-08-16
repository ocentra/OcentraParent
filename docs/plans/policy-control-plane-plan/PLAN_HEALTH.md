# Policy Control Plane Plan Health

Health: audit-open. Real contract/source coverage exists, but proof routing, plan closure claims, and cross-plan completion truth are not yet fully closed.

Known risks: duplicate policy truth, ad hoc domain compilers, assistant writes without parent confirmation, schedule/DST bugs, stale/offline delivery, missing audit, event replay drift, and fake-green closure claims.

## Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `policy-control-plane-plan`.
- Ownership path: this plan is coordinated via `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and the selected workpack plus proof inventory.

## State

- Current state: route/contract docs and retained proof exist, but WP01/WP03 are contract-only with no trusted durable source owner or shipped compiler caller; WP02/WP04/WP05 also remain open. WP06/WP07/WP08 route evidence without establishing those production paths.
- Current action: implement WP01 source authority before WP03 compiler composition, then consume that output in WP04/Screen-AI; keep all production-open workpacks out of DONE/PR_READY.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update proof links under `docs/proof/policy-control-plane-plan/`.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and `PROOF_INDEX.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not claim READY from UI preview alone.
- Do not claim READY from compiler output or compiler tests as runtime domain effects.
- Do not claim READY from event model proof as delivery/ack proof.
- Do not claim READY from assistant draft as parent approval.
- Do not claim READY from child request as parent approval.
- Do not claim READY from single-domain ack as global active policy.
- Do not claim READY from policy delivery as enforcement authority.
- Do not claim READY from focused contract passes while WP02/WP05 remain open.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
source truth
schedule/timezone/DST/conflict
parent authoring and preview
domain compiler contracts
delivery/ack/audit
ask-parent and overrides
event model and replay
rollout proof gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
WP02 parent authoring/preview/conflict/approval rendered surfaces
WP01 trusted identity-backed durable policy source registration/query owner
WP03 shipped active-source-to-domain compiler and artifact persistence/delivery
WP04 trusted delivery authority and inspectable execution trace
WP05 ask-parent/override parent confirmation/assistant/portal/chat integration
child-agent validation handoff for ask-parent path
device-trust high-risk parent presence handoff
data-custody policy export/delete/sync handoff
enforcement authority/rollback handoff
broader architecture gate failure in agent-protocol-domain re-export debt
```

## Decision routes and failure controls

- Decision route: follow `AGENTS.md`, the selected workpack path, `WORKPACK_FAMILIES.md` when needed, and the proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, proof artifacts are absent, or known risks remain unmitigated with no explicit deferral.
- Platform proof boundary: real iOS/macOS proof is an external-platform constraint on this host; Windows, Android, WSL, and Docker paths remain expected where relevant and should not be reported as blocked unless a real dependency prevents them.

## Proof mapping

- Required proof before READY: explicit artifact files under `docs/proof/policy-control-plane-plan/`, matching focused validation logs, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- The current checkout has proof files for WP01/WP03/WP04/WP06/WP07/WP08, but WP01/WP03 are contract proof and WP04 is dependency-blocked; production gaps remain open in WP01-WP05.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and the assigned workpack.

## PR-ready rule

The whole plan is PR-ready only when WP01-WP05 have real production callers/authority, required tests, and proof (or an explicitly non-product scope decision), and WP06 consumes the final status.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, no-claim language, and remaining open workpacks listed.
