<!-- agent-capsule -->

> Agent Capsule
> Doc: App And Game Evidence Sessions Architecture
> Kind: architecture/reference documentation; read only when selected by plan route, source router, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# App And Game Evidence Sessions Architecture

This plan defines the V0.5.2 bridge from low-level process/window capture to
parent-usable native app and game evidence. It is a local device evidence plan,
not an AI scanner and not a cloud activity store.

Native app implementation planning lives in
[Native Apps Plan](../plans/app-plan/README.md). That folder narrows the
non-browser app side of this combined app/game architecture, adds
authority-tier routing for platform control, and keeps this file as the shared
source architecture.

Shared native app/game implementation planning lives in
[App + Game Plan](../plans/app-game-plan/README.md). That folder owns the
combined evidence-spine workpacks, native game slice, app/game test blueprint,
and proof-pack routing while this architecture remains the source contract.

Browser games are covered by browser URL/tab evidence. This document covers
native Windows apps, native games, launchers, Microsoft Store packages, and
known game install/library signals where the child-device agent can observe them
locally.

## Product Bar

Parents need answers to these questions:

- Which apps, launchers, and games are installed or detectable on this device?
- Which apps, launchers, and games are running now?
- Which app or game is foreground-active?
- How long has a known or possible game been running and foreground-active in a
  parent-selected time window?
- Which evidence ids support a summary, decision, or enforcement action?
- Is a process confidently known, possibly a game, unknown, unsupported,
  permission-limited, or stale?

The system must not claim more than the evidence proves. "Steam is running" is
not the same as "a Steam game is running." "A game-like executable is foreground"
is not the same as "this exact catalog title is known" unless a deterministic
match or explicit classifier result supports it.

## Local-First Boundary

Normal custody path:

```text
Windows process/window/install/launcher adapters
  -> typed app/game observations
  -> encrypted local NDJSON journal
  -> local SQLite query/read model
  -> local policy and optional local AI/classifier digest
  -> local/LAN parent portal
```

Remote reports or sync are parent-approved exports. Ocentra-hosted services are
not the default store for app/game sessions, launcher libraries, process
history, parent rules, or generated activity reports.

## Evidence Sources

### Process And Window Evidence

The V0.3 process/window adapter remains the foundation. Windows can enumerate
processes with Tool Help snapshots and observe the foreground window with Win32
window APIs. Relevant Microsoft references:

