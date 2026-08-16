# V0.5 Browser URL And Video AI Intelligence Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `V0.5 Browser URL And Video AI Intelligence Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This document defines the browser-owned intelligence layer for URL, page, and
video understanding. It belongs in the browser plan because it starts from
managed browser evidence, but it crosses social/video, local AI, policy,
memory, reporting, and enforcement boundaries.

This is a plan document only. It does not claim product-complete video
analysis, hidden page loading, remote AI, social feed analysis, or real-time
blocking until the source evidence, model/runtime path, policy decision, adapter
proof, and audit output exist.

## Core Rule

```text
Browser evidence proves what URL, page, or video was observed.
AI analysis classifies what that evidence appears to mean.
Parent policy decides what should happen.
Enforcement adapters apply only proved policy actions.
```

Never let AI act directly:

```text
AI result is evidence.
Policy decision is authority.
Enforcement adapter is execution.
```

Correct flow:

```text
URL evidence
-> URL intelligence extraction
-> optional metadata fetch
-> optional hidden managed analysis load
-> local AI classification
-> structured AI result
-> memory/cache lookup and update
-> deterministic parent policy evaluation
-> allow / warn / parent-review / limit / block / unknown
-> enforcement only if adapter proof exists
-> audit event
```

Forbidden flow:

```text
URL goes to AI
AI says risky
agent blocks without parent policy and audit
```

## Why This Exists

Domain control is not enough. A child can open the same video platform for a
math lesson, science lecture, music video, gaming stream, prank compilation,
political content, misinformation, self-harm content, rage-bait shorts, or an
educational documentary. The browser plan needs a path that can answer:

- what URL was opened;
- whether the URL is a video, short, channel, search, article, social feed, or
  unknown page;
- which platform and stable ids are available;
- which metadata can be cited;
- whether the content appears educational, entertainment, social, risky, or
  unknown;
- whether a parent rule allows, warns, asks, limits, blocks, or defers;
- whether a previous decision can be reused safely;
- which source evidence, model, prompt, policy, memory, and audit refs explain
  the final state.

## Two Browser Faces

### Face A: Browser Capability And Evidence

This layer answers:

- which browser is installed or running;
- whether it is managed by Ocentra;
- what URL/title/domain/tab evidence was observed;
- whether exact URL evidence or only process/domain evidence exists;
- whether the bridge is fresh, stale, degraded, or unknown;
- whether active-tab state is known or unknown.

Sources include managed browser CDP/BiDi/bridge output, target lists, URL,
title, domain, process/window fallback, network/domain fallback, and unmanaged
browser detection. This layer does not decide child safety.

### Face B: URL, Page, And Video Intelligence

This layer answers:

- what kind of content the evidence appears to describe;
- whether it is a YouTube, Vimeo, TikTok, Shorts, Reel, Twitch, Facebook,
  Instagram, Reddit, Discord, generic web, or unknown target;
- which metadata, transcript, thumbnail, rating, title, description, or platform
  ids are available;
- whether local AI can classify the item with confidence and reason codes;
- whether a previous evidence-backed memory result is still usable;
- whether the policy should allow, warn, time-limit, ask parent, block, or mark
  unknown.

This layer still does not enforce directly.

## Stage 0: Evidence Intake

Input must come from typed browser evidence, normally managed browser evidence.
Unmanaged browser process or network-only evidence can support domain-level or
bypass risk, but it must not claim exact page/video semantics.

Required intake fields:

- source evidence id;
- source kind, such as managed browser, unmanaged browser, or network/domain;
- browser family and managed session refs when available;
- URL/title/domain/origin where evidence permits;
- observed/fresh/stale timestamps;
- custody label;
- active-tab proof state;
- degraded/manual-required reason codes.

## Stage 1: Fast URL Shape Classification

This stage is deterministic, cheap, and runs before AI. It identifies URL shape,
platform, target ids, and reason codes.

It should answer:

- valid web URL, browser-internal URL, file URL, download, or unknown;
- video, short video, channel, playlist, search, article, forum, social feed,
  game, or unknown;
- YouTube, YouTube Shorts, Vimeo, TikTok, Instagram, Facebook, Twitch, X/Twitter,
  Reddit, Discord, generic web, or unknown;
- platform video id, channel id, playlist id, post id, or query where present;
- confidence and reason codes.

Contract family:

```text
BrowserUrlShapeClassification
BrowserUrlShapePlatformIds
BrowserUrlShapeReasonCode
BrowserUrlShapeClassificationResult
```

Do not use the URL shape result to claim actual video content. It proves only
what can be parsed from the URL and already captured title/domain metadata.

## Stage 2: Local Memory Lookup

Before expensive AI work, check local evidence-backed memory. Memory is a
derived index, not source truth.

Allowed memory keys:

