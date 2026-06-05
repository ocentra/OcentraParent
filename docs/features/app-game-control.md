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
- Supporting docs: [app settings inventory](../app-control-settings-inventory.md)
  and [game settings inventory](../game-control-settings-inventory.md).
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
- The service now exposes those staged app/game journal and SQLite projections
  through typed app-use and games activity-surface read models, including
  inventory, runtime, foreground, launcher/source-count, daily rollup,
  capability, and evidence-ref fields.
- The parent portal App/Game Sessions surface now renders those service-backed
  app-use and games read-model rows in a dedicated dashboard intent and SVG
  surface with separate inventory, running, foreground, launcher-only,
  unknown-review, manual-required capability, game-budget gap, and evidence
  counts.
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
- Parent-domain policy preview handoff contracts now map already-compiled
  app/game dry-run policy decisions into read-only preview rows, separating
  native app versus native game target meaning while keeping evaluator runtime,
  timers, adapter dispatch, child delivery, and platform enforcement unclaimed.
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
- Backend source-status rows now preserve manual-required, degraded, and
  not-claimed capability states as non-ready read-model state, so source
  freshness cannot look ready when a platform/source still needs proof.
- The parent App/Game Sessions dashboard now consumes those service-backed
  `sourceStatusRows` and renders source row counts, fresh source counts,
  source-kind capability state, latest observed timestamps, and evidence ref
  counts through the existing metric/evidence surfaces without adding backend,
  policy, or adapter claims.
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
The app/game policy preview handoff proof is also contract-level: it consumes
compiled dry-run decisions and produces parent-domain preview rows only; it
does not run the policy evaluator, persist policy previews in the service,
render authoring UI, start timers, deliver child notifications, dispatch
adapters, or prove platform enforcement.
The app/game time-budget proof is contract-level: it does not yet provide
runtime service evaluation, Rust/WebSocket parity, portal budget authoring,
notification delivery, child request UX, service persistence, adapter
execution, or platform timer/rollback execution.
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
      game-context candidates as evidence-backed soft/manual policy inputs; live
      candidate production, runtime service consumption, portal category UI, and
      parent/child UX remain.
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
      rows. Live evaluator, authoring UI, persistence, timers, and enforcement
      remain. Rust protocol
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
- [ ] Child-facing reason/status is referenced in the runtime audit; finished
      child request/status UX remains. Child-facing UX contracts and
      text-domain copy tokens now cover respectful warning, approval-needed,
      time-limit, request, manual-required, and unavailable states with
      evidence/child refs and no diagnostics; live child UI, notifications,
      service persistence, and platform execution remain.
- [ ] Adapter capability status per platform. Cross-platform authority and
      broad-blocking gate contracts now record manual-required, unavailable,
      and not-claimed proof requirements. Platform-extension routing now maps
      every MAC, IOS, ANDROID, and LINUX row to proof packs and handoffs, but
      runtime adapter capability/UI proof remains.
- [ ] Blocking/time-limit proof before done claim. Scoped owned-process
      time-limit proof exists; broad block-launch/hide/suspend/shield and
      allowlist remain manual-required or not-claimed until platform setup,
      authority-tier, rollback, audit, and platform proof are attached.

## Next AI Instructions

Do not equate session evidence with blocking. Keep app identity quality,
category confidence, policy decisions, and adapter results as separate typed
states. Treat
`scripts/test/v0-8-enforcement-integrity-runtime-audit.mjs` as scoped
app/game time-limit audit proof only, not broad installed-app blocking proof.
Browser-game and cloud-gaming web surfaces belong in the browser plan when the
source is managed browser evidence; native games, launchers, process/session
duration, and broad app blocking stay in this app/game feature.
