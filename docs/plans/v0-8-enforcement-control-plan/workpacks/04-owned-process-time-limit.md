# 04 Owned-Process Time Limit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `04 Owned-Process Time Limit`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

Sources: [20-step plan](../v0-8-enforcement-control-20-step-plan.md),
[test blueprint](../v0-8-enforcement-control-test-blueprint.md),
[folder README](../README.md),
[app-game-control feature](../../features/app-game-control.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Allow the narrow owned-process terminate/time-limit path only when target
identity, policy decision refs, and adapter capability are all present and
audited.

## Central schema boundary

```text
schema-domain owns public enforcement action, process identity, result, and audit schemas when they cross package/crate/protocol boundaries.
policy-control-plane-plan owns upstream policy authority and time-budget intent.
app-game-plan owns app/game process identity and stored session evidence.
enforcement-domain and agent-protocol may expose helper/proof/adapter parity surfaces only.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/app-game-control.md`
- `../../expectations/enforcement.md`

## Target State

Scoped process control is safe, explicit, auditable, restart-aware, and
parent-visible as one narrow adapter capability.

## Required proof fields

```text
canonical_schema_owner_state
policy_decision_ref_state
process_identity_state
session_or_evidence_ref_state
adapter_capability_state
mismatch_state
already_exited_state
unavailable_state
rollback_state
audit_state
manual_required_state
no_broad_app_block_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/04-owned-process-time-limit/`

Focused validation should record:

- `npm run test --workspace @ocentra-parent/enforcement-domain -- enforcement`
- `cargo test -p ocentra-parent-agent-core enforcement`
- `cargo test -p ocentra-parent-agent-service enforcement`
- selected app/game proof only when this slice consumes app/game handoff state

## AI Worker Checklist

- [x] Require pid and process identity checks before action.
- [x] Record mismatch, already-exited, unavailable, and success outcomes.
- [x] Tie limits to policy decisions and app/game evidence refs.
- [x] Add rollback/recovery where the adapter supports it.
- [x] Keep broad app blocking manual-required.

## Where We Are

Owned-process terminate/time-limit proof exists in narrow form. It must remain
clearly separate from broad app blocking.

## Negative Cases

- pid mismatch or stale process identity must reject rather than hit the wrong target
- already-exited targets must stay no-op/audited rather than success
- missing policy decision or evidence refs must block dispatch-ready state
- unsupported rollback or restart behavior must stay manual-required
- broad installed-app targets must not be upgraded into owned-process control

## Manual-Required Gaps

- Broad installed-app blocking remains manual-required until a separate adapter
  and proof path exists.
- Platform-specific restart/rollback behavior remains manual-required where the
  adapter cannot prove it.
- Mobile and non-Windows parity remain unclaimed.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch recorded: `codex/enforcement-wp06-managed-browser-adapter`.
- [x] Validation commands and results are recorded by `npm run test:enforcement-owned-process-time-limit-proof`.
- [x] Retained proof route: `docs/proof/v0-8-enforcement-control-plan/slice-04-owned-process-time-limit.md`; runtime artifact is `test-results/v0-8-owned-process-time-limit-proof/proof.json`.
- [x] Known gaps/manual-required states are listed in this workpack and the proof note.