- [CreateToolhelp32Snapshot](https://learn.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot)
- [Taking a snapshot and viewing processes](https://learn.microsoft.com/en-us/windows/win32/toolhelp/taking-a-snapshot-and-viewing-processes)
- [GetForegroundWindow](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getforegroundwindow)

Required normalized fields:

- `evidenceId`
- `observedAt`
- `sourceId`
- `adapterId`
- `deviceLocalUserRef`
- `processId`
- `parentProcessId`, when available
- `processName`
- `executablePath`, when available and policy permits
- `executableHash`, when available
- `publisherName` and `signatureStatus`, when available
- `windowId`, when available
- `windowTitle`, when available and permitted by the capture setting
- `foregroundState`: `foreground`, `background`, `notWindowed`,
  `unknown`, `unsupported`, `permissionLimited`, `adapterError`
- `observationMode`: `processSnapshot`, `foregroundWindow`,
  `processExit`, `processStart`, `inventoryScan`

Process/window evidence alone can prove that a process is running or foreground.
It cannot prove browser URL, game title, launcher library membership, or content.

### Installed App Inventory

The inventory adapter builds a local installed-app candidate list from
Windows-local sources. It must treat every source as partial:

- uninstall registry entries, Start Menu shortcuts, and known install folders
- Microsoft Store/UWP package inventory using
  [PackageManager.FindPackagesForUser](https://learn.microsoft.com/en-us/uwp/api/windows.management.deployment.packagemanager.findpackagesforuser)
- executable metadata, publisher/signature, and file hash
- known launcher library manifests when present and readable

Inventory output is not a policy decision. It is evidence that can later help
match a process to an app, game, launcher, or unknown executable.

### Launcher Hints

Launcher adapters observe local manifests or library metadata only when present
and readable. They must not read private account tokens, credentials, chat,
purchase history, cloud saves, or launcher network traffic.

Initial launcher candidates:

- Steam
- Epic Games Launcher
- Xbox app / Microsoft Store game packages
- Riot Client
- Battle.net
- EA app
- Ubisoft Connect
- GOG Galaxy
- Roblox
- Minecraft launchers

Steam should be treated as a manifest-backed launcher source, not a privileged
truth source. Valve's Steamworks docs describe build/depot manifests as part of
the Steam application/depot model, but local install evidence still needs local
validation and evidence ids before Ocentra uses it:

- [Steamworks builds and manifests](https://partner.steamgames.com/doc/store/application/builds)
- [Steamworks install scripts](https://partner.steamgames.com/doc/sdk/installscripts?l=english)

Launcher hints must produce typed confidence and provenance:

- `launcherKind`
- `launcherInstallRef`
- `launcherLibraryRef`
- `launcherAppId`, when available
- `launcherTitle`, when available
- `launcherExecutablePathHints`
- `launcherManifestPath`, when safe to record
- `observedAt`
- `capabilityStatus`
- `evidenceIds`

## Classification States

App/game classification is deterministic first. AI/classifier output is allowed
only as a later support layer over stored evidence or agent-generated digests.

Use these states:

- `knownApp`: deterministic app match, not classified as a game.
- `knownGame`: deterministic game match from trusted local evidence.
- `knownLauncher`: launcher process or launcher-owned helper process.
- `launcherGameCandidate`: launcher and process evidence suggest a child game
  process, but title or app id is not proven.
- `possiblyGame`: executable or metadata resembles a game, but no deterministic
  match exists.
- `unknownProcess`: process is observed but not classified.
- `permissionLimited`: adapter cannot read enough metadata.
- `unsupportedPlatform`: platform/source cannot produce this evidence.
- `stale`: last evidence is too old for current-state claims.
- `adapterError`: adapter failed and recorded the error class.

Unknown and possible states are valid evidence. They must not be silently
promoted to known games.

## Session Model

An app/game session is a derived read model backed by raw evidence ids. It is
not portal state and not AI output.

Session identity:

- `sessionId`
- `deviceId`
- `localUserRef`
- `primaryProcessIdentity`
- `classificationState`
- `appOrGameRef`, when known
- `launcherRef`, when relevant
- `startedAt`
- `lastObservedAt`
- `endedAt`, when inferred
- `endReason`: `processExit`, `timeoutInferred`, `deviceShutdown`,
  `agentRestart`, `unknown`

Duration fields:

- `runningDurationMs`
- `foregroundDurationMs`
- `backgroundDurationMs`
- `lastForegroundAt`
- `lastBackgroundAt`
- `observationGapMs`
- `confidence`

Evidence references:

- process observation evidence ids
- foreground window evidence ids
- inventory evidence ids
- launcher manifest evidence ids
- classifier/digest evidence id, only if used
- policy decision id, only after policy evaluation
- enforcement result id, only after enforcement handoff

Sessionization rules:

1. Start a session when a process identity first appears or when a known
   launcher starts a linked child game process.
2. Continue a session while the same process identity or launcher-linked child
   identity remains observed within the configured gap window.
3. Add foreground time only when foreground/window evidence proves active focus.
4. Add running time from process observations, not portal refresh intervals.
5. Close a session on observed process exit or after a configured stale timeout.
6. Reopen a new session after a longer gap unless a launcher-specific adapter
   proves continuity.
7. Keep title changes as evidence, not as separate sessions, unless process or
   launcher identity changes.

## Query Store Read Models

SQLite should expose read models that are cheap for the local/LAN portal and
policy evaluator:

- `app_game_inventory_current`
- `app_game_running_now`
- `app_game_foreground_now`
- `app_game_sessions_recent`
- `app_game_session_daily_rollup`
- `app_game_policy_candidates`
- `app_game_unknown_candidates`
- `app_game_launcher_status`

Every row that supports a parent-visible claim must carry source/custody:

- `localJournal`
- `localSqlite`
- `parentCache`
- `parentOwnedStorage`
- `ocentraNonActivityMetadata`
- `unavailable`
- `stale`

## Policy And Enforcement Handoff

Policy consumes session summaries and evidence refs. It does not scan the OS.

Supported policy targets:

- app id or app ref
- executable identity
- launcher app id
- launcher kind
- game title
- game category
- `knownGame`
- `possiblyGame`
- `unknownProcess`

Initial policy decisions should be dry-run unless enforcement is explicitly
enabled. Enforcement handoff must reference:

- `policyDecisionId`
- `sessionId`
- `targetProcessId`, when current and safe
- `targetExecutablePath`, when available
- `targetLauncherRef`, when relevant
- `action`: `observeOnly`, `askParent`, `blockLaunch`, `terminateProcess`,
  `timeLimitReached`
- `expectedChildMessage`
- `evidenceIds`

Enforcement results:

- `notEnabled`
- `observeOnly`
- `terminated`
- `alreadyExited`
- `permissionLimited`
- `targetChanged`
- `failed`
- `blockedLaunch`
- `askParentCreated`

## AI And Classifier Boundary

AI does not inspect OS state, enumerate processes, read launcher files, count
time, or invent app duration. AI may consume one of these local inputs:

- stored evidence references
- agent-generated app/game session digest
- unknown/ambiguous candidate digest
- parent rule context

AI/classifier output must include:

- source digest id
- evidence ids
- classification candidate
- confidence in `0..1`
- uncertainty reason
- no-action fallback

Policy must remain deterministic over typed inputs. AI may help classify an
unknown candidate, but parent-controlled rules decide what to do.

## Portal Boundary

The parent portal displays local/LAN read models. It does not run app
inventory, process scans, launcher parsing, timers, AI classification, or
enforcement.

Portal views should show:

- installed/detectable app and game inventory
- running now
- foreground now
- recent sessions
- time-window rollups
- unknown and possibly-game candidates
- launcher integration status
- evidence id and source/custody labels
- policy dry-run result
- enforcement result when enabled

When evidence is unavailable, stale, unsupported, or permission-limited, the
portal must show that state rather than hiding it.

## Implementation Phases

Phase 1: Contract and read-model plan

- Add TypeScript contracts for inventory, process app/game observation,
  foreground session evidence, session summary, launcher hints, classification
  state, and evidence refs.
- Add Rust protocol shape only after TypeScript contracts are explicit.
- Add parser/schema tests with invalid states and confidence ranges.

Phase 2: Windows local adapters

- Reuse process and foreground-window observations.
- Add inventory adapter for registry/shortcut/package sources.
- Add launcher manifest adapters one launcher at a time, starting with Steam or
  Microsoft Store/Xbox only if a real local fixture and manual validation path
  are available.
- Record capability status for every adapter.

Phase 3: Journal and SQLite ingest

- Write raw observations to encrypted NDJSON journal.
- Ingest raw observations and derived summaries into SQLite.
- Rebuild summaries from journal replay.
- Add duplicate and stale observation handling.

Phase 4: Portal visibility

- Add local read-model query.
- Show inventory, running, foreground, session rollups, unknown candidates, and
  source/custody labels.
- Keep the existing one-panel portal behavior rather than adding endless result
  boxes.

Phase 5: Policy dry-run

- Feed session summaries into parent rules.
- Produce dry-run decisions for app/game limits before enforcement.
- Add child-facing message contract without terminating anything.

Phase 6: Enforcement handoff

- Add observe-only, ask-parent, block launch, and terminate handoff contracts.
- Keep actual process termination behind explicit parent settings and platform
  capability checks.

## Validation Plan

Contract tests:

- Decode valid inventory, launcher hint, session summary, policy target, and
  enforcement handoff examples.
- Reject invalid confidence values outside `0..1`.
- Reject impossible duration states such as foreground time greater than running
  time.
- Preserve unknown/possibly-game states without forcing a known title.

Rust tests:

- Map process/window observations to app/game candidates.
- Derive foreground and running duration from ordered observations.
- Preserve degraded adapter statuses.
- Rebuild session summaries from journal replay and SQLite ingest.

Integration smoke:

- Service remains responsive while app/game observation runs.
- Local query returns recent sessions and unknown candidates.
- Portal displays evidence source/custody and stale/unavailable states.

Manual Windows validation:

- Observe Notepad or another known app as running and foreground.
- Observe a supported launcher as running without claiming a child game.
- Observe a launcher-started game only when local process/launcher evidence
  supports the distinction.
- Confirm no browser URL, screenshots, keystrokes, chat text, launcher tokens,
  or decrypted network payloads are recorded.

## Done Signal

V0.5.2 is done when the repo has typed contracts, local adapter boundaries,
journal/query-store flow, and portal/policy inputs that can represent native
app/game sessions with running time, foreground time, known/unknown states, and
evidence refs without relying on AI to scan the machine or Ocentra cloud storage
to hold child activity.
