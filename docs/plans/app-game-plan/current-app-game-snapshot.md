# Current App + Game Snapshot - 2026-06-02

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `Current App + Game Snapshot - 2026-06-02`
> Kind: current snapshot; read for status/gap claims.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Current code-first correction - 2026-08-15

The authoritative current snapshot is [CODE_AUDIT.md](CODE_AUDIT.md): 220/220
workpacks are mapped, 170 have no bounded Phase 1 writing gap, and 50 still
need production code or expected tests. The June narrative below preserves
historical implementation sequencing and removed TypeScript-owner references;
it must not override the current tracked-source audit.

## Product Claim Boundary

Current source proves a scoped app/game evidence and enforcement path. It does
not prove a product-complete app/game subsystem.

Proved today:

- App/game session contracts and read-model proof exist.
- App/game evidence claim, AI classification digest, and parent app/game control
  authority schemas now exist as TypeScript contract proof.
- App/game layered identity and identity-merge schemas now exist as TypeScript
  contract proof.
- Rust protocol parity now mirrors the app/game evidence claim, AI digest
  reference, AI classification digest, layered identity, and identity-merge
  shapes from `packages/activity-domain` with serialization proof.
- Rust protocol parity now also mirrors the parent-domain app/game approval
  authority/action-result, platform authority matrix, and AI classifier result
  boundary shapes with serialization proof and no live adapter claim.
- App/game journal/SQLite ingest now stores and projects the newly mirrored
  evidence claim, identity, approval authority, approval action-result,
  platform authority matrix, and AI classifier result protocol rows through
  staged encrypted-journal replay with no-use, manual-required, and
  AI-cannot-enforce rejection guards.
- App/game service read models now preserve refs for those staged
  evidence-claim, identity, approval authority/action-result, platform
  authority matrix, and AI classifier result rows in the existing app-use/games
  evidence vectors, without adding live classifier execution, policy
  consumption, dedicated portal rows, or adapter execution.
- App/game inventory evidence rows now exist as TypeScript contract proof with
  source, custody, category candidates, stale/permission-limited states, and
  no-use guards.
- Rust protocol now mirrors app/game inventory evidence rows, and `agent-core`
  has a typed Windows-installed inventory record adapter/parser proof that keeps
  inventory separate from runtime or foreground use.
- Microsoft Store/UWP/AppX/MSIX package identity now has a typed `agent-core`
  parser proof for store app/game inventory rows, deterministic package/AUMID
  runtime merge checks, and AppUserModelId policy-target handoff.
- Windows process runtime evidence now has a first-class TypeScript contract,
  Rust protocol parity, and a staged `agent-core` parser proof for process
  appearance, persistence, exit closure, unknown process state, launcher
  runtime-only state, permission-limited metadata, and runtime session-summary
  readiness without foreground claims.
- Windows foreground app/game evidence now has a first-class TypeScript
  contract, Rust protocol parity, and a staged `agent-core` parser proof for
  active foreground focus, foreground switch closure, background no-time guards,
  omitted title refs, permission-limited metadata, launcher-only focus, unknown
  foreground process state, and foreground-is-not-content guards.
- Launcher evidence now has a first-class TypeScript contract, Rust protocol
  parity, and a staged `agent-core` parser proof for launcher-only rows,
  launcher foreground staying launcher-only, launcher-game candidates, proved
  child-game promotion, permission-limited launcher state, and
  launcher-is-not-game guards.
- Cross-platform authority matrix rows now have a first-class parent-domain
  TypeScript contract and test proof for authority tier, setup state, proof
  state, parent-visible limitation, proof-needed fields, and no-execute gates
  across Windows, macOS, Linux, Android, and iOS/iPadOS hard-control actions.
- App/game category-risk taxonomy rows now have first-class activity-domain
  TypeScript contract and test proof for native app categories, native game
  categories, risk candidates, game context signals, source refs, confidence,
  reason codes, evidence refs, parent display overrides, AI digest refs,
  policy-candidate actions, duplicate ids, and `notEnforcement` state.
