# App + Game UI/UX Requirements Guide

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game UI/UX Requirements Guide`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## UI Rule

The portal must explain evidence strength, capability state, and product meaning
without implying unsupported runtime or enforcement behavior.

## Parent Outcomes

Parents should be able to see:

- which apps and games are installed or detectable;
- which apps and games are running now;
- which app or game was foreground/active and when;
- running and foreground duration;
- launcher-only versus active-game state;
- launcher-game candidates;
- unknown/new apps and unknown/new games;
- risk app candidates and game-risk candidates;
- app categories and game categories with source/confidence;
- ratings, multiplayer, UGC, and purchase signals where sourced;
- policy and schedule outcomes;
- app/game control authority tier;
- which actions are available now;
- which actions require setup, enrollment, admin/root, system extension,
  supervision, MDM, device-owner, entitlement, or manual proof;
- evidence and audit refs for every claim.

## Main Surfaces

- App/game overview.
- Installed apps.
- Installed games.
- Running now.
- Foreground now.
- Recent app/game sessions.
- Daily rollups.
- Launchers and launcher-game candidates.
- New/unknown apps and games.
- Risk apps and game-risk candidates.
- Approval requests.
- App rules.
- Game budgets.
- Capability/platform matrix.
- Evidence details.
- Audit timeline.

## Row/Card Requirements

Every app/game row should include:

- display label;
- product kind: app, game, launcher, candidate, unknown, system, browser handoff;
- category and source/confidence;
- inventory state;
- running state;
- foreground state;
- duration;
- policy state;
- capability/authority state;
- evidence source label;
- freshness/last observed;
- next action or manual-required reason.

## Launcher UI

Launcher rows must distinguish:

- launcher installed;
- launcher running;
- launcher foreground;
- launcher-game candidate;
- known game child process;
- unknown game-like executable.

The UI must explain why launcher-only evidence is not the same as active game
play.

## Approval UI

Approval states must cover:

- unknown app approval;
- unknown game approval;
- new app detected;
- new game detected;
- portable executable;
- launcher-game candidate;
- installer/updater;
- risk candidate;
- approval granted once;
- approval granted for schedule;
- denied;
- expired;
- manual-required block.

## Capability Labels

Use explicit capability labels:

- Can observe.
- Can warn.
- Can ask parent.
- Can count running time.
- Can count foreground time.
- Can time-limit.
- Can terminate running process.
- Can shield.
- Can hide/suspend.
- Can block launch.
- Block launch manual-required.
- Permission required.
- Device owner required.
- MDM required.
- Supervision required.
- System extension required.
- Admin/root service required.
- Entitlement/signing required.
- Not claimed.

## Evidence Detail UI

Evidence details may show:

- source type;
- observed time;
- freshness;
- custody;
- identity fields used;
- confidence;
- reason codes;
- redacted path/hash/signature refs;
- policy decision refs;
- action/audit refs.

Evidence details must not show:

- raw command lines with secrets;
- app internal documents;
- chat/message content;
- keystrokes;
- screenshots unless screen-evidence scope explicitly owns them;
- launcher credentials or tokens;
- decrypted network payloads.

## Snapshot States

Required UI proof states include:

- normal inventory;
- running now;
- foreground now;
- ended session;
- launcher only;
- launcher-game candidate;
- known game;
- unknown/new app;
- unknown/new game;
- risk app;
- game-risk candidate;
- approval request;
- policy preview;
- warn/ask/time-limit;
- action result;
- stale;
- degraded;
- permission-required;
- manual-required;
- unsupported or not-claimed;
- malicious/long values;
- narrow viewport.
