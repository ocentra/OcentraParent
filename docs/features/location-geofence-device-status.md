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
  counts, deleted-at metadata, deleted evidence citation IDs, and an active
  product-surface summary with kind/device/capability counts plus latest active
  row metadata. The parent portal `policy-tracking` route consumes that service
  read model as a narrow live summary, service-data coverage panel, and live
  service-backed citation rows beside the P1 fixture rows. The repeatable proof
  commands are
  `node scripts/test/tracking-plan-service-read-model-proof.mjs`, with
  artifacts
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json`
  and
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json`,
  plus `npm run test:tracking-plan-service-data-ui-proof`, with artifacts
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
  and `test-results/tracking-plan-service-data-ui-proof/proof.json`.
- Hosted parent `policy-tracking` route screenshot and accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It starts the
  real Rust service against a seeded temporary ActivityStore SQLite database,
  drives the parent route through Playwright, captures desktop, mobile, hosted
  child-safe check-in, and hosted child-runtime UI proof screenshots, writes
  accessibility summary output, proves calm child check-in copy/actions plus
  child disclosure, safe/help response, and location-share consent copy inside
  the hosted route only, and keeps `productClaimReady=false`.
- P1 evidence-quality gate proof now exists through
  `npm run test:tracking-plan-evidence-quality-gate-proof`. It validates
  location UI evidence refs, geofence rule/source refs, nearby-place provider
  context, AI evidence/no-final-action constraints, alert policy-decision refs,
  and retention delete/export before/after proof through parser-backed fixtures,
  retention helpers, parent-domain contracts, and the existing portal citation
  test while keeping live device/provider behavior unclaimed.
- WP26 tracking provider-notification proof now exists through
  `node scripts/test/tracking-provider-notification-proof.mjs`. It maps
  tracking alert intents from the tracking policy read model into the existing
  V0.8 notification provider-status boundary, preserves evidence refs, policy
  decision refs, notification status refs, reason refs, and sensitive-detail
  modes, and records manual-required/unavailable provider states without
  claiming provider delivery, receipts, credentials, parent notification UI,
  child-device delivery, physical-device proof, retry/runtime workers, or
  adapter dispatch.
- WP27 escalation readiness proof now exists through
  `node scripts/test/tracking-escalation-readiness-proof.mjs`. It derives
  parent acknowledgement, child check-in, urgent second-guardian, and critical
  multi-channel manual-readiness rows from the existing tracking policy read
  model, keeps AI as non-authoritative evidence, and explicitly rejects
  emergency auto-contact, provider delivery, child-device delivery, parent UI,
  production worker, and physical-device claims.
- WP28 temporary live tracking mode proof now exists through
  `node scripts/test/tracking-temporary-live-mode-proof.mjs`. It derives
  parent-authorized active, battery-degraded, permission-degraded, expired
  auto-stop, retention-delete-ready, and manual-required rows from the existing
  temporary live tracking grants, keeps duration/cadence, audit, policy,
  evidence, degraded-state, and retention refs attached, and explicitly rejects
  live/current/background location runtime, provider delivery, remote relay,
  parent portal live-map runtime, child-device delivery, production worker, and
  physical-device claims.
- WP20 Google Places/POI provider adapter proof now exists through
  `node scripts/test/tracking-poi-provider-adapter-proof.mjs`. It builds a
  bounded Google Places Nearby Search request contract with a production-safe
  field mask, maps real-shaped provider response rows into nearby-place
  category/distance/confidence/ambiguity evidence, and records provider
  unavailable degradation without claiming live Google execution, credentials,
  exact place, or physical-device proof.
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
- iOS simulator package proof is now routed through
  `npm run test:tracking-plan-ios-simulator-proof`. On macOS package-preview CI
  it writes tracking-specific proof artifacts after the existing Xcode
  simulator build and simctl install/launch smoke. On non-macOS hosts it writes
  explicit `manual_required` proof instead of pretending simulator execution
  happened.
- WP31 platform manual-state proof now exists through
  `node scripts/test/tracking-platform-manual-state-proof.mjs`. It validates
  schema-backed Android, iOS, desktop, Linux, web, emulator-scaffold, and
  child-runtime delivery rows that render as manual-required, unavailable,
  not-claimed, or scaffold-only states with `productClaimReady=false`, and
  writes proof under
  `output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/`.
- Platform permissions, mobile physical-device proof, full runtime adapters,
  provider delivery, notification delivery, actual child-device delivery/runtime
  execution, full parent/child UI beyond the hosted parent route, and broader
  read-model product surfaces beyond this hosted service-data coverage remain
  not product-complete.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is now in contract, P1 fixture/runtime proof, narrow P2