- App/game AI classifier boundary proof now adds a parent-domain
  policy-facing result contract that requires stored evidence refs, confidence
  bounds, runtime/model/prompt refs, fallback state, and evidence-only policy
  handoff while rejecting direct action, duration, and raw scan fields before
  policy can consume classifier output.
- App/game sessionization now derives deterministic running, foreground,
  background, stale-gap, process-exit, replay-stable, and daily rollup duration
  rows from stored SQLite observations with TypeScript and Rust protocol parity.
- App/game journal/SQLite ingest now has staged encrypted-journal replay proof
  for typed inventory, runtime, foreground, and launcher rows, including
  inventory, running-now, foreground-now, launcher, and daily rollup read-model
  projection plus invalid evidence and duplicate-duration guards.
- App/game service read models now expose the staged journal/SQLite projection
  through typed app-use and games activity-surface DTOs with product,
  classification, inventory, runtime, foreground, capability, last-observed,
  source-count, and evidence-ref fields.
- A real `sysinfo` live process snapshot source now feeds runtime-only app/game
  rows through core encrypted-journal replay, SQLite projection, and the
  service activity-capture journal/store path.
- The service startup activity-capture path now repeats bounded live process
  capture on a protocol-owned cadence, and focused proof shows two capture
  cycles append two queryable app/game runtime rows while foreground stays
  unclaimed.
- A live active-window foreground source now feeds app/game foreground records
  and journal events through core encrypted-journal replay and SQLite
  projection with opaque window/title refs, without title/content capture,
  service capture, policy, or adapter claims.
- The service activity-capture path can now append that app/game foreground
  event on the same bounded cadence when the active-window source is available,
  proving optional foreground rows through the existing journal/store/read-model
  path without content, policy, adapter, or platform support claims.
- Native game budget policy contracts now provide dry-run proof for known-game
  counts, launcher-only exclusion, parent-approved launcher-game candidate
  inclusion, advisory rating/UGC/multiplayer/purchase signals, and
  no-enforcement handoff.
- App/game policy target compiler contracts now provide dry-run proof for
  identity-backed specific targets, unknown-state targets, category/risk/game
  signal targets, schedule proof, capability/authority refs, device/local-user
  freshness, and manual-required block-launch output.
- App/game time-budget contracts now provide dry-run proof for stored app/game
  session refs, running versus foreground duration modes, schedule evidence
  refs, bonus-time approval/audit refs, ask-parent/manual-required handoff,
  effective budget math, and restart-recovered timer refs.
- Child-facing app/game UX contracts now provide proof for warning,
  approval-needed, time-limit, request submitted/approved/denied,
  manual-required, and unavailable states with text-domain copy tokens,
  evidence refs, child reason/status refs, and private-diagnostic guards.
- Scoped Windows owned-process app/game time-limit proof now covers dry-run
  no-action, stale action mismatch rejection before adapter execution,
  recover/cancel of the preserved timer, and owned/current expiry through the
  existing process adapter.
- Broad app/game blocking proof gates now cover manual-required, unavailable,
  and not-claimed contract states for Windows AppLocker/App Control, AppLocker
  audit-only non-enforce proof, macOS hard-block proof, Linux
  mechanism/distro/session proof, Android normal-mode hide/suspend, iOS
  ManagedSettings shielding, and iOS process-kill no-claim. The gate matrix
  proves these states cannot dispatch adapters and that supported upgrades
  require setup, authority-tier, rollback, audit, and platform-specific proof
  references.
- Platform-extension routing proof now records all MAC, IOS, ANDROID, and LINUX
  extension checklist rows with authority tier, setup state, manual tags,
  proof-pack paths, and cross-plan handoff, while keeping every row
  manual-required or not-claimed until real platform proof is attached.
- Install/store handoff proof now records new inventory, installer/updater,
  store package install, game purchase signal, uninstall, and tamper candidate
  rows with evidence refs, approval/tamper feature routes, context-only
  store/purchase guards, parent-visible manual-required states, and no adapter
  or policy-decision claims.
