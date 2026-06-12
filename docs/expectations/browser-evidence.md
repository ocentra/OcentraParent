<!-- agent-capsule -->

> Agent Capsule
> Doc: Browser URL And Tab Evidence Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

# Browser URL And Tab Evidence Expectations

Browser evidence is the product bridge between low-level device observation and
useful parent safety decisions. Process/window capture may prove that a browser
is active. Network/domain capture may prove network destinations. Neither one
proves which browser tab is open, which URL is active, or what page title the
child is seeing.

The focused implementation architecture lives in
[Browser URL And Tab Evidence Capture Architecture](../architecture/browser-url-tab-evidence-capture.md).
The current implementation and proof plan lives in
[Browser Plan](../plans/browser-plan/README.md).

## Outcome Bar

Parent outcome:

- A parent can see which supported browsers are installed or detectable.
- A parent can see which supported browsers are running.
- A parent can see which browsers are supported for managed URL capture now,
  which are installed but unsupported, and which require a later adapter.
- A parent can see open browser windows and tabs where the browser integration
  permits it.
- A parent can see the active browser tab, exact URL, page title, normalized
  domain, timestamp, evidence id, and source integration id.
- A parent can tell when browser URL/tab evidence is unavailable, permission
  limited, unsupported, stale, or degraded.
- A parent can tell whether evidence came from a live managed browser session,
  a stale managed session, an unmanaged browser detection, local cache, or a
  parent-owned export/report path.

Child-device outcome:

- The child-device agent collects browser evidence through a deliberate browser
  integration boundary. The preferred MVP path is an Ocentra-managed browser
  launch/profile plus a browser-supported local bridge; browser extensions are
  not the default product path.
- Browser evidence is journaled and ingested before the portal or local AI uses
  it.
- Browser evidence collection must not block the service event loop.
- Browser-like processes outside the managed Ocentra browser boundary are
  reported as unmanaged browser use and possible bypass.
- The child-device agent stores browser evidence locally before any portal,
  policy, or AI consumer receives it. Ocentra-hosted services are not the
  default store for URL history, titles, browser evidence, reports, or rules.

## Data Scope

Browser evidence may record:

- Browser family and supported status.
- Browser process/running status.
- Browser channel and browser version where available.
- Browser profile id where available and safe.
- Managed/unmanaged browser status.
- Managed browser session id and bridge capability status.
- Window id and tab id where available.
- Browser target id where available.
- Active/inactive/unknown state for windows and tabs, plus the proof source
  that justifies the state.
- Exact tab URL.
- Normalized domain and origin.
- Page title.
- Observation timestamp and freshness/expiry timestamp.
- Evidence id, source id, adapter id, and capability status.
- Managed install state, permission state, bridge state, and degraded reason.

Browser evidence must not record unless a later milestone explicitly approves it:

- Page body text.
- Chat message content.
- Screenshots.
- Keystrokes.
- Form values.
- Cookies, tokens, local storage, or browser secrets.
- Decrypted HTTPS payloads.

## Trust Boundary

- Browser integrations observe browser state and emit typed browser evidence.
- The Ocentra-managed browser boundary owns exact URL/tab evidence. Native
  process/window and network/domain adapters may detect unmanaged browser use,
  but must not infer exact URLs from that evidence.
- A browser extension may supplement the managed bridge only when it is installed
  into an Ocentra-managed profile, talks to a registered native host, and reports
  permission/install state through typed contracts. It must not be the default
  product path for unmanaged personal profiles.
- Native process/window and network/domain adapters must not guess browser tab
  URLs.
- Mapping code normalizes browser evidence into shared activity/evidence
  contracts.
- Service capture must append browser evidence to the encrypted journal first,
  then replay journaled records into the SQLite query store and read models.
  Duplicate event ids must not double count, and restart reads must preserve
  stable evidence ids and stale/degraded fields.
- Local AI and policy evaluators consume only schema-valid browser evidence with
  evidence references.
- Portal views display browser evidence but do not run browser capture.

## Expected Deliverables

- Supported-browser capability contract.
- Browser running-state contract.
- Managed browser launcher/profile contract.
- Managed install and permission state contract.
- Browser window/tab evidence contract.
- Active-tab evidence contract.
- URL/title/domain normalization contract.
- Browser integration status and degraded reason contract.
- Unmanaged browser detection event and possible-bypass status, including
  process id/name, redacted executable path ref when available, signature/hash
  refs when available, family/channel guess, process kind, confidence, and
  detection reason.
- Journal write and query-store ingest path.
- Portal recent browser activity view.
- Managed browser intervention rows with policy decision refs, intervention
  action refs, audit refs, evidence refs, target type/value, browser boundary
  state, exact URL claim state, unmanaged detection state, and child delivery
  state.