- normalized URL;
- canonical video id;
- platform plus video id;
- platform plus channel id;
- domain plus path hash;
- content metadata hash;
- parent-approved exception id;
- previous AI analysis id;
- previous policy decision id.

Memory may answer known allowed educational video, known blocked video,
previously approved/denied parent decision, known safe channel during homework,
known entertainment channel with time budget, known risky domain, or known
unknown needing reanalysis.

Every memory hit must cite evidence refs, analysis refs, parent action refs, or
policy version refs. If it cannot cite its source, it cannot drive blocking.

Contract family:

```text
BrowserUrlIntelligenceMemoryHit
BrowserUrlIntelligenceMemoryKey
BrowserUrlIntelligenceMemoryStaleReason
```

## Stage 3: Metadata Extraction

Try cheap metadata before hidden page loading or AI.

Allowed metadata sources:

- browser title from managed evidence;
- OpenGraph title/description;
- schema.org Article or VideoObject metadata;
- platform video id and channel id;
- channel name;
- thumbnail URL/hash where allowed;
- duration, publish date, captions/transcript availability;
- platform category/rating/restricted signals where available;
- hidden managed analysis load output when that later workpack is proved.

Platform metadata is evidence, not authority. Do not treat platform labels as
parent policy.

Contract family:

```text
BrowserUrlMetadataEvidence
BrowserUrlMetadataSource
BrowserUrlMetadataDegradedReason
```

## Stage 4: Hidden Managed Analysis Load

Hidden analysis is optional and must be separately proved. It loads a URL in an
isolated Ocentra-owned analysis context so the system can gather structured
signals without disturbing the child unless policy requires a temporary hold.

Allowed properties:

- Ocentra-owned analysis profile;
- separate from the child visible managed profile;
- no personal cookies by default;
- no child account login by default;
- no child session token sharing;
- strict timeout;
- no autoplay audio;
- no arbitrary downloads;
- no form submit;
- no CAPTCHA automation claims;
- no login bypass claims;
- bounded retention of structured summaries only.

Hidden analysis states:

```text
not-needed
queued
loading
metadata-only
analysis-ready
blocked-by-robots-or-login
timeout
platform-restricted
network-error
unsupported-content
manual-required
```

Hidden analysis must not steal cookies from the child browser, access account
tokens, submit forms, click recommendations, watch full videos by default,
retain raw page bodies indefinitely, or claim frame/audio understanding unless
that input was actually captured and allowed.

## Stage 5: AI Analysis Pass

AI consumes structured context only. It does not read raw browser state, browser
profiles, DevTools payloads, SQLite files, journals, or OS state directly.

Input must include:

- request id and schema version;
- child profile, device, and policy version refs;
- source browser evidence refs;
- URL shape refs and metadata evidence refs;
- normalized URL/domain and platform ids where available;
- page title/description where evidence permits;
- transcript, thumbnail, screen, memory, and graph refs only when separately
  allowed and evidence-backed;
- parent rule refs and schedule context refs;
- requested task, such as URL safety, video safety, educational relevance,
  category classification, parent-review summary, or policy decision support;
- model runtime preference;
- custody label.

Output must include:

- analysis id and request id;
- source evidence, metadata, memory, graph, and parent rule refs;
- content kind and optional video kind;
- benefit signals and risk signals;
- recommended policy input, not final policy action;
- confidence and uncertainty reason codes;
- parent summary and optional child-safe summary;
- model runtime ref and prompt template version;
- analyzed/expiry timestamps;
- degraded state when model, transcript, metadata, hidden load, language, or
  timeout conditions reduce confidence.

Contract family:

```text
BrowserUrlAiAnalysisInput
BrowserUrlAiAnalysisResult
BrowserContentCategory
BrowserContentModifier
BrowserBenefitSignals
BrowserRiskSignals
BrowserAiPromptTemplate
```

## Stage 6: Deterministic Policy Evaluation

The policy evaluator takes browser evidence, URL shape classification, metadata
evidence, AI result, memory result, parent rules, schedule, child profile, and
current mode. It produces the final typed decision.

Allowed decision outcomes:

```text
allow
warn
ask_parent
time_limit
block
unknown
```

Allowed enforcement modes:

```text
observe_only
dry_run
active
manual_required
unavailable
```

Examples:

- parent allows educational videos during homework, AI returns high-confidence
  math lesson, policy decides allow;
- parent asks for gaming videos during school, AI returns gaming video,
  policy decides parent-review;
- parent blocks adult content always, AI returns high adult risk, policy
  decides block if adapter proof exists;
- AI returns low confidence or missing transcript, policy decides unknown,
  parent-review, warn, or stricter fallback based on parent settings.

Contract family:

```text
BrowserPolicyDecision
BrowserPolicyDecisionReasonCode
BrowserPolicyDecisionAuditRef
```

## Stage 7: Enforcement And Post-Analysis Action

