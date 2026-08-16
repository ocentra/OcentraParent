# V0.5 Managed Browser Test Blueprint

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `V0.5 Managed Browser Test Blueprint`
> Kind: test blueprint reference; read only when local expectations route here.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This is the companion requirement blueprint for the
[V0.5 Managed Browser Full Scope Plan](v0-5-managed-browser-full-scope-plan.md).
The plan defines what to build. This blueprint defines the tests, fixtures,
proof gates, and quality bars required while building it.

## Scope

Included:

- Browser inventory.
- Managed browser profiles.
- Managed Chrome, Edge, and Chrome for Testing launch.
- Chromium DevTools Protocol bridge custody.
- Exact URL/title/tab evidence.
- Active-tab proof boundaries.
- Unmanaged browser detection.
- Browser policy/action compilation.
- URL/page/video intelligence classification from browser evidence.
- Local AI evidence result and deterministic policy handoff.
- Social platform route, account creation, account switch, feed/reel/short, and
  approval-gate detection from browser evidence.
- Browser-game portal, WebGL/canvas, cloud-gaming, unblocked-site, game
  account/purchase, and educational-game gating from browser evidence.
- Managed warning/block delivery.
- Journal and SQLite persistence.
- Portal read models and UI.
- Security and tamper boundaries.
- Cross-platform manual proof states.

Excluded until separately proved:

- Exact unmanaged browser URL evidence.
- Exact active-tab claims from `/json/list` alone.
- Page body capture, screenshots, keystrokes, browser secrets, decrypted HTTPS,
  cookies, tokens, local storage, or form values.
- General OS browser blocking without AppLocker/App Control or platform proof.
- Android/iOS/Safari desktop-CDP-style control.
- Extension/native-host security outside a managed profile.
- Video semantic analysis from URL metadata alone.
- AI direct enforcement or remote/API AI as a default child-safety path.
- Private message capture, native-app per-reel blocking, account identity proof,
  or all-platform social account detection without separate proof.
- Exact cloud-streamed game content, game chat content, native-game scene
  control, or safe-game claims from canvas/WebGL/domain/title alone.

## Core Test Principles

### BROWSER-TEST-001: Managed exact evidence only

Requirement: Exact URL/title/tab evidence requires a current managed browser
session, managed profile id, source id, adapter id, timestamps, custody label,
and journaled storage path.

Proof: Contract, mapper, journal, service, and portal tests reject exact URL
evidence without those fields.

Acceptance: Parent UI can show exact URL only from managed, journaled browser
evidence.

### BROWSER-TEST-002: Target list is not active tab proof

Requirement: `/json/list` page targets prove visible tab targets, not active
tab focus.

Proof: Unit/integration tests map target-list evidence to `unknown` active
state unless a separate proof source is present.

Acceptance: UI says active tab is unknown when only target-list proof exists.

### BROWSER-TEST-003: Unmanaged evidence has no exact URL

Requirement: Browser-like processes outside the managed boundary can only emit
bypass/process evidence.

Proof: Contract/security tests reject unmanaged evidence containing exact URL,
tab id, managed active-tab state, or page title unless a later approved
milestone adds a separate allowed field.

Acceptance: Unmanaged Chrome/Firefox/portable browser is visible as possible
bypass and exact URL unavailable.

### BROWSER-TEST-004: Bridge custody is enforced

Requirement: The agent consumes only Ocentra-launched loopback bridge endpoints
for the current managed session.

Proof: Security tests reject wrong port, non-loopback host, default profile,
wrong profile, wrong process, stale session id, and raw debugger URL leakage.

Acceptance: The service cannot attach to a random personal browser debugging
port.

### BROWSER-TEST-005: Journal before portal

Requirement: Browser evidence is journaled and query-store replayable before
portal, policy, or AI consumers see it.

Proof: Persistence tests prove journal write, SQLite replay, stable evidence
ids, stale/degraded replay, and read models sourced from storage.

Acceptance: Portal cannot directly consume CDP payloads.

### BROWSER-TEST-006: Policy actions require capability proof

Requirement: Observe and dry-run do not execute adapters. Warn/block/terminate
or OS block actions require policy decision refs, evidence refs, target refs,
audit refs, and adapter capability proof.

Proof: Policy compile/service tests reject unsupported or manual-required
actions and fail if dry-run executes an adapter.

