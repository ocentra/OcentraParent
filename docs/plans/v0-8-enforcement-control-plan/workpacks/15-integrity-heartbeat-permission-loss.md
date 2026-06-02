# 15 Integrity Heartbeat And Permission Loss

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
