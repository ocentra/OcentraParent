# Tracking Control Settings Inventory

Generated from `BaselineTrackingControlCatalog`.
Total settings: 338

Use this as the raw review list for deciding parent-facing grouping.

## Tab: rules

### Location management

#### Location management

1.  Enable device location features?

- settingId: `location.enabled`
- policyLane: `rules`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 148
- acceptedOptions: Enabled | Disabled
- helperText: needs-effect-wiring via location-capability-registry

2.  What location posture should this device use?

- settingId: `location.defaultPosture`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 172
- acceptedOptions: Off | Last known | Check-in | Arrival alerts | Temporary live | Missing device
- helperText: needs-effect-wiring via location-capability-registry

3.  Where should location behavior execute?

- settingId: `location.executionMode`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 215
- acceptedOptions: Local Child Agent | Lan Live | Authenticated Relay | Authoring Only | Unavailable
- helperText: needs-effect-wiring via location-capability-registry

### Core Terms

#### Location Capability State

4.  `service-disabled`

- settingId: `tracking-guide-core-terms-location-capability-state-047`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 116
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

5.  `manual-required`

- settingId: `tracking-guide-core-terms-location-capability-state-049`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 118
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

6.  `offline-last-known-only`

- settingId: `tracking-guide-core-terms-location-capability-state-050`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 119
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-capability-registry

7.  `battery-throttled`

- settingId: `tracking-guide-core-terms-location-capability-state-051`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 120
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### The Main Capability Truth

#### The Main Capability Truth

8.  Battery saver, low-power modes, radio state, and network reachability can degrade tracking.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-059`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 145
- acceptedOptions: Battery Saver | Low Power Modes | Radio State | Network Reachability Can Degrade Tracking
- helperText: degraded via background-location-permission-and-disclosure-proof

### Device Online, Offline, And Battery State

#### Device Online, Offline, And Battery State

9.  Last agent heartbeat.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-156`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 350
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

10. Last location sample.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-157`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 351
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

11. Last successful parent sync.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-158`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 352
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

12. Battery percentage and charging state where available.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-159`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 353
- acceptedOptions: Battery Percentage | Charging State Where Available
- helperText: degraded via background-location-permission-and-disclosure-proof

13. Low-power/battery-saver state where available.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-160`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 354
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

14. Pending upload count.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-163`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 357
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### Policy Modes To Represent Later In UI

#### Missing Device Mode

15. Parent marks device missing.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-296`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 667
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

16. Device contact, battery, and network state become prominent.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-298`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 669
- acceptedOptions: Device Contact | Battery | Network State Become Prominent
- helperText: degraded via background-location-permission-and-disclosure-proof

17. Tracking a powered-off or offline device.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-301`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 678
- acceptedOptions: Tracking A Powered Off | Offline Device
- helperText: degraded via location-capability-registry

### Future UI Rules

#### Future UI Rules

18. Show degraded/offline/manual-required states instead of disabled-looking mystery failures.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-330`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 739
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

19. last-known only;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-332`
- policyLane: `rules`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 747
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

20. missing device mode;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-336`
- policyLane: `rules`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 751
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: evidence

### Core Terms

#### Device Location Evidence

21. Latitude, longitude, altitude, heading, speed, and bearing where exposed.

- settingId: `tracking-guide-core-terms-device-location-evidence-025`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 28
- acceptedOptions: Latitude | Longitude | Altitude | Heading | Speed | Bearing Where Exposed
- helperText: needs-effect-wiring via location-capability-registry

22. Horizontal and vertical accuracy.

- settingId: `tracking-guide-core-terms-device-location-evidence-026`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 29
- acceptedOptions: Horizontal | Vertical Accuracy
- helperText: already-represented via precision-permission-and-accuracy-proof

23. Provider kind: GPS/GNSS, Wi-Fi, cellular, IP, Bluetooth/beacon, fused provider, user-entered default, or unknown.

- settingId: `tracking-guide-core-terms-device-location-evidence-028`
- policyLane: `evidence`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 31
- acceptedOptions: Gps Gnss | Wi Fi | Cellular | Ip | Bluetooth Beacon | Fused Provider | User Entered Default | Unknown
- helperText: manual-required via location-provider-freshness-accuracy-proof

#### Location Capability State

24. `adapter-error`

- settingId: `tracking-guide-core-terms-location-capability-state-052`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 121
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Policy Modes To Represent Later In UI

#### Missing Device Mode

25. Clear separation between Ocentra evidence and OS account features.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-300`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 674
- acceptedOptions: Clear Separation Between Ocentra Evidence | Os Account Features
- helperText: needs-effect-wiring via location-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

26. Raw child activity evidence is local-first by default.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-311`
- policyLane: `evidence`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 707
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

27. [`docs/expectations/policy.md`](expectations/policy.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-319`
- policyLane: `evidence`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 723
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

## Tab: live

### Live tracking

#### Live tracking

28. When can live tracking run?

- settingId: `live.mode`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 301
- acceptedOptions: Disabled | Parent Started Temporary | During Active Trip | During Missing Device | During Alert Investigation
- helperText: proof-required via location-provider-freshness-accuracy-proof

29. What is the maximum live session duration?

- settingId: `live.maxSessionMinutes`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 315
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

30. How often should live tracking request updates?

- settingId: `live.updateCadence`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 324
- acceptedOptions: One Shot | On Change | Battery Balanced | High Accuracy Burst | Manual Refresh Only
- helperText: proof-required via location-provider-freshness-accuracy-proof

31. What should happen when battery is low?

- settingId: `live.whenBatteryLow`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 332
- acceptedOptions: Continue | Reduce Cadence | Last Known Only | Ask Parent | Stop Live Session
- helperText: degraded via location-provider-freshness-accuracy-proof

### Core Terms

#### Check-In

32. The parent wants low-noise confirmation instead of a live map.

- settingId: `tracking-guide-core-terms-check-in-040`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 91
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

### Capability Matrix

#### Capability Matrix

33. Capability matrix row | Capability=Foreground live tracking | Mobile child agent=Yes, while app/session active | Desktop/laptop child agent=Sometimes, while app/service active | Required layer=OS location API | Important limit=Requires visible use or active session semantics.

- settingId: `tracking-guide-capability-matrix-capability-matrix-064`
- policyLane: `live`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 157
- acceptedOptions: Capability Foreground Live Tracking | Mobile Child Agent Yes While App Session Active | Desktop Laptop Child Agent Sometimes While App Service Active | Required Layer Os Location Api | Important Limit Requires Visible Use Or Active Session Semantics
- helperText: permission-required via location-provider-freshness-accuracy-proof

34. Capability matrix row | Capability=Background live tracking | Mobile child agent=Platform-dependent | Desktop/laptop child agent=Limited | Required layer=Background execution plus permission | Important limit=Throttled, battery-sensitive, and often entitlement-bound.

- settingId: `tracking-guide-capability-matrix-capability-matrix-065`
- policyLane: `live`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 158
- acceptedOptions: Capability Background Live Tracking | Mobile Child Agent Platform Dependent | Desktop Laptop Child Agent Limited | Required Layer Background Execution Plus Permission | Important Limit Throttled Battery Sensitive And Often Entitlement Bound
- helperText: permission-required via location-provider-freshness-accuracy-proof

### Live Tracking: What Is Possible

#### Live Tracking: What Is Possible

35. Parent opens live map.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-076`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 177
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

36. Child is travelling between known places.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-077`
- policyLane: `live`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 178
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

37. Child missed an expected arrival.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-078`
- policyLane: `live`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 179
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

38. Device is marked missing.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-079`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 180
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

39. Parent explicitly starts a temporary safety session.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-080`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 181
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

40. A policy rule asks for short-term verification after a geofence miss.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-081`
- policyLane: `live`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 182
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

41. Session id, child id, device id, requester, and reason code.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-082`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 186
- acceptedOptions: Session Id | Child Id | Device Id | Requester | Reason Code
- helperText: proof-required via location-provider-freshness-accuracy-proof

42. Requested accuracy: approximate, precise, or best-available.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-083`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 187
- acceptedOptions: Approximate | Precise | Best Available
- helperText: permission-required via location-provider-freshness-accuracy-proof

