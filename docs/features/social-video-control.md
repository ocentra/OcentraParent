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
  delivery, and enforcement. The SOCIAL-08 live proof captures real public
  Reddit, Twitch, TikTok, Instagram, and YouTube Shorts route surfaces with
  Playwright, stores route-only URL/title hashes and screenshots, and parses the
  captures through the classifier without storing page body, DOM, feed content,
  messages, credentials, or recommendation semantics.
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
- `social-policy-schedule-time-budget-proof` now strengthens SOCIAL-12 so
  contract-only social decision candidates must carry explicit schedule and
  time-budget refs plus schedule/time-budget states. Manual-required or
  unavailable schedule/time-budget states remain non-final fallback candidates;
  final policy execution, runtime gates, and enforcement remain unclaimed.
- Browser-plan SOCIAL-13 now adds managed-browser account gate-plan contracts
  from route-only account-flow evidence, sanitized form-shape evidence, and
  policy/approval refs. They model candidate holds, blocks, manual review, and
  unknown warning states while rejecting browser runtime pause/block claims,
  UI delivery, final policy decisions, credentials, form submission, account
  creation, connector/native claims, and enforcement. The refreshed proof uses
  real public browser captures for Facebook signup, Pinterest login, Reddit
  register, and Instagram signup and persists only route-only URLs, screenshots,
  title hashes, sanitized visible control kinds, parsed plan summaries, and
  no-claim negative checks.
- Browser-plan SOCIAL-14 now adds managed-browser feed/short/video route
  gate-plan contracts from typed feed classification, bounded metadata, and
  policy/approval/time-limit refs. They model route action candidates while
  rejecting browser block/redirect/CSS hide/tab close execution, applied time
  limits, UI delivery, final policy, content capture, recommendation modeling,
  connector/native claims, and enforcement. Live proof now uses Playwright
  against real public Reddit, Twitch, TikTok, Instagram, YouTube, and Vimeo
  surfaces, stores screenshots plus redacted hashes/statuses, and validates five
  route-gate candidate plans through the built contracts without storing raw
  page bodies, DOM, titles, credentials, form submissions, feed semantics, or
  video content.
- Browser-plan SOCIAL-15 now adds unmanaged social bypass detector contracts.
  They turn redacted unmanaged/browser-like process evidence into bypass-only
  managed-browser-required social evidence while rejecting exact URLs, route
  proof, account/feed/video/message proof, UI delivery, process control,
  connector/native claims, and enforcement. SOCIAL-15 also has a local
  live-process proof that launches a real system browser against public
  social/video surfaces, records only redacted executable/process/command/target
  refs, and feeds that process-only evidence through the detector contract.
- Browser-plan SOCIAL-16 now adds Android native social app capability matrix
  contracts. They keep Android social native app support at app-level,
  permission-required, manual-required, unavailable, or not-implemented states
  while rejecting native route proof, per-video/per-reel blocking, content
  capture, account identity, connector/UI/runtime-adapter, and enforcement
  claims. SOCIAL-16 also has a real host proof that checks the local adb
  installation and attached-device state; the current proof records adb present
  with no attached device and keeps Android native app support manual-device
  proof required.
- Browser-plan SOCIAL-17 now adds iOS Screen Time/ManagedSettings social
  capability matrix contracts. They keep iOS social native app support
  entitlement-required, token-selection-required, or
  manual-device-proof-required across FamilyControls authorization,
  application-token selection, web-domain-token selection, DeviceActivity
  monitor state, and ManagedSettings shield states while rejecting Apple
  entitlement approval, raw app identity, route proof, content capture,
  connector/UI/runtime-adapter, and enforcement claims. SOCIAL-17 also has a
  real host proof for this lane's Apple/iOS tooling boundary; the current
  Windows proof records no Darwin host, no Apple tooling, no attached iOS
  device, and host-tooling-unavailable state.
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
- Browser-plan SOCIAL-20 now also renders an honest parent Browser-route social
  dashboard in the real portal from the service-backed Rust WebSocket
  `agent.browser.social-dashboard.read-model.reported` event. The proof captures
  desktop/mobile screenshots for the six-row parent social snapshot at
  `output/browser-plan-proof/social-20-parent-social-dashboard-ux/06-ui-snapshots/`
  and records `test-results/social-dashboard-ui-proof/proof.json`. This proves
  the rendered parent social dashboard read-model only; connector authorization,
  native app control, final policy execution, notifications, and enforcement
  remain unclaimed.
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
  explanation/audit rows with evidence and policy links, and the service now
  reports them through `agent.browser.social-audit-explanation.read-model.get`.
  They still reject runtime audit stores, notifications, raw
  account/video/message content, connector authorization, native app control,
  final policy decisions, and enforcement.
- Browser-plan SOCIAL-23 now adds a proof artifact gate for SOCIAL-01 through
  SOCIAL-22. The generated manifest checks checklist ownership, proof directory
  references, required proof files, README references, and feature/expectation
  coverage. SOCIAL-20 has rendered parent Browser-route dashboard proof,
  SOCIAL-21 has child-agent-served intervention proof, and SOCIAL-22 has
  rendered parent explanation proof plus service-backed explanation read-model
  proof, while connector/native runtime, final policy, and enforcement proof
  remain manual-required.
