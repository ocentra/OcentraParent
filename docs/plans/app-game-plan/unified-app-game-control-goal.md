# Unified Native App/Game Control Goal

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `Unified Native App/Game Control Goal`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

## Short Goal Pointer

Read and follow `docs/plans/app-game-plan/unified-app-game-control-goal.md`.
Finish the unified native app plus native game control scope without chasing
micro PR churn or routine sync noise. Sync with `main` only when it is actually
useful for this full product-completion branch.

## Main Goal

Finish the Ocentra Parent native app plus native game scope from the `app-plan`
and `app-game-plan` lanes as one unified product-control capability.

- Keep one shared low-level evidence and control spine for native apps and
  native games.
- Treat native games as apps at the operating-system and control layer.
- Keep native apps and native games as separate parent-facing product meanings
  only where behavior, policy, budgets, labels, or approval meaning differs.
- Do not duplicate browser-game work. Browser games and cloud games stay in
  `browser-plan`.
- Build real implementation slices, not claims.
- Every meaningful slice needs proof, tests or validation, docs/checklist
  decisions, hub reporting, and a commit/push when it is in a useful validated
  state and coordination rules allow it.

## Product Model

Native games are specialized native apps.

The shared spine owns:

- identity and executable/package identity;
- installed inventory;
- runtime/process evidence;
- foreground evidence;
- launcher evidence;
- sessionization and duration;
- journal and SQLite ingest;
- read models and service events;
- policy target compilation;
- budget, schedule, bonus-time, and enforcement readiness;
- notification and local outbox handoff;
- parent and child warning/block/request flows;
- platform authority and proof routing.

Native game meaning adds:

- launcher-backed game candidate state;
- game category and risk context;
- game budgets and launcher policy;
- game-specific parent labels and child UX;
- purchase/store handoff meaning;
- native-game proof rows and platform limitations.

Do not create separate duplicate app-control and game-control systems unless a
real platform or product boundary requires it.

## App Plan Scope

- App identity model.
- Installed app inventory model.
- Windows installed app inventory adapter.
- Windows Store/UWP/AppX inventory adapter.
- Windows process runtime evidence.
- Windows foreground app evidence.
- Cross-platform authority matrix.
- App category and risk taxonomy.
- App sessionization and duration.
- Journal and SQLite app ingest.
- App read models and service events.
- Parent portal app inventory/running/session surfaces.
- New/unknown app approval flow.
- Risk app detection.
- Policy target compiler for app rules.
- Time budget, schedule, and bonus-time integration.
- Child-facing warning/block/request UX.
- Windows owned-process terminate/time-limit proof.
- Broad blocking proof gates.
- App AI classifier digest boundary.
- Platform extension checklist/proof routing.
- Install/uninstall approval handoff.
- Performance/service health.
- E2E/manual proof artifacts.
- Rollout checklist and PR gate.
- Later backend/read-model continuation work: Rust protocol parity, authority
  storage, live process/foreground/inventory sources, service capture bridges,
  category/risk routing, notification intent, local outbox bridge.

## App Plus Game Plan Scope

- Shared app/game contract boundary.
- Source index/doc reconciliation.
- Current app/game snapshot and gap map.
- App/game identity model.
- Inventory evidence model.
- Windows installed app/game inventory.
- Windows Store/UWP/AppX/MSIX inventory.
- Windows process runtime evidence.
- Windows foreground app/game evidence.
- Launcher evidence and game candidate model.
- Cross-platform authority matrix.
- App and game category/risk taxonomy.
- Sessionization and duration engine.
- Journal and SQLite ingest.
- Read models and service events.
- Parent portal app/game dashboard surfaces.
- Unknown app/game approval flow.
- Native game budgets and launcher policy.
- Policy target compiler for app/game rules.
- Time budget, schedule, and bonus-time integration.
- Child-facing app/game warning and request UX.
- Windows owned-process terminate/time-limit proof.
- Broad blocking proof gates.
- AI classifier digest boundary.
- Platform extension checklist/proof routing.
- Install, uninstall, purchase, and store handoffs.
- Performance/service health.
- E2E/manual proof/rollout/PR gate.
- Rust protocol parity.
- Journal/SQLite authority classifier storage.
- Live process, foreground, Windows inventory, Windows Store, and Windows
  registry sources.