Enforcement depends on capability and adapter proof:

- allow: do nothing;
- warn: show a warning or interstitial;
- parent-review: temporary hold or allow-with-pending based on parent setting;
- time-limit: start or continue a timer;
- block: redirect, close, or navigate to a block page only if managed adapter
  proof exists;
- unknown: follow explicit parent fallback.

If the child is already watching while analysis runs, do not pretend blocking
was real time. Use post-analysis labels such as:

- background reviewed;
- continue allowed;
- warning shown after review;
- playback stopped after review;
- parent approval requested after review;
- future visits blocked;
- remembered with expiry.

## Real-Time Modes

### Mode A: Immediate Allow With Background Review

Use for low-risk domains, strong safe memory hits, educational allowlists, and
parent settings that allow review later.

### Mode B: Temporary Hold Until Classification

Use for unknown video during school/homework, new social/video URL, strict
parent settings, low-trust platform, or high-risk candidates.

Suggested budgets:

- fast path: 1-3 seconds;
- AI path: 5-15 seconds;
- timeout: explicit parent fallback.

### Mode C: Allow Known, Hold Unknown

Best default:

- known allowed: instant allow;
- known blocked: instant block when adapter proof exists;
- known parent-review: ask immediately;
- unknown low-risk: background review;
- unknown video/social/high-risk: temporary hold.

### Mode D: Observe Only

Record and classify for reports only. Do not interrupt the child.

### Mode E: Strict Mode

Use for younger children, school, homework, or bedtime. Unknown videos are held,
low confidence asks parent, social feeds are held/blocked by platform policy,
and known educational allowlists continue.

## AI Provider Routing

Provider kinds:

- child-device local AI;
- family AI hub local to the household;
- parent-approved remote/API AI;
- metadata only or no AI.

Provider route modes:

- local only;
- local then family AI hub;
- local then parent-approved remote;
- metadata only;
- ask parent when AI unavailable.

Rules:

- remote AI must not be default for blocking;
- parent must explicitly enable remote/API use;
- data scope, retention, custody, provider, and no-retention status must be
  visible before use;
- remote output cannot override stricter local parent rules;
- remote outage does not disable local safety;
- every provider route must be auditable.

Contract family:

```text
BrowserAiProviderRoute
BrowserAiProviderCapability
BrowserAiProviderDegradedState
```

## Memory, Cache, And Knowledge Graph

Cache levels:

- L1 in-memory active-session cache;
- L2 SQLite URL/video analysis cache;
- L3 local vector/semantic memory;
- L4 family AI hub shared cache;
- L5 parent-approved remote report bundle.

Cache keys:

- canonical URL;
- normalized origin plus path;
- platform video id;
- platform channel id;
- metadata hash;
- transcript hash;
- thumbnail perceptual hash only if approved;
- model id plus prompt version;
- policy version;
- child profile.

Invalidate when parent policy, model, prompt, metadata, transcript, platform id,
parent override, confidence, or TTL changes. Dynamic feeds, search results,
homepages, social feeds, and livestreams need shorter TTLs than stable videos or
parent-approved educational sites.

Memory cannot say "this video is safe forever." It can say "this video was
classified under model X, prompt Y, policy Z, from evidence A/B/C, and expires
at time T."

## Analysis Depth

Level 0: URL only.
Uses URL, domain, path, query, and title if already available. It can classify
platform, video id, search URL, obvious domain candidates, and memory hits. It
cannot claim video content or page body meaning.

Level 1: Metadata.
Uses title, description, OpenGraph, schema.org, duration, channel, thumbnail
metadata/hash, category, and rating. Good for most fast decisions.

Level 2: Transcript or captions.
Uses captions/transcript when available and allowed. Better for educational and
risk detection.

Level 3: Visual/audio summary.
Future explicit feature. Uses sampled frames, local vision model summary, audio
transcript, or screen summary only when separately proved and allowed.

Level 4: Full interactive page analysis.
Rare and high-cost. Uses hidden managed page load, visible DOM metadata summary,
structured extraction, and limited screenshot summary only when explicitly
allowed.

## Parent Settings

Parent settings should not expose raw platform toggles as product complexity.
They should produce clear modes:

- how strict unknown videos should be;
- whether educational videos are allowed during school/homework;
- whether entertainment videos are time-limited;
- whether Shorts/Reels/TikTok-style videos are stricter;
- whether low-confidence AI asks parent;
- whether remote AI is allowed for hard cases;
- whether decisions are remembered;
- how long URL/video decisions are cached;
- what happens while analysis runs;
- what happens if AI fails.

Safe defaults:

- known educational: allow;
- known entertainment: time-limit;
- unknown video during homework: ask parent;
- unknown video during free time: allow with background review;
- adult/self-harm/gambling: block when proof exists;
- low confidence: ask parent or warn;
- AI unavailable: explicit parent fallback.

