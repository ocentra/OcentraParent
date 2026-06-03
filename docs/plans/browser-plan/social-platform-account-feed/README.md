# Social Platform Account Feed Workpacks

This folder owns the focused browser-plan workpacks for managed-browser social
platform account, feed, short-video, livestream, messaging-route, upload/post,
and bypass evidence. It turns the
[V0.5 Social Platform Account Feed And Gating Plan](../v0-5-social-platform-account-feed-gating-plan.md)
into small implementation rows with proof roots.

## Source Boundaries

Browser-plan-owned:

- managed-browser exact URL route evidence for social platforms;
- managed-browser page/form-shape evidence when a later adapter proves it;
- managed-browser account creation, login, account-switch, feed, reel, short,
  livestream, messaging-route, upload/post, and settings route gates;
- unmanaged browser social use as bypass/process evidence only;
- browser-plan proof packs for contract, parser, UI-not-applicable, manual, and
  rollout rows.

Adjacent, not owned here:

- native social app runtime control belongs to app/game and mobile platform
  rows until browser evidence is the source;
- platform connector authorization is parent-approved and optional;
- screen/message/media capture needs the screen-evidence and privacy review
  paths before any product claim;
- parent policy decisions belong to typed policy contracts and cannot be
  replaced by route evidence or AI output;
- child/parent visual surfaces need explicit UI proof before delivery claims.

## Workpack Map

| Row       | Boundary                                  | First Proof Root                                                                    |
| --------- | ----------------------------------------- | ----------------------------------------------------------------------------------- |
| SOCIAL-01 | Plan folder and README                    | `output/browser-plan-proof/social-01-social-video-gating-plan-folder-readme/`       |
| SOCIAL-02 | Platform and route contract schemas       | `output/browser-plan-proof/social-02-platform-route-contracts/`                     |
| SOCIAL-03 | Social URL pattern library                | `output/browser-plan-proof/social-03-social-url-pattern-library/`                   |
| SOCIAL-04 | Signup/login/account-switch evidence      | `output/browser-plan-proof/social-04-account-flow-evidence-contracts/`              |
| SOCIAL-05 | Managed DOM/form-shape detector           | `output/browser-plan-proof/social-05-managed-dom-form-shape-detector/`              |
| SOCIAL-06 | Social account identity registry          | `output/browser-plan-proof/social-06-social-account-identity-registry/`             |
| SOCIAL-07 | Parent approval request/decision          | `output/browser-plan-proof/social-07-parent-approval-contracts/`                    |
| SOCIAL-08 | Feed/reels/shorts route classification    | `output/browser-plan-proof/social-08-feed-reels-shorts-route-classification/`       |
| SOCIAL-09 | Video/social metadata extractor           | `output/browser-plan-proof/social-09-video-social-metadata-extractor/`              |
| SOCIAL-10 | Social AI analysis contracts              | `output/browser-plan-proof/social-10-social-ai-analysis-contracts/`                 |
| SOCIAL-11 | Social risk/benefit signal model          | `output/browser-plan-proof/social-11-social-risk-benefit-signal-model/`             |
| SOCIAL-12 | Parent policy compiler for social targets | `output/browser-plan-proof/social-12-parent-policy-compiler-social-targets/`        |
| SOCIAL-13 | Managed browser account creation gate     | `output/browser-plan-proof/social-13-managed-browser-account-creation-gate/`        |
| SOCIAL-14 | Managed browser feed/short/video gate     | `output/browser-plan-proof/social-14-managed-browser-feed-short-video-gate/`        |
| SOCIAL-15 | Unmanaged social bypass detector          | `output/browser-plan-proof/social-15-unmanaged-social-bypass-detector/`             |
| SOCIAL-16 | Android native-app capability matrix      | `output/browser-plan-proof/social-16-android-native-app-capability-matrix/`         |
| SOCIAL-17 | iOS Screen Time/ManagedSettings matrix    | `output/browser-plan-proof/social-17-ios-screentime-managedsettings-matrix/`        |
| SOCIAL-18 | Platform connector authorization boundary | `output/browser-plan-proof/social-18-platform-connector-authorization-boundary/`    |
| SOCIAL-19 | Memory/cache decisions                    | `output/browser-plan-proof/social-19-memory-cache-account-video-channel-decisions/` |
| SOCIAL-20 | Parent social dashboard UX                | `output/browser-plan-proof/social-20-parent-social-dashboard-ux/`                   |
| SOCIAL-21 | Child approval/block UX                   | `output/browser-plan-proof/social-21-child-approval-block-ux/`                      |
| SOCIAL-22 | Audit and explanation read model          | `output/browser-plan-proof/social-22-audit-explanation-read-model/`                 |
| SOCIAL-23 | Tests, fixtures, Playwright, manual proof | `output/browser-plan-proof/social-23-tests-fixtures-playwright-manual-proof/`       |
| SOCIAL-24 | Rollout and manual-required labels        | `output/browser-plan-proof/social-24-rollout-manual-required-labels/`               |

