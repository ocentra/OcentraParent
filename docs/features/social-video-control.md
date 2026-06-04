# Social And Video Control

## Parent Outcome

Parents can configure rules for social apps, messaging surfaces, video
platforms, channels, video URLs, categories, schedules, and time budgets, with
evidence-backed explanations and alerts where platforms allow it.

## Ocentra Requirement

Social and video control must be first-class. It cannot be hidden under generic
app blocking or vague AI claims. The product must name the source, permission,
privacy boundary, confidence, parent rule, and action.

## Roadmap And Expectations

- Roadmap: V5 parent policy product, V6 mobile agents, V3 notifications.
- Expectations: [social/video](../expectations/social-video-control.md),
  [policy](../expectations/policy.md), [AI](../expectations/ai.md),
  [screen evidence](../expectations/screen-evidence.md).
- Browser URL/video intelligence planning:
  [browser URL and video AI intelligence plan](../plans/browser-plan/v0-5-browser-url-video-ai-intelligence-plan.md).
- Browser social/platform gating planning:
  [social platform account feed and gating plan](../plans/browser-plan/v0-5-social-platform-account-feed-gating-plan.md).
- Modules: `packages/parent-domain`, `packages/activity-domain`,
  `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
video safety, social app controls, and message/content monitoring.

Bark, Qustodio, FamiSafe, FamilyTime, and similar products emphasize social,
message, video, and alert coverage. This is one of Ocentra's highest-risk gaps.

## Current Ocentra State

- App/category policy intent exists.
- Browser URL, app sessions, screen summaries, and local AI architecture can
  support future social/video evidence.
- The browser-plan social workpack folder now maps managed-browser social
  account/feed rows to proof roots and keeps native apps, connectors,
  screen/message capture, UI delivery, policy authority, and enforcement as
  explicit adjacent or future proof boundaries.
- Browser-plan SOCIAL-02 now adds activity-domain social platform route
  evidence contracts for managed-browser URL-shape social routes,
  unmanaged-bypass states, and native-app manual-required states. These
  contracts reject account identity proof, message/feed content semantics, AI
  authority, policy authority, connector claims, native app control, and
  enforcement.
- Browser-plan SOCIAL-03 now adds an activity-domain social URL pattern adapter
  from exact managed URL-shape classifications to validated social route
  evidence. It covers known social domains and route patterns for signup,
  login, account switch, settings/privacy, messaging, upload/post, livestream,
  feed, profile, post, and video routes, while rejecting unmanaged browser
  rows and fake-domain rows. It does not prove account identity, message/feed
  content, AI decisions, policy decisions, connector access, native app control,
  UI delivery, or enforcement.
- Browser-plan SOCIAL-04 now adds route-only signup/login/account-switch
  evidence contracts. These contracts can link managed-browser account-flow
  signals back to social route evidence and can represent manual-required
  unsupported sources, but they reject account identity, credentials, form
  submission, completed account creation, login success, parent approval
  decisions, policy decisions, connector access, native app control, UI
  delivery, and enforcement.
- Browser-plan SOCIAL-05 now adds a sanitized form-shape detector contract for
  signup, login, and account-switch forms. It accepts control-kind hints linked
  to route-only account-flow evidence and rejects field values, raw DOM,
  credentials, form submission, account identity, parent approval decisions,
  policy decisions, connector access, native app control, UI delivery, and
  enforcement.
- Browser-plan SOCIAL-06 now adds a privacy-preserving social account identity
  registry contract. It supports unverified route-context entries,
  parent-declared hash refs, and manual-required state, while rejecting raw
  handles, display names, platform account ids, credentials, platform
  verification, connector authorization, policy decisions, native app control,
  UI delivery, and enforcement.
- Browser-plan SOCIAL-07 now adds parent-domain approval request/decision
  contracts for social account signup, login, account switch, and
  manual-required states. They reference evidence ids without importing
  activity-domain and reject raw account data, credentials, notification
  delivery, UI rendering, child notification, policy/action execution, connector
  authorization, native app control, and enforcement.
- Browser-plan SOCIAL-08 now adds route-only feed/reels/shorts classification
  contracts. They distinguish dynamic feeds, short-video feed surfaces, and exact
  single-short-video routes from managed-browser route evidence plus sanitized
  surface hints, while rejecting feed content, recommendations, messages, AI
  decisions, policy decisions, connector access, native app control, UI
  delivery, and enforcement.
- Browser-plan SOCIAL-09 now adds bounded social/video metadata-ref extraction
  contracts for managed social video, post, or feed routes. They can record
  title, description, author hash, thumbnail hash, duration, publish date,
  category, and restriction refs, while rejecting page body, transcript text,
  messages, feed content, AI decisions, policy decisions, connector access,
  native app control, UI delivery, and enforcement.
- Browser-plan SOCIAL-10 now adds social-specific AI analysis contracts for
  managed-browser social evidence. They define task-scoped inputs, prompt
  templates, candidate classifications, confidence, uncertainty, runtime refs,
  and degraded states while rejecting raw browser/page/feed/message/transcript
  or screenshot state, final policy actions, enforcement, raw model/content
  storage, connector claims, native app control, UI delivery, and runtime AI
  execution claims.
- Browser-plan SOCIAL-11 now adds candidate social risk/benefit signal model
  contracts. Signal rows carry canonical risk or benefit kinds, severity,
  state, confidence, and evidence refs from typed social AI analysis results
  while rejecting raw message/feed/page/model use, account identity verification
  claims, final policy decisions, connector/native claims, UI delivery, and
  enforcement.
- Browser-plan SOCIAL-12 now adds parent-domain social policy compiler
  contracts. They consume parent-owned evidence, signal-set, rule, and schedule
  refs to produce non-final decision candidates while rejecting raw signal
  payloads, raw model text, activity-domain object transfer, UI/runtime/native/
  connector/enforcement claims, and direct policy execution.
- Browser-plan SOCIAL-13 now adds managed-browser account gate-plan contracts
  from route-only account-flow evidence, sanitized form-shape evidence, and
  policy/approval refs. They model candidate holds, blocks, manual review, and
  unknown warning states while rejecting browser runtime pause/block claims,
  UI delivery, final policy decisions, credentials, form submission, account
  creation, connector/native claims, and enforcement.
- Browser-plan SOCIAL-14 now adds managed-browser feed/short/video route
  gate-plan contracts from typed feed classification, bounded metadata, and
  policy/approval/time-limit refs. They model route action candidates while
  rejecting browser block/redirect/CSS hide/tab close execution, applied time
  limits, UI delivery, final policy, content capture, recommendation modeling,
  connector/native claims, and enforcement.
- Browser-plan SOCIAL-15 now adds unmanaged social bypass detector contracts.
  They turn redacted unmanaged/browser-like process evidence into bypass-only
  managed-browser-required social evidence while rejecting exact URLs, route
  proof, account/feed/video/message proof, UI delivery, process control,
  connector/native claims, and enforcement.
- Browser-plan SOCIAL-16 now adds Android native social app capability matrix
  contracts. They keep Android social native app support at app-level,
  permission-required, manual-required, unavailable, or not-implemented states
  while rejecting native route proof, per-video/per-reel blocking, content
  capture, account identity, connector/UI/runtime-adapter, and enforcement
  claims.
- Browser-plan SOCIAL-17 now adds iOS Screen Time/ManagedSettings social
  capability matrix contracts. They keep iOS social native app support
  entitlement-required, token-selection-required, or
  manual-device-proof-required across FamilyControls authorization,
  application-token selection, web-domain-token selection, DeviceActivity
  monitor state, and ManagedSettings shield states while rejecting Apple
  entitlement approval, raw app identity, route proof, content capture,
  connector/UI/runtime-adapter, and enforcement claims.
- Browser-plan SOCIAL-18 now adds platform connector authorization boundary
  contracts. They model Google/YouTube supervision, Meta Family Center, TikTok
  Family Pairing, platform export/import, and parent-provided account refs as
  optional parent-authorized adjacent sources while rejecting token storage,
  OAuth clients, provider API calls, raw account/message/feed data, account
  identity verification, core gating dependency, policy decisions, AI runtime,
  UI delivery, native app control, and enforcement.
- Browser-plan SOCIAL-19 now adds parent-domain social decision memory-cache
  contracts for account, video, and channel decision refs. Fresh hits can be
  reused for policy input only when they cite decision refs and have no
  invalidation reasons; stale, miss, and manual-required rows cannot drive
  policy input. The contracts reject runtime cache stores, AI caches, raw
  account/video/message content, connector data, UI delivery, native app
  control, final policy decisions, and enforcement.
- Browser-plan SOCIAL-20 now adds parent-domain parent social dashboard UX
  contracts for the account approval queue, feed/video gates, native app
  capability, connector boundaries, decision memory, and manual-required gaps.
  These are section/action/status contracts only; they reject rendered portal
  UI, notifications, runtime data fetch, policy decisions, connector
  authorization, native app control, and enforcement.
- Browser-plan SOCIAL-21 now adds parent-domain child approval/block UX
  contracts for approval pending, blocked route candidates, warning candidates,
  manual review, time-limit candidates, and native-app unavailable states. These
  are child-facing state/action contracts only; they reject rendered child UI,
  notifications, browser block execution, block-page rendering, applied time
  limits, final policy decisions, connector authorization, native app control,
  and enforcement.
- Browser-plan SOCIAL-22 now adds parent-domain audit/explanation read-model
  contracts for account approval, feed/video gates, native-app gaps, connector
  boundaries, decision memory, and manual-required gaps. These are ref-only
  explanation/audit rows with evidence and policy links; they reject runtime
  audit stores, rendered explanation UI, notifications, raw account/video/message
  content, connector authorization, native app control, final policy decisions,
  and enforcement.
- Browser-plan SOCIAL-23 now adds a proof artifact gate for SOCIAL-01 through
  SOCIAL-22. The generated manifest checks checklist ownership, proof directory
  references, required proof files, README references, and feature/expectation
  coverage while keeping Playwright manual-required because no rendered social UI
  exists in this slice.
- Browser-plan SOCIAL-24 now adds rollout/manual-required status labels for
  SOCIAL-01 through SOCIAL-23. SOCIAL rollout state: partial/manual-required.
  Product completion remains unclaimed.
- Browser-plan package export closure now exposes the existing parent-domain
  social contract modules as public package subpaths and reconciles the
  browser-plan docs to stop treating package exports as the remaining blocker.
  Runtime connector behavior, native app control, rendered UI, final policy
  execution, enforcement, and product checklist completion remain unclaimed.
- `social-video-source-privacy-proof` now adds an activity-domain source/privacy
  evidence-summary contract for first-class social/video source refs. It can
  cite managed-browser social route refs, bounded social/video metadata refs,
  parent-provided URL/channel refs, optional connector authorization refs,
  screen-summary refs, and platform/native manual-required states while
  rejecting raw content, raw messages, raw video, screenshots, connector tokens,
  connector API calls, native app control, final policy decisions, and
  enforcement.
- First-class social/video product contracts and runtime proof are incomplete.

## Current Gap

Ocentra does not yet have complete social/message/video sources, privacy
settings, alert contracts, confidence handling, platform proof, or parent UI.

## Checklist

- [ ] Social platform and video target contracts.
- [ ] Source permissions and custody settings. Source/privacy evidence-summary
      proof exists; parent settings UI and runtime custody settings remain.
- [ ] URL/video/channel/app evidence summaries. Source/privacy summary proof can
      cite URL/channel refs and existing managed-browser refs; full app/native
      and rendered explanation proof remain.
- [ ] Local AI analysis path with confidence.
- [ ] Parent sensitivity settings.
- [ ] Alert and report integration.
- [ ] Policy schedule/time-budget integration.
- [ ] Platform-specific unavailable/manual-required states.

## Next AI Instructions

Do not claim "we analyze the video itself" until the actual input, model/runtime
path, confidence, decision, and audit output are proved. Build the source and
privacy model first. Managed-browser URL/video intelligence work belongs in the
browser plan and must feed this feature only through evidence-backed,
confidence-labeled social/video contracts. Managed-browser social account,
secondary-account, feed, short-video, livestream, messaging-route, and upload
gates belong in the browser plan until source evidence or action leaves the
browser boundary.
Browser-game cloud streams and UGC/multiplayer game risk are adjacent to
social/video risk, but the browser-game source plan owns managed-browser game
evidence until the source becomes social/video or native-app specific.
