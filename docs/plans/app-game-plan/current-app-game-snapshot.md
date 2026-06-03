# Current App + Game Snapshot - 2026-06-02

## Product Claim Boundary

Current source proves a scoped app/game evidence and enforcement path. It does
not prove a product-complete app/game subsystem.

Proved today:

- App/game session contracts and read-model proof exist.
- App/game evidence claim, AI classification digest, and parent app/game control
  authority schemas now exist as TypeScript contract proof.
- App/game layered identity and identity-merge schemas now exist as TypeScript
  contract proof.
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
- App-control and game-control catalog/authoring contracts exist in
  `packages/parent-domain`.
- Rust app/game session protocol mirrors exist.
- SQLite-backed app/game observation helpers exist.
- Scoped Windows owned-process app time-limit proof exists.
- Portal live activity and policy-preview surfaces can render service-backed
  evidence and policy states.
- Existing docs keep broad installed-app blocking manual-required.

Not proved today:

- Product-complete app inventory and identity quality.
- Live Windows registry, Start Menu, executable metadata, signature/hash, or
  launcher manifest crawling.
- Live Microsoft Store/UWP/AppX/MSIX package enumeration, Store API integration,
  install approval, purchase approval, or package-wide blocking.
- Live Windows process polling, process start/exit subscription, executable
  metadata collection, publisher/signature/hash collection, or portal runtime
  dashboard rows for the new WP08 contract. Journal replay and service
  read-model exposure are currently staged fixture proof, not live source
  wiring.
- Live Windows foreground-window polling, active-window subscription, or portal
  foreground dashboard rows for the new WP09 contract. Journal replay and
  service read-model exposure are currently staged fixture proof, not live
  source wiring.
- Product-complete native game catalog, live launcher disambiguation, and live
  game budget product behavior beyond WP18 dry-run contract proof.
- Runtime app/game policy target evaluation beyond WP19 dry-run contract proof.
- Runtime app/game time-budget evaluation beyond WP20 dry-run contract proof.
- Live new/unknown app and unknown game approval flow beyond the WP17
  contract-level request/decision proof.
- Live category/risk classification, catalog enrichment, policy compiler
  routing, service/read-model parity, and portal category/risk rows for the new
  WP12 contract.
- Broad app/game blocking outside scoped owned-process proof.
- macOS, Linux, Android, iOS, MDM, supervised/device-owner, Endpoint Security,
  AppLocker/App Control, Screen Time, ManagedSettings, store/signing, entitlement,
  kiosk, and single-app claims.
- Runtime adapter proof for the new authority matrix rows. The WP11 contract
  names what proof is needed to move up, but it does not attach real platform
  enrollment, permission, rollback, cleanup, service, or portal evidence.

## Current Contracts

`packages/activity-domain` currently provides combined app/game primitives,
identity and identity-merge proof, inventory evidence rows, inventory entry,
process observation, evidence claim, session summary/query/report, AI digest
reference, and AI classification digest contracts. These support useful
evidence and session proof, but they do not yet cover the complete shared
app/game contract stack from this plan:

- launcher evidence live source adapters beyond the staged launcher row/parser
  proof;
- foreground evidence rows beyond the staged foreground-window parser proof,
  including live capture, encrypted journal-file replay, and portal foreground
  rows;
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
handoff, effective budget math, and timer recovery refs. Those catalogs,
authority contracts, approval contracts, budget contracts, compiler contracts,
and time-budget contracts are product-control inputs, not a complete shared
evidence spine, live approval workflow, live game budget product flow, runtime
evaluator, or proof that broad app/game blocking works.

## Current Runtime

`crates/agent-core` currently has SQLite-backed app/game observation and
deterministic sessionization helpers, typed Windows installed-record and
Store/UWP package adapter/parser proof, staged Windows process runtime parser
proof, staged Windows foreground-window parser proof, staged Windows launcher
evidence parser proof, and scoped Windows owned-process time-limit helpers.
This is a strong base for workpacks 11, 12, 13, 14, and 21, but it is not the same
as:

- live Windows installed app inventory crawling;
- live UWP/AppX package inventory enumeration;
- live Windows process runtime polling or subscribed process start/exit events;
- live Windows foreground-window polling or subscribed foreground transition
  events;
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
  now expose typed runtime state, but live process capture, executable metadata
  crawling, and dedicated portal runtime rows are not implemented yet.
- Foreground evidence contracts and Rust foreground-row parity are present, a
  staged Windows foreground-window parser proof exists, staged journal/SQLite
  replay proof now projects foreground-now rows, and service activity-surface
  read models now expose typed foreground state, but live foreground capture,
  dedicated portal foreground rows, and content-aware claims are not
  implemented.
- Rust protocol parity has not yet mirrored the WP01 evidence claim, AI digest,
  app/game control authority schemas, or WP04 identity schemas.
- Journal and SQLite ingest now covers staged app/game inventory, runtime,
  foreground, launcher, and daily rollup rows, and the service now maps those
  rows into app-use and games activity-surface read models. It does not yet
  store the WP01 evidence-claim or authority proof shapes, and it is not yet
  wired to live source subscriptions or dedicated portal dashboard rows.
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
- Launcher evidence has contract/protocol/parser proof and service DTO exposure
  from staged projection rows, but live launcher crawling, dedicated portal
  rows, and game-budget policy are not product-complete. A launcher row must
  not become a game session unless linked child-game proof exists.
- Game categories, ratings, UGC, multiplayer, purchase signals, and app risk
  labels are now represented as contract-level policy inputs, but not live
  classifier output, product UI proof, or safety decisions by themselves.
- Existing scoped Windows owned-process time-limit proof should be reused, not
  expanded into broad blocking claims.

## Worker Handoff Notes - 2026-06-02

- WP01 completed the first TypeScript contract boundary on
  `codex/app-plan-work` with proof under
  `output/app-game-plan-proof/01-contract-boundary-and-effect-schemas/`.
- WP02/WP03 reconcile routing and snapshot docs only. They do not change product
  checklist status and do not claim runtime completion.
- WP04 adds TypeScript identity model proof only. It does not add runtime
  identity merge, inventory adapters, Rust parity, or portal identity rows.
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
- Next implementation work should either add dedicated portal dashboard
  consumption of the service rows, live Windows inventory source readers, or
  mirror the remaining WP01/WP04 TypeScript shapes into Rust protocol before
  service/runtime consumers depend on them.

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