- Unmanaged browser fallback rows with explicit action state: report-only,
  warn-child, parent-review, terminate-process, relaunch-managed-browser,
  OS-block configured, OS-block manual-required, allowed unmanaged exception,
  degraded, or unavailable. These rows must not include exact unmanaged URLs,
  social account/feed/video identifiers, browser-game titles/accounts/purchase
  identifiers, or cloud-gaming title claims.
- Windows AppLocker/App Control proof-state rows for readiness-check,
  audit-only, enforced, manual-required, unavailable, and failed paths. These
  rows may name publisher/path/hash/package identity kinds and
  administrator/manual/service permission requirements, but they must not claim
  launch prevention, policy creation/update, rollback, unmanaged exact URLs, or
  broad browser control without real Windows artifacts.
- Optional extension/native-host boundary rows for installed/enabled, disabled,
  permission-required, native-host-missing, origin-invalid, schema-invalid,
  length-invalid, stale-heartbeat, and managed-profile-bound states. These rows
  may describe minimum permissions and native-message validation, but they must
  not claim unmanaged personal-profile capture, runtime signal capture, or
  browser-game evidence without separate package/install/native-host proof.
- Browser performance/service-health rows for inventory scan, support-matrix
  derivation, 100-tab CDP target mapping, journal write per event, 10000-event
  SQLite replay, unmanaged process scan, rapid bridge reconnect, memory/cache
  lookup invalidation, portal 100-tab render, URL/video metadata extraction,
  local AI queue timeout, browser-game runtime signal collection, and
  cloud-gaming heuristic timeout. Rows must distinguish fixture-backed measured
  proof from manual-required future/runtime paths and must not set runtime
  claims until real artifacts exist.
- Browser-game parent policy compiler rows that consume only parent-owned
  browser-game evidence refs, analysis refs, mobile capability refs, rule refs,
  and schedule refs, and return candidate-only allow, warn, parent-review, block,
  time-limit, manual-review, or unknown outcomes. These rows must not claim
  final policy authority, runtime gate execution, UI delivery, native game
  control, cloud-frame analysis, raw game payload storage, or enforcement.
- Browser-game managed hold/block adapter rows that link policy candidate refs,
  child UX refs, managed intervention adapter proof refs, and audit refs for
  hold-until-classified, parent-approval hold, block, and warn paths. Candidate
  allow/time-limit, manual-required cloud, and unavailable native/unmanaged rows
  must remain non-executing. These rows must reject raw URL/page/game payloads,
  child cookie/session reuse, unmanaged exact URL claims, browser mutation,
  rendered child pages, notification delivery, final policy decisions, applied
  time limits, cloud-frame analysis, native game control, and enforcement.
- Browser-game journal/SQLite read-model rows that index managed browser
  evidence journal replay, app-game session report proof, adapter audit refs,
  manual-required cloud rows, and unavailable native/unmanaged rows. Proof rows
  must carry journal entry refs, SQLite row refs, source read-model refs, proof
  refs, positive event/row counts, and matching reason codes. They must reject
  raw URL/page/game/title/account/purchase storage, child cookie/session reuse,
  cloud title certainty, browser mutation, rendered UI, final policy decisions,
  and enforcement.
- Browser-game platform/route contract rows that describe platform kinds, route
  surface kinds, source kinds, custody labels, pattern refs, evidence refs,
  confidence, reviewed/candidate/manual-required status, and managed-browser
  requirements. They must reject raw domains, raw URLs, raw paths, raw page
  bodies, live URL parser claims, runtime detection claims, AI classification
  claims, final policy decisions, native game control, cloud-frame analysis, and
  enforcement.
- Browser-game URL shape parser rows that accept unknown input and emit only
  protocol/host/path-depth shape, route surface kind, route hint booleans,
  query/fragment shape booleans, reason codes, confidence, and a route-shape
  fingerprint. They must reject raw URL, domain, path, query, and fragment
  storage, browser navigation claims, runtime detection claims, AI
  classification claims, final policy decisions, cloud-frame analysis, native
  game control, and enforcement.
- Browser-game runtime signal detector rows that emit only signal kind, source
  kind, fingerprint, evidence refs, confidence, status, reason codes, and
  managed-browser proof requirements for canvas, WebGL, Gamepad API,
  fullscreen, pointer-lock, audio, animation-loop, iframe surface,
  cloud-streaming, and unknown/manual-required shapes. They must reject raw DOM,
  canvas, stream, audio, and gamepad input storage, browser instrumentation,
  runtime detection execution, AI classification claims, final policy decisions,
  cloud-frame analysis, native game control, and enforcement.