## Child Experience

Child-facing states should be calm:

- opening;
- checking this page;
- this looks educational and is allowed;
- this video needs parent approval;
- this site is limited right now;
- this page is blocked by your parent rule;
- this page could not be classified.

Do not use copy that shames the child or implies surveillance. Preferred tone:

```text
Ocentra is checking whether this page matches your family rules.
```

## Parent Experience

Parent views should show:

- URL/video/platform;
- classification and confidence;
- evidence sources;
- model/provider used;
- prompt/model version where relevant;
- policy rule matched;
- final decision;
- action taken;
- memory/cache result;
- whether the child saw the page before the decision;
- degraded/manual-required state.

Example state:

```text
YouTube video reviewed
Title: Introduction to Fractions - Grade 5 Math
Classification: Educational / Math
Evidence: browser URL, title, metadata, transcript summary
AI provider: child device local
Policy: allow educational videos during homework
Decision: allowed
Remembered: yes, 30 days
```

Post-analysis example:

```text
Video reviewed after playback started
Classification: entertainment / risky prank
Confidence: medium
Policy: ask parent for unknown entertainment during school time
Decision: parent approval required
Action: playback stopped after review
```

## Category, Risk, And Benefit Model

Content categories should be first-class, for example:

- education;
- homework;
- research;
- news;
- entertainment;
- gaming;
- music;
- social;
- shopping;
- communication;
- adult;
- violence;
- self-harm;
- drugs/alcohol;
- gambling;
- hate/harassment;
- weapons;
- misinformation;
- unknown.

Modifiers should include video, short video, livestream, comments-heavy,
recommendation feed, search results, download, login required, dynamic feed,
user generated, platform restricted, and low confidence.

Classify benefit as well as risk. Benefit signals can include educational
value, homework relevance, age appropriateness, skill building, creativity, and
physical activity. Risk signals can include adult, violence, self-harm,
drugs/alcohol, gambling, hate/harassment, misinformation, social manipulation,
distraction, addictive pattern, and privacy risk.

## Queue And Scheduling

Analysis should be queued and priority-aware:

- P0 strict hold waiting for decision;
- P1 active child page unknown video;
- P2 active child page normal URL;
- P3 background review;
- P4 memory refresh;
- P5 report enrichment.

Timeout behavior must be policy-owned:

- P0 timeout falls back quickly to ask/block/allow based on parent rule;
- P1 timeout can warn or ask;
- P2 timeout can become background only;
- P3-P5 can wait or degrade.

Contract family:

```text
BrowserAnalysisJob
BrowserAnalysisPriority
BrowserAnalysisJobStatus
BrowserAnalysisTimeoutPolicy
```

## Complex Cases

YouTube homepage: dynamic feed. Do not classify as one video. Policy may allow
with limit, block feed, allow search only, or ask parent.

YouTube video: URL has stable video id. Fetch metadata/transcript where allowed,
classify the exact video, and cache by video id.

YouTube Shorts: short-video format. Apply stricter distraction defaults when
parent settings require it, and classify exact video when possible.

Search results: classify query if available. Do not classify every result unless
opened or a separately approved preview mode exists.

Login required: do not steal child session cookies into hidden analysis. Fall
back to visible browser title/URL and parent settings.

Dynamic social feed: short TTL, hard to classify as stable, platform/feed policy
may apply, and screen-summary integration is later proof.

Livestream: metadata can change, risk can change, and time-limited/recheck
policy is required.

Comments risk: a video can be educational while comments are risky. Unless
comments are captured and allowed, do not claim comment analysis.

Multilingual content: detect language, choose capable provider, and degrade to
ask/warn/unknown when unsupported.

## Enhancement Workpack Split

This intelligence layer can be assigned as sub-workpacks under the browser plan:

1. Browser AI intelligence plan links.
2. URL shape classification contracts.
3. Platform/video URL parser library.
4. Browser intelligence memory contracts.
5. Metadata extraction contracts.
6. Hidden managed analysis profile design.
7. Hidden analysis loader adapter.
8. AI analysis input/output contracts.
9. Local AI provider routing.
10. Family AI hub routing.
11. Parent-approved remote AI boundary.
12. Prompt/template versioning.
13. Structured category/risk/benefit model.
14. URL/video analysis queue.
15. Memory/cache store.
16. Knowledge graph references.
17. Policy evaluator integration.
18. Post-analysis action model.
19. Child-facing checking/warning UX.
20. Parent explanation/audit UX.
21. YouTube parser and metadata adapter.
22. Vimeo/generic video parser.
23. Dynamic feed/social URL handling.
24. Provider degraded/fallback behavior.
25. Proof gates, fixtures, tests, and rollout.

## Implementation Checkpoint - 2026-06-03