- App/game performance-health proof now records generated-scale budget rows for
  1,000 inventory records, 500 runtime rows, 500 foreground transitions, 10,000
  journal records, 100,000 replay observations, 1,000 policy compile parses,
  500 existing dashboard intent rows, and parent-visible degraded adapter
  health without claiming live platform throughput or browser DOM render proof.
- App/game final rollout/evidence gate proof now verifies the app-game WP01-WP27
  and app-plan WP01-WP26 proof roots, records E2E/manual scenario routing,
  writes final app-game WP28 and app-plan WP27/WP28 proof packs, and captures
  no-claim/manual-required and PR-ready report requirements without claiming new
  runtime, portal, or platform support.
- App-control and game-control catalog/authoring contracts exist in
  `packages/parent-domain`.
- Rust app/game session protocol mirrors exist.
- SQLite-backed app/game observation helpers exist.
- Scoped Windows owned-process app time-limit proof exists and remains scoped to
  owned/current process time-limit expiry, not package-wide blocking.
- Portal live activity and policy-preview surfaces can render service-backed
  evidence and policy states.
- Existing docs keep broad installed-app blocking manual-required.

Not proved today:

- Product-complete app inventory and identity quality.
- Live Windows registry, Start Menu, executable metadata, signature/hash, or
  launcher manifest crawling.
- Live Microsoft Store/UWP/AppX/MSIX package enumeration, Store API integration,
  install approval, purchase approval, or package-wide blocking.
- Live Windows process start/exit subscription, executable metadata collection,
  publisher/signature/hash collection, or portal runtime dashboard rows for the
  new WP08 contract. Bounded live process polling now feeds the service
  journal/store path, but richer source subscriptions and metadata crawling
  remain gaps.
- Subscribed foreground transition events or dedicated portal foreground source
  rows for the WP09 contract. Core foreground-source proof and bounded service
  capture now exist, but subscription-style transitions and portal rendering are
  not wired.
- Product-complete native game catalog, live launcher disambiguation, and live
  game budget product behavior beyond WP18 dry-run contract proof.
- Runtime app/game policy target evaluation beyond WP19 dry-run contract proof.
- Runtime app/game time-budget evaluation beyond WP20 dry-run contract proof.
- Live child-facing app/game warning/request UI beyond WP21 contract/text proof.
- Live new/unknown app and unknown game approval flow beyond the WP17
  contract-level request/decision proof.
- Live category/risk classification, catalog enrichment, policy compiler
  routing, service/read-model parity, and portal category/risk rows for the new
  WP12 contract.
- Live AI classifier provider execution, model quality, runtime classifier
  service events, policy evaluator consumption, portal classifier rows, and
  adapter enforcement for the WP24 classifier boundary. WP31 stores/projections
  staged classifier result rows, and WP38 carries their refs through app-use/
  games evidence vectors; neither executes a provider or feeds policy.
- Broad app/game blocking outside scoped owned-process proof.
- Broad-blocking gate contracts now make no-claim/manual-required platform
  requirements explicit, but they do not implement AppLocker/App Control, MDM,
  Endpoint Security, Device Owner/Profile Owner, FamilyControls/
  ManagedSettings, cgroup/systemd, AppArmor/SELinux, package restriction,
  rollback execution, service events, or portal capability UI.
- macOS, Linux, Android, iOS, MDM, supervised/device-owner, Endpoint Security,
  AppLocker/App Control, Screen Time, ManagedSettings, store/signing, entitlement,
  kiosk, and single-app claims.
- Runtime adapter proof for the new authority matrix rows. The WP11 contract
  names what proof is needed to move up, but it does not attach real platform
  enrollment, permission, rollback, cleanup, service, or portal evidence.
- Live implementation proof for the new platform-extension routing rows. WP25
  proves route coverage and promotion guards only; it does not implement
  platform adapters, enrollment, rollback execution, service events, or portal
  capability UI.
- Live store integration, install/purchase approval UI, package-manager or store
  interception, billing entitlement logic, uninstall blocking, and anti-tamper
  behavior. WP26 proves handoff routing and no-claim guards only.