43. Requested cadence: one-shot, on-change, interval, high-accuracy burst, or geofence-only.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-084`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 188
- acceptedOptions: One Shot | On Change | Interval | High Accuracy Burst | Geofence Only
- helperText: proof-required via location-provider-freshness-accuracy-proof

44. Maximum duration and auto-stop reason.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-085`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 190
- acceptedOptions: Maximum Duration | Auto Stop Reason
- helperText: proof-required via location-provider-freshness-accuracy-proof

45. Permission requirement and user-visible disclosure state.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-086`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 191
- acceptedOptions: Permission Requirement | User Visible Disclosure State
- helperText: permission-required via location-provider-freshness-accuracy-proof

46. Delivery path: local, LAN, authenticated relay, parent cache, or parent-owned storage.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-087`
- policyLane: `live`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 192
- acceptedOptions: Local | Lan | Authenticated Relay | Parent Cache | Parent Owned Storage
- helperText: future-gap via location-provider-freshness-accuracy-proof

47. Audit events for start, update, degrade, stop, and parent reveal.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-088`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 194
- acceptedOptions: Audit Events For Start | Update | Degrade | Stop | Parent Reveal
- helperText: degraded via location-provider-freshness-accuracy-proof

48. OSes may throttle background updates.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-089`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 198
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

49. Apps may need foreground service notification, background mode, entitlement, device-owner state, MDM supervision, or explicit user permission.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-090`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 199
- acceptedOptions: Apps May Need Foreground Service Notification | Background Mode | Entitlement | Device Owner State | Mdm Supervision | Explicit User Permission
- helperText: future-gap via location-provider-freshness-accuracy-proof

50. High accuracy increases battery use.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-091`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 201
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

51. Indoor GPS may fail or fall back to Wi-Fi/cell/IP estimates.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-092`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 202
- acceptedOptions: Indoor Gps May Fail | Fall Back To Wi Fi Cell Ip Estimates
- helperText: degraded via location-provider-freshness-accuracy-proof

52. A child can turn off device location services, revoke permission, uninstall the app where allowed, power off the device, or lose network.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-093`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 203
- acceptedOptions: A Child Can Turn Off Device Location Services | Revoke Permission | Uninstall The App Where Allowed | Power Off The Device | Lose Network
- helperText: permission-required via location-provider-freshness-accuracy-proof

53. Live map updates should never imply that a stale point is still current.

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-094`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 205
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

### Device Online, Offline, And Battery State

#### Device Online, Offline, And Battery State

54. Low battery: reduce cadence and explain degraded state.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-167`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 364
- acceptedOptions: Reduce Cadence | Explain Degraded State
- helperText: degraded via background-location-permission-and-disclosure-proof

### Child-Facing Disclosure

#### Child-Facing Disclosure

55. Live map temporarily active.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-203`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 434
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-provider-freshness-accuracy-proof

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

56. Battery throttled -> reduce cadence and record degraded state.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-212`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 454
- acceptedOptions: Battery Throttled Reduce Cadence | Record Degraded State
- helperText: degraded via background-location-permission-and-disclosure-proof

### Platform Capability Notes

#### Windows

57. Continuous tracking has battery impact and should be cadence-limited.

- settingId: `tracking-guide-platform-capability-notes-windows-223`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 486
- acceptedOptions: Continuous Tracking Has Battery Impact | Should Be Cadence Limited
- helperText: degraded via background-location-permission-and-disclosure-proof

### Policy Modes To Represent Later In UI

#### Temporary Live Map

58. Parent starts a time-limited live tracking session.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-288`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 648
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

59. Agent sends repeated updates while online and permitted.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-289`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 649
- acceptedOptions: Agent Sends Repeated Updates While Online | Permitted
- helperText: already-represented via location-provider-freshness-accuracy-proof

60. Session has a visible reason, duration, and audit trail.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-290`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 650
- acceptedOptions: Session Has A Visible Reason | Duration | Audit Trail
- helperText: already-represented via location-provider-freshness-accuracy-proof

61. Location permission.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-291`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 654
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

62. Runtime update path.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-292`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 655
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

63. Local/LAN/relay delivery path.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-293`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 656
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

64. Battery-aware cadence.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-294`
- policyLane: `live`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 657
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

65. High sensitivity and battery impact.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-295`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 661
- acceptedOptions: High Sensitivity | Battery Impact
- helperText: degraded via location-provider-freshness-accuracy-proof

### Future UI Rules

#### Future UI Rules

66. Show live tracking only when a fresh update path is available.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-321`
- policyLane: `live`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 730
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

67. Show child-facing disclosure mode when background/live tracking is enabled.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-329`
- policyLane: `live`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 738
- acceptedOptions: Enabled | Disabled
- helperText: future-gap via location-provider-freshness-accuracy-proof

68. temporary live map;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-335`
- policyLane: `live`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 750
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

## Tab: places

### Places and geofences

#### Places and geofences

69. Enable parent-defined places?

- settingId: `places.enabled`
- policyLane: `places`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 430
- acceptedOptions: Enabled | Disabled
- helperText: proof-required via geofence-region-schedule-transition-proof

70. What minimum radius should place geofences use?

- settingId: `places.minimumRadiusMeters`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 437
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

71. What if geofence monitoring is unavailable?

- settingId: `geofences.whenUnavailable`
- policyLane: `places`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 446
- acceptedOptions: Fallback To Check In | Fallback To Sampled Location | Report Unavailable | Ask Parent | Disable Geofence Rules
- helperText: proof-required via geofence-region-schedule-transition-proof

72. Which geofence transitions should be used?

- settingId: `geofences.transitionTypes`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 460
- acceptedOptions: Enter | Exit | Dwell | Missed Arrival | Stale At Place
- helperText: proof-required via geofence-region-schedule-transition-proof

### Core Terms

#### Location History

73. Arrival and departure audit.

- settingId: `tracking-guide-core-terms-location-history-033`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 64
- acceptedOptions: Arrival | Departure Audit
- helperText: proof-required via geofence-region-schedule-transition-proof

### The Main Capability Truth

#### The Main Capability Truth

74. Geofence alerts are not instant and may be throttled.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-057`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 143
- acceptedOptions: Geofence Alerts Are Not Instant | May Be Throttled
- helperText: degraded via background-location-permission-and-disclosure-proof

### Capability Matrix

#### Capability Matrix

75. Capability matrix row | Capability=Geofence enter/exit | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak or app-running-only on desktop | Required layer=OS geofence/region monitoring | Important limit=Delayed events, radius limits, count limits, false exits.

- settingId: `tracking-guide-capability-matrix-capability-matrix-066`
- policyLane: `places`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 159
- acceptedOptions: Capability Geofence Enter Exit | Mobile Child Agent Android Ios With Limits | Desktop Laptop Child Agent Weak Or App Running Only On Desktop | Required Layer Os Geofence Region Monitoring | Important Limit Delayed Events Radius Limits Count Limits False Exits
- helperText: future-gap via geofence-region-schedule-transition-proof

76. Capability matrix row | Capability=Dwell alerts | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak | Required layer=Geofence plus dwell support | Important limit=Not immediate; large radius may be required.

- settingId: `tracking-guide-capability-matrix-capability-matrix-067`
- policyLane: `places`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 160
- acceptedOptions: Capability Dwell Alerts | Mobile Child Agent Android Ios With Limits | Desktop Laptop Child Agent Weak | Required Layer Geofence Plus Dwell Support | Important Limit Not Immediate Large Radius May Be Required
- helperText: future-gap via geofence-region-schedule-transition-proof

77. Capability matrix row | Capability=Parent arrival/departure alerts | Mobile child agent=Yes, with geofence/check-in evidence | Desktop/laptop child agent=Limited | Required layer=Geofence or sampled evidence | Important limit=Notifications must minimize sensitive detail.

- settingId: `tracking-guide-capability-matrix-capability-matrix-071`
- policyLane: `places`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 164
- acceptedOptions: Capability Parent Arrival Departure Alerts | Mobile Child Agent Yes With Geofence Check In Evidence | Desktop Laptop Child Agent Limited | Required Layer Geofence Or Sampled Evidence | Important Limit Notifications Must Minimize Sensitive Detail
- helperText: degraded via geofence-region-schedule-transition-proof

### Location History: What Is Possible

#### Location History: What Is Possible