- Browser-plan SOCIAL-24 now adds rollout/manual-required status labels for
  SOCIAL-01 through SOCIAL-23. SOCIAL rollout state: partial/manual-required.
  Product completion remains unclaimed; the rendered SOCIAL-20/SOCIAL-21/SOCIAL-22
  proof does not upgrade connector/native runtime, final policy, enforcement, or
  release readiness.
- Browser-plan package export closure now exposes the existing parent-domain
  social contract modules as public package subpaths and reconciles the
  browser-plan docs to stop treating package exports as the remaining blocker.
  Runtime connector behavior, native app control, rendered UI, final policy
  execution, enforcement, and product checklist completion remain unclaimed.
- `social-alert-report-intent-proof` now adds parent-domain alert/report intent
  contracts for high-risk social signals, account approval alerts, feed/video
  gate alerts, weekly summaries, manual-required states, and unavailable
  capability states. The contract links dashboard panel refs, explanation refs,
  evidence refs, policy refs, audit refs, optional parent report/action refs,
  and local-outbox refs while rejecting raw account/video/message content,
  screenshots, provider delivery, report delivery, parent notification UI,
  final policy decisions, and enforcement.
- `social-parent-sensitivity-settings-proof` now adds parent-domain sensitivity
  setting contracts for high-risk alerts, feed/video review, account-flow review,
  connector data use, native-app gap review, and weekly summary sensitivity
  rows. Contract-only policy candidate rows require source/privacy refs, AI
  aggregate refs, dashboard refs, evidence refs, schedule refs, and time-budget
  refs while rejecting raw messages/video, screenshots, connector tokens/API
  calls, runtime settings UI, final policy decisions, and enforcement.
- Browser-plan AI-23 now adds live dynamic social URL proof for real public
  Instagram, TikTok, Facebook, Twitch, X/Twitter, Reddit, and Discord route
  surfaces. The proof stores only response statuses, content types, lengths,
  hashes, redirect host/path hashes, typed route ids, and no-claim flags while
  rejecting account/content capture, AI decisions, policy decisions, native app
  control, connector access, UI delivery, enforcement, and product checklist
  completion claims.
- `social-video-source-privacy-proof` now adds an activity-domain source/privacy
  evidence-summary contract for first-class social/video source refs. It can
  cite managed-browser social route refs, bounded social/video metadata refs,
  parent-provided URL/channel refs, optional connector authorization refs,
  screen-summary refs, and platform/native manual-required states while
  rejecting raw content, raw messages, raw video, screenshots, connector tokens,
  connector API calls, native app control, final policy decisions, and
  enforcement.
- `social-video-source-custody-settings-proof` now adds an activity-domain
  source permission and custody settings contract over source/privacy evidence
  refs. It can model enabled redacted-ref use, parent-review-required connector
  refs, disabled/manual-required/unavailable source states, retention labels,
  and parent review refs while rejecting raw message/video custody, screenshots,
  connector tokens/API calls, runtime settings UI, runtime custody mutation,
  final policy decisions, and enforcement.
- `social-video-ai-signal-aggregate-proof` now adds an activity-domain
  aggregate contract that links the source/privacy summary ref to existing
  bounded social AI analysis result refs, candidate risk/benefit signal-set
  refs, and managed-browser route gate/action candidate refs. It keeps raw
  content, raw messages, raw video, screenshots, connector tokens/API calls,
  native app control, final policy decisions, alert delivery, rendered UI, and
  enforcement as explicit non-claims.
- Browser-plan AI-22 now has live Vimeo and generic VideoObject metadata proof
  for the managed-browser URL/video evidence boundary. The proof uses real
  public Vimeo and TED pages, stores only redacted hashes/lengths/statuses, and
  keeps runtime AI execution, policy authority, UI, connector/native behavior,
  and enforcement unclaimed.
- First-class social/video product contracts and runtime proof are incomplete.

## Current Gap

Ocentra does not yet have complete social/message/video sources, privacy
settings, delivered alerts, confidence handling, platform proof, or parent UI.

## Checklist

- [ ] Social platform and video target contracts.
- [ ] Source permissions and custody settings.
      `social-video-source-custody-settings-proof` exists for contract-only
      source permission and custody rows over source/privacy refs; parent
      settings UI, runtime custody settings remain.
- [ ] URL/video/channel/app evidence summaries. Source/privacy summary proof can
      cite URL/channel refs and existing managed-browser refs; full app/native
      and rendered explanation proof remain.
- [ ] Local AI analysis path with confidence. AI signal aggregate proof now
      links source/privacy, candidate AI analysis, candidate signals, and route
      gate/action refs without claiming runtime AI execution or final policy.
- [ ] Parent sensitivity settings. Sensitivity settings contract proof exists
      for ref-only policy/alert candidates; runtime settings UI, runtime
      custody mutation, final policy, and enforcement remain.
- [ ] Alert and report integration. Alert/report intent proof exists with
      ref-only local-outbox/report linkage; provider delivery, rendered
      notification UI, final policy, and enforcement remain.
- [ ] Policy schedule/time-budget integration. Compiler contract proof now
      requires explicit schedule and time-budget refs/states; runtime policy
      execution, applied schedules/budgets, and enforcement remain.
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
