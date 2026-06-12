<!-- agent-capsule -->

> Agent Capsule
> Doc: Capture Feature Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Capture Feature Expectations

Capture features create real observations from a child device. They do not make
safety decisions by themselves. Their job is to produce honest, typed evidence
that storage, query, portal, policy, and later AI features can consume.

## Outcome Bar

Parent outcome:

- A parent can see which apps, processes, windows, and later network/domain
  destinations were observed on the child device, with clear timestamps and
  source attribution.
- A parent can tell when capture is unavailable, disabled, degraded, or only
  partially supported on the current platform.
- A parent can explicitly enable, disable, and review sensitive capture modes
  such as screen analysis through typed portal controls.

Child-device outcome:

- The agent samples or subscribes to platform activity without blocking local
  service responsiveness.
- The agent writes observations through the journal and query-store path before
  the portal presents them as evidence.

Platform scope:

- Windows process and foreground-window activity are the V0.3 required target.
- Windows network/domain observation is V0.4 and must remain separate from
  process/window capture unless contracts explicitly join them.
- Browser URL/tab evidence is a separate post-foundation capture slice. It must
  use an Ocentra-managed browser integration boundary and must not be inferred
  from process/window or network/domain capture alone.
- Other platforms may expose capability status or scaffolded adapters, but must
  not claim capture parity until real OS behavior and tests exist.

Data scope:

- Process/window capture may record process identity, executable name or path
  where allowed, process id when appropriate, active/foreground state, window
  title metadata when available and acceptable for the milestone, observation
  timestamps, source ids, adapter ids, and capability status.
- Process/window capture must not claim browser URL visibility, page content,
  chat content, video metadata, screenshots, keystrokes, or decrypted network
  payloads.
- Network/domain capture, when implemented, may record normalized
  domain/IP/port/process correlation where available, but not decrypted HTTPS
  content.
- Network flow capture may record process-attributed flow metadata, connection
  counts, first/last seen, bytes sent/received where available, DNS/domain
  attribution, VPN/proxy/tunnel indicators, and unusual-traffic summaries. It
  must not store raw packet dumps as the normal evidence model or claim exact
  browser URLs from flow metadata.
- Browser URL/tab capture, when implemented, may record supported browser,
  running status, profile/window/tab ids where available, active tab state, exact
  URL, normalized domain, page title, observation timestamp, evidence id, source
  id, adapter id, and capability status. It must not record page body text,
  screenshots, keystrokes, form values, browser secrets, or decrypted HTTPS
  payloads.
- Browser instances outside the managed Ocentra browser boundary are not URL
  evidence. They must be reported as unmanaged browser use with explicit
  capability status and, in later enforcement milestones, may be blocked or
  terminated according to parent policy.
- App/game capture may record process, executable path, publisher/signature/hash
  where available, foreground state, window title, installed app/game metadata,
  launcher manifest hints, and queryable session summaries such as run count,
  running time, foreground time, first seen, and last seen. AI must not be the
  scanner for these facts.
- Screen evidence capture, when enabled, may record encrypted temporary image
  queue jobs, local OCR/vision summaries, categories, confidence, source evidence
  references, image digest, and deletion status. Long-term evidence should be the
  structured summary, not permanent screenshots, unless a later milestone adds an
  explicit parent-controlled retention feature.

Trust boundary:

- Platform adapters observe OS state and return adapter observations.
- Mapping code converts adapter observations into domain activity events.
- The agent service owns scheduling, status, journal writes, ingest, and typed
  service responses.
- The portal requests and displays capture status and evidence; it does not run
  capture adapters or OS commands.
- Sensitive capture modes are disabled or observe-only until a parent enables
  them through typed controls that the child-device agent validates.

Contract boundary:

- Capability status, observation source ids, adapter ids, event payloads,
  unsupported/degraded reasons, and capture control/query intents belong in
  shared domain contracts before runtime code depends on them.
- Windows-specific details may live behind adapter boundaries, but emitted
  events should use platform-neutral concepts where possible.
- Unknown or partial attribution is a first-class contract state, not a place
  to guess.

## Expected Deliverables

- Platform-specific adapter behind a platform-neutral boundary.
- Capability/status intent for available, unavailable, disabled, degraded, and
  unsupported states.
- Observation-to-activity event mapping.
- Source id, adapter id, host/device reference, observation mode, and timestamp
  metadata.
- Typed failure reason when the OS capability is unavailable, permission-limited,
  unsupported, or errors during observation.
- Journal write path from real observations.
- Query-store ingest path from real observations.
- Recent activity read model for portal visibility.
- App/game session read model for duration, foreground time, and category-ready
  evidence when app/game capture is in scope.
