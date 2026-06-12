<!-- agent-capsule -->

> Agent Capsule
> Doc: Portal Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Portal Feature Expectations

Portal features must exercise the real agent path.

The portal is a parent-facing control and observability surface. In production,
the main household portal should be a local packaged app or mobile parent app,
with Tauri as the preferred desktop-shell candidate unless a later architecture
decision replaces it. It does not execute child-device work. It sends typed
queries, rule updates, approval decisions, and visibility requests to the
child-device agent, then renders validated events and read models returned by
that agent.

The portal is also where sensitive capabilities become parent-controlled product
options. It should show what is enabled, disabled, observe-only, dry-run, or
enforcement-eligible, and it should make clear that household rules come from the
parent, not hidden Ocentra defaults.

## Outcome Bar

Parent outcome:

- A parent can open the local or LAN portal and see real service health,
  evidence-store status, recent activity, capture status, and diagnostics for
  the connected child-device agent.
- The portal makes it clear when data is live, stale, degraded, unavailable, or
  out of scope.
- The portal lets a parent review and change supported observation, cadence,
  trigger, retention, rule, and enforcement settings through typed intents.
- The portal shows whether data came from a live local/LAN child agent, parent
  device cache, parent-owned storage, or an Ocentra-hosted stateless compile
  request.

Child-device outcome:

- The child-device agent remains the execution boundary for capture, storage,
  policy, AI, timers, enforcement, and service state.
- Portal actions are typed requests to the agent, not browser-side shortcuts.

Platform scope:

- V0.5 focuses on web portal visibility for the Windows-first local agent.
- The same portal contracts should work over loopback and explicit LAN mode.
- Web does not become a child-device agent.
- The current Vite app is a development scaffold for exercising the real service
  path. The production parent portal should be packaged for parent devices
  before it is represented as the real product portal.
- `family.ocentra.ca` is a public/download/account/subscription surface and may
  host authenticated status or stateless report compilation. It is not the
  default child-activity data store.

Data scope:

- In scope: service health, connection state, journal status, ingest status,
  query-store status, recent processes/apps/windows, later recent domains,
  typed failure states, event ids, timestamps, source ids, and copy/debug
  summaries.
- Out of scope for V0.5: raw journal file browsing, raw SQLite file browsing,
  decrypted content payloads, screenshots, keystrokes, chat text, and hidden
  surveillance views.
- Out of scope by default: storing child activity evidence, reports, or parent
  rules in Ocentra-hosted web infrastructure.

Trust boundary:

- The Rust service validates and executes requests.
- The portal validates returned payloads through Effect Schema before rendering.
- Copy/debug output is for troubleshooting and handoff, not a private data
  export feature.
- Hosted web surfaces may authenticate, download, manage billing, show connector
  status, or invoke stateless report compilation, but they do not own child
  evidence or policy execution.

Contract boundary:

- Portal routes, DOM ids, button/action descriptors, and display text live in
  domain packages.
- Agent requests and events use shared protocol contracts.
- New portal controls require explicit typed intent or query contracts before
  UI code sends them.

## Expected Deliverables

- UI reads typed domain/protocol contracts.
- UI validates agent events and read models through Effect Schema.
- UI uses text/domain packages for display text and stable DOM ids.
- One clear result area for intent output where appropriate.
- Timeline or table pattern for recent activity and logs.
- Device health panel with service, journal, ingest, query, and capture status.
- Copy/debug affordance for sharing current result, connection state, and recent
  typed diagnostics.
- Explicit rule/query/approval intent contracts for any parent action.
- Explicit parent-control views for sensitive features that show current
  setting, actor, effective device/child, and last result.
- Source/custody indicators for live local/LAN, parent cache, parent-owned
  storage, Ocentra-hosted non-activity metadata, and unavailable states.
- Playwright coverage when UI behavior changes.

## Acceptance

- Portal connects to the real local service in tests.
- Playwright proves the visible behavior against the real service path.
- Control clicks update existing panels instead of appending endless boxes unless
  the feature is explicitly a log view.