- AI-01 is represented by this plan, the browser-plan source index, and the
  implementation checklist. The plan remains a browser-owned enhancement path
  that starts from managed browser evidence and does not replace the social,
  local AI, policy, memory, reporting, or enforcement source docs.
- AI-02 now has schema-backed URL shape classification contracts in
  `packages/activity-domain/src/browser-url-intelligence-schemas.ts`. The
  contract accepts deterministic URL/platform/id shape rows only when exact
  managed browser URL evidence exists, and it rejects content semantics, AI
  decision, policy decision, and unmanaged/network exact-page claims.
- AI-03 now has a deterministic parser helper in
  `packages/activity-domain/src/browser-url-intelligence.ts`. The parser maps
  supported URL shapes into the AI-02 contract for YouTube video, Shorts,
  channel, playlist, search, Vimeo video, TikTok video/feed, and generic web
  rows. Unsupported schemes, credential-bearing URLs, unmanaged process rows,
  and network/domain rows stay rejected or unknown/non-exact.
- AI-04 now has schema-backed memory hit/miss/stale/manual-required contracts
  in `packages/activity-domain/src/browser-url-intelligence-schemas.ts`. Fresh
  hits must cite source evidence, policy version, expiry, and an analysis or
  parent action ref before they can drive policy input. Stale, miss, and
  manual-required rows cannot drive policy input, and no memory row can enforce
  directly.
- AI-05 now has schema-backed metadata evidence contracts in
  `packages/activity-domain/src/browser-url-metadata-schemas.ts`. Metadata rows
  can carry browser title, OpenGraph/schema.org/platform fields, thumbnail refs,
  duration, publish date, captions availability, and platform labels as
  evidence for AI input, while rejecting page-body capture, transcript text
  capture, hidden-analysis metadata without proof, AI decisions, policy
  decisions, and platform metadata as policy authority.
- AI-06 now has schema-backed hidden managed analysis profile design contracts
  in `packages/activity-domain/src/browser-hidden-analysis-schemas.ts`. The
  design requires an Ocentra-owned profile separate from the child visible
  profile, bounded retention, timeout/summary budgets, and no child cookies,
  session tokens, autoplay audio, downloads, form submits, CAPTCHA/login bypass
  claims, or raw page-body retention. Metadata-only and analysis-ready states
  require a later loader proof ref.
- AI-07 now has a typed hidden analysis loader adapter boundary in
  `packages/activity-domain/src/browser-hidden-analysis-loader.ts`, exported as
  focused activity-domain subpaths. The deterministic planner can advance safe
  queued designs to `loading` or return manual-required for disabled/unavailable
  capability, but result schemas still reject page-body capture, transcript text
  capture, and metadata-only or analysis-ready states without a loader proof ref.
- AI-08 now has schema-backed AI analysis input/output contracts in
  `packages/activity-domain/src/browser-ai-analysis-schemas.ts` with literal
  and id values split into
  `packages/activity-domain/src/browser-ai-analysis-values.ts`. Inputs reference
  stored browser evidence, URL shape, metadata, memory, graph, parent rule,
  schedule, prompt template, model preference, and custody refs without raw
  browser state, DevTools payloads, SQLite paths, journals, or OS state. Outputs
  expose content category/modifier, benefit/risk signals, confidence,
  uncertainty, parent/child summaries, model/runtime refs, and candidate-only
  policy input while rejecting final policy action, enforcement, and raw content
  storage claims.
- AI-09 now has schema-backed local AI provider routing contracts in
  `packages/activity-domain/src/browser-ai-provider-routing-schemas.ts`.
  Provider capability rows prove whether a child-device local AI route is
  available, disabled, missing a model, unavailable, or resource-exhausted with
  visible custody, retention, provider, and no-retention state. The deterministic
  local planner selects a local runtime only when the AI-08 request asks for
  local execution and the provider supports the task; otherwise it returns
  manual-required or unavailable without silently defaulting to family hub or
  remote AI. Routes reject hidden visibility, remote-default-for-blocking,
  remote override, and remote-outage-disables-local-safety claims.
- AI-10 now has schema-backed family AI hub routing contracts in
  `packages/activity-domain/src/browser-ai-family-hub-routing-schemas.ts`. The
  family hub route is a local-household fallback only: it can be selected after
  the child-device local provider was attempted and did not serve the request,
  the parent allowed household hub routing, the AI-08 request permits
  local-preferred routing, and the hub capability proves task support,
  no-retention custody, a household route ref, and a model runtime ref. Routes
  reject hidden visibility, remote provider selection, remote default blocking,
  and attempts to use the family hub before the local provider path is exhausted.
- AI-11 now has schema-backed parent-approved remote AI boundary contracts in
  `packages/activity-domain/src/browser-ai-remote-boundary-schemas.ts` with
  literal/id values split into
  `packages/activity-domain/src/browser-ai-remote-boundary-values.ts`. Remote
  approval, capability, and route contracts require explicit parent approval,
  visible structured data scope, no-retention mode, provider visibility, local
  safety fallback, and audit evidence. Routes reject raw browser state, page
  body, transcript text, screenshot access, remote default blocking, remote
  override of stricter local rules, and remote outages disabling local safety.
