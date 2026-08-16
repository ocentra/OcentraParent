# V0.5 App + Game Shared Evidence Spine Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `V0.5 App + Game Shared Evidence Spine Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

## Product Rule

Apps and games share evidence. Apps and games do not share product meaning.

Inventory proves presence. Runtime proves running. Foreground proves active
focus. Session proves duration. Launcher evidence helps identify games. AI may
classify unknown evidence. Parent policy decides action. Adapter proof decides
enforcement.

## Shared Evidence Sources

### Inventory Evidence

Inventory evidence may come from:

- Windows uninstall registry;
- Windows Start Menu shortcuts;
- Windows Store/UWP/AppX/MSIX packages;
- known install locations;
- executable metadata;
- publisher/signature/hash;
- macOS `.app` bundles;
- macOS LaunchServices/Spotlight metadata;
- Linux desktop entries;
- Linux package managers;
- Flatpak;
- Snap;
- AppImage or portable executable scans in bounded known paths;
- Android package manager and managed-profile inventory;
- iOS FamilyControls tokens and MDM installed-app query;
- parent catalog and manual labels.

Inventory means the app/game appears installed or detectable. Inventory must not
set running, foreground, duration, child-use, or enforcement state.

### Runtime Evidence

Runtime evidence may come from:

- process snapshot;
- process start;
- process exit;
- foreground window;
- active app notification where the platform provides it;
- Android UsageStats;
- Android UsageEvents;
- Android Accessibility state if enabled;
- iOS DeviceActivity;
- managed-device app activity events.

Runtime means a process, package, or app token was observed. Runtime must not
claim foreground unless foreground evidence exists.

### Foreground Evidence

Foreground evidence proves active focus. It may include a process/app identity,
timestamp, permitted title/ref, and permission-limited state. It must not claim
screen content, chat content, video/game content, or user intent.

### Launcher Evidence

Launcher evidence may come from Steam, Epic Games Launcher, Xbox app,
Microsoft Store game packages, Riot, Battle.net, EA app, Ubisoft Connect, GOG
Galaxy, Roblox, Minecraft Launcher, itch.io, native cloud-game clients, and
manual parent catalog mappings.

Launcher evidence answers:

- Is only the launcher installed?
- Is only the launcher running?
- Is a child game process linked?
- Is a specific app/game id present in a local manifest?
- Is this a launcher-game candidate?
- Is this still only launcher UI?

Launcher evidence must not become an active game session without child-game
proof.

## Shared Contract Families

The shared contract stack should cover:

- `AppGameIdentity`
- `AppGameInventoryEvidence`
- `AppGameRuntimeEvidence`
- `AppGameForegroundEvidence`
- `AppGameLauncherEvidence`
- `AppGameSessionSummary`
- `AppGameCategoryCandidate`
- `AppGameRiskCandidate`
- `AppGamePolicyTarget`
- `AppGameApprovalRequest`
- `AppGameCapabilityStatus`
- `AppGameAuthorityTier`
- `AppGamePolicyDecision`
- `AppGameEnforcementAction`
- `AppGameEnforcementResult`
- `AppGameAiClassificationDigest`

Contracts must include schema version, evidence refs, observed timestamps,
source ids, adapter ids, confidence, stale/degraded states, custody, redaction,
proof refs, and capability status.

## Identity Model

Identity can include:

- platform;
- kind: native app, native game, launcher, browser handoff, system app, unknown;
- package id;
- bundle id;
- AppUserModelId;
- desktop entry id;
- application token ref;
- executable path ref;
- publisher/signature ref;
- file hash ref;
- launcher app id;
- launcher manifest id;
- store id;
- parent label;
- display name;
- identity confidence.

Display name alone is weak identity and must not deterministically merge app or
game rows.

## Session Rules

- Running duration derives from runtime evidence.
- Foreground duration derives only from foreground evidence.
- Background duration derives from running minus foreground where valid.
- Foreground duration must not exceed running duration.
- Sessions replay deterministically from journal/SQLite evidence.
- A launcher-only session is not an active game session.
- A launcher-game candidate can count toward a game budget only when a
  parent-authored policy permits that candidate state.

## Policy And Enforcement Flow

```text
stored evidence refs
  -> session summary
  -> optional classifier digest
  -> deterministic policy target compiler
  -> parent policy decision
  -> authority-tier and capability check
  -> dry-run result
  -> adapter action only where proved
  -> enforcement result and audit refs
```

Manual-required, not-claimed, unavailable, and dry-run states must never execute
adapters.

## Done Signal

The shared spine is credible when app and game inventory, runtime, foreground,
session, policy, authority, and enforcement proof all use one evidence model,
while app-specific and game-specific product rows remain visibly distinct.