78. `location-summary`: derived day/trip/place summary with redacted detail.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-096`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 214
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

79. `geofence-transition`: arrival, departure, dwell, missed arrival, or stale state.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-097`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 215
- acceptedOptions: Arrival | Departure | Dwell | Missed Arrival | Stale State
- helperText: degraded via geofence-region-schedule-transition-proof

80. Place/geofence audit: medium retention.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-102`
- policyLane: `places`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 224
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### Geofences

#### Geofences

81. Arrived at school during a schedule.

- settingId: `tracking-guide-geofences-geofences-136`
- policyLane: `places`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 295
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

82. Left school before dismissal.

- settingId: `tracking-guide-geofences-geofences-137`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 296
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

83. Did not arrive at practice by a time.

- settingId: `tracking-guide-geofences-geofences-138`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 297
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

84. Stayed near home after bedtime.

- settingId: `tracking-guide-geofences-geofences-139`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 298
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

85. Notify if device leaves a travel corridor.

- settingId: `tracking-guide-geofences-geofences-140`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 299
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

86. Region id, label token, latitude, longitude, radius, schedule, transition type, dwell duration, and expiration.

- settingId: `tracking-guide-geofences-geofences-141`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `multi`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 303
- acceptedOptions: Region Id | Label Token | Latitude | Longitude | Radius | Schedule | Transition Type | Dwell Duration | Expiration
- helperText: proof-required via geofence-region-schedule-transition-proof

87. Minimum radius and maximum count per platform.

- settingId: `tracking-guide-geofences-geofences-142`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 305
- acceptedOptions: Minimum Radius | Maximum Count Per Platform
- helperText: proof-required via geofence-region-schedule-transition-proof

88. Proof requirement: platform geofence, sampled location, check-in, or manual.

- settingId: `tracking-guide-geofences-geofences-143`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 306
- acceptedOptions: Platform Geofence | Sampled Location | Check In | Manual
- helperText: manual-required via geofence-region-schedule-transition-proof

89. Fallback when geofence monitoring is unavailable.

- settingId: `tracking-guide-geofences-geofences-144`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 307
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

90. Debounce/noise control to avoid alert storms.

- settingId: `tracking-guide-geofences-geofences-145`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 308
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

91. Geofence events can be delayed.

- settingId: `tracking-guide-geofences-geofences-146`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 312
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

92. Small radii can be unreliable.

- settingId: `tracking-guide-geofences-geofences-147`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 313
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

93. Wi-Fi, cell, and GPS availability affect transition quality.

- settingId: `tracking-guide-geofences-geofences-148`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 314
- acceptedOptions: Wi Fi | Cell | Gps Availability Affect Transition Quality
- helperText: proof-required via location-provider-freshness-accuracy-proof

94. Some platforms wake apps for geofence events; others only work while the app is running.

- settingId: `tracking-guide-geofences-geofences-149`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 315
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

95. Geofence count limits require compile-time validation.

- settingId: `tracking-guide-geofences-geofences-150`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 317
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

96. Dwell events are useful to reduce alert noise but can delay notifications.

- settingId: `tracking-guide-geofences-geofences-151`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 318
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### Reports And Maps

#### Reports And Maps

97. Visualize accuracy radius when useful.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-171`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 375
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

98. Distinguish live, last-known, check-in, geofence, and manual/default points.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-173`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 377
- acceptedOptions: Distinguish Live | Last Known | Check In | Geofence | Manual Default Points
- helperText: manual-required via location-provider-freshness-accuracy-proof

99. Arrivals/departures by place.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-178`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 385
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

100.  Missed arrival/departure alerts.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-179`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 386
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### Child-Facing Disclosure

#### Child-Facing Disclosure

101.  Arrival/departure alerts enabled.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-202`
- policyLane: `places`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 433
- acceptedOptions: Enabled | Disabled
- helperText: manual-required via geofence-region-schedule-transition-proof

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

102.  Geofence unavailable -> use scheduled check-in or sampled location if allowed.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-209`
- policyLane: `places`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 450
- acceptedOptions: Geofence Unavailable Use Scheduled Check In | Sampled Location If Allowed
- helperText: proof-required via geofence-region-schedule-transition-proof

### Platform Capability Notes

#### macOS

103.  Region monitoring only while the app is running and the system is awake, according to Apple geofence documentation.

- settingId: `tracking-guide-platform-capability-notes-macos-227`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 497
- acceptedOptions: Region Monitoring Only While The App Is Running | The System Is Awake | According To Apple Geofence Documentation
- helperText: proof-required via geofence-region-schedule-transition-proof

#### Android

104.  Geofencing API with platform count, permission, delay, and radius limits.

- settingId: `tracking-guide-platform-capability-notes-android-245`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 544
- acceptedOptions: Geofencing Api With Platform Count | Permission | Delay | Radius Limits
- helperText: permission-required via geofence-region-schedule-transition-proof

### Policy Modes To Represent Later In UI

#### Observe Last Known Location

105.  Geofence setup.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-269`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 599
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

106.  Arrival/departure alerts unless separately enabled.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-272`
- policyLane: `places`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 605
- acceptedOptions: Enabled | Disabled
- helperText: proof-required via geofence-region-schedule-transition-proof

#### Arrival And Departure Alerts

107.  Parent defines places and schedules.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-280`
- policyLane: `places`; cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 629
- acceptedOptions: Parent Defines Places | Schedules
- helperText: proof-required via geofence-region-schedule-transition-proof

108.  Agent records geofence or sampled evidence.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-281`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 630
- acceptedOptions: Agent Records Geofence | Sampled Evidence
- helperText: proof-required via geofence-region-schedule-transition-proof

109.  Notifications are sent only through alert rules.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-282`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 631
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

110.  Geofence or sampled-location capability.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-283`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 635
- acceptedOptions: Geofence | Sampled Location Capability
- helperText: proof-required via geofence-region-schedule-transition-proof

111.  Debounce/noise controls.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-284`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 636
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

112.  Custody and retention settings.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-285`
- policyLane: `places`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 637
- acceptedOptions: Custody | Retention Settings
- helperText: proof-required via geofence-region-schedule-transition-proof

113.  Instant transitions.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-286`
- policyLane: `places`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 641
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

114.  Small-radius precision.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-287`
- policyLane: `places`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 642
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

#### Location-Based Policy Decision

115.  Location evidence can contribute to local policy, such as "ask if leaving school early" or "notify if not at practice by 18:00".

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-302`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 684
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

### Future UI Rules

#### Future UI Rules

116.  Show exact coordinate reveal separately from summary/place reporting.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-323`
- policyLane: `places`; cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 732
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

117.  Show geofence alerts as delayed/coarse arrival/departure evidence.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-325`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 734
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via geofence-region-schedule-transition-proof

118.  geofence arrival/departure;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-334`
- policyLane: `places`; cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 749
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via geofence-region-schedule-transition-proof

## Tab: approvals

### Check-ins

#### Check-ins

119.  How should check-ins work?

- settingId: `checkIns.mode`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 386
- acceptedOptions: Disabled | Parent Requested | Scheduled | Geofence Miss | Policy Triggered
- helperText: needs-effect-wiring via location-capability-registry

120.  Should check-ins include location?

- settingId: `checkIns.includeLocation`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 394
- acceptedOptions: Never | When Permitted | Require Current Location | Allow Child Choice
- helperText: needs-effect-wiring via location-capability-registry

121.  When is a check-in unanswered?

- settingId: `checkIns.unansweredAfterMinutes`
- policyLane: `approvals`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 402
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

122.  Which child responses are allowed?

- settingId: `checkIns.allowedResponses`
- policyLane: `approvals`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 411
- acceptedOptions: Safe | Arriving | Leaving | Delayed | Need Help | Call Me | Custom Note
- helperText: needs-effect-wiring via location-capability-registry

### Alerts

#### Alerts

123.  Which location events should notify a parent?

- settingId: `alerts.enabledReasons`
- policyLane: `approvals`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 479
- acceptedOptions: Arrival | Departure | Early Departure | Missed Arrival | Unanswered Check In | Need Help Check In | Device Offline During Trip | Location Permission Lost | Live Session Started | Missing Device Found
- helperText: needs-effect-wiring via location-capability-registry

124.  What location detail may appear in push/email/SMS bodies?

- settingId: `alerts.sensitiveDetailsInProviderBody`
- policyLane: `approvals`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 498
- acceptedOptions: None | Minimal | Place Label Only | Approximate Area | Exact Coordinate
- helperText: needs-effect-wiring via location-capability-registry

### Core Terms

#### Check-In

125.  The child device is on a constrained platform.

- settingId: `tracking-guide-core-terms-check-in-039`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 90
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

126.  The agent can send an ask/confirm prompt without tracking movement all day.

- settingId: `tracking-guide-core-terms-check-in-041`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 92
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Check-In And Safety Prompts

#### Check-In And Safety Prompts

127.  A check-in response without a fresh coordinate is still useful.

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-152`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 338
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