## Proof Rules

- Start every row from managed exact browser evidence or explicit
  manual-required/unavailable state.
- Keep route evidence, page/form-shape evidence, account identity, AI analysis,
  policy decision, action, and UI delivery as separate proof layers.
- Use `ui-not-applicable.md` for contract-only rows.
- Use screenshots only when a parent or child visual surface changes.
- Preserve no-claim language for native apps, platform connectors, raw messages,
  screenshots/media, account identity proof, and remote AI defaults.

## Current State

SOCIAL-01 only creates this workpack home and proof-root map. It does not add
schemas, parsers, runtime adapters, policy decisions, UI delivery, platform
connector logic, native app support, or enforcement.

SOCIAL-02 now adds schema-backed platform and route evidence contracts in
`packages/activity-domain/src/browser-social-platform-route-schemas.ts`. The
contracts reference URL-shape classification ids for managed-browser route
evidence and preserve manual-required/bypass-only states for unmanaged or native
app sources. Public package/barrel exports remain pending source/package
coordination.

SOCIAL-03 now adds
`packages/activity-domain/src/browser-social-url-patterns.ts`, a deterministic
pattern adapter from exact managed URL-shape classifications to validated
browser social route evidence. It maps known social domains and route patterns,
including signup/login/account-switch/settings, messaging, upload, livestream,
feed/profile/post/video, Snapchat, and Pinterest, while rejecting unmanaged
browser evidence and fake-domain rows. Public package/barrel exports remain
pending source/package coordination.

SOCIAL-04 now adds
`packages/activity-domain/src/browser-social-account-flow-schemas.ts`, a
route-only account-flow evidence contract for managed-browser signup, login, and
account-switch social routes. It can represent manual-required account-flow
states, but it does not prove account identity, credentials, form submission,
completed account creation, login success, parent approval decisions, policy
decisions, UI delivery, connector access, native app control, or enforcement.
Public package/barrel exports remain pending source/package coordination.

SOCIAL-05 now adds
`packages/activity-domain/src/browser-social-form-shape-detector.ts`, a
sanitized control-kind detector for signup, login, and account-switch form
shapes. It accepts account-flow route evidence plus control kinds only and
rejects field values, raw DOM, credentials, form submission, account identity,
parent approval decisions, policy decisions, UI delivery, connector access,
native app control, and enforcement. Public package/barrel exports remain
pending source/package coordination.

SOCIAL-06 now adds
`packages/activity-domain/src/browser-social-account-identity-registry.ts`, a
privacy-preserving identity registry contract. It supports unverified
route-context entries, parent-declared hash refs, and manual-required state, but
does not capture raw handles, display names, platform account ids, credentials,
platform verification, connector authorization, parent UI, policy decisions,
native app control, or enforcement. Public package/barrel exports remain pending
source/package coordination.

SOCIAL-07 now adds `packages/parent-domain/src/social-parent-approval.ts`, a
parent-domain request/decision contract for social account signup, login,
account-switch, and manual-required approval states. It references evidence ids
without importing activity-domain and rejects raw account data, credentials,
notification delivery, UI rendering, child notification, policy/action
execution, connector authorization, native app control, and enforcement. Public
package/barrel exports remain pending source/package coordination.

SOCIAL-08 now adds
`packages/activity-domain/src/browser-social-feed-route-classification.ts`, a
route-only classifier for dynamic feeds, short-video feed surfaces, and exact
single-short-video routes. It consumes validated route evidence and sanitized
surface hints only, without claiming feed content, recommendations, messages, AI
decisions, policy decisions, connector access, native app control, UI delivery,
or enforcement. Public package/barrel exports remain pending source/package
coordination.

SOCIAL-09 now adds
`packages/activity-domain/src/browser-social-video-metadata.ts`, a bounded
metadata-ref extractor for managed social video/post/feed route evidence. It can
record title, description, author hash, thumbnail hash, duration, publish date,
category, and restriction refs, but rejects page body, transcript text, messages,
feed content, AI decisions, policy decisions, connector access, native app
control, UI delivery, and enforcement. Public package/barrel exports remain
pending source/package coordination.

