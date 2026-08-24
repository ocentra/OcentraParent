<!-- agent-capsule -->

> Agent Capsule
> Doc: App And Game Control
> Kind: feature documentation; read only when selected by FEATURE_ROUTE_INDEX, PLAN_INDEX, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# App And Game Control

## Parent Outcome

Parents can see which apps and games are active, understand time spent, set app
or category limits, approve exceptions, and enforce supported limits.

## Ocentra Requirement

App/game control starts with evidence: inventory, identity, running time,
foreground time, category candidates, and confidence. Blocking or time limits
require typed policy decisions and platform adapter proof.

## Roadmap And Expectations

- Roadmap: V0.5.2 app/game evidence, V0.8 enforcement, V5 policy product.
- Expectations: [app/game evidence](../expectations/app-game-evidence.md),
  [policy](../expectations/policy.md),
  [enforcement](../expectations/enforcement.md).
- Supporting docs: [app settings inventory](../plans/app-game-plan/workpacks/app-control-settings-inventory.md)
  and [game settings inventory](../plans/app-game-plan/workpacks/game-control-settings-inventory.md).
- Shared native app/game planning:
  [app + game plan](../plans/app-game-plan/README.md).
- Native app planning:
  [native apps plan](../plans/app-plan/README.md).
- Browser-game planning:
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-core`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
app inventory, app block/app limits, screen time schedules, and install
approval/purchases.

Google, Apple, Microsoft, Bark, Qustodio, Norton, Net Nanny, and Kidslox expose
app visibility, limits, and blocking. Ocentra must provide comparable parent
control with better evidence and local audit.

## Current Ocentra State

- App/game session contracts and read-model proof exist.
- Rust protocol now mirrors app/game evidence claim, AI digest reference,
  classification digest, identity, identity-merge, approval authority/action
  result, platform authority matrix, and classifier boundary shapes, preserving
  the evidence-only/no-adapter boundary before service or runtime consumers
  depend on them.
- Stored app/game observation rows now derive deterministic running,
  foreground, background, stale-gap, process-exit, replay-stable, and daily
  rollup duration proof without upgrading inventory, launcher, or foreground
  evidence into content or enforcement authority.
- Staged encrypted journal-file replay now persists typed inventory, runtime,
  foreground, and launcher evidence through SQLite and projects inventory,
  running-now, foreground-now, launcher, and daily rollup rows while preserving
  the no-use/no-content/no-launcher-promotion boundaries.
- The session-duration replay gate now ties those staged journal/SQLite rows
  back to the shared sessionization contracts: replayed runtime and foreground
  evidence produces daily rollup duration changes, replay order reconstructs the
  same summary, and duplicate replay cannot double-count duration or sessions.
- The service now exposes those staged app/game journal and SQLite projections
  through typed app-use and games activity-surface read models, including
  inventory, runtime, foreground, launcher/source-count, daily rollup,
  capability, and evidence-ref fields.
- The parent portal App/Game Sessions surface now renders those service-backed
  app-use and games read-model rows in a dedicated dashboard intent and SVG
  surface with separate inventory, running, foreground, launcher-only,
  unknown-review, manual-required capability, game-budget gap, and evidence
  counts.
- The App/Game Sessions portal visibility gate now proves stale,
  permission-required, manual-required, and not-claimed app/game states remain
  visible through dashboard rows, capability summaries, and policy-readiness
  route details instead of being hidden or upgraded into adapter/policy claims.
- The raw executable path UI leak gate now proves the App/Game Sessions
  dashboard can receive service rows carrying private Windows executable-path
  refs while the parent-visible intent and SVG dashboard omit the raw path and
  `executablePathRef` field, rendering only labels, state, counts, capability,
  durations, and evidence refs.
- The malicious metadata UI safety gate now proves a long script-like app/game
  display label remains a manual-required/risk text row in the parent dashboard
  intent, while the SVG dashboard source renders labels through bounded text
  sizing/truncation and avoids app/game dashboard HTML injection sinks.
- The app/game evidence-boundary UI safety gate now proves inventory-only rows
  do not become usage, running rows do not become foreground, foreground rows do
  not reveal content strings, launcher rows stay launcher-only without
  child-game proof, and unknown processes stay review candidates instead of
  becoming known games.
- App/game unknown approval contracts now represent new inventory apps, unknown
  runtime processes, portable/installer candidates, launcher-game candidates,
  unknown game-like executables, child status/reason refs, parent response
  scope, expiry, audit-backed replay state, and manual-required block outcomes
  without dispatching unsupported adapters.
- Native game budget policy contracts now represent game budget targets,
  running/foreground duration modes, launcher-only exclusion, parent-approved
  launcher-game candidate inclusion, advisory rating/UGC/multiplayer/purchase
  signals, dry-run preview decisions, and no-enforcement handoff guards.
- App/game policy target compiler contracts now require identity,
  unknown-state, category, schedule, capability, authority, device, local-user,
  and freshness proof before accepting app/game rule compile requests, and keep
  unproved block-launch in manual-required dry-run output.
- App/game time-budget contracts now consume stored app/game session refs,
  schedule evidence, bonus-time approval/audit refs, dry-run/manual-required
  handoff state, and restart-recovered timer refs before representing exceeded
  budget decisions.
- Child-facing app/game UX contracts now represent calm warning,
  approval-needed, time-limit, request submitted/approved/denied,
  manual-required, and unavailable states with text-domain copy tokens,
  evidence refs, child reason/status refs, and no private diagnostics.
- Package/process identity and owned-process time-limit proof now includes
  scoped real-service dry-run no-action, stale timer mismatch rejection before
  adapter execution, preserved-timer recovery/cancel, and owned/current expiry
  proof.
- The V0.8 product-control spine separates app time-limit and scoped
  owned-process control from broad installed-app blocking, so downstream policy
  and device surfaces can show time-limit/control-capable versus report-only or
  manual-required states.
- The V0.8 product-control runtime path now exposes app time-limit and scoped
  owned-process states through a Rust service WebSocket read model and typed
  agent-protocol adapter while broad app blocking stays manual-required.
- The V0.8 policy-dispatch proof now validates parent actor, target device,
  policy decision, schedule, app/game session evidence refs, adapter capability,
  timer state, approval state, audit refs, and child reason codes before
  dispatch-ready app/game time-limit states.
- The V0.8 broad-adapter proof now exposes a service-backed runtime read model
  that keeps owned-process/app timer support as implemented-boundary while
  broad installed-app blocking remains manual-required.
- The V0.8 supported-adapter runtime proof now narrows the implemented app/game
  claim to Windows owned-process time-limit support with evidence, timer,
  rollback, and audit references while keeping package-wide app blocking
  manual-required.
- The app/game adapter execution readiness live surface now carries that
  supported-adapter proof through TypeScript protocol parsing, Rust
  command/event parity, the agent-service WebSocket path, portal command
  contracts, portal live state, and a parent-safe panel intent. It keeps only
  the scoped Windows owned-process time-limit row execution-allowed and keeps
  broad installed-app blocking, platform enforcement, provider delivery,
  child-device delivery, raw private rows/targets, and private diagnostics
  unclaimed.
- The app/game adapter dispatch preflight live handoff now connects that
  readiness surface to the existing V0.8 policy-dispatch spine through
  TypeScript protocol parsing, Rust command/event parity, the agent-service
  WebSocket path, portal command contracts, portal live state, and a
  parent-safe panel intent. It marks only the scoped Windows owned-process
  app/game time-limit row as dispatch-eligible, keeps seven unsupported,
  unavailable, degraded, broad, or manual-required rows blocked before
  dispatch, and still does not claim adapter execution, broad blocking,
  platform enforcement, provider delivery, child-device delivery, raw private
  rows/targets, or private diagnostics.
- The app/game adapter execution readiness and dispatch preflight rows now
  carry explicit `hostCapabilityState`, host evidence refs, and parent-safe
  `hostCapabilityProbeRefs`. Windows scoped/manual/artifact rows report local
  host capability as `available` with a Windows host probe ref; Android reports
  ADB host visibility when `adb`, `ANDROID_HOME`, or `ANDROID_SDK_ROOT` exposes
  platform-tools, with separate path/SDK probe refs; Linux reports WSL/Docker
  host visibility when present, with separate WSL/Docker probe refs. macOS and
  iOS stay `not-applicable` on the Windows-local service host. Probe refs are
  opaque parent-safe refs only, not raw paths, device serials, distro names, or
  private diagnostics. Android and Linux host capability signals remain
  visibility-only and do not make rows dispatch eligible.
- The app/game scoped adapter dispatch command-result handoff now turns the
  single dispatch-eligible scoped Windows owned-process app/game time-limit row
  into a parent-visible result handoff to `agent.enforcement.execute` /
  `agent.enforcement.audit.reported`. The service and portal keep the other
  seven broad, degraded, unavailable, unsupported, or manual-required rows
  blocked before command handoff and still do not claim adapter execution,
  broad blocking, platform enforcement, provider delivery, child-device
  delivery, raw private rows/targets, or private diagnostics.
- The app/game adapter dispatch execution-audit seam now records one
  service-local execution audit for that scoped Windows owned-process
  app/game time-limit command-result row, while the other seven broad,
  degraded, unavailable, unsupported, or manual-required rows remain blocked
  before execution audit. This is parent-visible audit progress only; actual
  adapter execution, broad blocking, platform enforcement, provider delivery,
  child-device delivery, raw private rows/targets, and private diagnostics
  remain unclaimed.
- The Windows owned-process time-limit proof now extends that same real-service
  harness with dry-run and stale-action negative paths while preserving the
  manual-required boundary for broad package/app blocking.
- The app/game broad-blocking proof gates now add a focused parent-domain
  matrix for block-launch, allowlist/hide/suspend/shield, AppLocker audit-only,
  Android normal-mode hide/suspend, iOS shielding, and iOS process-kill
  no-claim states. It proves manual-required, unavailable, and not-claimed
  rows cannot dispatch adapters and names setup, authority-tier, rollback,
  audit, and platform-specific proof needed before any broad blocking claim can
  move up.
- The V0.8 enforcement integrity runtime audit now exposes supported app/game
  time-limit success, expiry, rollback, parent override/supersede, dry-run,
  stale decision rejection, wrong-device rejection, child-status refs, timer
  refs, rollback refs, audit refs, and permission/dependency unavailable states
  through TypeScript/Rust/service proof without upgrading broad installed-app
  blocking.
- Launcher evidence and launcher-game candidate contracts now exist with Rust
  protocol parity and staged Windows launcher parser proof. Launcher-only,
  launcher foreground, launcher-game candidate, and proved child-game states
  are separated so launcher evidence cannot become fake known-game proof.
- Cross-platform app/game authority matrix contracts now represent platform,
  action, authority tier, setup state, proof state, parent-visible limitation,
  proof needed to claim, and no-execute guards for Windows, macOS, Linux,
  Android, and iOS/iPadOS hard-control rows.
- Platform-extension routing contracts now map every MAC, IOS, ANDROID, and
  LINUX extension checklist row to authority tier, setup state, manual tags,
  proof-pack paths, and cross-plan handoff while keeping all current rows
  manual-required or not-claimed.
- Platform-extension proof-pack readiness rows now cover macOS, iOS, Android,
  and Linux with separate native-app/native-game product meanings, checklist
  refs, required proof refs, and explicit no live adapter/no adapter
  dispatch/no broad blocking/no privileged mobile/no provider execution/no
  child delivery claims.
- Install/store handoff contracts now route new app/game inventory,
  installer/updater processes, store package install signals, game purchase
  signals, uninstall deltas, and tamper/uninstall candidates to app-game
  evidence, app-install/purchase approval, or enforcement-integrity/tamper docs
  with evidence refs, parent-visible manual-required states, and no adapter or
  policy-decision claims.
- App/game performance-health contracts now record generated-scale budgets for
  inventory, runtime polling, foreground debounce, journal writes, session
  replay, policy compile, existing dashboard intent rows, and parent-visible
  degraded adapter health without claiming live OS, live adapter, or browser DOM
  throughput.
- App/game category-risk taxonomy contracts now represent native app
  categories, native game categories, risk candidates, game context signals,
  source kind, source ref, confidence, reason code, evidence refs, parent
  display override, AI digest refs, policy-candidate action, and a
  `notEnforcement` guard.
- Native app risk detection contracts now add app-only candidate proof for
  known VPN/proxy, remote desktop, torrent/download, AI chatbot, unknown
  name/publisher/hash, local AI digest, and parent display override rows with
  confidence/source disclosure, no-content claims, no-direct-enforcement
  guards, ask parent/manual-review routing, and risk-app category-proof policy
  routing.
- App/game AI classifier boundary contracts now add policy-facing proof that
  classifier output must cite stored evidence refs, stay within confidence
  bounds, name runtime/model/prompt/fallback refs, and remain evidence-only
  while rejecting direct action, duration, and raw scan fields before policy
  consumption.
- Rust protocol now mirrors that classifier boundary plus the app/game control
  approval authority/action-result and platform authority matrix shapes for
  serialization proof only.
- The app/game journal and SQLite projection now store and replay typed
  evidence claim, identity, approval authority, approval action-result,
  platform authority matrix, and AI classifier result protocol rows while
  rejecting inventory-use upgrades, inactive authority grants, manual-required
  action execution, manual platform adapter execution, and classifier direct
  action/raw-content claims.
- The service app-use and games read models now carry those staged
  evidence-claim, identity, approval authority/action-result, platform
  authority matrix, and AI classifier result row refs through the existing
  evidence vector, without creating policy, portal UI, live classifier, or
  adapter claims.
- The service app-use and games read models now also expose explicit staged
  boundary row counts for evidence claim, identity, approval
  authority/action-result, platform authority matrix/rows, and AI classifier
  result rows in the existing read-model payloads so later portal and policy
  consumers can detect those rows without parsing the evidence vector.
- The service now also exposes a dedicated app/game boundary read-model
  command/event for those staged evidence-claim, identity, approval
  authority/action-result, platform authority matrix/rows, and AI classifier
  result counts plus citation refs, without adding portal UI, policy
  consumption, provider execution, adapter execution, or platform support
  claims.
- The service now exposes a dedicated app/game policy readiness read-model
  command/event that derives policy evidence, approval authority, approval
  action history, platform authority, and AI classifier readiness rows from the
  existing app/game service model, keeps missing inputs visible as
  missing/manual-required, and fixes `adapterDispatchClaimed=false`.
- The policy readiness service read model now also consumes existing inventory
  category candidates and unknown/possible-game/launcher-game service rows,
  exposing `categoryCandidate` and `unknownReview` readiness rows plus
  `categoryRoutingReady`, `unknownReviewRequired`, `categoryCandidateRowCount`,
  and `unknownReviewRowCount` without dispatching adapters or upgrading unknown
  evidence into known app/game claims.
- The App/Game Sessions policy-readiness parent intent now renders those
  category candidate and unknown-review readiness rows plus category/unknown
  counts, keeping unknown app/game-like evidence in review state instead of
  hiding it behind generic policy readiness.
- The App/Game Sessions policy-readiness parent route now renders the
  service-backed evidence-claim, identity, approval authority, approval
  action-result, platform authority, and AI classifier row counts, plus
  per-readiness-row ready/manual-required/missing reasons, while preserving
  `adapterDispatchClaimed=false`.
- The main App/Game Sessions dashboard intent now also consumes those existing
  service-backed boundary row counts from app-use/game rows, exposing aggregate
  boundary and AI-classifier metrics plus parent-visible evidence-drawer
  boundary summaries without changing renderer ownership or claiming policy
  execution.
- Parent-domain policy preview handoff contracts now map already-compiled
  app/game dry-run policy decisions into read-only preview rows, separating
  native app versus native game target meaning while keeping evaluator runtime,
  timers, adapter dispatch, child delivery, and platform enforcement unclaimed.
- Parent-domain source-freshness preview gate contracts now require WP74
  source freshness readiness before WP70 policy preview rows are accepted:
  source-manual rows block preview before compiled decisions, and
  source-fresh native game rows can still remain compiler-manual-required.
- Parent-domain source-gated policy preview read-model contracts now project
  those gated rows into redacted future service/portal rows, preserving
  preview-ready, source-manual-required, and compiler-manual-required states
  without service runtime, portal UI, raw private source rows, adapter dispatch,
  child delivery, or platform enforcement claims.
- Parent-domain source-gated policy preview timer-handoff contracts now consume
  those redacted preview rows, mark preview-ready native app/native game rows as
  future timer sequencing candidates, and keep source-manual plus
  compiler-manual rows blocked before timer runtime without claiming service
  runtime, portal UI, policy evaluator execution, adapter dispatch, child
  delivery, platform enforcement, or raw private source-row access.
- Parent-domain source-gated policy preview timer scheduler-persistence
  contracts now consume the timer runtime-readiness rows, record which native
  app/native game rows still need service timer runtime, scheduler persistence,
  durable scheduler state-store, audit, and rollback proof, and keep source or
  compiler blockers blocked before scheduling without claiming runtime storage
  or enforcement.
- Parent-domain source-gated policy preview timer audit/rollback handoff
  contracts now consume those scheduler-persistence rows, record which native
  app/native game rows still need service timer runtime, scheduler persistence,
  durable scheduler state-store, audit trail, rollback plan, and
  audit/rollback read-model proof, and keep source or compiler blockers blocked
  before audit/rollback without claiming runtime storage, audit logs, rollback
  execution, or enforcement.
- Parent-domain source-gated policy preview timer audit/rollback read-model
  contracts now consume those audit/rollback handoff rows, project the same
  proof requirements into parent-visible read-model rows, and keep source or
  compiler blockers blocked without claiming service runtime, portal UI,
  durable audit logs, rollback execution, adapter dispatch, or enforcement.
- Parent-domain source-gated policy preview timer audit/rollback
  parent-surface intent contracts now consume those read-model rows, add
  parent-surface proof and drill-in refs for eligible native app/native game
  rows, and keep source or compiler blockers blocked without claiming service
  read APIs, portal UI, durable audit logs, rollback execution, adapter
  dispatch, or enforcement.
- Parent-domain source-gated policy preview timer service-readiness handoff
  contracts now consume those parent-surface intent rows, add future
  service-readiness and service read-API proof refs for eligible native
  app/native game rows, and keep source or compiler blockers blocked without
  claiming service runtime events, service read API implementation, portal UI,
  durable audit logs, rollback execution, adapter dispatch, or enforcement.
- Parent-domain source-gated policy preview timer service-readiness read-model
  contracts now consume those handoff rows, keep future service-readiness and
  service read-API proof refs visible for eligible native app/native game rows,
  and keep source or compiler blockers blocked without claiming agent protocol,
  service runtime events, service read API implementation, portal UI, durable
  audit logs, rollback execution, adapter dispatch, or enforcement.
- Parent-domain source-gated policy preview timer service-readiness protocol
  handoff contracts now consume those read-model rows, keep future
  agent-protocol command/event, Rust protocol mirror, service handler, and
  service read-API proof refs visible for eligible native app/native game rows,
  and keep source or compiler blockers blocked without claiming protocol
  implementation, service command registration, service event emission, portal
  UI, durable audit logs, rollback execution, adapter dispatch, or enforcement.
- Parent-domain source-gated policy preview timer service-readiness protocol
  read-model contracts now consume those protocol handoff rows, keep the same
  future protocol proof refs visible for eligible native app/native game rows,
  and keep source or compiler blockers blocked without claiming protocol
  implementation, service event/read-model emission, read API implementation,
  portal UI, durable audit logs, rollback execution, adapter dispatch, or
  enforcement.
- Parent-domain source-gated policy preview timer service-readiness read-API
  response consumer handoff contracts now consume response handoff rows, add
  future response-consumer proof refs for eligible native app/native game rows,
  and keep source or compiler blockers blocked without claiming service response
  implementation, response consumer implementation, portal response rendering,
  durable audit logs, rollback execution, adapter dispatch, child delivery, or
  enforcement.
- Parent-domain timer service read-API response consumer parent-surface handoff
  contracts now consume response-consumer handoff rows, add future
  parent-surface proof refs for eligible native app/native game rows, and keep
  source or compiler blockers blocked without claiming parent-surface
  rendering, portal rendering, service runtime, adapter dispatch, child
  delivery, platform enforcement, or raw private source rows.
- The portal App/Game Sessions route now renders that service-backed policy
  readiness read model as route cards with summary rows, readiness-kind rows,
  evidence refs, parser-failure visibility, and explicit no policy
  execution/no adapter dispatch product-claim copy.
- App/game notification intent contracts now represent parent notification
  readiness for time-limit, approval request, suspicious unknown,
  manual-required, and unavailable app/game states with evidence, policy, audit,
  child reason/status, minimal-payload, local-outbox-only, and
  no-provider/no-adapter claim guards.
- The service now exposes a dedicated app/game notification readiness
  read-model command/event that derives time-limit, approval-request,
  suspicious-unknown, manual-required, and unavailable notification readiness
  rows from the existing app/game service model while keeping provider
  delivery, receipt ingestion, local outbox runtime, scheduler runtime, adapter
  dispatch, parent UI, and child delivery claims false.
- The app/game notification local outbox bridge now maps validated
  local-outbox-eligible app/game notification intents into existing
  parent-owned local outbox JSONL records with minimal payload refs, while
  keeping manual-required and unavailable intents out of queued records and
  preserving no provider, scheduler, UI, child-delivery, adapter, broad
  blocking, or platform claims.
- The app/game notification scheduler bridge now maps linked local outbox rows
  into existing deterministic scheduler JSONL rows while leaving manual-required
  and unavailable rows unscheduled and preserving no runtime, provider, UI,
  child-delivery, adapter, broad-blocking, or platform claims.
- The app/game notification audit-history bridge now maps linked local outbox
  rows into existing logging-domain notification audit-history entries while
  keeping manual-required and unavailable rows blocked/manual and preserving no
  provider, retry runtime, UI, child-delivery, adapter, broad-blocking, or
  platform claims.
- The app/game notification provider preflight bridge now maps scheduled
  scheduler rows into provider-adapter-required rows with scheduler, outbox,
  decision, provider-channel, and reason refs preserved, while manual-required
  and unavailable rows stay blocked before any provider setup or delivery.
- The app/game notification preference preflight bridge now maps scheduled
  scheduler rows into parent-preference-required rows with provider-channel and
  reason refs preserved, while manual-required and unavailable rows stay blocked
  before parent preference, frequency-control, quiet-hours, UI, or delivery
  proof.
- The app/game notification provider-status handoff now maps those provider
  preflight rows into existing V0.8 notification provider-status boundary
  manual-required and unavailable rows, preserving scheduler/outbox/provider
  refs and keeping delivery, receipt, credential, UI, child delivery, runtime,
  adapter, broad-blocking, and platform claims false.
- The app/game notification preference-status handoff now maps preference
  preflight rows into V3 notification preference and quiet-hours status entries,
  preserving scheduler/outbox/provider/reason/preference/quiet-hours refs while
  keeping parent preference UI, notification UI, delivery, receipt, credential,
  child delivery, runtime, adapter, broad-blocking, and platform claims false.
- The app/game notification parent-surface intent proof now combines provider
  status and preference status rows into redacted parent-visible history and
  preference intent rows with drill-in/audit/manual-proof refs, while keeping
  rendered UI, provider delivery, receipts, credentials, child delivery,
  production runtime, adapter dispatch, broad-blocking, and platform claims
  false.
- Timer parent-surface child UX local artifact records now bridge into existing
  parent-owned local outbox JSONL records for deliverable native app/native game
  child UX states, while manual-required and unavailable states stay blocked
  before queued records and child delivery/provider/scheduler/UI/adapter/
  platform claims remain false.
- Timer parent-surface child UX local outbox rows now bridge into existing
  deterministic scheduler JSONL records for deliverable native app/native game
  child UX states, while manual-required and unavailable states remain
  unscheduled and child delivery/provider/retry/quiet-hours/UI/adapter/platform
  claims remain false.
- Timer parent-surface child UX scheduled rows now bridge into provider
  preflight rows that require provider adapter, credential, and smoke-proof
  setup before delivery can be claimed, while manual-required and unavailable
  rows remain blocked and provider delivery/receipt/UI/runtime/adapter/platform
  claims remain false.
- Timer parent-surface child UX provider-preflight rows now bridge into the
  existing V0.8 provider-status boundary as manual-required or unavailable rows,
  preserving scheduler/outbox/channel/readiness refs while provider delivery,
  receipt, credential, parent UI, child delivery, adapter, and platform claims
  remain false.
- Timer parent-surface child UX scheduled rows now also bridge into parent
  preference and quiet-hours preflight rows, requiring preference/frequency and
  quiet-hours setup before delivery can be claimed while manual-required and
  unavailable rows remain blocked and parent UI/provider/child/runtime/adapter/
  platform claims remain false.
- Timer parent-surface child UX preference-preflight rows now bridge into the
  existing V3 notification rule/provider/retry status boundary as manual setup
  or disabled status rows, preserving scheduler/outbox/preference/quiet-hours
  refs while parent preference mutation, parent UI, provider delivery, child
  delivery, retry, quiet-hours runtime, adapter, and platform claims remain
  false.
- Timer parent-surface child UX provider-status and preference-status rows now
  combine into redacted parent-surface intent rows with drill-in, audit,
  scheduler/outbox, provider, preference, quiet-hours, and manual-proof refs,
  while rendered parent UI, preference mutation, provider delivery, child
  delivery, runtime, adapter dispatch, and platform claims remain false.
- Timer parent-surface child UX parent-surface intent aggregates now flow through
  the live agent protocol, Rust service read model, and App/Game Sessions portal
  summary as manual-required, unavailable, history-visible, preference-setup,
  and reference-id visibility without rendering parent notification/preference
  UI or claiming delivery, mutation, adapter dispatch, platform enforcement, or
  raw private rows.
- Timer parent-surface child UX parent-surface intent rows now also feed a
  parent-domain preference setup draft read model that preserves parent-safe
  scheduler, outbox, drill-in, preference, quiet-hours, and manual-proof refs
  for future parent preference UI while keeping frequency controls, preference
  mutation, notification rule mutation, delivery, adapter dispatch, platform
  enforcement, and raw private rows unclaimed.
- The App/Game Sessions route now renders those service-backed child UX
  parent-surface records as separate read-only parent preference setup cards
  using the exported parent-domain draft status boundary, while parent
  preference UI controls, frequency controls, preference mutation, notification
  rule mutation, delivery, adapter dispatch, platform enforcement, and raw
  private rows remain unclaimed.
- The timer parent-surface service read model now emits dedicated child UX
  parent preference setup records through the agent-protocol-domain contract,
  Rust protocol mirror, and agent-service payload. The App/Game Sessions route
  renders those service-emitted setup records directly instead of locally
  deriving setup cards from parent-surface intent rows, while parent preference
  UI controls, frequency controls, preference mutation, notification rule
  mutation, delivery, adapter dispatch, platform enforcement, and raw private
  rows remain unclaimed.
- Request-ready timer parent-surface setup records now render a parent setup
  request action in the App/Game Sessions route. The action sends the
  schema-backed parent preference setup request command with parent-safe refs
  and selects the matching accepted event in the command-result panel, while
  preference mutation, notification rule writes, durable outbox storage,
  delivery, adapter dispatch, platform enforcement, and raw private rows remain
  unclaimed.
- The App/Game Sessions portal route now has a route-level app/game
  notification parent-surface panel that projects the live service
  notification-readiness read-model event into schema-backed manual/unavailable
  setup rows and otherwise shows a missing-service state, while keeping
  provider delivery, preference mutation, child delivery, scheduler/outbox
  runtime, adapter dispatch, broad-blocking, and platform claims false.
- The Rust core now has a live `sysinfo` process snapshot source that reads the
  current local process table into the existing app/game runtime record shape,
  uses opaque executable-path refs, and keeps runtime evidence from becoming
  foreground, content, policy, or adapter authority.
- The live process snapshot source can now emit app/game runtime journal events
  and replay them through the encrypted journal plus SQLite read model, proving
  the source-to-query-store path before service subscription or portal wiring.
- The service activity-capture path now appends bounded live process app/game
  runtime journal events into the encrypted journal and ActivityStore, so the
  existing app-use/games read-model path can query runtime-only rows from a real
  service capture without claiming foreground, policy, or adapter authority.
- The service startup activity-capture path now repeats that bounded capture on
  a protocol-owned cadence, and focused service proof shows two capture cycles
  append two queryable app/game runtime rows while keeping foreground unclaimed.
- The Rust core now has a live active-window foreground source that maps
  foreground window metadata into app/game foreground evidence records and
  journal events with opaque window/title refs, without raw title/path capture,
  service polling, portal UI, policy authority, or adapter execution.
- The Rust core now has a live Windows installed-app registry source that reads
  Windows Uninstall registry keys on Windows, exercises the same mapping through
  `.reg` export fixtures in tests, hashes registry/source/path details into
  opaque refs, filters hidden system components, and keeps those rows
  inventory-only.
- The service activity-capture path now appends bounded Windows Uninstall
  registry inventory events into the encrypted journal and ActivityStore, so the
  existing app-use/games read-model path can query registry-backed
  inventory-only rows without runtime, foreground, policy, or adapter claims.
- The app-use and games activity-surface read-model rows now expose typed
  `sourceStatusRows` that group inventory, runtime, foreground, and launcher
  evidence by source kind, row count, latest observed time, capability state,
  and evidence refs without adding portal UI, policy consumption, or adapter
  claims.
- The parent App/Game Sessions dashboard now consumes those service-backed
  `sourceStatusRows` and renders source row counts, fresh source counts,
  source-kind capability state, latest observed timestamps, and evidence ref
  counts through the existing metric/evidence surfaces without adding backend,
  policy, or adapter claims.
- Parent-domain source-freshness policy-consumption contracts now validate
  native app policy requests against fresh inventory, runtime, and foreground
  source rows and native game requests against fresh inventory, runtime,
  foreground, and launcher rows before policy compile. Stale, missing,
  permission-limited, unavailable, adapter-error, manual-required, and
  not-claimed rows remain manual-required, with no raw private source rows or
  adapter dispatch.
- The App/Game Sessions dashboard intent now also exposes grouped source-panel
  sections for app-use and game source rows with fresh/manual/evidence counts,
  source labels, and last-observed labels, preparing the dedicated source panel
  seam without touching the locked SVG surface.
- Parent-domain category/risk policy-routing contracts now map native app
  categories, risk candidates, native game categories, and game context signals
  into matching app/game policy target kinds with active category proof,
  confidence/source disclosure, supporting evidence refs, local-AI digest refs
  where applicable, manual-review/manual-required state, and
  `adapterDispatchState: not-dispatched`.
- The app/game final rollout/evidence gate now checks the app-game WP01-WP27
  and app-plan WP01-WP26 proof roots, writes final app-game WP28 and app-plan
  WP27/WP28 proof packs, and records the E2E/manual scenario routing,
  merge-blocking no-claim gates, product-doc decisions, and PR-ready reporting
  requirements without promoting any live platform or broad-blocking claim.
- Broad app blocking remains manual-required or unproved by platform.
- Raw app and game control setting inventories are preserved as design inputs,
  not product-complete implementation proof.
- The first merge-blocking display gate is now backed by
  `output/app-game-plan-proof/merge-gates/inventory-display`: the portal
  app/game dashboard intent test covers native-app, native-game, and launcher
  rows, the core dashboard maps inventory evidence into app/game usage metrics,
  and the App/Game Sessions route assertion requires inventory text without
  duplicating browser-game work or rendering raw private executable paths.
- The running/foreground merge-blocking display gate is now backed by
  `output/app-game-plan-proof/merge-gates/running-foreground-display`: the
  portal app/game dashboard keeps Running and Foreground as separate metrics and
  per-row counts, including running rows that are explicitly not foreground,
  without promoting runtime evidence to foreground usage.
- The AI-output direct-enforcement merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/ai-output-direct-enforcement`: AI
  classification digests expose only classification, confidence, action hints,
  and evidence/session refs; local-AI category candidates require digest refs
  and remain `notEnforcement`; and parent-domain category/risk policy routes
  keep adapter dispatch `not-dispatched`.