Acceptance: Parent UI sees what will happen and why an action is unavailable.

### BROWSER-TEST-007: AI is evidence, not enforcement

Requirement: URL/video AI output can recommend policy input only. It cannot
emit final enforcement actions, bypass parent policy, or execute an adapter.

Proof: Schema, policy integration, invalid-output, provider-routing, and audit
tests reject direct AI enforcement, remote-by-default model routes, memory hits
without source refs, and hidden analysis that uses child cookies.

Acceptance: Parent UI can show AI classification, confidence, model/provider,
evidence refs, policy rule, final decision, and action as separate fields.

### BROWSER-TEST-008: Social gates are first-class policy targets

Requirement: Social account creation, login, account switch, feed, reels,
shorts, livestream, messaging route, upload/post, and unknown social site states
must compile through typed policy/approval contracts. They must not be hidden as
generic browser blocks or generic AI flags.

Proof: Contract, pattern library, managed DOM/form, policy compile, approval,
intervention, unmanaged bypass, and Playwright tests cover known signup routes,
generic signup forms, secondary account signals, feed/short-video routes, and
manual-required native/platform states.

Acceptance: Parent UI can see the social target, evidence type, confidence,
approval state, policy decision, adapter capability, child delivery state, and
audit refs.

### BROWSER-TEST-009: Browser games are not generic websites

Requirement: Browser game portals, WebGL/canvas games, cloud gaming, unblocked
game sites, game account/purchase routes, educational games, and unknown games
must be typed policy targets where evidence supports them. Native games remain
app/game-control evidence.

Proof: URL-shape, runtime-signal, metadata, hidden-analysis safety, AI handoff,
policy compile, approval/time-budget, managed intervention, unmanaged bypass,
and Playwright tests cover game portals, cloud-gaming routes, WebGL/canvas
signals, purchase routes, unblocked portals, and educational-game claims.

Acceptance: Parent UI can see game evidence, runtime signals, classification,
rule, time budget/approval state, action, audit refs, and manual-required gaps.

## Required Test Layers

- Unit tests.
- Integration tests.
- Contract tests.
- Adapter fixture tests.
- Security tests.
- Persistence tests.
- E2E tests.
- Playwright UI tests.
- Manual browser validation.
- Performance tests.
- CI proof gates.

Default CI must not require a real installed browser unless the test is
explicitly marked manual or platform-specific.

## Required Fixture Families

- Browser inventory fixtures for Windows, macOS, Linux, Android, iOS, packaged,
  portable, block-only, and candidate browsers.
- CDP fixtures for Chrome/Edge `/json/version`, `/json/list`, multiple tabs,
  internal pages, missing URL, malformed payload, oversized target lists, and
  bad UTF-8.
- Managed session fixtures for launch pending, running managed, bridge
  connected, bridge disconnected, stale, adapter error, wrong profile, and wrong
  port.
- Evidence fixtures for managed tab evidence, active unknown, known active,
  stale, unmanaged Chrome, unmanaged Firefox, portable browser, and rejected
  unmanaged-with-URL.
- Policy fixtures for exact URL observe, dry-run block, manual-required block,
  domain rule, classifier missing, unmanaged report, unmanaged terminate, and
  unmanaged OS block manual-required.
- Browser intelligence fixtures for YouTube video, YouTube Shorts, YouTube
  homepage/feed, search results, login-required page, dynamic social feed,
  livestream, comments-heavy metadata, multilingual content, stale memory hit,
  invalid AI output, local-provider unavailable, and parent-approved remote
  disabled.
- Social platform fixtures for Facebook signup, Instagram signup, TikTok signup,
  Snapchat signup, Discord register, Reddit register, X/Twitter signup, Twitch
  signup, account switch, unknown social signup, feed, reels, shorts, livestream,
  messaging route, upload/post, fake-platform domain, and unmanaged social
  bypass.
- Browser-game fixtures for Roblox web game, now.gg, Xbox Cloud Gaming, GeForce
  NOW, Amazon Luna, Boosteroid, itch.io, CrazyGames, Poki, Coolmath Games,
  Miniclip, Kongregate-style portal, unblocked games portal, WebGL game,
  canvas-only page, iframe embedded game, game purchase route, loot-box route,
  cloud game with unknown title, educational game, fake educational label, and
  unmanaged browser game bypass.