- AI-12 now has schema-backed prompt/template versioning contracts in
  `packages/activity-domain/src/browser-ai-prompt-template-schemas.ts` with
  lifecycle/status values split into
  `packages/activity-domain/src/browser-ai-prompt-template-values.ts`. Prompt
  version records track template id/version, hash refs, change refs, model and
  policy compatibility, audit evidence, lifecycle state, supersession, and
  memory invalidation. Registries reject duplicate active task/model templates,
  and selection fails closed to manual-required when no active template supports
  the requested task, model, or policy version.
- AI-13 now has schema-backed structured category/risk/benefit model contracts
  in `packages/activity-domain/src/browser-ai-riskbenefit-model-schemas.ts`
  with taxonomy values split into
  `packages/activity-domain/src/browser-ai-riskbenefit-model-values.ts`.
  Taxonomy records make categories, modifiers, benefit signals, risk signals,
  unknown fallbacks, and version refs first-class. Assessment records require
  source evidence, confidence/uncertainty visibility, matching risk or benefit
  signals for key categories, and candidate-only state while rejecting platform
  labels as policy authority, final policy actions, and enforcement claims.
- AI-14 now has schema-backed URL/video analysis queue contracts in
  `packages/activity-domain/src/browser-ai-analysis-queue-schemas.ts` with job,
  priority, status, and timeout values split into
  `packages/activity-domain/src/browser-ai-analysis-queue-values.ts`. Queue jobs
  carry structured AI input refs, priority, status, parent-owned timeout policy,
  queued evidence ids, and optional matching results. P0/P1/P2 timeout
  dispositions are priority-bound, background jobs wait/degrade, completed jobs
  must match the input request id, and queued/running/degraded/timeout states
  cannot carry results or claim worker runtime, policy, or enforcement authority.
- AI-15 now has schema-backed memory/cache store contracts in
  `packages/activity-domain/src/browser-ai-memory-cache-store-schemas.ts` with
  key, TTL, and invalidation values split into
  `packages/activity-domain/src/browser-ai-memory-cache-store-values.ts`. Cache
  entries wrap existing memory-hit contracts with complete cache keys, TTL
  classes, invalidation reasons, bounded retention, no raw content storage, and
  no direct enforcement authority. Fresh entries must include model/prompt,
  policy, child profile, and content locator keys; stale/invalidated entries
  cannot drive policy input.
- AI-16 now has schema-backed knowledge graph reference contracts in
  `packages/activity-domain/src/browser-ai-knowledge-graph-schemas.ts` with
  graph/node/edge/source/use values split into
  `packages/activity-domain/src/browser-ai-knowledge-graph-values.ts`. Graph
  bundles cite stored evidence, metadata, memory, AI analysis, parent rule, or
  parent-approved source refs only. They reject raw content storage, platform
  labels as authority, direct policy/enforcement authority, stale or
  low-confidence policy-driving refs, duplicate nodes, and edges that point
  outside the bundle. No graph store, graph builder, model execution, policy
  evaluator, UI, enforcement, or product checklist update is claimed.
- AI-17 now has schema-backed policy evaluator integration contracts in
  `packages/activity-domain/src/browser-ai-policy-evaluator-schemas.ts` with
  evaluator ids, decision outcomes, modes, and reason codes split into
  `packages/activity-domain/src/browser-ai-policy-evaluator-values.ts`. Input
  bundles hand validated browser evidence, URL shape, metadata, AI result,
  memory/cache, graph, parent rule, schedule, child profile, and mode refs to
  the evaluator without raw model text, unvalidated AI output, portal UI state,
  final decision claims, or direct enforcement claims. Decision bundles require
  evidence refs, parent rule refs, reason codes, audit refs, fallback visibility,
  and adapter proof for active block decisions while rejecting AI/portal/direct
  enforcement authority. No evaluator runtime, parent-domain policy engine, UI,
  enforcement, or product checklist update is claimed.
- AI-18 now has schema-backed post-analysis action model contracts in
  `packages/activity-domain/src/browser-ai-post-analysis-action-schemas.ts` with
  action labels, triggers, timing, and delivery states split into
  `packages/activity-domain/src/browser-ai-post-analysis-action-values.ts`.
  Action plans link source evidence, AI analysis, policy decision, policy audit,
  parent rule, action label, timing, delivery, adapter proof, remember-expiry,
  and action audit refs. They reject real-time block claims after playback has
  started, browser runtime mutation claims, direct enforcement claims, delivered
  warning/stop/approval/future-block actions without adapter proof, remembered
  actions without expiry, and unknown decisions without manual or parent
  fallback action. No child UI, parent UI, browser runtime mutation, enforcement,
  or product checklist update is claimed.