128.  A fresh coordinate without child response should be labeled as location-only.

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-153`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 339
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

129.  Unanswered check-ins should produce a notification intent only through an explicit alert rule.

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-154`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 340
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

130.  Sensitive child notes should not appear in third-party notification previews.

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-155`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 342
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Policy Modes To Represent Later In UI

#### Check-In Only

131.  Parent can request a child response.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-273`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 611
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

132.  Unanswered check-ins can drive explicit notification rules.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-275`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 613
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

133.  Child response.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-278`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 622
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

134.  Remote relay and notifications must minimize child details.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-312`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 708
- acceptedOptions: Remote Relay | Notifications Must Minimize Child Details
- helperText: future-gap via authenticated-relay-proof-without-default-location-history-storage

135.  [`docs/expectations/notifications.md`](expectations/notifications.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-320`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 724
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Future UI Rules

#### Future UI Rules

136.  Show check-in as a separate workflow from tracking.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-326`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 735
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

137.  check-in only;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-333`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 748
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

138.  location-based policy alerts;

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-337`
- policyLane: `approvals`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 752
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: enforcement

### Accuracy Sources And Limits

#### Accuracy Sources And Limits

139.  GPS/GNSS: best outdoors, weaker indoors, battery-sensitive.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-120`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 263
- acceptedOptions: Best Outdoors | Weaker Indoors | Battery Sensitive
- helperText: degraded via location-provider-freshness-accuracy-proof

140.  IP address: coarse and often wrong for household, VPN, carrier-grade NAT, or corporate networks.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-123`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 267
- acceptedOptions: Coarse | Often Wrong For Household | Vpn | Carrier Grade Nat | Corporate Networks
- helperText: already-represented via precision-permission-and-accuracy-proof

141.  Bluetooth/beacon: local proximity, not global location.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-124`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 269
- acceptedOptions: Local Proximity | Not Global Location
- helperText: already-represented via precision-permission-and-accuracy-proof

142.  Manual/default location: a fallback, not current device proof.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-125`
- policyLane: `enforcement`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 270
- acceptedOptions: A Fallback | Not Current Device Proof
- helperText: manual-required via precision-permission-and-accuracy-proof

143.  Fused provider: OS/provider chooses from several sources.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-126`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 271
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

144.  `accuracyMeters`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-127`
- policyLane: `enforcement`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 275
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

145.  `altitudeAccuracyMeters` when known

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-128`
- policyLane: `enforcement`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 276
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

146.  `sourceKinds`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-129`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 277
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

147.  `confidence`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-131`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 279
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

148.  `isUserEnteredDefault`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-134`
- policyLane: `enforcement`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 282
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

149.  `isSimulatedOrDeveloperMode` if detectable

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-135`
- policyLane: `enforcement`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 283
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

150.  Device offline -> show last contact and queue parent request.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-211`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 453
- acceptedOptions: Device Offline Show Last Contact | Queue Parent Request
- helperText: degraded via location-capability-registry

151.  Parent relay unavailable -> continue local policy and send when reachable.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-214`
- policyLane: `enforcement`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 456
- acceptedOptions: Parent Relay Unavailable Continue Local Policy | Send When Reachable
- helperText: future-gap via authenticated-relay-proof-without-default-location-history-storage

### Policy Modes To Represent Later In UI

#### Location-Based Policy Decision

152.  Typed policy target.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-303`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 689
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

153.  Explicit fallback when proof is stale or unavailable.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-305`
- policyLane: `enforcement`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 691
- acceptedOptions: Explicit Fallback When Proof Is Stale | Unavailable
- helperText: degraded via location-capability-registry

154.  Local child-agent evaluation.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-306`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 692
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

155.  Portal-side policy evaluation.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-307`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 696
- acceptedOptions: Represented | Not Represented
- helperText: unavailable via location-capability-registry

156.  Guessing current location from stale last-known evidence.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-308`
- policyLane: `enforcement`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `unavailable`; runtimeOwner: `child-agent`; capabilityState: `unavailable`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 697
- acceptedOptions: Represented | Not Represented
- helperText: unavailable via location-provider-freshness-accuracy-proof

## Tab: reports

### Last known location

#### Last known location

157.  Show last known location on the map?

- settingId: `lastKnown.showOnMap`
- policyLane: `reports`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 351
- acceptedOptions: Enabled | Disabled
- helperText: already-represented via location-capability-registry

158.  When should a point become stale?

- settingId: `lastKnown.staleAfterMinutes`
- policyLane: `reports`; cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 358
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-capability-registry

159.  What should the UI show when location is stale?

- settingId: `lastKnown.whenStale`
- policyLane: `reports`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 367
- acceptedOptions: Hide Point | Show Stale | Show Stale With Contact State | Ask Check In | Notify Parent
- helperText: degraded via location-capability-registry

### The Main Capability Truth

#### The Main Capability Truth

160.  Last known location is not proof of current location.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-056`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 142
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

161.  Offline devices can only report last known location and last contact time.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-058`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 144
- acceptedOptions: Offline Devices Can Only Report Last Known Location | Last Contact Time
- helperText: degraded via location-capability-registry

### Capability Matrix

#### Capability Matrix

162.  Capability matrix row | Capability=Family map | Mobile child agent=Yes from latest evidence | Desktop/laptop child agent=Yes from latest evidence | Required layer=Query/read model plus map rendering | Important limit=Map must label freshness and accuracy.

- settingId: `tracking-guide-capability-matrix-capability-matrix-070`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 163
- acceptedOptions: Capability Family Map | Mobile Child Agent Yes From Latest Evidence | Desktop Laptop Child Agent Yes From Latest Evidence | Required Layer Query Read Model Plus Map Rendering | Important Limit Map Must Label Freshness And Accuracy
- helperText: already-represented via precision-permission-and-accuracy-proof

163.  Capability matrix row | Capability=Enforce location-based policy | Mobile child agent=Possible after proof | Desktop/laptop child agent=Limited | Required layer=Local policy plus location evidence | Important limit=Missing proof must degrade to ask/warn/report.

- settingId: `tracking-guide-capability-matrix-capability-matrix-075`
- policyLane: `reports`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 168
- acceptedOptions: Capability Enforce Location Based Policy | Mobile Child Agent Possible After Proof | Desktop Laptop Child Agent Limited | Required Layer Local Policy Plus Location Evidence | Important Limit Missing Proof Must Degrade To Ask Warn Report
- helperText: degraded via location-capability-registry

### Accuracy Sources And Limits

#### Accuracy Sources And Limits

164.  `freshnessSeconds`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-130`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 278
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### Device Online, Offline, And Battery State

#### Device Online, Offline, And Battery State

165.  Network type and reachability summary.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-161`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 355
- acceptedOptions: Network Type | Reachability Summary
- helperText: degraded via background-location-permission-and-disclosure-proof

166.  Offline: show last contact time and last known point.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-166`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 363
- acceptedOptions: Show Last Contact Time | Last Known Point
- helperText: degraded via background-location-permission-and-disclosure-proof

### Reports And Maps

#### Reports And Maps

167.  Show freshness on every point.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-170`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 374
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

168.  Let parents reveal exact coordinates only when the data scope permits it.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-174`
- policyLane: `reports`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 378
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

169.  Keep summaries useful without requiring raw trail exposure.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-175`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 379
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

170.  Recent location status by child/device.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-177`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 384
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

171.  Check-in timeline.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-180`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 387
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

172.  Device offline and battery timeline.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-181`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 388
- acceptedOptions: Device Offline | Battery Timeline
- helperText: degraded via background-location-permission-and-disclosure-proof

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

