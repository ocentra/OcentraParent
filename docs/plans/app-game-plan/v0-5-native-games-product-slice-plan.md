# V0.5 Native Games Product Slice Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `V0.5 Native Games Product Slice Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Native game product meaning is separate from native app product meaning even
though both use the shared app/game evidence spine.

## Scope

Native game scope includes:

- installed native games;
- portable native games;
- game launchers;
- launcher-only sessions;
- launcher-game candidates;
- store game packages;
- native cloud-game clients;
- Roblox, Minecraft, Steam, Epic, Xbox, Riot, Battle.net, EA, Ubisoft, GOG, and
  itch.io sources where available;
- game budgets;
- ratings, age labels, multiplayer, UGC, chat/voice capability, and purchases as
  policy inputs;
- unknown game-like executables.

Browser games and cloud games running inside managed browser tabs remain in the
browser plan.

## Game Categories

Native game category candidates include:

- educational game;
- casual game;
- puzzle game;
- strategy game;
- arcade game;
- shooter game;
- fighting game;
- horror game;
- casino-like game;
- sports game;
- racing game;
- simulation game;
- coding game;
- typing game;
- chess or board game;
- sandbox game;
- role-playing game;
- massively multiplayer game;
- cloud-native game client;
- launcher only;
- unknown game candidate.

## Game Context Signals

Game context can include rating, age label, violence/fear/gambling markers,
online multiplayer, UGC, chat/voice capability, in-app purchase capability,
store/source, and parent label. These are policy inputs and explanation context,
not proof that the game content is safe or unsafe.

## Launcher Rules

Launcher evidence must stay explicit:

- launcher installed is inventory only;
- launcher running is runtime only;
- launcher foreground is foreground launcher use only;
- launcher manifest row can identify a possible owned game;
- child game process proof is required for active known-game state;
- launcher-game candidate can be counted only by parent-configured candidate
  policy.

## Game Policy Targets

Native game policy targets include:

- specific game identity;
- launcher game id;
- store id;
- executable hash;
- package id or AppUserModelId;
- game category;
- unknown games;
- newly installed games;
- launcher-game candidates;
- multiplayer games;
- UGC games;
- purchase-capable games;
- casino-like games;
- mature-rated games;
- all games.

## Game Actions

Native game actions include allow, observe, warn, ask parent, game budget,
bonus time, time limit, terminate running, block launch where proved,
allowlist/school-mode where proved, manual required, and unavailable.

Broad game blocking stays manual-required until the platform adapter proves
authority tier, setup, rollback, and audit state.

## Parent UI Requirements

The parent UI must show:

- games overview;
- installed games;
- running games;
- foreground active game;
- launcher-only rows;
- launcher-game candidates;
- unknown game approval;
- game budgets and schedule outcomes;
- ratings and multiplayer/UGC/purchase signals where sourced;
- evidence drawer;
- capability/platform status;
- manual-required broad blocking labels.

## Done Signal

The native game product slice is credible when a parent can distinguish launcher
presence, launcher use, active known game, possible game, unknown game, game
budget state, and enforcement capability without fake claims.