- Browser-game metadata extractor rows that emit only metadata field kinds,
  source kinds, fingerprints, evidence refs, confidence, status, and reason
  codes for title, description, genre, age-rating, publisher, thumbnail,
  educational subject, cloud platform title, and unknown/manual-required shapes.
  They must reject raw title, description, page body, image, structured data,
  runtime DOM extraction, platform API calls, AI classification claims, final
  policy decisions, cloud-frame analysis, native game control, and enforcement.
- Browser-game hidden analysis profile safety rows that emit only isolated
  profile kinds, profile fingerprints, loader proof refs, evidence refs,
  confidence, status, reason codes, retention bounds, and safety flags. They
  must reject child cookie/session reuse, shared child storage, raw URL/page/game
  payload/frame storage or capture, browser instrumentation, hidden native
  control, AI classification claims, final policy decisions, UI rendering,
  cloud-frame analysis, native game control, and enforcement.
- Browser-game portal pattern library rows that model known-game, educational,
  UGC, indie, classic archive, school, and unknown portal families through
  route kinds, signal kinds, pattern fingerprints, evidence refs, confidence,
  and review states. They must reject raw domains, raw URLs, raw page titles,
  raw page bodies, runtime detection claims, AI classification claims, policy
  decisions, cloud-gaming ownership, and enforcement.
- Browser-game cloud-gaming pattern library rows that model cloud platform,
  cloud PC, mobile portal, browser-embedded cloud-game, native launcher prompt,
  and unknown/manual-required cloud surfaces through route kinds, signal kinds,
  pattern fingerprints, evidence refs, confidence, and review states. They must
  reject raw cloud domains, raw cloud URLs, raw cloud titles, raw stream frames,
  runtime detection claims, cloud-streamed frame analysis, per-game cloud-title
  certainty, native launcher/game control, final policy decisions, and
  enforcement.
- Browser-game educational classifier rows that classify educational,
  entertainment, misleading educational claim, unknown, manual-required, or
  unavailable candidates from evidence refs only. They may reference school URL,
  teacher/parent allowlist, metadata, AI classification, past approval,
  homework context, school platform, and platform self-label evidence, but they
  must not treat platform labels as authority, consume raw page/game/model
  payloads, execute account or purchase flows, make final policy decisions,
  execute runtime gates, render UI, inspect cloud-streamed frames, control
  native games, or enforce actions.
- Browser-game AI analysis input/result rows that consume typed evidence refs
  only for browser evidence, URL shape, runtime signals, metadata, screen
  summaries, parent rules, recent activity, memory, task, and custody labels.
  Results may produce candidate benefit/risk signals, game surface kind,
  modifiers, recommended policy input, confidence, uncertainty, summary refs,
  model runtime refs, prompt template version, expiry, and degraded/manual
  states. They must reject raw URLs, raw page body, raw game payloads, raw
  screen frames, raw model text, account/purchase execution, native game
  control, cloud-frame analysis, final policy decisions, runtime gate
  execution, rendered UI, and enforcement.
- Browser-game risk/benefit signal rows that model bounded evidence-backed risk
  and benefit candidates. Risk rows may cover violence, horror, adult themes,
  addictive loops, multiplayer/contact, chat, purchase, loot box/random item,
  UGC, privacy, unblocked-bypass, and unknown risk. Benefit rows may cover
  educational value, homework relevance, skill building, creativity, problem
  solving, parent-approved game, neutral, and unknown benefit. These rows must
  reject raw game payloads, chat content, page body, raw model text,
  account/purchase execution, cloud-frame analysis, native game control, final
  policy decisions, runtime gate execution, and enforcement.
- Browser-game memory/cache rows that store only refs or hashes for canonical
  URL, platform game, domain path, cloud title, parent decision, game category,
  policy version, child profile, parent rule set, and evidence. Fresh hits may
  feed policy input only with bounded TTL, decision refs, evidence refs, and
  required subject keys. Stale, miss, and manual-required rows must not drive
  policy input. These rows must not store raw URLs, raw platform game IDs, raw
  cloud game titles, raw game payloads, raw model text, runtime cache state, AI
  cache state, UI delivery, native game control, cloud-frame analysis, final
  policy decisions, or enforcement.
- Browser-game account/signup/purchase gate rows that model account creation,
  login, secondary account, purchase, subscription, loot box/random item,
  virtual currency, download/install, wallet/gambling-like payment,
  cloud-gaming start, and unknown-game start approval states from evidence refs
  only. These rows must reject raw URLs/titles/account identifiers,
  credentials, form submission, account creation, purchase/payment execution,
  launcher downloads, notifications, rendered UI, child notification, final
  policy decisions, runtime gate execution, native game control,
  cloud-frame analysis, and enforcement claims.
