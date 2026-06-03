# 15 Household Device Store

Sources: [20-step plan](../v0-9-lan-discovery-20-step-plan.md),
[test blueprint](../v0-9-lan-discovery-test-blueprint.md),
[UI/UX guide](../ui-ux-requirements-guide.md), and
[folder README](../README.md).

## Where We Are

The lane direction is to avoid duplicate truth: canonical household rows are
derived from LAN add-device state today. Production still needs durable device
registry behavior that Devices, Policy, Activity, Network, Tracking, AI, and
Account can all trust.

Current B-lane proof adds service-backed household rename/type persistence for
LAN-discovered neighbors. The portal sends canonical household decisions through
the Rust LAN service, receives `agent.lan-pairing.add-device.reported`, and
renders the updated add-device read model after refresh. This removes the
portal-only optimistic identity state for this path, but does not complete full
restart/recovery proof across every parent decision.

## Where We Want To Be

The household device store is the canonical durable registry for known devices,
evidence, role badges, manual assignment, rename, trust, ignore, revoked,
stale, offline, unsupported, and manual-required states. Derived read models do
not own separate truth.

## Requirement Checklist

- [~] Persist device records, evidence, manual name, assigned child, trusted
  state, ignored state, revoked state, online state, first-seen, and
  last-seen. Manual name/device type persistence for LAN-discovered
  neighbors is proved; broader state coverage remains open.
- [~] Preserve parent decisions across rescan and restart. Portal refresh and
  service readback are proved for rename/type; full restart coverage remains
  open.
- [ ] Support migrations and safe fallback to unpaired state when registry proof
      is unavailable.
- [ ] Keep routers and unsupported devices visible but non-enrollable.
- [ ] Expose custody/source labels for local, LAN, cache, unavailable, and
      manual-required states.

## Acceptance And Proof

- Store integration tests insert, update, reload, migrate, trust, ignore,
  revoke, mark stale/offline, and recover after restart.
- Rescan tests prove manual assignment and rename survive weak contradictory
  evidence.
- Product docs/checklist are updated only when proof status actually moves.

Current proof:

- `output/lan-plan-proof/15-household-device-store/devices-identity-routing-proof.md`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-identity-persisted.png`
- `output/lan-plan-proof/15-household-device-store/06-ui-snapshots/devices-update-gated.png`

## Parallel Ownership Notes

Storage workers must coordinate closely with merge and read-model workers. Do
not introduce a second "canonical" table or portal-only registry.