- Parent actions are represented as typed intents, not browser-executed work.
- Device-side execution result is visible when a parent action changes rules,
  approvals, query state, or device state.
- Logs/history use a table or timeline pattern.
- UI remains usable on common desktop and mobile widths.
- Browser-visible errors and warnings are treated as product issues unless
  proven harmless and documented.
- Empty, loading, stale, degraded, and failure states are visible and do not look
  like successful fake data.
- Hosted views do not present Ocentra-stored child activity when the source is
  actually local cache, parent-owned storage, or unavailable.

## V0.2 Through V0.5 Expectations

V0.2 portal proof:

- Shows real journal, ingest, query-store, and service health/status.
- Can request recent activity summaries through the service path.

V0.3 process/window visibility:

- Shows real Windows process/window observations only after they are journaled
  and queryable.
- Labels process/window evidence so parents do not confuse it with browser URL
  or content inspection.

V0.4 network/domain visibility:

- Shows recent domains/process correlation only when the network/domain adapter
  and contracts exist.
- Keeps unknown attribution visible instead of inventing category labels.

Browser evidence visibility:

- Shows managed-browser URL/tab evidence only when supplied by typed browser
  evidence.
- Labels unmanaged browser use as possible bypass instead of pretending exact
  URL visibility exists.
- Shows missing bridge, unsupported browser, stale evidence, and permission
  states as explicit capability status.

Screen evidence visibility:

- Shows local screen-analysis summaries, confidence, categories, source evidence
  refs, and policy result without exposing raw images by default.
- Shows whether the temporary image was deleted, expired, failed, or unavailable.
- Shows whether screen analysis is enabled, disabled, observe-only, dry-run, or
  enforcement-eligible, plus current cadence, trigger, and retention/deletion
  settings.
- Clearly explains that local screen analysis runs on the child device only when
  parent-controlled settings enable it, and that images do not leave the child
  PC under this feature.

V0.5 live activity portal:

- Provides a usable local parent visibility surface with health, activity,
  source, and diagnostics panels.
- Provides copy/debug output that can be pasted into an issue or handoff without
  exposing secrets or raw private content.
- Uses one primary result panel or a clear timeline/table instead of creating a
  new card for every action.

## Copy And Debug Affordances

- Copy should include current agent URL, connection state, request/response ids,
  event ids, timestamps, source ids, health state, and concise read-model rows.
- Copy should redact secrets, keys, local private file paths when not needed,
  raw encrypted payloads, and any future private content fields.
- Debug views should distinguish service logs, activity evidence, storage
  status, and protocol events.
- A failed copy action should report a visible typed failure state.

## Non-Goals

- Do not bypass the Rust service with hardcoded browser state.
- Do not run OS commands, capture adapters, AI safety evaluation, policy
  evaluation, enforcement, timers, or scripts in the portal.
- Do not let portal code become the source of truth for whether a child activity
  is allowed or blocked.
- Do not hide sensitive capability settings behind developer defaults.
- Do not create a polished marketing dashboard before the underlying data path
  exists.
- Do not show fake activity data as if it came from the child device.
- Do not implement blocking, content inspection, stealth behavior, or local AI
  decisions in V0.5 portal work.
- Do not present `family.ocentra.ca` as the source-of-truth parent portal for
  child activity data.
- Do not cache or retain child reports in Ocentra-hosted infrastructure unless a
  future explicit data-custody feature is approved.

## Validation Gates

- Type, lint, and contract validation for any touched portal/domain code.
- Playwright coverage for health, recent activity, copy/debug, degraded service,
  and visible failure states when UI behavior changes.
- Real Rust service smoke for portal connection and WebSocket protocol flow.
- Browser console check for visible errors and warnings on touched routes.
- Responsive checks for common desktop and mobile widths when layout changes.

## Done Signal

The portal shows real service data, validates payloads, sends only typed parent
intents to the device agent, has useful copy/debug affordances, communicates
degraded states honestly, and has Playwright coverage for the parent-visible
behavior.