- Browser-game cloud-gaming gate rows that model platform/session evidence from
  known cloud domains, streaming routes, gamepad/fullscreen/high-bandwidth/
  low-latency signal refs, and optional platform title/rating metadata refs.
  These rows may produce unknown cloud-game approval, mature cloud-game block,
  school-night block, time-budget, manual-required, and unavailable candidates
  only. They must reject raw cloud titles, raw stream frames, cloud-streamed
  frame analysis, per-game cloud title claims, native game/launcher control,
  game chat content, account/purchase flows, notifications, rendered UI, child
  notification, final policy decisions, runtime gate execution, and enforcement.
- Browser-game unblocked-site detection rows that model managed browser
  routes/pages, search intent, portal indexes, iframe embeds, proxy/mirror
  routes, hidden game origins, school bypass language, unmanaged browser
  process-only bypass evidence, manual-required states, and unavailable states
  from evidence refs only. These rows may recommend candidate actions for
  block-during-school, parent-review, allow-specific-game, block-unknown-iframe,
  bypass-evidence-only, manual-review, or unknown inputs, but they must reject
  raw URLs, raw page body, raw search queries, captured iframe content, exact
  unmanaged URL claims, native game control, cloud-frame analysis,
  account/purchase flows, rendered UI, final policy decisions, runtime gate
  execution, and enforcement.
- Browser-game UGC/multiplayer/chat risk rows that model UGC pages, experience
  pages, lobbies, profile/friends/message routes, launch prompts, and
  web-to-app launch surfaces from evidence refs only. They may recommend
  candidate controls for approved experiences, parent approval, chat blocking
  where capability refs exist, time limits, purchase approval, unknown UGC
  blocking, manual review, or unknown state, but they must not read chat
  content, store profile/account/experience identifiers, execute web-to-app
  launches or purchases, control native games, make final policy decisions,
  execute runtime gates, render UI, or enforce actions.
- Browser-game child checking/block UX rows that use schema-known child text
  token refs for checking unknown games, parent approval, blocked candidates,
  educational allowed state, time-limit candidates, cloud-gaming manual state,
  and native game unavailable state. These rows must reject raw child copy,
  rendered child UI, notification delivery, runtime browser block, block-page
  rendering, applied time limits, final policy decisions, cloud-frame analysis,
  native game control, and enforcement claims.
- Browser-game parent dashboard UX rows that organize detected game review,
  unknown-game approval, cloud-gaming approval, educational allowlist, game
  time-budget candidates, mobile/native capability gaps, and manual-required
  gaps for future parent surfaces. These rows must reject rendered portal UI,
  notification delivery, runtime data fetch, final policy decisions,
  cloud-frame analysis, native game control, and enforcement claims.
- Android/iOS browser-game capability matrix rows for owned browser shells,
  WebView/Custom Tabs, installed browser apps, cloud-gaming web sessions,
  device-owner browser policy, Family Controls, Safari web-domain tokens,
  application tokens, managed browser shells, and Web Clip/PWA surfaces. Rows
  must stay manual-required, token-limited, entitlement-required, app-level, or
  domain-level until real device/platform proof exists and must not claim exact
  game content, cloud-streamed frame analysis, native game control, game chat,
  runtime signals, UI delivery, or enforcement.
- Browser-game proof artifact gates may verify checklist ownership, proof
  folders, required source/security/validation/UI-marker files, README
  references, feature coverage, expectation boundary text, and rendered child
  intervention screenshots only when the proof opens real public browser-game,
  cloud-gaming, or game-store surfaces and serves the shared child intervention
  page through the Rust child-agent endpoint. They must not claim final policy
  decisions, product runtime browser-game detection, notification or approval
  delivery, cloud-streamed frame analysis, native game control, enforcement, or
  product completion.
- Browser-game rollout gates may label rows as complete,
  partial/manual-required, or open/manual-required only. GAME rollout state:
  partial/manual-required means product completion is unclaimed; missing route,
  runtime, metadata, AI, memory, parent dashboard runtime UI, notification or
  approval delivery, cloud-streamed frame-analysis, native game control, and
  enforcement proof must stay open/manual-required until separate artifacts
  exist.
- A browser evidence artifact manifest that points to managed profile,
  intervention, unmanaged Windows, policy dry-run, performance, screenshot, and
  manual-required proof paths. The manifest must classify rows as
  artifact-present, partial/manual-required, or manual-required so missing model,
  parent-decision, runtime-signal, cloud-session, unsupported-adapter, and
  cross-platform artifacts are not mistaken for proof.
- Tests for schema validation, URL/domain normalization, stale evidence, and
  unsupported/degraded states.
- Manual local validation against at least one supported browser.

## MVP Managed Browser Procedure

The Windows Rust agent should follow this procedure for browser URL/tab evidence:

