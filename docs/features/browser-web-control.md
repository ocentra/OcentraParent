# Browser And Web Control

## Parent Outcome

Parents can see and control web activity by site, URL, domain, category,
schedule, and exception where the browser/source boundary is proved.

## Ocentra Requirement

Exact URL/tab knowledge requires a managed browser or proved browser bridge.
Process/window or network metadata alone cannot claim exact page activity.
Blocking requires typed policy decisions and adapter proof.

## Roadmap And Expectations

- Roadmap: V0.5.1 browser evidence, V0.8 enforcement, V5 policy product.
- Expectations: [browser evidence](../expectations/browser-evidence.md),
  [policy](../expectations/policy.md),
  [enforcement](../expectations/enforcement.md).
- Supporting docs:
  [raw 1,057-setting inventory](../browser-control-1057-settings-inventory.md)
  and
  [questionnaire forest v1](../browser-policy-questionnaire-forest-v1.md).
- Working plan:
  [browser plan](../plans/browser-plan/README.md), including the
  [source index](../plans/browser-plan/source-index.md),
  [current snapshot](../plans/browser-plan/current-browser-snapshot.md),
  [full scope plan](../plans/browser-plan/v0-5-managed-browser-full-scope-plan.md),
  [URL/video AI intelligence plan](../plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md),
  [social platform account/feed gating plan](../plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md),
  [browser games/cloud gaming gating plan](../plans/browser-plan/v0-5-browser-games-cloud-gaming-gating-plan.md),
  [test blueprint](../plans/browser-plan/v0-5-managed-browser-test-blueprint.md),
  [UI/UX guide](../plans/browser-plan/ui-ux-requirements-guide.md), and
  [workpacks](../plans/browser-plan/workpacks/01-contract-boundary-and-effect-schemas.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `packages/agent-protocol-domain`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
web filtering/categories, search/platform restrictions, and video safety.

Google, Apple, Microsoft, Qustodio, Norton, Net Nanny, Canopy, Bark, and others
offer web filtering or content restrictions. Ocentra must match parent-visible
control while being more honest about managed versus unmanaged sources.

## Current Ocentra State

- Managed browser URL/tab evidence direction and status contracts exist.
- Browser/domain adapter proof now uses surface-specific contract guards so
  managed-session intervention, managed exact-URL manual-required state,
  unmanaged process-only termination/warn state, and network/domain manual or
  unavailable state cannot be drifted into stronger claims by direct parsing.
- The V0.8 product-control spine now exposes managed browser session control,
  unmanaged process-only fallback, and managed/unmanaged exact URL gaps as
  separate parent-visible action states for downstream policy/device screens.
- The V0.8 product-control runtime path now exposes those browser states through
  a Rust service WebSocket read model and typed agent-protocol adapter that link
  back to browser/domain adapter proof and keep exact URL control
  manual-required or not-claimed.
- The V0.8 policy-dispatch proof now carries unmanaged browser process fallback
  as report-only with real evidence references, child reason codes, audit refs,
  and service-backed source state instead of exact URL claims.
- The V0.8 broad-adapter proof now exposes a service-backed WebSocket read
  model and typed protocol adapter where managed browser session support is an
  implemented boundary, managed exact URL remains manual-required, and
  unmanaged exact evidence remains not-claimed.
- The V0.8 supported-adapter runtime proof now keeps exact active-tab
  enforcement explicitly not-claimed while app/game and network observe-only
  supported boundaries are represented separately.
- The V0.8 enforcement integrity runtime audit now carries browser/web-related
  non-execution states through the supported-adapter event path: dry-run and
  observe-only states do not execute adapters, stale/wrong-device intents reject
  before execution, and exact active-tab enforcement remains unclaimed.
- Unmanaged browser states can be represented as possible bypass and
  process-only fallback, not exact URL/tab proof.
- Browser-plan WP06/WP07 now add managed profile store and managed
  Chrome/Edge launcher/session proof for local managed profiles, while bridge
  custody, exact URL actions, active-tab enforcement, intervention, UI, mobile,
  and cross-platform support remain separate proof gates.
- Browser-plan WP08/WP09 now add loopback bridge custody checks and
  fixture-backed CDP version/list parser hardening for tab-list-only evidence;
  exact active-tab enforcement and managed URL actions remain unclaimed.
- Browser-plan WP10 now maps fixture-backed CDP target rows into normalized
  schema-valid tab evidence with stable derived tab ids, credential-stripped
  URL/origin/domain evidence, explicit degraded reason/query visibility, and
  journal/read-model round-trip coverage; exact active-tab proof and managed URL
  actions remain separate gates.
- Browser-plan WP10 now maps managed bridge observations into schema-valid tab
  evidence with normalized URL/origin/domain fields, stable identity/custody
  fields, tab id derivation, and journal/read-model proof; exact active-tab
  enforcement and managed URL actions remain unclaimed.
- Browser-plan WP11 now adds explicit active-tab proof-source labels across
  TypeScript contracts, Rust protocol/read models, service payloads, and portal
  parsing. Target-list-only evidence remains `unknown` and cannot be promoted to
  known-active or known-inactive without a later focus/activation proof source.
- Browser-plan WP12 now makes service activity capture journal-first for browser
  evidence ingest: events are appended to the encrypted journal and then replayed
  from appended journal lines into SQLite/read models, with duplicate id and
  restart proof for stable browser evidence rows.
- Browser-plan WP13 now exposes a typed browser inventory read-model get command
  and replayable reported event through the service WebSocket path. The payload
  keeps managed, unmanaged, active-tab, exact-URL, stale, degraded, and custody
  labels separate; it does not upgrade portal UI, SQLite inventory storage, or
  real browser/platform proof claims.
- Browser-plan WP03 now carries publisher-signature and file-hash evidence refs
  through activity-domain inventory contracts, Rust protocol, and service
  payload/read-model proof. Contract tests cover mixed managed, unmanaged, and
  unsupported catalog rows and reject empty identity refs. This does not upgrade
  live OS scanning, live signature/hash extraction, portal dashboard rendering,
  exact URL evidence, or blocking claims.
- Browser-plan WP04 default-root service proof now feeds the service inventory
  read-model scan with default Windows candidate roots before process
  observations. Fixture proof shows a default-root Edge install becomes a
  managed candidate row with exact URL still unavailable, without live registry,
  Start Menu, `.lnk`, AppX/MSIX, signature, UI, or enforcement claims.
- Browser-plan WP14 now surfaces the browser inventory read model in the parent
  portal route through service-backed inventory, exact URL capability, active-tab
  proof, and unmanaged fallback rows. The visible route shows inventory, exact
  URL capability, and active-tab proof cards, while unmanaged fallback remains
  report-only/not-claimed in domain proof; no social/video/game, child-facing,
  intervention-action, real platform, or exact active-tab enforcement claim is
  upgraded.
- Browser-plan WP15 now keeps browser policy authoring manifest coverage
  schema-owned for managed Chrome/Edge policy-writer inputs, URL allow/block
  lists, and browser-game policy questions. Contract proof covers manifest-owned
  patch checks, invalid policy-writer enum rejection, and browser-game
  questionnaire visibility; policy writing and enforcement remain
  manual-required/not-claimed until adapter proof exists.
- Browser-plan WP16 now compiles browser policy targets into typed effective
  rule result fields: target-proof requirement, capability state, action
  execution, AI authority, compile note, and deterministic parent-policy action.
  Exact URL, domain, classifier, social-route, browser-game, process-detection,
  policy-writer, adapter-action, observe/dry-run, and AI-candidate paths stay
  separated so unsupported states remain manual-required instead of becoming
  enforcement claims.
- Browser-plan WP17 now wires managed intervention action/audit/evidence refs
  and child delivery state through TypeScript contracts, Rust protocol,
  journal/read-model replay, service payloads, portal parsing, and a real
  Chrome/Firefox/Edge managed-browser proof harness. The harness proves block,
  warning, approval-hold, and checking-hold pages for site, video, social
  signup, social short-video, browser-game, game-purchase, and cloud-gaming
  targets while keeping unmanaged browser exact URL evidence, broad OS browser
  blocking, native app/game control, cloud-streamed frame analysis, and final
  child UX polish unclaimed.
- Browser-plan WP18 now carries unmanaged browser detection as process-only
  evidence through TypeScript contracts, Rust protocol/runtime status, service
  inventory read models, and portal status parsing. Supported, unsupported,
  portable, packaged, Tor/privacy, embedded, unknown browser-like, possible
  social bypass, possible browser-game bypass, and possible cloud-gaming bypass
  classifications remain bypass/process evidence only; exact URL/tab,
  social-account, social-route, feed/video, browser-game URL/title/account, and
  cloud-title claims are rejected or omitted.
- Browser-plan WP19 now exposes unmanaged fallback action states through
  TypeScript contracts, Rust protocol/activity-store read models, service
  payloads, policy compiler action execution states, and parent portal parsing.
  Report-only, warn-child, parent-review, terminate-process, relaunch managed
  browser, OS-block configured/manual-required, allowed unmanaged exception,
  degraded, and unavailable states stay separate from exact URL/tab evidence.
  Social and browser-game unmanaged use remains process/bypass evidence and
  managed-browser-required state, not exact account/feed/video/game proof.
- Browser-plan WP20 now represents Windows AppLocker/App Control proof states in
  the V0.8 browser/domain adapter proof model and Rust service read model:
  readiness-check, audit-only, enforced, manual-required, unavailable, and
  failed. Publisher/path/hash/package identity, administrator/manual/service
  permission requirements, and audit/rollback/failure event states are
  explicit, while launch prevention, policy creation/update, rollback, unmanaged
  exact URL, host network/domain blocking, and broad browser control remain
  unclaimed until real Windows artifacts exist.
- The V0.8 browser/enforcement timer recovery proof now adds a parent-domain
  read model, focused contract tests, Rust timer-state tests, and proof harness
  evidence for timer create/restart-recovered/cancel/recovery-needed/expired
  paths plus rollback-completed and rollback-unavailable states. The same proof
  keeps unmanaged-browser fallback process-scoped: report-only, warn-child,
  parent-review, terminate-process, relaunch-managed manual-required, degraded,
  and unavailable states remain separate from exact URL/tab/title/content
  claims.
- Browser-plan WP21 now adds an optional extension/native-host boundary
  contract and private Rust native-message validator. The boundary keeps
  extension install/enabled/disabled/permission-required/native-host-missing,
  minimum permission, managed-profile binding, origin/schema/length validation,
  heartbeat freshness, and runtime-signal-proof-required states explicit, while
  rejecting personal-profile capture, unmanaged-profile capture, and
  runtime-signal capture claims.
- Browser-plan WP22 now adds a typed browser performance/service-health read
  model and private Rust budget evaluator. Fixture gates cover inventory scan,
  support-matrix derivation, 100-tab CDP target mapping, journal write per
  event, 10000-event SQLite replay, unmanaged process scan, rapid bridge
  reconnect, and memory/cache lookup invalidation. Portal 100-tab rendering,
  URL/video metadata extraction, local AI queue timeout, browser-game runtime
  signals, and cloud-gaming heuristics remain manual-required without runtime
  claims.
- Browser-plan WP23 now adds a browser evidence artifact manifest that indexes
  existing managed profile, intervention, unmanaged Windows, dry-run adapter,
  and performance proof files. The manifest distinguishes artifact-present
  managed Edge/Chrome, unmanaged bypass, policy dry-run, managed block-page, and
  Windows rows from partial/manual URL-video, social, browser-game, and
  cloud-gaming rows, plus manual-required bridge-stale, unsupported-adapter, and
  non-Windows/mobile matrices.
- The raw browser setting inventory and reduced questionnaire forest are now
  preserved as design inputs, not product-complete implementation proof.

## Current Gap

Managed-browser exact URL action, category filtering, child-facing warning UX,
OS-level unmanaged browser blocking proof, and parent-facing rule UX are not
product-complete.
The broad-adapter proof adds runtime visibility for those states but does not
upgrade exact URL, unmanaged exact evidence, or host domain blocking claims.
Policy dispatch and supported-adapter runtime proof preserve the report-only or
not-claimed boundary, not active tab enforcement. The integrity runtime audit
adds proof that dry-run, observe-only, rejected, unavailable, and manual-required
states stay non-executing, but it still does not prove managed exact URL or
active-tab enforcement.
Timer recovery and unmanaged fallback now have focused V0.8 proof, including
parent-visible next-check/failure states and process-scoped fallback outcomes,
but that proof does not upgrade exact page evidence, host domain blocking,
child-facing warning UX, mobile browser control, or OS-level launch-prevention
claims.
Windows AppLocker/App Control states are represented for readiness, audit-only,
enforced, manual-required, unavailable, and failed paths, but this is not
production launch-prevention proof until real Windows policy apply, refresh,
rollback, audit, failure, and identity-target artifacts exist.
Extension/native-host support is still optional and not product-enabled: no
extension package, native-host registration, runtime signal capture, unmanaged
personal-profile capture, or browser-game evidence route is claimed until those
artifacts are separately packaged, permissioned, installed, and proved.
Browser performance/service-health now has fixture gates for contract and
budget evaluator behavior, but release performance remains manual-required until
hardware-specific runs, portal stress timing, URL/video provider behavior, local
AI queue behavior, browser-game runtime signals, and cloud-gaming heuristics are
separately measured through real runtime artifacts.
Browser evidence artifact coverage is now indexed, but missing or partial rows
remain gaps: bridge disconnect stale proof, unsupported/later-adapter proof,
macOS/Linux/Android/iOS matrices, URL/video model/provider classification,
social parent-decision/audit flow, browser-game runtime signals, and
cloud-gaming session heuristics are not upgraded by the manifest.
Browser-plan WP24 now records the base workpack rollout gate. It closes the
base browser-plan proof/checklist path only; AI, social/video, and browser-game
enhancement tracks remain open/manual-required until their separate contracts,
runtime proof, UI/manual artifacts, and rollout gates exist.
Browser-game/cloud-gaming GAME-01 now adds a focused browser-plan workpack
README and proof-root map. This records row ownership and no-claim boundaries
only; browser-game contracts, runtime signals, metadata, AI, parent policy,
child/parent UI, native app/game control, cloud-streamed frame analysis, and
enforcement remain open/manual-required.
Browser-game/cloud-gaming GAME-02 now adds parent-domain platform/route
contracts. Rows model browser-game portals, educational game sites, UGC game
platforms, cloud-gaming/cloud PC platforms, classic game archives, school game
platforms, and unknown/manual-required platforms through route surface kinds,
route source kinds, custody labels, pattern refs, evidence refs, confidence, and
status. They do not store raw domains, URLs, paths, page bodies, claim live URL
parsing, runtime detection, AI classification, policy decisions, native game
control, cloud-frame analysis, or enforcement.
Browser-game/cloud-gaming GAME-03 now adds parent-domain portal pattern library
contracts. Entries model known-game, educational, UGC, indie, classic archive,
school, and unknown portal families through route kinds, signal kinds, pattern
fingerprints, evidence refs, confidence, and review states. They do not store
raw domains, URLs, page titles, page bodies, claim runtime detection, AI
classification, policy decisions, cloud-gaming ownership, or enforcement.
Browser-game/cloud-gaming GAME-04 now adds parent-domain cloud-gaming pattern
library contracts. Entries model cloud-gaming platforms, cloud PC platforms,
mobile cloud-game portals, browser-embedded cloud-game surfaces, native launcher
prompt bridges, and unknown/manual-required cloud surfaces through route kinds,
signal kinds, pattern fingerprints, evidence refs, confidence, and review
states. They do not store raw cloud domains, URLs, titles, stream frames, claim
runtime detection, inspect cloud-streamed frames, claim per-game cloud-title
certainty, control native launchers or games, make final policy decisions, or
enforce actions.
Browser-game/cloud-gaming GAME-05 now adds a parent-domain redacted URL-shape
parser. It accepts unknown input, uses URL parsing only transiently, emits shape
and fingerprint fields, and does not store raw URLs, domains, paths, queries, or
fragments. It does not navigate browsers, claim runtime detection, run AI
classification, decide policy, inspect cloud frames, control native games, or
enforce actions.
Browser-game/cloud-gaming GAME-06 now adds parent-domain runtime signal detector
contracts. Signal rows model shape-only canvas, WebGL, Gamepad API, fullscreen,
pointer-lock, audio, animation-loop, iframe surface, cloud-streaming, and
unknown/manual-required states through fingerprints and evidence refs. They do
not store raw runtime data, instrument browsers, execute runtime detection, run
AI, decide policy, inspect cloud frames, control native games, or enforce
actions.
Browser-game/cloud-gaming GAME-07 now adds parent-domain metadata extractor
contracts. Field rows model redacted title, description, genre, age-rating,
publisher, thumbnail, educational subject, cloud platform title, and
unknown/manual-required metadata shapes through fingerprints and evidence refs.
They do not store raw metadata, scrape runtime DOM, call platform APIs, run AI,
decide policy, inspect cloud frames, control native games, or enforce actions.
Browser-game/cloud-gaming GAME-08 now adds parent-domain hidden analysis profile
safety contracts. Profile and loader rows model Ocentra-owned isolated profiles,
bounded retention, proof-backed metadata-only/analysis-ready states,
disabled-policy, proof-missing, manual-required, and unavailable states. They do
not reuse child cookies or sessions, share child storage, store or capture raw
URL/page/game/frame payloads, instrument browsers, control hidden native
surfaces, run AI, decide policy, render UI, inspect cloud frames, control native
games, or enforce actions.
Browser-game/cloud-gaming GAME-09 now adds parent-domain educational classifier
contracts for browser games. The classifier uses evidence refs for school URLs,
teacher/parent allowlists, metadata, AI classification refs, parent approval,
homework context, school platforms, platform self-labels, and manual-required
states, then returns candidate gate inputs only. It does not treat platform
labels as authority and does not claim raw page/game/model capture, final policy
decisions, runtime gates, UI rendering, native game control, cloud-frame
analysis, or enforcement.
Browser-game/cloud-gaming GAME-10 now adds parent-domain AI analysis contracts
for browser games. Inputs consume typed evidence refs only for browser
evidence, URL shape, runtime signals, metadata, screen summaries, parent rules,
recent activity, memory, task, and custody labels. Results model game
classification, educational check, risk classification, cloud-gaming detection,
UGC risk, purchase risk, and policy-support outputs as candidate-only signals,
recommended policy input, confidence, uncertainty, summary refs, model runtime
refs, prompt template version, expiry, and degraded/manual states. They do not
store raw URLs, page body, game payloads, screen frames, model text, execute
account/purchase flows, control native games, inspect cloud frames, render UI,
make final policy/runtime decisions, or enforce actions.
Browser-game/cloud-gaming GAME-11 now adds parent-domain browser-game
risk/benefit signal contracts. The signal set covers evidence-backed risk rows
for violence, horror, adult themes, addictive loops, multiplayer/contact, chat,
purchase, loot box/random item, UGC, privacy, unblocked-bypass, and unknown
risk; and benefit rows for educational value, homework relevance, skill
building, creativity, problem solving, parent-approved game, neutral, and
unknown benefit. It returns candidate recommended policy inputs only and rejects
raw game payloads, chat content, page body, raw model text, account/purchase
execution, cloud-frame analysis, native game control, final policy decisions,
runtime gate execution, and enforcement.
Browser-game/cloud-gaming GAME-12 now adds parent-domain memory/cache contracts
for browser-game decision refs. Cache keys are schema-backed refs or hashes for
canonical URL, platform game, domain path, cloud title, parent decision, game
category, policy version, child profile, parent rule set, and evidence. Fresh
hits can feed policy input only when bounded TTL, required subject keys,
evidence refs, and decision refs are present; stale, miss, and manual-required
rows cannot drive policy input. It does not store raw URLs, raw game IDs, raw
cloud titles, raw game payloads, raw model text, or claim runtime cache store,
AI cache, UI, native game control, cloud-frame analysis, final policy, or
enforcement.
Browser-game/cloud-gaming GAME-13 now adds parent-domain browser-game
account/signup/purchase gate contracts. The contracts cover account creation,
login, secondary account, purchase, subscription, loot box/random item, virtual
currency, download/install, wallet/gambling-like payment, cloud-gaming start,
and unknown-game start approval states as evidence-backed request/decision
candidates only. They reject raw URLs/titles/account identifiers, credentials,
form submission, account creation, purchase/payment execution, launcher
downloads, notifications, rendered UI, child notification, final policy
decisions, runtime gate execution, native game control, cloud-frame analysis,
and enforcement.
Browser-game/cloud-gaming GAME-14 now adds parent-domain cloud-gaming gate
contracts. The contracts cover known cloud domains, streaming session routes,
gamepad/fullscreen/high-bandwidth/low-latency signal refs, optional platform
title/rating metadata refs, unknown cloud-game approval, mature cloud-game
block candidates, school-night blocks, time-budget candidates, manual-required
content-frame gaps, and unavailable platform proof states while rejecting raw
cloud titles, raw stream frames, cloud-streamed frame analysis, per-game cloud
title claims, native game/launcher control, game chat content, account/purchase
flows, notifications, rendered UI, child notification, final policy decisions,
runtime gate execution, and enforcement.
Browser-game/cloud-gaming GAME-15 now adds parent-domain unblocked-site
detection contracts. The detection models managed routes/pages, search intent,
portal indexes, iframe embeds, proxy/mirror routes, hidden game origins, school
bypass language, unmanaged browser process-only bypass evidence,
manual-required states, and unavailable states. Candidate actions remain
block-during-school, parent-review, allow-specific-game, block-unknown-iframe,
bypass-evidence-only, manual-review, or unknown inputs. It does not store raw
URLs, raw page body, raw search queries, captured iframe content, exact
unmanaged URLs, native game control, cloud-frame analysis, account/purchase
flows, rendered UI, final policy/runtime decisions, or enforcement.
Browser-game/cloud-gaming GAME-16 now adds parent-domain UGC/multiplayer/chat
risk contracts. The assessment covers UGC pages, experience pages, lobbies,
profile/friends/message routes, launch prompts, and web-to-app launch surfaces,
then returns candidate controls for approved experiences, parent approval, chat
blocking where capability refs exist, time limits, purchase approval, unknown
UGC blocking, manual review, or unknown state. It does not read chat content,
store profile/account/experience identifiers, execute web-to-app launches or
purchases, control native games, claim final policy/runtime/UI delivery, or
enforce actions.
Browser-game/cloud-gaming GAME-17 now adds parent-domain candidate-only policy
compiler contracts for browser-game evidence, analysis, mobile capability,
parent rule, and schedule refs. These candidates are not final policy decisions,
runtime gate executions, UI delivery, native game control, cloud-frame analysis,
or enforcement.
Browser-game/cloud-gaming GAME-18 now adds parent-domain managed browser-game
hold/block adapter contracts that link policy candidate refs, child UX refs,
managed intervention adapter proof refs, and audit refs for hold, approval,
block, and warn paths. Candidate-only allow/time-limit, manual-required cloud,
and unavailable native/unmanaged rows remain non-executing, and the contracts
reject raw URL/page/game payloads, child cookie/session reuse, unmanaged exact
URL claims, browser mutation, rendered child pages, notification delivery,
final policy decisions, applied time limits, cloud-frame analysis, native game
control, and enforcement.
Browser-game/cloud-gaming GAME-19 now adds parent-domain child checking/block
UX contracts for browser games. The contracts cover unknown-game checking,
parent approval, blocked candidates, educational allowed messaging, time-limit
candidates, cloud-gaming manual-required state, and native game control
unavailable state while rejecting raw child copy, rendered child UI,
notification delivery, runtime browser blocking, block-page rendering, applied
time limits, final policy decisions, cloud-frame analysis, native game control,
and enforcement.
Browser-game/cloud-gaming GAME-20 now adds parent-domain parent dashboard UX
contracts for browser-game review surfaces. The contracts cover detected game
review, unknown-game approval queue, cloud-gaming approval, educational-game
allowlist, game time-budget candidates, mobile/native capability gaps, and
manual-required gaps while rejecting rendered portal UI, notification delivery,
runtime data fetch, final policy decisions, cloud-frame analysis, native game
control, and enforcement.
Browser-game/cloud-gaming GAME-21 now adds parent-domain journal/SQLite
read-model contracts for browser-game proof refs. The snapshot indexes managed
browser evidence journal replay, app-game session report proof, adapter audit
refs, manual-required cloud rows, and unavailable native/unmanaged rows while
rejecting raw URL/page/game/title/account/purchase storage, child session reuse,
cloud title certainty, browser mutation, rendered UI, final policy decisions,
and enforcement.
Browser-game/cloud-gaming GAME-22 now adds a proof artifact gate that checks
GAME-01 through GAME-21 proof-pack coverage and marks Playwright
manual-required because this slice does not render browser-game UI. This does
not claim screenshots, runtime browser-game detection, cloud-streamed frame
analysis, native game control, final policy execution, enforcement, or product
checklist status.
Browser-game/cloud-gaming GAME-23 now adds parent-domain Android/iOS capability
matrix contracts for mobile browser-game surfaces. Android and iOS rows remain
manual-required, token-limited, entitlement-required, app-level, or domain-level
only until real device/platform proof exists, and they do not claim exact game
content, cloud-streamed frame analysis, native game control, UI delivery, or
enforcement.
Browser-game/cloud-gaming GAME-24 now labels the game track
partial/manual-required through the rollout gate. Product checklist upgrade is
not claimed. GAME-01 is scaffold-proof-present and GAME-02 through GAME-24 are
partial/manual-required. Route, runtime, metadata, AI, memory, UI,
cloud-streamed frame-analysis, native-control, and enforcement proof still need
separate release-grade artifacts before product completion can be claimed.
Browser AI enhancement rows AI-01 and AI-02 now add plan linkage and
schema-backed URL shape classification contracts. These contracts classify URL
shape, platform, and stable ids only from exact managed-browser URL evidence and
reject content semantics, AI decision, policy decision, unmanaged process, and
network/domain exact-page claims. They do not add a runtime parser, AI provider,
policy evaluator, child/parent UI, or enforcement behavior.
AI-03 now adds the deterministic URL parser helper for those contracts. It can
map supported URL shapes such as YouTube video/Shorts/channel/search, Vimeo,
TikTok, and generic web rows, but it still does not extract metadata, run AI,
decide policy, render UI, or enforce actions.
AI-04 now adds browser URL intelligence memory contracts for hit, miss, stale,
and manual-required rows. Fresh memory hits may only become policy input when
they cite source evidence, policy version, expiry, and analysis or parent
action refs. Stale, miss, and manual-required rows cannot drive policy input,
and memory rows never enforce directly.
AI-05 now adds browser URL metadata evidence contracts for managed browser
title, OpenGraph/schema.org/platform metadata, thumbnail refs, duration, publish
date, captions availability, and platform labels. These rows are evidence for
AI input only; they reject page-body capture, transcript text capture,
hidden-analysis metadata without proof, AI decisions, policy decisions, and
platform metadata as policy authority.
AI-06 now adds hidden managed analysis profile design contracts. They require an
Ocentra-owned profile separate from the child visible profile, bounded
retention, timeout/summary budgets, no child cookies or session tokens, and no
autoplay/download/form-submit/CAPTCHA/login-bypass/raw-page-body claims.
Metadata-only and analysis-ready states remain gated on a later loader proof.
AI-07 now adds a typed hidden analysis loader adapter boundary. It can plan a
safe queued design into a loading state or return manual-required for disabled
or unavailable capability, but it still does not load a real hidden browser,
capture page body/transcript text, produce metadata-only or analysis-ready proof,
run AI, decide policy, render UI, or enforce actions.
AI-08 now adds browser URL/video AI analysis input and output contracts. Inputs
are structured refs only: browser evidence, URL shape, metadata, memory, graph,
parent rule, schedule, prompt template, model preference, and custody. Outputs
can carry category/modifier, benefit/risk, confidence, uncertainty, summaries,
model/runtime refs, and candidate policy input, but they reject final policy
actions, enforcement actions, raw browser state, raw content storage, DevTools
payloads, SQLite paths, journals, and OS state. No provider routing, local/remote
AI execution, UI, policy evaluator, enforcement, or product checklist upgrade is
claimed by this contract slice.
AI-09 now adds local AI provider routing contracts. The route planner selects a
child-device local runtime only when the AI-08 request prefers local execution
and the local provider capability supports that task with no-retention custody.
Unavailable, missing-model, unsupported-task, and manual-required states remain
explicit and do not fall through to family hub or remote/API AI. Routes also
reject hidden data-scope/retention/custody/provider visibility, remote default
blocking, remote override of stricter local rules, and remote outages disabling
local safety. No real model execution, family hub, remote provider, queue,
policy evaluator, UI, enforcement, or product checklist upgrade is claimed.
AI-10 now adds family AI hub routing contracts. The route planner can select a
local-household family hub only after local provider routing was attempted and
did not serve, parent settings allow household hub routing, the request is
local-preferred, and the hub capability proves task support, no-retention
custody, a household route ref, and a model runtime ref. It rejects remote
provider selection, remote default blocking, hidden visibility, and attempts to
use the family hub before the local provider path is exhausted. No LAN discovery
protocol, relay, source matrix, real hub runtime, queue, remote provider, UI,
policy evaluator, enforcement, or product checklist upgrade is claimed.
AI-11 now adds parent-approved remote AI boundary contracts. Remote approval,
capability, and route schemas require explicit parent approval, visible
structured data scope, no-retention mode, provider visibility, local safety
fallback, and audit evidence before a remote route can be selected. They reject
raw browser state, page body, transcript text, screenshots, remote default
blocking, remote override of stricter local rules, and remote outage disabling
local safety. No remote provider/API call, model execution, queue, policy
evaluator, UI, enforcement, or product checklist upgrade is claimed.
AI-12 now adds prompt/template versioning contracts. Version records carry
template id/version, hash refs, change refs, compatible model/runtime refs,
policy version refs, audit evidence, lifecycle state, supersession, and memory
invalidation requirements. Registries reject duplicate active prompt versions
for the same task/model runtime, and selection returns manual-required rather
than choosing a deprecated, unsupported-model, unsupported-policy, or missing
template. No prompt content storage, model execution, policy evaluator, UI,
enforcement, queue, memory store, or product checklist upgrade is claimed.
AI-13 now adds structured category/risk/benefit model contracts. The taxonomy
makes content categories, modifiers, benefit signals, risk signals, unknown
fallbacks, and taxonomy version refs first-class. Assessments require evidence
refs, confidence/uncertainty visibility, matching risk signals for high-risk
categories, meaningful benefit signals for education/homework/research, and
candidate-only state. Platform labels cannot be used as authority, and final
policy action or enforcement claims are rejected. No model execution, policy
evaluator, UI, enforcement, queue, memory store, or product checklist upgrade is
claimed.
AI-14 now adds URL/video analysis queue contracts. Jobs carry structured AI
input refs, priority, status, parent-owned timeout policy, queued evidence ids,
and optional matching results. P0 strict-hold, P1 active unknown video, P2
active normal URL, and P3-P5 background priorities have explicit timeout
dispositions. Completed jobs must match the input request id, and
queued/running/degraded/timeout states cannot carry results or claim worker
runtime, final policy action, or enforcement authority. No queue processor,
model execution, policy evaluator, UI, enforcement, memory store, or product
checklist upgrade is claimed.
AI-15 now adds memory/cache store contracts. Cache entries wrap existing
browser URL intelligence memory hits with complete cache keys, TTL classes,
invalidation reasons, bounded retention, raw-content rejection, and no direct
enforcement authority. Fresh entries must include model/prompt, policy, child
profile, and content locator keys before driving policy input; stale or
invalidated entries cannot drive policy input. Dynamic feeds, search results,
homepages, social feeds, and livestreams remain short-TTL. No persistent DB,
cache worker, model execution, policy evaluator, UI, enforcement, or product
checklist upgrade is claimed.
AI-16 now adds browser AI knowledge graph reference contracts. Graph bundles
carry only evidence-backed node and edge refs over stored browser evidence,
metadata, memory/cache, AI analysis, parent rule, external taxonomy, or
parent-approved source refs. Fresh graph bundles may support candidate policy
input only with a policy version and explicit policy-candidate use; stale,
low-confidence, platform-label-only, raw-content, direct-authority, duplicate
node, or dangling-edge bundles are rejected. No graph store, graph builder,
model execution, policy evaluator, UI, enforcement, or product checklist upgrade
is claimed.
AI-17 now adds browser AI policy evaluator integration contracts. Evaluator
input bundles can hand validated browser evidence, URL shape, metadata, AI
result, memory/cache, graph, parent rule, schedule, child profile, and mode refs
to a policy evaluator, but they reject raw model text, unvalidated AI output,
portal UI state, final decision claims, and direct enforcement claims. Decision
bundles require evidence refs, parent rule refs, reason codes, audit refs,
fallback visibility, and active-block adapter proof while rejecting AI, portal,
or direct enforcement authority. No evaluator runtime, parent-domain policy
engine, UI, enforcement, or product checklist upgrade is claimed.
AI-18 now adds browser AI post-analysis action model contracts. Action plans
label background review, continue, warning, stopped playback, parent approval,
future block, remembered-with-expiry, manual-required, and no-action outcomes
after policy review. They require source evidence, AI analysis, policy decision,
audit, parent rule, timing, delivery, adapter proof, remember-expiry, and action
audit refs as applicable. The contracts reject real-time blocking claims after
playback has started, runtime mutation/direct enforcement claims, delivered
warning/stop/approval/future-block actions without adapter proof, remembered
actions without expiry, and unknown decisions without manual or parent fallback
action. No child UI, parent UI, browser runtime mutation, enforcement, or
product checklist upgrade is claimed.
AI-19 now adds browser AI child-facing checking/warning UX state contracts and
text-domain copy tokens. Child snapshots use schema-known calm copy tokens for
opening, checking, allowed, warning, approval-required, limited, blocked,
unclassified, manual-required, and unavailable states, link evidence and
post-analysis actions where applicable, and require adapter proof before claiming
delivered checking, warning, block, or approval pages. They reject raw
child-facing copy, shaming/surveillance copy claims, visual-render claims,
state/token mismatches, rendered child pages without adapter proof, and
warning/block/approval states without matching post-analysis actions. Activity-domain and text-domain package subpath exports are now present; no visual UI, browser page renderer, runtime delivery, enforcement, or product checklist upgrade is claimed.
AI-20 now adds browser AI parent explanation/audit UX contracts and text-domain
parent explanation tokens. Explanation bundles link evidence, AI analysis,
policy decision, post-analysis action, child UX snapshot, memory/cache refs,
graph refs, and audit refs while making evidence, model runtime, prompt version,
policy rule, action, child experience, child-saw-page, degraded/manual fallback,
and audit visibility explicit. They reject raw page content, raw prompt text,
portal evaluation, policy authority, direct enforcement, hidden fallback, hidden
child engagement, missing audit sections, and mismatched source evidence. Activity-domain and text-domain package subpath exports are now present; no parent UI component, portal visual rendering, runtime delivery, enforcement, or product checklist upgrade is claimed.
AI-21 now extends deterministic YouTube parser coverage, adds a YouTube metadata
adapter, and has a live proof harness against a real public YouTube watch page
plus YouTube oEmbed metadata. Managed exact YouTube watch, Shorts, embed, live,
channel, and playlist URLs can produce schema-backed shape rows, and exact
YouTube video, short, channel, or playlist classifications can produce metadata
evidence with title, description, platform ids, channel name, thumbnail refs,
duration, publish date, captions/transcript availability, category/rating/
restricted signals, and degraded reasons. The live proof persists only status
codes, marker booleans, public platform id, hashes, lengths, and no-claim flags;
it does not persist raw watch-page HTML, raw page body, transcript text, cookies,
tokens, local storage, or raw title/description strings. Unmanaged or
non-YouTube classifications are rejected, and the adapter does not claim content
semantics authority, AI decisions, policy decisions, or policy authority.
Activity-domain package subpath exports are now present; no production metadata
fetcher/scheduler, transcript parser, hidden page load, AI execution, policy
evaluator, UI, enforcement, or product checklist upgrade is claimed.
AI-22 now extends deterministic Vimeo parser coverage and adds a Vimeo/generic
video metadata adapter. Managed exact Vimeo page and player URLs with numeric
video ids can produce schema-backed shape rows, and exact managed Vimeo video or
generic web schema.org VideoObject rows can produce metadata evidence with
title, description, platform video id, channel name, thumbnail refs, duration,
publish date, captions/transcript availability, category/rating/restricted
signals, and degraded reasons. Live proof now fetches real public Vimeo page
and player surfaces plus a real public generic VideoObject page, exercises the
built activity-domain parser/adapter, and persists only statuses, hosts, hashes,
lengths, platform ids, and no-claim flags. Unmanaged classifications and
generic OpenGraph-only rows are rejected, and the adapter does not capture page
body, transcript text, content semantics authority, AI decisions, policy
decisions, or policy authority. Activity-domain package subpath exports are now
present; no production network fetcher, transcript parser, hidden page load, AI
execution, policy evaluator, UI, enforcement, or product checklist upgrade is
claimed.
AI-23 now extends deterministic dynamic feed/social URL handling with live route
proof. The URL shape contract can represent social post, messaging,
upload/post, and livestream route targets alongside social feed, and the parser
recognizes visible route shapes for Instagram, TikTok, Facebook, Twitch,
X/Twitter, Reddit, and Discord only from managed exact URL evidence. The live
proof fetches real public route surfaces for those platforms and persists only
response statuses, content types, lengths, route/path/query/body hashes, title
hashes/lengths, redirect host/path hashes, typed classifications, and no-claim
flags. Dynamic feeds and social route rows carry dynamic-feed or social-route
reasons with medium/low confidence; exact reels/status/posts carry post ids
where visible. Unmanaged social rows remain unknown/non-exact, dynamic-feed TTL
stale memory rows cannot drive policy input, and parser rows keep content
semantics, AI decisions, and policy decisions false. Activity-domain package
subpath exports are now present; no account identity proof, feed recommendation
analysis, messaging/contact analysis, upload monitoring, livestream content
analysis, UI, enforcement, or product checklist upgrade is claimed.
AI-24 now adds provider degraded/fallback decision contracts for the browser AI
route chain. Decisions can select child-device local AI, family AI hub, or
parent-approved remote AI only when the selected route and runtime match the
existing route proof. Metadata-only and no-AI fallbacks keep runtime refs null
and expose fallback action/reason/audit refs. The contracts reject hidden
fallback, claimed AI analysis results, claimed policy decisions, disabled local
safety, remote default blocking, and remote outages disabling local safety.
Activity-domain package subpath exports are now present; no model execution, policy evaluator, UI, enforcement, runtime
delivery, or product checklist upgrade is claimed.
AI-25 now adds a URL/video AI proof-gate script and rollout manifest. The gate
checks AI-01 through AI-24 checklist rows, proof-pack directories, required
source/security/validation/UI-not-applicable artifacts, plan checkpoint
mentions, and no-claim rollout guard text. It emits JSON and Markdown proof
showing 18 contract-proof rows and six partial/manual-required rows. This closes
the AI enhancement proof index for the current D-lane slice while keeping
runtime model execution, UI delivery, policy authority, enforcement, and product checklist completion unclaimed.
The screen-AI browser trigger proof now adds an activity-domain contract plus
`scripts/test/screen-ai-browser-trigger-proof.mjs` for managed-browser URL,
browser-video, social-feed, and cloud-game trigger rows. The proof composes
typed browser AI input/result rows with screen-analysis result rows and the
parent-domain local-AI context builder, producing two ready local-AI contexts,
one social manual-required context, and one cloud-game unavailable context at
`test-results/screen-ai-browser-trigger-proof/proof.json`. It does not claim
portal UI, broad browser enforcement, authenticated-account social proof,
cloud-frame analysis, mobile browser parity, remote AI, or product checklist
completion.
SOCIAL-01 now adds the social platform account/feed workpack README under the
browser plan. It gives managed-browser social account, feed, short-video,
livestream, messaging-route, upload/post, and bypass rows a proof-root map while
leaving schemas, runtime adapters, policy decisions, UI delivery, native app
support, platform connectors, and enforcement unclaimed.
SOCIAL-02 now adds browser social platform route evidence contracts under
activity-domain. Managed-browser route evidence must cite URL-shape proof, while
unmanaged social use and native-app social states remain bypass-only or
manual-required. The contracts reject account identity, message/feed content
semantics, AI decisions, policy decisions, native app control, connector claims,
and enforcement.
SOCIAL-03 now adds a deterministic activity-domain social URL pattern adapter
from exact managed URL-shape classifications to validated social route evidence.
It maps known social domains and route patterns for signup, login,
account-switch, settings/privacy, messaging, upload/post, livestream, feed,
profile, post, and video routes, including Snapchat and Pinterest domain
patterns. It rejects unmanaged browser and fake-domain rows and does not claim
account identity proof, message/feed content, AI or policy decisions, UI,
native app control, connector access, or enforcement.
SOCIAL-04 now adds route-only account-flow evidence contracts for
managed-browser signup, login, and account-switch social routes. The contracts
link account-flow signals to validated social route evidence or manual-required
states, while rejecting account identity refs, credentials, form submission,
completed account creation, login success, parent approval decisions, policy
decisions, connector access, native app control, UI delivery, and enforcement.
SOCIAL-05 now adds a sanitized form-shape detector contract for managed-browser
signup, login, and account-switch forms. It consumes route-only account-flow
evidence plus control-kind hints and rejects field values, raw DOM, credentials,
form submission, account identity, parent approval decisions, policy decisions,
connector access, native app control, UI delivery, and enforcement.
SOCIAL-06 now adds privacy-preserving social account identity registry
contracts. They can record unverified route-context entries, parent-declared
hash refs, and manual-required state while rejecting raw handles, display names,
platform account ids, credentials, platform verification, connector
authorization, policy decisions, native app control, UI delivery, and
enforcement.
SOCIAL-07 now adds parent-domain social parent approval request/decision
contracts. Requests and decisions reference parent/family/child/device/actor and
evidence refs, remain contract-only, and reject raw account data, credentials,
notification delivery, UI rendering, child notification, policy/action
execution, connector authorization, native app control, and enforcement.
SOCIAL-08 now adds route-only feed/reels/shorts classification contracts for
managed social routes. They consume route evidence plus sanitized surface hints
to distinguish dynamic feeds, short-video feed surfaces, and exact
single-short-video routes without claiming feed content, recommendations,
messages, AI decisions, policy decisions, connector access, native app control,
UI delivery, or enforcement.
SOCIAL-09 now adds bounded metadata-ref extraction contracts for managed social
video/post/feed routes. They record metadata refs such as title, description,
author hash, thumbnail hash, duration, publish date, category, and restriction
signals while rejecting page body, transcript text, messages, feed content, AI
decisions, policy decisions, connector access, native app control, UI delivery,
and enforcement.
SOCIAL-10 now adds social-specific AI analysis contracts for managed-browser
social route evidence. Inputs and prompt templates require typed route,
metadata, feed, account, screen-summary, parent-rule, and memory refs as
applicable while rejecting raw browser/page/feed/message/transcript/screenshot,
native, and connector state. Results carry candidate classifications,
confidence, uncertainty, runtime refs, and degraded states while rejecting final
policy action, enforcement, raw model/content storage, connector, native app,
UI, runtime execution, and product checklist claims.
SOCIAL-11 now adds candidate social risk/benefit signal model contracts sourced
from typed SOCIAL-10 analysis results. Signal rows carry risk/benefit kind,
severity, state, confidence, and evidence refs, while signal sets preserve
analysis provenance and reject raw message/feed/page/model use, account identity
verification, final policy decisions, connector/native claims, UI delivery,
runtime gates, enforcement, and product checklist claims.
SOCIAL-12 now adds parent-domain social policy compiler contracts for social
targets. The compiler consumes parent-owned evidence, signal-set, parent-rule,
and schedule refs and emits non-final decision candidates for allow, warn,
parent-review, block, manual-review, or unknown outcomes. It rejects raw payloads,
activity-domain object transfer, UI/runtime/native/connector/enforcement claims,
and product checklist upgrades.
SOCIAL-13 now adds managed-browser social account gate-plan contracts from
route-only account-flow evidence, sanitized form-shape evidence, and
policy/approval refs. The plans keep account navigation/submission actions as
candidates and reject runtime browser pause/block, child/parent UI, final policy,
credential, form submission, account creation, native app, connector,
enforcement, and product checklist claims. The live proof now captures public
Facebook signup, Pinterest login, Reddit register, and Instagram signup surfaces
with Playwright, persists screenshots plus route-only proof JSON, and avoids raw
DOM, field values, credentials, form submission, and account creation claims.
SOCIAL-14 now adds managed-browser feed/short/video route gate-plan contracts
from typed feed classification, bounded metadata evidence, and
policy/approval/time-limit refs. The plans keep route allow/warn/parent-review/
block/limit/manual/unknown actions as candidates and reject browser block,
redirect, CSS/DOM hide, tab close, applied time limit, UI, final policy,
content capture, recommendation modeling, native app, connector, enforcement,
and product checklist claims.
SOCIAL-15 now adds unmanaged social bypass detector contracts from redacted
unmanaged/browser-like process evidence. The evidence is bypass-only and
managed-browser-required, rejecting exact URLs, social route/account/feed/video/
message proof, UI, process control, native app, connector, enforcement, and
product checklist claims.
SOCIAL-16 now adds Android native social app capability matrix contracts in
parent-domain. The matrix covers package visibility, UsageStats foreground,
accessibility route hints, VPN/domain hints, device-owner app control, and
managed-profile config, while rejecting native route proof, per-video/per-reel
blocking, content capture, account identity, runtime adapter, connector, UI,
enforcement, and product checklist claims.
SOCIAL-17 now adds iOS Screen Time/ManagedSettings social capability matrix
contracts in parent-domain. The matrix covers FamilyControls authorization,
app/web-domain token selection, DeviceActivity monitor state, and
ManagedSettings shield states while keeping Apple entitlement approval, device
proof, raw app identity, native route proof, content capture, runtime adapter,
connector, UI, enforcement, and product checklist claims unclaimed.
SOCIAL-18 now adds platform connector authorization boundary contracts in
parent-domain. The boundary keeps Google/YouTube supervision, Meta Family
Center, TikTok Family Pairing, platform export/import, and parent-provided
account refs optional and parent-authorized only, with no token storage, OAuth
client, provider API, raw account/message/feed data, core gating dependency,
UI, native control, enforcement, or product checklist claim.
SOCIAL-19 now adds parent-domain social decision memory-cache contracts for
account, video, and channel decision refs. Fresh cached decisions can feed
policy input only when decision refs are present and no invalidation reasons
exist; stale, miss, and manual-required rows cannot. This does not claim a
runtime cache store, AI cache, activity-domain export, raw content storage, UI,
native control, enforcement, or product checklist status.
SOCIAL-20 now adds parent-domain parent social dashboard UX section contracts
and text-domain copy tokens for account approvals, feed/video gates, native app
capability, connector boundaries, decision memory, and manual-required gaps.
This does not claim a rendered portal dashboard, runtime data fetch,
notification, connector authorization, native control, enforcement, or product
checklist status.
SOCIAL-21 now adds parent-domain child approval/block UX state contracts and
text-domain calm copy tokens for approval pending, blocked route candidates,
warnings, manual review, time-limit candidates, and native-app unavailable
states. This does not claim rendered child UI, notifications, browser block
execution, block-page rendering, applied time limits, final policy execution,
native control, enforcement, or product checklist status.
SOCIAL-22 now adds parent-domain social audit/explanation read-model contracts
for account approval, feed/video gate, native-app gap, connector boundary,
decision memory, and manual-required gap rows. This does not claim a runtime
audit store, rendered explanation UI, notifications, raw account/video/message
content, connector authorization, native control, final policy execution,
enforcement, or product checklist status.
SOCIAL-23 now adds a social proof artifact gate that checks SOCIAL-01 through
SOCIAL-22 proof-pack coverage and marks Playwright manual-required because this
slice does not render social UI. This does not claim screenshots, runtime
connector behavior, native control, final policy execution, enforcement, or
product checklist status.
SOCIAL-24 now labels the social track partial/manual-required through the
rollout gate. Product checklist upgrade is not claimed. Rendered social UI,
Playwright screenshots, connector/native runtime, final policy execution, and
enforcement remain unclaimed.

## Checklist

- [x] Managed browser launch/profile state.
- [ ] Exact URL/tab evidence.
- [x] Unmanaged-browser bypass status.
- [ ] Site/domain/category rule targets.
- [ ] Schedule and exception support.
- [ ] Dry-run preview with evidence refs.
- [ ] Adapter capability status.
- [ ] Real blocking/terminate proof where claimed.
- [ ] Exact active-tab enforcement and host domain blocking proof before any
      managed URL or network/domain claim upgrade.

## Next AI Instructions

Keep managed and unmanaged browser claims separate. Do not claim page semantics
from network metadata. If adding web control, update browser expectations,
policy contracts, enforcement status, and portal source labels together. Any
future exact active-tab claim must add new managed-browser artifacts beyond the
current integrity runtime audit proof. Use the browser plan folder for
implementation sequencing and workpack ownership; do not recreate browser
contracts, URL/video intelligence, policy catalogs, or UI surfaces outside the
existing package/crate layout unless an ownership boundary changes.
