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
  metadata collection, publisher/signature/hash collection, service events, or
  portal runtime rows for the new WP08 contract. Journal replay is currently
  staged fixture proof, not live source wiring.
- Live Windows foreground-window polling, active-window subscription, service
  events, or portal foreground rows for the new WP09 contract. Journal replay
  is currently staged fixture proof, not live source wiring.
- Product-complete native game catalog, live launcher disambiguation, and game
  budgets.
- New/unknown app and unknown game approval flow.
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
- unknown app and unknown game approval requests;
- authority tier and capability rows for every action;
- enforcement result and rollback proof for app/game actions.

`packages/parent-domain` currently provides separate app-control and
game-control catalog schemas/data, enforcement policy dispatch contracts, and
app/game control approval authority/request/decision/action-result contract
proof plus the WP11 platform authority matrix. Those catalogs and authority
contracts are product-control inputs, not a complete shared evidence spine or a
proof that broad app/game blocking works.

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
- service events and portal read-model exposure for replayed app/game rows;
- foreground app evidence adapters;
- game-specific launcher-child process disambiguation;
- broad block-launch enforcement;
- cross-platform runtime parity.

## Current Portal

The portal can show live activity, activity timeline, policy preview, capability
guidance, device rule scope, layout panels, and app/game navigation entry
points. It is not yet a complete parent-facing app/game dashboard.

Missing portal states include:

- installed apps and installed games without use claims;
- running apps/games without foreground claims;
- foreground active app/game without content claims;
- launcher-only versus active game rows;
- launcher-game candidate rows;
- unknown app and unknown game approval requests;
- game budget and game category rows;
- live risk app and risk game candidate production rows;
- platform capability matrix with manual-required proof;
- malicious/long metadata resilience screenshots.

## Current Gaps

- App/game identity contracts are present, but runtime identity merge behavior
  and adapter-fed identity refs are not implemented yet.
- Inventory evidence row contracts and Rust inventory-row parity are present,
  Windows installed-record plus Store/UWP package parser proof exists, and
  staged journal/SQLite replay proof now projects inventory rows, but live
  platform crawling, service events, and portal rows are not implemented yet.
- Runtime evidence contracts and Rust runtime-row parity are present, a staged
  Windows process runtime parser proof exists, and staged journal/SQLite replay
  proof now projects running-now rows, but live process capture, executable
  metadata crawling, service events, and portal runtime rows are not implemented
  yet.
- Foreground evidence contracts and Rust foreground-row parity are present, a
  staged Windows foreground-window parser proof exists, and staged
  journal/SQLite replay proof now projects foreground-now rows, but live
  foreground capture, service events, portal foreground rows, and content-aware
  claims are not implemented.
- Rust protocol parity has not yet mirrored the WP01 evidence claim, AI digest,
  app/game control authority schemas, or WP04 identity schemas.
- Journal and SQLite ingest now covers staged app/game inventory, runtime,
  foreground, launcher, and daily rollup rows. It does not yet store the WP01
  evidence-claim or authority proof shapes, and it is not yet wired to live
  source subscriptions, service events, or portal rows.
- Portal app/game dashboard rows do not yet consume the new contracts.
- Launcher evidence has contract/protocol/parser proof, but live launcher
  crawling, journal/read-model ingest, service events, portal rows, and
  game-budget policy are not product-complete. A launcher row must not become a
  game session unless linked child-game proof exists.
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
- Next implementation work should either add live Windows inventory source
  readers, encrypted journal ingest plus service/read-model events, or mirror
  the remaining WP01/WP04 TypeScript shapes into Rust protocol before
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