WP05 proof status: `scripts/test/browser-platform-windows-managed-cdp-proof.mjs`
now proves the local Windows managed-browser procedure through a temporary
Ocentra-owned profile and loopback CDP endpoint for a local proof page. The
artifact records that a real Chromium-family browser launched, `/json/version`
and `/json/list` responded, the exact local managed proof URL was observed, a
CDP screenshot was captured, and the temporary profile was removed. This is not
yet a product-complete active-tab enforcement, final policy execution, browser
blocking, or non-Windows support claim, and the artifact must not persist raw
executable paths, raw profile paths, raw CDP payloads, raw page content, or
browser secrets.

1. Inventory installed browsers.

   Detect Chrome, Edge, Brave, Firefox, Opera, and other browser-like executables
   where practical. Record browser family, version, executable path,
   signature/hash where available, and whether the browser is supported for
   managed URL capture. MVP URL/tab capture should start with Edge Stable and
   Chrome or Chrome for Testing. Brave may follow after executable identity,
   managed-profile launch flags, and bridge behavior are proven. Firefox, Opera,
   portable browsers, embedded WebViews, and unknown Chromium forks remain
   unsupported or unmanaged states until a separate adapter proof exists.

2. Create an Ocentra-managed browser profile.

   Store managed browser profile data under an Ocentra-owned path such as
   `C:\ProgramData\Ocentra\Parent\ManagedBrowsers\chrome-child-profile`. This
   profile is separate from the child's normal browser profile. Modern Chrome
   requires a non-default user data directory for remote debugging, so exact
   URL/tab capture must not depend on attaching to the default user profile.
   The profile needs a stable `profileId`, an internal `profilePathRef`, and a
   parent-visible custody label that says the profile is local to the child
   device.

3. Launch the browser through Ocentra.

   Start the approved browser from the agent or approved launcher with a managed
   user data directory and a localhost-only browser bridge, for example:

   ```powershell
   chrome.exe --user-data-dir="C:\ProgramData\Ocentra\Parent\ManagedBrowsers\chrome-child-profile" --remote-debugging-port=<reserved-local-port>
   ```

   The bridge must bind only to localhost. Prefer an agent-reserved random local
   port over a fixed public convention such as `9222`. Track the launched
   process id, executable path, profile path, bridge port, managed session id,
   and browser family.
   Record managed install state such as installed, not installed,
   installed-but-unsupported, managed-profile-ready, bridge-unavailable,
   permission-limited, or adapter-error.

4. Connect to the browser bridge.

   The Rust agent connects to the managed browser's local bridge endpoints:

   ```text
   http://127.0.0.1:<reserved-local-port>/json/version
   http://127.0.0.1:<reserved-local-port>/json/list
   ```

   `/json/version` identifies the browser and protocol endpoint. `/json/list`
   provides page/tab targets with ids, titles, URLs, target types, and WebSocket
   debugger URLs where the browser supports them.
   The agent must connect only to a bridge it launched for the managed session.
   It must reject unmanaged bridge endpoints, default-profile bridges, wildcard
   remote origins, and stale session ids.

5. Capture browser evidence.

   For each supported target, record browser family, managed session id, browser
   process id, profile id/path reference, window id where available, tab id,
   active/inactive state where available, exact URL, normalized domain/origin,
   page title, timestamp, evidence id, source id, adapter id, and capability
   status. Store that evidence through the encrypted journal and SQLite query
   store before the portal or local AI consumes it.
   Active state must be `known-active`, `known-inactive`, or `unknown`.
   Evidence also records an active proof source: `target-list-only`,
   `cdp-focus-activation`, `managed-extension-event`,
   `foreground-correlation`, or `owned-shell-event`. Target-list-only evidence
   must stay `unknown`; it is not enough to claim an active or inactive tab
   unless the adapter has separate proof for focus/activation or an equivalent
   managed source.

6. Detect unmanaged browsers.

   Process/window capture keeps watching for browser-like processes. If
   `chrome.exe`, `msedge.exe`, `brave.exe`, Firefox, Opera, a portable browser,
   or another browser-like process is running outside the Ocentra-managed
   session, record unmanaged browser evidence with process id, process name,
   redacted executable path ref, signature/hash refs where available, family
   guess, process kind, confidence, and possible-bypass reason. This is not
   successful URL/tab evidence. The event must not include an exact URL, browser
   history, page body, cookies, form data, exact social account/route/feed/video
   fields, exact browser-game URL/title/account/purchase fields, or exact cloud
   gaming title fields.

