# 19 Assignment, Revocation, And Audit

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current proof includes trusted registry and route-control direction, but the
full parent assignment, rename, trust, ignore, revocation, and audit behavior is
not product complete across restart, rescan, and portal flows.

## Where We Want To Be

Parent decisions are durable and explicit. Parent assignment links a device to a
child only through manual parent action or signed child-agent confirmation.
Rename, trust, ignore, revocation, stale, offline, and route rejection are
audited and survive restart.

## Requirement Checklist

- [ ] Add assignment, rename, trust, ignore, revoke, restore, and audit
      contracts.
- [ ] Reject anonymous, wrong-origin, wrong-route, wrong-device, replayed, and
      expired pairing/control requests.
- [ ] Preserve assignment and rename through rescan and weak evidence changes.
- [ ] Apply revocation before any new rule, query, approval, or heartbeat
      authority is accepted.
- [ ] Show selected-device and route status clearly in parent-visible state.

## Acceptance And Proof

- Service tests cover anonymous rejection, accepted pairing, wrong origin,
  wrong device, revoked device, service restart, safe unpaired fallback, and
  audited command rejection.
- Portal tests cover assignment modal, child selection, rename, trust, ignore,
  selected-device display, accepted route, rejected route, and offline/stale.
- Audit records include source, route, actor, target device, reason, and
  outcome.

## Parallel Ownership Notes

This work crosses domain, service, and portal state. Keep portal actions typed
and route-checked; do not let UI commands bypass the child-agent authority.
