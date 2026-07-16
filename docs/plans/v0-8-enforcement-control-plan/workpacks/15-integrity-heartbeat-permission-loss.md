# 15 Integrity Heartbeat And Permission Loss

<!-- agent-capsule -->

> Agent Capsule
> Plan: `v0-8-enforcement-control-plan`
> Doc: `15 Integrity Heartbeat And Permission Loss`
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

Define the parent-visible health state for running, stale, offline,
permission-limited, outdated, unsupported, or degraded child-agent control
surfaces without claiming anti-tamper behavior.

## Central schema boundary

```text
schema-domain owns public integrity-status, permission-loss, proof-level, and degraded-state schemas when they cross package/crate/protocol boundaries.
child-agent-runtime-distribution-plan and device-trust-bootstrap-plan own runtime presence, install, trust, and future hardening surfaces when selected.
v0-8-enforcement-control-plan owns parent-visible integrity state, manual-required visibility, and anti-claim boundaries.
```

## Source Inputs

- `../v0-8-enforcement-control-20-step-plan.md`
- `../v0-8-enforcement-control-test-blueprint.md`
- `../../features/enforcement-integrity-tamper.md`
- `../../expectations/enforcement.md`

## Target State

Parents can see whether the child-agent is running, stale, offline, stopped,
permission-limited, outdated, unsupported, or degraded.

## Required proof fields

```text
canonical_schema_owner_state
heartbeat_state
stale_state
offline_state
permission_loss_state
outdated_state
unsupported_state
degraded_state
proof_level_state
parent_visible_state
no_tamper_claim
no_claim
```

## Tests And Proof

Proof root: `output/v0-8-enforcement-control-plan-proof/15-integrity-heartbeat-permission-loss/`

Focused validation should record:

- `cargo test -p ocentra-parent-agent-service enforcement`
- selected service/read-model tests for heartbeat and stale transitions
- selected portal tests only when they show service-backed integrity state
- selected architecture gate for touched integrity/read-model surfaces

## AI Worker Checklist

- [ ] Define integrity status, heartbeat, permission-loss, and degraded states.
- [ ] Keep platform proof level attached to every status.
- [ ] Add service tests for heartbeat/stale transitions.
- [ ] Add portal/read-model visibility where service-backed.
- [ ] Avoid anti-tamper claims from heartbeat alone.

## Where We Are

Tamper/uninstall is a tracked gap. Parent-visible health can still improve
without claiming anti-tamper.

## Negative Cases

- missing heartbeats must not be shown as healthy
- stale/offline and permission-loss states must stay distinct
- unsupported or outdated platforms must not inherit ready status
- heartbeat visibility must not imply uninstall resistance or stealth
- portal summaries must not hide degraded/manual-required states

## Manual-Required Gaps

- Anti-tamper, uninstall resistance, stealth, or persistence behavior remain
  out of scope here.
- Platform-specific remediation flows remain dependency-owned slices.
- External/mobile parity remains unproved unless the selected proof covers it.

## Fill This Before Reporting DONE Or PR-ready

- [ ] Workpack id and branch recorded.
- [ ] Validation commands and results recorded in `16-validation-commands.log`.
- [ ] Proof artifacts under `output/v0-8-enforcement-control-plan-proof/15-integrity-heartbeat-permission-loss/`.
- [ ] Known gaps/manual-required states listed here and in the proof note.