7. Make the managed browser the normal child path.

   Parent/admin setup should create an Ocentra browser launcher and later may set
   the managed browser as the default browser. Existing normal browser tabs
   cannot be relied on for exact URL capture unless they are inside the managed
   browser boundary. Product behavior should be explicit: Ocentra-managed
   browser sessions provide exact URLs; unmanaged browser sessions are bypass
   evidence only. Android default-browser evidence must distinguish Device Owner
   persistent preferred activity policy mutation from Browser Role/default
   browser routing. Browser Role proof may show implicit VIEW routing to the
   owned shell, but it does not prove exact active-tab custody, final policy
   execution, broad content filtering, or silent default-browser assignment on a
   physical child device.
   Physical Android owned-shell current-runtime rows may be accepted only when
   the proof has install, explicit launch, UI-tree, and screenshot evidence from
   the physical target. Those rows still must not claim physical Device Owner,
   physical Browser Role routing, exact URL policy, active-tab proof, VPN/DNS,
   UsageStats, Accessibility, final policy execution, or enforcement.
   Physical Android owned-shell requested-URL custody rows may be accepted only
   when the same physical proof also has WebView/BROWSABLE declaration and
   local proof-page evidence. These rows may store a requested-URL ref only;
   they must not persist the raw URL or claim exact URL policy, active-tab
   custody, physical Device Owner/Browser Role routing, final policy execution,
   or enforcement.

8. Hand off stored evidence to portal, policy, and AI consumers.

   The portal reads browser status and recent activity through typed service read
   models. Policy and local AI receive browser evidence ids, timestamps,
   normalized domain/origin, URL/title summaries where allowed, capability
   status, and custody/source labels. They do not read browser profiles,
   DevTools endpoints, journal files, SQLite files, or raw protocol payloads.

## Acceptance

- The system can distinguish "Chrome is the foreground app" from "the active
  Chrome tab is https://example.com/".
- Active-tab claims have an exact URL, title, normalized domain, timestamp,
  evidence id, source id, adapter id, and explicit active proof source.
- Target-list-only evidence can show known tab rows but must keep active state
  `unknown` until a stronger focus/activation, extension, foreground
  correlation, or owned-shell proof source is implemented.
- Unsupported browsers and missing permissions are typed states, not silent
  failures.
- Managed install, profile, bridge, permission, stale, and degraded states are
  visible through contracts and portal read models.
- A normal or alternate browser running outside the managed Ocentra browser
  boundary is reported as unmanaged browser use; it is not counted as successful
  URL/tab capture.
- Browser evidence survives journal/query-store round trip before portal or AI
  use.
- Local AI input contracts can reference browser evidence by id.
- Managed intervention action and delivery proof can reference browser evidence
  by id without treating AI output as an enforcement authority.
- Unmanaged fallback action proof can reference process-only unmanaged browser
  evidence and policy/audit ids, but exact URL, social route, feed/video,
  browser-game, and cloud-gaming claims remain managed-browser-required or
  not-claimed unless later adapter proof adds those sources.
- AppLocker/App Control proof states can show readiness, audit-only, enforced,
  manual-required, unavailable, and failed status, but app-control prevention,
  policy create/update, rollback, and broad browser control remain unclaimed
  until Windows policy apply, refresh, audit, rollback, failure, and identity
  target artifacts are captured.
- Extension/native-host boundary proof can show managed-profile-only install,
  permission, origin, schema, length, heartbeat, and native-host-missing states,
  but runtime signal capture, unmanaged personal-profile capture, extension
  package install, native-host registration, and browser-game evidence remain
  unclaimed until separate artifacts prove those routes.
- Browser performance/service-health proof can show fixture-backed measured rows
  for inventory, support matrix, CDP target mapping, journal write, SQLite
  replay, unmanaged process scan, bridge reconnect, and memory/cache behavior,
  while hardware-specific release performance, portal render timing, URL/video
  provider behavior, local AI queue behavior, browser-game runtime signals, and
  cloud-gaming heuristics remain unclaimed until separately measured.
- Browser evidence artifact manifests can index existing JSON and screenshot
  proof for managed profiles, interventions, unmanaged Windows behavior,
  dry-run adapter behavior, and performance health, but partial/manual-required
  rows must remain gaps until fresh service, model/provider, parent-decision,
  runtime-signal, cloud-session, unsupported-adapter, and cross-platform
  artifacts exist.
- Browser rollout gates must preserve no-claim wording: base browser proof does
  not complete AI URL/video intelligence, social account/feed approval, or
  browser-game/cloud-gaming evidence until those enhancement tracks have
  separate contracts, runtime proof, UI/manual artifacts, and rollout gates.
- Browser URL shape classification contracts may identify deterministic URL
  shape, platform, and stable ids from exact managed-browser URL evidence, but
  they must not claim video content semantics, AI authority, policy authority,
  unmanaged exact URLs, or network-derived exact page activity.
- Browser URL shape parsers may deterministically map supported platforms into
  schema-valid shape rows, but unsupported schemes, credential-bearing URLs,
  unmanaged process rows, and network/domain rows must stay rejected,
  unknown/non-exact, or manual-required rather than promoted to exact content
  claims.
- Browser URL intelligence memory rows may drive policy input only when a fresh
  hit cites source evidence, policy version, expiry, and analysis or parent
  action refs. Miss, stale, and manual-required rows cannot drive policy input,
  and memory rows must never claim direct enforcement authority.
