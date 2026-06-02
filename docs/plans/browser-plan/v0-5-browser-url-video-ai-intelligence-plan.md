# V0.5 Browser URL And Video AI Intelligence Plan

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
-> allow / warn / ask-parent / limit / block / unknown
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
  category classification, ask-parent summary, or policy decision support;
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
  policy decides ask-parent;
- parent blocks adult content always, AI returns high adult risk, policy
  decides block if adapter proof exists;
- AI returns low confidence or missing transcript, policy decides unknown,
  ask-parent, warn, or stricter fallback based on parent settings.

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
- ask-parent: temporary hold or allow-with-pending based on parent setting;
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
- known ask-parent: ask immediately;
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
- policy maps result to allow, warn, ask-parent, time-limit, block, or unknown;
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
