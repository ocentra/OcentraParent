# Remote Access Plan Health

Health: execution-grade live-view route exists; standing paired access and proof matrix are present as architecture, but proof-backed closure remains open.

Known risks: remote access confused with screen capture, deferred control overclaimed from live-view proof, relay data retention, weak revocation/remove-device behavior, child disclosure gap, support/admin hidden access, relay replay/cross-household bugs, abuse/DoS risk.

## Required hygiene before PR_READY

- Update the assigned workpack.
- Update proof links under `output/remote-access-plan-proof/<workpack>/`.
- Update `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, and `PROOF_INDEX.md` if current state changes.
- Use `WORKPACK_FAMILIES.md` only when owner/proof family is unclear.
- Do not claim READY from local screen capture proof.
- Do not claim READY from LAN pairing or LAN transport proof.
- Do not claim READY from UI-only proof.
- Do not claim READY from relay route existence.
- Do not claim READY from legacy remote architecture docs alone.
- Do not claim READY from standing access without revoke/remove-device proof.
- Do not claim READY from live view proof as remote input/control proof.
- Do not claim READY from relay availability as data-retention permission.
- Do not claim READY for support/admin access without parent-visible grant and audit proof.
- Do not claim feature completeness until the relevant E2E tier in `TEST_PROOF_EXPECTATIONS.md` is explicitly proven or blocked.

## Known healthy boundaries

This plan intentionally separates:

```text
remote capability fabric
live-view relay
remote input/control deferred slice
pairing and standing grants
relay security and abuse controls
rollout proof gate
```

Do not collapse those boundaries.

## Known incomplete areas

```text
pairing/access lifecycle proof
standing access visibility proof
revoke/remove-device proof
reconnect/crash recovery denial proof
relay availability/fallback proof
relay abuse/rate-limit/replay/cross-household proof
child disclosure proof
retention/export/delete boundary for remote artifacts
portal remote route proof
support/admin parent-visible grant proof
```

## High-Information-Density Gate

### Scope and ownership

- Scope and ownership: this file governs documentation routing, state, and proof expectations for `remote-access-plan`.
- Ownership path: this plan is coordinated via `remote-access-plan/AGENTS.md`, `remote-access-plan/PLAN_STATE.md`, `remote-access-plan/NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `WORKPACK_FAMILIES.md`, and selected workpack files.

### State

- Current state: route and schema hygiene are present, but implementation/closure proof remains incomplete until checklist and workpack evidence are updated.
- Current action: keep this file and `remote-access-plan/PLAN_STATE.md` aligned before any DONE/PR_READY claim.

### Decision routes and failure controls

- Decision route: follow this plan's AGENTS landing decision, the selected workpack path, and the feature/doc proof matrix referenced in this file.
- Failure controls: do not claim completion when handoff routes are missing, checklist/workpack states diverge, proof artifacts are absent, or known risks remain unmitigated with no explicit deferral.

### Proof mapping

- Required proof before READY: explicit links from workpack checklist rows, proof artifacts named in this file, and cross-plan handoff notes in AGENTS/NEXT_ACTIONS.
- At minimum, align the following docs before READY: `AGENTS.md`, `PLAN_STATE.md`, `NEXT_ACTIONS.md`, `WORKPACK_INDEX.md`, `PROOF_INDEX.md`, `TEST_PROOF_EXPECTATIONS.md`, and the assigned plan workpack.

## PR-ready rule

The current live-view pass is PR-ready only when WP01, WP02, WP04, WP05, and WP06 close with proof artifacts or exact carried blockers. WP03 remains deferred and cannot be used to claim current-pass readiness.

A partial PR may be ready only when one selected workpack is closed with proof artifacts, command logs, negative cases, redacted diagnostics/custody notes, no-claim language, and remaining open workpacks listed.
