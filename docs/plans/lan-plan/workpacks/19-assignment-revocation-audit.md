# 19 Assignment, Revocation, And Audit

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

Current proof includes trusted registry and route-control direction, but the
full parent assignment, rename, trust, ignore, revocation, and audit behavior is
not product complete across restart, rescan, and portal flows.

Branch `codex/v0-9-lan-signed-discovery-relay-spine` strengthens the typed
registry, route-custody, revoked-route, relay/cache unavailable, and rejected
decision states. Parent-facing read-model consumption now shows parent
decision/audit/route-custody state and sends the existing add-device request
command for controllable LAN slots. First-class rename, trust, ignore, restore,
and revoke controls still need matching command surfaces before they can be
called product complete.

Current B-lane follow-up adds the missing neighbor-only rename/type routing
case: canonical household decisions without a child-agent id now route through
the local-network LAN service path, return add-device reported events, and
survive portal refresh. The installer/PIN handshake, second child-agent proof,
and full assignment/revocation negative proof remain open.

## Where We Want To Be

Parent decisions are durable and explicit. Parent assignment links a device to a
child only through manual parent action or signed child-agent confirmation.
Rename, trust, ignore, revocation, stale, offline, and route rejection are
audited and survive restart.

## Requirement Checklist

- [x] Current branch proves route-custody and trusted-registry safety states for
      accepted and rejected signed discovery/relay decisions.
- [x] Current branch models revoked, unavailable, manual-required, and
      wrong-target route states without allowing weak LAN evidence to become
      authority.
- [x] Portal selected-device details and Activity/Network diagnostics now show
      parent decision/audit rows, route custody, route rejection state, and
      relay/cache custody from the typed LAN read model.
- [x] Portal add-to-parent flow now calls the existing
      `agent.lan-pairing.add-device.request` command only for selected LAN
      slots with a controllable route.
- [x] Expose first-class portal controls for add, route select, rename, trust,
      ignore, restore, and revoke by reusing existing LAN command surfaces:
      `agent.lan-pairing.add-device.request` for household decisions plus route
      select/revoke commands for route custody. Portal command routing now sends
      LAN commands to the selected local-network child target.
- [x] Portal household rename/type for a LAN-discovered neighbor uses canonical
      device identity, routes over `local-network` even without a child-agent id,
      receives `agent.lan-pairing.add-device.reported`, and survives portal
      refresh from service readback.
- [ ] Reject anonymous, wrong-origin, wrong-route, wrong-device, replayed, and
      expired pairing/control requests.
- [~] Preserve assignment and rename through rescan and weak evidence changes.
  Rename/type survives portal refresh through service readback; full rescan,
  restart, assignment, and weak-evidence contradiction proof remains open.
- [ ] Apply revocation before any new rule, query, approval, or heartbeat
      authority is accepted.
- [x] Show selected-device and route status clearly in parent-visible state.

## Acceptance And Proof

- Service tests cover anonymous rejection, accepted pairing, wrong origin,
  wrong device, revoked device, service restart, safe unpaired fallback, and
  audited command rejection.
- Portal tests cover LAN slot metadata for parent decisions/proof fields and
  portal transport routing for LAN commands to local-network child targets.
  Focused browser proof covers editable rename/type input, refresh persistence,
  and Update/Capability gating for an unpaired LAN neighbor. Full first-run
  setup, recovery, and degraded UX tests remain tracked in the family setup
  feature gap.
- Audit records include source, route, actor, target device, reason, and
  outcome.

## Parallel Ownership Notes

This work crosses domain, service, and portal state. Keep portal actions typed
and route-checked; do not let UI commands bypass the child-agent authority.