- Browser URL metadata evidence may carry browser-title, OpenGraph, schema.org,
  platform id, thumbnail, duration, publish date, captions availability, and
  platform label fields as structured evidence for AI input. It must not capture
  page body or transcript text in this contract, and it must not claim content
  semantics, AI decisions, policy decisions, hidden-analysis proof, or platform
  metadata as policy authority.
- YouTube metadata adapters must start from managed exact YouTube URL shape
  evidence before producing metadata evidence. Watch, Shorts, embed, live,
  channel, and playlist shapes may carry platform ids and metadata refs, but
  unmanaged process rows, generic web rows, raw transcript text, page body, and
  platform labels as authority remain invalid.
- Vimeo and generic video metadata adapters must start from managed exact URL
  shape evidence before producing metadata evidence. Vimeo page and player URLs
  may carry numeric platform video ids; generic web rows require schema.org
  VideoObject metadata before they can claim video metadata. Unmanaged process
  rows, generic OpenGraph-only rows, raw transcript text, page body, and platform
  labels as authority remain invalid.
- Dynamic social route evidence must start from managed exact URL shape evidence
  before it can classify social feed, social post, messaging, upload/post, or
  livestream routes. Route shape evidence may carry visible platform, route,
  query, and post ids, but it must not claim account identity proof, feed
  recommendation semantics, messaging/contact risk, upload monitoring,
  livestream content analysis, AI decisions, policy decisions, or direct policy
  authority. Dynamic feed TTL stale memory rows cannot drive policy input.
- Hidden managed analysis profile designs must use an Ocentra-owned profile
  separate from the child visible profile, bounded retention, timeout and
  structured-summary budgets, and no child cookies, child session tokens,
  autoplay audio, downloads, form submits, CAPTCHA automation, login bypass, or
  raw page-body retention. Metadata-only and analysis-ready states require a
  separate loader proof before they can be accepted.
- Hidden analysis loader adapters may only plan safe isolated designs into
  queued/loading or explicit manual-required states until a real loader proof
  exists. Loader results must reject page-body capture, transcript text capture,
  and metadata-only or analysis-ready states without a loader proof ref.
- Browser URL/video AI analysis inputs must reference schema-valid evidence,
  URL shape, metadata, memory, graph, parent rule, schedule, prompt template,
  model preference, and custody refs only. They must not include raw browser
  state, DevTools payloads, SQLite paths, journals, OS state, page body, or
  transcript text.
- Browser URL/video AI analysis outputs may recommend candidate policy input
  and carry category/modifier, benefit/risk, confidence, uncertainty, summaries,
  model runtime, and prompt template refs, but they must not claim final policy
  action, enforcement action, or raw content storage authority.
- Browser AI local-provider routing may select a child-device local runtime only
  when the request asks for local execution and the provider capability proves
  task support, local custody, no retention, child-device execution, and a model
  runtime ref. Missing model, unavailable provider, resource pressure,
  unsupported task, or manual-required states must be explicit and must not
  silently fall through to family hub or remote/API AI.
- Browser AI provider routes must expose data scope, retention, custody,
  provider identity, no-retention state, and audit evidence before use. Remote AI
  must not become default blocking, override stricter local parent rules, or
  disable local safety when remote service is down.
- Browser AI family hub routing must be local-household fallback only. It may
  select a family hub only after child-device local provider routing was
  attempted and did not serve, parent settings allow household hub routing, and
  the hub capability proves task support, no-retention custody, a household
  route ref, and a model runtime ref. It must not select remote providers, claim
  remote default blocking, or bypass the local provider path.
- Browser AI remote routing must be parent-approved before use. Remote approval,
  capability, and route evidence must expose structured data scope, no-retention
  mode, provider identity, explicit approval, local safety fallback, and audit
  evidence. It must not capture raw browser state, page body, transcript text,
  screenshots, or become default blocking authority.
- Browser AI remote outages must not disable local safety, and parent-approved
  remote routes must not override stricter local parent rules.
- Browser AI provider fallback decisions must be visible to parent and child
  surfaces, carry audit evidence, and select only the runtime proven by the
  selected local, family-hub, or parent-approved remote route. Metadata-only and
  no-AI fallbacks must keep runtime refs null and expose explicit fallback
  action/reason labels. Fallback decisions must not claim AI analysis results,
  final policy decisions, remote default blocking, disabled local safety, or
  hidden degraded/manual-required states.
- Browser URL/video AI proof gates must validate the enhancement-row proof pack
  matrix before rollout claims. A passing gate may prove that AI contract,
  parser, metadata, provider, policy-candidate, UX-state, and fallback proof
  artifacts are present and no-claim guard text is preserved, but it must still
  mark product rollout partial/manual-required until runtime model execution,
  UI delivery, package exports, policy authority, and enforcement proof exist.
