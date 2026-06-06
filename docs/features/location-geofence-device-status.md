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
  plus retention settings read-model proof through
  `node scripts/test/tracking-retention-settings-read-model-proof.mjs`, with
  artifacts
  `output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json`
  and
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json`,
  plus `npm run test:tracking-plan-service-data-ui-proof`, with artifacts
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
  and `test-results/tracking-plan-service-data-ui-proof/proof.json`.
- Hosted parent `policy-tracking` route screenshot and accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It starts the
  real Rust service against a seeded temporary ActivityStore SQLite database,
  drives the React parent route through Playwright, renders the service-data
  coverage card and family dashboard rollup card beside the service read-model
  summary, captures desktop, mobile, hosted child-safe check-in, hosted
  child-runtime UI, hosted family dashboard rollup, and unsupported/manual
  platform render-state proof screenshots, writes accessibility summary output,
  proves calm child check-in copy/actions plus child disclosure, safe/help
  response, location-share consent copy, and manual-required/unavailable/
  authority-required unsupported platform rows inside the hosted route only, and
  keeps `productClaimReady=false`.
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
- WP26 tracking notification receipt boundary proof now exists through
  `node scripts/test/tracking-notification-receipt-boundary-proof.mjs`. It
  derives receipt-ingestion-required, manual-receipt-required, and
  provider-unavailable rows from the tracking provider-notification proof,
  preserves provider proof refs, evidence refs, policy decision refs,
  notification status refs, reason refs, provider attempt refs, and audit refs,
  and cites the V0.8 delivered-provider receipt-required contract while keeping
  webhook receipt ingestion runtime, provider delivery, credentials, adapter
  dispatch, child-device delivery, authority, physical-device proof, and
  production durable outbox storage unclaimed.
- WP26 tracking notification preference preflight proof now exists through
  `node scripts/test/tracking-notification-preference-preflight-proof.mjs`. It
  derives parent-preference-required, source-manual-required, and
  source-unavailable rows from the tracking provider-notification proof,
  preserves provider attempt refs, provider preference refs, evidence refs,
  policy decision refs, notification status refs, reason refs, and quiet-hours
  requirement refs, and keeps parent notification preference UI/history UI,
  frequency controls, quiet-hours timer runtime, provider delivery, receipt
  runtime, credentials, adapter dispatch, child-device delivery,
  physical-device proof, and production durable outbox storage unclaimed.
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
- WP25 tracking policy compiler runtime proof now exists through
  `node scripts/test/tracking-policy-compiler-runtime-proof.mjs`. It compiles
  parent-owned tracking rules for observe, notify, child check-in, parent
  acknowledgement, temporary live tracking, escalation, critical alert,
  suppress/no-action, and manual-required outcomes, keeps AI as evidence rather
  than final authority, and records no runtime enforcement, provider delivery,
  platform adapter, production worker, UI-complete, or physical-device claims.
- Required fixture-state coverage proof now exists through
  `node scripts/test/tracking-fixture-coverage-proof.mjs`. It records fresh,
  stale, offline, permission-denied, low-accuracy, ambiguous nearby place,
  exception, parent-acknowledged, child check-in, temporary-live-expired,
  missing-device, retention-deleted, remote-sync-disabled, and
  remote-AI-disabled coverage with artifact refs while keeping live-device,
  provider-delivery, child-runtime, physical-device, production-worker, and
  product-ready claims false.
- WP29 missing-device mode proof now exists through
  `node scripts/test/tracking-missing-device-mode-proof.mjs`. It proves
  parent-domain rows for last-known-only, offline/powered-off,
  contact-requested, and manual-required missing-device states, keeps
  last-known location, device contact, battery, connectivity, pending upload,
  parent action/audit refs, and UI state tokens prominent, and records explicit
  non-claims for current location, live tracking runtime, powered-off device
  tracking, remote sync runtime, provider delivery, portal runtime UI,
  physical-device proof, and OS lost-mode APIs.
- WP13 desktop presence hint proof now exists through
  `node scripts/test/tracking-desktop-presence-hint-proof.mjs`. It proves
  parent-domain rows for LAN presence hint, Wi-Fi presence hint, IP coarse hint,
  manual check-in, stale/offline last-known state, missing-device state, and
  desktop OS location manual-required state while rejecting GPS, precise
  location, physical-presence, LAN-pairing physical proof, OS location runtime,
  physical-device proof, and product-ready desktop tracking claims.
- WP32 report/policy consumer proof now exists through
  `node scripts/test/tracking-report-policy-consumer-proof.mjs`. It derives
  parent report summary, policy evidence drill-in, and retention audit/export
  consumer-readiness rows from the existing tracking service read-model and
  active product-surface summary proof, keeps source proof refs, evidence refs,
  policy decision refs, report surface refs, and audit refs attached, and
  explicitly rejects portal UI completion, child-device delivery, provider
  delivery, notification receipt ingestion, authority, physical-device, and
  product-complete claims.
- Unsupported and not-yet-proved tracking platform states now have a
  parent-domain manual-required/unavailable render-state proof through
  `node scripts/test/tracking-unsupported-platform-manual-proof.mjs`. It proves
  Android/iOS background and geofence rows, desktop OS location, web
  child-agent unavailability, and authority-required hard-control rows stay
  UI-ready without invented capability, product-ready, physical-device, authority,
  or standalone product claims. The hosted parent `policy-tracking` route now
  renders and screenshots those rows through
  `npm run test:tracking-plan-hosted-ui-proof`, without claiming physical-device
  execution, authority enrollment, provider delivery, or product-ready tracking.
- WP32 family dashboard rollup proof now exists through
  `node scripts/test/tracking-family-dashboard-rollup-proof.mjs`. It derives
  active family summary, child-attention summary, and retention-audit summary
  rows from the existing tracking service read-model, product-surface summary,
  and report/policy consumer proof refs while keeping full portal UI completion,
  child-device delivery, provider delivery, notification receipt ingestion,
  authority, physical-device, and product-complete claims false. The hosted
  parent `policy-tracking` route now renders those rows as a narrow family
  dashboard rollup card through `npm run test:tracking-plan-hosted-ui-proof`,
  with screenshot
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-family-dashboard-rollup.png`
  and `productClaimReady=false`.
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
- WP08/WP09 Android permission/background manual-required proof now exists
  through `node scripts/test/tracking-android-permission-background-proof.mjs`.
  It maps the existing emulator scaffold/manual proof plans into parent-domain
  rows for foreground permission grant, foreground location sample, background
  permission grant, and geofence transition gaps while keeping foreground
  permission, foreground sample, background permission, background runtime,
  geofence runtime, physical-device, authority, notification delivery, provider
  delivery, and product-ready Android tracking claims false.
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
- WP11/WP12 iOS Core Location manual-required proof now exists through
  `node scripts/test/tracking-ios-location-manual-required-proof.mjs`. It maps
  simulator/package proof refs and manual proof plans into parent-domain rows
  for When In Use authorization, foreground location sample, denied/restricted
  and services-disabled states, Always authorization, region transitions,
  significant-change/visit events, and background terminated/relaunch gaps while
  keeping Core Location runtime, entitlement, notification delivery,
  physical-device, authority, and product-ready iOS tracking claims false.