- Service capture bridges.
- Backend source freshness rows.
- Category/risk policy routing.
- Policy readiness service read model.
- Notification intent contract.
- Notification local outbox bridge.

## Implementation Direction

Work toward one unified app/game target service path that can carry all of
these cases through the same typed spine:

- normal native app;
- native game;
- launcher-backed game candidate;
- unknown app;
- unknown game-like executable;
- unsupported or manual-required platform row;
- report-only capability row;
- control-capable Windows owned-process time-limit row;
- future platform adapter rows only after proof exists.

The next meaningful implementation slices should prefer real runtime and UI
coverage over more proof-only documents:

- service/runtime command and read API wiring;
- parent-domain contracts and guards;
- journal/SQLite ingest and replay wiring;
- parent portal dashboard/read-model surface;
- child warning/request/outbox flow;
- category/risk policy routing;
- local notification/outbox bridge;
- platform support matrix and honest manual-required states.

## Anti-Churn Rule

Do not stop just because one checkpoint is done.

Do not chase old micro PR repairs, branch restacks, or routine main-advanced
messages unless they are genuinely needed for the active full-scope branch.
Routine sync is not the goal. Product completion is the goal.

Sync with `main` only when one of these is true:

- starting or recreating the full product-completion branch;
- before final validation for a useful pushed state;
- before pushing a branch intended for review;
- a real dependency from `main` is needed for current implementation;
- the active branch is blocked by a concrete conflict or failing validation
  caused by drift.

When syncing is needed:

- preserve local work first;
- fetch/rebase/merge latest `main` in the worker branch;
- resolve conflicts in the worker lane;
- rerun focused validation;
- continue the same full-scope goal.

## Coordination Rules

- Do not open PRs unless the user or primary explicitly asks after a useful
  `DONE` or `PR_READY` state.
- Do not merge.
- Do not push to `main`.
- Keep branch names and lane reports accurate.
- Do not block other lanes longer than needed.
- Do not create duplicate truth or duplicate code paths.
- If another lane owns a piece, coordinate and sequence it, but do not use that
  as an excuse to abandon the full goal.
- Keep hub informed with meaningful progress, blockers, or done states. Do not
  spam hub with routine idle or sync-only noise.

## Completion Standard

End state is real product capability, not hand-waving.

For each completed slice, provide:

- real implementation, not only docs or proof;
- contract-backed source changes;
- focused tests or proof scripts;
- validation command results;
- docs/features and checklist updates when status, proof, or gaps changed;
- exact branch and commit state;
- pushed branch only when it is in a useful validated state;
- known gaps and next product slice.

Full native app/game completion requires:

- shared app/game identity, inventory, runtime, foreground, launcher, session,
  and duration path;
- journal/SQLite ingest and replay;
- service read models and events;
- parent portal app/game dashboard surfaces;
- unknown approval and manual-required states;
- category/risk routing;
- policy compiler and budget/schedule/bonus-time integration;
- child warning/block/request UX contracts and local handoff;
- notification intent and local outbox bridge;
- Windows owned-process time-limit proof preserved;
- broad blocking and platform support represented honestly as supported,
  manual-required, unavailable, or not-claimed;
- rollout/checklist proof and E2E/manual artifacts.

## Recommended Working Branch

Use one full-scope branch for the continuation unless a true coordination
reason requires a split:

`codex/app-game-control-product-completion`

Prefer larger meaningful product slices over tiny proof-only branches. Commit
and push only at useful validated checkpoints.
