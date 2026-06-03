# Location, Geofence, And Device Status

## Parent Outcome

Parents can see last-known location, geofence transitions, battery/connectivity
status, and stale/offline/unavailable states where platform permissions allow.

## Ocentra Requirement

Location is not implied by LAN presence, IP address, or pairing. Location and
geofence behavior require explicit contracts, permissions, custody, retention,
and platform proof.

## Roadmap And Expectations

- Roadmap: V5 parent policy product, V6 mobile agents, V3 notifications.
- Expectations: [location/geofence](../expectations/location-geofence.md),
  [platforms](../expectations/platforms.md),
  [notifications](../expectations/notifications.md).
- Supporting docs:
  [tracking settings inventory](../tracking-control-settings-inventory.md),
  [tracking plan](../plans/tracking-plan/README.md).
- Modules: `packages/parent-domain`, `packages/activity-domain`,
  `platforms/android`, `platforms/ios`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
location, geofence, SOS, battery, notifications, and remote parent access.

Google, Apple, Microsoft, Bark, Qustodio, Kaspersky, FamilyTime, and FamiSafe
expose location or device-status features. Parents expect this category.

## Current Ocentra State

- A capability guide exists.
- A schema proposal exists.
- A first-class tracking plan now exists for location evidence, geofences,
  expected-place schedules, device status, child check-ins, temporary live
  tracking, missing-device mode, nearby-place intelligence, AI safety evidence,
  retention, custody, platform proof, UI, and rollout workpacks.
- Focused TypeScript contract proof now exists for activity tracking evidence
  and parent tracking policy/action contracts, with proof roots under
  `output/tracking-plan-proof/` and the repeatable proof command
  `node scripts/test/tracking-plan-contract-proof.mjs`.
- Platform permissions, mobile proof, Rust journal/SQLite runtime,
  retention/delete/export runtime, provider delivery, notification delivery,
  and UI are not product-complete.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is now in contract-proof progress, but remains a tracked
product gap until platform adapters, runtime engines, retention/delete/export,
journal/read models, provider delivery, notifications, and parent/child UI are
proved.

## Checklist

- [x] Location evidence contract.
- [x] Accuracy/source/stale-state fields.
- [x] Geofence rule and transition contracts.
- [x] Battery/connectivity status contract.
- [ ] Retention/delete settings.
- [x] Alert intent contract.
- [ ] Android permission/background proof.
- [ ] iOS entitlement/background proof.
- [x] Expected-place schedule and exception contracts.
- [x] Parent acknowledgement and escalation contracts.
- [x] Nearby-place ambiguity and AI safety evidence contracts.
- [ ] Journal/SQLite/read-model runtime proof.

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
Use `docs/plans/tracking-plan/README.md` for implementation sequencing and
workpack ownership. Keep AI as evidence, not authority, and keep LAN/IP/Wi-Fi
presence as hints only. The next implementation layer is Rust protocol,
journal/SQLite read-model proof, then platform adapters and UI snapshots.