- Platform permissions, mobile physical-device proof, full runtime adapters,
  provider delivery, notification delivery, actual child-device delivery/runtime
  execution, full parent/child UI beyond the hosted parent route, and broader
  read-model product surfaces beyond the report/policy consumer-readiness rows
  and hosted service-data coverage remain not product-complete.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is now in contract, P1 fixture/runtime proof, narrow P2
service read-model proof with retention-delete tombstone replay and active
product-surface summary fields, narrow portal summary-consumption proof, live
service-backed portal citation rows, hosted
parent route service-data coverage, hosted parent route screenshot/accessibility
proof with hosted child-safe check-in copy/actions, hosted child-runtime
disclosure/safe-help/location-share consent copy, and hosted family dashboard
rollup card, P1 local parent-defined place
store proof, pre-device proof-gate progress, Android emulator package/service/status
scaffold proof, WP08/WP09 Android permission/background manual-required proof,
WP11/WP12 iOS Core Location manual-required proof, P3 WSL/local replay proof,
P1 evidence-quality gate proof, WP26 tracking alert-to-provider-status handoff
and notification preference preflight proof, P1 escalation readiness proof for
acknowledgement/check-in/manual
escalation states, WP28 temporary live tracking mode proof for authorization/
duration/cadence/degraded/auto-stop/retention states, and WP20 Google Places/
POI provider request/response mapping proof, WP25 parent-policy compiler/
evaluator runtime proof, and required fixture-state coverage proof, WP29
missing-device mode parent-domain proof for last-known-only/offline/
contact-requested/manual-required states, manual-required/unavailable platform
render-state proof plus hosted unsupported/manual platform route screenshot, and
WP32 report/policy consumer-readiness proof for parent report summary, policy
drill-in, and retention audit/export rows, plus WP32 family dashboard rollup
proof for active family, child-attention, and retention-audit summary rows with
narrow hosted-route rendering, and retention settings read-model proof for
retention window, delete-after-alert, parent export, remote-sync disabled, and
remote-AI disabled rows. It remains a tracked product gap until platform
location and geofence runtime adapters, broader product read models beyond
these rows, full dashboard UI beyond the hosted parent route, actual
writable/live retention settings UI, actual live provider
execution/delivery, notification receipt ingestion, physical-device proof,
actual child-device delivery/runtime execution, remote sync runtime, OS
lost-mode APIs, production upload workers, and full parent/child UI
snapshots/accessibility beyond the hosted parent route are proved.

