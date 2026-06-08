<!-- agent-capsule -->

> Agent Capsule
> Doc: Location, Geofence, And Device Status
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

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
  [tracking settings inventory](../plans/tracking-plan/workpacks/tracking-control-settings-inventory.md),
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
- WP13 desktop presence hint proof now exists through
  `node scripts/test/tracking-desktop-presence-hint-proof.mjs`. It proves
  Windows/macOS precise desktop location remains manual-required, LAN/Wi-Fi/IP
  rows are hint-only, manual check-in is separate from automatic presence, and
  stale/offline/missing-device rows cannot be shown as live. It does not claim
  desktop OS location runtime, GPS/precise location, exact physical presence,
  physical-device proof, production behavior, or UI.
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
  plus retention settings writer-boundary preflight proof through
  `node scripts/test/tracking-retention-settings-writer-boundary-proof.mjs`, with
  artifacts
  `output/tracking-plan-proof/07-retention-and-custody-model/19-retention-settings-writer-boundary-proof.json`
  and
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json`,
  plus `npm run test:tracking-plan-service-data-ui-proof`, with artifacts
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/18-service-data-ui-proof.json`,
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/20-service-data-ui-proof.json`,
  and `test-results/tracking-plan-service-data-ui-proof/proof.json`. The
  refreshed service-data UI proof includes a service-backed citation matrix that
  verifies the route coverage card uses the same read-model command, event,
  payload field, citation fields, active evidence refs, deleted-evidence
  tombstone refs, and no-claim boundaries as the live citation rows.
- WP07/WP32 local retention settings mutation proof now exists through
  `node scripts/test/tracking-retention-settings-mutation-proof.mjs`. It
  applies the existing retention-window, delete-after-alert, parent-export,
  remote-sync-disabled, and remote-AI-disabled writer intents as local service
  mutation rows while keeping remote sync and remote AI disabled and keeping
  live writable UI, platform runtime, child-device delivery, physical-device,
  authority, provider-delivery, production, and product-ready claims false.
- WP07/WP32 service transport proof now exists for a typed retention settings
  write command through
  `node scripts/test/tracking-retention-settings-write-command-proof.mjs`. It
  validates the TypeScript request/result parser, Rust protocol serialization,
  and Rust service WebSocket response for a no-product-claim local service
  execution result tied to the existing mutation proof refs while keeping live
  writable UI, platform runtime, child-device delivery, physical-device,
  authority, provider-delivery, notification receipts, production, and
  product-ready claims false.
- WP07/WP32 local service state readback proof now exists through
  `node scripts/test/tracking-retention-local-service-state-proof.mjs`. It
  derives a parent-domain proof row from the typed write-command proof, keeps the
  applied retention values, local service state revision, snapshot ref, and
  local durable settings store ref in the evidence chain, writes WP07/WP32/WP33
  artifacts, and keeps writable product settings, platform runtime,
  child-device delivery, physical-device, authority, provider-delivery,
  notification receipts, production, and product-ready claims false.
- WP07/WP32 durable retention settings proof now exists through
  `node scripts/test/tracking-retention-durable-settings-proof.mjs`. It derives
  local durable settings rows from the local service state readback proof,
  records the Rust service durable store ref and persisted state, writes
  WP07/WP32/WP33 artifacts, and keeps writable product settings, platform
  runtime, child-device delivery, physical-device, authority, provider-delivery,
  notification receipts, production workers, and product-ready claims false.
- WP07/WP33 retention product-settings writable execution artifact proof now
  exists through
  `node scripts/test/tracking-retention-product-settings-writable-execution-proof.mjs`.
  It consumes the local service state readback proof, records a derivation
  matrix for source proof refs, local service revision, snapshot ref, durable
  store ref, applied retention values, and no-claim boundaries, and writes
  `output/tracking-plan-proof/tracking-retention/product-settings-writable-execution.json`
  plus WP07/WP33 companion artifacts for the local writable execution row while
  keeping portal writable UI, platform runtime retention enforcement,
  child-device delivery, provider delivery, notification receipts,
  physical-device proof, authority, production, and product-ready claims false.
- WP07/WP33 applied retention settings runtime bridge proof now exists through
  `node scripts/test/tracking-retention-applied-settings-runtime-bridge-proof.mjs`.
  It consumes the writable execution proof, classifies the local
  `tracking-retention/product-settings-writable-execution.json` artifact as
  present, keeps
  `tracking-retention/platform-runtime-retention-enforcement.json` missing, and
  preserves platform enforcement, production, authority, child-device,
  provider, physical-device, notification receipt, and product-ready claims as
  false.
- WP07/WP33 retention platform enforcement preflight proof now exists through
  `node scripts/test/tracking-retention-platform-enforcement-preflight-proof.mjs`.
  It consumes the retention runtime artifact gate, writes Android, iOS, and
  desktop platform-runtime acceptance rows plus a manual validation runbook,
  and is now consumed by the product-readiness closure and real-runtime handoff
  accounting. It keeps platform runtime retention enforcement, writable product
  settings, child-device delivery, physical-device proof, authority,
  production, and product-ready claims false.
- WP33 physical-device evidence review proof now exists through
  `node scripts/test/tracking-physical-device-evidence-review-proof.mjs`. It
  consumes the Android/iOS physical-device artifact gate, preserves the
  required artifact names, acceptance criteria, manual commands, and review
  notes, and now carries the Samsung S9 Android package/service/status proof as
  20 supporting status artifacts while keeping the Android physical
  location/geofence behavior artifact set missing. It keeps artifact content
  unaccepted, physical behavior unclaimed, and product-ready tracking false
  until real-device behavior artifacts exist and are reviewed.
  Product-readiness closure and real-runtime handoff now consume those review
  counts instead of treating file presence as behavior proof.
- Hosted parent `policy-tracking` route screenshot and accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It starts the
  real Rust service against a seeded temporary ActivityStore SQLite database,
  drives the React parent route through Playwright, renders the service-data
  coverage card, explicit service-backed citation detail card, read-only
  evidence drawer card, family dashboard rollup card, report/policy consumer
  card, and retention settings read/local service write card beside the service
  read-model summary, captures
  desktop, mobile, hosted service-backed citation detail, hosted evidence
  drawer, hosted child-safe check-in, hosted child-runtime UI, hosted family
  dashboard rollup, hosted report/policy consumer, hosted retention settings,
  hosted parent action readiness, hosted missing-device state, hosted parent
  overview/devices shell tracking summary cards with no-product-claim
  assertions, and unsupported/manual platform render-state proof screenshots,
  writes
  accessibility summary output, proves
  calm child check-in copy/actions plus child disclosure, safe/help response,
  location-share consent copy, typed retention settings local service write
  command/result rendering, the retention local service-state proof artifact on the hosted
  retention card, and manual-required/unavailable/authority-required unsupported
  platform rows inside the hosted route only, and keeps
  `productClaimReady=false`.
- WP30 child-runtime delivery boundary proof now exists through
  `node scripts/test/tracking-child-runtime-delivery-boundary-proof.mjs`. It
  links hosted child-runtime disclosure proof refs to WP18 child check-in
  timeout rows, records required child-device runtime proof refs for waiting,
  safe, help, call-parent, and timeout states, writes WP30/WP33 artifacts, and
  keeps actual child-device delivery/execution, rendered child-device runtime
  UI, provider delivery, notification receipt ingestion, live location runtime,
  physical-device proof, authority, production workers, and product-ready
  claims false.
- WP30 child-runtime execution readiness proof now exists through
  `node scripts/test/tracking-child-runtime-execution-readiness-proof.mjs`. It
  consumes the delivery boundary proof, emits deterministic delivery-envelope,
  execution-result, visible-snapshot, parent-receipt, and runtime-observation
  requirement refs for each child check-in state, writes WP30/WP33 artifacts,
  and keeps actual child-device delivery/execution, rendered child-device
  runtime UI, provider delivery, notification receipt ingestion, live location
  runtime, physical-device proof, authority, production workers, and
  product-ready claims false.
- WP30 child-runtime product-readiness blocker proof now exists through
  `node scripts/test/tracking-child-runtime-product-readiness-blocker-proof.mjs`.
  It consumes the snapshot-requirements proof plus the Android emulator
  readiness bridge, records package/foreground-service/local geofence
  prerequisite accounting, records the remaining actual delivery,
  execution-result, rendered child UI, parent receipt, runtime observation,
  physical-device, and authority blockers, writes WP30/WP33 artifacts, and
  keeps child-device delivery/execution, provider delivery, notification receipt
  ingestion, live location runtime, physical-device proof, authority, production
  workers, and product-ready claims false.
- WP30/WP33 child-runtime artifact gate proof now exists through
  `node scripts/test/tracking-child-runtime-artifact-gate-proof.mjs`. It checks
  the required child-device runtime execution proof root for delivery envelope,
  execution result, rendered child UI snapshot, parent receipt, runtime
  observation, consent state, device log, and result-summary artifacts, writes
  WP30/WP33 artifacts, and keeps child-device delivery/execution, rendered
  child-device UI runtime, parent receipt runtime, runtime observation,
  physical-device proof, authority, provider delivery, production workers, and
  product-ready tracking unclaimed until those real child-device artifacts
  exist.
- WP08/WP30/WP33 child-runtime Android emulator readiness bridge proof now
  exists through
  `node scripts/test/tracking-child-runtime-android-emulator-readiness-bridge-proof.mjs`.
  It links the existing Android emulator package/foreground-service/permission
  and local emulator geofence/dwell evidence to the child-runtime artifact gate,
  records the required, present, and missing child-device runtime artifact
  refs/counts, writes WP08/WP30/WP33 artifacts, and keeps child-device
  delivery/execution, rendered child-device UI runtime, parent receipt runtime,
  physical-device proof, authority, provider delivery, production workers, and
  product-ready tracking unclaimed.
- WP08/WP09/WP10/WP33 Android emulator artifact inventory proof now exists
  through
  `node scripts/test/tracking-android-emulator-artifact-inventory-proof.mjs`.
  It verifies the existing adb install/launch/runtime outputs, foreground
  permission UX artifacts, Android 11+ background Settings route attempt
  artifact, including the ATD emulator unavailable-Settings result,
  app-reported foreground location evidence, local emulator geofence transition
  and app-owned dwell evidence, device-status proof, and validation log as required local emulator
  artifacts, carries required/present/missing counts into product-readiness
  closure and real-runtime handoff accounting, and keeps Android system
  geofence delivery, Android system dwell, physical-device proof, authority, production runtime, and
  product-ready tracking unclaimed.
- WP08/WP09/WP10/WP33 Android physical-device runtime proof now exists through
  `node scripts/test/tracking-android-physical-device-runtime-proof.mjs`. It
  verifies the Samsung S9 Wi-Fi ADB target (`192.168.2.45:5555`) with debug APK
  push plus on-device package-manager install, launch attempt, foreground-service
  `ServiceRecord` with `isForeground=true`, device metadata,
  foreground/background location permissions, 4,177 foreground-service-backed
  background GPS samples, app-owned geofence registration, Android proximity
  registration metadata, battery/connectivity dumps, UI/keyguard screenshot, and
  logcat artifacts. Physical background location sampling is observed, but
  physical geofence transition/dwell counters and Android system geofence
  delivery remain zero/unclaimed; authority enrollment, production workers, and
  product-ready Android tracking remain unclaimed.
- WP10 Android status proof now consumes the Samsung S9 physical-device runtime
  battery/connectivity/status artifacts through
  `node scripts/test/tracking-android-status-proof.mjs`. The WP10 row is closed
  only for emulator plus physical battery/connectivity/status evidence; offline
  radio behavior, physical geofence behavior, production upload workers, and
  product-ready Android tracking remain unclaimed.
- WP08/WP09/WP30/WP33 local platform proof batch now exists through
  `node scripts/test/tracking-local-platform-proof-batch.mjs`. It aggregates
  Android emulator runtime evidence, Samsung S9 Android physical package/
  service/status evidence, WSL/local replay, hosted parent UI accessibility,
  product parent/child UI local artifacts, product-readiness closure, and
  real-runtime handoff accounting into one parent-domain proof bundle. The
  batch now records seven local proof-passed rows plus one manual-required
  real-runtime handoff row, and keeps Android physical behavior, iOS runtime,
  child-device runtime, authority, provider-delivery, production, and
  product-ready tracking claims false.
- WP33 local execution strategy proof now exists through
  `node scripts/test/tracking-local-execution-strategy-proof.mjs`. It records
  Windows host validation, WSL2/Ubuntu replay, Docker CLI availability, Android
  emulator proof routing, Samsung S9 physical status-only routing, macOS/iOS
  CI/manual routing, remaining physical/manual runtime routing, and the final
  sync/validation gate in one read model. The current proof has 8 execution
  strategy rows, 4 local-runnable rows, 4 CI-runnable rows, 2 manual-required
  rows, and 1 unavailable-here row for Docker on the current Windows PATH while
  keeping Android physical behavior, iOS runtime, child-device runtime,
  production runtime, and product-ready tracking claims false.
- WP08/WP09/WP10/WP11/WP12/WP13/WP31/WP33 cross-platform runtime capability
  proof now exists through
  `node scripts/test/tracking-cross-platform-runtime-capability-proof.mjs`. It
  aggregates the current Windows host toolchain, WSL/Linux replay, Docker
  container runtime availability, Android emulator runtime, Android physical
  package/service/status and geofence registration evidence, and macOS/iOS
  CI/manual routing into one parent-domain proof. The current Windows host,
  WSL/Linux, Docker container runtime, Android emulator, Android physical
  status, Android physical app-owned geofence registration, and Android physical
  system proximity registration rows pass locally; the refreshed harness
  normalizes WSL host probe output before writing proof artifacts and records
  Docker only when both CLI and daemon are reachable; macOS/iOS remain
  CI/manual-required on this Windows host. The proof keeps
  physical device behavior, authority, production, and product-ready tracking
  claims false.
- WP11/WP12/WP31/WP33 iOS simulator artifact inventory proof now exists
  through
  `node scripts/test/tracking-ios-simulator-artifact-inventory-proof.mjs`. It
  verifies the existing iOS simulator package proof artifacts, iOS
  manual-required Core Location rows, privacy disclosure release-gate proof,
  platform proof notes, and validation logs as required local simulator
  artifacts, carries required/present/missing simulator and manual-required
  counts into product-readiness closure and real-runtime handoff accounting,
  and keeps Core Location runtime, background region delivery, physical-device
  proof, authority, provider delivery, production runtime, and product-ready iOS
  tracking unclaimed.
- WP30/WP33 full product UI readiness blocker proof now exists through
  `node scripts/test/tracking-full-product-ui-readiness-blocker-proof.mjs`. It
  consumes the hosted UI artifact inventory and child-runtime artifact gate,
  writes WP30/WP33 artifacts, and records manual-required blockers for the
  hosted-route-only boundary, full parent overview/device-detail runtime UI,
  parent notification history/preferences runtime, retention settings production
  UI, rendered child-device runtime UI, parent receipt UI, cross-surface
  accessibility regression proof, physical-device UI proof, authority-gated UI
  proof, provider-delivery UI proof, production product UI, and product-ready
  tracking UI. Hosted-route evidence stays acknowledged, but full product
  parent/child UI, child-device runtime UI, physical-device, authority,
  provider delivery, production, and product-ready claims remain false.
- WP30/WP33 full product UI runtime artifact gate proof now exists through
  `node scripts/test/tracking-full-product-ui-runtime-artifact-gate-proof.mjs`.
  It imports the required product parent/child UI artifact refs from the full
  product UI readiness blocker source, checks the shared tracking proof root for
  parent overview, device detail, notification history/preferences, production
  retention-settings write result, rendered child-device check-in and location
  consent, safe/help response, cross-surface accessibility, and end-to-end
  product UI trace artifacts, writes WP30/WP33 artifacts, and keeps full product
  UI runtime, child-device delivery runtime, physical-device behavior,
  authority, provider delivery, production product UI, and product-ready
  tracking unclaimed until those runtime artifacts exist.
- WP30/WP33 full product UI runtime preflight proof now exists through
  `node scripts/test/tracking-full-product-ui-runtime-preflight-proof.mjs`.
  It consumes the runtime artifact gate and turns the four still-missing hard
  product UI runtime refs into manual-required acceptance rows: retention
  settings production write-result UI, rendered child-device check-in,
  rendered child-device location consent, and child-device safe/help response.
  The proof writes WP30/WP33 artifacts plus a manual validation runbook and is
  now consumed by product-readiness closure and real-runtime handoff accounting.
  Full product UI runtime, actual child-device runtime, physical-device,
  authority, provider-delivery, production UI, and product-ready claims remain
  false.
- WP30/WP33 full product UI local runtime artifact capture proof now exists
  through
  `node scripts/test/tracking-full-product-ui-local-runtime-artifact-capture-proof.mjs`.
  It derives the local parent overview shell screenshot, parent devices shell
  screenshot, notification history/preferences screenshot, hosted child check-in
  and child-runtime UI readiness screenshots, cross-surface accessibility
  report, and local end-to-end trace from the hosted UI proof stack into
  `output/tracking-plan-proof/product-parent-child-ui-runtime/`, writes WP30 and
  WP33 proof artifacts, now consumes the retention product-settings writable
  execution derivation proof and child-runtime artifact gate proof as closure
  evidence, and keeps full product UI runtime, child-device runtime,
  physical-device, authority, provider-delivery, production UI, and
  product-ready claims false. The local capture proof records eight local
  hosted parent-route artifacts, one retention writable execution row/derivation, and
  ten missing child-runtime artifacts from the real child-device runtime gate.
  The hard runtime UI artifacts still missing from the product UI artifact root
  are the retention settings production write-result UI, rendered child-device
  check-in, rendered child-device location consent, and child-device safe/help
  response; the full product UI runtime preflight now carries those four refs as
  manual-required rows in closure/handoff proof instead of treating local hosted
  screenshots as product runtime proof.
- WP33 production durable workers readiness blocker proof now exists through
  `node scripts/test/tracking-production-durable-workers-readiness-blocker-proof.mjs`.
  It consumes the production-support durable queue runtime boundary plus the
  existing provider-runtime, escalation-runtime, and retention durable-settings
  proof refs, writes WP33 artifacts, and records manual-required blockers for
  tracking location upload, retention cleanup, notification outbox, escalation
  timeout, provider receipt, child-device delivery, authority status, and
  production audit durable storage workers. The aggregate now preserves the
  production worker runtime artifact inventory from the artifact gate, including
  required, present, and missing artifact refs/counts. Production support
  boundary evidence is acknowledged, but tracking production worker execution,
  production durable storage, and product-ready tracking remain false.
- WP33 production worker runtime artifact gate proof now exists through
  `node scripts/test/tracking-production-worker-runtime-artifact-gate-proof.mjs`.
  It imports the required `tracking-production/*.json` artifact refs from the
  production durable workers readiness blocker source, checks the shared
  tracking proof root for location-upload, retention-cleanup, notification
  outbox, escalation-timeout, provider-receipt, child-device-delivery,
  authority-status, and audit durable storage runtime artifacts, writes WP33
  artifacts, and keeps production worker execution, durable production storage,
  physical-device behavior, authority, provider delivery/receipt runtime, and
  product-ready tracking unclaimed until those real production artifacts exist.
- WP33 production worker runtime preflight proof now exists through
  `node scripts/test/tracking-production-worker-runtime-preflight-proof.mjs`.
  It consumes the production worker runtime artifact gate and turns the eight
  missing production worker runtime refs into manual-required acceptance rows:
  location upload, retention cleanup, notification outbox, escalation timeout,
  provider receipt, child-device delivery, authority status, and audit durable
  storage. The proof writes WP33 artifacts plus a generated manual validation
  runbook, and product-readiness closure plus real-runtime handoff now carry
  those row counts forward while production worker execution, durable production
  storage, provider receipt runtime, authority runtime, physical-device proof,
  and product-ready tracking remain false.
- WP26/WP33 provider-delivery artifact gate proof now exists through
  `node scripts/test/tracking-provider-delivery-artifact-gate-proof.mjs`. It
  checks the provider-delivery proof root for the required real runtime
  artifacts, including redacted runtime config, credential-presence
  attestation, minimal payload, provider attempt/response, receipt webhook and
  ingestion result, retry/quiet-hours worker log, parent notification UI
  screenshot, and result summary. It writes WP26/WP33 artifacts and keeps
  provider delivery runtime, receipt ingestion runtime, credentials, adapter
  dispatch, retry/quiet-hours runtime, production durable outbox storage,
  child-device delivery, physical-device proof, authority, and product-ready
  tracking unclaimed until those real provider artifacts exist.
- WP26/WP33 provider-runtime readiness blocker proof now exists through
  `node scripts/test/tracking-provider-runtime-readiness-blocker-proof.mjs`.
  It consumes the provider-notification proof, receipt-boundary proof, local
  outbox readiness proof, and provider-delivery artifact gate, writes WP26/WP33
  artifacts, and records manual-required blockers for provider delivery runtime,
  webhook receipt ingestion runtime, credentials, adapter dispatch,
  retry/quiet-hours runtime, parent notification UI runtime, production durable
  outbox storage, child-device delivery, physical-device proof, authority, and
  product-ready tracking until the real provider-runtime artifacts exist. The
  aggregate now preserves the provider delivery runtime artifact inventory from
  the artifact gate, including required, present, and missing artifact
  refs/counts with zero provider runtime artifacts present.
- WP27/WP33 escalation-runtime readiness blocker proof now exists through
  `node scripts/test/tracking-escalation-runtime-readiness-blocker-proof.mjs`.
  It consumes the escalation readiness proof and provider-runtime blocker proof,
  writes WP27/WP33 artifacts, and records manual-required blockers for
  production escalation workers, production quiet-hours timers, provider
  delivery/receipt runtime, provider credentials, parent notification history
  runtime, child-device delivery, durable escalation storage, physical-device
  proof, authority proof, emergency auto-contact policy, and product-ready
  escalation until real runtime artifacts exist.
- WP27/WP33 escalation-runtime artifact gate proof now exists through
  `node scripts/test/tracking-escalation-runtime-artifact-gate-proof.mjs`.
  It consumes the escalation-runtime readiness blocker proof, classifies the
  required runtime artifacts as present or missing, writes WP27/WP33 artifacts,
  and keeps production escalation worker, quiet-hours timer, provider runtime,
  parent notification history runtime, child-device delivery, durable storage,
  emergency auto-contact, physical-device, authority, and product-ready claims
  false.
- WP30 child-runtime product-readiness blocker proof now consumes the snapshot
  requirements proof, Android emulator readiness bridge, and parent-child local
  runtime bridge. It records package launch, foreground-service, local emulator
  geofence counts, stored local runtime events, zero dead letters, child-agent
  phase coverage, and parent read-model projection while keeping actual
  child-device delivery/execution, rendered child-device UI, parent receipt
  runtime, physical-device proof, authority, provider delivery, production
  workers, and product-ready tracking unclaimed.
- WP33 tracking product-readiness closure proof now exists through
  `node scripts/test/tracking-product-readiness-closure-proof.mjs`. It cites the
  current pre-device, Android emulator, iOS simulator, WSL/local, hosted UI
  artifact inventory, iOS privacy disclosure release gate, Android system
  geofence blocker, notification receipt boundary, notification preference
  preflight, notification local outbox readiness, authority-enrollment
  manual-required, child-runtime blocker, child-runtime artifact gate, full
  product UI readiness blocker, full product UI local runtime artifact capture,
  full product UI runtime artifact gate, full product UI runtime preflight,
  production durable workers readiness
  blocker, production worker runtime artifact gate, production worker runtime
  preflight, physical-device artifact gate,
  provider-delivery artifact gate,
  provider-runtime readiness blocker, escalation-runtime readiness blocker, and
  retention product-readiness blocker proof refs, retention runtime artifact
  gate proof refs, and tracking claim-audit proof refs, writes
  `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json`,
  explicitly enumerates retention writable-settings and platform-enforcement
  blockers from the retention product-readiness proof, carries aggregate
  evidence for eight observed full-product UI local artifacts, one retention
  writable execution derivation row, ten required child-runtime artifacts with
  zero present and ten missing, two required retention runtime artifacts with
  one present local writable execution artifact and one missing platform
  retention runtime enforcement artifact, eight required production-worker
  runtime artifacts with zero present and eight missing, eight production-worker
  preflight rows with eight required artifacts, zero present artifacts, and
  eight missing artifacts, four full-product UI
  runtime preflight rows with four required artifacts, zero present artifacts,
  and four missing artifacts, and the claim-audit row
  counts. The claim-audit blocker rows are now tier-classified as seven
  physical-device/runtime rows, one approved-manual desktop row, one manual
  provider-runtime row, and two production-runtime rows. It keeps Android/iOS
  physical background, actual child-device runtime,
  full product parent/child UI, authority, provider delivery/receipt, production
  workers, and product-ready tracking unclaimed.
- WP33 tracking claim-audit proof now exists through
  `node scripts/test/tracking-claim-audit-proof.mjs`. It verifies Android
  physical background/geofence, iOS physical background/region,
  approved-manual desktop location, actual child-device runtime, full product
  parent/child UI runtime, authority, provider-delivery receipt runtime,
  retention product settings writable runtime, retention product runtime
  enforcement, production durable workers/storage, and escalation runtime
  artifacts before any final tracking claim can move
  beyond `manual-required`; its full product UI row now carries the runtime gate
  and local runtime artifact capture supporting proof refs, records five present
  required runtime-root artifact refs plus the local capture proof's eight hosted
  artifacts, and keeps the four hard full-product UI runtime gaps still required
  before that claim can advance. Its summary now classifies the
  eleven final manual-required claim rows into seven physical-device/runtime rows,
  one approved-manual desktop row, one manual provider-runtime row, and two
  production-runtime rows. Each row now carries a generated acceptance matrix
  with required criteria, manual validation commands, and artifact acceptance
  notes that keep local P3 evidence separate from the required P4 handoff. It writes
  `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/65-claim-audit-proof.json`
  and keeps every audited claim and product-ready tracking false.
- WP01/WP02 source reconciliation and current gap-map proof now exists through
  `node scripts/test/tracking-source-reconciliation-gap-map-proof.mjs`. It
  verifies the source index, current tracking snapshot, source docs, product
  checklist row, implementation checklist, pasted-content audit, and
  product-readiness closure blockers stay aligned while keeping
  physical-device, authority, provider delivery/receipt, production, and
  product-ready tracking claims false.
- WP33 real-runtime handoff proof now exists through
  `node scripts/test/tracking-real-runtime-handoff-proof.mjs`. It derives the
  remaining P4/manual artifact rows from the existing Android physical-device,
  iOS physical-device, child-runtime, full-product UI, authority, provider-delivery,
  retention-runtime, production worker, escalation-runtime, and
  product-readiness closure proofs through the parent-domain
  `tracking-real-runtime-handoff-proof` schema/parser and focused test, writes
  the WP33 handoff artifact plus a generated manual validation runbook with
  row-level blocker ids, validation commands, acceptance notes, and missing
  artifacts. It now carries the product-readiness closure accounting totals,
  including child-runtime, retention-runtime, production-worker,
  production-worker runtime preflight, and full-product UI runtime preflight
  required/present/missing artifact counts plus the claim-audit tier split,
  classifies seven physical/runtime rows, one provider-runtime row, and two
  production-runtime rows as
  non-CI-runnable manual follow-up, and keeps physical-device, actual
  child-device runtime, full product UI, authority, provider delivery/receipt,
  retention product runtime, production, escalation, and product-ready tracking
  unclaimed. The handoff rows now also consume the claim-audit acceptance
  matrix so the generated manual validation runbook carries the same
  acceptance criteria, manual commands, and claim-approval-false artifact notes
  as the claim audit.
- WP31/WP33 authority-runtime readiness blocker proof now exists through
  `node scripts/test/tracking-authority-runtime-readiness-blocker-proof.mjs`.
  It consumes the existing authority-enrollment manual-required proof rows,
  writes WP31/WP33 artifacts, and records authority-required blockers for
  Android device-owner enrollment, Android managed-profile enrollment, iOS
  Family Controls entitlement, iOS App Review approval, desktop managed policy,
  hard-control runtime, parent-visible authority status, physical-device
  authority proof, production authority workers, and product-ready authority
  behavior. It keeps enrolled authority, hard-control runtime, physical-device
  behavior, production workers, and product-ready tracking unclaimed.
- WP32 hosted report/policy consumer rendering proof now exists through
  `npm run test:tracking-plan-hosted-ui-proof`. It renders parent report
  summary, policy evidence drill-in, and retention audit export consumer rows
  that cite stored journal refs and stored read-model refs, captures
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-report-policy-consumer.png`,
  and writes
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/25-report-policy-consumer-hosted-ui-proof.json`
  plus
  `output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/32-report-policy-consumer-hosted-ui-proof.json`
  while keeping AI execution, product policy mutation, platform runtime,
  child-device delivery, provider delivery, notification receipt ingestion,
  physical-device proof, authority, production, and product-ready tracking
  unclaimed.
- WP31 platform extension inventory proof now exists through
  `node scripts/test/tracking-platform-extension-inventory-proof.mjs`. It
  verifies the existing Android emulator/status/manual-required artifacts, iOS
  simulator/manual-required artifacts, desktop hint-only artifact, and hosted
  unsupported/manual platform UI artifact in one evidence bundle while keeping
  Android/iOS physical-device behavior, background runtime, precise desktop
  location, authority enrollment, provider delivery, production upload workers,
  and product-ready tracking unclaimed.
- WP33 physical-device artifact gate proof now exists through
  `node scripts/test/tracking-physical-device-artifact-gate-proof.mjs`. It
  checks the Android physical background/geofence and iOS region-monitoring proof
  roots for the exact required artifact names, writes the gate proof under WP33
  plus `output/tracking-plan-proof/tracking-physical-device-artifact-gate-proof/`,
  and now generates a manual validation runbook with per-platform acceptance
  criteria, physical-device commands, and artifact acceptance notes. The rows stay
  `manual-required` with physical-device behavior, authority enrollment, provider
  delivery, production workers, and product-ready tracking unclaimed until those
  real-device artifacts exist and are reviewed.
- WP08 Android emulator foreground proof now exists through
  `npm run test:tracking-plan-android-emulator-proof`. It builds and installs
  the debug APK on `Pixel_9_Pro_XL_API_35`, observes foreground-service state,
  grants declared foreground location runtime permissions, and captures app UI
  text for `foreground-location-permission-granted` plus
  `current-location-sample-observed-emulator-location-manager` with provider
  `gps`, source `android-location-manager-current-listener-emulator`, observed
  epoch millis, accuracy meters, and raw latitude/longitude export in
  `output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json`.
  It also captures the Android permission-controller foreground location UX
  dialog before the scripted grant, including precise/approximate,
  while-using, one-time, and deny options in
  `test-results/tracking-plan-android-emulator-proof/13-foreground-location-permission-ux.json`.
  This is local emulator proof only; background/geofence transitions,
  physical-device behavior, authority, provider delivery, production upload
  workers, and product-ready Android tracking remain unclaimed.
- WP09 Android emulator background permission, foreground-service-backed
  background-activity sample, and local-geofence enter/exit proof now exists
  through `npm run test:tracking-plan-android-emulator-proof`. It declares
  `ACCESS_BACKGROUND_LOCATION`, grants it on the API 35 emulator, backgrounds
  the activity while the foreground service listens for emulator GPS updates,
  attempts to launch and capture the package app-details Settings page as the
  Android 11+ background-location settings-page flow artifact, but the current
  ATD emulator image has no resolvable Settings activity, so Settings routing
  remains unclaimed for that run,
  records app-owned background sample proof storage with provider/timestamp/
  accuracy, drives an outside/inside/inside-dwell/outside emulator `geo fix` route, records
  app-owned `LocationManager` GPS-listener local-geofence transition and dwell rows,
  records the active app-owned local geofence count against Android's documented
  100 geofences per app per device user limit, bridges to the WP10
  low-power/app-restart/pending-upload/manual-required status-gap rows, records
  Android `LocationManager.addProximityAlert` registration metadata separately,
  now splits Android proximity-alert broadcast transition counters from
  app-owned local listener transition counters, and preserves the final app
  UI/manual-required boundary separately from the app-owned proof storage rows.
  The generated proof lives at
  `output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json`.
  This is local emulator proof only. Android system geofence delivery remains
  unclaimed unless the separate system-proximity broadcast transition count is
  nonzero; Android system dwell transitions, physical-device behavior, authority, provider
  delivery, production upload workers, and product-ready Android tracking
  remain unclaimed.
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
- WP26 tracking notification local outbox readiness proof now exists through
  `node scripts/test/tracking-notification-local-outbox-readiness-proof.mjs`.
  It maps those tracking receipt boundary rows to the existing notification
  local outbox adapter and scheduler proof rows, preserving tracking evidence,
  policy, receipt-requirement, local outbox artifact, and scheduler artifact
  refs while keeping provider delivery, receipt ingestion runtime, credentials,
  cloud routing, parent notification UI, retry/quiet-hours workers,
  production durable outbox storage, child-device delivery, physical-device
  proof, authority, and product-ready notification behavior unclaimed.
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
- WP26/WP33 tracking notification preference status handoff proof now exists
  through
  `node scripts/test/tracking-notification-preference-status-handoff-proof.mjs`.
  It maps preference preflight rows into V3 notification preference and
  quiet-hours status entries while preserving provider attempt, provider
  preference, evidence, policy decision, notification status, reason, and
  quiet-hours refs. It keeps parent notification preference UI/history UI,
  preference mutation runtime, quiet-hours timer runtime, provider delivery,
  receipt runtime, credentials, cloud routing, child-device delivery,
  physical-device proof, authority, retry workers, production durable outbox
  storage, adapter dispatch, and product-ready notification behavior unclaimed.
- WP26 tracking notification parent-surface history intent proof now exists
  through
  `node scripts/test/tracking-notification-parent-surface-history-proof.mjs`.
  It joins tracking provider-notification, receipt boundary, and preference
  preflight proof rows into redacted parent history/preference intent rows,
  preserving provider attempt refs, receipt requirements, preference and
  quiet-hours requirements, evidence refs, policy decision refs, notification
  status refs, reason refs, audit refs, manual-proof requirements, and
  authenticated drill-in refs. It is a parent-domain/read-model intent proof
  only; rendered parent notification UI, parent preference mutation runtime,
  frequency controls, quiet-hours timer runtime, provider delivery, receipt
  ingestion runtime, credentials, cloud routing, child-device delivery,
  physical-device proof, authority proof, retry workers, durable production
  history/outbox storage, and adapter dispatch remain unclaimed.
- Hosted parent route notification history screenshot/accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It renders the
  existing WP26 parent-surface history/preference intent rows on the hosted
  `policy-tracking` route, captures the notification parent-surface screenshot,
  records accessibility/no-overlap assertions, and writes WP26/WP30/WP33 proof
  artifacts while keeping parent preference mutation, quiet-hours runtime,
  provider delivery, receipt ingestion runtime, child-device delivery,
  physical-device proof, authority, production storage, adapter dispatch, and
  product-ready tracking unclaimed.
- Hosted parent route parent action readiness screenshot/accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It renders the
  existing WP16 expected-place alert policy rows and WP17 parent acknowledgement
  action readiness rows on the hosted `policy-tracking` route, captures the
  parent action readiness screenshot, records accessibility/no-overlap
  assertions, and writes WP16/WP17/WP30/WP33 proof artifacts while keeping live
  service mutation, alert delivery, provider delivery, receipt ingestion,
  child-device runtime, physical-device proof, authority, production workers,
  adapter dispatch, and product-ready tracking unclaimed.
- Hosted parent route missing-device state screenshot/accessibility proof now
  exists through `npm run test:tracking-plan-hosted-ui-proof`. It renders the
  existing WP29 last-known-only, offline/powered-off, contact-requested, and
  manual-required missing-device rows on the hosted `policy-tracking` route,
  captures the missing-device screenshot, records accessibility/no-overlap
  assertions, and writes WP29/WP30/WP33 proof artifacts while keeping current
  location runtime, powered-off tracking, remote sync, provider delivery,
  physical-device proof, OS lost-mode API execution, authority, production
  workers, and product-ready tracking unclaimed.
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
  unavailable degradation. The same proof now emits provider parity readiness
  rows for Google, Apple MapKit, and OpenStreetMap/Nominatim: Google is
  request-mapped from the bounded contract proof, while Apple and OSM remain
  manual-required until provider terms, runtime, and authorization proof exist.
  It does not claim live Google/Apple/OSM execution, credentials, exact place,
  or physical-device proof.
- WP21 place-category ambiguity proof now exists through
  `node scripts/test/tracking-place-category-ambiguity-proof.mjs`. It builds
  parent-domain review rows from the existing POI adapter and proves
  no-accusation copy, low-accuracy ambiguity, multiple-place ambiguity,
  category-as-policy-input-only, and parent-defined zone override as
  policy-review input only. It does not claim live provider execution,
  provider delivery, exact-place presence, automatic action, physical-device
  proof, authority proof, production behavior, or full UI.
- WP24 AI provider routing proof now exists through
  `node scripts/test/tracking-ai-provider-routing-proof.mjs`. It proves a
  parent-domain route matrix with child-local as the only default safety path,
  parent-approved remote as the only remote-data route, degraded/unavailable/
  disabled rows, assistant preview-only no-write/no-enforcement boundaries, and
  evidence/custody refs on every AI context. It does not claim model execution,
  child-device runtime, provider delivery, assistant policy writes,
  enforcement, physical-device proof, production behavior, or UI.
- WP25 tracking policy compiler runtime proof now exists through
  `node scripts/test/tracking-policy-compiler-runtime-proof.mjs`. It compiles
  parent-owned tracking rules for observe, notify, child check-in, parent
  acknowledgement, temporary live tracking, escalation, critical alert,
  suppress/no-action, and manual-required outcomes, keeps AI as evidence rather
  than final authority, and records no runtime enforcement, provider delivery,
  platform adapter, production worker, UI-complete, or physical-device claims.
- WP16 expected-place alert policy proof now exists through
  `node scripts/test/tracking-expected-place-alert-policy-proof.mjs`. It maps
  expected-place policy decisions into parent alert, child check-in,
  suppression, and manual-required UI-readiness rows while preserving schedule
  rule refs, policy decision refs, alert intent refs, evidence refs, reason
  refs, audit refs, and UI surface refs. It does not claim rendered parent UI,
  alert delivery runtime, provider delivery, notification receipt runtime,
  child-device runtime, physical-device proof, authority proof, production
  workers, or adapter dispatch.
- WP17 parent acknowledgement action readiness proof now exists through
  `node scripts/test/tracking-parent-acknowledgement-action-readiness-proof.mjs`.
  It maps existing tracking alerts, acknowledgements, exceptions, false-alarm
  rows, child check-in decisions, and escalation rows into parent action
  readiness rows while preserving evidence refs, policy decision refs, alert
  refs, acknowledgement refs, escalation refs, audit refs, expiry refs, and UI
  surface refs. It does not claim rendered portal acknowledgement UI, live
  service mutation, provider delivery, notification receipt runtime,
  child-device runtime, physical-device proof, authority proof, production
  workers, or adapter dispatch.
- WP18 child check-in timeout escalation proof now exists through
  `node scripts/test/tracking-child-check-in-timeout-escalation-proof.mjs`. It
  maps existing child check-in requests and responses through the runtime
  resolver into waiting, safe response, help response escalation, call-parent
  escalation, and expired timeout escalation rows while preserving request,
  response, alert, evidence, policy, audit, parent action, timeout, and optional
  location-evidence refs. It does not claim child-device delivery runtime,
  child-device response runtime, rendered child UI, provider delivery,
  notification receipt runtime, live location sample runtime, physical-device
  proof, authority proof, production timeout workers, or adapter dispatch.
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
- WP32 report/policy consumer proof now exists through
  `node scripts/test/tracking-report-policy-consumer-proof.mjs`. It derives
  parent report summary, policy evidence drill-in, and retention audit/export
  consumer-readiness rows from the existing tracking service read-model and
  active product-surface summary proof, keeps source proof refs, evidence refs,
  stored journal refs, stored read-model row refs, policy decision refs, report
  surface refs, and audit refs attached, and explicitly rejects portal UI
  completion, child-device delivery, provider delivery, notification receipt
  ingestion, authority, physical-device, AI execution, and product-complete
  claims.
- WP24/WP32 AI stored-ref consumer proof now exists through
  `node scripts/test/tracking-ai-stored-ref-consumer-proof.mjs`. It derives AI
  parent-report context, policy-drill-in context, and metadata-fallback context
  rows from the existing AI provider-route proof and report/policy consumer
  proof, requires stored journal refs plus stored read-model row refs before AI
  report/policy use, and keeps AI model execution, assistant policy writes,
  assistant enforcement, child-device runtime, provider delivery, notification
  receipt ingestion, authority, physical-device, production, and product-ready
  claims false.
- WP32 hosted storage default boundary proof now exists through
  `node scripts/test/tracking-hosted-storage-default-boundary-proof.mjs`. It
  keeps tracking journal, SQLite read-model, parent export, AI context, and
  remote-sync custody local, parent-owned, or remote-disabled by default, and
  keeps Ocentra-hosted default storage, raw location remote upload, SQLite
  snapshot remote upload, remote sync, remote AI, portal UI, service mutation,
  platform runtime, device delivery, authority, production, and product-ready
  claims false.
- WP32 report/export read-model proof now exists through
  `node scripts/test/tracking-report-export-read-model-proof.mjs`. It derives
  redacted report export, retention audit export, family dashboard summary, and
  policy drill-in export packet rows from the existing tracking service
  read-model, product-surface summary, report/policy consumer, family dashboard
  rollup, and retention settings proof refs while keeping raw location payload
  export, rendered report UI, service mutation, platform runtime, child-device
  delivery, provider delivery, notification receipt ingestion, authority,
  physical-device, production, and product-complete claims false.
- Hosted report/export packet rendering now exists through
  `npm run test:tracking-plan-hosted-ui-proof`. The parent `policy-tracking`
  route renders those redacted report/export packet rows and captures
  `output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-report-export.png`,
  while raw location payload export, service mutation, platform runtime,
  child-device delivery, provider delivery, notification receipt ingestion,
  authority, physical-device proof, production, and product-ready export behavior
  remain unclaimed.
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
  service state, fused foreground sample state, UI tree, screenshot, logcat,
  battery, connectivity dumps, and
  a WP10 status-gap bridge for local low-power, killed/restarted,
  pending-upload, and manual-required rows,
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
- WP08 Android emulator proof now records foreground permission-controller UX
  dialog, foreground permission grant, app-emitted current `LocationManager`
  provider/timestamp/accuracy/source metadata, Google Play Services fused
  foreground sample provider/timestamp/accuracy/source metadata, and raw
  latitude/longitude proof export under
  `output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json`.
  This is emulator-only proof; physical Android device behavior, authority,
  provider delivery, notification delivery, and product-ready tracking remain
  unclaimed.
- WP10 Android status proof now exists through
  `node scripts/test/tracking-android-status-proof.mjs`. It records
  parent-domain rows for low-power degradation, app killed/restarted
  auditability, pending-upload count auditability, and manual-required platform
  proof while keeping foreground location samples, background runtime, geofence
  transitions, notification delivery, device-owner authority, physical-device
  behavior, production upload workers, and product-ready Android tracking
  unclaimed.
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
- WP11/WP12 iOS manual-required rows are now also wrapped by a WP33 companion
  proof gate through `node scripts/test/tracking-ios-location-wp33-gate-proof.mjs`,
  which records the same seven manual-required rows under the rollout/PR gate
  without adding a duplicate iOS location contract or claiming Core Location
  runtime, entitlement, notification delivery, physical-device, authority, or
  product-ready iOS tracking.
- WP12 iOS App Store/privacy disclosure release-gate proof now exists through
  `node scripts/test/tracking-ios-privacy-disclosure-release-proof.mjs`. It records
  location-purpose, background-location, region-monitoring, notification,
  data-custody, App Store review, and privacy label evidence rows under WP12
  and WP33 proof artifacts while keeping release, product-ready iOS tracking,
  Core Location runtime, entitlement, TestFlight/device, physical-device,
  authority, and notification delivery claims blocked.
- WP31/WP33 authority enrollment manual-required proof now exists through
  `node scripts/test/tracking-authority-enrollment-manual-required-proof.mjs`.
  It records Android device-owner, Android managed-profile, iOS Family Controls
  entitlement, iOS App Review approval, and desktop managed-policy evidence
  rows while keeping authority enrollment, hard-control runtime,
  physical-device behavior, provider delivery, production workers, and
  product-ready tracking unclaimed.
- Platform permissions, mobile physical-device proof, full runtime adapters,
  provider delivery, notification delivery, actual child-device delivery/runtime
  execution, full parent/child UI beyond the hosted parent route, and broader
  read-model product surfaces beyond the report/policy consumer-readiness rows
  and hosted service-data coverage remain not product-complete.
- Hosted parent route screenshot and accessibility artifacts now have a focused
  inventory proof through
  `node scripts/test/tracking-hosted-ui-artifact-inventory-proof.mjs`. It
  verifies the stored hosted screenshot PNGs, including report/export,
  notification parent-surface, parent action readiness, missing-device, and
  unsupported/manual platform screenshots, hosted evidence drawer proof,
  unsupported/manual platform proof, accessibility assertions, 10-card
  no-overlap layout geometry, and no-product-claim boundary while keeping full
  parent/child UI beyond the hosted route, child-device runtime,
  physical-device proof, authority, provider delivery, production proof, and
  product-ready tracking unclaimed.
- Raw tracking/location settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Location/geofence is now in contract, P1 fixture/runtime proof, narrow P2
service read-model proof with retention-delete tombstone replay and active
product-surface summary fields, narrow portal summary-consumption proof, live
service-backed portal citation rows, hosted
parent route service-data coverage, hosted parent route screenshot/accessibility
proof with hosted child-safe check-in copy/actions, hosted child-runtime
disclosure/safe-help/location-share consent copy, hosted read-only evidence
drawer drill-in, hosted family dashboard rollup card, and hosted retention
settings read-model card, P1 local parent-defined place
store proof, pre-device proof-gate progress, Android emulator package/service/status
scaffold proof, WP08/WP09 Android permission/background manual-required proof,
WP11/WP12 iOS Core Location manual-required proof, P3 WSL/local replay proof,
P1 evidence-quality gate proof, WP26 tracking alert-to-provider-status handoff,
notification receipt boundary proof, notification preference preflight proof,
local outbox readiness proof, and parent-surface notification history intent
proof, P1 escalation readiness proof for
acknowledgement/check-in/manual
escalation states, WP28 temporary live tracking mode proof for authorization/
duration/cadence/degraded/auto-stop/retention states, and WP20 Google Places/
POI provider request/response mapping proof, WP25 parent-policy compiler/
evaluator runtime proof, and required fixture-state coverage proof, WP29
missing-device mode parent-domain proof for last-known-only/offline/
contact-requested/manual-required states plus hosted missing-device route
screenshot/accessibility proof, manual-required/unavailable platform render-state
proof plus hosted unsupported/manual platform route screenshot, and
WP32 report/policy consumer-readiness proof for parent report summary, policy
drill-in, and retention audit/export rows, plus WP32 family dashboard rollup
proof for active family, child-attention, and retention-audit summary rows with
narrow hosted-route rendering, and retention settings read-model proof for
retention window, delete-after-alert, parent export, remote-sync disabled, and
remote-AI disabled rows plus narrow hosted-route rendering, plus
retention-settings writer-boundary preflight proof for the same five settings
rows, plus typed service transport local write-command proof for retention
settings execution, plus hosted route command/result rendering for that typed
local write result, plus local durable settings persistence proof, plus WP32
report/export read-model packet proof for redacted report, retention audit,
family dashboard summary, and policy drill-in export packets, plus WP24/WP32 AI
stored-ref consumer proof for AI parent-report,
policy-drill-in, and metadata-fallback context rows,
plus hosted notification parent-surface history/preference intent rendering
with screenshot/accessibility proof, hosted missing-device state rendering
with screenshot/accessibility proof, and local outbox/scheduler artifact
readiness proof for tracking receipt rows, plus child-runtime
product-readiness blocker proof over the existing delivery/execution/snapshot
requirement rows and a child-runtime artifact gate for the real device
delivery/execution artifact set.
It remains a tracked product gap until platform location and
geofence runtime adapters, broader product read models beyond these rows, full
dashboard UI beyond the hosted parent route, applied product-ready service
mutation execution, actual live provider execution/delivery, notification
receipt ingestion runtime beyond hosted history rendering, physical-device
proof, actual child-device delivery/runtime execution, remote sync runtime, OS
lost-mode APIs, production upload workers, and full parent/child UI snapshots/
accessibility beyond the hosted parent route are proved.

## Checklist

- [x] Location evidence contract.
- [x] Accuracy/source/stale-state fields.
- [x] Geofence rule and transition contracts.
- [x] Battery/connectivity status contract.
- [ ] Retention/delete/export settings. P1 retention-delete,
      parent-owned export, UI-visible deleted-history hiding fixture proof, and
      P2 retention settings read-model rows plus hosted route rendering now
      exist. Writer-boundary preflight proof now validates the retention window,
      delete-after-alert, parent export, remote-sync disabled, and remote-AI
      disabled write intents, and the service transport now accepts a typed
      local write command/result. The hosted parent route now sends and
      renders that local service execution result. Local service-state readback,
      local durable settings persistence proof, and the retention
      product-settings writable execution artifact now exist; applied
      product-ready writable retention execution, platform runtime, and
      production hardening remain pending. Product-readiness blocker proof now
      enumerates the remaining blockers over the durable-settings evidence, and
      retention runtime artifact gate proof now checks the writable product
      settings and platform retention runtime artifact refs without upgrading
      the retention feature to product-ready.
- [x] Alert intent contract.
- [x] Expected-place alert policy UI-readiness rows. This proves parent
      alert/check-in/suppression/manual readiness rows from expected-place
      decisions and schedule rule refs. It is not rendered parent UI, alert
      delivery runtime, provider delivery, receipt runtime, child runtime,
      physical-device, authority, production worker, or adapter dispatch proof.
- [x] Tracking notification parent-surface history intent rows. This proves
      redacted parent history/preference readiness rows over the existing
      provider, receipt, and preference proof refs. It is not rendered
      notification UI, parent preference mutation, provider delivery, receipt
      runtime, child-device delivery, physical-device, authority, or
      product-ready notification behavior.
- [x] Tracking notification local outbox readiness rows. This proves tracking
      receipt rows can cite the existing parent-owned local outbox adapter and
      scheduler artifact refs. It is not provider delivery, receipt ingestion
      runtime, production durable outbox storage, retry/quiet-hours worker
      execution, child-device delivery, physical-device proof, authority, or
      product-ready notification behavior.
- [x] Tracking authority runtime artifact gate. This consumes the existing
      authority enrollment/manual-required and runtime-readiness blocker proofs,
      classifies the required enrolled-device authority evidence artifacts as
      present or missing, and keeps authority enrollment, hard-control runtime,
      parent-visible authority status, physical-device, production worker, and
      product-ready claims false.
- [x] Hosted notification parent-surface history UI proof renders those rows on
      the hosted parent `policy-tracking` route with screenshot/accessibility
      evidence. This is not parent preference mutation, quiet-hours runtime,
      provider delivery, receipt ingestion runtime, child-device delivery,
      physical-device, authority, production storage, adapter dispatch, or
      product-ready notification behavior.
- [ ] Android permission/background runtime proof. WP08 emulator foreground
      permission/sample metadata and WP09 emulator background permission,
      Android 11+ settings-page route-attempt artifact, foreground-service-backed
      background-activity sample, plus local-geofence enter/exit proof now
      exist, and WP08/WP09 parent-domain manual-required
      proof rows still preserve the remaining Android system geofencing, dwell,
      and device gaps. Android system geofence blocker proof now records that
      app-owned local listener transitions exist while Android system proximity
      broadcast counters are zero, and it separately classifies the missing
      system-proximity broadcast, dwell-transition, physical-device, and
      authority runtime artifacts; Android system geofencing/dwell and
      physical-device proof remain pending.
- [x] Android emulator package launch, foreground-service scaffold, battery,
      connectivity, and Samsung S9 physical status proof. This is not
      foreground location, offline radio behavior, physical geofence behavior,
      production upload worker, or product-ready Android proof.
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
- [x] iOS WP33 companion gate records the same WP11/WP12 manual-required rows
      under `output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/27-ios-location-manual-required-proof.json`
      while preserving all Core Location runtime, entitlement,
      notification-delivery, physical-device, authority, and product-ready
      non-claims.
- [x] iOS App Store/privacy disclosure release-gate proof records required
      release evidence rows under
      `output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/20-ios-privacy-disclosure-release-proof.json`
      and the WP33 companion artifact while keeping release/product-ready iOS
      tracking blocked until disclosure, Apple review, entitlement,
      TestFlight/device, and runtime Core Location proof exist.
- [x] Authority enrollment manual-required proof records Android device-owner,
      Android managed-profile, iOS Family Controls entitlement, iOS App Review,
      and desktop managed-policy evidence requirements under WP31/WP33 without
      claiming authority enrollment, hard-control runtime, physical-device
      behavior, provider delivery, production workers, or product-ready
      tracking.
- [x] Expected-place schedule and exception contracts.
- [x] Parent acknowledgement and escalation contracts.
- [x] Parent acknowledgement action readiness rows. This proves parent
      acknowledge-safe, expected/exception, false-alarm, child check-in request,
      and escalation manual-review readiness rows from existing alert and
      acknowledgement contracts. It is not rendered portal acknowledgement UI,
      live service mutation, provider delivery, receipt runtime, child runtime,
      physical-device, authority, production worker, or adapter dispatch proof.
- [x] Hosted parent action readiness rendering for expected-place alert policy
      rows and parent acknowledgement action rows. This is hosted read-only UI
      proof only; it is not live service mutation, alert/provider delivery,
      receipt ingestion, child-device runtime, physical-device, authority,
      production worker, adapter dispatch, or product-ready tracking proof.
- [x] Full product UI runtime artifact gate proof records the exact required
      parent overview, parent device-detail, notification history/preferences,
      retention production write result, rendered child-device check-in,
      rendered child-device location consent, safe/help response,
      cross-surface accessibility, and end-to-end product UI trace artifact
      refs before full product UI runtime can be treated as present. The local
      runtime artifact capture proof now supplies parent overview, parent device
      detail, notification, retention local write-result, hosted child
      check-in/readiness, hosted child-runtime/readiness, cross-surface
      accessibility, and local trace artifacts from the hosted proof stack
      without satisfying the P4 runtime refs. The full product UI runtime
      preflight proof now turns the four remaining hard runtime refs into
      manual-required rows carried by closure/handoff accounting. Rows remain
      manual-required and keep full product
      UI runtime, child-device delivery runtime, physical-device behavior,
      authority, provider delivery, production product UI, and product-ready
      tracking unclaimed.
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
- [x] Child check-in timeout escalation readiness rows. This proves waiting,
      safe, help, call-parent, and expired-timeout rows from the existing child
      check-in resolver and policy refs, including optional location-sample
      request state, attached response location-evidence refs, prompt/response
      audit coverage, alert outcome projection, and rule-only timeout
      escalation basis. It is not child-device delivery, rendered child UI,
      provider delivery, receipt runtime, live location runtime,
      physical-device, authority, production timeout worker, or adapter dispatch
      proof.
- [x] Escalation runtime artifact gate. This consumes the existing
      escalation-runtime readiness blocker proof, classifies required
      escalation runtime artifacts as present or missing, and keeps production
      escalation worker, quiet-hours timer, provider runtime, parent
      notification history runtime, child-device delivery, durable storage,
      emergency auto-contact, physical-device, authority, and product-ready
      claims false.
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
      location copy gates. This is not child-device delivery, physical-device,
      provider delivery, remote sync, or OS lost-mode API proof.
- [x] Hosted missing-device state UI proof renders those WP29 rows on the
      hosted parent `policy-tracking` route with screenshot/accessibility
      evidence. This is not current-location runtime, powered-off tracking,
      remote sync, provider delivery, physical-device proof, OS lost-mode API
      execution, authority, production worker, or product-ready tracking proof.
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
      product-complete proof; the rows now require stored journal refs and
      stored read-model row refs before report/policy use.
- [x] WP24/WP32 AI stored-ref consumer proof for AI parent-report,
      policy-drill-in, and metadata-fallback context rows. This is not AI model
      execution, assistant policy-write, assistant enforcement, child-device
      delivery/runtime execution, provider delivery, notification receipt
      ingestion, authority, physical-device, production, or product-complete
      proof; the rows require provider-route proof refs, report/policy consumer
      proof refs, stored journal refs, and stored read-model row refs before AI
      report/policy use.
- [x] WP32 hosted storage default boundary proof for journal, SQLite
      read-model, parent export, AI context, and remote-sync custody defaults.
      This is not Ocentra-hosted default storage, raw location remote upload,
      SQLite snapshot remote upload, remote sync, remote AI, portal UI, service
      mutation, platform runtime, child-device delivery/runtime execution,
      provider delivery, notification receipt ingestion, authority,
      physical-device, production, or product-complete proof.
- [x] WP32 report/export read-model proof for redacted report export,
      retention audit export, family dashboard summary, and policy drill-in
      export packet rows, plus narrow hosted parent-route rendering of those
      rows. This is evidence-ref packet readiness and hosted packet rendering
      only; it is not raw location payload export, service mutation, platform
      runtime, child-device delivery/runtime execution, provider delivery,
      notification receipt ingestion, authority, physical-device, or
      product-complete proof.
- [x] WP32 family dashboard rollup proof for active family summary,
      child-attention summary, and retention-audit summary rows, plus narrow
      hosted parent-route rendering of those rows. This is not full dashboard
      UI beyond the hosted route, child-device delivery/runtime execution,
      provider delivery, notification receipt ingestion, authority,
      physical-device, or product-complete proof.
- [x] WP32 retention settings hosted UI proof for retention window,
      delete-after-alert, parent export, remote-sync disabled, and remote-AI
      disabled read-model rows on the hosted parent route, plus typed retention
      settings local service execution command/result rendering with a local
      service state revision and visible
      `output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json`
      reference. This is not writable product settings, product-ready service
      mutation execution, platform runtime, child-device delivery/runtime
      execution, provider delivery, authority, physical-device, or
      product-complete proof.
- [x] WP07/WP32 retention settings writer-boundary proof for retention window,
      delete-after-alert, parent export, remote-sync disabled, and remote-AI
      disabled write intents. This is local validation and service-mutation
      preflight only; it is not executed service mutation, live retention UI,
      platform runtime, child-device delivery/runtime execution, provider
      delivery, notification receipt ingestion, authority, physical-device, or
      product-complete proof.
- [x] WP07/WP32 typed retention settings service write command proof. This
      proves protocol request/result parsers, Rust protocol serialization, and
      Rust service WebSocket response for a no-product-claim local execution
      result with a local in-service state revision, snapshot ref, and local
      durable settings store ref. The hosted route now sends and renders that
      result without claiming product-ready service execution, platform runtime,
      child-device delivery/runtime execution, provider delivery, notification
      receipt ingestion, authority, physical-device, or product-complete proof.
- [x] WP07/WP32 retention local service state readback proof. This derives the
      local service state revision, snapshot ref, durable settings store ref,
      and applied retention values from the accepted write-command proof into
      parent-domain rows while keeping writable product settings, platform
      runtime, child-device delivery/runtime execution, provider delivery,
      notification receipt ingestion, authority, physical-device, production,
      and product-complete proof unclaimed.
- [x] WP07/WP32 durable retention settings proof. This derives local durable
      persistence rows from the local service state readback proof, records the
      Rust service durable store ref and persisted state, and keeps writable
      product settings, platform runtime, child-device delivery/runtime
      execution, provider delivery, notification receipt ingestion, authority,
      physical-device, production workers, and product-complete proof
      unclaimed.
- [x] P1 local parent-defined place store proof for CRUD/import/export/delete
      with parent-device-local default storage and remote sync disabled.
- [x] P3 WSL/local replay proof for the tracking read-model proof stack and
      linked-worktree toolchain mapping.
- [x] P1 parent portal tracking-state fixture surface, local parent-route screenshot,
      and local proof artifact references.
- [x] Hosted parent `policy-tracking` route screenshot and accessibility proof
      against the real Rust service and seeded ActivityStore, including a
      rendered service-data coverage card, explicit service-backed citation
      detail card, hosted read-only evidence drawer card, hosted child-safe
      check-in copy/actions card, hosted
      child-runtime disclosure/safe-help/location-share consent card, plus a
      family dashboard rollup card, parent action readiness card,
      missing-device state card, and retention settings local service write result
      card. This is not
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
- [x] Hosted UI artifact inventory proof verifies the stored hosted screenshot
      PNGs, including report/export, notification parent-surface, parent action
      readiness, missing-device, and unsupported/manual platform screenshots,
      evidence drawer proof output, unsupported/manual platform proof output,
      parent overview/devices shell screenshots around the tracking route with
      service-backed tracking summary and no-product-claim assertions,
      accessibility assertions, child-runtime product-readiness blocker proof,
      and 11-card no-overlap layout geometry without claiming full product
      parent/child UI, child-device runtime,
      physical-device proof, authority, provider delivery, production proof, or
      product-ready tracking.
- [x] WP08 Android emulator foreground permission and app-reported current
      `LocationManager` sample metadata plus raw coordinate proof export,
      including foreground permission UX dialog capture. This is local emulator
      evidence only; Google Play Services fused foreground sample metadata is
      observed on the emulator, while background/geofence, physical-device,
      authority, provider delivery, production workers, and product-ready
      Android tracking remain unclaimed.
- [x] WP09 Android emulator background permission,
      Android 11+ app settings-page route-attempt artifact,
      foreground-service-backed background-activity sample, and local-geofence
      enter/exit/dwell proof plus active geofence-limit representation and WP10
      status-gap bridge plus separate proximity-alert registration metadata.
      Android system geofence blocker proof now records zero system proximity
      broadcast counters from that same emulator evidence, local evidence
      artifact refs, and the required Android runtime artifact refs that are
      still missing. This is local emulator evidence only; Android system
      geofence delivery, Android system dwell transitions, physical-device, authority,
      provider delivery, production workers, and product-ready Android tracking
      remain unclaimed.
- [x] WP11/WP12 iOS manual-required proof now records simulator package/manual
      evidence refs plus required, present, and missing Core Location/region
      runtime artifact refs for When In Use authorization, foreground samples,
      degraded states, Always authorization, region transitions,
      significant-change/visit events, background relaunch, and entitlement
      approval. This is artifact accounting only; no iOS Core Location runtime,
      physical-device, authority, provider delivery, production worker, or
      product-ready tracking claim is made.
- [x] Evidence-quality gate proof for tracking UI evidence refs, geofence
      source refs, nearby-place context fields, AI no-final-action constraints,
      alert policy-decision refs, and retention before/after proof. This is not
      live device/provider behavior proof.
- [x] Local platform proof batch aggregates Android emulator runtime evidence,
      Samsung S9 Android physical package/service/status evidence, WSL/local
      replay, hosted parent UI accessibility, product parent/child UI local
      artifacts, product-readiness closure, and real-runtime handoff accounting
      into one proof bundle. This is local/hosted/status proof only; Android
      physical behavior, iOS Core Location runtime, actual child-device runtime,
      authority enrollment, provider delivery, production runtime, and
      product-ready tracking remain unclaimed.
- [ ] Full live parent/child UI screenshots and accessibility proof beyond the
      hosted parent shell. Hosted parent overview/devices shell screenshots now
      render the service-backed tracking summary and no-product-claim cards as
      local/CI proof only, and full-product UI local artifact capture now also
      copies the hosted retention settings local write-result plus hosted child
      check-in/runtime readiness screenshots into the product UI artifact root
      without filling the production write-result or child-device runtime
      artifact refs. The full product UI runtime preflight now records those
      four remaining refs as manual-required acceptance rows and runbook entries;
      actual child-device delivery/runtime execution and full product
      parent/child UI remain pending.

## Next AI Instructions

Do not infer precise location from IP/network data. Treat mobile permission,
background execution, retention, and custody as first-class requirements.
Use `docs/plans/tracking-plan/README.md` for implementation sequencing and
workpack ownership. Keep AI as evidence, not authority, and keep LAN/IP/Wi-Fi
presence as hints only. The pre-device proof gate is now repeatable, and the
current WP32 read-model/accounting proof set is closed at the local-dev/hosted
route tier. Retention platform enforcement preflight now names the Android,
iOS, and desktop manual artifact rows required to close the missing platform
runtime enforcement artifact, but it is not product-ready runtime proof. The
product-readiness closure and real-runtime handoff now carry those preflight
counts. Full product UI runtime preflight also now carries four manual-required
product UI runtime rows for the missing production write-result, child check-in,
child consent, and safe/help response artifacts. Production worker runtime
preflight now carries eight manual-required worker/storage rows for the missing
production tracking artifacts while keeping production/product-ready claims
false. Claim-audit and real-runtime handoff now give
retention-writable-product-settings-required its own retention product settings
writable runtime row tied to the production retention write-result artifact,
instead of leaving that blocker implicit under the broader full-product UI row.
The local platform proof batch now gives Android emulator runtime evidence,
Samsung S9 Android physical package/service/status evidence, WSL/local replay,
hosted parent UI accessibility, product UI local artifacts,
product-readiness closure, and real-runtime handoff accounting one repeatable
local proof bundle without upgrading any physical behavior, child-device
runtime, authority, provider, production, or product-ready claim.
The next implementation layers are platform runtime artifacts required for
retention product readiness beyond the local applied-settings bridge, full
portal UI
snapshots/accessibility beyond the hosted route, Android system geofencing/dwell
proof, iOS Core Location/region proof beyond simulator package launch,
production worker runtime artifacts, then physical Android/iOS proof and
authority proof only when matching devices are enrolled.