- AI-19 now has schema-backed child-facing checking/warning UX state contracts
  in `packages/activity-domain/src/browser-ai-child-ux-schemas.ts` and calm copy
  tokens in `packages/text-domain/src/browser-child-ux.ts`. Snapshots label
  opening, checking, allowed, warning, approval-required, limited, blocked,
  unclassified, manual-required, and unavailable states with schema-known text
  tokens, evidence refs, post-analysis action linkage where applicable, and
  adapter proof before delivered checking, warning, block, or approval pages can
  be claimed. The contracts reject raw copy, shaming/surveillance copy claims,
  state/token mismatches, rendered page delivery without adapter proof, and
  warning/block/approval states without matching post-analysis actions. The
  rendered proof uses a real Chrome/Chromium CDP session, captures a live public
  YouTube watch page before intervention, writes the shared child intervention
  renderer HTML through the Rust child-agent file hook, serves it from
  `/api/browser/intervention/page`, and captures checking, warning,
  approval-required, limited, and blocked screenshots. No final policy
  execution, unmanaged browser control, native/mobile blocking, connector
  behavior, enforcement, or product checklist update is claimed.
- AI-20 now has schema-backed parent explanation/audit UX contracts in
  `packages/activity-domain/src/browser-ai-parent-explanation-schemas.ts` and
  parent explanation text tokens in
  `packages/text-domain/src/browser-parent-explanation.ts`. Explanation bundles
  link evidence, AI analysis, policy decision, post-analysis action, child UX
  snapshot, memory/cache refs, graph refs, and audit refs while requiring
  visibility for evidence, model runtime, prompt version, policy rule, action,
  child experience, child-saw-page, degraded/manual fallback, and audit trail
  fields when applicable. They reject raw page content, raw prompt text, portal
  evaluation, policy authority, direct enforcement, hidden fallback, hidden child
  engagement, missing audit sections, and mismatched source evidence. The
  rendered proof consumes the AI-19 live YouTube CDP child UX evidence JSON,
  schema-decodes the parent explanation bundle, renders the Browser review
  region on the real portal `#/browser` route, captures desktop and mobile
  screenshots, and proves evidence/model/policy/action/child-delivery/audit
  visibility without rendering the raw YouTube URL. Runtime service delivery of
  parent explanation bundles, notification/report delivery, final policy
  authority, browser mutation, remote AI, enforcement, raw page content, raw
  prompt text, and product checklist update remain unclaimed.
- AI-21 now extends deterministic YouTube URL parser coverage and adds a YouTube
  metadata adapter in `packages/activity-domain/src/browser-youtube-metadata.ts`.
  The parser recognizes managed exact YouTube watch, Shorts, embed, live,
  channel, and playlist shapes. The adapter accepts exact managed YouTube video,
  short, channel, or playlist classifications and emits
  `BrowserUrlMetadataEvidence` with title, description, platform ids, channel
  name, thumbnail refs, duration, publish date, captions/transcript availability,
  category/rating/restricted signals, and degraded reasons. It rejects unmanaged
  or non-YouTube classifications and does not capture page body, transcript text,
  content semantics authority, AI decisions, policy decisions, or policy
  authority. Public package/barrel exports are pending source/package export
  coordination; no network fetcher, transcript parser, hidden page load, AI
  execution, policy evaluator, enforcement, or product checklist update is
  claimed.
- AI-22 now extends Vimeo URL parser coverage and adds a Vimeo/generic video
  metadata adapter in `packages/activity-domain/src/browser-video-metadata.ts`.
  The parser recognizes managed exact Vimeo page URLs and player URLs with
  numeric video ids. The adapter accepts exact managed Vimeo video
  classifications or exact managed generic web classifications with schema.org
  VideoObject metadata, then emits `BrowserUrlMetadataEvidence` with title,
  description, platform video id, channel name, thumbnail refs, duration,
  publish date, captions/transcript availability, category/rating/restricted
  signals, and degraded reasons. It rejects unmanaged classifications and
  generic OpenGraph-only rows, and it does not capture page body, transcript
  text, content semantics authority, AI decisions, policy decisions, or policy
  authority. Public package/barrel exports are pending source/package export
  coordination; no network fetcher, transcript parser, hidden page load, AI
  execution, policy evaluator, enforcement, or product checklist update is
  claimed.