## Checklist

- [x] Location evidence contract.
- [x] Accuracy/source/stale-state fields.
- [x] Geofence rule and transition contracts.
- [x] Battery/connectivity status contract.
- [ ] Retention/delete/export settings. P1 retention-delete,
      parent-owned export, UI-visible deleted-history hiding fixture proof, and
      P2 retention settings read-model rows now exist; actual writable product
      settings and live service-backed retention UI remain pending.
- [x] Alert intent contract.
- [ ] Android permission/background runtime proof. WP08/WP09 parent-domain
      manual-required proof rows now exist for foreground permission,
      foreground sample, background permission, and geofence transition gaps;
      real permission grant/sample/transition runtime and physical-device proof
      remain pending.
- [x] Android emulator package launch, foreground-service scaffold, battery,
      and connectivity proof. This is not foreground location or geofence
      proof.
- [x] iOS simulator package build/install/launch proof routing. This is not
      Core Location, entitlement, background region, notification, physical-device,
      or child-agent parity proof.
- [ ] iOS entitlement/background proof. WP11/WP12 parent-domain
      manual-required proof rows now exist for When In Use authorization,
      foreground sample, denied/restricted and services-disabled states, Always
      authorization, region transitions, significant-change/visit events, and
      background terminated/relaunch gaps; real Core Location authorization,
      samples, background/region runtime, entitlement, notification delivery,
      physical-device, and authority proof remain pending.
- [x] Expected-place schedule and exception contracts.
- [x] Parent acknowledgement and escalation contracts.
- [x] WP25 P1 parent-policy compiler/evaluator runtime proof for tracking
      rules, deterministic parent-policy final authority, AI non-authority,
      manual-required fallback, and dry-run/preview output. This is not runtime
      enforcement, provider delivery, platform adapter, production worker,
      product UI, or physical-device proof.
