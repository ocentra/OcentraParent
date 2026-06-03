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
  Report-only, warn-child, ask-parent, terminate-process, relaunch managed
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
