<!-- agent-capsule -->

> Agent Capsule
> Doc: Tracking Control Settings Inventory
> Kind: repo documentation; read only when routed by root AGENTS, docs indexes, feature route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Tracking Control Settings Inventory

Generated from `BaselineTrackingControlCatalog`.
Total settings: 338

Use this as the raw review list for deciding parent-facing grouping, proof gaps, and policy UX.
This is a generated inventory of current typed catalog data, not product-complete implementation proof.

## Source Documents

- docs/device-location-tracking-capability-guide.md
- docs/device-location-tracking-schema-proposal.md

## Tab: rules

### location-management

#### location-management-controls

1.  Enable device location features?

- settingId: `location.enabled`
- policyLane: `rules`; sectionId: `location-management`; groupId: `location-management-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 148; sourceText: Enable device location features?
- acceptedOptions: Enabled | Disabled
- helperText: needs-effect-wiring via location-capability-registry

2.  What location posture should this device use?

- settingId: `location.defaultPosture`
- policyLane: `rules`; sectionId: `location-management`; groupId: `location-management-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 172; sourceText: What location posture should this device use?
- acceptedOptions: Off | Last known | Check-in | Arrival alerts | Temporary live | Missing device
- helperText: needs-effect-wiring via location-capability-registry

3.  Where should location behavior execute?

- settingId: `location.executionMode`
- policyLane: `rules`; sectionId: `location-management`; groupId: `location-management-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 215; sourceText: Where should location behavior execute?
- acceptedOptions: Local Child Agent | Lan Live | Authenticated Relay | Authoring Only | Unavailable
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-core-terms

#### tracking-guide-core-terms-location-capability-state

4.  Represent: `service-disabled`

- settingId: `tracking-guide-core-terms-location-capability-state-047`
- policyLane: `rules`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 116; sourceText: `service-disabled`
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

5.  Represent: `manual-required`

- settingId: `tracking-guide-core-terms-location-capability-state-049`
- policyLane: `rules`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 118; sourceText: `manual-required`
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

6.  Represent: `offline-last-known-only`

- settingId: `tracking-guide-core-terms-location-capability-state-050`
- policyLane: `rules`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 119; sourceText: `offline-last-known-only`
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-capability-registry

7.  Represent: `battery-throttled`

- settingId: `tracking-guide-core-terms-location-capability-state-051`
- policyLane: `rules`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 120; sourceText: `battery-throttled`
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-the-main-capability-truth

#### tracking-guide-the-main-capability-truth-the-main-capability-truth

8.  Represent: Battery saver, low-power modes, radio state, and network reachability can degrade tracking

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-059`
- policyLane: `rules`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 145; sourceText: Battery saver, low-power modes, radio state, and network reachability can degrade tracking.
- acceptedOptions: Battery Saver | Low Power Modes | Radio State | Network Reachability Can Degrade Tracking
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-device-online-offline-and-battery-state

#### tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state

9.  Represent: Last agent heartbeat

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-156`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 350; sourceText: Last agent heartbeat.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

10. Represent: Last location sample

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-157`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 351; sourceText: Last location sample.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

11. Represent: Last successful parent sync

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-158`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 352; sourceText: Last successful parent sync.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

12. Represent: Battery percentage and charging state where available

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-159`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 353; sourceText: Battery percentage and charging state where available.
- acceptedOptions: Battery Percentage | Charging State Where Available
- helperText: degraded via background-location-permission-and-disclosure-proof

13. Represent: Low-power/battery-saver state where available

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-160`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 354; sourceText: Low-power/battery-saver state where available.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

14. Represent: Pending upload count

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-163`
- policyLane: `rules`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 357; sourceText: Pending upload count.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode

15. Represent: Parent marks device missing

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-296`
- policyLane: `rules`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 667; sourceText: Parent marks device missing.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

16. Represent: Device contact, battery, and network state become prominent

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-298`
- policyLane: `rules`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 669; sourceText: Device contact, battery, and network state become prominent.
- acceptedOptions: Device Contact | Battery | Network State Become Prominent
- helperText: degraded via background-location-permission-and-disclosure-proof

17. Represent: Tracking a powered-off or offline device

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-301`
- policyLane: `rules`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 678; sourceText: Tracking a powered-off or offline device.
- acceptedOptions: Tracking A Powered Off | Offline Device
- helperText: degraded via location-capability-registry

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

18. Represent: Show degraded/offline/manual-required states instead of disabled-looking mystery failures

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-330`
- policyLane: `rules`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 739; sourceText: Show degraded/offline/manual-required states instead of disabled-looking mystery failures.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

19. Represent: last-known only

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-332`
- policyLane: `rules`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 747; sourceText: last-known only;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

20. Represent: missing device mode

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-336`
- policyLane: `rules`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 751; sourceText: missing device mode;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: evidence

### tracking-guide-core-terms

#### tracking-guide-core-terms-device-location-evidence

21. Represent: Latitude, longitude, altitude, heading, speed, and bearing where exposed

- settingId: `tracking-guide-core-terms-device-location-evidence-025`
- policyLane: `evidence`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 28; sourceText: Latitude, longitude, altitude, heading, speed, and bearing where exposed.
- acceptedOptions: Latitude | Longitude | Altitude | Heading | Speed | Bearing Where Exposed
- helperText: needs-effect-wiring via location-capability-registry

22. Represent: Horizontal and vertical accuracy

- settingId: `tracking-guide-core-terms-device-location-evidence-026`
- policyLane: `evidence`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 29; sourceText: Horizontal and vertical accuracy.
- acceptedOptions: Horizontal | Vertical Accuracy
- helperText: already-represented via precision-permission-and-accuracy-proof

23. Represent: Provider kind: GPS/GNSS, Wi-Fi, cellular, IP, Bluetooth/beacon, fused provider, user-entered default, or unknown

- settingId: `tracking-guide-core-terms-device-location-evidence-028`
- policyLane: `evidence`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 31; sourceText: Provider kind: GPS/GNSS, Wi-Fi, cellular, IP, Bluetooth/beacon, fused provider, user-entered default, or unknown.
- acceptedOptions: Gps Gnss | Wi Fi | Cellular | Ip | Bluetooth Beacon | Fused Provider | User Entered Default | Unknown
- helperText: manual-required via location-provider-freshness-accuracy-proof

#### tracking-guide-core-terms-location-capability-state

24. Represent: `adapter-error`

- settingId: `tracking-guide-core-terms-location-capability-state-052`
- policyLane: `evidence`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 121; sourceText: `adapter-error`
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode

25. Represent: Clear separation between Ocentra evidence and OS account features

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-300`
- policyLane: `evidence`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 674; sourceText: Clear separation between Ocentra evidence and OS account features.
- acceptedOptions: Clear Separation Between Ocentra Evidence | Os Account Features
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-current-ocentra-parent-posture

#### tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

26. Represent: Raw child activity evidence is local-first by default

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-311`
- policyLane: `evidence`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 707; sourceText: Raw child activity evidence is local-first by default.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