- Live OS inventory/process/foreground throughput, encrypted journal disk
  throughput and corruption recovery, browser DOM/Playwright rendering for 500
  app/game rows, and live adapter health telemetry. WP27 proves generated-scale
  and existing dashboard intent bounds only.
- Final rollout/evidence gate proof is review infrastructure only. It proves
  proof-root completeness and no-claim routing for this branch, but it does not
  add missing live source adapters, approval UI, notifications, policy runtime,
  platform adapters, product reports, or browser DOM proof for every UI state.

## Current Contracts

`packages/activity-domain` currently provides combined app/game primitives,
identity and identity-merge proof, inventory evidence rows, inventory entry,
process observation, evidence claim, session summary/query/report, AI digest
reference, and AI classification digest contracts. These support useful
evidence and session proof, but they do not yet cover the complete shared
app/game contract stack from this plan:

- launcher evidence live source adapters beyond the staged launcher row/parser
  proof;
- foreground evidence rows beyond the staged foreground-window parser, core
  active-window source proof, and bounded service capture bridge, including
  dedicated portal foreground rows;
- runtime-fed native app and native game category quality beyond the WP12
  category/risk taxonomy contract proof;
- rating, UGC, multiplayer, and purchase signals;
- policy targets for app and game product slices;
- live unknown app and unknown game approval requests beyond contract proof;
- authority tier and capability rows for every action;
- enforcement result and rollback proof for app/game actions.

`packages/parent-domain` currently provides separate app-control and
game-control catalog schemas/data, enforcement policy dispatch contracts, and
app/game control approval authority/request/decision/action-result contract
proof plus the WP11 platform authority matrix. WP17 now adds contract-level
unknown app/game approval candidates, child status/reason refs, response scope,
expiry, audit-backed replay state, and manual-required outcomes. WP18 now adds
contract-level native game budget dry-run targets, launcher inclusion policy,
advisory signal boundaries, and no-enforcement handoff. WP19 now adds
contract-level app/game policy target compiler requests and compiled dry-run
decisions. WP20 now adds contract-level app/game time-budget decisions with
stored session refs, schedule refs, bonus approval/audit refs, dry-run/manual
handoff, effective budget math, and timer recovery refs. WP21 now adds
contract-level child-facing warning/request cards that link copy tokens,
evidence refs, child reason/status refs, and manual/unavailable no-action
guards. WP25 now adds platform-extension routing contracts for MAC, IOS,
ANDROID, and LINUX rows, including proof-pack and cross-plan handoff
requirements. WP26 now adds install/store handoff contracts for new inventory,
installer/updater, store package install, game purchase signal, uninstall, and
tamper candidate rows. WP27 now adds performance-health contracts for generated
scale budgets and degraded adapter health. Those catalogs, authority contracts,
approval contracts, budget contracts, compiler contracts, time-budget
contracts, child UX contracts, extension routing contracts, install/store
handoff contracts, and performance-health contracts are product-control inputs,
not a complete shared evidence spine, live approval workflow, live game budget
product flow, runtime evaluator, child UI, live platform throughput proof, or
proof that broad app/game blocking works.

## Current Runtime

`crates/agent-core` currently has SQLite-backed app/game observation and
deterministic sessionization helpers, typed Windows installed-record and
Store/UWP package adapter/parser proof, staged Windows process runtime parser
proof, staged Windows foreground-window parser proof, core live active-window
foreground source proof, staged Windows launcher evidence parser proof, and
scoped Windows owned-process time-limit helpers.
This is a strong base for workpacks 11, 12, 13, 14, and 21, but it is not the same
as:

- live Windows installed app inventory crawling;
- live UWP/AppX package inventory enumeration;
- live Windows process runtime polling or subscribed process start/exit events;
- subscribed foreground transition events and dedicated portal foreground source
  rows;
- live launcher manifest adapters;
- dedicated portal dashboard rendering for replayed app/game rows;
- foreground app evidence adapters;
- game-specific launcher-child process disambiguation;
- broad block-launch enforcement;
- cross-platform runtime parity.

## Current Portal

