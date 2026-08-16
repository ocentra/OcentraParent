# V0.5 Location Platform Deep Dive

<!-- agent-capsule -->

> Agent Capsule
> Plan: `tracking-plan`
> Doc: `V0.5 Location Platform Deep Dive`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This file captures platform reality for tracking work. It is planning input,
not product proof. Workers must re-verify official platform docs and produce
real device artifacts before claiming runtime behavior.

## Authority Tier Matrix

| Tier              | Examples                                      | Product claim allowed                                       |
| ----------------- | --------------------------------------------- | ----------------------------------------------------------- |
| Precise sample    | GPS/fused/current/Core Location with accuracy | Current or last-known location with freshness and accuracy. |
| Platform geofence | Android geofence, iOS region monitoring       | Enter/exit/dwell only with rule and evidence refs.          |
| Background mode   | Android background, iOS Always/background     | Background behavior only after real-device proof.           |
| Managed device    | Device owner, supervised iOS, MDM locate      | Managed behavior only after enrollment proof.               |
| Presence hint     | LAN, Wi-Fi, IP, pairing, app foreground       | Context hint only, never precise child location.            |
| Manual check-in   | Child response, parent-entered status         | Reported check-in, not automatic physical proof.            |

## Platform Status Matrix

| Platform | Current status            | Required before product claim                                       |
| -------- | ------------------------- | ------------------------------------------------------------------- |
| Android  | Planning only             | Permission screenshots, foreground sample, background/geofence run. |
| iOS      | Planning only             | Authorization screenshots, region/background proof, degraded proof. |
| Windows  | Hint/manual-required only | OS location or explicit hint-only UI proof.                         |
| macOS    | Hint/manual-required only | OS location or explicit hint-only UI proof.                         |
| Linux    | Hint/manual-required only | Manual check-in or explicit hint-only UI proof.                     |
| Web      | Parent portal only        | Browser geolocation cannot become child-device tracking proof.      |

## Android

Authority tiers:

- foreground permission;
- background permission;
- geofence transition;
- foreground service notification if used;
- managed/device-owner helper if later needed.

Required proof:

- `ACCESS_FINE_LOCATION`;
- `ACCESS_BACKGROUND_LOCATION` for Android 10+ background behavior;
- settings-page background permission flow on Android 11+;
- geofence enter/exit/dwell;
- active geofence limit handling;
- fused location sample;
- last-known location sample;
- battery throttling;
- offline pending upload;
- app-killed/reboot behavior;
- Play policy and privacy disclosure review before release claims.

Proof artifacts:

```text
output/tracking-plan-proof/android/location-permission.png
output/tracking-plan-proof/android/background-location-settings.png
output/tracking-plan-proof/android/geofence-enter.json
output/tracking-plan-proof/android/geofence-exit.json
output/tracking-plan-proof/android/geofence-dwell.json
output/tracking-plan-proof/android/fused-location-sample.json
output/tracking-plan-proof/android/last-known-location.json
output/tracking-plan-proof/android/battery-throttled.json
output/tracking-plan-proof/android/offline-pending-upload.json
```

## iOS

Authority tiers:

- When In Use;
- Always;
- region monitoring;
- significant-change;
- visits;
- background modes;
- local notifications;
- MDM/supervised/lost mode if later needed.

Required proof:

- authorization screenshots;
- current location sample proof;
- background event proof;
- region transition proof;
- significant-change proof;
- visit event proof;
- low-power/app-terminated degraded proof;
- local notification proof;
- MDM/supervised locate/lost-mode proof if applicable;
- App Store privacy disclosure review before release claims.

Proof artifacts:

```text
output/tracking-plan-proof/ios/authorization-when-in-use.png
output/tracking-plan-proof/ios/authorization-always.png
output/tracking-plan-proof/ios/region-enter.json
output/tracking-plan-proof/ios/region-exit.json
output/tracking-plan-proof/ios/significant-change.json
output/tracking-plan-proof/ios/visit-event.json
output/tracking-plan-proof/ios/background-degraded.md
output/tracking-plan-proof/ios/location-services-disabled.png
```

## Desktop

Authority tiers:

- OS location service;
- manual check-in;
- Wi-Fi/LAN presence hint;
- IP coarse hint;
- missing-device status.

Required proof:

- precise OS location if available;
- Windows location service sample/hint proof;
- macOS location service sample/hint proof;
- Linux location service/manual-checkin proof;
- LAN/home Wi-Fi presence hint proof;
- IP coarse hint no-GPS guard;
- battery/connectivity desktop proof;
- missing laptop mode proof;
- desktop notification proof.

Hard gate:

```text
Desktop IP, LAN, and Wi-Fi presence cannot be shown as precise child location.
```

## Managed Device Paths

Managed Android, supervised iOS/iPadOS, and MDM paths can improve setup,
permission, tamper resistance, and lost-device behavior, but each is a separate
claim. No managed-device path may be implied from a package scaffold or generic
mobile app shell.

Each managed path needs:

- enrollment state;
- supervision/device-owner proof;
- supported command list;
- permission behavior;
- user-visible disclosure;
- audit refs;
- rollback/deactivation behavior;
- platform/store policy review.

## Hard No-Claim Rules

- Do not show LAN, Wi-Fi, IP, or pairing as GPS.
- Do not claim background tracking from foreground-only permission.
- Do not claim iOS Always behavior from When In Use authorization.
- Do not claim managed locate/lost-mode behavior without enrollment proof.
- Do not claim a child is inside a POI from a nearby-provider result alone.
- Do not claim a device is live when only last-known or cached data exists.

## Official References To Re-Verify

- Android Developers: Create and monitor geofences.
- Android Developers: Request background location.
- Android Developers: Request location updates.
- Apple Developer: Monitoring the user's proximity to geographic regions.
- Apple Platform Deployment: device management and lock/locate docs.
- Google Places API: Nearby Search field masks and circle restrictions.