- The Android normal-mode suspend/hide merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/android-normal-mode-no-suspend-hide`:
  Android package suspend remains manual-required and blocked before adapter
  dispatch unless Device Owner or Profile Owner proof exists.
- The unknown-process auto-promotion merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/unknown-process-auto-promotion`:
  display-label-only identities remain weak, unknown, and unknown-executable;
  heuristic category/risk labels stay manual-review candidates; and the
  App/Game Sessions dashboard renders unknown-process and possible-game rows as
  manual-required review state instead of known native-game usage.
- The iOS process scan/kill merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/ios-no-process-scan-kill`: iOS
  process enumeration and terminate-process behavior remain not-claimed,
  not-dispatched, and blocked from adapter calls unless real FamilyControls,
  ManagedSettings, or supervised MDM proof exists.
- The dry-run no-action merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/dry-run-no-action`: time-budget
  runtime decisions stay dry-run-only, preview handoff rows require disabled
  enforcement handoff and `not-dispatched` adapter state, and unproved
  block-launch decisions remain manual-required instead of executable blocks.
- The manual-required no-adapter merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/manual-required-no-adapter`: broad
  blocking gates block manual-required rows before adapter dispatch, policy
  preview rows keep manual-required block-launch `not-dispatched`, and
  category/risk manual-review routes remain out of compile-ready execution.
- The launcher/child-game merge-blocking display gate is now backed by
  `output/app-game-plan-proof/merge-gates/launcher-child-game-boundary`: launcher
  contracts require child-game proof before known-game classification, and the
  App/Game Sessions dashboard keeps launcher rows as launcher-only counts rather
  than active child-game usage.
- The foreground/content merge-blocking display gate is now backed by
  `output/app-game-plan-proof/merge-gates/foreground-content-boundary`: the
  foreground evidence contract restricts content knowledge to `notClaimed`, and
  the App/Game Sessions dashboard renders foreground state as source/read-model
  refs and counts without exposing window title refs, raw titles, or executable
  paths.
- The Linux universal-block merge-blocking gate is now backed by
  `output/app-game-plan-proof/merge-gates/linux-universal-block-proof`: Linux
  broad block-launch behavior remains unavailable/manual-required without a
  named mechanism, target distro, session, rollback, and audit proof.

## Current Gap

Parent-visible app catalog/dashboard UI now has an initial service-backed
App/Game Sessions surface, but category quality, unknown approvals, game budget
policy, live launcher crawling, install/approval linkage, child request flow,
broad blocking, and cross-platform parity are incomplete. Broad installed-app
blocking remains manual-required beyond the scoped
owned-process/app-session proof, including in the broad-adapter,
supported-adapter runtime service proofs, and the focused broad-blocking gate
matrix. The integrity runtime audit proves typed timer/rollback/child-status
visibility for scoped app/game outcomes, but does not prove package-wide app
blocking, polished child request UX, install approval, or runtime
cross-platform parity. The authority and broad-blocking gate matrices are
contract proof only; they do not prove AppLocker/App Control, MDM, Endpoint
Security, Device Owner/Profile Owner, FamilyControls/ManagedSettings,
cgroup/systemd, or kiosk/single-app adapter behavior. The platform-extension
routing matrix adds proof-pack coverage for those rows, but it is also
contract-only and does not prove runtime platform support. The category/risk
taxonomy is contract proof only; it does not prove live catalog enrichment,
local AI classifier quality, policy compiler routing, portal category UI, or
runtime app/game category decisions. Native app risk detection and app/game AI
classifier boundary proof now have Rust protocol parity, staged journal/SQLite
storage projection for classifier rows, service app-use/games evidence
refs/counts, and a dedicated service-backed boundary read-model event for
staged row counts and citation refs. Core live Windows shortcut inventory source
proof now adds bounded Start Menu shortcut scans with hashed source refs, and the
service activity-capture path can append those inventory-only journal events into
the existing encrypted journal/store/read-model path. Core packaged-app manifest
proof now maps bounded `AppxManifest.xml` evidence into store-package
inventory-only rows and journal events with hashed source refs, and the service
capture path can append those packaged-app inventory events into the existing
encrypted journal/store/read-model path. Core registry crawling now maps Windows
Uninstall registry evidence into inventory-only rows and journal events with
hashed source/path refs, and the service capture path can append those
registry-backed inventory rows into the same journal/store/read-model path.
Backend app-use/games read-model rows now include grouped source
freshness/status rows for inventory, runtime, foreground, and launcher sources,
and the portal intent now exposes app-use and game source-panel sections for
those rows. Parent-domain category/risk routing now turns category, risk, and
game-context candidates into soft/manual policy target inputs only when active
category proof and supporting evidence refs exist. A service-backed policy
readiness read model can now report whether the required evidence, approval
authority, and platform authority rows are available before evaluator
consumption, while action history/classifier gaps remain manual-required. Richer
source-panel SVG rendering, category/readiness rendering, local model
quality/provider execution, policy evaluator consumption, runtime service
policy evaluation, and platform enforcement remain unproved. Live
process snapshots now replay through the local journal/SQLite path in core and
through the service activity-capture journal/store path for bounded runtime
rows; recurring service capture freshness is now proved, and the service
capture bridge can also append optional app/game foreground rows from the core
active-window source with opaque window/title refs. Richer process start/exit
and foreground transition subscriptions, richer source subscriptions,
policy/runtime consumers, dedicated source panel polish, and adapter execution
remain separate gaps.
The install/store handoff proof is contract-only: it does not prove live store
integration, Google Play, Apple App Store, Microsoft Store, package-manager
interception, billing entitlement logic, portal approval UI, platform adapter
execution, uninstall blocking, or anti-tamper behavior.
The performance-health proof is contract/generated-scale only: it does not
prove live OS inventory/process/foreground throughput, encrypted journal disk
throughput or corruption recovery, browser DOM/Playwright rendering, live
platform adapters, approval/store behavior, or broad blocking.
The final rollout/evidence gate is proof-review infrastructure, not runtime
capability: it proves the branch can be reviewed with explicit proof paths and
no-claim boundaries, but it does not add live source crawling, finished
approval/notification UI, cross-platform platform adapters, live classifier
quality, browser DOM proof for every UI state, or product reports.
The unknown approval proof is still contract-level: it does not yet provide
finished parent/child approval UI, notification delivery, persisted service
read models, live candidate creation from platform adapters, or platform hard
blocking.
The native game budget proof is also contract-level: it does not yet provide a
policy target compiler, live game budget authoring UI, service persistence,
budget notifications, or adapter execution.
The app/game policy target compiler proof is contract-level: it does not yet
provide runtime service evaluation, Rust/WebSocket parity, portal rule
authoring, timer integration, notifications, rollback, or adapter execution.
The app/game policy preview handoff and source-freshness preview gate proofs
are also contract-level: they consume source readiness plus compiled dry-run
decisions and produce parent-domain preview rows or manual-required blockers
only; they do not run the policy evaluator, persist policy previews in the
service, render authoring UI, start timers, deliver child notifications,
dispatch adapters, or prove platform enforcement.
The app/game time-budget proof now includes parent-domain runtime decision
construction for dry-run, warn-only, ask-parent, manual-required, and
approved-bonus outcomes, but it does not yet provide service persistence,
Rust/WebSocket parity, portal budget authoring, notification delivery, child
request UX, adapter execution, or platform timer/rollback execution.
The child-facing UX proof is contract/text-level: it does not yet provide a
native child app, overlay, portal preview, notification delivery, service
persistence, Rust/WebSocket parity, or platform adapter execution.
The app/game notification readiness service proof adds a service-backed
command/event for local intent readiness rows, but it does not provide provider
delivery, provider receipt ingestion, production local outbox or scheduler
runtime, parent notification UI, child app/overlay delivery, policy evaluator
execution, adapter dispatch, broad blocking, or platform support.
The app/game notification local outbox bridge proof links eligible app/game
notification intents to the existing parent-owned local outbox JSONL record
schema, but it does not provide durable production outbox storage, provider
delivery, provider receipt ingestion, quiet-hours/retry scheduler execution,
parent notification UI, child delivery, policy evaluator execution, adapter
dispatch, broad blocking, or platform support.
The app/game notification scheduler bridge proof links eligible app/game local
outbox rows to the existing notification scheduler JSONL record schema, but it
does not provide production retry workers, quiet-hours timer runtime, durable
production outbox storage, provider delivery, provider receipt ingestion, parent
notification UI, child delivery, policy evaluator execution, adapter dispatch,
broad blocking, or platform support.
The app/game notification audit-history bridge proof links eligible app/game
local outbox rows to the existing logging-domain notification audit-history
schema, but it does not provide provider delivery, provider receipt ingestion,
production retry workers, quiet-hours timer runtime, durable outbox/history
storage, parent notification UI, child delivery, policy evaluator execution,
adapter dispatch, broad blocking, or platform support.
The app/game notification provider preflight proof turns scheduled scheduler
rows into explicit provider-adapter, credential, and provider-smoke-proof
requirements, but it does not provide provider delivery, provider receipt
ingestion, credentials, production retry workers, quiet-hours timer runtime,
durable production outbox storage, parent notification UI, child delivery,
policy evaluator execution, adapter dispatch, broad blocking, or platform
support.
The app/game notification preference preflight proof turns scheduled scheduler
rows into explicit parent preference, frequency-control, and quiet-hours proof
requirements, but it does not provide parent preference UI, frequency controls,
provider delivery, provider receipt ingestion, credentials, production retry
workers, quiet-hours timer runtime, durable production outbox storage, child
delivery, policy evaluator execution, adapter dispatch, broad blocking, or
platform support.
The app/game notification provider-status handoff proof maps those preflight
rows into V0.8 provider-status manual-required/unavailable boundary rows, but it
does not provide provider delivery, provider receipt ingestion, credentials,
production retry workers, quiet-hours timer runtime, durable production outbox
storage, parent notification UI/history/preferences, child delivery, policy
evaluator execution, adapter dispatch, broad blocking, or platform support.
The app/game notification preference-status handoff proof maps preference
preflight rows into V3 notification preference/quiet-hours status entries, but
it does not provide parent preference UI, frequency controls, parent
notification UI, provider delivery, provider receipt ingestion, credentials,
production retry workers, quiet-hours timer runtime, durable production outbox
storage, child delivery, policy evaluator execution, adapter dispatch, broad
blocking, or platform support.
The app/game notification parent-surface intent proof combines provider and
preference status rows into future parent history/preference intent rows, and
the App/Game Sessions route can now project the live service readiness event
into redacted parent setup rows. It still does not mutate parent preferences,
send or receive provider notifications, run scheduler/outbox or production
retry/quiet-hours workers, deliver to child devices, execute adapters, prove
mobile UI, or prove platform support.
The app/game timer parent-surface service read model now has a dedicated
agent-protocol-domain contract, Rust protocol mirror, and service websocket
command/event backed by the existing app-game ActivityStore service read model.
It reports native-app/native-game target rows plus source/compiler/manual
blockers for parent-surface consumption, while explicitly not claiming timer
runtime, scheduler persistence, durable storage, audit/rollback runtime, adapter
dispatch, child delivery, platform enforcement, or raw private source rows.
The parent portal App/Game Sessions route now consumes that live service event,
shows a refresh command, and renders timer parent-surface rows for native app
and native game targets while keeping timer runtime, scheduler persistence,
audit/rollback runtime, adapter dispatch, child delivery, platform enforcement,
and raw private source rows unclaimed.
The timer parent-surface service now also reads the existing enforcement active
timer state file before reporting the read model. When a parsed active timer
state exists, the read model can report timer runtime, scheduler persistence,
and durable scheduler state-store visibility for the parent surface while still
leaving live scheduling execution, audit/rollback runtime, adapter dispatch,
child delivery, broad blocking, platform enforcement, and raw private source
rows unclaimed.
The parent portal timer parent-surface intent now renders those active
state-store flags directly, including separate timer runtime, scheduler
persistence, durable scheduler storage, audit runtime, and rollback runtime
rows, so parent-visible copy no longer says the active state store is unclaimed
when the service reports one.
The timer parent-surface service now also maps existing active enforcement timer
audit sequence and rollback token references into the shared app/game read
model. Parent-visible audit and rollback rows can therefore show active-state
visibility when those references exist, while durable audit log read-models,
rollback execution, adapter dispatch, child delivery, broad blocking, platform
enforcement, and raw private source rows remain unclaimed.
The timer parent-surface service now also maps existing app/game SQLite replay
control action-result rows into parent-visible count and result-reference
fields. The parent portal renders those action-result refs as replay
visibility, while live scheduling automation, adapter dispatch, child delivery,
broad blocking, platform enforcement, and raw private source rows remain
unclaimed.
The same timer parent-surface read model now aggregates parent-safe
action-result statuses, capability states, and enforcement-result statuses from
those replay rows. Parents can see whether replayed control rows are enforced,
manual-required, supported, or unavailable without receiving raw action request
payloads, adapter execution access, child delivery, platform enforcement, or
private source rows.
The timer parent-surface path now also carries child-facing reason and status
reference ids from existing app/game approval action-result rows into the
service read model and parent portal. This gives the runtime audit and parent
surface a child-safe reference chain for future warning/request UX while still
not delivering child notifications, running adapters, exposing diagnostics, or
claiming platform enforcement.
The timer parent-surface read model now also reports child UX handoff readiness
counts and result-reference ids for replayed action-result rows that have both
child reason and child status refs. Parent-domain handoff validation turns
child UX cards into ready or blocked local handoff rows without claiming child
delivery, notification delivery, adapter dispatch, platform enforcement, private
diagnostics, or raw private source rows.

## Checklist

- [ ] App/game inventory and identity. Rust protocol parity now mirrors the
      shared evidence claim, AI digest reference/classification digest,
      identity, and identity-merge shapes. Staged journal/SQLite projection now
      preserves evidence claim and identity rows, but runtime identity merge
      behavior, live adapter-fed identity refs, and product-complete identity UI
      remain.
- [ ] Running and foreground session evidence. Contract and local
      SQLite-row session-duration proof now exists, and staged journal-file
      replay proof now covers typed fixture rows; service app-use/games
      read-model DTOs now expose those projected rows; the parent portal now
      has a dedicated app/game dashboard surface for those rows; a real
      `sysinfo` process snapshot source now emits runtime-only process records
      with opaque path refs and replays them through encrypted journal/SQLite
      in core and through the recurring service activity-capture journal/store
      path; a core live foreground-window source now emits foreground evidence
      rows with opaque window/title refs through the journal/SQLite path.
      The service capture bridge can append those foreground rows when the
      active-window source is available, and it can append bounded live Windows
      shortcut inventory rows with hashed source refs. Core packaged-app
      manifest proof now maps `AppxManifest.xml` evidence into store-package
      inventory-only rows, and service capture can append those packaged-app
      rows into the journal/store/read-model path. Core Windows registry source
      proof now maps Uninstall registry evidence into inventory-only rows with
      hashed source/path refs, and service capture can append those registry
      rows into the journal/store/read-model path. App-use/games read-model
      rows now expose grouped backend source status/freshness rows, and the
      parent App/Game Sessions dashboard renders source and fresh-source counts
      plus source-kind evidence summaries. Source-panel intent sections now
      group those rows for the next rendering seam. Dedicated source-panel SVG
      rendering, richer source subscriptions, and policy integration remain.
- [ ] Category and unknown-state handling. Unknown approval contracts now keep
      weak app/game evidence in review/report-only/manual-required states with
      evidence refs, child status refs, expiry, and audit-backed persistence
      fields. Category/risk routing contracts now keep category, risk, and
      game-context candidates as evidence-backed soft/manual policy inputs.
      Runtime service consumption now exposes existing inventory category
      candidates and unknown/possible-game/launcher-game service rows through
      `categoryCandidate` and `unknownReview` policy-readiness rows plus
      category/unknown counts, and the parent policy-readiness intent now
      renders those rows/counts as review-ready details. Live candidate
      production, finished parent/child approval UX, live classifier quality,
      runtime policy evaluator persistence, notification delivery,
      cross-platform adapter proof, and product reports remain.
- [ ] App/category schedule and time-budget rules. Native game budget dry-run
      contracts now exist for known-game counts, launcher-only exclusion,
      parent-approved candidate inclusion, and advisory signal boundaries.
      App/game policy target compiler contracts now validate identity,
      unknown-state, category, schedule, capability, authority, device,
      local-user, and freshness proof before dry-run decisions, and category/risk
      policy-routing contracts now map category/risk candidates into matching
      compiler target kinds without adapter dispatch. The service now has a
      policy readiness read-model command/event for required app/game policy
      input row availability, and a parent-domain policy preview handoff now
      maps compiled dry-run decisions into read-only native app/game preview
      rows, while the App/Game Sessions portal route renders those readiness
      rows with source row counts and explicit ready/manual-required/missing
      reasons and the main dashboard exposes aggregate boundary/AI counts but
      no policy execution or adapter dispatch claim. Live evaluator, authoring
      UI, persistence, timers, and enforcement remain. Rust protocol
      parity now exists for approval authority/action-result, platform
      authority matrix, and classifier boundary shapes, and staged
      journal/SQLite projection plus service read-model evidence refs/counts
      can carry those rows before live classifier/provider, dedicated portal,
      policy, or adapter consumers are added. App/game
      time-budget contracts now consume stored session refs, schedule refs,
      bonus approval/audit refs, and timer recovery refs before dry-run or
      manual-required decisions; live evaluator, UI, notification, persistence,
      and adapter execution remain.
- [ ] Ask-parent and bonus-time flow. Contract proof now requires approval and
      audit refs before bonus time extends a budget and keeps ask parent/manual
      states dry-run only. App/game notification intent contracts now tie
      approval/time-limit/manual/unavailable alert readiness to evidence,
      policy, audit, child reason/status, and minimal payload refs, and the
      service can now report notification readiness rows for those states, but
      provider delivery, production outbox/scheduler runtime, and finished
      parent/child UX remain. The local outbox bridge can now write and reread
      parent-owned JSONL records for eligible app/game notification intents,
      while manual-required and unavailable intents stay out of queued records.
      The scheduler bridge can now write and reread scheduler JSONL rows for
      linked local outbox records while manual-required and unavailable rows
      remain unscheduled. The audit-history bridge can now write metadata-only
      audit-history handoff entries for linked, manual-required, and unavailable
      rows without claiming provider delivery, history UI, or child delivery.
      The provider preflight bridge now turns scheduled rows into
      provider-adapter-required rows with adapter, credential, and provider
      smoke-proof requirements before any delivery claim.
      The preference preflight bridge now turns scheduled rows into
      parent-preference-required rows with preference, frequency, and
      quiet-hours proof requirements before any delivery claim.
      The provider-status
      handoff maps those preflight rows into V0.8 provider-status
      manual-required/unavailable boundary rows while provider delivery,
      receipts, credentials, UI, child delivery, runtime, adapter dispatch,
      broad blocking, and platform claims remain false.
      The preference-status handoff maps preference preflight rows into V3
      notification preference/quiet-hours manual-required or disabled status
      entries while parent preference UI, parent notification UI, delivery,
      receipts, credentials, child delivery, runtime, adapter dispatch, broad
      blocking, and platform claims remain false.
      The parent-surface intent maps provider-status and preference-status rows
      into redacted future history/preference cards with drill-in, audit, and
      manual-proof refs while rendered UI, provider delivery, receipts,
      credentials, child delivery, runtime, adapter dispatch, broad blocking,
      and platform claims remain false.
      The App/Game Sessions route panel now projects the live service
      notification-readiness event into schema-backed parent setup rows and
      shows missing-service state otherwise, while provider delivery, parent
      preference mutation, child delivery, scheduler/outbox runtime, adapter
      dispatch, broad blocking, mobile UI, and platform claims remain false.
      Source-gated policy preview timer-status proof now classifies WP78
      timer-handoff rows by the proof still required before scheduling:
      timer-runtime proof, source-freshness proof, or compiler-decision proof.
      It remains parent-domain proof only and does not claim service events,
      portal UI, evaluator runtime, timer scheduling, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer runtime-readiness proof now consumes
      those timer-status rows and records that future scheduling still requires
      service timer runtime, scheduler persistence, audit, and rollback proof,
      while source and compiler blockers remain blocked before timer runtime.
      It remains parent-domain proof only and does not claim service events,
      portal UI, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, audit/rollback runtime, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer scheduler-persistence proof now consumes
      those runtime-readiness rows and records that future scheduling still
      requires service timer runtime, scheduler persistence, durable scheduler
      state-store, audit, and rollback proof, while source and compiler blockers
      remain blocked before scheduler persistence. It remains parent-domain
      proof only and does not claim service events, portal UI, evaluator
      runtime, timer runtime/scheduling, scheduler persistence runtime, durable
      scheduler storage, audit/rollback runtime, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer audit/rollback handoff proof now
      consumes those scheduler-persistence rows and records that future
      scheduling still requires service timer runtime, scheduler persistence,
      durable scheduler state-store, audit trail, rollback plan, and
      audit/rollback read-model proof, while source and compiler blockers remain
      blocked before audit/rollback. It remains parent-domain proof only and
      does not claim service events, portal UI, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model handoff proof now consumes those
      parent-surface status handoff rows and records that future parent-visible
      status read-model consumption still requires parent-surface status
      read-model proof, while source and compiler blockers remain blocked
      before status read-model visibility. It remains parent-domain proof only
      and does not claim service command registration, service handler
      implementation, service event emission, service read API implementation,
      response implementation, response consumer implementation, parent-surface
      read-model/status/status-read-model implementation, parent-surface
      rendering, portal UI/consumer rendering, agent-protocol implementation,
      Rust protocol mirror, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit runtime,
      durable audit logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model parent-surface handoff proof now consumes
      those status read-model rows and records that future parent-visible
      surface rendering still requires parent-surface proof, while source and
      compiler blockers remain blocked before rendering. It remains
      parent-domain proof only and does not claim service runtime events, read
      APIs, response consumer implementation, parent-surface rendering, portal
      UI/consumer rendering, agent-protocol implementation, Rust protocol
      mirror, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime, durable
      audit logs, rollback runtime/execution, adapter dispatch, child delivery,
      broad blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model parent-surface read-model handoff proof
      now consumes those parent-surface handoff rows and records that future
      parent-visible surface read-model implementation still requires
      parent-surface read-model proof, while source and compiler blockers
      remain blocked before rendering. It remains parent-domain proof only and
      does not claim service runtime events, read APIs, response consumer
      implementation, parent-surface read-model implementation,
      parent-surface rendering, portal UI/consumer rendering, agent-protocol
      implementation, Rust protocol mirror, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model parent-surface read-model contract now
      consumes those parent-surface read-model handoff rows and records
      parent-safe native app/native game read-model rows for future
      parent-visible consumption. It remains parent-domain contract proof only
      and does not claim package exports, service runtime events, read APIs,
      response consumer implementation, runtime read-model persistence,
      parent-surface rendering, portal UI/consumer rendering, agent-protocol
      implementation, Rust protocol mirror, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model parent-surface read-model service
      handoff proof now consumes those parent-safe read-model rows and records
      that future service wiring still requires service read-model/event/API
      proof, while source and compiler blockers remain blocked before runtime
      service visibility. It remains parent-domain proof only and does not
      claim package exports, service command registration, service handler
      implementation, service read-model emission, service events, read APIs,
      response consumer implementation, runtime persistence, parent-surface
      rendering, portal UI/consumer rendering, agent-protocol implementation,
      Rust protocol mirror, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit runtime,
      durable audit logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status read-model parent-surface read-model service
      read-model handoff proof now consumes those service handoff rows and
      records that future service visibility still requires service read-model
      proof before runtime service emission or API visibility, while source and
      compiler blockers remain blocked before service read-model runtime
      emission. It remains parent-domain proof only and does not claim package
      exports, service command registration, service handler implementation,
      service read-model runtime emission, service events, read APIs, response
      consumer implementation, runtime persistence, parent-surface rendering,
      portal UI/consumer rendering, agent-protocol implementation, Rust protocol
      mirror, evaluator runtime, timer runtime/scheduling, scheduler persistence
      runtime, durable scheduler storage, audit runtime, durable audit logs,
      rollback runtime/execution, adapter dispatch, child delivery, broad
      blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service event handoff proof now consumes
      the service read-model handoff rows and records that future service
      visibility still requires service event proof before runtime service event
      emission or API visibility, while source and compiler blockers remain
      blocked. It remains parent-domain proof only and does not claim package
      exports, service command registration, service handler implementation,
      service read-model runtime emission, service event runtime emission,
      service events, read APIs, response consumer implementation, runtime
      persistence, parent-surface rendering, portal UI/consumer rendering,
      agent-protocol implementation, Rust protocol mirror, evaluator runtime,
      timer runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime/logs, rollback runtime/execution, adapter dispatch,
      child delivery, broad blocking, platform enforcement, or raw private
      source rows.
      Source-gated policy preview timer service read API handoff proof now
      consumes the service event handoff rows and records that future service
      visibility still requires service read API proof before read API
      implementation or response visibility, while source and compiler blockers
      remain blocked. It remains parent-domain proof only and does not claim
      package exports, service command registration, service handler
      implementation, service read-model runtime emission, service event
      runtime emission, service events, read APIs, read API responses, response
      consumer implementation, runtime persistence, parent-surface rendering,
      portal UI/consumer rendering, agent-protocol implementation, Rust protocol
      mirror, evaluator runtime, timer runtime/scheduling, scheduler persistence
      runtime, durable scheduler storage, audit runtime/logs, rollback
      runtime/execution, adapter dispatch, child delivery, broad blocking,
      platform enforcement, or raw private source rows.
      Source-gated policy preview timer service read API response handoff proof
      now consumes the service read API handoff rows and records that future
      service visibility still requires service read API response proof before
      response implementation or response-consumer visibility, while source and
      compiler blockers remain blocked. It remains parent-domain proof only and
      does not claim package exports, service command registration, service
      handler implementation, service read-model runtime emission, service
      event runtime emission, service events, read APIs, read API responses,
      response consumer implementation, runtime persistence, parent-surface
      rendering, portal UI/consumer rendering, agent-protocol implementation,
      Rust protocol mirror, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit
      runtime/logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer service read API response consumer
      handoff proof now consumes the service read API response handoff rows and
      records that future parent-surface and portal consumption still require
      response-consumer proof, while source and compiler blockers remain
      blocked. It remains parent-domain proof only and does not claim package
      exports, service command registration, service handler implementation,
      service read-model runtime emission, service event runtime emission,
      service events, read APIs, read API responses, response consumer
      implementation, runtime persistence, parent-surface rendering, portal
      UI/consumer rendering, agent-protocol implementation, Rust protocol
      mirror, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime/logs,
      rollback runtime/execution, adapter dispatch, child delivery, broad
      blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service read API response consumer
      parent-surface handoff proof now consumes the service read API response
      consumer handoff rows and records that future parent-visible surface
      consumption still requires parent-surface proof, while source and compiler
      blockers remain blocked. It remains parent-domain proof only and does not
      claim package exports, service command registration, service handler
      implementation, service read-model runtime emission, service event
      runtime emission, service events, read APIs, read API responses, response
      consumer implementation, runtime persistence, parent-surface rendering,
      portal UI/consumer rendering, agent-protocol implementation, Rust protocol
      mirror, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime/logs,
      rollback runtime/execution, adapter dispatch, child delivery, broad
      blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer audit/rollback read-model proof now
      consumes those audit/rollback handoff rows and records that future
      parent-visible scheduling status still requires service timer runtime,
      scheduler persistence, durable scheduler state-store, audit trail,
      rollback plan, and audit/rollback read-model proof, while source and
      compiler blockers remain blocked before visibility. It remains
      parent-domain proof only and does not claim service events, portal UI,
      evaluator runtime, timer runtime/scheduling, scheduler persistence
      runtime, durable scheduler storage, audit runtime, durable audit logs,
      rollback runtime/execution, adapter dispatch, child delivery, broad
      blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer audit/rollback parent-surface intent
      proof now consumes those read-model rows and records that future
      parent-surface visibility still requires service timer runtime,
      scheduler persistence, durable scheduler state-store, audit trail,
      rollback plan, audit/rollback read-model proof, and parent-surface proof,
      while source and compiler blockers remain blocked before visibility. It
      remains parent-domain proof only and does not claim service events/read
      APIs, portal UI, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime, durable
      audit logs, rollback runtime/execution, adapter dispatch, child delivery,
      broad blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness handoff proof now
      consumes those parent-surface intent rows and records that future service
      visibility still requires service timer runtime, scheduler persistence,
      durable scheduler state-store, audit trail, rollback plan,
      audit/rollback read-model proof, parent-surface proof, service-readiness
      proof, and service read-API proof, while source and compiler blockers
      remain blocked before service/read API visibility. It remains
      parent-domain proof only and does not claim service events/read API
      implementation, portal UI, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit runtime,
      durable audit logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer service-readiness read-API response
      handoff proof now consumes those read-API handoff rows and records that
      future response visibility still requires service read-API response proof,
      while source and compiler blockers remain blocked before response
      visibility. It remains parent-domain proof only and does not claim service
      command registration, service handler implementation, service event
      emission, service read API implementation or response implementation,
      agent-protocol implementation, Rust protocol mirror, portal UI, evaluator
      runtime, timer runtime/scheduling, scheduler persistence runtime, durable
      scheduler storage, audit runtime, durable audit logs, rollback
      runtime/execution, adapter dispatch, child delivery, broad blocking,
      platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness read-API response
      consumer handoff proof now consumes those response handoff rows and
      records that future parent/service consumption still requires response
      consumer proof, while source and compiler blockers remain blocked before
      response consumption. It remains parent-domain proof only and does not
      claim service command registration, service handler implementation,
      service event emission, service read API implementation, response
      implementation, response consumer implementation, portal UI/consumer
      rendering, agent-protocol implementation, Rust protocol mirror, evaluator
      runtime, timer runtime/scheduling, scheduler persistence runtime, durable
      scheduler storage, audit runtime, durable audit logs, rollback
      runtime/execution, adapter dispatch, child delivery, broad blocking,
      platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface handoff proof now consumes those response-consumer rows and
      records that future parent-visible consumption still requires
      parent-surface proof, while source and compiler blockers remain blocked
      before parent-surface visibility. It remains parent-domain proof only and
      does not claim service command registration, service handler
      implementation, service event emission, service read API implementation,
      response implementation, response consumer implementation,
      parent-surface rendering, portal UI/consumer rendering, agent-protocol
      implementation, Rust protocol mirror, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface read-model handoff proof now consumes those parent-surface
      handoff rows and records that future parent-visible read-model
      consumption still requires parent-surface read-model proof, while source
      and compiler blockers remain blocked before read-model visibility. It
      remains parent-domain proof only and does not claim service command
      registration, service handler implementation, service event emission,
      service read API implementation, response implementation, response
      consumer implementation, parent-surface read-model implementation,
      parent-surface rendering, portal UI/consumer rendering, agent-protocol
      implementation, Rust protocol mirror, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness response consumer
      parent-surface status handoff proof now consumes those parent-surface
      read-model handoff rows and records that future parent-visible status
      consumption still requires parent-surface status proof, while source and
      compiler blockers remain blocked before status visibility. It remains
      parent-domain proof only and does not claim service command registration,
      service handler implementation, service event emission, service read API
      implementation, response implementation, response consumer
      implementation, parent-surface read-model/status implementation,
      parent-surface rendering, portal UI/consumer rendering, agent-protocol
      implementation, Rust protocol mirror, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness read-model proof now
      consumes those handoff rows and records that future service read-model
      visibility still requires service timer runtime, scheduler persistence,
      durable scheduler state-store, audit trail, rollback plan,
      audit/rollback read-model proof, parent-surface proof, service-readiness
      proof, and service read-API proof, while source and compiler blockers
      remain blocked before service/read-model visibility. It remains
      parent-domain proof only and does not claim agent protocol, service
      events/read API implementation, portal UI, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness protocol handoff
      proof now consumes those read-model rows and records that future
      protocol/service visibility still requires agent-protocol command/event
      proof, Rust protocol mirror proof, service handler proof, and service
      read-API proof, while source and compiler blockers remain blocked before
      protocol handoff. It remains parent-domain proof only and does not claim
      agent-protocol implementation, Rust protocol mirror, service command
      registration, service event emission, service read API implementation,
      portal UI, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime, durable
      audit logs, rollback runtime/execution, adapter dispatch, child delivery,
      broad blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness protocol read-model
      proof now consumes those protocol handoff rows and records that future
      protocol/read-model visibility still requires agent-protocol
      command/event proof, Rust protocol mirror proof, service handler proof,
      and service read-API proof, while source and compiler blockers remain
      blocked before service read-model events. It remains parent-domain proof
      only and does not claim agent-protocol implementation, Rust protocol
      mirror, service command registration, service event/read-model emission,
      service read API implementation, portal UI, evaluator runtime, timer
      runtime/scheduling, scheduler persistence runtime, durable scheduler
      storage, audit runtime, durable audit logs, rollback runtime/execution,
      adapter dispatch, child delivery, broad blocking, platform enforcement,
      or raw private source rows.
      Source-gated policy preview timer service-readiness protocol command
      handoff proof now consumes those protocol read-model rows and records
      that future command/service visibility still requires agent-protocol
      command proof, reported-event proof, Rust protocol mirror proof, service
      handler proof, and service read-API proof, while source and compiler
      blockers remain blocked before service command registration. It remains
      parent-domain proof only and does not claim agent-protocol
      implementation, Rust protocol mirror, service command registration,
      service handler implementation, service event emission, service read API
      implementation, portal UI, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit runtime,
      durable audit logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
      Source-gated policy preview timer service-readiness service-handler
      handoff proof now consumes those protocol command-handoff rows and
      records that future service visibility still requires service handler
      proof and service read-API proof, while source and compiler blockers
      remain blocked before service handler implementation. It remains
      parent-domain proof only and does not claim service command registration,
      service handler implementation, service event emission, service read API
      implementation, agent-protocol implementation, Rust protocol mirror,
      portal UI, evaluator runtime, timer runtime/scheduling, scheduler
      persistence runtime, durable scheduler storage, audit runtime, durable
      audit logs, rollback runtime/execution, adapter dispatch, child delivery,
      broad blocking, platform enforcement, or raw private source rows.
      Source-gated policy preview timer service-readiness read-API handoff proof
      now consumes those service-handler handoff rows and records that future
      read API visibility still requires service read-API proof, while source
      and compiler blockers remain blocked before read API visibility. It
      remains parent-domain proof only and does not claim service command
      registration, service handler implementation, service event emission,
      service read API implementation, agent-protocol implementation, Rust
      protocol mirror, portal UI, evaluator runtime, timer runtime/scheduling,
      scheduler persistence runtime, durable scheduler storage, audit runtime,
      durable audit logs, rollback runtime/execution, adapter dispatch, child
      delivery, broad blocking, platform enforcement, or raw private source
      rows.
- [ ] Child-facing reason/status is referenced in the runtime audit; finished
      child request/status UX remains. Child-facing UX contracts and
      text-domain copy tokens now cover respectful warning, approval-needed,
      time-limit, request, manual-required, and unavailable states with
      evidence/child refs and no diagnostics. Parent-domain child UX handoff
      rows now produce a schema-validated local JSONL artifact for ready app
      and game rows while blocked missing-ref rows stay out of the artifact,
      and the timer parent-surface service/portal read model now renders
      artifact record counts, skipped counts, artifact refs, and structured
      parent-safe artifact records with source result ids, target domains, and
      child reason/status refs. Deliverable child UX artifact rows now bridge
      to parent-owned local outbox JSONL records and then deterministic
      scheduler JSONL rows, then provider-preflight rows that require adapter,
      credential, and smoke-proof setup before any delivery claim, while
      manual-required and unavailable rows stay blocked/unscheduled. Child UX
      provider-preflight rows now bridge into the existing V0.8 provider-status
      boundary as manual-required or unavailable rows. Provider and preference
      status handoffs now combine into redacted parent-surface intent rows, and
      the live timer parent-surface service/portal read model renders the
      parent-surface intent manual-required, unavailable, history-visible,
      preference-setup, and reference-id aggregates. Rendered parent
      notification/preference UI, live child UI, provider delivery/receipt
      ingestion, retry execution, quiet-hours runtime, durable service
      persistence/export, package export, adapter dispatch, and platform
      execution remain. The live timer parent-surface read model now also
      carries structured parent-surface intent records with source result ids,
      parent-surface intent refs, artifact refs, target domains, drill-in refs,
      and manual-proof refs while keeping raw targets and private diagnostics
      out of the parent portal summary. App/Game Sessions now renders those
      structured records as parent action cards with readable manual-action and
      preference-setup states, parent-safe refs, and explicit no-claim adapter,
      child-delivery, and platform states. Parent-domain now derives
      parent-safe preference setup draft rows from those intent records for a
      later parent preference UI/mutation seam without claiming the UI,
      frequency controls, mutation, notification rule writes, delivery,
      adapter dispatch, platform enforcement, or raw private rows. The portal
      now renders those draft-ready/unavailable-visible setup records as
      read-only parent preference setup cards from the live App/Game Sessions
      event, and the timer parent-surface service now emits dedicated setup
      records so portal cards consume service read-model records directly
      instead of deriving them from parent-surface intent rows, still without
      claiming preference controls or mutation. The timer parent-surface read
      model now also carries parent preference setup request-ready/unavailable
      status and parent-safe request refs for those service records, and the
      `agent.activity.app-game.timer-parent-surface.parent-preference-setup.request`
      command/event returns an accepted command-boundary result for parent-safe
      setup refs. The App/Game Sessions portal marks request-ready setup records
      as UI-ready while preference mutation, notification rule writes, provider
      delivery, durable outbox storage, adapter dispatch, platform enforcement,
      raw private rows, and raw targets remain unclaimed. Request-ready setup
      rows now expose a parent setup request action that sends that command and
      displays the accepted result through the existing command-result panel,
      still without claiming durable preference mutation, notification rule
      writes, delivery, adapter dispatch, platform enforcement, raw private
      rows, or raw targets. The accepted request result now also includes
      parent-safe action-result handoff refs and explicit action-result
      persistence status, so the command boundary can feed the existing
      app/game action-result read-model path without claiming durable
      preference mutation, notification rule writes, delivery, adapter
      dispatch, platform enforcement, raw private rows, or raw targets. The
      accepted request result now marks persistence as claimed only after the
      service writes a replayable manual-required approval action-result row
      into the local ActivityStore, keeping the parent setup command on the
      shared app/game journal spine while still not mutating parent
      preferences, notification rules, adapters, platforms, delivery paths,
      raw private rows, or raw targets. The accepted request result also
      carries parent-safe mutation receipt refs/status, and the service stores
      a local receipt event beside the action-result row. That receipt proves
      the request was captured for the future preference mutation path while
      keeping durable preference mutation, notification rule writes, provider
      delivery, child runtime delivery, adapter dispatch, platform enforcement,
      raw private rows, and raw targets unclaimed. The accepted request result
      now also carries child-runtime delivery handoff refs/status, and the
      service persists a local handoff-ready audit event only after the local
      ActivityStore write succeeds. This proves the setup path can be handed to
      a future child runtime without claiming provider delivery, receipt
      ingestion, actual child runtime delivery, durable outbox storage, adapter
      dispatch, broad blocking, platform enforcement, raw private source rows,
      raw target values, or private diagnostics. The portal command-result
      panel now parses that accepted setup result and renders parent-safe
      action-result persistence, mutation receipt, and child-runtime handoff
      refs/status above the raw event payload, without upgrading the no-claim
      delivery, provider, outbox, adapter, broad-blocking, platform, raw-row,
      raw-target, or private-diagnostics boundaries. The accepted request
      result now also carries service-local child-runtime delivery queue
      refs/status, and the service persists a local queue audit event only
      after the action-result, mutation receipt, and handoff rows are accepted
      by the ActivityStore. This proves a queued local service handoff seam
      without claiming actual child runtime delivery, provider delivery,
      receipt ingestion, durable production outbox storage, adapter dispatch,
      broad blocking, platform enforcement, raw private source rows, raw target
      values, or private diagnostics. The portal command-result panel now
      renders those queue refs/status as parent-safe service-local readiness
      beside the action-result, mutation receipt, and child-runtime handoff
      rows, still without upgrading queue readiness into child delivery,
      provider delivery, receipt ingestion, durable outbox runtime, adapter
      dispatch, broad blocking, platform enforcement, raw private source rows,
      raw target values, or private diagnostics. The accepted request result
      now also carries service-local child-runtime delivery dispatch
      refs/status, and the service persists a dispatch-ready audit event after
      the queue row is accepted by the ActivityStore. This proves the queue can
      advance to a local dispatch seam without claiming actual child runtime
      receipt, provider delivery, receipt ingestion, durable production outbox
      runtime, adapter dispatch, broad blocking, platform enforcement, raw
      private source rows, raw target values, or private diagnostics.
      The portal command-result panel now renders those dispatch refs/status
      beside the action-result persistence, mutation receipt, handoff, and queue
      details while keeping dispatch readiness distinct from delivery,
      provider, durable outbox, adapter, broad-blocking, platform-enforcement,
      raw-source, raw-target, and private-diagnostic claims.
      The accepted request result now also carries service-local child-runtime
      delivery receipt-required refs/status, and the service persists a local
      receipt-required audit event after the dispatch-ready row is accepted by
      the ActivityStore. This proves the local dispatch seam now records that a
      future child-runtime receipt is required before delivery can be claimed,
      while still not claiming actual child runtime delivery or receipt,
      provider delivery, provider receipt ingestion, durable production outbox
      runtime, adapter dispatch, broad blocking, platform enforcement, raw
      private source rows, raw target values, or private diagnostics.
      The portal command-result panel now renders those receipt-required
      refs/status beside the action-result persistence, mutation receipt,
      handoff, queue, and dispatch details while keeping receipt-required
      readiness distinct from actual child runtime delivery/receipt, provider
      delivery/receipt ingestion, durable outbox runtime, adapter dispatch,
      broad-blocking, platform-enforcement, raw-source, raw-target, and
      private-diagnostic claims.
      The accepted request result now also carries service-local child-runtime
      delivery receipt-pending refs/status, and the service persists a local
      receipt-pending audit event after receipt-required is accepted by the
      ActivityStore. This proves the local path can track that a child-runtime
      receipt is awaited while still not claiming actual child runtime delivery
      or receipt, provider delivery/receipt ingestion, durable production
      outbox runtime, adapter dispatch, broad blocking, platform enforcement,
      raw private source rows, raw target values, or private diagnostics.
      The portal command-result panel now renders those receipt-pending
      refs/status beside the action-result persistence, mutation receipt,
      handoff, queue, dispatch, and receipt-required details while keeping
      receipt-pending readiness distinct from actual child runtime
      delivery/receipt, provider delivery/receipt ingestion, durable outbox
      runtime, adapter dispatch, broad-blocking, platform-enforcement,
      raw-source, raw-target, and private-diagnostic claims.
      The accepted request result now also carries service-local child-runtime
      delivery receipt-ingested refs/status, and the service persists a local
      receipt-ingested audit event after receipt-pending is accepted by the
      ActivityStore. This proves the service can record a local receipt
      ingestion boundary for the setup path while still not claiming provider
      receipt ingestion, durable production outbox runtime, adapter dispatch,
      broad blocking, platform enforcement, raw private source rows, raw target
      values, or private diagnostics. The portal command-result panel renders
      those receipt-ingested refs/status beside the action-result persistence,
      mutation receipt, handoff, queue, dispatch, receipt-required, and
      receipt-pending details while keeping local receipt ingestion distinct
      from provider delivery/receipt ingestion and platform enforcement.
      The accepted request result now also carries service-local durable outbox
      refs/status, and the service appends a parent-safe JSONL outbox record
      after the ActivityStore setup rows persist. The portal command-result
      panel renders those durable local outbox refs/status beside the setup
      receipt chain while keeping provider delivery, provider receipt
      ingestion, adapter dispatch, broad blocking, platform enforcement, raw
      private source rows, raw target values, and private diagnostics
      unclaimed.
      The accepted request result now also carries provider-delivery readiness
      refs/status. After the durable local outbox record is appended, the
      service persists a provider-delivery manual-required audit row and the
      portal command-result panel renders those readiness refs/status beside
      the setup chain. This is readiness visibility only: provider delivery,
      provider receipt ingestion, adapter dispatch, broad blocking, platform
      enforcement, raw private source rows, raw target values, and private
      diagnostics remain unclaimed.
      The accepted request result now also carries provider-delivery attempt
      refs/status. After the provider-delivery readiness row persists, the
      service persists a provider-delivery manual-required attempt audit row
      and the portal command-result panel renders those attempt refs/status.
      This is a manual-required handoff only: provider delivery execution,
      provider receipt ingestion, adapter dispatch, broad blocking, platform
      enforcement, raw private source rows, raw target values, and private
      diagnostics remain unclaimed.
      The accepted request result now also carries provider adapter and
      provider credential/manual-proof requirement refs/status. After the
      provider-delivery attempt row persists, the service records local
      manual-required requirement audit rows and the portal command-result
      panel renders those refs/status. This is preflight visibility only:
      provider delivery execution, provider receipt ingestion, adapter
      dispatch, broad blocking, platform enforcement, raw private source rows,
      raw target values, and private diagnostics remain unclaimed.
      The durable local setup outbox JSONL record now also carries the provider
      adapter and provider credential/manual-proof preflight requirement IDs and
      statuses, so persisted outbox rows retain the explicit manual-required
      provider blockers. This is durable preflight visibility only: provider
      delivery execution, provider receipt ingestion, adapter dispatch, broad
      blocking, platform enforcement, raw private source rows, raw target
      values, and private diagnostics remain unclaimed.
      The accepted request result now also carries provider-delivery local
      queue refs/status. After provider adapter and credential/manual-proof
      preflight rows persist, the service records a local provider-delivery
      queue audit row, the durable setup outbox serializes the queue ID/status,
      and the parent command-result panel renders the queue refs/status. This
      is a local queue seam only: provider delivery execution, provider receipt
      ingestion, adapter dispatch, broad blocking, platform enforcement, raw
      private source rows, raw target values, and private diagnostics remain
      unclaimed.
      The local provider-delivery seam now also tracks provider receipt-required
      and receipt-pending refs/status. The setup path persists local receipt
      tracking audit rows after the provider local queue row, writes the
      receipt tracking fields into the durable setup outbox, and renders those
      refs/status in the parent command-result details. This remains a
      parent-safe local tracking seam only: provider delivery execution,
      external provider receipt ingestion, adapter dispatch, broad blocking,
      platform enforcement, raw private source rows, raw target values, and
      private diagnostics remain unclaimed.
      The provider receipt tracking seam now also carries a local
      receipt-ingested boundary. The setup path persists a provider
      receipt-ingested audit row after receipt-pending tracking, serializes the
      receipt-ingested fields into the durable setup outbox, and renders those
      refs/status in parent command-result details while still keeping provider
      delivery execution, external provider receipt ingestion, adapter
      dispatch, broad blocking, platform enforcement, raw private source rows,
      raw target values, and private diagnostics unclaimed.
      The parent command-result surface now also aggregates the durable local
      outbox, provider queue, receipt-required, receipt-pending, and
      receipt-ingested chain into a parent-readable provider delivery status,
      next action, proof-state, and no-claim boundary. This keeps the detailed
      refs available while making the provider-delivery blocker understandable
      without adding a new protocol command during E-D shared protocol locks.
      The remaining app/game merge-blocking no-claim gates are now closed by a
      current parent-domain validation batch: classifier output remains
      evidence-only, dry-run action results stay `would-enforce`,
      manual-required action results cannot carry adapter enforcement results,
      Android normal-mode hide/suspend remains manual-required without owner
      proof, iOS process-kill rows remain unclaimed, and macOS hard block rows
      require MDM, Endpoint Security, or System Extension proof. This is
      contract hardening and proof only; provider delivery execution, adapter
      dispatch, broad blocking, platform enforcement, raw private rows, and the
      central product checklist remain unclaimed.
      The main App/Game Sessions dashboard now turns existing service-backed
      boundary counts and row states into parent-readable readiness blocker
      cards in the evidence drawer: missing approval action result, AI
      classifier evidence-only review, manual-required capability, and unknown
      approval review. It also exposes an aggregate `Readiness blockers` metric
      while keeping policy execution, adapter dispatch, broad blocking,
      platform enforcement, raw private source rows, raw target values, and
      private diagnostics unclaimed.
      The main App/Game Sessions dashboard also accepts the existing app/game
      platform-extension proof-pack readiness read model and turns macOS, iOS,
      Android, and Linux manual-required/not-executed rows into parent-visible
      `Platform gaps`, `Adapter executed`, capability, and evidence-drawer
      limitation rows. The UI proof keeps provider dispatch targets and raw
      platform diagnostics out of the dashboard and keeps broad blocking,
      child-device delivery, adapter execution, and platform enforcement
      unclaimed.
- [ ] Adapter capability status per platform. Cross-platform authority and
      broad-blocking gate contracts now record manual-required, unavailable,
      and not-claimed proof requirements. Platform-extension routing now maps
      every MAC, IOS, ANDROID, and LINUX row to proof packs and handoffs, and
      WP73 proof-pack readiness rows keep macOS, iOS, Android, and Linux
      native-app/native-game proof requirements visible. The App/Game Sessions
      dashboard now renders those platform proof-pack limitations as
      parent-visible UI rows without leaking private diagnostics or claiming
      adapter execution. Runtime adapter execution proof remains.
      App/game adapter execution readiness now derives an app/game-specific
      read model from the existing V0.8 supported-adapter runtime proof: only
      the scoped Windows owned-process time-limit boundary is
      execution-allowed, while broad app blocking, Linux, macOS, Android, iOS,
      and degraded permission/dependency rows stay blocked before execution.
      This narrows the runtime adapter execution truth without claiming broad
      blocking, child delivery, provider delivery, platform enforcement, or
      private diagnostics.
      Adapter readiness and dispatch preflight rows now also expose
      `hostCapabilityState`, host evidence refs, and parent-safe
      `hostCapabilityProbeRefs`: Windows scoped/manual/artifact rows are
      `available` with a Windows host probe ref, Android and Linux can show
      local ADB/WSL/Docker host visibility with separate opaque probe refs when
      detected, and macOS/iOS remain `not-applicable` from this Windows-local
      host path. These host states and probe refs do not upgrade Android/Linux
      into dispatch eligibility or platform enforcement.
      App/game adapter dispatch preflight now connects the live execution
      readiness read model to the existing V0.8 policy dispatch spine:
      exactly one scoped Windows owned-process app/game time-limit row is
      dispatch-eligible, and all broad, degraded, unavailable, unsupported, or
      manual-required rows stay blocked before dispatch. This is preflight
      visibility only; adapter dispatch execution, broad blocking, child
      delivery, provider delivery, platform enforcement, raw private rows, and
      private diagnostics remain unclaimed.
      App/game adapter dispatch result now turns that scoped preflight row into
      a parent-visible command-result handoff to `agent.enforcement.execute` /
      `agent.enforcement.audit.reported`, while every broad, degraded,
      unavailable, unsupported, or manual-required row remains blocked before
      command handoff. Adapter dispatch execution, broad blocking, child
      delivery, provider delivery, platform enforcement, raw private rows, and
      private diagnostics remain unclaimed.
      The dispatch result read model also records a service-local execution
      audit for the scoped Windows owned-process app/game timer command-result
      row and keeps all other rows blocked before execution audit. Actual
      adapter execution, broad blocking, child delivery, provider delivery,
      platform enforcement, raw private rows, and private diagnostics remain
      unclaimed.
      The dispatch result path now also accepts real `agent.enforcement.execute`
      audit evidence for that same scoped Windows owned-process app/game timer
      row. The read-model command remains side-effect-free and reports
      execution evidence as missing until a real
      `agent.enforcement.audit.reported` payload is attached; the focused proof
      runs the real service execution path that records audit events to journal
      and store. Broad installed-app blocking, platform enforcement outside the
      scoped Windows owned-process boundary, provider delivery, child-device
      delivery, raw private rows, raw target values, and private diagnostics
      remain unclaimed.
      The dispatch result live command now reads the latest persisted
      `activity.enforcement.audit-recorded` fields from `ActivityStore` and
      attaches that real store-backed evidence to the same scoped Windows
      owned-process app/game timer row. The default command no longer needs a
      hand-attached payload after the real enforcement execute path has written
      an audit row, while the side-effect-free builder remains available for
      explicit evidence tests. Broad installed-app blocking, platform
      enforcement outside the scoped Windows owned-process boundary, provider
      delivery, child-device delivery, raw private source rows, raw target
      values, and private diagnostics remain unclaimed.
      The dispatch path now also exposes an explicit manual
      `agent.activity.app-game.adapter-dispatch.execute` command that invokes
      the existing `agent.enforcement.execute` path only for that scoped
      Windows owned-process app/game timer row and returns a parent-safe
      `agent.activity.app-game.adapter-dispatch.executed` result. The read-model
      command remains side-effect-free, and portal overview polling does not
      auto-run execution. Broad installed-app blocking, platform enforcement
      outside the scoped Windows owned-process boundary, provider delivery,
      child-device delivery, raw private source rows, raw target values, and
      private diagnostics remain unclaimed.
      The portal live activity state now also retains the latest manual
      dispatch executed event separately from the side-effect-free read model,
      and the dispatch result panel renders parent-safe execute command,
      result, status, audit, and readback details. This makes the manual
      executed result visible without turning overview polling into execution
      and without claiming broad installed-app blocking, non-scoped platform
      enforcement, provider delivery, child-device delivery, raw private rows,
      raw targets, or private diagnostics.
      The App/Game Sessions route now mounts that adapter dispatch surface as
      parent-visible route cards with side-effect-free preflight/result refresh
      controls and a separate explicit manual execute button that is shown only
      when the scoped Windows owned-process app/game timer row is accepted for
      command handoff. The button sends
      `agent.activity.app-game.adapter-dispatch.execute` and selects the
      `agent.activity.app-game.adapter-dispatch.executed` result event while
      keeping overview polling, broad installed-app blocking, non-scoped
      platform enforcement, provider delivery, child-device delivery, raw
      private rows, raw targets, and private diagnostics unclaimed.
- [x] Platform manual artifact gates carry host probe refs without support
      upgrades. Windows manual rows now carry a Windows host-local probe ref,
      Android manual rows carry separate ADB path and SDK probe refs, and the
      Linux unavailable row carries separate WSL and Docker probe refs through
      the V0.8 OS adapter manual artifact gate read model and proof. macOS and
      iOS rows keep empty probe-ref lists because Windows-local execution
      cannot prove those platforms. Android/Linux support, macOS/iOS local
      execution, broad installed-app blocking, non-scoped platform enforcement,
      provider delivery, child-device delivery, raw private rows/targets, and
      private diagnostics remain unclaimed.
- [x] Host capability summary counts are parent-visible. The service-backed
      app/game adapter execution readiness and dispatch preflight read models
      now include aggregate available, not-detected, not-applicable, and
      probe-ref counts derived from the same rows that carry Windows,
      Android, Linux, macOS, and iOS host capability states. TypeScript
      protocol parsers reject count mismatches, and portal-domain summary
      intents render those counts so Android ADB and Linux WSL/Docker host
      visibility is visible without exposing raw paths, device serials, distro
      names, or private diagnostics. Android/Linux dispatch eligibility,
      macOS/iOS local execution, broad installed-app blocking, non-scoped
      platform enforcement, provider delivery, and child-device delivery remain
      unclaimed.
- [x] Android physical-device proof is redacted and non-promoting. The
      app/game Android proof boundary now accepts only a physical-device ADB
      target with parent-safe build/package/policy-state evidence, stores only
      package counts rather than package names, redacts raw device serials, and
      keeps normal-mode hide/suspend, adapter dispatch, platform enforcement,
      broad blocking, provider delivery, and child-device delivery unclaimed
      unless Device Owner or Profile Owner proof is attached.
- [x] Linux WSL runtime proof is redacted and non-promoting. The app/game Linux
      proof boundary now records WSL2 Ubuntu kernel, distro, package-manager,
      process, session, and Docker CLI visibility as parent-safe counts/states
      while redacting raw package names, process names, distro names, and host
      paths. Linux broad blocking, adapter dispatch, platform enforcement,
      rollback, audit, provider delivery, and child-device delivery remain
      unclaimed until the named Linux mechanism/distro/session/rollback/audit
      proof set is complete.
- [x] Android/Linux platform proof status is parent-visible and non-promoting.
      The parent-domain status read model now aggregates the Android physical
      device proof and Linux WSL runtime proof into review-only platform rows,
      and portal-domain renders package/runtime visibility counts, proof refs,
      and open gaps without leaking raw package names, raw process names, raw
      distro names, raw device serials, or host paths. Android/Linux adapter
      dispatch, broad blocking, platform enforcement, provider delivery,
      child-device delivery, rollback, audit, and private diagnostics remain
      unclaimed; macOS/iOS stay outside local runtime proof on this Windows
      host.
- [x] App/game platform proof status is service-backed and portal-rendered.
      The Rust protocol/service and TypeScript protocol now expose
      `agent.activity.app-game.platform-proof-status.read-model.get` /
      `.reported`, and the App/Game Sessions route renders Windows scoped
      execution, Android host visibility, Linux host visibility, and macOS/iOS
      not-locally-provable rows. The surface remains parent-safe and does not
      claim adapter dispatch, broad installed-app blocking, platform
      enforcement, provider delivery, child-device delivery, raw private
      rows/targets, or private diagnostics.
- [x] Android UsageEvents foreground proof is redacted and non-promoting. The
      physical Android proof harness now records redacted `dumpsys usagestats`
      event sample counts and foreground activity event counts from the Samsung
      Galaxy S9 target without storing package names, class names, raw device
      serials, or raw app activity rows. Android platform proof status treats
      this as foreground visibility only; durable child-runtime replay, Device
      Owner/Profile Owner authority, hide/suspend, broad blocking, platform
      enforcement, provider delivery, and child-device delivery remain
      unclaimed.
- [ ] Linux display/socket readiness source is present but not proof-complete.
      Rust production code classifies only WSLg/native Linux after WSLg-specific
      trusted runtime/socket checks, validates fixed canonical X11/Wayland roots,
      and performs bounded Unix-socket connects without publishing paths,
      titles, or process rows. No Linux probe or retained proof was run in this
      source-only phase; static WSL/Docker presence does not mint a detail ref.
      Active foreground capture, App/Game ownership, policy restrictions,
      launch blocking, rollback, audit, broad blocking, platform enforcement,
      provider delivery, and child-device delivery remain unclaimed.
- [ ] Platform proof status has a typed Linux detail-ref path, pending tests and
      retained live evidence. The service attaches Linux refs only after a
      separately owned source-ready preflight succeeds; no display/socket
      result alone mints a tool or active-window ref. Android durable replay
      and Linux active foreground capture remain open, as do platform enforcement,
      adapter dispatch, provider delivery, child-device delivery, raw private
      source rows, raw target values, and private diagnostics.
- [x] Android UsageEvents replay readiness is parent-safe and count-only. The
      parent-domain replay read model turns the redacted physical-device
      UsageEvents foreground sample counts into a durable-replay-ready runtime
      visibility row and feeds that ref into parent-domain platform proof
      status. It still does not store raw UsageEvents rows, package names, raw
      activity data, child runtime delivery, Device Owner/Profile Owner
      authority, hide/suspend, adapter dispatch, or platform enforcement.
- [ ] Linux foreground capture readiness has a Rust source boundary but remains
      validation-open. A typed display/socket preflight can report only a
      trusted bounded socket outcome; remote/invalid `DISPLAY`, pure Wayland,
      xprop/xdotool, and xwd/convert capture are unavailable until separate
      process/artifact custody owners exist. This lane did not run tests or
      proof and does not claim active foreground capture, selected-window/title
      capture, App/Game ownership, adapter dispatch, Linux policy enforcement,
      provider delivery, or child-device delivery.
- [x] Android UsageEvents child-runtime replay is count-only and non-promoting.
      The parent-domain child-runtime replay row now consumes the redacted
      UsageEvents replay readiness row and attaches a child-runtime replay
      consumer boundary without storing raw UsageEvents rows, package names, or
      raw activity data. Android child-device delivery, Device Owner/Profile
      Owner authority, hide/suspend, adapter dispatch, broad blocking, provider
      delivery, and platform enforcement remain unclaimed.
- [ ] Linux foreground source preflight is typed and fail-closed, pending tests
      and live proof. Rust source reports trusted WSLg/native display/socket
      outcomes only; remote/invalid `DISPLAY`, pure Wayland, xprop/xdotool,
      and active-window state remain unavailable because no owned process
      custody primitive exists. The service does not spawn an orphanable
      per-request worker. It does not report a current host result, raw window
      identity, App/Game ownership, enforcement authority, or capture claim.
      Selected-window/title capture is unavailable because raw-title search
      violates the metadata boundary.
      AppArmor/SELinux/package-manager enforcement, rollback, audit, adapter
      dispatch, provider delivery, and child-device delivery remain unclaimed.
- [x] Android authority preflight is explicit and non-promoting. The
      parent-domain preflight row now maps the physical Android policy-state
      proof into hide, suspend, uninstall-block, lock-task, and managed
      configuration rows. On the current Samsung Galaxy S9 target every row is
      blocked before adapter dispatch because Device Owner/Profile Owner proof
      is absent, and `not-proved` states are not treated as owner proof.
      Android action execution, platform enforcement, provider delivery,
      child-device delivery, raw package names, and raw device serial custody
      remain unclaimed.
- [x] Android Accessibility overlay preflight is explicit and non-promoting.
      The parent-domain preflight row now maps redacted physical Android
      Accessibility settings into warning, block, request, and usage-context
      overlay rows. The proof stores enabled-state and enabled-service count
      only, keeps service/component names redacted, and keeps overlay actions
      blocked before adapter dispatch until service enablement, overlay runtime
      proof, and child-device delivery proof exist. Overlay execution, platform
      enforcement, provider delivery, raw overlay content, and raw service-name
      custody remain unclaimed.
- [x] Windows broad-blocking authority preflight is explicit and non-promoting.
      The parent-domain preflight row now maps the AppLocker/App Control
      broad-blocking manual gates into block-launch, system-app allowlist,
      rollback, and audit-custody rows. Windows host visibility is attached
      through an opaque probe ref, but AppLocker/App Control enforce proof,
      system-app allowlist proof, rollback proof, and audit custody proof
      remain required before broad installed-app launch blocking can dispatch.
      Raw executable paths, raw policy XML, adapter dispatch, platform
      enforcement, provider delivery, and child-device delivery remain
      unclaimed.
- [x] Windows local policy evidence is sampled without a broad-blocking claim.
      The parent-domain proof runner now samples AppIDSvc, AppLocker local
      policy readability, and Device Guard/App Control state as parent-safe
      counts and booleans. Raw policy XML, raw executable paths, raw publisher
      rules, broad blocking, adapter dispatch, platform enforcement, and
      child-device delivery remain unclaimed. AppLocker/App Control enforcement,
      system-app allowlist proof, rollback proof, audit custody proof, adapter
      dispatch, and child-device delivery remain open.
- [x] Platform proof status consumes platform preflight detail refs. The
      parent-domain status model carries typed Windows, Android, and Linux
      detail refs when a separately owned source-ready preflight supplies
      them; the unavailable Linux runtime path emits no refs. Windows broad
      blocking, Android owner authority, Android overlay runtime, Linux
      foreground/policy mechanisms, adapter dispatch, platform enforcement,
      provider delivery, child-device delivery, raw private rows/targets, and
      private diagnostics remain unclaimed.
- [x] Apple platform proof is CI-required and non-promoting. The parent-domain
      Apple CI preflight maps existing macOS and iOS manual artifact gates into
      CI-required rows and feeds them into the shared platform proof status
      when supplied. Windows-local execution is not counted as macOS or iOS
      proof; macOS MDM/Endpoint/System Extension/rollback/audit, iOS
      FamilyControls/DeviceActivity/ManagedSettings/TestFlight/device proof,
      adapter dispatch, platform enforcement, provider delivery, and
      child-device delivery remain unclaimed.
- [x] Linux Docker host preflight is explicit and non-promoting. The
      parent-domain preflight row now records Docker CLI, daemon, context,
      image, and container inventory visibility as parent-safe booleans and
      counts. Docker context names, image names, container ids, raw paths, and
      private daemon diagnostics are not stored. Container policy, adapter
      dispatch, platform enforcement, provider delivery, and child-device
      delivery remain unclaimed.
- [x] Android UsageEvents capability bridge is package-local and non-promoting.
      Android package source now includes an app/game UsageEvents capability
      bridge and MainActivity surfaces its bridge state. Parent-domain keeps
      UsageStats as settings-grant-required and rejects raw UsageEvents storage,
      package-name custody, adapter dispatch, platform enforcement, and
      child-device delivery claims. The Android manifest still does not declare
      `PACKAGE_USAGE_STATS`; runtime collection, settings grant, Device
      Owner/Profile Owner authority, provider delivery, and Play policy proof
      remain open.
- [x] Android UsageEvents runtime preflight is package-local and non-promoting.
      Android package source now checks UsageStats AppOps state and UsageStats
      service visibility, and MainActivity surfaces only the permission
      preflight state. Parent-domain keeps runtime sample collection blocked
      until proof exists and rejects raw UsageEvents storage, package-name
      custody, adapter dispatch, platform enforcement, and child-device delivery
      claims. Settings grant, runtime sample proof, Device Owner/Profile Owner
      authority, provider delivery, and Play policy proof remain open.
- [x] Android UsageEvents package sampling is count-only. The Android package
      preflight can query UsageEvents and reduce the result to total event count
      and foreground event count when UsageStats is granted. Parent-domain
      accepts only count summaries and rejects raw UsageEvents storage,
      package-name custody, raw activity rows, adapter dispatch, platform
      enforcement, and child-device delivery claims. Physical-device settings
      grant and live package sample observation remain separate proof work.
- [x] Android UsageEvents package runtime is physically launched. The Android
      debug package is installed and MainActivity is launched on the physical
      Samsung Galaxy S9 proof target, with AppOps and package UI state reduced
      to parent-safe UsageEvents permission/sample states. Raw device serials,
      raw UI XML, package lists, UsageEvents rows, package names, activity rows,
      adapter dispatch, platform enforcement, provider delivery, and
      child-device delivery remain unclaimed.
- [x] Android Accessibility runtime is package-declared and non-promoting. The
      Android package now declares an Ocentra AccessibilityService with
      `BIND_ACCESSIBILITY_SERVICE`, includes a service config that listens for
      window-state changes without requesting window-content retrieval, and
      exposes parent-safe declaration/runtime/event-sample states through
      MainActivity. Parent-domain proof rejects raw Accessibility event rows,
      raw service/component names, raw overlay content, overlay execution,
      adapter dispatch, platform enforcement, and child-device delivery claims.
      Service enablement, event sample observation, overlay runtime execution,
      Device Owner/Profile Owner authority, provider delivery, and Play policy
      proof remain open.
- [ ] Linux active-window tool probing is explicitly unavailable and
      source-only. Rust does not spawn xprop/xdotool because process-group
      containment cannot prove custody across setsid/pid-namespace escapes;
      `_NET_ACTIVE_WINDOW` therefore remains observed/not-observed in the type
      but is never observed by the runtime path. No test or proof artifact was
      produced and no static ref is emitted. Raw window titles, raw process
      names, selected-window/title capture, App/Game ownership, foreground
      capture, adapter dispatch, platform enforcement, provider delivery, and
      child-device delivery remain unclaimed.
- [ ] Platform proof status has a runtime detail-ref path pending an owned
      source-ready probe, tests, and retained proof. Linux refs are derived
      only from typed preflight outcomes and are empty for the unavailable
      runtime path; Docker/WSL presence is not evidence. Android
      Accessibility, Windows local policy, and existing physical-device rows
      remain separate, with adapter dispatch, broad blocking, platform
      enforcement, provider delivery, child-device delivery, raw policy XML,
      raw executable paths, raw service names, raw event rows, raw window titles,
      and private diagnostics unclaimed.
- [x] Child-device delivery readiness is explicit and non-promoting. The
      parent-domain read model now maps the existing child UX provider-status
      handoff into child-transport-required, manual-required, and unavailable
      rows for native app and native game child-facing surfaces. It carries only
      parent-safe transport refs and keeps runtime child transport, receipt
      ingestion, provider delivery execution, platform delivery channel
      execution, adapter dispatch, platform enforcement, and raw private source
      rows unclaimed.
- [x] Child-device runtime writer envelopes are explicit and non-executing. The
      parent-domain writer read model converts transport-required child-device
      delivery readiness rows into writer-envelope-ready rows while keeping
      manual-required and unavailable rows blocked. Runtime writer execution,
      child runtime transport, receipt ingestion, provider delivery execution,
      platform delivery channel execution, adapter dispatch, platform
      enforcement, and raw private source rows remain unclaimed.
- [x] Child runtime transport and receipt boundary is explicit and
      non-executing. The parent-domain boundary read model converts
      writer-envelope-ready rows into child-runtime-transport-required rows with
      required receipt contract refs, while manual-required and unavailable rows
      remain blocked. Runtime transport execution, receipt ingestion, provider
      delivery execution, platform delivery channel execution, adapter
      dispatch, platform enforcement, and raw private source rows remain
      unclaimed.
- [x] Child runtime transport and receipt boundary is service-visible. The
      TypeScript and Rust agent protocols now register
      `agent.activity.app-game.child-runtime-transport-receipt.read-model.get`
      / `.reported`, and the Rust service returns a parent-safe read model
      through the WebSocket command handler. Runtime transport execution,
      receipt ingestion, provider delivery execution, platform delivery channel
      execution, adapter dispatch, platform enforcement, and raw private source
      rows remain unclaimed.
- [x] Child runtime transport and receipt boundary is parent-visible. The
      App/Game Sessions parent portal route now requests and renders the
      service-backed child runtime transport receipt read model with
      transport-required, manual-required, and unavailable rows plus parent-safe
      transport refs, receipt refs, and open gaps. Runtime transport execution,
      receipt ingestion, provider delivery execution, platform delivery channel
      execution, adapter dispatch, platform enforcement, and raw private source
      rows remain unclaimed.
- [x] Android child runtime transport receipt readiness is child-app visible.
      The Android child agent package now exposes a parent-safe transport
      channel, internal receipt-store, and receipt-ack status bundle in the
      activity UI, and parent-domain proof requires those states before the row
      can be counted as ready. Physical child runtime transport execution,
      receipt ingestion, provider delivery execution, platform delivery channel
      execution, adapter dispatch, platform enforcement, and raw private source
      rows remain unclaimed.
- [x] Android child runtime local receipt append/readback is child-app visible.
      The Android child agent package now writes a deterministic parent-safe
      receipt marker to its internal app files, reads it back, and renders local
      receipt append/readback states in the activity UI. Parent-domain proof
      accepts only package-local receipt write/readback evidence. Physical child
      runtime transport execution, service receipt ingestion, provider delivery
      execution, platform delivery channel execution, adapter dispatch,
      platform enforcement, and raw private source rows remain unclaimed.
- [x] Android child runtime local receipt append/readback is physically proved.
      The Android debug package installs and launches on the physical Samsung
      Galaxy S9 proof target, and debug `run-as` reads the deterministic
      internal receipt marker. UIAutomator receipt text capture is attempted but
      remains non-guaranteed on this phone; service receipt ingestion, provider
      delivery, platform delivery channel execution, adapter dispatch, platform
      enforcement, and raw private source rows remain unclaimed.
- [x] Android child runtime local receipt acknowledgement is package-local and
      physically proved. The Android child agent now writes and reads both a
      local receipt marker and a local receipt-ack marker, surfaces ack/readback
      states in `MainActivity`, and the physical Samsung Galaxy S9 proof target
      exposes both marker files through debug `run-as`. Runtime transport,
      service receipt ingestion, provider delivery, platform delivery channel
      execution, adapter dispatch, platform enforcement, and raw private source
      rows remain unclaimed.
- [x] Android child runtime package-local receipt channel is physically proved.
      The Android child agent now declares a non-exported in-package receipt
      receiver, `MainActivity` can trigger it for proof mode, and the physical
      Samsung Galaxy S9 proof target exposes channel, receipt, and receipt-ack
      marker files through debug `run-as`. Service receipt ingestion, provider
      delivery, platform delivery outside the package, adapter dispatch,
      platform enforcement, and raw private source rows remain unclaimed.
- [x] Android child runtime package-local delivery intake is physically
      proved. The Android child agent now declares a non-exported in-package
      delivery receiver, `MainActivity` can trigger it for proof mode, and the
      physical Samsung Galaxy S9 proof target exposes delivery, channel,
      receipt, and receipt-ack marker files through debug `run-as`. Service
      delivery or receipt ingestion, provider delivery, platform delivery
      outside the package, adapter dispatch, platform enforcement, and raw
      private source rows remain unclaimed.
- [x] Android child runtime package-local delivery queue and drain are
      physically proved. The Android child agent now records delivery intake,
      queue, and drain markers while preserving package-local channel, receipt,
      and receipt-ack custody. The physical Samsung Galaxy S9 proof target
      exposes all six marker files through debug `run-as`; service delivery or
      receipt ingestion, provider delivery, platform delivery outside the
      package, adapter dispatch, platform enforcement, and raw private source
      rows remain unclaimed.
- [x] Android UsageEvents granted sample observation is physically proved. The
      Android child agent now renders UsageEvents sample and foreground-event
      counts from its package-local preflight, and the physical Samsung Galaxy
      S9 proof target grants `GET_USAGE_STATS` through AppOps before launching
      the debug package and observing a count-only live sample. Parent-domain
      accepts only granted AppOps, activity-visible count evidence, and opaque
      proof refs; raw UsageEvents rows, package names, activity rows, raw UI
      XML, raw device serials, Device/Profile Owner authority, Play policy
      proof, child-device delivery, adapter dispatch, provider delivery, broad
      blocking, and platform enforcement remain unclaimed.
- [x] Android Accessibility enabled-sample boundary is physically checked. The
      Android child agent now renders a count-only Accessibility window-state
      event field, and parent-domain has a success contract for settings-enabled
      bound services with observed event counts. The physical Samsung Galaxy S9
      proof runner emits that success read model only if Android binds the
      package service; otherwise it records a manual-required blocker proof for
      the target. Raw Accessibility event rows, service names, overlay content,
      raw UI XML, raw device serials, Device/Profile Owner authority, Play
      policy proof, child-device delivery, adapter dispatch, provider delivery,
      broad blocking, and platform enforcement remain unclaimed.
- [x] Android child runtime local app/game notification is physically proved.
      The Android child agent package creates a dedicated app/game local
      notification channel, posts a package-local child notice through Android
      `NotificationManager`, and writes a package-local marker for debug
      readback. Parent-domain accepts only channel, post, marker, and opaque
      proof refs; provider delivery, platform delivery outside the package,
      child request approval round trip, adapter dispatch, broad blocking,
      platform enforcement, and raw private source-row custody remain
      unclaimed.
- [x] Android child runtime local app/game notification request action is
      package-local. The Android child agent package attaches an ask-parent
      action to the local app/game notification, routes the action to a
      non-exported in-package receiver, and writes a package-local request
      marker for debug readback. Parent-domain accepts only channel, post,
      action, marker, and opaque proof refs; service request ingestion, parent
      approval round trip, provider delivery, platform delivery outside the
      package, adapter dispatch, broad blocking, platform enforcement, and raw
      private source-row custody remain unclaimed.
- [x] Android child runtime local app/game request queue is package-local. The
      Android child agent package records request queue, readback, and drain
      markers after the local notification request action. Parent-domain
      accepts only action, queue, readback, drain, and opaque proof refs;
      service request ingestion, parent approval round trip, provider delivery,
      platform delivery outside the package, adapter dispatch, broad blocking,
      platform enforcement, and raw private source-row custody remain
      unclaimed.
- [x] Blocking/time-limit proof before done claim. Scoped owned-process
      time-limit proof exists; broad block-launch/hide/suspend/shield and
      allowlist remain manual-required or not-claimed until platform setup,
      authority-tier, rollback, audit, and platform proof are attached.
      The feature-local done gate now aggregates the scoped
      `agent.activity.app-game.adapter-dispatch.execute` proof, the explicit
      App/Game Sessions parent action proof, and the broad-blocking
      manual-required/not-claimed gate proof. The gate fails if read-model
      refresh executes, if broad installed-app blocking gains a dispatch row or
      adapter call, or if non-scoped platform enforcement, provider delivery,
      child-device delivery, raw private rows/targets, or private diagnostics
      become claimed.

## Next AI Instructions

Do not equate session evidence with blocking. Keep app identity quality,
category confidence, policy decisions, and adapter results as separate typed
states. Treat
`scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs` as scoped
app/game time-limit audit proof only, not broad installed-app blocking proof.
Browser-game and cloud-gaming web surfaces belong in the browser plan when the
source is managed browser evidence; native games, launchers, process/session
duration, and broad app blocking stay in this app/game feature.