- Browser AI prompt templates must be versioned by id/version and audit refs.
  Prompt version records must expose hash refs, compatible model/runtime refs,
  policy version refs, lifecycle state, supersession, and whether prompt changes
  invalidate memory.
- Browser AI prompt selection must use one active version for a task/model
  runtime and must fail closed to manual-required when the active template is
  missing, deprecated, retired, unsupported by the model, or unsupported by the
  policy version. Prompt contracts must not store raw prompt text or capture raw
  page body or transcript text.
- Browser AI category/risk/benefit assessments must keep category, modifier,
  benefit, risk, confidence, uncertainty, source support, and taxonomy version
  fields explicit. Education/homework/research classifications need benefit
  signals; high-risk classifications need matching risk signals.
- Platform category/rating labels may be evidence inputs, but they must not be
  used as policy authority. Structured assessments must not claim final policy
  action or enforcement authority.
- Browser URL/video analysis queue jobs must expose priority, status,
  parent-owned timeout policy, queued evidence ids, and structured AI input refs.
  Completed jobs may carry an AI result only when it matches the input request
  id.
- Browser URL/video queue timeout semantics must be explicit by priority:
  P0 strict-hold uses parent policy fallback quickly, P1 may warn or ask, P2 may
  become background-only, and P3-P5 may wait or degrade. Queue contracts must not
  claim queue processing, worker runtime, final policy action, or enforcement.
- Browser AI memory/cache entries must cite complete cache keys: model/prompt,
  policy version, child profile, and at least one content locator such as
  canonical URL, platform video id, or normalized origin/path. Fresh entries may
  drive policy input only through the existing memory-hit contract.
- Browser AI memory/cache entries must expose TTL class and invalidation reason.
  Dynamic feeds, search results, homepages, social feeds, and livestreams need
  short TTLs. Stale, expired, or invalidated entries cannot drive policy input,
  and cache snapshots must not store raw content or claim direct enforcement.
- Browser AI knowledge graph references must be evidence-backed node/edge
  bundles, not an authority layer. They may cite stored browser evidence,
  metadata, memory/cache, AI analysis, parent rule, external taxonomy, or
  parent-approved source refs, but they must not store raw content or use
  platform labels as policy authority.
- Browser AI knowledge graph bundles may support candidate policy input only
  when fresh, policy-versioned, explicitly marked for policy-candidate support,
  and free of low-confidence hidden state, stale refs, duplicate nodes, dangling
  edges, direct policy authority, or direct enforcement claims.
- Browser AI policy evaluator integration inputs may hand validated browser
  evidence, URL shape, metadata, AI result, memory/cache, graph, parent rule,
  schedule, child profile, and evaluator mode refs to the policy layer. They
  must not include raw model text, unvalidated AI output, portal UI state, final
  decision claims, or direct enforcement claims.
- Browser policy decision bundles that reference AI output must include evidence
  refs, parent rule refs, reason codes, audit refs, fallback visibility for
  unknown outcomes, and adapter proof before an active block decision is valid.
  They must not treat AI output, graph refs, portal state, or the browser
  evidence layer as direct enforcement authority.
- Browser AI post-analysis action plans must keep after-the-fact review honest:
  background review, continue, warning, stopped playback, parent approval,
  future block, remembered-with-expiry, manual-required, and no-action labels
  need evidence, policy decision, audit, timing, delivery, and adapter-proof refs
  where applicable.
- Browser AI post-analysis action plans must not claim real-time blocking after
  playback has already started. Delivered warning, stop, approval, or future
  block actions require adapter proof; remembered actions require an expiry; and
  unknown decisions require explicit manual or parent fallback action.
- Browser AI child-facing UX snapshots must use schema-known calm copy tokens
  rather than arbitrary raw child copy. Checking, warning, block, and approval
  child pages require adapter proof before they can be claimed as delivered, and
  warning/block/approval states must link to matching post-analysis actions.
- Browser AI parent explanation bundles must cite the same evidence carried by
  AI analysis, policy decision, and post-analysis action records. They must make
  model runtime, prompt version, policy rule, action, child-saw-page,
  degraded/manual fallback, and audit refs visible without including raw page
  content or raw prompt text.
- Tests prove invalid URLs, missing required ids, unsupported browsers,
  unmanaged browser detections, stale evidence, permission-limited states, and
  degraded bridge states are rejected or marked degraded.
- No page body, screenshots, keystrokes, browser secrets, or decrypted traffic
  are captured.

## Done Signal

A local run records real browser URL/tab evidence from a supported browser into
the journal and query store, shows it in the portal with honest capability
status, reports unmanaged browser use separately, and makes the evidence
addressable by local AI or policy contracts without claiming content inspection.
