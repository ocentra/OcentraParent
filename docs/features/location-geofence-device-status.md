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
  [tracking settings inventory](../tracking-control-settings-inventory.md).
- Modules: `packages/parent-domain`, `packages/activity-domain`,
  `platforms/android`, `platforms/ios`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
location, geofence, SOS, battery, notifications, and remote parent access.

Google, Apple, Microsoft, Bark, Qustodio, Kaspersky, FamilyTime, and FamiSafe
expose location or device-status features. Parents expect this category.

## Current Ocentra State

- A capability guide exists.
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

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
