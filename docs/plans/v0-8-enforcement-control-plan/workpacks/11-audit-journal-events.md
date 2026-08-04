# 11 Audit And Journal Events

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `11 Audit And Journal Events`
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
[enforcement-integrity-tamper feature](../../features/enforcement-integrity-tamper.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Define the durable audit and journal trail for every enforcement state
transition so parent surfaces, reports, and proof scripts can reconstruct what
happened and why.

## Central schema boundary

```text
schema-domain owns public audit event, action result, reason, and reference schemas when they cross package/crate/protocol boundaries.
eventing-plan WP06 Journal Replay And Lineage owns generic replay, idempotency, and journal mechanics.
policy-control-plane-plan owns upstream policy authority and approval semantics.
v0-8-enforcement-control-plan owns enforcement-specific action, rollback, approval, and visibility event meaning.
```

## Required handoff to WP04 trusted dispatch

WP11 is not a downstream reporting embellishment. Before WP04 is scheduled for
dispatch-ready proof, Eventing WP06 Journal Replay And Lineage is reopened by
its Eventing owner and must provide its generic replay/idempotency mechanics
handoff. WP11 then provides the enforcement-specific durable journal contract
and route to that actual prerequisite.

The handoff must make these states queryable and durable enough for WP04 to
consume: accepted/rejected dispatch intent, adapter result/no-op/mismatch/
unavailable, rollback/recovery, actor/target/policy/evidence references, and
redacted parent-visible receipt/read-model references. If the selected
Eventing WP06 is reopened with cited proof absent in this checkout. Until its
actual handoff is retained, WP11 is blocked and WP04 remains
unscheduled/manual-required; an exact blocker records the gap but does not
satisfy it. Neither may manufacture a local journal or advance action state
from a no-op.

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/enforcement-integrity-tamper.md`
- `../../expectations/enforcement.md`

## Target State

Every product-control transition has a durable event that can be queried by
portal, reports, and proof scripts with evidence and policy references.

## Required proof fields

```text
canonical_schema_owner_state
accepted_state
rejected_state
adapter_result_state
no_op_state
rollback_state
approval_state
evidence_ref_state
actor_route_target_state
query_state
redaction_state
no_summary_invention_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/11-audit-journal-events/`

Focused validation should record:

- `cargo test -p ocentra-parent-agent-service enforcement`
- `cargo test -p ocentra-parent-agent-core enforcement`
- selected eventing or journal query proof for this slice
- selected portal/report consumers only when they render audit-backed history

## AI Worker Checklist

- [ ] Journal action accepted, action rejected, adapter result, and no-op.
- [ ] Journal timer and rollback transitions.
- [ ] Journal approvals, denials, expiry, and overrides.
- [ ] Include evidence, policy, actor, route, and target references.
- [ ] Add read-model/query coverage for recent action history.

## Where We Are

Enforcement action states exist, but product trust requires durable audit for
actions, failures, previews, timer transitions, and approvals.

## Negative Cases

- missing audit entries must block ready claims
- redaction gaps must not be hidden behind generic success summaries
- replay/idempotency drift must stay explicit rather than silently duplicating outcomes
- actor, target, or route-less events must not count as complete audit proof
- UI/report summaries must not invent action history that the journal cannot query

## Manual-Required Gaps

- Export, long-term retention, and sync/report delivery remain separate
  dependency-owned slices when selected.
- Cross-process or cross-device replay behavior remains unclaimed unless the
  selected proof explicitly covers it.
- Assistant or notification consumers remain downstream, not proof of audit
  completeness.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/11-audit-journal-events/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
