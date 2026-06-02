# Current App + Game Snapshot - 2026-06-02

## Product Claim Boundary

Current merged source proves a scoped app/game evidence and enforcement path. It
does not prove a product-complete app/game subsystem.

Proved today:

- App/game session contracts and read-model proof exist.
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
inventory entry, process observation, session summary/query/report, and AI
digest reference contracts. These support useful evidence and session proof, but
they do not yet cover the complete shared app/game contract stack from this
plan:

- layered app/game identity;
- platform-specific inventory source type;
- launcher evidence and launcher-only state;
- native app categories;
- native game categories, rating, UGC, multiplayer, and purchase signals;
- policy targets for app and game product slices;
- unknown app and unknown game approval requests;
- authority tier and capability rows for every action;
- enforcement result and rollback proof for app/game actions.

`packages/parent-domain` currently provides separate app-control and
game-control catalog schemas/data plus enforcement policy dispatch contracts.
Those catalogs are product-control inputs, not a complete shared evidence spine
or a proof that broad app/game blocking works.

## Current Runtime

`crates/agent-core` currently has SQLite-backed app/game observation and session
helpers plus scoped Windows owned-process time-limit helpers. This is a strong
base for workpacks 08, 12, 13, and 21, but it is not the same as:

- installed app inventory adapters;
- UWP/AppX inventory adapters;
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

- The current docs split native app planning and browser-game planning but do
  not yet give native games a separate product slice inside a shared app/game
  implementation plan.
- App/game contract names are combined, but several enum values and proof gates
  still need explicit game-specific meaning.
- Launcher evidence is not product-complete. A launcher row must not become a
  game session unless linked child-game proof exists.
- Game categories, ratings, UGC, multiplayer, and purchase signals are policy
  inputs and parent-facing context, not safety decisions by themselves.
- Existing scoped Windows owned-process time-limit proof should be reused, not
  expanded into broad blocking claims.

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