The portal can show live activity, activity timeline, policy preview, capability
guidance, device rule scope, layout panels, app/game navigation entry points,
and a dedicated App/Game Sessions dashboard surface backed by app-use and games
activity-surface read models. It is not yet a complete parent-facing app/game
product flow.

Missing portal states include:

- full app/game identity and catalog quality beyond reported read-model rows;
- live unknown app and unknown game approval request surfaces beyond contract
  proof;
- game budget policy authoring and category rows;
- live risk app and risk game candidate production rows;
- platform capability matrix rows backed by live authority proof;
- malicious/long metadata resilience screenshots beyond the focused portal
  route proof.

## Current Gaps

- App/game identity contracts are present, but runtime identity merge behavior
  and adapter-fed identity refs are not implemented yet.
- Inventory evidence row contracts and Rust inventory-row parity are present,
  Windows installed-record plus Store/UWP package parser proof exists, staged
  journal/SQLite replay proof now projects inventory rows, and service
  activity-surface read models now expose typed inventory state, but live
  platform crawling and dedicated portal dashboard rows are not implemented yet.
- Runtime evidence contracts and Rust runtime-row parity are present, a staged
  Windows process runtime parser proof exists, staged journal/SQLite replay
  proof now projects running-now rows, and service activity-surface read models
  now expose typed runtime state. Bounded live process capture now refreshes
  that same service path, but executable metadata crawling, richer
  subscriptions, and dedicated portal runtime rows are not implemented yet.
- Foreground evidence contracts and Rust foreground-row parity are present, a
  staged Windows foreground-window parser proof exists, core live active-window
  source proof now emits foreground rows and journal events with opaque
  window/title refs, journal/SQLite replay now projects foreground-now rows, the
  bounded service capture bridge can append optional foreground rows, and
  service activity-surface read models expose typed foreground state. Dedicated
  portal foreground rows, subscribed foreground transitions, and content-aware
  claims are not implemented.
- Rust protocol parity now mirrors the WP01 evidence claim, AI digest
  reference/classification digest, WP04 identity/identity-merge shapes, the
  app/game control authority/action-result schemas, the platform authority
  matrix, and the WP24 parent-domain classifier boundary. WP31 adds staged
  journal/SQLite storage and read-model projection for evidence claim,
  identity, authority, action-result, platform authority matrix, and classifier
  result rows. WP38 carries those staged row refs through existing app-use/games
  service read-model evidence vectors. Live source subscriptions, classifier
  execution, dedicated classifier service events, policy runtime, portal
  authority/classifier rows, and adapter execution remain incomplete.
- Journal and SQLite ingest now covers staged app/game inventory, runtime,
  foreground, launcher, daily rollup, evidence-claim, identity, approval
  authority, approval action-result, platform authority matrix, and AI
  classifier result rows. The service still maps only the established app-use
  and games activity-surface rows, but those rows now retain staged
  authority/classifier storage refs in their evidence vectors. The new
  authority/classifier rows are not yet wired to live source subscriptions,
  dedicated service events, policy consumers, or portal dashboard rows.
- Portal App/Game Sessions dashboard rows now consume the app-use and games
  activity-surface DTOs through a shared dashboard intent, but approval,
  policy, game-budget, live source, and platform-authority surfaces remain
  incomplete.
- Unknown approval flow now has parent-domain contract proof for weak
  app/game candidates, child refs, response scopes, expiry, replay state, and
  manual-required blocks, but live candidate production, notification delivery,
  service read models, and parent/child approval UI remain incomplete.
- Native game budget policy now has parent-domain contract proof for game
  budget targets, known-game inclusion, launcher-only exclusion,
  parent-approved launcher-game candidate inclusion, advisory rating/UGC,
  multiplayer, and purchase signals, and dry-run-only outcomes. It does not
  yet provide policy compiler integration, service persistence, portal budget
  authoring/preview UI, bonus-time integration, notifications, or adapter
  execution.