SOCIAL-10 now adds social-specific AI analysis contracts in
`packages/activity-domain/src/browser-social-ai-analysis-values.ts`,
`packages/activity-domain/src/browser-social-ai-analysis-schemas.ts`, and
`packages/activity-domain/src/browser-social-ai-analysis-result-builder.ts`.
They define typed social analysis tasks, prompt-template boundaries, input
evidence refs, candidate classifications, confidence, uncertainty, model runtime
refs, and degraded states for managed-browser social route evidence. They reject
raw browser/page/feed/message/transcript/screenshot/native/connector state,
final policy actions, enforcement, raw model text/content storage, native app
control, connector claims, and inconsistent degraded states. Public
package/barrel exports remain pending source/package coordination.

SOCIAL-11 now adds
`packages/activity-domain/src/browser-social-risk-benefit-values.ts` and
`packages/activity-domain/src/browser-social-risk-benefit-signals.ts`, a
candidate social risk/benefit signal model sourced from typed SOCIAL-10 analysis
results. Signal rows carry canonical risk or benefit kinds, severity, state,
confidence, and evidence refs, while signal sets copy analysis provenance and
degraded state. They reject raw message/feed/page/model use, account identity
verification claims, final policy decisions, native app control, connector
claims, UI delivery, and enforcement. Public package/barrel exports remain
pending source/package coordination.

SOCIAL-12 now adds parent-domain social policy compiler contracts in
`packages/parent-domain/src/social-policy-compiler-values.ts` and
`packages/parent-domain/src/social-policy-compiler.ts`. The compiler consumes
parent-owned evidence, signal-set, parent-rule, and schedule refs and produces
decision candidates for allow, warn, ask-parent, block, manual-review, or
unknown outcomes. It rejects raw signal payloads, raw model text,
activity-domain object transfer, UI/runtime/enforcement, native app, and
connector claims. Decision candidates are not final policy decisions or
enforcement handoffs. Public package/barrel exports remain pending
source/package coordination.

SOCIAL-13 now adds
`packages/activity-domain/src/browser-social-account-creation-gate.ts`, a
managed-browser account gate-plan contract for route-only account-flow evidence
and sanitized form-shape evidence. It can model allow-navigation,
hold-for-parent-approval, block-submit, manual-review, and unknown-warn
candidates while rejecting browser runtime pause/block claims, child/parent UI,
final policy decisions, credentials, form submissions, account creation, native
app control, connector claims, and enforcement. Public package/barrel exports
remain pending source/package coordination.

SOCIAL-14 now adds
`packages/activity-domain/src/browser-social-feed-video-route-gate-values.ts`,
`packages/activity-domain/src/browser-social-feed-video-route-gate-guards.ts`,
and `packages/activity-domain/src/browser-social-feed-video-route-gate.ts`, a
managed-browser feed/short/video route gate-plan contract. It combines typed
feed route classification, bounded video metadata evidence, and
policy/approval/time-limit refs to model allow, warn, ask-parent, block, limit,
manual-review, and unknown-warn candidates. It rejects browser navigation block
execution, redirects, CSS/DOM hiding, tab closing, applied time limits,
child/parent UI, final policy decisions, feed/video content capture,
recommendation modeling, native app control, connector claims, and enforcement.
Public package/barrel exports remain pending source/package coordination.

SOCIAL-15 now adds
`packages/activity-domain/src/browser-social-unmanaged-bypass-detector-values.ts`
and `packages/activity-domain/src/browser-social-unmanaged-bypass-detector.ts`,
an unmanaged social bypass detector contract. It converts redacted unmanaged or
browser-like process evidence into bypass-only social evidence with
managed-browser-required state. It rejects exact URL proof, managed-session
boundaries, route evidence, social account proof, feed/video route proof,
messages, account identity, native app control, connector access, child/parent
UI, process termination, managed browser relaunch, and enforcement. Public
package/barrel exports remain pending source/package coordination.

SOCIAL-16 now adds
`packages/parent-domain/src/social-android-native-app-capability-matrix-values.ts`
and `packages/parent-domain/src/social-android-native-app-capability-matrix.ts`,
an Android native social app capability matrix. It covers package visibility,
UsageStats foreground evidence, accessibility route hints, VPN/domain hints,
device-owner app control, and managed-profile config as app-level,
permission-required, manual-required, unavailable, or not-implemented states. It
rejects native route proof, per-video/per-reel blocking, messages, account
identity, accessibility content capture, device-owner enrollment, VPN content
inspection, runtime adapters, connector access, UI delivery, and enforcement.
Public package/barrel exports remain pending source/package coordination.

