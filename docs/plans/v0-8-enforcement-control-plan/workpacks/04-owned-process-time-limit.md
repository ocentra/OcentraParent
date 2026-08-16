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

## Required durable-journal handoff before scheduling

WP04 may describe process identity and preflight constraints, but it must not
be scheduled for dispatch-ready or action-state proof until its audit/journal
prerequisite is available. The required order is:

```text
eventing-plan WP06 Journal Replay And Lineage generic replay/idempotency/journal mechanics
-> WP11 enforcement-specific durable journal contract and proof route
-> WP04 trusted dispatch, adapter result/no-op/mismatch/unavailable, and rollback state
```

WP11 owns enforcement-specific journal meaning; Eventing WP06 owns generic
mechanics. WP04 consumes their explicit handoff. Eventing WP06 is reopened with
its cited proof absent in this checkout, so it must produce its actual handoff
before WP11 proceeds. If either is absent, stale, or only a test double, record
`manual_required_state`/a precise blocker and keep
dispatch-ready, receipt, and parent-visible action claims unavailable.

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
wp11_journal_handoff_state
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
- the selected WP11 durable-journal proof and its Eventing WP06 Journal Replay And Lineage handoff

## AI Worker Checklist

- [x] Require pid and process identity checks before action.
- [x] Record mismatch, already-exited, unavailable, and success outcomes.
- [x] Tie limits to policy decisions and app/game evidence refs.
- [x] Add rollback/recovery where the adapter supports it.
- [x] Keep broad app blocking manual-required.

## Where We Are

Owned-process terminate/time-limit proof exists in narrow form. It must remain
clearly separate from broad app blocking.

### 2026-08-16 trusted-adapter production slice (code drafted; validation deferred)

The production boundary now has a private branded target path rooted in
authenticated grant consumption. The signed managed-process contract carries
only a managed-process identity; the verifier resolves PID/name, canonical
executable-path reference, and process-start identity from the local
ActivityStore launcher/session read model, then the Windows adapter re-checks
those values before effect. It persists an adapter-owned execution trace with
result and rollback state. Raw `OwnedProcessTerminationTarget` remains
evidence-only. This is code drafted and unvalidated: focused tests, durable
WP11 journal handoff, retained proof, and runtime receipt integration remain
deferred; missing local launcher/session authority fails closed/manual-required.

### Active pre-dispatch contract packet

`codex/v08-wp04-grant-handoff-contract` establishes only a typed authenticated
grant carrier and a receiver-owned pinned-verifier boundary. It does not route
a grant into the executor, call an adapter, add a journal, create a receipt, or
change any checkbox: WP11 remains the scheduling prerequisite for dispatch.
Its focused negative proof is retained under this workpack's deterministic
proof root.

### 2026-07-28 receipt persistence precondition

The receipt-required policy-delivery transition currently fails closed on the
ordinary handoff. A matching public receipt is not trusted-adapter authority
and must not be used to claim persistence or execution. This workpack remains
open until the parent-runtime-to-agent-service trusted dispatch ledger issues
non-forgeable adapter authority, persists verified provenance, and supports an
authenticated reload boundary. It is not a claim that a portal envelope has
trusted authority or that an OS adapter has executed.

The remaining execution boundary is explicit: agent-service currently rebuilds
policy inputs from command payload fields and does not yet consume an
authenticated persisted trusted-delivery record. Until that bridge exists,
direct service payloads cannot be used as WP04 trusted-adapter proof.

### 2026-08-10 code/test correction

The Windows owned-process adapter now records an already-exited target as an
audited `NoOp` with `PROCESS_ALREADY_EXITED`, rather than claiming
`ActuallyEnforced`. The time-limit boundary preserves its separate `Expired`
outcome for that terminal process result. The focused agent-core enforcement
target passed 53 tests and the agent-service enforcement target passed 40;
Rust format, targeted clippy, architecture policy, Enforcer guard, and diff
checks also passed on the consolidated E: branch. This is a code/test slice,
not WP04 completion: the checklist remains 0/5 and the trusted persisted
dispatch, durable journal, rollback, retained proof, CI, and main-merge gates
remain open.

## Negative Cases

- pid mismatch or stale process identity must reject rather than hit the wrong target
- already-exited targets must stay no-op/audited rather than success
- missing policy decision or evidence refs must block dispatch-ready state
- unsupported rollback or restart behavior must stay manual-required
- broad installed-app targets must not be upgraded into owned-process control

## Manual-Required Gaps

- Broad installed-app blocking remains manual-required until a separate adapter
  and proof path exists.
- Trusted dispatch remains unscheduled/manual-required until Eventing WP06
  supplies its actual handoff and WP11 then supplies the durable-journal proof.
  A precise blocker records the gap but does not satisfy either prerequisite.
- Platform-specific restart/rollback behavior remains manual-required where the
  adapter cannot prove it.
- Mobile and non-Windows parity remain unclaimed.

## Fill This Before Reporting DONE Or PR-ready

- [x] Workpack id and branch recorded: `codex/enforcement-wp06-managed-browser-adapter`.
- [x] Validation commands and results are recorded by `npm run test:enforcement-owned-process-time-limit-proof`.
- [x] Retained proof route: `docs/proof/v0-8-enforcement-control-plan/slice-04-owned-process-time-limit.md`; runtime artifact is `test-results/v0-8-owned-process-time-limit-proof/proof.json`.
- [x] Known gaps/manual-required states are listed in this workpack and the proof note.
