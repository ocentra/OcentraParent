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

## Where We Want To Be

The household device store is the canonical durable registry for known devices,
evidence, role badges, manual assignment, rename, trust, ignore, revoked,
stale, offline, unsupported, and manual-required states. Derived read models do
not own separate truth.

## Requirement Checklist

- [ ] Persist device records, evidence, manual name, assigned child, trusted
      state, ignored state, revoked state, online state, first-seen, and
      last-seen.
- [ ] Preserve parent decisions across rescan and restart.
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

## Parallel Ownership Notes

Storage workers must coordinate closely with merge and read-model workers. Do
not introduce a second "canonical" table or portal-only registry.