173.  Current location unavailable -> show last known location with timestamp.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-207`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 448
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

### Platform Capability Notes

#### macOS

174.  Do not assume Windows service behavior maps to macOS.

- settingId: `tracking-guide-platform-capability-notes-macos-231`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 507
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

#### Android

175.  Fused Location Provider for last known, current, and periodic updates.

- settingId: `tracking-guide-platform-capability-notes-android-244`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 543
- acceptedOptions: Fused Location Provider For Last Known | Current | Periodic Updates
- helperText: needs-effect-wiring via location-capability-registry

### Policy Modes To Represent Later In UI

#### Observe Last Known Location

176.  Show newest location evidence when available.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-265`
- policyLane: `reports`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 592
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

177.  Do not run continuous tracking.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-267`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 594
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

178.  Live relay.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-270`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 600
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

179.  Real-time movement.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-271`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 604
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

#### Missing Device Mode

180.  Agent tries to provide current or last known location.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-297`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 668
- acceptedOptions: Agent Tries To Provide Current | Last Known Location
- helperText: needs-effect-wiring via location-capability-registry

#### Location-Based Policy Decision

181.  Evidence freshness and accuracy thresholds.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-304`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 690
- acceptedOptions: Evidence Freshness | Accuracy Thresholds
- helperText: proof-required via precision-permission-and-accuracy-proof

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

182.  Parent surfaces author rules and view reports; they do not execute child capture or policy.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-310`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 705
- acceptedOptions: Parent Surfaces Author Rules | View Reports They Do Not Execute Child Capture | Policy
- helperText: already-represented via location-capability-registry