- Portal fixtures for empty inventory, mixed inventory, managed session,
  degraded session, recent tab evidence, stale tab evidence, unmanaged bypass,
  and browser policy preview.
- Malicious fixtures for script titles, oversized URL/title, punycode, invalid
  URL, JavaScript/data schemes, oversized CDP payloads, path traversal profile,
  and debugger URL leak.

## Proof Matrix

| Capability                      | Unit   | Integration    | Contract | E2E                   | Playwright     | Manual              |
| ------------------------------- | ------ | -------------- | -------- | --------------------- | -------------- | ------------------- |
| Browser inventory               | yes    | yes            | yes      | optional              | yes            | yes                 |
| Managed profile creation        | yes    | yes            | yes      | yes                   | visible state  | yes                 |
| Edge managed launch             | yes    | fake/real      | yes      | yes                   | yes            | yes                 |
| Chrome managed launch           | yes    | fake/real      | yes      | yes                   | yes            | yes                 |
| CDP bridge read                 | parser | fake server    | yes      | yes                   | yes            | yes                 |
| Exact URL evidence              | yes    | yes            | yes      | yes                   | yes            | yes                 |
| Active tab claim                | yes    | yes            | yes      | only if proof exists  | yes            | yes                 |
| Unmanaged browser detection     | yes    | yes            | yes      | yes                   | yes            | yes                 |
| No unmanaged URL claim          | yes    | yes            | yes      | yes                   | yes            | yes                 |
| Bridge disconnect stale         | yes    | yes            | yes      | yes                   | yes            | yes                 |
| Journal persistence             | yes    | yes            | yes      | yes                   | visible        | yes                 |
| Policy dry-run                  | yes    | yes            | yes      | yes                   | yes            | yes                 |
| URL/video intelligence          | yes    | yes            | yes      | yes                   | yes            | manual model review |
| Social account/feed gates       | yes    | yes            | yes      | yes                   | yes            | platform proof      |
| Browser-game/cloud-gaming gates | yes    | yes            | yes      | yes                   | yes            | platform proof      |
| Managed block                   | yes    | yes            | yes      | required before claim | yes            | yes                 |
| Unmanaged terminate             | yes    | yes            | yes      | required before claim | yes            | yes                 |
| AppLocker/App Control           | model  | adapter/manual | yes      | real proof required   | status visible | yes                 |

## E2E Scenarios

- Managed Edge URL evidence.
- Managed Chrome or Chrome for Testing URL evidence.
- Unmanaged browser bypass.
- Bridge disconnect makes evidence stale.
- Policy dry-run does not block.
- Managed block page appears only after intervention proof.
- Managed social signup requires parent approval.
- Managed YouTube Shorts or TikTok-style feed is limited/blocked by route when
  policy says so.
- Managed unknown game portal asks parent.
- Managed cloud gaming requires approval when policy says so.
- Managed educational game is allowed without opening all games.
- Unsupported browser state remains honest.

## Playwright UI Scenarios

- Browser inventory page mixed states.
- Managed browser session states.
- Tab evidence states.
- Unmanaged browser bypass states.
- Browser policy preview capability/manual-required states.
- Browser URL/video intelligence explanation states.
- Social account approval, approved-account, secondary-account, feed/short-video,
  and messaging-route states.
- Browser-game dashboard, game evidence drawer, educational-game allow, cloud
  gaming approval, game purchase gate, and unblocked-site states.
- Browser warning/block delivery states.
- Malicious title/URL escaping and long-value layout safety.

## Manual Browser Validation Matrix

Windows proof is required first:

- Windows 11 child device.
- Edge Stable installed.
- Chrome or Chrome for Testing installed.
- Firefox installed.
- One unsupported browser if available.

Required Windows rows:

- Edge managed profile launches.
- Edge `/json/version` is reachable on loopback.
- Edge `/json/list` returns page targets.
- Edge evidence is journaled.
- Edge evidence is visible in portal.
- Chrome managed profile launches.
- Chrome evidence is journaled.
- Default Chrome profile is not used.
- Unmanaged Chrome is detected as bypass.
- Firefox is shown as unsupported/later-adapter unless a Firefox adapter is
  separately proved.
- Bridge disconnect marks stale.
- Dry-run records would-block without blocking.
- Real block is claimed only after block-page proof.

macOS, Linux, Android, and iOS manual matrices stay dormant until those
workpacks start.

## CI Gates

Use repo-local commands rather than invented package-manager aliases:

