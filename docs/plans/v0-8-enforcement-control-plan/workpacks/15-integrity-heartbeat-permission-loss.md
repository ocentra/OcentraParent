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
[test blueprint](../v0-8-enforcement-control-test-blueprint.md), and
[folder README](../README.md).

## Where We Are

Tamper/uninstall is a tracked gap. Parent-visible health can still improve
without claiming anti-tamper.

## Where We Want To Be

Parents can see whether the child-agent is running, stale, offline, stopped,
permission-limited, outdated, unsupported, or degraded.

## Requirement Checklist

- [ ] Define integrity status, heartbeat, permission-loss, and degraded states.
- [ ] Keep platform proof level attached to every status.
- [ ] Add service tests for heartbeat/stale transitions.
- [ ] Add portal/read-model visibility where service-backed.
- [ ] Avoid anti-tamper claims from heartbeat alone.

## Acceptance And Proof

Proof output labels health states separately from tamper/uninstall protection.

## Parallel Ownership Notes

This workpack can proceed before anti-tamper design, but it must not cross into
stealth or persistence hardening.