- App/game policy target compiler now has parent-domain contract proof for
  app/game targets, identity/unknown/category/schedule/capability/authority
  proof, device/local-user/freshness rejection, dry-run-only decisions, and
  manual-required unproved block-launch. It does not yet provide Rust/service
  parity, runtime evaluator execution, portal rule authoring/preview UI, timer
  integration, notifications, rollback, or adapter execution.
- App/game time-budget policy now has parent-domain contract proof for stored
  app/game session refs, running versus foreground duration modes, schedule
  evidence, bonus-time approval/audit refs, ask-parent/manual-required dry-run
  states, effective budget math, and restart-recovered timer refs. It does not
  yet provide Rust/service parity, runtime evaluator execution, service
  persistence, portal budget authoring/preview UI, notification delivery, child
  request UX, adapter execution, or platform timer/rollback execution.
- Child-facing app/game UX now has parent-domain/text-domain contract proof for
  respectful warning, approval-needed, time-limit, request submitted/approved/
  denied, manual-required, and unavailable states with safe copy tokens,
  evidence refs, child reason/status refs, and no private diagnostics. It does
  not yet provide live child UI, native overlay rendering, portal preview
  screenshots, notification delivery, service persistence, Rust/WebSocket
  parity, adapter execution, or platform shield/block behavior.
- Launcher evidence has contract/protocol/parser proof and service DTO exposure
  from staged projection rows, but live launcher crawling, dedicated portal
  rows, and game-budget policy are not product-complete. A launcher row must
  not become a game session unless linked child-game proof exists.
- Game categories, ratings, UGC, multiplayer, purchase signals, and app risk
  labels are now represented as contract-level policy inputs, but not live
  classifier output, product UI proof, or safety decisions by themselves.
- Existing scoped Windows owned-process time-limit proof should be reused, not
  expanded into broad blocking claims.
- Existing broad-blocking gate proof should be reused as the no-claim guard
  before any future platform adapter, rollback, or UI work moves broad
  blocking out of manual-required/unavailable/not-claimed state.

## Worker Handoff Notes - 2026-06-02

- WP01 completed the first TypeScript contract boundary on
  `codex/app-plan-work` with proof under
  `output/app-game-plan-proof/01-contract-boundary-and-effect-schemas/`.
- WP02/WP03 reconcile routing and snapshot docs only. They do not change product
  checklist status and do not claim runtime completion.
- WP04 adds TypeScript identity model proof only. It does not add runtime
  identity merge, inventory adapters, or portal identity rows. Rust parity for
  the WP04 identity shapes is covered by WP29.
- WP05 adds TypeScript inventory evidence row proof only. It does not add
  platform adapters, journal ingest, Rust parity, or portal inventory rows.
- WP06 adds Rust inventory-row parity and a typed Windows installed inventory
  adapter/parser proof only. It does not add live registry crawling, shell-link
  parsing, journal ingest, service/runtime inventory events, or portal inventory
  rows.
- WP07 adds Store/UWP/AppX/MSIX package parser proof only. It does not add live
  package enumeration, Store API integration, install/purchase approval, journal
  ingest, service/runtime inventory events, or portal inventory rows.
- WP08 adds process runtime evidence contract/protocol/parser proof only. It
  does not add live process polling, process-capture integration, journal
  ingest, SQLite replay, service/runtime events, foreground evidence, portal
  runtime rows, policy execution, or broad blocking.
- WP09 adds foreground evidence contract/protocol/parser proof only. It does not
  add live foreground-window polling, window-capture integration, journal ingest,
  SQLite replay, service/runtime events, portal foreground rows, content
  knowledge, policy execution, or broad blocking.
- WP10 adds launcher evidence contract/protocol/parser proof only. It does not
  add live launcher manifest crawling, live launcher-child process linking,
  journal ingest, SQLite replay, service/runtime events, portal launcher rows,
  game-budget policy, install/purchase approval, or broad blocking.
- WP11 adds cross-platform authority matrix contract/test proof only. It does
  not add AppLocker/App Control, MDM, Endpoint Security, Device Owner/Profile
  Owner, FamilyControls/ManagedSettings, cgroup/systemd, root/admin, kiosk, or
  single-app runtime adapters.
