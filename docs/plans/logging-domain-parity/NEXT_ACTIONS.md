<!-- agent-capsule -->

> Agent Capsule
> Plan: `logging-domain-parity`
> Doc: `Logging Domain Parity Next Actions`
> Kind: resume queue and highest-open work.
> Read when: starting or resuming after PLAN_STATE.md.
> Stop rule: Pick one workpack; do not broaden into unrelated plans.
> Proves: next-action routing only.
> Does not prove: implementation completion or PR readiness.
> Proof rule: Update this file only when queue state changes.

<!-- /agent-capsule -->

# Logging Domain Parity Next Actions

## How to use

1. Confirm the branch is `codex/tracking-plan-full-continuation-a`.
2. Pick one workpack from `WORKPACK_INDEX.md`.
3. Open that workpack only.
4. Fill the workpack pre-edit note.
5. Implement, test, run, proof, then update docs.

## Highest-priority queue

Completed in this checkout:

```text
WP01 current-state/reference audit
WP02 TypeScript package parity
WP03 parent architecture and routing
WP09 log control, retention, and bridge lifecycle
WP04 Rust logging-core
WP05 local validation evidence
WP07 MCP query interface
WP08 logger instrumentation and adoption
WP10 proof trace pipeline
WP06 validation and enforcement re-audit
```

### 1. No open workpacks remain in this plan

Expected result:

```text
if work resumes here, treat it as regression repair, rollout follow-up, or scope expansion backed by a new user assignment
```

### 2. Preserve completion boundaries

```text
do not reinterpret unrelated portal workspace build failures as logging-domain parity regressions unless the failing files or tests enter this plan's owned logging surfaces
```

## PR readiness guard

A PR-ready slice should close a named workpack or explicitly list remaining rows.

Do not create a PR that only:

```text
updates checklist text
adds proof prose
renames docs
adds TODO comments
```

unless the assigned workpack is explicitly proof-routing-only.

## Actioned completion tracker

- [x] Re-check this plan route from `README.md`, `AGENTS.md`, and `PLAN_STATE.md`.
- [x] Select one workpack from `WORKPACK_INDEX.md`.
- [x] Implement at least one real source/test behavior before proof/doc updates.
- [x] Record focused commands and evidence path before reporting progress.
- [x] Close the remaining WP08 portal and script adoption rows before claiming the plan folder complete.
- [x] Re-audit the downstream WP10 completion claims before calling the broader plan done.
- [x] Re-audit the downstream WP06 completion claims before calling the broader plan done.
- [x] Complete the final plan-level completion audit against live MCP, TS, Rust, portal proof-trace, and validation evidence.
- [x] Report the completion boundary honestly, including the unrelated portal workspace build failures outside this plan scope.
