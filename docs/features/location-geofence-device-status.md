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
- Tracking proof tiers now separate P0/P1/P2 code readiness from P4/P5/P6
  product claims. P1 fixture/runtime proof exists for deterministic geofence
  transition evaluation, expected-place decision evaluation, retention delete
  read-model filtering, parent-owned retention export, UI-visible
  deleted-history hiding, parent acknowledgement impact, child check-in
  resolution, and Rust ActivityStore tracking event ingest into SQLite via
  `node scripts/test/tracking-plan-runtime-proof.mjs`. The same proof command
  now records a P1 parent portal tracking-state fixture route and test for
  first-target UI states, local proof artifact references, and captures a local
  rendered parent-route screenshot under
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/`.
- P2 service-boundary proof now exists for a narrow
  `agent.activity.tracking.read-model.get` WebSocket command that reads
  tracking event rows from the shared ActivityStore SQLite query store and
  reports citation IDs through `trackingReadModel`. That P2 proof now also
  exposes retention-delete rows as tombstone replay rows with active/tombstone
  counts, deleted-at metadata, and deleted evidence citation IDs. The parent
  portal `policy-tracking` route consumes that service read model as a narrow
  live summary beside the P1 fixture rows. The repeatable proof command is
  `node scripts/test/tracking-plan-service-read-model-proof.mjs`, with
  artifacts
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json`
  and
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`.
- Pre-device gap-closure proof now exists through
  `node scripts/test/tracking-plan-pre-device-proof.mjs`. It reruns the
  tracking contract/runtime/service proofs, runs the mobile child-agent
  scaffold proof stack, builds the Android debug package through the existing
  Android proof scripts, and writes the aggregate artifact
  `output/tracking-plan-proof/pre-device-gap-closure/proof-summary.json` plus
  Android Studio, iOS simulator, WSL/local, and physical-device proof plans.
- P1 local parent-defined place store proof now exists through
  `npm run test:tracking-plan-local-place-store-proof`. It proves
  schema-backed local CRUD/import/export/delete helper behavior for
  parent-defined safe and restricted places, keeps default storage
  parent-device-local with remote sync disabled, and writes artifacts under
  `output/tracking-plan-proof/22-local-parent-defined-place-database/`.
- Android emulator package/service/status proof now exists through
  `npm run test:tracking-plan-android-emulator-proof`. It builds the Android
  debug APK, installs and launches it on an emulator, captures foreground
  service state, UI tree, screenshot, logcat, battery, and connectivity dumps,
  and writes proof under
  `output/tracking-plan-proof/08-android-foreground-location-adapter/`,
  `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/`,
  `output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/`,
  and `test-results/tracking-plan-android-emulator-proof/`.
- P3 WSL/local replay proof now exists through
  `npm run test:tracking-plan-wsl-local-proof`. It records WSL2/Ubuntu
  toolchain evidence, the linked-worktree Git mapping needed for this
  Windows-hosted worktree, the tracking contracts build, the service read-model
  proof, and the Rust core tracking read-model test under
  `output/tracking-plan-proof/wsl-local-replay/`.
- Platform permissions, mobile physical-device proof, full runtime adapters,
  provider delivery, notification delivery, full parent/child UI, hosted screenshots,
  accessibility, and richer live service-backed UI evidence citations remain not
  product-complete.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is now in contract, P1 fixture/runtime proof, narrow P2
service read-model proof with retention-delete tombstone replay, narrow portal
summary-consumption proof, P1 local parent-defined place store proof,
pre-device proof-gate progress, and Android emulator package/service/status
scaffold proof, and P3 WSL/local replay proof. It remains a tracked product gap
until platform location and geofence adapters, broader product read models,
provider delivery, notifications, physical-device proof, full parent/child UI
snapshots, accessibility, hosted UI proof, and richer live service-backed UI
evidence citations are proved.

## Checklist

- [x] Location evidence contract.
- [x] Accuracy/source/stale-state fields.
- [x] Geofence rule and transition contracts.
- [x] Battery/connectivity status contract.
- [ ] Retention/delete/export settings. P1 retention-delete,
      parent-owned export, and UI-visible deleted-history hiding fixture proof
      exist; product settings and live service-backed retention UI remain
      pending.
- [x] Alert intent contract.
- [ ] Android permission/background proof.
- [x] Android emulator package launch, foreground-service scaffold, battery,
      and connectivity proof. This is not foreground location or geofence
      proof.
- [ ] iOS entitlement/background proof.
- [x] Expected-place schedule and exception contracts.
- [x] Parent acknowledgement and escalation contracts.
- [x] Nearby-place ambiguity and AI safety evidence contracts.
- [x] P1 ActivityStore tracking-event SQLite ingest proof.
- [x] P2 service-backed tracking read-model command proof for SQLite journal
      rows, citation IDs, and retention-delete tombstone replay.
- [x] Narrow parent portal summary consumption of the service-backed tracking
      read model.
- [x] P1 local parent-defined place store proof for CRUD/import/export/delete
      with parent-device-local default storage and remote sync disabled.
- [x] P3 WSL/local replay proof for the tracking read-model proof stack and
      linked-worktree toolchain mapping.
- [x] P1 parent portal tracking-state fixture surface, local parent-route screenshot,
      and local proof artifact references.
- [x] Pre-device gap-closure proof gate and Android Studio/iOS simulator/WSL/manual
      proof plans.
- [ ] Richer live service-backed UI citations beyond the narrow summary.
- [ ] Live parent/child UI screenshots, hosted proof, and accessibility proof.

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
Use `docs/plans/tracking-plan/README.md` for implementation sequencing and
workpack ownership. Keep AI as evidence, not authority, and keep LAN/IP/Wi-Fi
presence as hints only. The pre-device proof gate is now repeatable; the next
implementation layers are richer tracking journal/read-model surfaces, full
portal UI snapshots/accessibility, remaining Android foreground-location and
background/geofence runtime proof, iOS simulator/local proof, then physical
Android/iOS proof and authority proof only when matching devices are enrolled.
