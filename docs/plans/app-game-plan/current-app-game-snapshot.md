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
- Product-complete native game catalog, launcher disambiguation, and game
  budgets.
- New/unknown app and unknown game approval flow.
- Risk app and game-risk candidates with source/confidence.
- Broad app/game blocking outside scoped owned-process proof.
- macOS, Linux, Android, iOS, MDM, supervised/device-owner, Endpoint Security,
  AppLocker/App Control, Screen Time, ManagedSettings, store/signing, entitlement,
  kiosk, and single-app claims.

## Current Contracts

`packages/activity-domain` currently provides combined app/game primitives,
identity and identity-merge proof, inventory evidence rows, inventory entry,
process observation, evidence claim, session summary/query/report, AI digest
reference, and AI classification digest contracts. These support useful
evidence and session proof, but they do not yet cover the complete shared
app/game contract stack from this plan:

- launcher evidence and launcher-only state beyond the initial launcher
  no-claim guards;
- runtime-fed native app and native game category quality beyond inventory
  candidates;
- rating, UGC, multiplayer, and purchase signals;
- policy targets for app and game product slices;
- unknown app and unknown game approval requests;
- authority tier and capability rows for every action;
- enforcement result and rollback proof for app/game actions.

`packages/parent-domain` currently provides separate app-control and
game-control catalog schemas/data, enforcement policy dispatch contracts, and
app/game control approval authority/request/decision/action-result contract
proof. Those catalogs and authority contracts are product-control inputs, not a
complete shared evidence spine or a proof that broad app/game blocking works.

## Current Runtime

`crates/agent-core` currently has SQLite-backed app/game observation and session
helpers, typed Windows installed-record and Store/UWP package
adapter/parser proof, and scoped Windows owned-process time-limit helpers. This
is a strong base for workpacks 08, 12, 13, and 21, but it is not the same as:

- live Windows installed app inventory crawling;
- live UWP/AppX package inventory enumeration;
- launcher manifest adapters;
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
- risk app and risk game candidates;
- platform capability matrix with manual-required proof;
- malicious/long metadata resilience screenshots.

## Current Gaps

- App/game identity contracts are present, but runtime identity merge behavior
  and adapter-fed identity refs are not implemented yet.
- Inventory evidence row contracts and Rust inventory-row parity are present,
  and Windows installed-record plus Store/UWP package parser proof exists, but
  live platform crawling, journal ingest, read models, and portal rows are not
  implemented yet.
- Rust protocol parity has not yet mirrored the WP01 evidence claim, AI digest,
  app/game control authority schemas, or WP04 identity schemas.
- Journal and SQLite ingest do not yet store the new evidence claim and
  authority proof shapes.
- Portal app/game dashboard rows do not yet consume the new contracts.
- Launcher evidence is not product-complete. A launcher row must not become a
  game session unless linked child-game proof exists.
- Game categories, ratings, UGC, multiplayer, and purchase signals are policy
  inputs and parent-facing context, not safety decisions by themselves.
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
- Next implementation work should either add live Windows inventory source
  readers, runtime/foreground adapters, or mirror the remaining WP01/WP04
  TypeScript shapes into Rust protocol before service/runtime consumers depend
  on them.

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