183.  [`docs/product-roadmap.md`](product-roadmap.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-315`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 719
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

### Future UI Rules

#### Future UI Rules

184.  Show last known location separately from current/live location.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-322`
- policyLane: `reports`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 731
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

## Tab: setup

### Permission and disclosure

#### Permission and disclosure

185.  What location permission is required?

- settingId: `permissions.minimumPermission`
- policyLane: `setup`; cardKind: `single-choice-many`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 238
- acceptedOptions: None | Foreground Approximate | Foreground Precise | Background Approximate | Background Precise | Supervised Or Device Owner | Platform Managed Lost Mode
- helperText: permission-required via location-capability-registry

186.  What should happen if permission is missing?

- settingId: `permissions.whenPermissionMissing`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 254
- acceptedOptions: Show Setup Required | Fallback To Check In | Fallback To Last Known | Report Unavailable | Ask Parent | Disable Location Features
- helperText: permission-required via location-capability-registry

187.  Allow approximate location when precise is not granted?

- settingId: `permissions.allowApproximateFallback`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 269
- acceptedOptions: Enabled | Disabled
- helperText: permission-required via precision-permission-and-accuracy-proof

188.  What should the child device disclose?

- settingId: `permissions.childDisclosure`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-schema-proposal.md`; sourceLine: 276
- acceptedOptions: None | Show Enabled | Show Mode And Last Sample | Show Live Session Active | Show Background Tracking Active
- helperText: permission-required via location-capability-registry

### Core Terms

#### Device Location Evidence

189.  Permission state: denied, foreground-only, background, approximate, precise, reduced-accuracy, supervised/managed, or unknown.

- settingId: `tracking-guide-core-terms-device-location-evidence-029`
- policyLane: `setup`; cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 33
- acceptedOptions: Denied | Foreground Only | Background | Approximate | Precise | Reduced Accuracy | Supervised Managed | Unknown
- helperText: permission-required via background-location-permission-and-disclosure-proof

190.  Device state: online, offline, low power, battery saver, airplane mode, no signal, service disabled, or adapter unavailable.

- settingId: `tracking-guide-core-terms-device-location-evidence-030`
- policyLane: `setup`; cardKind: `single-choice-many`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 35
- acceptedOptions: Online | Offline | Low Power | Battery Saver | Airplane Mode | No Signal | Service Disabled | Adapter Unavailable
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### Check-In

191.  Background location is unavailable or not appropriate.

- settingId: `tracking-guide-core-terms-check-in-038`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 89
- acceptedOptions: Background Location Is Unavailable | Not Appropriate
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### Location Capability State

192.  `ready-precise-background`

- settingId: `tracking-guide-core-terms-location-capability-state-042`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 111
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

193.  `ready-foreground-only`

- settingId: `tracking-guide-core-terms-location-capability-state-043`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 112
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

194.  `ready-approximate-only`

- settingId: `tracking-guide-core-terms-location-capability-state-044`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 113
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

195.  `permission-required`

- settingId: `tracking-guide-core-terms-location-capability-state-045`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 114
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

196.  `background-permission-required`

- settingId: `tracking-guide-core-terms-location-capability-state-046`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 115
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

### The Main Capability Truth

#### The Main Capability Truth

197.  Location permission is user-visible and revocable.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-053`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 138
- acceptedOptions: Location Permission Is User Visible | Revocable
- helperText: permission-required via location-capability-registry

198.  Background location is a separate capability from foreground location.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-054`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 139
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

199.  Approximate/reduced accuracy must be represented separately from precise location.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-055`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 140
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

### Capability Matrix

#### Capability Matrix

200.  Capability matrix row | Capability=One-time current location | Mobile child agent=Yes, with permission | Desktop/laptop child agent=Sometimes, with permission | Required layer=OS location API | Important limit=Fresh fix may fail indoors, offline, or with service off.

- settingId: `tracking-guide-capability-matrix-capability-matrix-062`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 155
- acceptedOptions: Capability One Time Current Location | Mobile Child Agent Yes With Permission | Desktop Laptop Child Agent Sometimes With Permission | Required Layer Os Location Api | Important Limit Fresh Fix May Fail Indoors Offline Or With Service Off
- helperText: permission-required via location-provider-freshness-accuracy-proof

201.  Capability matrix row | Capability=Last known location | Mobile child agent=Yes, if provider cache exists | Desktop/laptop child agent=Sometimes | Required layer=OS location API or local cache | Important limit=May be stale, null, approximate, or user-entered.

- settingId: `tracking-guide-capability-matrix-capability-matrix-063`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 156
- acceptedOptions: Capability Last Known Location | Mobile Child Agent Yes If Provider Cache Exists | Desktop Laptop Child Agent Sometimes | Required Layer Os Location Api Or Local Cache | Important Limit May Be Stale Null Approximate Or User Entered
- helperText: permission-required via precision-permission-and-accuracy-proof

202.  Capability matrix row | Capability=Check-in with current location | Mobile child agent=Yes, with prompt and permission | Desktop/laptop child agent=Yes, if current fix available | Required layer=Notification/prompt plus location API | Important limit=Child must respond unless automatic policy exists.

- settingId: `tracking-guide-capability-matrix-capability-matrix-068`
- policyLane: `setup`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 161
- acceptedOptions: Capability Check In With Current Location | Mobile Child Agent Yes With Prompt And Permission | Desktop Laptop Child Agent Yes If Current Fix Available | Required Layer Notification Prompt Plus Location Api | Important Limit Child Must Respond Unless Automatic Policy Exists
- helperText: permission-required via location-provider-freshness-accuracy-proof

### Location History: What Is Possible

#### Location History: What Is Possible

203.  Raw precise trail: disabled by default unless explicitly enabled.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-105`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 227
- acceptedOptions: Enabled | Disabled
- helperText: permission-required via precision-permission-and-accuracy-proof

204.  Accuracy, freshness, and permission state.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-107`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 232
- acceptedOptions: Accuracy | Freshness | Permission State
- helperText: permission-required via precision-permission-and-accuracy-proof

### Device Location Permissions

#### Device Location Permissions

205.  No permission.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-111`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 241
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

206.  Foreground/when-in-use permission.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-112`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 242
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

207.  Background/always permission.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-113`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 243
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

208.  Approximate or reduced-accuracy permission.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-114`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 244
- acceptedOptions: Approximate | Reduced Accuracy Permission
- helperText: permission-required via precision-permission-and-accuracy-proof

209.  Precise/full-accuracy permission.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-115`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 245
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

210.  OS location service disabled.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-116`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 246
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

211.  Device policy allowed, denied, or user-in-control.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-117`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 247
- acceptedOptions: Device Policy Allowed | Denied | User In Control
- helperText: permission-required via location-capability-registry

212.  Supervised/device-owner/MDM-only state.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-118`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 248
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

213.  Unknown or stale state.

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-119`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 249
- acceptedOptions: Unknown | Stale State
- helperText: permission-required via location-capability-registry

### Accuracy Sources And Limits

#### Accuracy Sources And Limits

214.  Wi-Fi: strong for urban/indoor approximate positioning when databases and nearby access points are available.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-121`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 264
- acceptedOptions: Strong For Urban Indoor Approximate Positioning When Databases | Nearby Access Points Are Available
- helperText: permission-required via precision-permission-and-accuracy-proof

215.  `isApproximate`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-132`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 280
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

216.  `isPrecise`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-133`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 281
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

### Device Online, Offline, And Battery State

#### Device Online, Offline, And Battery State

217.  Permission/service state.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-162`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 356
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

218.  Online with permission: show fresh or actively updating state.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-164`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 361
- acceptedOptions: Show Fresh | Actively Updating State
- helperText: permission-required via background-location-permission-and-disclosure-proof

219.  Online without permission: show permission-required state and last known point.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-165`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 362
- acceptedOptions: Show Permission Required State | Last Known Point
- helperText: permission-required via background-location-permission-and-disclosure-proof

220.  Service disabled: show location-service-disabled, not "tracking failed".

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-168`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 365
- acceptedOptions: Show Location Service Disabled | Not Tracking Failed
- helperText: permission-required via background-location-permission-and-disclosure-proof

### Reports And Maps

#### Reports And Maps

221.  Distinguish approximate from precise.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-172`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 376
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

222.  Location permission health.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-182`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 389
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

### Child-Facing Disclosure

#### Child-Facing Disclosure

223.  Location controls disabled.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-200`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 431
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

224.  Check-in only.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-201`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 432
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

225.  Background location enabled by parent and OS permission.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-204`
- policyLane: `setup`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 435
- acceptedOptions: Background Location Enabled By Parent | Os Permission
- helperText: permission-required via background-location-permission-and-disclosure-proof

226.  Last-known report only because device is offline.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-205`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 436
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

227.  Location unavailable because permission/service is off.

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-206`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 437
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

228.  Background permission missing -> offer foreground/check-in mode.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-208`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 449
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

229.  Precise permission denied -> use approximate-only rules or mark precise rules unavailable.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-210`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 451
- acceptedOptions: Precise Permission Denied Use Approximate Only Rules | Mark Precise Rules Unavailable
- helperText: permission-required via precision-permission-and-accuracy-proof

### Platform Capability Notes

#### Windows

230.  `Windows.Devices.Geolocation.Geolocator` for one-time and continuous location where the app has permission.

- settingId: `tracking-guide-platform-capability-notes-windows-215`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 471
- acceptedOptions: Windows Devices Geolocation Geolocator For One Time | Continuous Location Where The App Has Permission
- helperText: permission-required via location-capability-registry

231.  Wi-Fi BSSID access increasingly tied to precise-location consent.

- settingId: `tracking-guide-platform-capability-notes-windows-218`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 476
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

232.  Location can be approximate, IP-derived, stale, manually configured, or unavailable.

- settingId: `tracking-guide-platform-capability-notes-windows-221`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `manual-proof`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 482
- acceptedOptions: Location Can Be Approximate | Ip Derived | Stale | Manually Configured | Unavailable
- helperText: permission-required via precision-permission-and-accuracy-proof

233.  A background Windows service cannot assume Store-app-style foreground consent semantics without implementation proof.

- settingId: `tracking-guide-platform-capability-notes-windows-222`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 484
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### macOS

234.  TCC permissions, background execution, launchd behavior, signing, and notarization matter.

- settingId: `tracking-guide-platform-capability-notes-macos-230`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 505
- acceptedOptions: Tcc Permissions | Background Execution | Launchd Behavior | Signing | Notarization Matter
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### Linux

235.  IP/Wi-Fi based approximate location if the service/provider is available.

- settingId: `tracking-guide-platform-capability-notes-linux-234`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 519
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

236.  Browser geolocation with user permission for web surfaces, separate from the child agent.

- settingId: `tracking-guide-platform-capability-notes-linux-235`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 520
- acceptedOptions: Browser Geolocation With User Permission For Web Surfaces | Separate From The Child Agent
- helperText: permission-required via location-capability-registry

237.  Permission agents and desktop portals vary.

- settingId: `tracking-guide-platform-capability-notes-linux-238`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 527
- acceptedOptions: Permission Agents | Desktop Portals Vary
- helperText: permission-required via location-capability-registry

#### Android

238.  Foreground location with `ACCESS_COARSE_LOCATION` and/or `ACCESS_FINE_LOCATION`.

- settingId: `tracking-guide-platform-capability-notes-android-241`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 538
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

239.  Background location with `ACCESS_BACKGROUND_LOCATION` when core functionality and Play policy allow it.

- settingId: `tracking-guide-platform-capability-notes-android-242`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 540
- acceptedOptions: Background Location With Access Background Location When Core Functionality | Play Policy Allow It
- helperText: permission-required via background-location-permission-and-disclosure-proof

240.  Approximate vs precise permission state.

- settingId: `tracking-guide-platform-capability-notes-android-243`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 542
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

241.  Foreground service and visible notification for long-running location use where required.

- settingId: `tracking-guide-platform-capability-notes-android-246`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 545
- acceptedOptions: Foreground Service | Visible Notification For Long Running Location Use Where Required
- helperText: permission-required via background-location-permission-and-disclosure-proof

242.  Background location is restricted and must be core to the app.

- settingId: `tracking-guide-platform-capability-notes-android-248`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 552
- acceptedOptions: Background Location Is Restricted | Must Be Core To The App
- helperText: permission-required via background-location-permission-and-disclosure-proof

243.  Background updates are throttled.

- settingId: `tracking-guide-platform-capability-notes-android-249`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 553
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

244.  Approximate permission may be the only granted precision.

- settingId: `tracking-guide-platform-capability-notes-android-251`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 555
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

245.  Users can change permission, precision, and location service settings.

- settingId: `tracking-guide-platform-capability-notes-android-252`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 556
- acceptedOptions: Users Can Change Permission | Precision | Location Service Settings
- helperText: permission-required via location-capability-registry

#### iOS And iPadOS

246.  Background location with required mode, authorization, and user disclosure.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-257`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 569
- acceptedOptions: Background Location With Required Mode | Authorization | User Disclosure
- helperText: permission-required via background-location-permission-and-disclosure-proof

247.  Always/background location has explicit prompts and disclosure requirements.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-261`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 577
- acceptedOptions: Always Background Location Has Explicit Prompts | Disclosure Requirements
- helperText: permission-required via background-location-permission-and-disclosure-proof

### Policy Modes To Represent Later In UI

#### Observe Last Known Location

248.  Show freshness, accuracy, source, permission, and custody.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-266`
- policyLane: `setup`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 593
- acceptedOptions: Show Freshness | Accuracy | Source | Permission | Custody
- helperText: permission-required via precision-permission-and-accuracy-proof

249.  Background permission.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-268`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 598
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### Check-In Only

250.  Agent may include current location if permission allows.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-274`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 612
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

251.  Foreground prompt/notification.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-276`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 617
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

252.  Optional location permission.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-277`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 618
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

253.  Current location when permission/service is unavailable.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-279`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 623
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

#### Missing Device Mode

254.  Existing permission or platform-specific lost-device capability.

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-299`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 673
- acceptedOptions: Existing Permission | Platform Specific Lost Device Capability
- helperText: permission-required via location-capability-registry

### Future UI Rules

#### Future UI Rules

255.  Show approximate/reduced accuracy as a first-class state.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-324`
- policyLane: `setup`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 733
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via precision-permission-and-accuracy-proof

256.  Keep permission state close to every control.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-327`
- policyLane: `setup`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 736
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: platform

### Core Terms

#### Device Location Evidence

257.  Timestamp from the platform provider and ingest timestamp from Ocentra.

- settingId: `tracking-guide-core-terms-device-location-evidence-027`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 30
- acceptedOptions: Timestamp From The Platform Provider | Ingest Timestamp From Ocentra
- helperText: already-represented via location-capability-registry

#### Location Capability State

258.  `platform-unsupported`

- settingId: `tracking-guide-core-terms-location-capability-state-048`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 117
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### Capability Matrix

#### Capability Matrix

259.  Capability matrix row | Capability=Lost-device location | Mobile child agent=OS/product-specific | Desktop/laptop child agent=OS/product-specific | Required layer=OS lost mode/Find My/device management | Important limit=Not a generic third-party API on every platform.

- settingId: `tracking-guide-capability-matrix-capability-matrix-073`
- policyLane: `platform`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 166
- acceptedOptions: Capability Lost Device Location | Mobile Child Agent Os Product Specific | Desktop Laptop Child Agent Os Product Specific | Required Layer Os Lost Mode Find My Device Management | Important Limit Not A Generic Third Party Api On Every Platform
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

### Accuracy Sources And Limits

#### Accuracy Sources And Limits

260.  Cellular: useful wide-area estimate on mobile devices.

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-122`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 266
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### Missing-Proof Fallbacks

#### Missing-Proof Fallbacks

261.  Platform unsupported -> show unavailable/manual-required, not a fake toggle.

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-213`
- policyLane: `platform`; cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 455
- acceptedOptions: Platform Unsupported Show Unavailable Manual Required | Not A Fake Toggle
- helperText: manual-required via location-capability-registry

### Platform Capability Notes

#### Windows

262.  Windows location service using GPS, Wi-Fi, cell towers, and IP where available.

- settingId: `tracking-guide-platform-capability-notes-windows-216`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 473
- acceptedOptions: Windows Location Service Using Gps | Wi Fi | Cell Towers | Ip Where Available
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

263.  Windows privacy settings and MDM/Policy CSP for whether Windows apps may access location.

- settingId: `tracking-guide-platform-capability-notes-windows-217`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 474
- acceptedOptions: Windows Privacy Settings | Mdm Policy Csp For Whether Windows Apps May Access Location
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

264.  Child-agent service/contact state independent of location service state.

- settingId: `tracking-guide-platform-capability-notes-windows-219`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 477
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

265.  Desktop/laptop hardware may have no GPS or cellular radio.

- settingId: `tracking-guide-platform-capability-notes-windows-220`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 481
- acceptedOptions: Desktop Laptop Hardware May Have No Gps | Cellular Radio
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

266.  Product claims need real Windows adapter proof, not only a contract.

- settingId: `tracking-guide-platform-capability-notes-windows-224`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 487
- acceptedOptions: Product Claims Need Real Windows Adapter Proof | Not Only A Contract
- helperText: proof-required via location-capability-registry

#### macOS

267.  Core Location authorization and location updates.

- settingId: `tracking-guide-platform-capability-notes-macos-225`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 495
- acceptedOptions: Core Location Authorization | Location Updates
- helperText: needs-effect-wiring via location-capability-registry

268.  Reduced/full accuracy state.

- settingId: `tracking-guide-platform-capability-notes-macos-226`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 496
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

269.  Find My Mac as an Apple account feature, not a generic third-party tracking API.

- settingId: `tracking-guide-platform-capability-notes-macos-228`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 499
- acceptedOptions: Find My Mac As An Apple Account Feature | Not A Generic Third Party Tracking Api
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

270.  MDM/device management posture for managed-device features where available.

- settingId: `tracking-guide-platform-capability-notes-macos-229`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 501
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

271.  Lost-device and Find My behavior should be described as OS/account feature context unless Ocentra has an approved API and proof.

- settingId: `tracking-guide-platform-capability-notes-macos-232`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 508
- acceptedOptions: Lost Device | Proof
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

#### Linux

272.  GeoClue over D-Bus on desktops that ship/configure it.

- settingId: `tracking-guide-platform-capability-notes-linux-233`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 518
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

273.  Manual/default location fallback.

- settingId: `tracking-guide-platform-capability-notes-linux-236`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 522
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

274.  No universal Linux live-location stack exists across distros.

- settingId: `tracking-guide-platform-capability-notes-linux-237`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 526
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

275.  Headless/service deployments may have no useful location provider.

- settingId: `tracking-guide-platform-capability-notes-linux-239`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 528
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

276.  Product claims must name distro/service assumptions and real proof.

- settingId: `tracking-guide-platform-capability-notes-linux-240`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 529
- acceptedOptions: Product Claims Must Name Distro Service Assumptions | Real Proof
- helperText: proof-required via location-capability-registry

#### Android

277.  Device owner/profile owner policy only after real Android Enterprise or device owner proof.

- settingId: `tracking-guide-platform-capability-notes-android-247`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 547
- acceptedOptions: Device Owner Profile Owner Policy Only After Real Android Enterprise | Device Owner Proof
- helperText: proof-required via location-capability-registry

278.  Geofencing responsiveness can be delayed.

- settingId: `tracking-guide-platform-capability-notes-android-250`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 554
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

279.  Store policy matters for a child-agent product.

- settingId: `tracking-guide-platform-capability-notes-android-253`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 557
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

#### iOS And iPadOS

280.  Core Location When In Use and Always authorization.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-254`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 566
- acceptedOptions: Core Location When In Use | Always Authorization
- helperText: needs-effect-wiring via background-location-permission-and-disclosure-proof

281.  Reduced/full accuracy state.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-255`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 567
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

282.  Standard, significant-change, visit, and region monitoring where permitted.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-256`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 568
- acceptedOptions: Standard | Significant Change | Visit | Region Monitoring Where Permitted
- helperText: needs-effect-wiring via location-capability-registry

283.  Family Sharing/Find My location sharing as an Apple user feature, not an Ocentra-owned raw telemetry API.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-258`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 570
- acceptedOptions: Family Sharing Find My Location Sharing As An Apple User Feature | Not An Ocentra Owned Raw Telemetry Api
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

284.  Supervised MDM Lost Mode device location for managed/supervised devices only.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-259`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 572
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

285.  Third-party apps cannot silently access general Find My data.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-260`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 576
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

286.  Region monitoring has platform limits and is not instant.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-262`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 578
- acceptedOptions: Region Monitoring Has Platform Limits | Is Not Instant
- helperText: future-gap via location-capability-registry

287.  MDM device location is tied to Lost Mode and supervision requirements.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-263`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 579
- acceptedOptions: Mdm Device Location Is Tied To Lost Mode | Supervision Requirements
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

288.  Entitlements, App Store review, Family Controls, and device supervision affect what is shippable.

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-264`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 580
- acceptedOptions: Entitlements | App Store Review | Family Controls | Device Supervision Affect What Is Shippable
- helperText: future-gap via location-capability-registry

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

289.  Android and iOS capability claims are currently scaffold/manual-required until real device proof exists.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-313`
- policyLane: `platform`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 709
- acceptedOptions: Android
- helperText: manual-required via location-capability-registry

290.  Platform claims must distinguish implemented, scaffold-only, unavailable, degraded, manual-required, and not-implemented states.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-314`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 711
- acceptedOptions: Platform Claims Must Distinguish Implemented | Scaffold Only | Unavailable | Degraded | Manual Required | Not Implemented States
- helperText: future-gap via location-capability-registry

291.  [`docs/expectations/platforms.md`](expectations/platforms.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-318`
- policyLane: `platform`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 722
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

## Tab: data

### Core Terms

#### Device Location Evidence

292.  Custody label: child-local, parent-device cache, LAN-live, parent-owned storage, Ocentra-hosted non-activity metadata, or unavailable.

- settingId: `tracking-guide-core-terms-device-location-evidence-031`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 37
- acceptedOptions: Child Local | Parent Device Cache | Lan Live | Parent Owned Storage | Ocentra Hosted Non Activity Metadata | Unavailable
- helperText: already-represented via location-provider-freshness-accuracy-proof

#### Location History

293.  Parent reports.

- settingId: `tracking-guide-core-terms-location-history-032`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 63
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

294.  Missed check-in investigation.

- settingId: `tracking-guide-core-terms-location-history-034`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 65
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

295.  Device lost/stolen review.

- settingId: `tracking-guide-core-terms-location-history-035`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 66
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

296.  Safety explanation after an alert.

- settingId: `tracking-guide-core-terms-location-history-036`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 67
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

297.  Export/delete/retention flows.

- settingId: `tracking-guide-core-terms-location-history-037`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 68
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### The Main Capability Truth

#### The Main Capability Truth

298.  Parent-owned storage and local/LAN operation are the normal custody boundary.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-060`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 147
- acceptedOptions: Parent Owned Storage | Local Lan Operation Are The Normal Custody Boundary
- helperText: already-represented via local-history-custody-retention-proof

299.  Every strict policy or alert must carry evidence source, timestamp, accuracy, custody, and adapter state.

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-061`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 148
- acceptedOptions: Every Strict Policy | Alert Must Carry Evidence Source | Timestamp | Accuracy | Custody | Adapter State
- helperText: already-represented via precision-permission-and-accuracy-proof

### Capability Matrix

#### Capability Matrix

300.  Capability matrix row | Capability=Location history | Mobile child agent=Yes, if locally journaled | Desktop/laptop child agent=Yes, if locally journaled | Required layer=Agent journal/query store | Important limit=Retention/custody/delete controls required.

- settingId: `tracking-guide-capability-matrix-capability-matrix-069`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 162
- acceptedOptions: Capability Location History | Mobile Child Agent Yes If Locally Journaled | Desktop Laptop Child Agent Yes If Locally Journaled | Required Layer Agent Journal Query Store | Important Limit Retention Custody Delete Controls Required
- helperText: already-represented via local-history-custody-retention-proof

301.  Capability matrix row | Capability=Exact continuous movement trail | Mobile child agent=Sometimes, but expensive and sensitive | Desktop/laptop child agent=Usually no | Required layer=Frequent location updates | Important limit=Battery, consent, and retention concerns.

- settingId: `tracking-guide-capability-matrix-capability-matrix-072`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 165
- acceptedOptions: Capability Exact Continuous Movement Trail | Mobile Child Agent Sometimes But Expensive And Sensitive | Desktop Laptop Child Agent Usually No | Required Layer Frequent Location Updates | Important Limit Battery Consent And Retention Concerns
- helperText: manual-required via background-location-permission-and-disclosure-proof

302.  Capability matrix row | Capability=Remote live location away from LAN | Mobile child agent=Future, via relay or parent storage | Desktop/laptop child agent=Future | Required layer=Authenticated relay/sync | Important limit=Ocentra must not become default location-history store.

- settingId: `tracking-guide-capability-matrix-capability-matrix-074`
- policyLane: `data`; cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 167
- acceptedOptions: Capability Remote Live Location Away From Lan | Mobile Child Agent Future Via Relay Or Parent Storage | Desktop Laptop Child Agent Future | Required Layer Authenticated Relay Sync | Important Limit Ocentra Must Not Become Default Location History Store
- helperText: future-gap via location-provider-freshness-accuracy-proof

### Location History: What Is Possible

#### Location History: What Is Possible

303.  `location-point`: individual timestamped point with accuracy and provider.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-095`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 213
- acceptedOptions: Individual Timestamped Point With Accuracy | Provider
- helperText: already-represented via precision-permission-and-accuracy-proof

304.  `check-in`: child response, optional location point, note, and prompt reason.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-098`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 217
- acceptedOptions: Child Response | Optional Location Point | Note | Prompt Reason
- helperText: already-represented via local-history-custody-retention-proof

305.  `device-contact`: last online, last sync, battery, and network state.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-099`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 218
- acceptedOptions: Last Online | Last Sync | Battery | Network State
- helperText: degraded via background-location-permission-and-disclosure-proof

306.  `audit-only`: policy decision or parent action without raw coordinate.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-100`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 219
- acceptedOptions: Policy Decision | Parent Action Without Raw Coordinate
- helperText: proof-required via location-provider-freshness-accuracy-proof

307.  Live point stream: short retention.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-101`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 223
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

308.  Policy/audit references: longer retention.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-103`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 225
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

309.  Parent-exported report: parent-chosen retention.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-104`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 226
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

310.  Evidence source and adapter version.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-106`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 231
- acceptedOptions: Evidence Source | Adapter Version
- helperText: already-represented via local-history-custody-retention-proof

311.  Custody and retention labels.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-108`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 233
- acceptedOptions: Custody | Retention Labels
- helperText: already-represented via local-history-custody-retention-proof

312.  Redaction/deletion state.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-109`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 234
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

313.  Parent reveal/audit trail for sensitive exact coordinates.

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-110`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 235
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

### Device Online, Offline, And Battery State

#### Device Online, Offline, And Battery State

314.  Adapter error: show degraded status with audit reference.

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-169`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 366
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### Reports And Maps

#### Reports And Maps

315.  Record parent reveal, export, delete, and retention actions.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-176`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 380
- acceptedOptions: Record Parent Reveal | Export | Delete | Retention Actions
- helperText: already-represented via local-history-custody-retention-proof

316.  Exportable parent report with custody labels.

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-183`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 390
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### Custody, Retention, And Audit

#### Custody, Retention, And Audit

317.  Raw location evidence lives on the child device.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-184`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 399
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

318.  Parent surfaces read local/LAN, parent cache, parent-owned storage, or cloud relay through typed contracts.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-185`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 400
- acceptedOptions: Parent Surfaces Read Local Lan | Parent Cache | Parent Owned Storage | Cloud Relay Through Typed Contracts
- helperText: future-gap via local-history-custody-retention-proof

319.  Ocentra-hosted services may route minimal notification or relay metadata, but they must not retain raw location history by default.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-186`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 402
- acceptedOptions: Ocentra Hosted Services May Route Minimal Notification | Relay Metadata | But They Must Not Retain Raw Location History By Default
- helperText: future-gap via local-history-custody-retention-proof

320.  Retention policy per data class.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-187`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 407
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

321.  Delete expired raw points.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-188`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 408
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

322.  Keep redacted summaries only if parent policy allows.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-189`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 409
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

323.  Export/delete flows that name data classes and destinations.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-190`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 410
- acceptedOptions: Export Delete Flows That Name Data Classes | Destinations
- helperText: already-represented via local-history-custody-retention-proof

324.  Audit for parent reveal, export, sync, delete, policy change, and strict alert action.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 411
- acceptedOptions: Audit For Parent Reveal | Export | Sync | Delete | Policy Change | Strict Alert Action
- helperText: already-represented via local-history-custody-retention-proof

325.  Policy decision or parent action.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-192`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 416
- acceptedOptions: Policy Decision | Parent Action
- helperText: proof-required via local-history-custody-retention-proof

326.  Evidence reference.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-193`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 417
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

327.  Location source and adapter state.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-194`
- policyLane: `data`; cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 418
- acceptedOptions: Location Source | Adapter State
- helperText: already-represented via local-history-custody-retention-proof

328.  Accuracy/freshness.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-195`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 419
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

329.  Custody label.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 420
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

330.  Retention class.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-197`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 421
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

331.  Notification intent reference where alerts were sent.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-198`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 422
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

332.  Failure/degraded reason.

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-199`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 423
- acceptedOptions: Represented | Not Represented
- helperText: degraded via local-history-custody-retention-proof

### Current Ocentra Parent Posture

#### Current Ocentra Parent Posture

333.  Child-device agents own capture, journal, query, local AI, policy, and enforcement paths.

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-309`
- policyLane: `data`; cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 703
- acceptedOptions: Child Device Agents Own Capture | Journal | Query | Local Ai | Policy | Enforcement Paths
- helperText: needs-effect-wiring via local-history-custody-retention-proof

334.  [`docs/expectations/data-custody.md`](expectations/data-custody.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-316`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 720
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

335.  [`docs/expectations/sync-export.md`](expectations/sync-export.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-317`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 721
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### Future UI Rules

#### Future UI Rules

336.  Keep custody, retention, and delete/export status visible for location history.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-328`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 737
- acceptedOptions: Keep Custody | Retention | Delete Export Status Visible For Location History
- helperText: future-gap via local-history-custody-retention-proof

337.  Every strict action should have an audit path: detected state, parent rule, mechanism, outcome, timestamp, accuracy, freshness, custody, and evidence reference.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-331`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `multi`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 741
- acceptedOptions: Detected State | Parent Rule | Mechanism | Outcome | Timestamp | Accuracy | Freshness | Custody | Evidence Reference
- helperText: future-gap via precision-permission-and-accuracy-proof

338.  parent-owned history/report export.

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-338`
- policyLane: `data`; cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: `docs/device-location-tracking-capability-guide.md`; sourceLine: 753
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via local-history-custody-retention-proof