- AI-23 now extends URL shape contracts and parser coverage for dynamic
  feed/social URL handling. `BrowserUrlShapeTargetKind` includes social post,
  messaging, upload/post, and livestream route targets alongside social feed,
  and the deterministic parser recognizes visible route shapes for Instagram,
  TikTok, Facebook, Twitch, X/Twitter, Reddit, and Discord when the source is
  managed exact URL evidence. Dynamic feeds, home/explore/following routes, and
  social feeds carry dynamic-feed or social-route reason codes with medium/low
  confidence. Exact reels/status/posts carry post ids where visible. Unmanaged
  social rows remain unknown/non-exact, dynamic-feed TTL stale memory rows
  cannot drive policy input, and parser rows keep content semantics, AI
  decisions, and policy decisions false. Public package/barrel exports remain
  pending source/package coordination; no account identity proof, feed
  recommendation analysis, messaging/contact analysis, upload monitoring,
  livestream content analysis, UI, enforcement, or product checklist update is
  claimed.
- AI-24 now adds schema-backed provider degraded/fallback decision contracts in
  `packages/activity-domain/src/browser-ai-provider-fallback-schemas.ts`.
  Fallback decisions join the existing local provider, family AI hub, and
  parent-approved remote route proofs with one visible/auditable result. Local,
  family-hub, and remote selections must match the selected route runtime;
  remote selections also require explicit parent approval and local safety
  fallback, and no selected local or family-hub route. Metadata-only and no-AI
  fallbacks keep runtime refs null and expose fallback action/reason labels.
  Decisions reject hidden fallback, claimed AI analysis results, claimed policy
  decisions, disabled local safety, remote default blocking, remote outage
  disables local safety, selected remote fallback while local or family hub is
  already selected, and selected runtime mismatches. Activity-domain package
  subpath exports and the AI-24 proof pack are present; no model execution,
  provider call, policy evaluator, UI, enforcement, runtime delivery, or product
  checklist update is claimed.
- AI-25 now adds a deterministic proof-gate script in
  `scripts/test/browser-url-video-ai-proof-gates.mjs`. The gate validates AI-01
  through AI-24 checklist status/owner/proof-directory references, required
  source/security/validation proof files, UI-not-applicable markers where
  applicable, rendered child/parent UI proof artifacts for AI-19 and AI-20, plan
  checkpoint mentions, and critical no-claim rollout guard text. Its refreshed
  generated manifest records 24 contract-proof rows and zero
  partial/manual-required rows for AI-01 through AI-24. Browser AI proof-gate
  coverage is complete, while product rollout remains unclaimed. No runtime
  model execution, final policy authority, enforcement, or product checklist
  update is claimed.

If these become implementation assignments, create focused workpack files or
worker messages before code changes. Do not mix all 25 into one PR.

## Required Test Categories

- URL parser tests.
- Platform parser tests.
- Metadata extraction tests.
- Hidden loader safety tests.
- AI input schema tests.
- AI output schema tests.
- Invalid model output tests.
- Provider routing tests.
- Memory hit/miss/stale tests.
- Policy integration tests.
- Real-time timeout tests.
- Post-analysis action tests.
- Playwright child/parent UX tests.
- Manual model validation tests.

Critical proof:

- AI cannot enforce directly.
- Remote AI cannot run by default.
- Memory cannot drive block without evidence refs.
- Hidden analysis cannot use child cookies by default.
- Unknown/timeout must become explicit parent fallback.

## Must-Not-Claim List

Do not claim:

- Ocentra analyzes YouTube video content from URL alone;
- platform labels prove safety;
- comments are safe without comment evidence;
- active tab is known if only target list exists;
- dynamic feeds can be classified permanently;
- remote AI is default;
- blocking was real time if analysis finished later;
- login-only content can be inspected without permission;
- AI memory can be remembered forever without expiry/source refs.

## Minimum MVP

The first useful MVP is:

- managed Edge/Chrome exact URL evidence exists;
- URL shape classifier detects YouTube video, short, channel, and search;
- memory cache checks video id and domain;
- metadata extractor uses title/description available from browser/metadata;
- local AI classifies educational, entertainment, risky, or unknown;
- policy maps result to allow, warn, parent-review, time-limit, block, or unknown;
- unknown/timeout fallback works;
- parent sees evidence, AI result, policy rule, and action;
- unmanaged browser remains bypass/process-only evidence.

No full hidden page load is required for the first MVP if metadata plus local AI
proves value.

## Product Done Signal

This layer is product-credible only when:

- a child opens a supported managed browser URL;
- Ocentra captures exact URL evidence;
- Ocentra classifies URL/video with typed local AI result or explicit degraded
  state;
- the result cites evidence, model, prompt, memory, and parent rule refs;
- policy deterministically chooses allow, warn, ask, limit, block, or unknown;
- action applies only if enforcement capability exists;
- the parent can see why;
- the child sees calm, understandable UI;
- the same URL/video can be remembered with expiry and evidence refs;
- remote AI is optional, parent-approved, and never required for normal safety.

Final rule:

```text
Do not block because AI said so.
Block only because parent policy matched a schema-valid, evidence-backed AI
result and a proved adapter can enforce it.
```
