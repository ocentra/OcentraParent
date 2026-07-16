# 13 Service Read Models And API

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `13 Service Read Models And API`
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
[app-game-control feature](../../features/app-game-control.md),
[browser-web-control feature](../../features/browser-web-control.md),
[policy-schedules-approvals feature](../../features/policy-schedules-approvals.md), and
[enforcement expectation](../../expectations/enforcement.md).

## Purpose

Provide one coherent service-backed control-state read model so parent surfaces
consume the same typed action/capability/audit truth that the service exposes.

## Central schema boundary

```text
schema-domain owns public read-model, reason, capability, and status schemas when they cross package/crate/protocol boundaries.
agent-protocol and agent-protocol-domain own transport/read-model parity only.
portal-ux-household-surfaces-plan owns rendered UX consumption, not read-model truth.
v0-8-enforcement-control-plan owns the service-visible control-state contract and no-hardcoded-UI boundary.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/app-game-control.md`
- `../../features/browser-web-control.md`
- `../../features/policy-schedules-approvals.md`
- `../../expectations/enforcement.md`

## Target State

The Rust service returns validated read models for current capability, recent
actions, pending approvals, active timers, degraded states, and manual-required
gaps.

## Required proof fields

```text
canonical_schema_owner_state
protocol_contract_state
capability_matrix_state
active_timer_state
pending_approval_state
audit_history_state
manual_required_state
parent_visible_state
portal_validation_state
no_hardcoded_ui_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/13-service-read-models-and-api/`

Focused validation should record:

- `cargo test -p ocentra-parent-agent-service enforcement`
- `npm run test --workspace @ocentra-parent/agent-protocol-domain -- enforcement`
- selected portal tests that consume the same read model when this slice names them
- selected architecture gate for touched service/protocol/portal surfaces

## AI Worker Checklist

- [ ] Add or extend protocol contracts before service output.
- [ ] Include capability matrix row, proof level, source evidence, and route.
- [ ] Include active timer and pending approval summaries.
- [ ] Include recent audit/action history.
- [ ] Validate payloads in portal consumers.

## Where We Are

The service exposes multiple proof/read paths. Parent surfaces need one coherent
product-control state.

## Negative Cases

- payloads missing capability, manual-required, or audit history state must not claim completeness
- stale protocol or portal consumers must not hardcode or infer control state
- pending approval and timer summaries must not disappear when action rows are present
- service output must not upgrade UI-only or evidence-only facts into authority
- audit history gaps must block broad surface-readiness claims

## Manual-Required Gaps

- Portal rendering remains a downstream consumer slice and must not be used as
  read-model proof by itself.
- Cross-device API shape, report export, or sync delivery surfaces remain
  separate dependency-owned slices.
- Mobile or external-client parity remains unclaimed unless the selected proof
  explicitly covers it.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/13-service-read-models-and-api/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