```powershell
npm run lint
npm run test
npm run validate:rust
npm run test:e2e
```

Browser-specific focused gates:

```powershell
npm run test:managed-browser-matrix
npm run test:managed-browser-service-proof
npm run test:managed-browser-intervention
node scripts/test/v0-8-browser-domain-adapter-proof.mjs
node scripts/test/windows-managed-unmanaged-browser-enforcement-proof.mjs
```

Manual-required tests must be labelled with explicit reason strings such as:

```text
manual browser proof required
requires-edge
requires-chrome
requires-windows-app-control
```

No ignored or manual test is allowed without a reason.

## Merge Blockers

Block merge if any of these happen:

- Unmanaged browser evidence contains exact URL.
- Target-list evidence marks active tab `known-active` without proof.
- Portal shows stale URL as current after bridge disconnect.
- Default browser profile is accepted for exact evidence.
- Non-Ocentra CDP bridge is accepted.
- Raw `webSocketDebuggerUrl` appears in portal output.
- Browser evidence bypasses journal before portal.
- Policy action executes in dry-run mode.
- AI output executes an action directly.
- Remote/API AI runs by default in the child safety path.
- Memory drives a block without evidence, policy, or parent-action refs.
- Hidden analysis uses child cookies, tokens, default profile, or session data.
- Social signup from a fake platform domain becomes high confidence.
- Parent approval token can be replayed, used after expiry, or used for the
  wrong child/device.
- Unmanaged browser submits exact social URL or account evidence.
- Canvas/WebGL alone auto-classifies a page as a game.
- Cloud gaming URL claims exact game title without metadata.
- Game purchase/signup hidden analysis clicks, downloads, logs in, or purchases.
- Unmanaged browser submits exact browser-game URL or runtime evidence.
- Manual-required action executes an adapter.
- Unsupported browser shows exact URL available.
- Network/domain evidence is mapped as browser tab evidence.
- Process/window evidence is mapped as exact URL evidence.

## Done Signal Per Workpack

Each browser workpack is done only when it has:

- typed contracts;
- runtime/service behavior;
- journal/read-model path if evidence-facing;
- portal state if parent-facing;
- unit tests;
- integration tests;
- contract tests;
- security tests if boundary-facing;
- Playwright tests if UI-facing;
- manual-required gaps listed;
- proof artifact path listed;
- source docs/checklist updated if status changed.

## Minimum Serious MVP Test Set

Do not go below:

- support matrix, inventory model, managed profile model, managed session state,
  URL/domain normalization, active tab state, unmanaged forbidden URL, and
  policy target compile unit tests;
- URL shape classification, metadata evidence, AI result schema, provider
  route, memory stale/miss/hit, and policy handoff unit tests;
- social platform mapping, signup pattern, form-shape, account identity,
  approval request, route kind, social AI, and social policy target tests;
- browser-game platform mapping, game URL shape, cloud-gaming route, runtime
  signal, game metadata, game AI, game approval, game memory, and game policy
  target tests;
- fake Windows inventory, temp managed profile store, fake Chromium launcher,
  fake CDP server, tab evidence mapper, journal/SQLite replay, unmanaged
  process detection, and policy dry-run integration tests;
- BrowserInventoryRow, ManagedBrowserSession, BrowserTabEvidence,
  UnmanagedBrowserEvidence, BrowserReadModel, and BrowserPolicyAction contract
  tests;
- default profile rejection, wrong bridge port, non-loopback bridge, stale
  session, unmanaged URL claim, raw debugger URL leak, and malformed CDP
  security tests;
- managed Edge/Chrome evidence, unmanaged Chrome bypass, bridge disconnect
  stale, policy dry-run, and managed block E2E proof;
- inventory, managed session, tab evidence, unmanaged bypass,
  capability/manual-required, and malicious UI Playwright coverage.

## Final Quality Bar

The subsystem is solid only when:

```text
A managed Edge/Chrome session can produce exact URL/title evidence.
That evidence is typed, journaled, replayed, and visible in the portal.
The evidence clearly says whether active tab is known or unknown.
Unmanaged browser use is visible as bypass/process evidence only.
Unsupported browsers are honest states, not hidden failures.
Blocking/warning actions never execute without capability proof.
Parent UI never upgrades weak evidence into stronger claims.
Manual proof artifacts exist before product claims are upgraded.
```
