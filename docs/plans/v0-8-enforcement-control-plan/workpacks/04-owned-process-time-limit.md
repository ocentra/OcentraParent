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

## WP04-S1: Authenticated Parent-Runtime Delivery

This is the first required implementation slice for WP04. It closes the gap
between an authorized policy decision and the agent-service guard: a test-only
delivery record or a raw command envelope is not a production control path.

The completed path must be:

```text
authorized policy decision
-> durable parent-runtime outbox
-> authenticated, target-bound delivery envelope with monotonic sequence
-> agent-service verify and consume once
-> adapter result and journal
-> receipt bound to delivery/action/rollback state
-> parent-runtime idempotent receipt transition or manual-required recovery
```

### Ownership and boundary

- `policy-control-core` owns delivery and execution-receipt state validation.
- `parent-runtime-core` owns durable issuance, retry, expiry, receipt ingest,
  and must never mark an effect executed without the bound agent receipt.
- `agent-protocol` owns the cross-process envelope and receipt DTOs.
- `agent-service` owns authentication, target/evidence/PID binding, one-time
  consume, adapter dispatch, and receipt emission. Raw envelopes stay rejected.
- This workpack owns the assembled authority-to-effect proof. App/game keeps
  process/session evidence ownership; portal consumes a later receipt-backed
  read model and is not part of WP04-S1.

### Required contract and failure conditions

Each issued delivery needs a durable delivery id, strictly monotonic target
sequence, issuer binding, decision reference, target/evidence/PID binding,
issued/expiry time, and rollback reference where the adapter supports rollback.

- A forged, expired, stale, target-mismatched, or replayed delivery must have
  no adapter/journal effect.
- Parent restart and lost-receipt retry must preserve the same delivery id and
  sequence; retry must not duplicate the controlled effect.
- A receipt with mismatched delivery, action, target, or rollback binding must
  not advance parent state.
- Unavailable transport, failed execution, or missing rollback support must
  stay visible as failed/manual-required; they are not a success or ready claim.

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

- [ ] Require pid and process identity checks before action.
- [ ] Record mismatch, already-exited, unavailable, and success outcomes.
- [ ] Tie limits to policy decisions and app/game evidence refs.
- [ ] Add rollback/recovery where the adapter supports it.
- [ ] Keep broad app blocking manual-required.
- [ ] WP04-S1: issue durable authenticated deliveries from parent runtime and
      prove agent consume-once, receipt binding, restart/retry, and recovery.

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
- Trusted dispatch remains unscheduled/manual-required until Eventing WP06
  supplies its actual handoff and WP11 then supplies the durable-journal proof.
  A precise blocker records the gap but does not satisfy either prerequisite.
- Platform-specific restart/rollback behavior remains manual-required where the
  adapter cannot prove it.
- Mobile and non-Windows parity remain unclaimed.
- WP04-S1 is not complete until a production parent-runtime issuer/outbox and
  receipt transition exist. Test seeding and an agent-side rejection guard alone
  do not satisfy this workpack.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/04-owned-process-time-limit/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