27. Represent: [`docs/expectations/policy.md`](expectations/policy.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-319`
- policyLane: `evidence`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 723; sourceText: [`docs/expectations/policy.md`](expectations/policy.md)
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

## Tab: live

### live-tracking

#### live-tracking-controls

28. When can live tracking run?

- settingId: `live.mode`
- policyLane: `live`; sectionId: `live-tracking`; groupId: `live-tracking-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 301; sourceText: When can live tracking run?
- acceptedOptions: Disabled | Parent Started Temporary | During Active Trip | During Missing Device | During Alert Investigation
- helperText: proof-required via location-provider-freshness-accuracy-proof

29. What is the maximum live session duration?

- settingId: `live.maxSessionMinutes`
- policyLane: `live`; sectionId: `live-tracking`; groupId: `live-tracking-controls`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 315; sourceText: What is the maximum live session duration?
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

30. How often should live tracking request updates?

- settingId: `live.updateCadence`
- policyLane: `live`; sectionId: `live-tracking`; groupId: `live-tracking-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 324; sourceText: How often should live tracking request updates?
- acceptedOptions: One Shot | On Change | Battery Balanced | High Accuracy Burst | Manual Refresh Only
- helperText: proof-required via location-provider-freshness-accuracy-proof

31. What should happen when battery is low?

- settingId: `live.whenBatteryLow`
- policyLane: `live`; sectionId: `live-tracking`; groupId: `live-tracking-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 332; sourceText: What should happen when battery is low?
- acceptedOptions: Continue | Reduce Cadence | Last Known Only | Ask Parent | Stop Live Session
- helperText: degraded via location-provider-freshness-accuracy-proof

### tracking-guide-core-terms

#### tracking-guide-core-terms-check-in

32. Represent: The parent wants low-noise confirmation instead of a live map

- settingId: `tracking-guide-core-terms-check-in-040`
- policyLane: `live`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-check-in`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 91; sourceText: The parent wants low-noise confirmation instead of a live map.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

33. Represent: Capability matrix row | Capability=Foreground live tracking | Mobile child agent=Yes, while app/session active | Desktop/laptop child agent=Sometimes, while app/service active | Required layer=OS location API | Important limit=Requires visible use or active session semantics

- settingId: `tracking-guide-capability-matrix-capability-matrix-064`
- policyLane: `live`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 157; sourceText: Capability matrix row | Capability=Foreground live tracking | Mobile child agent=Yes, while app/session active | Desktop/laptop child agent=Sometimes, while app/service active | Required layer=OS location API | Important limit=Requires visible use or active session semantics.
- acceptedOptions: Capability Foreground Live Tracking | Mobile Child Agent Yes While App Session Active | Desktop Laptop Child Agent Sometimes While App Service Active | Required Layer Os Location Api | Important Limit Requires Visible Use Or Active Session Semantics
- helperText: permission-required via location-provider-freshness-accuracy-proof

34. Represent: Capability matrix row | Capability=Background live tracking | Mobile child agent=Platform-dependent | Desktop/laptop child agent=Limited | Required layer=Background execution plus permission | Important limit=Throttled, battery-sensitive, and often entitlement-bound

- settingId: `tracking-guide-capability-matrix-capability-matrix-065`
- policyLane: `live`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 158; sourceText: Capability matrix row | Capability=Background live tracking | Mobile child agent=Platform-dependent | Desktop/laptop child agent=Limited | Required layer=Background execution plus permission | Important limit=Throttled, battery-sensitive, and often entitlement-bound.
- acceptedOptions: Capability Background Live Tracking | Mobile Child Agent Platform Dependent | Desktop Laptop Child Agent Limited | Required Layer Background Execution Plus Permission | Important Limit Throttled Battery Sensitive And Often Entitlement Bound
- helperText: permission-required via location-provider-freshness-accuracy-proof

### tracking-guide-live-tracking-what-is-possible

#### tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible

35. Represent: Parent opens live map

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-076`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 177; sourceText: Parent opens live map.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

36. Represent: Child is travelling between known places

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-077`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 178; sourceText: Child is travelling between known places.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

37. Represent: Child missed an expected arrival

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-078`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 179; sourceText: Child missed an expected arrival.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

38. Represent: Device is marked missing

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-079`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 180; sourceText: Device is marked missing.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

39. Represent: Parent explicitly starts a temporary safety session

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-080`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 181; sourceText: Parent explicitly starts a temporary safety session.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

40. Represent: A policy rule asks for short-term verification after a geofence miss

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-081`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 182; sourceText: A policy rule asks for short-term verification after a geofence miss.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

41. Represent: Session id, child id, device id, requester, and reason code

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-082`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 186; sourceText: Session id, child id, device id, requester, and reason code.
- acceptedOptions: Session Id | Child Id | Device Id | Requester | Reason Code
- helperText: proof-required via location-provider-freshness-accuracy-proof

42. Represent: Requested accuracy: approximate, precise, or best-available

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-083`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 187; sourceText: Requested accuracy: approximate, precise, or best-available.
- acceptedOptions: Approximate | Precise | Best Available
- helperText: permission-required via location-provider-freshness-accuracy-proof

43. Represent: Requested cadence: one-shot, on-change, interval, high-accuracy burst, or geofence-only

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-084`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 188; sourceText: Requested cadence: one-shot, on-change, interval, high-accuracy burst, or geofence-only.
- acceptedOptions: One Shot | On Change | Interval | High Accuracy Burst | Geofence Only
- helperText: proof-required via location-provider-freshness-accuracy-proof

44. Represent: Maximum duration and auto-stop reason

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-085`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 190; sourceText: Maximum duration and auto-stop reason.
- acceptedOptions: Maximum Duration | Auto Stop Reason
- helperText: proof-required via location-provider-freshness-accuracy-proof

45. Represent: Permission requirement and user-visible disclosure state

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-086`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 191; sourceText: Permission requirement and user-visible disclosure state.
- acceptedOptions: Permission Requirement | User Visible Disclosure State
- helperText: permission-required via location-provider-freshness-accuracy-proof

46. Represent: Delivery path: local, LAN, authenticated relay, parent cache, or parent-owned storage

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-087`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 192; sourceText: Delivery path: local, LAN, authenticated relay, parent cache, or parent-owned storage.
- acceptedOptions: Local | Lan | Authenticated Relay | Parent Cache | Parent Owned Storage
- helperText: future-gap via location-provider-freshness-accuracy-proof

47. Represent: Audit events for start, update, degrade, stop, and parent reveal

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-088`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 194; sourceText: Audit events for start, update, degrade, stop, and parent reveal.
- acceptedOptions: Audit Events For Start | Update | Degrade | Stop | Parent Reveal
- helperText: degraded via location-provider-freshness-accuracy-proof

48. Represent: OSes may throttle background updates

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-089`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 198; sourceText: OSes may throttle background updates.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

49. Represent: Apps may need foreground service notification, background mode, entitlement, device-owner state, MDM supervision, or explicit user permission

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-090`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 199; sourceText: Apps may need foreground service notification, background mode, entitlement, device-owner state, MDM supervision, or explicit user permission.
- acceptedOptions: Apps May Need Foreground Service Notification | Background Mode | Entitlement | Device Owner State | Mdm Supervision | Explicit User Permission
- helperText: future-gap via location-provider-freshness-accuracy-proof

50. Represent: High accuracy increases battery use

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-091`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 201; sourceText: High accuracy increases battery use.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

51. Represent: Indoor GPS may fail or fall back to Wi-Fi/cell/IP estimates

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-092`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 202; sourceText: Indoor GPS may fail or fall back to Wi-Fi/cell/IP estimates.
- acceptedOptions: Indoor Gps May Fail | Fall Back To Wi Fi Cell Ip Estimates
- helperText: degraded via location-provider-freshness-accuracy-proof

52. Represent: A child can turn off device location services, revoke permission, uninstall the app where allowed, power off the device, or lose network

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-093`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 203; sourceText: A child can turn off device location services, revoke permission, uninstall the app where allowed, power off the device, or lose network.
- acceptedOptions: A Child Can Turn Off Device Location Services | Revoke Permission | Uninstall The App Where Allowed | Power Off The Device | Lose Network
- helperText: permission-required via location-provider-freshness-accuracy-proof

53. Represent: Live map updates should never imply that a stale point is still current

- settingId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible-094`
- policyLane: `live`; sectionId: `tracking-guide-live-tracking-what-is-possible`; groupId: `tracking-guide-live-tracking-what-is-possible-live-tracking-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 205; sourceText: Live map updates should never imply that a stale point is still current.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

### tracking-guide-device-online-offline-and-battery-state

#### tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state

54. Represent: Low battery: reduce cadence and explain degraded state

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-167`
- policyLane: `live`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 364; sourceText: Low battery: reduce cadence and explain degraded state.
- acceptedOptions: Reduce Cadence | Explain Degraded State
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-child-facing-disclosure

#### tracking-guide-child-facing-disclosure-child-facing-disclosure

55. Represent: Live map temporarily active

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-203`
- policyLane: `live`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 434; sourceText: Live map temporarily active.
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-provider-freshness-accuracy-proof

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

56. Represent: Battery throttled -> reduce cadence and record degraded state

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-212`
- policyLane: `live`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 454; sourceText: Battery throttled -> reduce cadence and record degraded state.
- acceptedOptions: Battery Throttled Reduce Cadence | Record Degraded State
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-platform-capability-notes

#### tracking-guide-platform-capability-notes-windows

57. Represent: Continuous tracking has battery impact and should be cadence-limited

- settingId: `tracking-guide-platform-capability-notes-windows-223`
- policyLane: `live`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 486; sourceText: Continuous tracking has battery impact and should be cadence-limited.
- acceptedOptions: Continuous Tracking Has Battery Impact | Should Be Cadence Limited
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map

58. Represent: Parent starts a time-limited live tracking session

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-288`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 648; sourceText: Parent starts a time-limited live tracking session.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

59. Represent: Agent sends repeated updates while online and permitted

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-289`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 649; sourceText: Agent sends repeated updates while online and permitted.
- acceptedOptions: Agent Sends Repeated Updates While Online | Permitted
- helperText: already-represented via location-provider-freshness-accuracy-proof

60. Represent: Session has a visible reason, duration, and audit trail

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-290`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 650; sourceText: Session has a visible reason, duration, and audit trail.
- acceptedOptions: Session Has A Visible Reason | Duration | Audit Trail
- helperText: already-represented via location-provider-freshness-accuracy-proof

61. Represent: Location permission

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-291`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 654; sourceText: Location permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

62. Represent: Runtime update path

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-292`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 655; sourceText: Runtime update path.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

63. Represent: Local/LAN/relay delivery path

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-293`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 656; sourceText: Local/LAN/relay delivery path.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

64. Represent: Battery-aware cadence

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-294`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 657; sourceText: Battery-aware cadence.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-provider-freshness-accuracy-proof

65. Represent: High sensitivity and battery impact

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map-295`
- policyLane: `live`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-temporary-live-map`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 661; sourceText: High sensitivity and battery impact.
- acceptedOptions: High Sensitivity | Battery Impact
- helperText: degraded via location-provider-freshness-accuracy-proof

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

66. Represent: Show live tracking only when a fresh update path is available

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-321`
- policyLane: `live`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 730; sourceText: Show live tracking only when a fresh update path is available.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

67. Represent: Show child-facing disclosure mode when background/live tracking is enabled

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-329`
- policyLane: `live`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 738; sourceText: Show child-facing disclosure mode when background/live tracking is enabled.
- acceptedOptions: Enabled | Disabled
- helperText: future-gap via location-provider-freshness-accuracy-proof

68. Represent: temporary live map

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-335`
- policyLane: `live`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 750; sourceText: temporary live map;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

## Tab: places

### places-geofences

#### places-geofences-controls

69. Enable parent-defined places?

- settingId: `places.enabled`
- policyLane: `places`; sectionId: `places-geofences`; groupId: `places-geofences-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 430; sourceText: Enable parent-defined places?
- acceptedOptions: Enabled | Disabled
- helperText: proof-required via geofence-region-schedule-transition-proof

70. What minimum radius should place geofences use?

- settingId: `places.minimumRadiusMeters`
- policyLane: `places`; sectionId: `places-geofences`; groupId: `places-geofences-controls`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 437; sourceText: What minimum radius should place geofences use?
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

71. What if geofence monitoring is unavailable?

- settingId: `geofences.whenUnavailable`
- policyLane: `places`; sectionId: `places-geofences`; groupId: `places-geofences-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 446; sourceText: What if geofence monitoring is unavailable?
- acceptedOptions: Fallback To Check In | Fallback To Sampled Location | Report Unavailable | Ask Parent | Disable Geofence Rules
- helperText: proof-required via geofence-region-schedule-transition-proof

72. Which geofence transitions should be used?

- settingId: `geofences.transitionTypes`
- policyLane: `places`; sectionId: `places-geofences`; groupId: `places-geofences-controls`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 460; sourceText: Which geofence transitions should be used?
- acceptedOptions: Enter | Exit | Dwell | Missed Arrival | Stale At Place
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-core-terms

#### tracking-guide-core-terms-location-history

73. Represent: Arrival and departure audit

- settingId: `tracking-guide-core-terms-location-history-033`
- policyLane: `places`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 64; sourceText: Arrival and departure audit.
- acceptedOptions: Arrival | Departure Audit
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-the-main-capability-truth

#### tracking-guide-the-main-capability-truth-the-main-capability-truth

74. Represent: Geofence alerts are not instant and may be throttled

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-057`
- policyLane: `places`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 143; sourceText: Geofence alerts are not instant and may be throttled.
- acceptedOptions: Geofence Alerts Are Not Instant | May Be Throttled
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

75. Represent: Capability matrix row | Capability=Geofence enter/exit | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak or app-running-only on desktop | Required layer=OS geofence/region monitoring | Important limit=Delayed events, radius limits, count limits, false exits

- settingId: `tracking-guide-capability-matrix-capability-matrix-066`
- policyLane: `places`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 159; sourceText: Capability matrix row | Capability=Geofence enter/exit | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak or app-running-only on desktop | Required layer=OS geofence/region monitoring | Important limit=Delayed events, radius limits, count limits, false exits.
- acceptedOptions: Capability Geofence Enter Exit | Mobile Child Agent Android Ios With Limits | Desktop Laptop Child Agent Weak Or App Running Only On Desktop | Required Layer Os Geofence Region Monitoring | Important Limit Delayed Events Radius Limits Count Limits False Exits
- helperText: future-gap via geofence-region-schedule-transition-proof

76. Represent: Capability matrix row | Capability=Dwell alerts | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak | Required layer=Geofence plus dwell support | Important limit=Not immediate; large radius may be required

- settingId: `tracking-guide-capability-matrix-capability-matrix-067`
- policyLane: `places`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 160; sourceText: Capability matrix row | Capability=Dwell alerts | Mobile child agent=Android/iOS with limits | Desktop/laptop child agent=Weak | Required layer=Geofence plus dwell support | Important limit=Not immediate; large radius may be required.
- acceptedOptions: Capability Dwell Alerts | Mobile Child Agent Android Ios With Limits | Desktop Laptop Child Agent Weak | Required Layer Geofence Plus Dwell Support | Important Limit Not Immediate Large Radius May Be Required
- helperText: future-gap via geofence-region-schedule-transition-proof

77. Represent: Capability matrix row | Capability=Parent arrival/departure alerts | Mobile child agent=Yes, with geofence/check-in evidence | Desktop/laptop child agent=Limited | Required layer=Geofence or sampled evidence | Important limit=Notifications must minimize sensitive detail

- settingId: `tracking-guide-capability-matrix-capability-matrix-071`
- policyLane: `places`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 164; sourceText: Capability matrix row | Capability=Parent arrival/departure alerts | Mobile child agent=Yes, with geofence/check-in evidence | Desktop/laptop child agent=Limited | Required layer=Geofence or sampled evidence | Important limit=Notifications must minimize sensitive detail.
- acceptedOptions: Capability Parent Arrival Departure Alerts | Mobile Child Agent Yes With Geofence Check In Evidence | Desktop Laptop Child Agent Limited | Required Layer Geofence Or Sampled Evidence | Important Limit Notifications Must Minimize Sensitive Detail
- helperText: degraded via geofence-region-schedule-transition-proof

### tracking-guide-location-history-what-is-possible

#### tracking-guide-location-history-what-is-possible-location-history-what-is-possible

78. Represent: `location-summary`: derived day/trip/place summary with redacted detail

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-096`
- policyLane: `places`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 214; sourceText: `location-summary`: derived day/trip/place summary with redacted detail.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

79. Represent: `geofence-transition`: arrival, departure, dwell, missed arrival, or stale state

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-097`
- policyLane: `places`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 215; sourceText: `geofence-transition`: arrival, departure, dwell, missed arrival, or stale state.
- acceptedOptions: Arrival | Departure | Dwell | Missed Arrival | Stale State
- helperText: degraded via geofence-region-schedule-transition-proof

80. Represent: Place/geofence audit: medium retention

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-102`
- policyLane: `places`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 224; sourceText: Place/geofence audit: medium retention.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-geofences

#### tracking-guide-geofences-geofences

81. Represent: Arrived at school during a schedule

- settingId: `tracking-guide-geofences-geofences-136`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 295; sourceText: Arrived at school during a schedule.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

82. Represent: Left school before dismissal

- settingId: `tracking-guide-geofences-geofences-137`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 296; sourceText: Left school before dismissal.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

83. Represent: Did not arrive at practice by a time

- settingId: `tracking-guide-geofences-geofences-138`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 297; sourceText: Did not arrive at practice by a time.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

84. Represent: Stayed near home after bedtime

- settingId: `tracking-guide-geofences-geofences-139`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 298; sourceText: Stayed near home after bedtime.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

85. Represent: Notify if device leaves a travel corridor

- settingId: `tracking-guide-geofences-geofences-140`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 299; sourceText: Notify if device leaves a travel corridor.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

86. Represent: Region id, label token, latitude, longitude, radius, schedule, transition type, dwell duration, and expiration

- settingId: `tracking-guide-geofences-geofences-141`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `number-card`; selectionMode: `multi`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 303; sourceText: Region id, label token, latitude, longitude, radius, schedule, transition type, dwell duration, and expiration.
- acceptedOptions: Region Id | Label Token | Latitude | Longitude | Radius | Schedule | Transition Type | Dwell Duration | Expiration
- helperText: proof-required via geofence-region-schedule-transition-proof

87. Represent: Minimum radius and maximum count per platform

- settingId: `tracking-guide-geofences-geofences-142`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 305; sourceText: Minimum radius and maximum count per platform.
- acceptedOptions: Minimum Radius | Maximum Count Per Platform
- helperText: proof-required via geofence-region-schedule-transition-proof

88. Represent: Proof requirement: platform geofence, sampled location, check-in, or manual

- settingId: `tracking-guide-geofences-geofences-143`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 306; sourceText: Proof requirement: platform geofence, sampled location, check-in, or manual.
- acceptedOptions: Platform Geofence | Sampled Location | Check In | Manual
- helperText: manual-required via geofence-region-schedule-transition-proof

89. Represent: Fallback when geofence monitoring is unavailable

- settingId: `tracking-guide-geofences-geofences-144`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 307; sourceText: Fallback when geofence monitoring is unavailable.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

90. Represent: Debounce/noise control to avoid alert storms

- settingId: `tracking-guide-geofences-geofences-145`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 308; sourceText: Debounce/noise control to avoid alert storms.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

91. Represent: Geofence events can be delayed

- settingId: `tracking-guide-geofences-geofences-146`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 312; sourceText: Geofence events can be delayed.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

92. Represent: Small radii can be unreliable

- settingId: `tracking-guide-geofences-geofences-147`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 313; sourceText: Small radii can be unreliable.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

93. Represent: Wi-Fi, cell, and GPS availability affect transition quality

- settingId: `tracking-guide-geofences-geofences-148`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 314; sourceText: Wi-Fi, cell, and GPS availability affect transition quality.
- acceptedOptions: Wi Fi | Cell | Gps Availability Affect Transition Quality
- helperText: proof-required via location-provider-freshness-accuracy-proof

94. Represent: Some platforms wake apps for geofence events; others only work while the app is running

- settingId: `tracking-guide-geofences-geofences-149`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 315; sourceText: Some platforms wake apps for geofence events; others only work while the app is running.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

95. Represent: Geofence count limits require compile-time validation

- settingId: `tracking-guide-geofences-geofences-150`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 317; sourceText: Geofence count limits require compile-time validation.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

96. Represent: Dwell events are useful to reduce alert noise but can delay notifications

- settingId: `tracking-guide-geofences-geofences-151`
- policyLane: `places`; sectionId: `tracking-guide-geofences`; groupId: `tracking-guide-geofences-geofences`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 318; sourceText: Dwell events are useful to reduce alert noise but can delay notifications.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-reports-and-maps

#### tracking-guide-reports-and-maps-reports-and-maps

97. Represent: Visualize accuracy radius when useful

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-171`
- policyLane: `places`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 375; sourceText: Visualize accuracy radius when useful.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

98. Represent: Distinguish live, last-known, check-in, geofence, and manual/default points

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-173`
- policyLane: `places`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 377; sourceText: Distinguish live, last-known, check-in, geofence, and manual/default points.
- acceptedOptions: Distinguish Live | Last Known | Check In | Geofence | Manual Default Points
- helperText: manual-required via location-provider-freshness-accuracy-proof

99. Represent: Arrivals/departures by place

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-178`
- policyLane: `places`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 385; sourceText: Arrivals/departures by place.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

100.  Represent: Missed arrival/departure alerts

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-179`
- policyLane: `places`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 386; sourceText: Missed arrival/departure alerts.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-child-facing-disclosure

#### tracking-guide-child-facing-disclosure-child-facing-disclosure

101.  Represent: Arrival/departure alerts enabled

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-202`
- policyLane: `places`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 433; sourceText: Arrival/departure alerts enabled.
- acceptedOptions: Enabled | Disabled
- helperText: manual-required via geofence-region-schedule-transition-proof

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

102.  Represent: Geofence unavailable -> use scheduled check-in or sampled location if allowed

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-209`
- policyLane: `places`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 450; sourceText: Geofence unavailable -> use scheduled check-in or sampled location if allowed.
- acceptedOptions: Geofence Unavailable Use Scheduled Check In | Sampled Location If Allowed
- helperText: proof-required via geofence-region-schedule-transition-proof

### tracking-guide-platform-capability-notes

#### tracking-guide-platform-capability-notes-macos

103.  Represent: Region monitoring only while the app is running and the system is awake, according to Apple geofence documentation

- settingId: `tracking-guide-platform-capability-notes-macos-227`
- policyLane: `places`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 497; sourceText: Region monitoring only while the app is running and the system is awake, according to Apple geofence documentation.
- acceptedOptions: Region Monitoring Only While The App Is Running | The System Is Awake | According To Apple Geofence Documentation
- helperText: proof-required via geofence-region-schedule-transition-proof

#### tracking-guide-platform-capability-notes-android

104.  Represent: Geofencing API with platform count, permission, delay, and radius limits

- settingId: `tracking-guide-platform-capability-notes-android-245`
- policyLane: `places`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 544; sourceText: Geofencing API with platform count, permission, delay, and radius limits.
- acceptedOptions: Geofencing Api With Platform Count | Permission | Delay | Radius Limits
- helperText: permission-required via geofence-region-schedule-transition-proof

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location

105.  Represent: Geofence setup

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-269`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 599; sourceText: Geofence setup.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

106.  Represent: Arrival/departure alerts unless separately enabled

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-272`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 605; sourceText: Arrival/departure alerts unless separately enabled.
- acceptedOptions: Enabled | Disabled
- helperText: proof-required via geofence-region-schedule-transition-proof

#### tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts

107.  Represent: Parent defines places and schedules

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-280`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `schedule-card`; selectionMode: `single`; controlKind: `schedule`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 629; sourceText: Parent defines places and schedules.
- acceptedOptions: Parent Defines Places | Schedules
- helperText: proof-required via geofence-region-schedule-transition-proof

108.  Represent: Agent records geofence or sampled evidence

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-281`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 630; sourceText: Agent records geofence or sampled evidence.
- acceptedOptions: Agent Records Geofence | Sampled Evidence
- helperText: proof-required via geofence-region-schedule-transition-proof

109.  Represent: Notifications are sent only through alert rules

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-282`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 631; sourceText: Notifications are sent only through alert rules.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

110.  Represent: Geofence or sampled-location capability

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-283`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 635; sourceText: Geofence or sampled-location capability.
- acceptedOptions: Geofence | Sampled Location Capability
- helperText: proof-required via geofence-region-schedule-transition-proof

111.  Represent: Debounce/noise controls

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-284`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 636; sourceText: Debounce/noise controls.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

112.  Represent: Custody and retention settings

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-285`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 637; sourceText: Custody and retention settings.
- acceptedOptions: Custody | Retention Settings
- helperText: proof-required via geofence-region-schedule-transition-proof

113.  Represent: Instant transitions

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-286`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 641; sourceText: Instant transitions.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

114.  Represent: Small-radius precision

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts-287`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-arrival-and-departure-alerts`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 642; sourceText: Small-radius precision.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via geofence-region-schedule-transition-proof

#### tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision

115.  Represent: Location evidence can contribute to local policy, such as "ask if leaving school early" or "notify if not at practice by 18:00"

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-302`
- policyLane: `places`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 684; sourceText: Location evidence can contribute to local policy, such as "ask if leaving school early" or "notify if not at practice by 18:00".
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

116.  Represent: Show exact coordinate reveal separately from summary/place reporting

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-323`
- policyLane: `places`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `place-list-card`; selectionMode: `single`; controlKind: `place-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 732; sourceText: Show exact coordinate reveal separately from summary/place reporting.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

117.  Represent: Show geofence alerts as delayed/coarse arrival/departure evidence

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-325`
- policyLane: `places`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 734; sourceText: Show geofence alerts as delayed/coarse arrival/departure evidence.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via geofence-region-schedule-transition-proof

118.  Represent: geofence arrival/departure

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-334`
- policyLane: `places`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `geofence-list-card`; selectionMode: `single`; controlKind: `geofence-list`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Geofence decisions require region shape, radius, schedule, transition type, freshness, and fallback proof.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 749; sourceText: geofence arrival/departure;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via geofence-region-schedule-transition-proof

## Tab: approvals

### check-ins

#### check-ins-controls

119.  How should check-ins work?

- settingId: `checkIns.mode`
- policyLane: `approvals`; sectionId: `check-ins`; groupId: `check-ins-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 386; sourceText: How should check-ins work?
- acceptedOptions: Disabled | Parent Requested | Scheduled | Geofence Miss | Policy Triggered
- helperText: needs-effect-wiring via location-capability-registry

120.  Should check-ins include location?

- settingId: `checkIns.includeLocation`
- policyLane: `approvals`; sectionId: `check-ins`; groupId: `check-ins-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 394; sourceText: Should check-ins include location?
- acceptedOptions: Never | When Permitted | Require Current Location | Allow Child Choice
- helperText: needs-effect-wiring via location-capability-registry

121.  When is a check-in unanswered?

- settingId: `checkIns.unansweredAfterMinutes`
- policyLane: `approvals`; sectionId: `check-ins`; groupId: `check-ins-controls`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 402; sourceText: When is a check-in unanswered?
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

122.  Which child responses are allowed?

- settingId: `checkIns.allowedResponses`
- policyLane: `approvals`; sectionId: `check-ins`; groupId: `check-ins-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 411; sourceText: Which child responses are allowed?
- acceptedOptions: Safe | Arriving | Leaving | Delayed | Need Help | Call Me | Custom Note
- helperText: needs-effect-wiring via location-capability-registry

### alerts

#### alerts-controls

123.  Which location events should notify a parent?

- settingId: `alerts.enabledReasons`
- policyLane: `approvals`; sectionId: `alerts`; groupId: `alerts-controls`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 479; sourceText: Which location events should notify a parent?
- acceptedOptions: Arrival | Departure | Early Departure | Missed Arrival | Unanswered Check In | Need Help Check In | Device Offline During Trip | Location Permission Lost | Live Session Started | Missing Device Found
- helperText: needs-effect-wiring via location-capability-registry

124.  What location detail may appear in push/email/SMS bodies?

- settingId: `alerts.sensitiveDetailsInProviderBody`
- policyLane: `approvals`; sectionId: `alerts`; groupId: `alerts-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 498; sourceText: What location detail may appear in push/email/SMS bodies?
- acceptedOptions: None | Minimal | Place Label Only | Approximate Area | Exact Coordinate
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-core-terms

#### tracking-guide-core-terms-check-in

125.  Represent: The child device is on a constrained platform

- settingId: `tracking-guide-core-terms-check-in-039`
- policyLane: `approvals`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-check-in`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 90; sourceText: The child device is on a constrained platform.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

126.  Represent: The agent can send an ask/confirm prompt without tracking movement all day

- settingId: `tracking-guide-core-terms-check-in-041`
- policyLane: `approvals`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-check-in`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 92; sourceText: The agent can send an ask/confirm prompt without tracking movement all day.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-check-in-and-safety-prompts

#### tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts

127.  Represent: A check-in response without a fresh coordinate is still useful

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-152`
- policyLane: `approvals`; sectionId: `tracking-guide-check-in-and-safety-prompts`; groupId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 338; sourceText: A check-in response without a fresh coordinate is still useful.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

128.  Represent: A fresh coordinate without child response should be labeled as location-only

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-153`
- policyLane: `approvals`; sectionId: `tracking-guide-check-in-and-safety-prompts`; groupId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 339; sourceText: A fresh coordinate without child response should be labeled as location-only.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

129.  Represent: Unanswered check-ins should produce a notification intent only through an explicit alert rule

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-154`
- policyLane: `approvals`; sectionId: `tracking-guide-check-in-and-safety-prompts`; groupId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 340; sourceText: Unanswered check-ins should produce a notification intent only through an explicit alert rule.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

130.  Represent: Sensitive child notes should not appear in third-party notification previews

- settingId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts-155`
- policyLane: `approvals`; sectionId: `tracking-guide-check-in-and-safety-prompts`; groupId: `tracking-guide-check-in-and-safety-prompts-check-in-and-safety-prompts`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 342; sourceText: Sensitive child notes should not appear in third-party notification previews.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only

131.  Represent: Parent can request a child response

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-273`
- policyLane: `approvals`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 611; sourceText: Parent can request a child response.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

132.  Represent: Unanswered check-ins can drive explicit notification rules

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-275`
- policyLane: `approvals`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 613; sourceText: Unanswered check-ins can drive explicit notification rules.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

133.  Represent: Child response

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-278`
- policyLane: `approvals`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 622; sourceText: Child response.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-current-ocentra-parent-posture

#### tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

134.  Represent: Remote relay and notifications must minimize child details

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-312`
- policyLane: `approvals`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 708; sourceText: Remote relay and notifications must minimize child details.
- acceptedOptions: Remote Relay | Notifications Must Minimize Child Details
- helperText: future-gap via authenticated-relay-proof-without-default-location-history-storage

135.  Represent: [`docs/expectations/notifications.md`](expectations/notifications.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-320`
- policyLane: `approvals`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 724; sourceText: [`docs/expectations/notifications.md`](expectations/notifications.md)
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

136.  Represent: Show check-in as a separate workflow from tracking

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-326`
- policyLane: `approvals`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 735; sourceText: Show check-in as a separate workflow from tracking.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

137.  Represent: check-in only

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-333`
- policyLane: `approvals`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 748; sourceText: check-in only;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

138.  Represent: location-based policy alerts

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-337`
- policyLane: `approvals`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 752; sourceText: location-based policy alerts;
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: enforcement

### tracking-guide-accuracy-sources-and-limits

#### tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits

139.  Represent: GPS/GNSS: best outdoors, weaker indoors, battery-sensitive

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-120`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 263; sourceText: GPS/GNSS: best outdoors, weaker indoors, battery-sensitive.
- acceptedOptions: Best Outdoors | Weaker Indoors | Battery Sensitive
- helperText: degraded via location-provider-freshness-accuracy-proof

140.  Represent: IP address: coarse and often wrong for household, VPN, carrier-grade NAT, or corporate networks

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-123`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 267; sourceText: IP address: coarse and often wrong for household, VPN, carrier-grade NAT, or corporate networks.
- acceptedOptions: Coarse | Often Wrong For Household | Vpn | Carrier Grade Nat | Corporate Networks
- helperText: already-represented via precision-permission-and-accuracy-proof

141.  Represent: Bluetooth/beacon: local proximity, not global location

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-124`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 269; sourceText: Bluetooth/beacon: local proximity, not global location.
- acceptedOptions: Local Proximity | Not Global Location
- helperText: already-represented via precision-permission-and-accuracy-proof

142.  Represent: Manual/default location: a fallback, not current device proof

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-125`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 270; sourceText: Manual/default location: a fallback, not current device proof.
- acceptedOptions: A Fallback | Not Current Device Proof
- helperText: manual-required via precision-permission-and-accuracy-proof

143.  Represent: Fused provider: OS/provider chooses from several sources

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-126`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 271; sourceText: Fused provider: OS/provider chooses from several sources.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

144.  Represent: `accuracyMeters`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-127`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 275; sourceText: `accuracyMeters`
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

145.  Represent: `altitudeAccuracyMeters` when known

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-128`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 276; sourceText: `altitudeAccuracyMeters` when known
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

146.  Represent: `sourceKinds`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-129`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 277; sourceText: `sourceKinds`
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

147.  Represent: `confidence`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-131`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 279; sourceText: `confidence`
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

148.  Represent: `isUserEnteredDefault`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-134`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 282; sourceText: `isUserEnteredDefault`
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

149.  Represent: `isSimulatedOrDeveloperMode` if detectable

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-135`
- policyLane: `enforcement`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 283; sourceText: `isSimulatedOrDeveloperMode` if detectable
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

150.  Represent: Device offline -> show last contact and queue parent request

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-211`
- policyLane: `enforcement`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 453; sourceText: Device offline -> show last contact and queue parent request.
- acceptedOptions: Device Offline Show Last Contact | Queue Parent Request
- helperText: degraded via location-capability-registry

151.  Represent: Parent relay unavailable -> continue local policy and send when reachable

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-214`
- policyLane: `enforcement`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 456; sourceText: Parent relay unavailable -> continue local policy and send when reachable.
- acceptedOptions: Parent Relay Unavailable Continue Local Policy | Send When Reachable
- helperText: future-gap via authenticated-relay-proof-without-default-location-history-storage

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision

152.  Represent: Typed policy target

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-303`
- policyLane: `enforcement`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 689; sourceText: Typed policy target.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

153.  Represent: Explicit fallback when proof is stale or unavailable

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-305`
- policyLane: `enforcement`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 691; sourceText: Explicit fallback when proof is stale or unavailable.
- acceptedOptions: Explicit Fallback When Proof Is Stale | Unavailable
- helperText: degraded via location-capability-registry

154.  Represent: Local child-agent evaluation

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-306`
- policyLane: `enforcement`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 692; sourceText: Local child-agent evaluation.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

155.  Represent: Portal-side policy evaluation

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-307`
- policyLane: `enforcement`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `unavailable`; runtimeOwner: `portal-only`; capabilityState: `unavailable`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 696; sourceText: Portal-side policy evaluation.
- acceptedOptions: Represented | Not Represented
- helperText: unavailable via location-capability-registry

156.  Represent: Guessing current location from stale last-known evidence

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-308`
- policyLane: `enforcement`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `unavailable`; runtimeOwner: `child-agent`; capabilityState: `unavailable`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 697; sourceText: Guessing current location from stale last-known evidence.
- acceptedOptions: Represented | Not Represented
- helperText: unavailable via location-provider-freshness-accuracy-proof

## Tab: reports

### last-known

#### last-known-controls

157.  Show last known location on the map?

- settingId: `lastKnown.showOnMap`
- policyLane: `reports`; sectionId: `last-known`; groupId: `last-known-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 351; sourceText: Show last known location on the map?
- acceptedOptions: Enabled | Disabled
- helperText: already-represented via location-capability-registry

158.  When should a point become stale?

- settingId: `lastKnown.staleAfterMinutes`
- policyLane: `reports`; sectionId: `last-known`; groupId: `last-known-controls`
- cardKind: `number-card`; selectionMode: `single`; controlKind: `number`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 358; sourceText: When should a point become stale?
- acceptedOptions: Represented | Not Represented
- helperText: degraded via location-capability-registry

159.  What should the UI show when location is stale?

- settingId: `lastKnown.whenStale`
- policyLane: `reports`; sectionId: `last-known`; groupId: `last-known-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 367; sourceText: What should the UI show when location is stale?
- acceptedOptions: Hide Point | Show Stale | Show Stale With Contact State | Ask Check In | Notify Parent
- helperText: degraded via location-capability-registry

### tracking-guide-the-main-capability-truth

#### tracking-guide-the-main-capability-truth-the-main-capability-truth

160.  Represent: Last known location is not proof of current location

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-056`
- policyLane: `reports`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 142; sourceText: Last known location is not proof of current location.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

161.  Represent: Offline devices can only report last known location and last contact time

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-058`
- policyLane: `reports`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 144; sourceText: Offline devices can only report last known location and last contact time.
- acceptedOptions: Offline Devices Can Only Report Last Known Location | Last Contact Time
- helperText: degraded via location-capability-registry

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

162.  Represent: Capability matrix row | Capability=Family map | Mobile child agent=Yes from latest evidence | Desktop/laptop child agent=Yes from latest evidence | Required layer=Query/read model plus map rendering | Important limit=Map must label freshness and accuracy

- settingId: `tracking-guide-capability-matrix-capability-matrix-070`
- policyLane: `reports`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 163; sourceText: Capability matrix row | Capability=Family map | Mobile child agent=Yes from latest evidence | Desktop/laptop child agent=Yes from latest evidence | Required layer=Query/read model plus map rendering | Important limit=Map must label freshness and accuracy.
- acceptedOptions: Capability Family Map | Mobile Child Agent Yes From Latest Evidence | Desktop Laptop Child Agent Yes From Latest Evidence | Required Layer Query Read Model Plus Map Rendering | Important Limit Map Must Label Freshness And Accuracy
- helperText: already-represented via precision-permission-and-accuracy-proof

163.  Represent: Capability matrix row | Capability=Enforce location-based policy | Mobile child agent=Possible after proof | Desktop/laptop child agent=Limited | Required layer=Local policy plus location evidence | Important limit=Missing proof must degrade to ask/warn/report

- settingId: `tracking-guide-capability-matrix-capability-matrix-075`
- policyLane: `reports`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 168; sourceText: Capability matrix row | Capability=Enforce location-based policy | Mobile child agent=Possible after proof | Desktop/laptop child agent=Limited | Required layer=Local policy plus location evidence | Important limit=Missing proof must degrade to ask/warn/report.
- acceptedOptions: Capability Enforce Location Based Policy | Mobile Child Agent Possible After Proof | Desktop Laptop Child Agent Limited | Required Layer Local Policy Plus Location Evidence | Important Limit Missing Proof Must Degrade To Ask Warn Report
- helperText: degraded via location-capability-registry

### tracking-guide-accuracy-sources-and-limits

#### tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits

164.  Represent: `freshnessSeconds`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-130`
- policyLane: `reports`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 278; sourceText: `freshnessSeconds`
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### tracking-guide-device-online-offline-and-battery-state

#### tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state

165.  Represent: Network type and reachability summary

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-161`
- policyLane: `reports`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 355; sourceText: Network type and reachability summary.
- acceptedOptions: Network Type | Reachability Summary
- helperText: degraded via background-location-permission-and-disclosure-proof

166.  Represent: Offline: show last contact time and last known point

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-166`
- policyLane: `reports`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `child-agent`; capabilityState: `degraded`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 363; sourceText: Offline: show last contact time and last known point.
- acceptedOptions: Show Last Contact Time | Last Known Point
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-reports-and-maps

#### tracking-guide-reports-and-maps-reports-and-maps

167.  Represent: Show freshness on every point

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-170`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 374; sourceText: Show freshness on every point.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

168.  Represent: Let parents reveal exact coordinates only when the data scope permits it

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-174`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 378; sourceText: Let parents reveal exact coordinates only when the data scope permits it.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

169.  Represent: Keep summaries useful without requiring raw trail exposure

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-175`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 379; sourceText: Keep summaries useful without requiring raw trail exposure.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

170.  Represent: Recent location status by child/device

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-177`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 384; sourceText: Recent location status by child/device.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

171.  Represent: Check-in timeline

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-180`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 387; sourceText: Check-in timeline.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

172.  Represent: Device offline and battery timeline

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-181`
- policyLane: `reports`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 388; sourceText: Device offline and battery timeline.
- acceptedOptions: Device Offline | Battery Timeline
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

173.  Represent: Current location unavailable -> show last known location with timestamp

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-207`
- policyLane: `reports`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 448; sourceText: Current location unavailable -> show last known location with timestamp.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

### tracking-guide-platform-capability-notes

#### tracking-guide-platform-capability-notes-macos

174.  Represent: Do not assume Windows service behavior maps to macOS

- settingId: `tracking-guide-platform-capability-notes-macos-231`
- policyLane: `reports`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 507; sourceText: Do not assume Windows service behavior maps to macOS.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

#### tracking-guide-platform-capability-notes-android

175.  Represent: Fused Location Provider for last known, current, and periodic updates

- settingId: `tracking-guide-platform-capability-notes-android-244`
- policyLane: `reports`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 543; sourceText: Fused Location Provider for last known, current, and periodic updates.
- acceptedOptions: Fused Location Provider For Last Known | Current | Periodic Updates
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location

176.  Represent: Show newest location evidence when available

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-265`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 592; sourceText: Show newest location evidence when available.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

177.  Represent: Do not run continuous tracking

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-267`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 594; sourceText: Do not run continuous tracking.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-capability-registry

178.  Represent: Live relay

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-270`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `child-agent`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 600; sourceText: Live relay.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

179.  Represent: Real-time movement

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-271`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 604; sourceText: Real-time movement.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

#### tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode

180.  Represent: Agent tries to provide current or last known location

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-297`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `child-agent`; capabilityState: `available`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 668; sourceText: Agent tries to provide current or last known location.
- acceptedOptions: Agent Tries To Provide Current | Last Known Location
- helperText: needs-effect-wiring via location-capability-registry

#### tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision

181.  Represent: Evidence freshness and accuracy thresholds

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision-304`
- policyLane: `reports`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-location-based-policy-decision`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 690; sourceText: Evidence freshness and accuracy thresholds.
- acceptedOptions: Evidence Freshness | Accuracy Thresholds
- helperText: proof-required via precision-permission-and-accuracy-proof

### tracking-guide-current-ocentra-parent-posture

#### tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

182.  Represent: Parent surfaces author rules and view reports; they do not execute child capture or policy

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-310`
- policyLane: `reports`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 705; sourceText: Parent surfaces author rules and view reports; they do not execute child capture or policy.
- acceptedOptions: Parent Surfaces Author Rules | View Reports They Do Not Execute Child Capture | Policy
- helperText: already-represented via location-capability-registry

183.  Represent: [`docs/product-roadmap.md`](product-roadmap.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-315`
- policyLane: `reports`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 719; sourceText: [`docs/product-roadmap.md`](product-roadmap.md)
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-capability-registry

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

184.  Represent: Show last known location separately from current/live location

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-322`
- policyLane: `reports`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 731; sourceText: Show last known location separately from current/live location.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-provider-freshness-accuracy-proof

## Tab: setup

### permissions

#### permissions-controls

185.  What location permission is required?

- settingId: `permissions.minimumPermission`
- policyLane: `setup`; sectionId: `permissions`; groupId: `permissions-controls`
- cardKind: `single-choice-many`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 238; sourceText: What location permission is required?
- acceptedOptions: None | Foreground Approximate | Foreground Precise | Background Approximate | Background Precise | Supervised Or Device Owner | Platform Managed Lost Mode
- helperText: permission-required via location-capability-registry

186.  What should happen if permission is missing?

- settingId: `permissions.whenPermissionMissing`
- policyLane: `setup`; sectionId: `permissions`; groupId: `permissions-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 254; sourceText: What should happen if permission is missing?
- acceptedOptions: Show Setup Required | Fallback To Check In | Fallback To Last Known | Report Unavailable | Ask Parent | Disable Location Features
- helperText: permission-required via location-capability-registry

187.  Allow approximate location when precise is not granted?

- settingId: `permissions.allowApproximateFallback`
- policyLane: `setup`; sectionId: `permissions`; groupId: `permissions-controls`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 269; sourceText: Allow approximate location when precise is not granted?
- acceptedOptions: Enabled | Disabled
- helperText: permission-required via precision-permission-and-accuracy-proof

188.  What should the child device disclose?

- settingId: `permissions.childDisclosure`
- policyLane: `setup`; sectionId: `permissions`; groupId: `permissions-controls`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-schema-proposal.md
- sourceLine: 276; sourceText: What should the child device disclose?
- acceptedOptions: None | Show Enabled | Show Mode And Last Sample | Show Live Session Active | Show Background Tracking Active
- helperText: permission-required via location-capability-registry

### tracking-guide-core-terms

#### tracking-guide-core-terms-device-location-evidence

189.  Represent: Permission state: denied, foreground-only, background, approximate, precise, reduced-accuracy, supervised/managed, or unknown

- settingId: `tracking-guide-core-terms-device-location-evidence-029`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `multi-choice-many`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 33; sourceText: Permission state: denied, foreground-only, background, approximate, precise, reduced-accuracy, supervised/managed, or unknown.
- acceptedOptions: Denied | Foreground Only | Background | Approximate | Precise | Reduced Accuracy | Supervised Managed | Unknown
- helperText: permission-required via background-location-permission-and-disclosure-proof

190.  Represent: Device state: online, offline, low power, battery saver, airplane mode, no signal, service disabled, or adapter unavailable

- settingId: `tracking-guide-core-terms-device-location-evidence-030`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `single-choice-many`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 35; sourceText: Device state: online, offline, low power, battery saver, airplane mode, no signal, service disabled, or adapter unavailable.
- acceptedOptions: Online | Offline | Low Power | Battery Saver | Airplane Mode | No Signal | Service Disabled | Adapter Unavailable
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### tracking-guide-core-terms-check-in

191.  Represent: Background location is unavailable or not appropriate

- settingId: `tracking-guide-core-terms-check-in-038`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-check-in`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 89; sourceText: Background location is unavailable or not appropriate.
- acceptedOptions: Background Location Is Unavailable | Not Appropriate
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### tracking-guide-core-terms-location-capability-state

192.  Represent: `ready-precise-background`

- settingId: `tracking-guide-core-terms-location-capability-state-042`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 111; sourceText: `ready-precise-background`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

193.  Represent: `ready-foreground-only`

- settingId: `tracking-guide-core-terms-location-capability-state-043`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 112; sourceText: `ready-foreground-only`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

194.  Represent: `ready-approximate-only`

- settingId: `tracking-guide-core-terms-location-capability-state-044`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 113; sourceText: `ready-approximate-only`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

195.  Represent: `permission-required`

- settingId: `tracking-guide-core-terms-location-capability-state-045`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 114; sourceText: `permission-required`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

196.  Represent: `background-permission-required`

- settingId: `tracking-guide-core-terms-location-capability-state-046`
- policyLane: `setup`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 115; sourceText: `background-permission-required`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

### tracking-guide-the-main-capability-truth

#### tracking-guide-the-main-capability-truth-the-main-capability-truth

197.  Represent: Location permission is user-visible and revocable

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-053`
- policyLane: `setup`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 138; sourceText: Location permission is user-visible and revocable.
- acceptedOptions: Location Permission Is User Visible | Revocable
- helperText: permission-required via location-capability-registry

198.  Represent: Background location is a separate capability from foreground location

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-054`
- policyLane: `setup`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 139; sourceText: Background location is a separate capability from foreground location.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

199.  Represent: Approximate/reduced accuracy must be represented separately from precise location

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-055`
- policyLane: `setup`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 140; sourceText: Approximate/reduced accuracy must be represented separately from precise location.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

200.  Represent: Capability matrix row | Capability=One-time current location | Mobile child agent=Yes, with permission | Desktop/laptop child agent=Sometimes, with permission | Required layer=OS location API | Important limit=Fresh fix may fail indoors, offline, or with service off

- settingId: `tracking-guide-capability-matrix-capability-matrix-062`
- policyLane: `setup`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 155; sourceText: Capability matrix row | Capability=One-time current location | Mobile child agent=Yes, with permission | Desktop/laptop child agent=Sometimes, with permission | Required layer=OS location API | Important limit=Fresh fix may fail indoors, offline, or with service off.
- acceptedOptions: Capability One Time Current Location | Mobile Child Agent Yes With Permission | Desktop Laptop Child Agent Sometimes With Permission | Required Layer Os Location Api | Important Limit Fresh Fix May Fail Indoors Offline Or With Service Off
- helperText: permission-required via location-provider-freshness-accuracy-proof

201.  Represent: Capability matrix row | Capability=Last known location | Mobile child agent=Yes, if provider cache exists | Desktop/laptop child agent=Sometimes | Required layer=OS location API or local cache | Important limit=May be stale, null, approximate, or user-entered

- settingId: `tracking-guide-capability-matrix-capability-matrix-063`
- policyLane: `setup`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 156; sourceText: Capability matrix row | Capability=Last known location | Mobile child agent=Yes, if provider cache exists | Desktop/laptop child agent=Sometimes | Required layer=OS location API or local cache | Important limit=May be stale, null, approximate, or user-entered.
- acceptedOptions: Capability Last Known Location | Mobile Child Agent Yes If Provider Cache Exists | Desktop Laptop Child Agent Sometimes | Required Layer Os Location Api Or Local Cache | Important Limit May Be Stale Null Approximate Or User Entered
- helperText: permission-required via precision-permission-and-accuracy-proof

202.  Represent: Capability matrix row | Capability=Check-in with current location | Mobile child agent=Yes, with prompt and permission | Desktop/laptop child agent=Yes, if current fix available | Required layer=Notification/prompt plus location API | Important limit=Child must respond unless automatic policy exists

- settingId: `tracking-guide-capability-matrix-capability-matrix-068`
- policyLane: `setup`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 161; sourceText: Capability matrix row | Capability=Check-in with current location | Mobile child agent=Yes, with prompt and permission | Desktop/laptop child agent=Yes, if current fix available | Required layer=Notification/prompt plus location API | Important limit=Child must respond unless automatic policy exists.
- acceptedOptions: Capability Check In With Current Location | Mobile Child Agent Yes With Prompt And Permission | Desktop Laptop Child Agent Yes If Current Fix Available | Required Layer Notification Prompt Plus Location Api | Important Limit Child Must Respond Unless Automatic Policy Exists
- helperText: permission-required via location-provider-freshness-accuracy-proof

### tracking-guide-location-history-what-is-possible

#### tracking-guide-location-history-what-is-possible-location-history-what-is-possible

203.  Represent: Raw precise trail: disabled by default unless explicitly enabled

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-105`
- policyLane: `setup`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 227; sourceText: Raw precise trail: disabled by default unless explicitly enabled.
- acceptedOptions: Enabled | Disabled
- helperText: permission-required via precision-permission-and-accuracy-proof

204.  Represent: Accuracy, freshness, and permission state

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-107`
- policyLane: `setup`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 232; sourceText: Accuracy, freshness, and permission state.
- acceptedOptions: Accuracy | Freshness | Permission State
- helperText: permission-required via precision-permission-and-accuracy-proof

### tracking-guide-device-location-permissions

#### tracking-guide-device-location-permissions-device-location-permissions

205.  Represent: No permission

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-111`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 241; sourceText: No permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

206.  Represent: Foreground/when-in-use permission

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-112`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 242; sourceText: Foreground/when-in-use permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

207.  Represent: Background/always permission

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-113`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 243; sourceText: Background/always permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

208.  Represent: Approximate or reduced-accuracy permission

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-114`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 244; sourceText: Approximate or reduced-accuracy permission.
- acceptedOptions: Approximate | Reduced Accuracy Permission
- helperText: permission-required via precision-permission-and-accuracy-proof

209.  Represent: Precise/full-accuracy permission

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-115`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 245; sourceText: Precise/full-accuracy permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

210.  Represent: OS location service disabled

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-116`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 246; sourceText: OS location service disabled.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

211.  Represent: Device policy allowed, denied, or user-in-control

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-117`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 247; sourceText: Device policy allowed, denied, or user-in-control.
- acceptedOptions: Device Policy Allowed | Denied | User In Control
- helperText: permission-required via location-capability-registry

212.  Represent: Supervised/device-owner/MDM-only state

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-118`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 248; sourceText: Supervised/device-owner/MDM-only state.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

213.  Represent: Unknown or stale state

- settingId: `tracking-guide-device-location-permissions-device-location-permissions-119`
- policyLane: `setup`; sectionId: `tracking-guide-device-location-permissions`; groupId: `tracking-guide-device-location-permissions-device-location-permissions`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 249; sourceText: Unknown or stale state.
- acceptedOptions: Unknown | Stale State
- helperText: permission-required via location-capability-registry

### tracking-guide-accuracy-sources-and-limits

#### tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits

214.  Represent: Wi-Fi: strong for urban/indoor approximate positioning when databases and nearby access points are available

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-121`
- policyLane: `setup`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 264; sourceText: Wi-Fi: strong for urban/indoor approximate positioning when databases and nearby access points are available.
- acceptedOptions: Strong For Urban Indoor Approximate Positioning When Databases | Nearby Access Points Are Available
- helperText: permission-required via precision-permission-and-accuracy-proof

215.  Represent: `isApproximate`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-132`
- policyLane: `setup`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 280; sourceText: `isApproximate`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

216.  Represent: `isPrecise`

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-133`
- policyLane: `setup`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-domain`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 281; sourceText: `isPrecise`
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

### tracking-guide-device-online-offline-and-battery-state

#### tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state

217.  Represent: Permission/service state

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-162`
- policyLane: `setup`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 356; sourceText: Permission/service state.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

218.  Represent: Online with permission: show fresh or actively updating state

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-164`
- policyLane: `setup`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 361; sourceText: Online with permission: show fresh or actively updating state.
- acceptedOptions: Show Fresh | Actively Updating State
- helperText: permission-required via background-location-permission-and-disclosure-proof

219.  Represent: Online without permission: show permission-required state and last known point

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-165`
- policyLane: `setup`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 362; sourceText: Online without permission: show permission-required state and last known point.
- acceptedOptions: Show Permission Required State | Last Known Point
- helperText: permission-required via background-location-permission-and-disclosure-proof

220.  Represent: Service disabled: show location-service-disabled, not "tracking failed"

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-168`
- policyLane: `setup`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `child-agent`; capabilityState: `permission-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 365; sourceText: Service disabled: show location-service-disabled, not "tracking failed".
- acceptedOptions: Show Location Service Disabled | Not Tracking Failed
- helperText: permission-required via background-location-permission-and-disclosure-proof

### tracking-guide-reports-and-maps

#### tracking-guide-reports-and-maps-reports-and-maps

221.  Represent: Distinguish approximate from precise

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-172`
- policyLane: `setup`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 376; sourceText: Distinguish approximate from precise.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

222.  Represent: Location permission health

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-182`
- policyLane: `setup`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 389; sourceText: Location permission health.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

### tracking-guide-child-facing-disclosure

#### tracking-guide-child-facing-disclosure-child-facing-disclosure

223.  Represent: Location controls disabled

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-200`
- policyLane: `setup`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 431; sourceText: Location controls disabled.
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

224.  Represent: Check-in only

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-201`
- policyLane: `setup`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 432; sourceText: Check-in only.
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

225.  Represent: Background location enabled by parent and OS permission

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-204`
- policyLane: `setup`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 435; sourceText: Background location enabled by parent and OS permission.
- acceptedOptions: Background Location Enabled By Parent | Os Permission
- helperText: permission-required via background-location-permission-and-disclosure-proof

226.  Represent: Last-known report only because device is offline

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-205`
- policyLane: `setup`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 436; sourceText: Last-known report only because device is offline.
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

227.  Represent: Location unavailable because permission/service is off

- settingId: `tracking-guide-child-facing-disclosure-child-facing-disclosure-206`
- policyLane: `setup`; sectionId: `tracking-guide-child-facing-disclosure`; groupId: `tracking-guide-child-facing-disclosure-child-facing-disclosure`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 437; sourceText: Location unavailable because permission/service is off.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

228.  Represent: Background permission missing -> offer foreground/check-in mode

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-208`
- policyLane: `setup`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 449; sourceText: Background permission missing -> offer foreground/check-in mode.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

229.  Represent: Precise permission denied -> use approximate-only rules or mark precise rules unavailable

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-210`
- policyLane: `setup`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 451; sourceText: Precise permission denied -> use approximate-only rules or mark precise rules unavailable.
- acceptedOptions: Precise Permission Denied Use Approximate Only Rules | Mark Precise Rules Unavailable
- helperText: permission-required via precision-permission-and-accuracy-proof

### tracking-guide-platform-capability-notes

#### tracking-guide-platform-capability-notes-windows

230.  Represent: `Windows.Devices.Geolocation.Geolocator` for one-time and continuous location where the app has permission

- settingId: `tracking-guide-platform-capability-notes-windows-215`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 471; sourceText: `Windows.Devices.Geolocation.Geolocator` for one-time and continuous location where the app has permission.
- acceptedOptions: Windows Devices Geolocation Geolocator For One Time | Continuous Location Where The App Has Permission
- helperText: permission-required via location-capability-registry

231.  Represent: Wi-Fi BSSID access increasingly tied to precise-location consent

- settingId: `tracking-guide-platform-capability-notes-windows-218`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 476; sourceText: Wi-Fi BSSID access increasingly tied to precise-location consent.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

232.  Represent: Location can be approximate, IP-derived, stale, manually configured, or unavailable

- settingId: `tracking-guide-platform-capability-notes-windows-221`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `manual-proof`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 482; sourceText: Location can be approximate, IP-derived, stale, manually configured, or unavailable.
- acceptedOptions: Location Can Be Approximate | Ip Derived | Stale | Manually Configured | Unavailable
- helperText: permission-required via precision-permission-and-accuracy-proof

233.  Represent: A background Windows service cannot assume Store-app-style foreground consent semantics without implementation proof

- settingId: `tracking-guide-platform-capability-notes-windows-222`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 484; sourceText: A background Windows service cannot assume Store-app-style foreground consent semantics without implementation proof.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### tracking-guide-platform-capability-notes-macos

234.  Represent: TCC permissions, background execution, launchd behavior, signing, and notarization matter

- settingId: `tracking-guide-platform-capability-notes-macos-230`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 505; sourceText: TCC permissions, background execution, launchd behavior, signing, and notarization matter.
- acceptedOptions: Tcc Permissions | Background Execution | Launchd Behavior | Signing | Notarization Matter
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### tracking-guide-platform-capability-notes-linux

235.  Represent: IP/Wi-Fi based approximate location if the service/provider is available

- settingId: `tracking-guide-platform-capability-notes-linux-234`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 519; sourceText: IP/Wi-Fi based approximate location if the service/provider is available.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

236.  Represent: Browser geolocation with user permission for web surfaces, separate from the child agent

- settingId: `tracking-guide-platform-capability-notes-linux-235`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 520; sourceText: Browser geolocation with user permission for web surfaces, separate from the child agent.
- acceptedOptions: Browser Geolocation With User Permission For Web Surfaces | Separate From The Child Agent
- helperText: permission-required via location-capability-registry

237.  Represent: Permission agents and desktop portals vary

- settingId: `tracking-guide-platform-capability-notes-linux-238`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `portal-only`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 527; sourceText: Permission agents and desktop portals vary.
- acceptedOptions: Permission Agents | Desktop Portals Vary
- helperText: permission-required via location-capability-registry

#### tracking-guide-platform-capability-notes-android

238.  Represent: Foreground location with `ACCESS_COARSE_LOCATION` and/or `ACCESS_FINE_LOCATION`

- settingId: `tracking-guide-platform-capability-notes-android-241`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 538; sourceText: Foreground location with `ACCESS_COARSE_LOCATION` and/or `ACCESS_FINE_LOCATION`.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

239.  Represent: Background location with `ACCESS_BACKGROUND_LOCATION` when core functionality and Play policy allow it

- settingId: `tracking-guide-platform-capability-notes-android-242`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 540; sourceText: Background location with `ACCESS_BACKGROUND_LOCATION` when core functionality and Play policy allow it.
- acceptedOptions: Background Location With Access Background Location When Core Functionality | Play Policy Allow It
- helperText: permission-required via background-location-permission-and-disclosure-proof

240.  Represent: Approximate vs precise permission state

- settingId: `tracking-guide-platform-capability-notes-android-243`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 542; sourceText: Approximate vs precise permission state.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

241.  Represent: Foreground service and visible notification for long-running location use where required

- settingId: `tracking-guide-platform-capability-notes-android-246`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 545; sourceText: Foreground service and visible notification for long-running location use where required.
- acceptedOptions: Foreground Service | Visible Notification For Long Running Location Use Where Required
- helperText: permission-required via background-location-permission-and-disclosure-proof

242.  Represent: Background location is restricted and must be core to the app

- settingId: `tracking-guide-platform-capability-notes-android-248`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 552; sourceText: Background location is restricted and must be core to the app.
- acceptedOptions: Background Location Is Restricted | Must Be Core To The App
- helperText: permission-required via background-location-permission-and-disclosure-proof

243.  Represent: Background updates are throttled

- settingId: `tracking-guide-platform-capability-notes-android-249`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 553; sourceText: Background updates are throttled.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

244.  Represent: Approximate permission may be the only granted precision

- settingId: `tracking-guide-platform-capability-notes-android-251`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 555; sourceText: Approximate permission may be the only granted precision.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via precision-permission-and-accuracy-proof

245.  Represent: Users can change permission, precision, and location service settings

- settingId: `tracking-guide-platform-capability-notes-android-252`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 556; sourceText: Users can change permission, precision, and location service settings.
- acceptedOptions: Users Can Change Permission | Precision | Location Service Settings
- helperText: permission-required via location-capability-registry

#### tracking-guide-platform-capability-notes-ios-and-ipados

246.  Represent: Background location with required mode, authorization, and user disclosure

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-257`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 569; sourceText: Background location with required mode, authorization, and user disclosure.
- acceptedOptions: Background Location With Required Mode | Authorization | User Disclosure
- helperText: permission-required via background-location-permission-and-disclosure-proof

247.  Represent: Always/background location has explicit prompts and disclosure requirements

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-261`
- policyLane: `setup`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 577; sourceText: Always/background location has explicit prompts and disclosure requirements.
- acceptedOptions: Always Background Location Has Explicit Prompts | Disclosure Requirements
- helperText: permission-required via background-location-permission-and-disclosure-proof

### tracking-guide-policy-modes-to-represent-later-in-ui

#### tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location

248.  Represent: Show freshness, accuracy, source, permission, and custody

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-266`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `permission-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 593; sourceText: Show freshness, accuracy, source, permission, and custody.
- acceptedOptions: Show Freshness | Accuracy | Source | Permission | Custody
- helperText: permission-required via precision-permission-and-accuracy-proof

249.  Represent: Background permission

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location-268`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-observe-last-known-location`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 598; sourceText: Background permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via background-location-permission-and-disclosure-proof

#### tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only

250.  Represent: Agent may include current location if permission allows

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-274`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 612; sourceText: Agent may include current location if permission allows.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

251.  Represent: Foreground prompt/notification

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-276`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 617; sourceText: Foreground prompt/notification.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

252.  Represent: Optional location permission

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-277`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 618; sourceText: Optional location permission.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-capability-registry

253.  Represent: Current location when permission/service is unavailable

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only-279`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-check-in-only`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 623; sourceText: Current location when permission/service is unavailable.
- acceptedOptions: Represented | Not Represented
- helperText: permission-required via location-provider-freshness-accuracy-proof

#### tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode

254.  Represent: Existing permission or platform-specific lost-device capability

- settingId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode-299`
- policyLane: `setup`; sectionId: `tracking-guide-policy-modes-to-represent-later-in-ui`; groupId: `tracking-guide-policy-modes-to-represent-later-in-ui-missing-device-mode`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `permission-required`; runtimeOwner: `os-adapter`; capabilityState: `permission-required`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 673; sourceText: Existing permission or platform-specific lost-device capability.
- acceptedOptions: Existing Permission | Platform Specific Lost Device Capability
- helperText: permission-required via location-capability-registry

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

255.  Represent: Show approximate/reduced accuracy as a first-class state

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-324`
- policyLane: `setup`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 733; sourceText: Show approximate/reduced accuracy as a first-class state.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via precision-permission-and-accuracy-proof

256.  Represent: Keep permission state close to every control

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-327`
- policyLane: `setup`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location controls require explicit OS permission state, precision state, and child-facing disclosure evidence.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 736; sourceText: Keep permission state close to every control.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via location-capability-registry

## Tab: platform

### tracking-guide-core-terms

#### tracking-guide-core-terms-device-location-evidence

257.  Represent: Timestamp from the platform provider and ingest timestamp from Ocentra

- settingId: `tracking-guide-core-terms-device-location-evidence-027`
- policyLane: `platform`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 30; sourceText: Timestamp from the platform provider and ingest timestamp from Ocentra.
- acceptedOptions: Timestamp From The Platform Provider | Ingest Timestamp From Ocentra
- helperText: already-represented via location-capability-registry

#### tracking-guide-core-terms-location-capability-state

258.  Represent: `platform-unsupported`

- settingId: `tracking-guide-core-terms-location-capability-state-048`
- policyLane: `platform`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-capability-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 117; sourceText: `platform-unsupported`
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

259.  Represent: Capability matrix row | Capability=Lost-device location | Mobile child agent=OS/product-specific | Desktop/laptop child agent=OS/product-specific | Required layer=OS lost mode/Find My/device management | Important limit=Not a generic third-party API on every platform

- settingId: `tracking-guide-capability-matrix-capability-matrix-073`
- policyLane: `platform`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 166; sourceText: Capability matrix row | Capability=Lost-device location | Mobile child agent=OS/product-specific | Desktop/laptop child agent=OS/product-specific | Required layer=OS lost mode/Find My/device management | Important limit=Not a generic third-party API on every platform.
- acceptedOptions: Capability Lost Device Location | Mobile Child Agent Os Product Specific | Desktop Laptop Child Agent Os Product Specific | Required Layer Os Lost Mode Find My Device Management | Important Limit Not A Generic Third Party Api On Every Platform
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

### tracking-guide-accuracy-sources-and-limits

#### tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits

260.  Represent: Cellular: useful wide-area estimate on mobile devices

- settingId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits-122`
- policyLane: `platform`; sectionId: `tracking-guide-accuracy-sources-and-limits`; groupId: `tracking-guide-accuracy-sources-and-limits-accuracy-sources-and-limits`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 266; sourceText: Cellular: useful wide-area estimate on mobile devices.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

### tracking-guide-missing-proof-fallbacks

#### tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks

261.  Represent: Platform unsupported -> show unavailable/manual-required, not a fake toggle

- settingId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks-213`
- policyLane: `platform`; sectionId: `tracking-guide-missing-proof-fallbacks`; groupId: `tracking-guide-missing-proof-fallbacks-missing-proof-fallbacks`
- cardKind: `toggle`; selectionMode: `single`; controlKind: `toggle`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 455; sourceText: Platform unsupported -> show unavailable/manual-required, not a fake toggle.
- acceptedOptions: Platform Unsupported Show Unavailable Manual Required | Not A Fake Toggle
- helperText: manual-required via location-capability-registry

### tracking-guide-platform-capability-notes

#### tracking-guide-platform-capability-notes-windows

262.  Represent: Windows location service using GPS, Wi-Fi, cell towers, and IP where available

- settingId: `tracking-guide-platform-capability-notes-windows-216`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 473; sourceText: Windows location service using GPS, Wi-Fi, cell towers, and IP where available.
- acceptedOptions: Windows Location Service Using Gps | Wi Fi | Cell Towers | Ip Where Available
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

263.  Represent: Windows privacy settings and MDM/Policy CSP for whether Windows apps may access location

- settingId: `tracking-guide-platform-capability-notes-windows-217`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 474; sourceText: Windows privacy settings and MDM/Policy CSP for whether Windows apps may access location.
- acceptedOptions: Windows Privacy Settings | Mdm Policy Csp For Whether Windows Apps May Access Location
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

264.  Represent: Child-agent service/contact state independent of location service state

- settingId: `tracking-guide-platform-capability-notes-windows-219`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 477; sourceText: Child-agent service/contact state independent of location service state.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

265.  Represent: Desktop/laptop hardware may have no GPS or cellular radio

- settingId: `tracking-guide-platform-capability-notes-windows-220`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 481; sourceText: Desktop/laptop hardware may have no GPS or cellular radio.
- acceptedOptions: Desktop Laptop Hardware May Have No Gps | Cellular Radio
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

266.  Represent: Product claims need real Windows adapter proof, not only a contract

- settingId: `tracking-guide-platform-capability-notes-windows-224`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-windows`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `os-adapter`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 487; sourceText: Product claims need real Windows adapter proof, not only a contract.
- acceptedOptions: Product Claims Need Real Windows Adapter Proof | Not Only A Contract
- helperText: proof-required via location-capability-registry

#### tracking-guide-platform-capability-notes-macos

267.  Represent: Core Location authorization and location updates

- settingId: `tracking-guide-platform-capability-notes-macos-225`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 495; sourceText: Core Location authorization and location updates.
- acceptedOptions: Core Location Authorization | Location Updates
- helperText: needs-effect-wiring via location-capability-registry

268.  Represent: Reduced/full accuracy state

- settingId: `tracking-guide-platform-capability-notes-macos-226`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 496; sourceText: Reduced/full accuracy state.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

269.  Represent: Find My Mac as an Apple account feature, not a generic third-party tracking API

- settingId: `tracking-guide-platform-capability-notes-macos-228`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 499; sourceText: Find My Mac as an Apple account feature, not a generic third-party tracking API.
- acceptedOptions: Find My Mac As An Apple Account Feature | Not A Generic Third Party Tracking Api
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

270.  Represent: MDM/device management posture for managed-device features where available

- settingId: `tracking-guide-platform-capability-notes-macos-229`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 501; sourceText: MDM/device management posture for managed-device features where available.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

271.  Represent: Lost-device and Find My behavior should be described as OS/account feature context unless Ocentra has an approved API and proof

- settingId: `tracking-guide-platform-capability-notes-macos-232`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-macos`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 508; sourceText: Lost-device and Find My behavior should be described as OS/account feature context unless Ocentra has an approved API and proof.
- acceptedOptions: Lost Device | Proof
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

#### tracking-guide-platform-capability-notes-linux

272.  Represent: GeoClue over D-Bus on desktops that ship/configure it

- settingId: `tracking-guide-platform-capability-notes-linux-233`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 518; sourceText: GeoClue over D-Bus on desktops that ship/configure it.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

273.  Represent: Manual/default location fallback

- settingId: `tracking-guide-platform-capability-notes-linux-236`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `manual-proof`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 522; sourceText: Manual/default location fallback.
- acceptedOptions: Represented | Not Represented
- helperText: manual-required via location-capability-registry

274.  Represent: No universal Linux live-location stack exists across distros

- settingId: `tracking-guide-platform-capability-notes-linux-237`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 526; sourceText: No universal Linux live-location stack exists across distros.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-provider-freshness-accuracy-proof

275.  Represent: Headless/service deployments may have no useful location provider

- settingId: `tracking-guide-platform-capability-notes-linux-239`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 528; sourceText: Headless/service deployments may have no useful location provider.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

276.  Represent: Product claims must name distro/service assumptions and real proof

- settingId: `tracking-guide-platform-capability-notes-linux-240`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-linux`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 529; sourceText: Product claims must name distro/service assumptions and real proof.
- acceptedOptions: Product Claims Must Name Distro Service Assumptions | Real Proof
- helperText: proof-required via location-capability-registry

#### tracking-guide-platform-capability-notes-android

277.  Represent: Device owner/profile owner policy only after real Android Enterprise or device owner proof

- settingId: `tracking-guide-platform-capability-notes-android-247`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-domain`; capabilityState: `protected`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 547; sourceText: Device owner/profile owner policy only after real Android Enterprise or device owner proof.
- acceptedOptions: Device Owner Profile Owner Policy Only After Real Android Enterprise | Device Owner Proof
- helperText: proof-required via location-capability-registry

278.  Represent: Geofencing responsiveness can be delayed

- settingId: `tracking-guide-platform-capability-notes-android-250`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 554; sourceText: Geofencing responsiveness can be delayed.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

279.  Represent: Store policy matters for a child-agent product

- settingId: `tracking-guide-platform-capability-notes-android-253`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-android`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-domain`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 557; sourceText: Store policy matters for a child-agent product.
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

#### tracking-guide-platform-capability-notes-ios-and-ipados

280.  Represent: Core Location When In Use and Always authorization

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-254`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 566; sourceText: Core Location When In Use and Always authorization.
- acceptedOptions: Core Location When In Use | Always Authorization
- helperText: needs-effect-wiring via background-location-permission-and-disclosure-proof

281.  Represent: Reduced/full accuracy state

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-255`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 567; sourceText: Reduced/full accuracy state.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

282.  Represent: Standard, significant-change, visit, and region monitoring where permitted

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-256`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 568; sourceText: Standard, significant-change, visit, and region monitoring where permitted.
- acceptedOptions: Standard | Significant Change | Visit | Region Monitoring Where Permitted
- helperText: needs-effect-wiring via location-capability-registry

283.  Represent: Family Sharing/Find My location sharing as an Apple user feature, not an Ocentra-owned raw telemetry API

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-258`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 570; sourceText: Family Sharing/Find My location sharing as an Apple user feature, not an Ocentra-owned raw telemetry API.
- acceptedOptions: Family Sharing Find My Location Sharing As An Apple User Feature | Not An Ocentra Owned Raw Telemetry Api
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

284.  Represent: Supervised MDM Lost Mode device location for managed/supervised devices only

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-259`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 572; sourceText: Supervised MDM Lost Mode device location for managed/supervised devices only.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

285.  Represent: Third-party apps cannot silently access general Find My data

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-260`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 576; sourceText: Third-party apps cannot silently access general Find My data.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

286.  Represent: Region monitoring has platform limits and is not instant

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-262`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 578; sourceText: Region monitoring has platform limits and is not instant.
- acceptedOptions: Region Monitoring Has Platform Limits | Is Not Instant
- helperText: future-gap via location-capability-registry

287.  Represent: MDM device location is tied to Lost Mode and supervision requirements

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-263`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 579; sourceText: MDM device location is tied to Lost Mode and supervision requirements.
- acceptedOptions: Mdm Device Location Is Tied To Lost Mode | Supervision Requirements
- helperText: future-gap via platform-managed-lost-mode-or-supervision-proof

288.  Represent: Entitlements, App Store review, Family Controls, and device supervision affect what is shippable

- settingId: `tracking-guide-platform-capability-notes-ios-and-ipados-264`
- policyLane: `platform`; sectionId: `tracking-guide-platform-capability-notes`; groupId: `tracking-guide-platform-capability-notes-ios-and-ipados`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 580; sourceText: Entitlements, App Store review, Family Controls, and device supervision affect what is shippable.
- acceptedOptions: Entitlements | App Store Review | Family Controls | Device Supervision Affect What Is Shippable
- helperText: future-gap via location-capability-registry

### tracking-guide-current-ocentra-parent-posture

#### tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

289.  Represent: Android and iOS capability claims are currently scaffold/manual-required until real device proof exists

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-313`
- policyLane: `platform`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `manual-required`; runtimeOwner: `os-adapter`; capabilityState: `manual-required`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 709; sourceText: Android and iOS capability claims are currently scaffold/manual-required until real device proof exists.
- acceptedOptions: Android
- helperText: manual-required via location-capability-registry

290.  Represent: Platform claims must distinguish implemented, scaffold-only, unavailable, degraded, manual-required, and not-implemented states

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-314`
- policyLane: `platform`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `future-gap`; runtimeOwner: `os-adapter`; capabilityState: `future-gap`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 711; sourceText: Platform claims must distinguish implemented, scaffold-only, unavailable, degraded, manual-required, and not-implemented states.
- acceptedOptions: Platform Claims Must Distinguish Implemented | Scaffold Only | Unavailable | Degraded | Manual Required | Not Implemented States
- helperText: future-gap via location-capability-registry

291.  Represent: [`docs/expectations/platforms.md`](expectations/platforms.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-318`
- policyLane: `platform`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `os-adapter`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 722; sourceText: [`docs/expectations/platforms.md`](expectations/platforms.md)
- acceptedOptions: Represented | Not Represented
- helperText: needs-effect-wiring via location-capability-registry

## Tab: data

### tracking-guide-core-terms

#### tracking-guide-core-terms-device-location-evidence

292.  Represent: Custody label: child-local, parent-device cache, LAN-live, parent-owned storage, Ocentra-hosted non-activity metadata, or unavailable

- settingId: `tracking-guide-core-terms-device-location-evidence-031`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-device-location-evidence`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 37; sourceText: Custody label: child-local, parent-device cache, LAN-live, parent-owned storage, Ocentra-hosted non-activity metadata, or unavailable.
- acceptedOptions: Child Local | Parent Device Cache | Lan Live | Parent Owned Storage | Ocentra Hosted Non Activity Metadata | Unavailable
- helperText: already-represented via location-provider-freshness-accuracy-proof

#### tracking-guide-core-terms-location-history

293.  Represent: Parent reports

- settingId: `tracking-guide-core-terms-location-history-032`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 63; sourceText: Parent reports.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

294.  Represent: Missed check-in investigation

- settingId: `tracking-guide-core-terms-location-history-034`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 65; sourceText: Missed check-in investigation.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

295.  Represent: Device lost/stolen review

- settingId: `tracking-guide-core-terms-location-history-035`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 66; sourceText: Device lost/stolen review.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

296.  Represent: Safety explanation after an alert

- settingId: `tracking-guide-core-terms-location-history-036`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 67; sourceText: Safety explanation after an alert.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

297.  Represent: Export/delete/retention flows

- settingId: `tracking-guide-core-terms-location-history-037`
- policyLane: `data`; sectionId: `tracking-guide-core-terms`; groupId: `tracking-guide-core-terms-location-history`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 68; sourceText: Export/delete/retention flows.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### tracking-guide-the-main-capability-truth

#### tracking-guide-the-main-capability-truth-the-main-capability-truth

298.  Represent: Parent-owned storage and local/LAN operation are the normal custody boundary

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-060`
- policyLane: `data`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 147; sourceText: Parent-owned storage and local/LAN operation are the normal custody boundary.
- acceptedOptions: Parent Owned Storage | Local Lan Operation Are The Normal Custody Boundary
- helperText: already-represented via local-history-custody-retention-proof

299.  Represent: Every strict policy or alert must carry evidence source, timestamp, accuracy, custody, and adapter state

- settingId: `tracking-guide-the-main-capability-truth-the-main-capability-truth-061`
- policyLane: `data`; sectionId: `tracking-guide-the-main-capability-truth`; groupId: `tracking-guide-the-main-capability-truth-the-main-capability-truth`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 148; sourceText: Every strict policy or alert must carry evidence source, timestamp, accuracy, custody, and adapter state.
- acceptedOptions: Every Strict Policy | Alert Must Carry Evidence Source | Timestamp | Accuracy | Custody | Adapter State
- helperText: already-represented via precision-permission-and-accuracy-proof

### tracking-guide-capability-matrix

#### tracking-guide-capability-matrix-capability-matrix

300.  Represent: Capability matrix row | Capability=Location history | Mobile child agent=Yes, if locally journaled | Desktop/laptop child agent=Yes, if locally journaled | Required layer=Agent journal/query store | Important limit=Retention/custody/delete controls required

- settingId: `tracking-guide-capability-matrix-capability-matrix-069`
- policyLane: `data`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 162; sourceText: Capability matrix row | Capability=Location history | Mobile child agent=Yes, if locally journaled | Desktop/laptop child agent=Yes, if locally journaled | Required layer=Agent journal/query store | Important limit=Retention/custody/delete controls required.
- acceptedOptions: Capability Location History | Mobile Child Agent Yes If Locally Journaled | Desktop Laptop Child Agent Yes If Locally Journaled | Required Layer Agent Journal Query Store | Important Limit Retention Custody Delete Controls Required
- helperText: already-represented via local-history-custody-retention-proof

301.  Represent: Capability matrix row | Capability=Exact continuous movement trail | Mobile child agent=Sometimes, but expensive and sensitive | Desktop/laptop child agent=Usually no | Required layer=Frequent location updates | Important limit=Battery, consent, and retention concerns

- settingId: `tracking-guide-capability-matrix-capability-matrix-072`
- policyLane: `data`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `manual-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `manual-required`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 165; sourceText: Capability matrix row | Capability=Exact continuous movement trail | Mobile child agent=Sometimes, but expensive and sensitive | Desktop/laptop child agent=Usually no | Required layer=Frequent location updates | Important limit=Battery, consent, and retention concerns.
- acceptedOptions: Capability Exact Continuous Movement Trail | Mobile Child Agent Sometimes But Expensive And Sensitive | Desktop Laptop Child Agent Usually No | Required Layer Frequent Location Updates | Important Limit Battery Consent And Retention Concerns
- helperText: manual-required via background-location-permission-and-disclosure-proof

302.  Represent: Capability matrix row | Capability=Remote live location away from LAN | Mobile child agent=Future, via relay or parent storage | Desktop/laptop child agent=Future | Required layer=Authenticated relay/sync | Important limit=Ocentra must not become default location-history store

- settingId: `tracking-guide-capability-matrix-capability-matrix-074`
- policyLane: `data`; sectionId: `tracking-guide-capability-matrix`; groupId: `tracking-guide-capability-matrix-capability-matrix`
- cardKind: `status-card`; selectionMode: `single`; controlKind: `read-only-status`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 167; sourceText: Capability matrix row | Capability=Remote live location away from LAN | Mobile child agent=Future, via relay or parent storage | Desktop/laptop child agent=Future | Required layer=Authenticated relay/sync | Important limit=Ocentra must not become default location-history store.
- acceptedOptions: Capability Remote Live Location Away From Lan | Mobile Child Agent Future Via Relay Or Parent Storage | Desktop Laptop Child Agent Future | Required Layer Authenticated Relay Sync | Important Limit Ocentra Must Not Become Default Location History Store
- helperText: future-gap via location-provider-freshness-accuracy-proof

### tracking-guide-location-history-what-is-possible

#### tracking-guide-location-history-what-is-possible-location-history-what-is-possible

303.  Represent: `location-point`: individual timestamped point with accuracy and provider

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-095`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 213; sourceText: `location-point`: individual timestamped point with accuracy and provider.
- acceptedOptions: Individual Timestamped Point With Accuracy | Provider
- helperText: already-represented via precision-permission-and-accuracy-proof

304.  Represent: `check-in`: child response, optional location point, note, and prompt reason

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-098`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 217; sourceText: `check-in`: child response, optional location point, note, and prompt reason.
- acceptedOptions: Child Response | Optional Location Point | Note | Prompt Reason
- helperText: already-represented via local-history-custody-retention-proof

305.  Represent: `device-contact`: last online, last sync, battery, and network state

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-099`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 218; sourceText: `device-contact`: last online, last sync, battery, and network state.
- acceptedOptions: Last Online | Last Sync | Battery | Network State
- helperText: degraded via background-location-permission-and-disclosure-proof

306.  Represent: `audit-only`: policy decision or parent action without raw coordinate

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-100`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 219; sourceText: `audit-only`: policy decision or parent action without raw coordinate.
- acceptedOptions: Policy Decision | Parent Action Without Raw Coordinate
- helperText: proof-required via location-provider-freshness-accuracy-proof

307.  Represent: Live point stream: short retention

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-101`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 223; sourceText: Live point stream: short retention.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

308.  Represent: Policy/audit references: longer retention

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-103`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 225; sourceText: Policy/audit references: longer retention.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

309.  Represent: Parent-exported report: parent-chosen retention

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-104`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 226; sourceText: Parent-exported report: parent-chosen retention.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

310.  Represent: Evidence source and adapter version

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-106`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 231; sourceText: Evidence source and adapter version.
- acceptedOptions: Evidence Source | Adapter Version
- helperText: already-represented via local-history-custody-retention-proof

311.  Represent: Custody and retention labels

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-108`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 233; sourceText: Custody and retention labels.
- acceptedOptions: Custody | Retention Labels
- helperText: already-represented via local-history-custody-retention-proof

312.  Represent: Redaction/deletion state

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-109`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 234; sourceText: Redaction/deletion state.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

313.  Represent: Parent reveal/audit trail for sensitive exact coordinates

- settingId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible-110`
- policyLane: `data`; sectionId: `tracking-guide-location-history-what-is-possible`; groupId: `tracking-guide-location-history-what-is-possible-location-history-what-is-possible`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `parent-owned-storage`; capabilityState: `protected`
- proofRequirement: Current or live location claims require source, permission, freshness, accuracy, custody, and adapter-state proof; last-known evidence must be labeled stale when applicable.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 235; sourceText: Parent reveal/audit trail for sensitive exact coordinates.
- acceptedOptions: Represented | Not Represented
- helperText: proof-required via location-provider-freshness-accuracy-proof

### tracking-guide-device-online-offline-and-battery-state

#### tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state

314.  Represent: Adapter error: show degraded status with audit reference

- settingId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state-169`
- policyLane: `data`; sectionId: `tracking-guide-device-online-offline-and-battery-state`; groupId: `tracking-guide-device-online-offline-and-battery-state-device-online-offline-and-battery-state`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `os-adapter`; capabilityState: `degraded`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 366; sourceText: Adapter error: show degraded status with audit reference.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via background-location-permission-and-disclosure-proof

### tracking-guide-reports-and-maps

#### tracking-guide-reports-and-maps-reports-and-maps

315.  Represent: Record parent reveal, export, delete, and retention actions

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-176`
- policyLane: `data`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 380; sourceText: Record parent reveal, export, delete, and retention actions.
- acceptedOptions: Record Parent Reveal | Export | Delete | Retention Actions
- helperText: already-represented via local-history-custody-retention-proof

316.  Represent: Exportable parent report with custody labels

- settingId: `tracking-guide-reports-and-maps-reports-and-maps-183`
- policyLane: `data`; sectionId: `tracking-guide-reports-and-maps`; groupId: `tracking-guide-reports-and-maps-reports-and-maps`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 390; sourceText: Exportable parent report with custody labels.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### tracking-guide-custody-retention-and-audit

#### tracking-guide-custody-retention-and-audit-custody-retention-and-audit

317.  Represent: Raw location evidence lives on the child device

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-184`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 399; sourceText: Raw location evidence lives on the child device.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via location-provider-freshness-accuracy-proof

318.  Represent: Parent surfaces read local/LAN, parent cache, parent-owned storage, or cloud relay through typed contracts

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-185`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 400; sourceText: Parent surfaces read local/LAN, parent cache, parent-owned storage, or cloud relay through typed contracts.
- acceptedOptions: Parent Surfaces Read Local Lan | Parent Cache | Parent Owned Storage | Cloud Relay Through Typed Contracts
- helperText: future-gap via local-history-custody-retention-proof

319.  Represent: Ocentra-hosted services may route minimal notification or relay metadata, but they must not retain raw location history by default

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-186`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `parent-owned-storage`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 402; sourceText: Ocentra-hosted services may route minimal notification or relay metadata, but they must not retain raw location history by default.
- acceptedOptions: Ocentra Hosted Services May Route Minimal Notification | Relay Metadata | But They Must Not Retain Raw Location History By Default
- helperText: future-gap via local-history-custody-retention-proof

320.  Represent: Retention policy per data class

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-187`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 407; sourceText: Retention policy per data class.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

321.  Represent: Delete expired raw points

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-188`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 408; sourceText: Delete expired raw points.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

322.  Represent: Keep redacted summaries only if parent policy allows

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-189`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 409; sourceText: Keep redacted summaries only if parent policy allows.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

323.  Represent: Export/delete flows that name data classes and destinations

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-190`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 410; sourceText: Export/delete flows that name data classes and destinations.
- acceptedOptions: Export Delete Flows That Name Data Classes | Destinations
- helperText: already-represented via local-history-custody-retention-proof

324.  Represent: Audit for parent reveal, export, sync, delete, policy change, and strict alert action

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-191`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 411; sourceText: Audit for parent reveal, export, sync, delete, policy change, and strict alert action.
- acceptedOptions: Audit For Parent Reveal | Export | Sync | Delete | Policy Change | Strict Alert Action
- helperText: already-represented via local-history-custody-retention-proof

325.  Represent: Policy decision or parent action

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-192`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `proof-required`; runtimeOwner: `child-agent`; capabilityState: `protected`
- proofRequirement: Location-based policy decisions require fresh location evidence, typed parent rule, deterministic child-agent evaluation, and explicit ask/warn/report fallback.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 416; sourceText: Policy decision or parent action.
- acceptedOptions: Policy Decision | Parent Action
- helperText: proof-required via local-history-custody-retention-proof

326.  Represent: Evidence reference

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-193`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 417; sourceText: Evidence reference.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

327.  Represent: Location source and adapter state

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-194`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `single-choice-compact`; selectionMode: `single`; controlKind: `single-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 418; sourceText: Location source and adapter state.
- acceptedOptions: Location Source | Adapter State
- helperText: already-represented via local-history-custody-retention-proof

328.  Represent: Accuracy/freshness

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-195`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 419; sourceText: Accuracy/freshness.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via precision-permission-and-accuracy-proof

329.  Represent: Custody label

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-196`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 420; sourceText: Custody label.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

330.  Represent: Retention class

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-197`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 421; sourceText: Retention class.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

331.  Represent: Notification intent reference where alerts were sent

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-198`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 422; sourceText: Notification intent reference where alerts were sent.
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

332.  Represent: Failure/degraded reason

- settingId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit-199`
- policyLane: `data`; sectionId: `tracking-guide-custody-retention-and-audit`; groupId: `tracking-guide-custody-retention-and-audit-custody-retention-and-audit`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `degraded`; runtimeOwner: `parent-owned-storage`; capabilityState: `degraded`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 423; sourceText: Failure/degraded reason.
- acceptedOptions: Represented | Not Represented
- helperText: degraded via local-history-custody-retention-proof

### tracking-guide-current-ocentra-parent-posture

#### tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture

333.  Represent: Child-device agents own capture, journal, query, local AI, policy, and enforcement paths

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-309`
- policyLane: `data`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `multi-choice-normal`; selectionMode: `multi`; controlKind: `multi-choice`
- effectStatus: `needs-effect-wiring`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: none
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 703; sourceText: Child-device agents own capture, journal, query, local AI, policy, and enforcement paths.
- acceptedOptions: Child Device Agents Own Capture | Journal | Query | Local Ai | Policy | Enforcement Paths
- helperText: needs-effect-wiring via local-history-custody-retention-proof

334.  Represent: [`docs/expectations/data-custody.md`](expectations/data-custody.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-316`
- policyLane: `data`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 720; sourceText: [`docs/expectations/data-custody.md`](expectations/data-custody.md)
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

335.  Represent: [`docs/expectations/sync-export.md`](expectations/sync-export.md)

- settingId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture-317`
- policyLane: `data`; sectionId: `tracking-guide-current-ocentra-parent-posture`; groupId: `tracking-guide-current-ocentra-parent-posture-current-ocentra-parent-posture`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `already-represented`; runtimeOwner: `parent-owned-storage`; capabilityState: `available`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 721; sourceText: [`docs/expectations/sync-export.md`](expectations/sync-export.md)
- acceptedOptions: Represented | Not Represented
- helperText: already-represented via local-history-custody-retention-proof

### tracking-guide-future-ui-rules

#### tracking-guide-future-ui-rules-future-ui-rules

336.  Represent: Keep custody, retention, and delete/export status visible for location history

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-328`
- policyLane: `data`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 737; sourceText: Keep custody, retention, and delete/export status visible for location history.
- acceptedOptions: Keep Custody | Retention | Delete Export Status Visible For Location History
- helperText: future-gap via local-history-custody-retention-proof

337.  Represent: Every strict action should have an audit path: detected state, parent rule, mechanism, outcome, timestamp, accuracy, freshness, custody, and evidence reference

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-331`
- policyLane: `data`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `multi`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 741; sourceText: Every strict action should have an audit path: detected state, parent rule, mechanism, outcome, timestamp, accuracy, freshness, custody, and evidence reference.
- acceptedOptions: Detected State | Parent Rule | Mechanism | Outcome | Timestamp | Accuracy | Freshness | Custody | Evidence Reference
- helperText: future-gap via precision-permission-and-accuracy-proof

338.  Represent: parent-owned history/report export

- settingId: `tracking-guide-future-ui-rules-future-ui-rules-338`
- policyLane: `data`; sectionId: `tracking-guide-future-ui-rules`; groupId: `tracking-guide-future-ui-rules-future-ui-rules`
- cardKind: `retention-card`; selectionMode: `single`; controlKind: `retention`
- effectStatus: `future-gap`; runtimeOwner: `portal-only`; capabilityState: `future-gap`
- proofRequirement: Location history requires local custody, retention, deletion, and parent-reveal audit proof before display or export.
- sourceDocument: docs/device-location-tracking-capability-guide.md
- sourceLine: 753; sourceText: parent-owned history/report export.
- acceptedOptions: Represented | Not Represented
- helperText: future-gap via local-history-custody-retention-proof
