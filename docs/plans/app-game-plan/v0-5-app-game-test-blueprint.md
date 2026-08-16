# V0.5 App + Game Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `V0.5 App + Game Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Core Proof Rule

Every product claim needs a test or proof artifact. Every unsupported,
manual-required, unavailable, and not-claimed state needs a visible negative
test proving it does not execute.

## Required Test Layers

- Unit tests for contract parsers and derived state.
- Integration tests for adapters, journal, SQLite replay, policy compiler, and
  service events.
- Contract tests for TypeScript/Rust parity.
- Security-negative tests for no-claim boundaries and metadata safety.
- E2E tests for real service paths where runtime behavior exists.
- Playwright tests for parent portal UI states.
- Manual platform proof for authority-tier claims.
- Performance tests for inventory/session scale.

## Core Invariants

Evidence invariants:

- inventory cannot set running or foreground;
- running cannot set foreground;
- foreground cannot set content;
- launcher cannot become game without child-game proof;
- display name alone is weak identity;
- AI classification cannot set action authority.

Session invariants:

- foreground duration never exceeds running duration;
- replay reconstructs the same sessions;
- stale gaps are explicit;
- reused process ids do not corrupt old sessions;
- launcher-only sessions remain launcher-only.

Policy invariants:

- parent actor, target device, child profile, rule version, schedule, evidence
  refs, and capability state are validated before action;
- game budgets consume game sessions or explicit launcher-game candidate policy;
- app rules consume app identities/categories with source confidence;
- unknown app/game stays unknown until evidence/classification improves.

Enforcement invariants:

- dry-run never executes;
- manual-required never executes;
- unavailable never executes;
- adapter actions require authority-tier proof;
- rollback and audit refs are required for strong actions.

## Minimum Serious MVP Test Set

Unit:

- app/game identity;
- inventory evidence;
- runtime evidence;
- foreground evidence;
- launcher evidence;
- session model;
- app category taxonomy;
- game category taxonomy;
- policy target compiler;
- authority tier;
- AI no-direct-enforcement.

Integration:

- Windows inventory fixtures;
- Windows process fixtures;
- Windows foreground fixtures;
- launcher-only and launcher-game candidate fixtures;
- identity merge;
- sessionization;
- journal/SQLite replay;
- policy dry-run;
- unknown app approval;
- unknown game approval;
- risk app detection;
- game budget dry-run.

Contract:

- `AppGameIdentity`;
- `AppGameInventoryEvidence`;
- `AppGameRuntimeEvidence`;
- `AppGameForegroundEvidence`;
- `AppGameLauncherEvidence`;
- `AppGameSessionSummary`;
- `AppGamePolicyDecision`;
- `AppGameEnforcementResult`;
- `AppGameApprovalRequest`;
- `AppGameCapabilityStatus`.

Security:

- weak evidence no-upgrade;
- manual-required guard;
- platform authority guard;
- path redaction;
- malicious metadata escaping;
- stale evidence rejection;
- launcher-is-not-game;
- AI-is-not-authority.

E2E:

- Windows app/game inventory to portal;
- Windows runtime session;
- foreground duration;
- unknown app approval;
- unknown game approval;
- launcher not game;
- launcher game candidate;
- risk app detection;
- time-limit dry-run;
- owned-process enforcement where already scoped;
- broad block manual-required.

Playwright:

- app/game dashboard;
- inventory details;
- running/foreground states;
- launcher UI;
- evidence drawer;
- unknown approval;
- risk categories;
- game budgets;
- policy authoring;
- platform matrix;
- manual-required labels;
- malicious/long metadata;
- narrow viewport.

## Required Fixtures

Fixture roots should include:

```text
fixtures/app-game/inventory/
fixtures/app-game/runtime/
fixtures/app-game/launchers/
fixtures/app-game/sessions/
fixtures/app-game/policy/
fixtures/app-game/ui/
```

Initial fixture names:

- `windows-registry.json`
- `windows-store.json`
- `macos-bundles.json`
- `linux-desktop-entries.json`
- `android-packages.json`
- `ios-tokens.json`
- `process-snapshot.json`
- `foreground-window.json`
- `android-usage-events.json`
- `ios-device-activity.json`
- `steam-launcher-only.json`
- `steam-child-game-candidate.json`
- `epic-launcher-only.json`
- `xbox-store-game.json`
- `roblox-native.json`
- `minecraft-launcher.json`
- `app-foreground-session.json`
- `game-session.json`
- `launcher-only-session.json`
- `stale-gap-session.json`
- `app-observe.json`
- `app-unknown-ask.json`
- `game-budget-dry-run.json`
- `launcher-only-observe.json`
- `broad-block-manual-required.json`
- `dashboard-mixed.json`
- `unknown-approval.json`
- `launcher-status.json`
- `manual-required.json`
- `malicious-metadata.json`

## CI Gates

Use repo-native commands, not pasted package-manager names:

```powershell
npm run lanes:guard
npm run hub:guard
npm run lint:schema-boundaries
npm run test --workspace @ocentra-parent/activity-domain
npm run test --workspace @ocentra-parent/parent-domain
cargo test -p ocentra-parent-agent-protocol
cargo test -p ocentra-parent-agent-core
npm run test --workspace @ocentra-parent/portal
npm run test:e2e --workspace @ocentra-parent/portal
npm run format:check
git diff --check
```

Run `npm run validate` before final handoff unless the user explicitly asks for
a narrower proof.

## Required Proof Pack

The required proof pack is defined in
[implementation checklist](implementation-checklist.md). Missing proof must be
written as manual-required/not-applicable with reason.