- [x] Required fixture coverage proof for fresh, stale, offline,
      permission-denied, low-accuracy, ambiguous nearby place, exception,
      parent acknowledgement, child check-in, temporary-live-expired,
      missing-device, retention-deleted, remote-sync-disabled, and
      remote-AI-disabled states. This is fixture/read-model coverage only; it is
      not live-device, child-runtime, provider-delivery, physical-device, or
      product-ready proof.
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
- [x] Missing-device mode parent-domain proof for last-known-only,
      offline/powered-off, contact-requested, and manual-required states,
      including last-known evidence refs, contact/battery/connectivity/pending
      upload status, parent action/audit refs, UI state tokens, and no-current
      location copy gates. This is not portal runtime UI, child-device delivery,
      physical-device, provider delivery, remote sync, or OS lost-mode API proof.
- [x] Desktop presence hint parent-domain proof for LAN, Wi-Fi, IP coarse hint,
      manual check-in, stale/offline last-known, missing-device, and desktop OS
      location manual-required rows. This is not GPS, precise-location,
      physical-presence, OS location runtime, physical-device, provider
      delivery, portal/runtime UI completion, or product-ready desktop tracking
      proof.
- [x] P1 ActivityStore tracking-event SQLite ingest proof.
- [x] P2 service-backed tracking read-model command proof for SQLite journal
      rows, citation IDs, retention-delete tombstone replay, and active
      kind/device/capability product-surface summary fields.
- [x] Narrow parent portal summary consumption and live citation rows for the
      service-backed tracking read model.
- [x] Hosted parent route service-data coverage for the service-backed tracking
      read model, including active/tombstone row counts, kind coverage, custody,
      capability, and active/deleted evidence references.
- [x] WP32 report/policy consumer-readiness proof for parent report summary,
      policy evidence drill-in, and retention audit/export rows. This is not
      portal UI completion, child-device delivery, provider delivery,
      notification receipt ingestion, authority, physical-device, or
      product-complete proof.
- [x] WP32 family dashboard rollup proof for active family summary,
      child-attention summary, and retention-audit summary rows, plus narrow
      hosted parent-route rendering of those rows. This is not full dashboard
      UI beyond the hosted route, child-device delivery/runtime execution,
      provider delivery, notification receipt ingestion, authority,
      physical-device, or product-complete proof.
- [x] P1 local parent-defined place store proof for CRUD/import/export/delete
      with parent-device-local default storage and remote sync disabled.
- [x] P3 WSL/local replay proof for the tracking read-model proof stack and
      linked-worktree toolchain mapping.
- [x] P1 parent portal tracking-state fixture surface, local parent-route screenshot,
      and local proof artifact references.
- [x] Hosted parent `policy-tracking` route screenshot and accessibility proof
      against the real Rust service and seeded ActivityStore, including a
      rendered service-data coverage card, hosted child-safe check-in
      copy/actions card, and hosted child-runtime disclosure/safe-help/location-share
      consent card, plus a family dashboard rollup card. This is not
      child-device delivery/runtime execution, full dashboard UI beyond the
      hosted route, or physical-device proof.
- [x] Pre-device gap-closure proof gate and Android Studio/iOS simulator/WSL/manual
      proof plans.
- [x] Parent-domain unsupported-platform manual-required/unavailable render-state
      proof for Android/iOS background/geofence rows, desktop OS location, web
      child-agent unavailability, and authority-required hard-control rows. This
      is not physical-device or authority proof.
- [x] Hosted parent `policy-tracking` route screenshot/accessibility proof for
      unsupported/manual platform rows, including manual-required, unavailable,
      and authority-required rendered states. This is not physical-device
      execution, authority enrollment, provider delivery, or product-ready
      tracking proof.
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
full portal UI snapshots/accessibility beyond the hosted route, remaining
Android foreground-location and background/geofence runtime proof, iOS Core
Location/region proof beyond simulator package launch, then physical Android/iOS
proof and authority proof only when matching devices are enrolled.