- WP12 adds category/risk taxonomy contract/test proof only. It does not add
  live category enrichment, local AI classifier quality, policy compiler
  routing, service/read-model parity, portal rows, or direct enforcement.
- WP13 adds deterministic local SQLite-row sessionization proof for running,
  foreground, background, stale gap, process exit, replay order, session end
  reasons, observation gaps, and daily rollups. It does not add encrypted
  journal-file ingest/replay, live source subscriptions, service events, portal
  dashboard rows, policy execution, UI proof, or broad blocking.
- WP14 adds staged encrypted journal-file append/replay plus SQLite projection
  proof for typed inventory, runtime, foreground, launcher, running-now,
  foreground-now, launcher, and daily rollup rows. It does not add live source
  subscriptions, service events, portal dashboard rows, policy execution,
  approval flow, corruption/recovery proof, UI proof, or broad blocking.
- WP15 adds service-backed app-use and games activity-surface read-model DTOs
  over the staged app-game journal/SQLite projection. It does not add dedicated
  portal dashboard UI, policy/approval read models, live source subscriptions,
  platform authority changes, UI proof, or broad blocking.
- WP18 adds parent-domain native game budget contract/test proof for known-game
  counts, launcher-only exclusion, parent-approved launcher-game candidate
  inclusion, advisory rating/UGC/multiplayer/purchase signal boundaries, and
  dry-run-only decisions. It does not add Rust/service parity, budget
  persistence, portal authoring/preview UI, notifications, bonus-time
  integration, platform adapter execution, or broad blocking.
- WP19 adds parent-domain app/game policy target compiler contract/test proof
  for specific, unknown, category/risk, game-signal, schedule, capability,
  authority, device, local-user, and stale evidence boundaries. It does not add
  Rust/service parity, runtime evaluator execution, portal rule authoring,
  notifications, timers, rollback, platform adapter execution, or broad
  blocking.
- WP20 adds parent-domain app/game time-budget contract/test proof for stored
  session refs, running versus foreground modes, schedule evidence refs,
  bonus-time approval/audit refs, ask-parent/manual-required dry-run states,
  effective budget math, and restart-recovered timer refs. It does not add
  Rust/service parity, runtime evaluator execution, service persistence, portal
  budget authoring/preview UI, notification delivery, child request UX, adapter
  execution, or platform timer/rollback execution.
- WP21 adds parent-domain/text-domain child-facing UX contract/test proof for
  warning, approval-needed, time-limit, request submitted/approved/denied,
  manual-required, and unavailable states. It does not add live child UI,
  overlay rendering, portal preview screenshots, notification delivery, service
  persistence, Rust/WebSocket parity, adapter execution, or platform
  shield/block behavior.
- WP23 adds parent-domain app/game broad-blocking proof gates for Windows,
  macOS, Linux, Android, and iOS/iPadOS manual-required, unavailable, and
  not-claimed states. It does not add runtime adapters, service events, portal
  UI, rollback execution, or any broad app/game blocking support claim.
- WP25 adds parent-domain platform-extension routing proof for MAC, IOS,
  ANDROID, and LINUX extension rows. It does not promote any row out of
  manual-required/not-claimed state or add runtime platform adapters.
- WP26 adds parent-domain install/store handoff proof for new app/game
  inventory, installer/updater process, store package install, game purchase,
  uninstall, and tamper candidate rows. It does not add store integration,
  approval UI, platform adapter execution, uninstall blocking, or anti-tamper
  behavior.
- WP29 adds Rust protocol parity for the activity-domain WP01 evidence claim,
  AI digest reference/classification digest, and WP04 identity/identity-merge
  shapes. It does not add service storage, journal/SQLite ingest for those
  shapes, live adapters, portal identity rows, policy runtime, or product
  status movement.
- WP30 adds Rust protocol parity for parent-domain app/game approval authority,
  approval request/decision/action result, platform authority matrix, and
  WP24 classifier result boundary shapes. It does not add service storage,
  live classifier/provider execution, policy evaluator consumption, portal
  authority/classifier rows, adapter execution, or product status movement.