service read-model proof with retention-delete tombstone replay and active
product-surface summary fields, narrow portal summary-consumption proof, live
service-backed portal citation rows, hosted
parent route service-data coverage, hosted parent route screenshot/accessibility
proof with hosted child-safe check-in copy/actions and hosted child-runtime
disclosure/safe-help/location-share consent copy, P1 local parent-defined place
store proof, pre-device proof-gate progress, Android emulator package/service/status
scaffold proof, P3 WSL/local replay proof, P1 evidence-quality gate proof, WP26
tracking alert-to-provider-status handoff proof, P1 escalation readiness proof
for acknowledgement/check-in/manual escalation states, WP28 temporary live
tracking mode proof for authorization/duration/cadence/degraded/auto-stop/
retention states, and WP20 Google Places/POI provider request/response mapping
proof, and WP31 platform manual-state proof for unsupported/manual-required/
not-claimed display states. It remains a tracked product gap until platform
location and geofence adapters, broader product read models, actual live provider
execution/delivery, notification receipt ingestion, physical-device proof,
actual child-device delivery/runtime execution, and full parent/child UI
snapshots/accessibility beyond the hosted parent route are proved.

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
- [x] iOS simulator package build/install/launch proof routing. This is not
      Core Location, entitlement, background region, notification, physical-device,
      or child-agent parity proof.
- [ ] iOS entitlement/background proof.
- [x] Expected-place schedule and exception contracts.
- [x] Parent acknowledgement and escalation contracts.
- [x] Tracking alert intents map to provider-status boundary evidence with
      evidence refs, policy decision refs, notification status refs, reason refs,
      and minimal/authenticated-drill-in payload boundaries. This is not
      provider delivery or receipt proof.
- [x] WP27 P1
      escalation readiness proof for acknowledgement cancellation, child
      check-in resolution, urgent second-guardian manual readiness, critical
      multi-channel manual readiness, AI non-authority, and no emergency
      auto-contact.
- [x] WP28 P1 temporary live tracking mode proof for parent authorization,
      child disclosure, bounded duration/cadence, battery/permission degraded
      states, auto-stop, retention-delete readiness, and no live runtime,
      provider, relay, parent live-map, child-device, production worker, or
      physical-device claims.
- [x] Nearby-place ambiguity, Google Places/POI provider adapter request/
      response mapping, and AI safety evidence contracts. Live provider
      execution/credentials and exact-place/physical-device proof remain
      unclaimed.
- [x] P1 ActivityStore tracking-event SQLite ingest proof.
- [x] P2 service-backed tracking read-model command proof for SQLite journal
      rows, citation IDs, retention-delete tombstone replay, and active
      kind/device/capability product-surface summary fields.
- [x] Narrow parent portal summary consumption and live citation rows for the
      service-backed tracking read model.
- [x] Hosted parent route service-data coverage for the service-backed tracking
      read model, including active/tombstone row counts, kind coverage, custody,
      capability, and active/deleted evidence references.
- [x] P1 local parent-defined place store proof for CRUD/import/export/delete
      with parent-device-local default storage and remote sync disabled.
- [x] P3 WSL/local replay proof for the tracking read-model proof stack and
      linked-worktree toolchain mapping.
- [x] P1 parent portal tracking-state fixture surface, local parent-route screenshot,
      and local proof artifact references.
- [x] Hosted parent `policy-tracking` route screenshot and accessibility proof
      against the real Rust service and seeded ActivityStore, including a
      hosted child-safe check-in copy/actions card and hosted child-runtime
      disclosure/safe-help/location-share consent card. This is not
      child-device delivery/runtime execution, full service-data UI, or
      physical-device proof.
- [x] Pre-device gap-closure proof gate and Android Studio/iOS simulator/WSL/manual
      proof plans.
- [x] Evidence-quality gate proof for tracking UI evidence refs, geofence
      source refs, nearby-place context fields, AI no-final-action constraints,
      alert policy-decision refs, and retention before/after proof. This is not
      live device/provider behavior proof.
- [ ] Full live parent/child UI screenshots and accessibility proof beyond the
      hosted parent route.

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
Use `docs/plans/tracking-plan/README.md` for implementation sequencing and
workpack ownership. Keep AI as evidence, not authority, and keep LAN/IP/Wi-Fi
presence as hints only. The pre-device proof gate is now repeatable; the next
implementation layers are broader tracking journal/read-model product surfaces,
full portal UI snapshots/accessibility beyond the hosted route and hosted
child-safe check-in proof, remaining Android foreground-location and
background/geofence runtime proof, iOS Core Location/region proof beyond
simulator package launch, then physical Android/iOS proof and authority proof
only when matching devices are enrolled.