- Screen-analysis queue and summary read model when screen evidence is in scope.
- Local AI evaluation input only when a later milestone explicitly introduces a
  dry-run policy or AI preview contract.
- Dev/local portal visibility for captured evidence through the real service
  path.

## Acceptance

- Tests prove mapping from adapter observation to activity event.
- Service remains responsive while capture is active.
- Capture can be disabled in dev and reports that state clearly.
- Sensitive capture modes show parent-selected settings and capability status
  rather than pretending they are always active.
- Capture failures do not crash the service.
- Platform claims are scoped to real tested behavior.
- Captured page, video-link, app, domain, process, or window context is explicit
  about what was observed and what was not observed.
- Observations are journaled and ingested into SQLite before portal views depend
  on them.
- Manual local validation can show current process/window evidence in the portal
  on Windows.

## Windows Process And Window Capture

V0.3 expected behavior:

- Observe real running processes on Windows.
- Observe foreground app/window activity when the OS API allows it.
- Record timestamps, source ids, adapter ids, and observation mode.
- Distinguish snapshot observations from foreground/active-window observations.
- Avoid blocking the WebSocket intent loop or health/status handling.
- Report access-denied, unavailable, no-active-window, and adapter-error states
  as typed failures or degraded statuses.
- Do not claim browser URL visibility from process/window capture alone.

Useful parent-facing examples:

- "Chrome is the foreground app" is in scope.
- "Discord is running" is in scope if observed from process state.
- "The child is on youtube.com" is not proven by process/window capture alone.
- "The page content is unsafe" is not in scope for V0.3.

## Windows Network And Domain Observation

V0.4 expected behavior:

- Observe domain/IP/port/process correlation where available.
- Observe process-attributed network flow metadata such as protocol, local and
  remote endpoints, connection state, timestamps, counts, and bytes where the
  adapter supports them.
- Derive queryable summaries such as top processes, top destinations, high-volume
  flows, new destinations, and unusual VPN/proxy/tunnel indicators.
- Prefer normalized intent events over raw packet dumps.
- Do not decrypt HTTPS payloads.
- Do not claim full browser URLs, page content, chat content, or search terms
  from network flow metadata.
- Do not claim content inspection.
- Record unknown attribution clearly instead of guessing.
- Keep network/domain observation as a separate adapter and contract slice from
  process/window capture unless a deliberate join contract exists.

## Browser URL And Tab Evidence

Expected behavior:

- Enumerate supported browser integrations.
- Detect supported browsers that are running.
- Observe browser windows and tabs where the browser integration permits it.
- Identify the active browser tab.
- Record exact URL, page title, normalized domain, timestamp, evidence id,
  source id, adapter id, and capability status.
- Report unsupported browser, unmanaged browser, missing managed bridge, missing
  permission, stale evidence, and adapter-error states explicitly.
- Store browser evidence through journal and query-store paths before portal or
  AI use.
- Keep browser evidence separate from process/window and network/domain capture
  unless a deliberate join contract links evidence ids.

Useful parent-facing examples:

- "Chrome is running" is process evidence.
- "Chrome connected to youtube.com" is network/domain evidence.
- "The active Chrome tab is https://www.youtube.com/watch?v=..." is browser
  URL/tab evidence.
- "The video content is unsafe" is a later AI/policy classification, not raw
  browser capture.

## Service Responsiveness

- Capture work must be async or isolated enough that health, status, and portal
  intents still respond.
- Slow OS calls must be bounded by timeouts or background tasks.
- Backpressure must surface as typed status instead of unbounded memory growth.
- A capture adapter crash or permission failure must not take down the whole
  service.

## Non-Goals

- Do not add blocking.
- Do not add untyped AI classification.
- Do not add stealth or anti-tamper behavior.
- Do not claim unsupported OS capabilities.
- Do not inspect content, decrypt traffic, log keystrokes, or take screenshots
  in V0.2 through V0.5.
- Do not run capture from the portal.

## Validation Gates

- Contract tests for capture status, source ids, observation payloads, and
  failure reasons.
- Adapter mapping tests using real parser/mapping code, not mocks or spies.
- Rust service tests for capability/status intents and nonblocking behavior.
- Journal and SQLite integration tests proving real observations enter the
  evidence path.
- Manual Windows local run for process/window evidence before claiming V0.3
  parent-visible behavior.
- Portal Playwright coverage when the UI shows capture status or activity.

## Done Signal

A local run records real OS observations into the journal and query store, keeps
the service responsive, reports degraded states honestly, and lets the portal
show those observations through the real service path.