- WP31 adds staged encrypted-journal and SQLite read-model projection for the
  newly mirrored evidence claim, identity, approval authority/action result,
  platform authority matrix, and classifier result protocol rows. It does not
  add live source adapters, live classifier/provider execution, dedicated
  service events, portal authority/classifier rows, policy evaluator
  consumption, adapter execution, or product status movement.
- WP32 adds a real `sysinfo` process snapshot source in `agent-core` that reads
  the current local process table into app/game runtime records, hashes
  executable paths into opaque refs, and preserves runtime-only/no-foreground
  boundaries. It does not add journal subscription, service events, portal
  source freshness, policy consumption, or adapter execution.
- WP33 adds a core bridge from live process snapshots into app/game runtime
  journal events, encrypted-journal replay, and SQLite read-model rows. It does
  not add service polling/subscription, portal source freshness, foreground
  capture, policy consumption, or adapter execution.
- WP34 wires those bounded live process app/game runtime journal events into the
  existing `agent-service` activity-capture journal/store path, proves the
  service store projects a runtime row into the app/game read model, and keeps
  recurring polling, foreground source, portal freshness, policy, and adapter
  execution out of scope.
- WP35 adds a recurring bounded service capture cadence for that same live
  process journal bridge and proves two capture cycles refresh app/game runtime
  rows through the existing journal/store/read-model path without foreground,
  policy, or adapter claims.
- WP36 adds a core live active-window foreground source that maps foreground
  metadata into foreground rows and journal events with opaque window/title refs,
  while leaving service capture, portal UI, policy, and adapters out of scope.
- WP37 wires that foreground source into the bounded service capture event list,
  proving optional foreground rows through the existing journal/store/read-model
  path while leaving portal UI, policy, adapters, and platform support out of
  scope.
- WP38 carries staged evidence-claim, identity, approval authority/action-result,
  platform authority matrix, and AI classifier result row refs through the
  existing service app-use/games evidence vectors. It does not add live
  classifier/provider execution, dedicated authority/classifier service events,
  policy consumption, portal rows, adapter execution, or product status
  movement.
- WP39 adds explicit staged boundary row counts for evidence claim, identity,
  approval authority/action-result, platform authority matrix/rows, and AI
  classifier result rows to the existing service app-use/games read-model
  payloads. It does not add live classifier/provider execution, dedicated
  authority/classifier event streams, policy consumption, portal rows, adapter
  execution, or product status movement.
- WP40 adds a dedicated app/game boundary read-model command/event and payload
  parser for staged evidence-claim, identity, approval authority/action-result,
  platform authority matrix/rows, and AI classifier result counts plus citation
  refs. It does not add portal rows, policy consumption, live provider
  execution, adapter execution, or product status movement.
- WP41 adds a bounded core live Windows shortcut inventory source that reads
  Start Menu `.lnk` entries from provided or platform-discovered roots, hashes
  path-derived source and desktop-entry refs, and replays inventory-only journal
  events into SQLite rows. It does not add registry crawling, Store package
  enumeration, service capture, portal source freshness, policy consumption, or
  adapter execution.
- WP42 wires that live Windows shortcut inventory source into the bounded
  service activity-capture event list, proving inventory-only rows through the
  existing encrypted journal/store/read-model path while leaving registry
  crawling, Store package enumeration, portal source freshness, policy
  consumption, adapters, and platform support out of scope.
- Next implementation work should either add portal rows for the newly projected
  boundary event, polish portal source freshness states, add policy/runtime
  consumers for those protocol structs, or expand live inventory source coverage
  only with no-claim proof gates.

## Enhancement Rule

Enhance current paths in place:

- add shared app/game evidence contracts to `packages/activity-domain`;
- add app/game policy and control contracts to `packages/parent-domain`;
- mirror Rust-crossing shapes in `crates/agent-protocol`;
- extend `agent-core` store/session helpers before service/portal claims;
- render through existing portal live-activity and policy-preview surfaces;
- reuse existing proof scripts where they cover the claim;
- add new proof only where existing scripts cannot cover app/game-specific
  requirements.