SOCIAL-17 now adds
`packages/parent-domain/src/social-ios-screen-time-capability-matrix-values.ts`
and `packages/parent-domain/src/social-ios-screen-time-capability-matrix.ts`, an
iOS Screen Time/ManagedSettings capability matrix. It keeps Apple entitlement,
token selection, DeviceActivity, and ManagedSettings shield states explicit
while rejecting raw app identity, native route proof, content capture, runtime
adapters, connector access, UI delivery, and enforcement.

SOCIAL-18 now adds
`packages/parent-domain/src/social-platform-connector-authorization-values.ts`
and `packages/parent-domain/src/social-platform-connector-authorization.ts`, a
platform connector authorization boundary for optional Google/YouTube, Meta,
TikTok, platform export/import, and parent-provided account refs. It models
parent authorization, custody, scopes, expiry, revocation, manual-required
state, and proof refs without token storage, OAuth clients, provider APIs, raw
account/message/feed data, policy decisions, UI, native control, or enforcement.

SOCIAL-19 now adds
`packages/parent-domain/src/social-decision-memory-cache-values.ts` and
`packages/parent-domain/src/social-decision-memory-cache.ts`, a bounded
decision-memory contract for account, video, and channel refs. Fresh hits may
feed policy input only when decision refs are present and no invalidation
reasons exist; stale, miss, and manual-required rows cannot. It does not claim a
runtime cache store, raw content storage, connector data storage, UI, native
control, final policy decisions, or enforcement.

SOCIAL-20 now adds `packages/parent-domain/src/social-dashboard-ux-values.ts`
and `packages/parent-domain/src/social-dashboard-ux.ts`, parent social dashboard
UX section contracts for account approvals, feed/video gates, native app
capability, connector boundaries, decision memory, and manual-required gaps. It
does not claim rendered portal UI, runtime fetches, notifications, connector
authorization, native control, policy execution, or enforcement.

SOCIAL-21 now adds
`packages/parent-domain/src/social-child-approval-block-ux-values.ts` and
`packages/parent-domain/src/social-child-approval-block-ux.ts`, child-facing
approval/block UX state/action contracts for approval pending, blocked route
candidates, warnings, manual review, time-limit candidates, and native app
unavailable states. It does not claim rendered child UI, notifications, browser
block execution, block-page rendering, time-limit application, policy execution,
native control, connector authorization, or enforcement.

SOCIAL-22 now adds
`packages/parent-domain/src/social-audit-explanation-read-model-values.ts` and
`packages/parent-domain/src/social-audit-explanation-read-model.ts`, a ref-only
audit/explanation read-model contract for account approval, feed/video gate,
native-app gap, connector boundary, decision memory, and manual-required gap
rows. It links evidence, policy, parent approval, memory, manual gap, and audit
refs while rejecting runtime audit stores, rendered explanation UI,
notifications, raw account/video/message content, connector authorization,
native app control, final policy decisions, and enforcement.

SOCIAL-23 now adds `scripts/test/social-platform-account-feed-proof-artifacts.mjs`,
a proof artifact gate for SOCIAL-01 through SOCIAL-22. It validates checklist
ownership, proof directory references, required proof files, README references,
and feature/expectation coverage, then writes
`test-results/social-platform-account-feed-proof-artifacts/proof.json` and
`output/browser-plan-proof/social-23-tests-fixtures-playwright-manual-proof/01-social-proof-artifact-manifest.md`.
Playwright remains manual-required because the current social rows are
contract/proof rows without rendered social UI.

SOCIAL-24 now adds `scripts/test/social-platform-account-feed-rollout-gate.mjs`,
a rollout/manual-required label gate for SOCIAL-01 through SOCIAL-23. It writes
`test-results/social-platform-account-feed-rollout-gate/proof.json` and
`output/browser-plan-proof/social-24-rollout-manual-required-labels/01-rollout-manual-required-labels.md`.
SOCIAL rollout state: partial/manual-required. Product checklist upgrade is not
claimed. Rendered social UI, Playwright screenshots, connector/native runtime,
final policy execution, enforcement, release readiness, and product completion
remain unclaimed.
