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
- Runtime contracts, platform permissions, mobile proof, and UI are not
  product-complete.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is a tracked product gap. Ocentra must build it or explicitly
position away from it before consumer-facing parity claims.

## Checklist

- [ ] Location evidence contract.
- [ ] Accuracy/source/stale-state fields.
- [ ] Geofence rule and transition contracts.
- [ ] Battery/connectivity status.
- [ ] Retention/delete settings.
- [ ] Alert integration.
- [ ] Android permission/background proof.
- [ ] iOS entitlement/background proof.
- [ ] Expected-place schedule and exception model.
- [ ] Parent acknowledgement and escalation model.
- [ ] Nearby-place ambiguity and AI safety evidence model.
- [ ] Journal/SQLite/read-model proof.

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
Use `docs/plans/tracking-plan/README.md` for implementation sequencing and
workpack ownership. Keep AI as evidence, not authority, and keep LAN/IP/Wi-Fi
presence as hints only.
