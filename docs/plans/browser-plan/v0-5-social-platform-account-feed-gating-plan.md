# V0.5 Social Platform Account Feed And Gating Plan

<!-- agent-capsule -->

> Agent Capsule
> Plan: `browser-plan`
> Doc: `V0.5 Social Platform Account Feed And Gating Plan`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This document defines the browser-plan-owned social media control and detection
path. It belongs in this folder when the source surface is a managed browser,
browser URL/page/video evidence, unmanaged browser bypass, or browser policy
gate. Native apps, platform connectors, mobile permissions, screen evidence,
policy approvals, and local AI remain adjacent feature boundaries and must be
linked rather than duplicated.

This is a plan document only. It does not claim full platform control,
per-reel native-app blocking, private message reading, account identity proof,
or remote AI behavior until evidence, policy, adapter, custody, and audit proof
exist.

## External Platform Context

The pasted enhancement included current public-platform context. Treat it as
competitive/background pressure, not as Ocentra source truth:

- Meta continues expanding teen controls across Instagram, Facebook, Messenger,
  and Family Center. That supports the product need for platform-aware social
  controls, but Ocentra must not depend on platform account state alone.
- TikTok Family Pairing exposes parent/guardian controls such as screen-time,
  content, privacy, and safety settings. That supports the need to model
  platform controls, but Ocentra still needs device/browser evidence when the
  child switches app, browser, profile, or account.
- YouTube and kids-video research supports the claim that platform labels and
  child-safe modes are not enough. Ocentra must cite URL, metadata, transcript,
  screen, model, confidence, policy, and audit evidence before making product
  claims.

Source pointers from the enhancement:

- [Reuters-syndicated Meta teen controls coverage, June 2, 2026](https://m.investing.com/news/stock-market-news/meta-expands-teen-content-controls-globally-tests-new-instagram-tool-to-diversify-feeds-4721704?ampMode=1).
- [TikTok Family Pairing support documentation](https://support.tiktok.com/en/safety-hc/account-and-user-safety/family-pairing).
- [KidsTube: Detection, Characterization and Analysis of Child Unsafe Content and Promoters on YouTube](https://arxiv.org/abs/1608.05966).

Refresh these links before using them in marketing, PR copy, or product claims.

## Covered Platforms And Surfaces

This plan covers:

```text
Facebook
Instagram
Messenger
TikTok
Snapchat
YouTube
YouTube Shorts
Vimeo
Twitch
Discord
Reddit
X/Twitter
Pinterest
Roblox/social-game surfaces
generic social platforms
new or unknown social sites
fake/new account attempts
login/signup detection
feed/reel/short/video detection
message/contact risk signals
parent approval gates
AI analysis
memory/cache
policy decisions
enforcement
tests and proof
```

## 1. Core Product Rule

Social media control must be first-class.

```text
Social platform access is a policy target.
Social account creation is a policy target.
Social login is a policy target.
Social feed/reels/shorts are policy targets.
Social video URLs are policy targets.
Social messaging/contact risk is a policy target where evidence is allowed.
```

Ocentra must not hide social media under:

```text
generic browser block
generic app block
generic AI flag
generic screen time only
```

The parent should be able to say:

```text
No new social accounts without my approval.
Allow YouTube educational videos.
Limit Shorts/Reels/TikTok-style feeds.
Ask me before Instagram account creation.
Block Facebook signup forms.
Warn for unknown social login.
Ask parent if a fake/new account is detected.
Allow school-related videos during homework.
Block unknown livestreams during bedtime.
```

## 2. Ocentra Differentiator

Most platforms provide some built-in teen controls, but those controls depend
on:

```text
the child using the correct real age;
the child using the supervised account;
the platform classifying content correctly;
the parent linking to that platform's own supervision tool;
the platform exposing useful controls;
the child not creating a second/fake account;
the child not switching browser/profile/device;
the child not using web instead of app or app instead of web.
```

Ocentra should stand out by controlling the device/browser boundary instead of
trusting only platform account state.

Product goal:

```text
If a child tries to create or use a social account, Ocentra sees the attempt as
evidence, applies parent policy, and gates the action when capability exists.
```

## 3. Main Rule For Account Creation

Default product rule:

```text
Any new social account creation requires parent approval unless parent
explicitly allows it.
```

This includes:

```text
new Facebook account
new Instagram account
new TikTok account
new Snapchat account
new Discord account
new Reddit account
new X/Twitter account
new YouTube/Google account if configured
new Twitch account
new unknown social signup
new secondary account on an already-used platform
login to an unknown account
switching to another account
using incognito/private profile for social login
using a new browser profile for social login
```

Policy result examples:

```text
allow existing approved account
ask parent for new account
block signup
allow signup only with parent present
allow platform but block account creation
allow social browsing but block posting/messaging
allow educational video but block feed/reels/shorts
```

## 4. Evidence Before Authority

Ocentra must keep evidence types separate:

```text
URL evidence: proves page URL.
Page-shape evidence: suggests signup/login/feed/video.
Metadata evidence: title, OpenGraph, schema, platform hints.
DOM-form evidence: proves fields/buttons/signals if captured by managed page adapter.
Screen/OCR evidence: proves visible text/images only if screen feature enabled.
Process/app evidence: proves app/browser running.
Network evidence: proves destination/domain only.
AI result: classifies evidence.
Parent policy: decides action.
Enforcement adapter: executes action where proved.
```

Never claim:

```text
child made fake account
```

from one weak signal. Say instead:

```text
Possible new social account creation attempt.
Evidence: signup URL + visible create-account form + unknown account id.
Parent approval required.
```

## 5. Platform Capability Matrix

### 5.1 Web Platforms In Managed Browser

Best Ocentra coverage:

```text
URL detection
signup/login route detection
page title/metadata detection
DOM/form-shape detection if managed bridge supports it
hidden analysis load
local AI classification
block/warn/ask/limit in managed browser
```

Examples:

```text
facebook.com/r.php
instagram.com/accounts/emailsignup/
tiktok.com/signup
snapchat.com/signup
discord.com/register
reddit.com/register
x.com/i/flow/signup
twitch.tv/signup
```

### 5.2 Native Mobile/Desktop Apps

Native apps are harder. Possible evidence:

```text
app installed
app launched
foreground app
usage time
notifications metadata where allowed
screen summary where allowed
network destination
Android UsageStats / Accessibility / VPN if permissioned
iOS Screen Time / ManagedSettings where available
```

Not always possible:

```text
exact screen
exact account id
exact message content
exact feed/video URL
signup form content
```

Mobile app account creation gating usually requires one of:

```text
block app until parent approval
Device Owner / MDM / managed profile
Screen Time / ManagedSettings on iOS
Accessibility/UsageStats/VPN on Android with explicit permission
Ocentra-owned browser or web flow
manual parent setup
```

### 5.3 Platform Connectors

Platform connectors are optional and parent-authorized only.

Potential connectors:

```text
Google/YouTube account supervision
Meta Family Center state if exposed
TikTok Family Pairing state if exposed
platform export/import
parent-provided account handles
```

Do not depend on platform connectors for core gating.

## 6. Detection Layers

Use multiple layers. Each layer has different proof strength.

### Layer 1: Domain And URL Shape

Fast deterministic detection.

Detect:

```text
social domain
signup route
login route
account switch route
profile route
feed route
video route
short/reel route
messaging route
upload/post route
settings/privacy route
```

Contract family:

```text
SocialUrlShape
SocialUrlShapePlatformIds
SocialRouteKind
SocialUrlShapeReasonCode
```

Expected fields:

```text
shape id
source evidence id
normalized URL
normalized domain
platform
route kind
platform account/video/post/channel ids where available
confidence
reason codes
```

Proof strength:

```text
High for exact known routes.
Medium for generic route patterns.
Low for unknown/dynamic SPA states.
```

### Layer 2: Managed Page Metadata

Extract:

```text
page title
OpenGraph title/description
schema.org metadata
canonical URL
profile/account metadata where visible
video id/channel id/post id
app-deep-link hints
```

Good for video/page classification, platform detection, account/profile
detection, public post detection, and feed/reel/video distinction.

Not enough for actual message contents, private account activity, hidden
recommendations, or full feed semantics.

### Layer 3: Managed DOM/Form Shape

Only inside managed browser and only if implemented.

Detect:

```text
email/phone field
password field
birthdate field
username field
create account button
sign up button
continue with Google/Apple/Facebook
switch account button
logout/login state indicators
new account flow
```

Contract family:

```text
SocialAccountFlowEvidence
SocialAccountFlowKind
SocialDetectedFormField
```

Expected flow kinds:

```text
new_account_signup
login_existing_account
account_switch
profile_creation
age_entry
phone_verification
email_verification
unknown_account_flow
```

This is the main proof for account creation gating.

### Layer 4: Screen/OCR/Vision Summary

Optional and permissioned.

Useful when:

```text
SPA route hides URL state
native app has no URL
signup/login visible on screen
shorts/reels visible but URL is not enough
video content needs visual summary
```

Screen/OCR/vision must be controlled by privacy settings. Do not keep raw
screenshots permanently unless a separate setting and proof allow it.

### Layer 5: Network/Domain Observation

Useful for:

```text
social domain used
app contacted social platform
unknown browser/app accessed platform
VPN/proxy/circumvention hint
```

Not enough for account creation proof, message proof, specific video proof, or
specific feed proof.

### Layer 6: AI Analysis

AI consumes typed evidence.

AI can classify:

```text
signup attempt likelihood
fake/new account attempt likelihood
social platform kind
feed vs video vs messaging
educational vs entertainment video
risky account/profile
risky content category
unknown/low confidence
```

AI cannot enforce.

## 7. Account Creation Gate

### 7.1 Gate Trigger

Trigger when strong or medium evidence appears:

```text
known signup URL
DOM form with create-account signals
birthdate/username/password flow
social login provider flow
new profile creation screen
account switch to unknown account
platform says "create your account"
child enters account registration flow
```

### 7.2 Gate Decision

Policy:

```text
If platform is social and flow is signup:
  require parent approval unless explicit allow rule exists.
```

Decision states:

```text
allowed_existing_account
parent_approval_required
blocked_by_rule
allowed_by_parent_exception
manual_review_required
unknown_flow_warn_only
```

### 7.3 Enforcement

In managed browser:

```text
pause navigation
show Ocentra approval screen
block form submit
redirect to parent approval page
close tab if strict
allow only after parent approval token
```

In app/native:

```text
block app launch until approval
show warning overlay where allowed
use OS app controls where proved
ask parent
record attempt
```

### 7.4 Parent Approval Payload

Contract family:

```text
SocialAccountCreationApprovalRequest
SocialAccountCreationApprovalDecision
SocialAccountCreationApprovalToken
```

Approval request fields:

```text
request id
child profile ref
device ref
platform
flow evidence refs
browser evidence refs
screenshot summary ref when allowed
AI analysis ref when used
proposed account hints
confidence
reason codes
requested at
expires at
```

Parent copy:

```text
Aarav may be creating a new Instagram account.
Evidence: Instagram signup page, email/password fields, username field.
Action needed: allow once, allow platform account creation, block, or ask child.
```

## 8. Fake Or Secondary Account Detection

Detect:

```text
new account signup
unknown login
account switcher shows multiple accounts
logout then signup
different username/account id from approved account
new email/phone credential flow
private/incognito profile use
new browser profile use
social app installed after being blocked on web
same platform used from unsupported browser/app
```

Contract family:

```text
SocialAccountIdentity
SocialAccountStatus
SocialAccountHint
```

Account statuses:

```text
approved
pending_parent_approval
blocked
unknown
suspected_secondary
revoked
```

Rules:

```text
Unknown account on approved platform -> ask parent.
Second account on same platform -> ask parent.
Account switch to unapproved account -> block/ask.
Fake age/birthdate entry evidence -> high-risk approval request.
```

Do not accuse the child. Use copy:

```text
This may be a new or different account.
Parent approval is required before continuing.
```

## 9. Feed, Reels, And Shorts Control

Social media risk is often feed style, not just domain.

Detect route kinds:

```text
feed
reels
shorts
stories
livestream
watch page
comments
search
profile
messages
upload/post
```

Policy examples:

```text
Allow YouTube video URLs, block YouTube Shorts.
Allow Instagram messages with known contacts, limit Reels.
Allow TikTok only on weekends.
Block infinite feeds during homework.
Allow Vimeo educational videos.
Ask parent for livestreams.
Block upload/posting for child account.
Block comments view if unsupported or risky.
```

Managed browser enforcement possibilities:

```text
URL path block
route block
CSS/DOM hiding only if owned adapter proof exists
redirect feed routes to safe page
close tab
time limit route/session
warn child
ask parent
```

Native app possibilities:

```text
app time limit
app block
screen/OCR detection with warning
Accessibility-based route detection on Android if explicitly enabled
iOS Screen Time category/app limits
```

Do not claim per-video or per-reel blocking inside native apps unless adapter
proof exists.

## 10. Messaging And Contact Risk

Messaging is privacy-sensitive. Default Ocentra should avoid raw message
capture.

Allowed evidence levels:

```text
Level 0: app/site messaging route used
Level 1: contact/account metadata if visible and allowed
Level 2: notification metadata if OS allows and parent enables
Level 3: local screen OCR summary if parent enables
Level 4: raw message content only after separate privacy/security review
```

Policy options:

```text
block messaging route
allow known contacts only if contact proof exists
ask parent for unknown contact
warn for adult/unknown contact risk
report messaging time
limit messaging schedule
```

Never secretly collect messages.

## 11. Social Platform Policy Targets

Social platforms:

```text
facebook
instagram
messenger
threads
tiktok
snapchat
youtube
youtube_shorts
vimeo
twitch
discord
reddit
x_twitter
pinterest
whatsapp
telegram
roblox
unknown_social
generic_web
```

Policy target kinds:

```text
platform
route_kind
account_creation
unknown_account
secondary_account
video
channel
feed
short_video_feed
messaging
upload_post
livestream
unknown_social_site
```

Actions:

```text
allow
observe
warn
time_limit
ask_parent
block
hold_until_parent_approval
block_signup
block_account_switch
require_managed_browser
manual_required
```

## 12. Social Signup Pattern Library

Maintain a versioned pattern library.

Contract family:

```text
SocialSignupPattern
SocialSignupPatternConfidence
SocialSignupPatternProofState
```

Pattern fields:

```text
pattern id
platform
URL patterns
route hints
title hints
form field hints
button text hints
DOM selector hints when supported
confidence
last verified at
proof required
```

Examples:

```text
facebook signup route
instagram signup route
TikTok signup route
Snapchat signup route
Discord register route
Reddit register route
X/Twitter signup flow
Twitch signup route
```

Because platforms change frequently, patterns must be versioned and tested.

## 13. Social AI Analysis

Input contract family:

```text
SocialAiAnalysisInput
SocialAiAnalysisTask
```

Input fields:

```text
request id
child profile ref
device ref
source evidence refs
platform
route kind
URL shape ref
page metadata refs
account flow evidence refs
screen summary refs
parent rule refs
memory refs
task
custody label
```

Tasks:

```text
signup_attempt_classification
fake_or_secondary_account_risk
feed_risk_classification
video_safety
messaging_risk_summary
platform_policy_support
```

Output contract family:

```text
SocialAiAnalysisResult
SocialAiClassification
SocialRiskSignals
```

Classifications:

```text
new_account_attempt
existing_account_login
secondary_account_suspected
feed_browsing
short_video_browsing
video_watch
messaging
upload_posting
livestream
educational_video
entertainment_video
risky_content
unknown
```

Risk signals:

```text
adult
groomingContact
unknownAdultContact
cyberbullying
selfHarm
violence
addictiveFeed
misinformation
fakeAccount
privacyExposure
```

The result must include recommended policy input, confidence, uncertainty reason
codes, parent summary, optional child-safe summary, model runtime ref, prompt
template version, analyzed timestamp, and degraded state.

## 14. Parent Policy Examples

Strict young child mode:

```text
No social accounts without parent approval.
Block account creation.
Block unknown social platforms.
Block short-video feeds.
Allow educational YouTube videos after analysis.
Ask parent for unknown videos.
Block messaging routes.
```

Teen balanced mode:

```text
Approved accounts only.
Ask parent for new/secondary accounts.
Limit TikTok/Reels/Shorts to 30 minutes.
Allow YouTube educational content.
Warn for unknown adult contact risk.
Allow messaging during set hours.
Report high-risk searches/content.
```

Observe-only mode:

```text
Do not block.
Detect social account creation attempts.
Report new accounts.
Classify feeds/videos.
Show weekly summary.
Alert only for high-risk signals.
```

## 15. Parent UI Requirements

Required screens/sections:

```text
Social Platforms Overview
Approved Social Accounts
Pending Account Approval Requests
New/Fake Account Attempts
Platform Rule Builder
Feed/Reels/Shorts Rules
Video Platform Rules
Messaging/Contact Risk Rules
Evidence Details
AI Analysis Details
Action/Audit Timeline
Manual Required / Unsupported States
```

Parent approval card:

```text
New Instagram account attempt

Child: Aarav
Device: Aarav's laptop
Platform: Instagram
Evidence:
- Instagram signup URL
- Create account form detected
- Username field detected
- Password field detected

Ocentra recommendation:
Ask parent before continuing

Actions:
[Allow this account]
[Allow Instagram accounts always]
[Block this attempt]
[Ask child for reason]
[View evidence]
```

## 16. Child UX Requirements

Child-facing copy:

```text
Your family rules require parent approval before creating a new social account.
```

```text
This looks like a new or different account. Ask your parent to continue.
```

```text
Short-video feeds are limited right now.
```

```text
This video is being checked. You can continue once it matches your family rules.
```

Avoid:

```text
fake account accusation
shame language
"you are being watched"
"AI caught you"
```

## 17. Enforcement Matrix

| Surface                        | Detect account creation | Gate account creation  | Detect video/feed  | Block video/feed         | Notes                                         |
| ------------------------------ | ----------------------- | ---------------------- | ------------------ | ------------------------ | --------------------------------------------- |
| Managed Chrome/Edge web        | high                    | high if adapter exists | high               | medium/high by route/URL | Best desktop path.                            |
| Unmanaged browser              | low/medium              | block browser/app only | low                | block browser/app only   | No exact URL claim.                           |
| Android web in Ocentra browser | high if owned shell     | high                   | high               | high                     | Best mobile path if built.                    |
| Android native app             | medium with permissions | app-level mostly       | medium             | app-level mostly         | Route-level needs Accessibility/screen proof. |
| iOS native app                 | limited                 | app/category limits    | limited            | app/category limits      | Use Screen Time/ManagedSettings.              |
| Platform connector             | platform-dependent      | platform-dependent     | platform-dependent | platform-dependent       | Parent authorized only.                       |
| Network DNS/VPN                | domain only             | domain/app only        | low                | domain/app only          | No page/account proof.                        |

## 18. Tests And Validation

Unit tests:

```text
platform domain mapping
signup URL pattern matching
route-kind classification
social account flow evidence schema
social account identity schema
social policy target schema
AI input/output schema
risk signal 0..1 validation
unknown/degraded state validation
```

Integration tests:

```text
managed browser detects Facebook signup URL
managed browser detects Instagram signup URL
managed browser detects TikTok signup URL
managed DOM detects signup form fields
metadata extraction classifies YouTube video vs Shorts
policy engine requires parent approval for account_creation
unknown account login triggers parent-review
secondary account signal triggers parent-review
short-video route triggers time-limit/block
unmanaged browser cannot create exact social evidence
```

Contract tests:

```text
SocialPlatformTarget
SocialAccountFlowEvidence
SocialAccountIdentity
SocialAiAnalysisInput
SocialAiAnalysisResult
SocialVideoPolicyRule
SocialPolicyDecision
SocialVideoAlert
ParentApprovalRequest
```

E2E tests:

```text
Child opens Instagram signup in managed browser -> parent approval required.
Child opens Facebook signup -> parent approval required.
Child opens approved YouTube educational video -> allowed after analysis.
Child opens YouTube Shorts during homework -> blocked/limited.
Child opens TikTok feed -> time-limit or block according to policy.
Child opens unknown social site signup -> ask parent.
Child opens same platform with second account -> ask parent.
Child opens social platform in unmanaged browser -> bypass evidence, no URL claim.
```

Playwright tests:

```text
parent sees pending account approval request
parent allows one account attempt
parent blocks account attempt
parent views evidence drawer
parent sees no exact URL for unmanaged browser
parent configures short-video rule
parent configures social signup rule
child-facing approval screen appears
child-facing block screen appears
malicious platform title is escaped
long username/platform fields truncate safely
```

Security tests:

```text
fake platform page mimicking Instagram does not become high confidence without domain proof
signup form on unknown domain maps to unknown_social_signup
HTML/script in username/title escaped
AI cannot enforce directly
memory without evidence refs cannot approve account
parent approval token cannot be replayed
expired approval request rejected
wrong child/device approval rejected
unmanaged browser cannot submit fake exact evidence
```

## 19. Manual Proof Requirements

Before claiming platform support, capture proof.

For each platform:

```text
browser URL evidence
route classification
account creation detection
parent approval gate
child-facing hold/block screen
parent approval/reject flow
journal evidence
SQLite read model
portal screenshot
policy decision audit
fallback/degraded state
```

Proof artifact paths:

```text
output/social-proof/facebook-signup-gate/
output/social-proof/instagram-signup-gate/
output/social-proof/tiktok-feed-limit/
output/social-proof/youtube-shorts-block/
output/social-proof/youtube-educational-allow/
output/social-proof/unmanaged-social-bypass/
```

## 20. Workpack Split

```text
01. Social/video gating plan folder and README
02. Platform and route contract schemas
03. Social URL pattern library
04. Signup/login/account-switch evidence contracts
05. Managed DOM/form-shape detector
06. Social account identity registry
07. Parent approval request/decision contracts
08. Feed/reels/shorts route classification
09. Video/social metadata extractor
10. Social AI analysis contracts
11. Social risk/benefit signal model
12. Parent policy compiler for social targets
13. Managed browser account creation gate
14. Managed browser feed/short/video route gate
15. Unmanaged social bypass detector
16. Android native-app capability matrix
17. iOS Screen Time/ManagedSettings capability matrix
18. Platform connector authorization boundary
19. Memory/cache for account/video/channel decisions
20. Parent social dashboard UX
21. Child approval/block UX
22. Audit and explanation read model
23. Tests, fixtures, Playwright, manual proof
24. Rollout and manual-required status labels
```

If these become implementation assignments, create focused workpack files or
worker messages before code changes. Do not mix all 24 into one PR.

## Implementation Checkpoint - 2026-06-03

- SOCIAL-01 now creates
  `docs/plans/browser-plan/social-platform-account-feed/README.md` as the
  browser-plan social workpack home. The README maps all SOCIAL-01 through
  SOCIAL-24 rows to first proof roots and restates the managed-browser,
  adjacent-feature, proof, and no-claim boundaries. It does not add schemas,
  parsers, runtime adapters, policy decisions, UI delivery, platform connector
  logic, native app support, or enforcement.
- SOCIAL-02 now adds schema-backed platform and route evidence contracts in
  `packages/activity-domain/src/browser-social-platform-route-schemas.ts`.
  `BrowserSocialRouteEvidence` links managed-browser social route evidence to
  URL-shape classification ids and target kinds, while unmanaged social bypass
  and native-app social states stay manual-required or bypass-only. The
  contracts reject account identity proof, message content, feed content
  semantics, AI decisions, policy decisions, enforcement, native app control,
  and platform connector claims. Package subpath exports are now present; no parser, runtime adapter, policy evaluator, UI,
  native app support, connector, or enforcement is claimed.
- SOCIAL-03 now adds a deterministic social URL pattern library in
  `packages/activity-domain/src/browser-social-url-patterns.ts`. The adapter
  maps exact managed URL-shape classifications into validated
  `BrowserSocialRouteEvidence` for known social domains and route patterns,
  including signup, login, account-switch, settings/privacy, messaging,
  upload/post, livestream, feed, profile, post, and video routes. It also covers
  domain-pattern matches for platforms not yet first-class in the URL-shape
  parser, such as Snapchat and Pinterest. Unmanaged browser rows and fake-domain
  rows are rejected, and the output keeps account identity, message/feed content,
  AI, policy, native app, connector, UI, and enforcement claims false. Package subpath exports are now present; no DOM/form
  detector, account identity proof, policy evaluator, runtime gate, UI, native
  app support, connector, or enforcement is claimed.
- SOCIAL-04 now adds schema-backed signup/login/account-switch evidence
  contracts in
  `packages/activity-domain/src/browser-social-account-flow-schemas.ts`. The
  contract represents route-only managed-browser account-flow evidence derived
  from validated account-signup, login, and account-switch social route
  evidence, plus manual-required rows for unsupported sources. It rejects account
  identity refs, parent approval request refs, credentials, form field values,
  form submission, account creation completion, login success, account-switch
  completion, parent approval decisions, AI decisions, policy decisions, native
  app control, connector access, and enforcement. Package subpath exports are now present; no DOM/form detector, identity
  registry, parent approval decision, policy evaluator, runtime gate, UI, native
  app support, connector, or enforcement is claimed.
- SOCIAL-05 now adds a sanitized managed form-shape detector contract in
  `packages/activity-domain/src/browser-social-form-shape-detector.ts`. The
  detector accepts route-only social account-flow evidence and control-kind hints
  only, then emits form-shape evidence for signup, login, or account-switch
  forms when required controls are present. It rejects captured field values,
  raw DOM capture, weak/insufficient control sets, credentials, form submission,
  account identity, parent approval decisions, AI decisions, policy decisions,
  native app control, connector access, and enforcement. Package subpath exports are now present; no runtime DOM adapter,
  field-value capture, account identity registry, parent approval flow, policy
  evaluator, runtime gate, UI, native app support, connector, or enforcement is
  claimed.
- SOCIAL-06 now adds a privacy-preserving social account identity registry
  contract in
  `packages/activity-domain/src/browser-social-account-identity-registry.ts`.
  The registry can record unverified route-context entries from account-flow
  evidence, parent-declared hash refs, and manual-required states without raw
  account data. It rejects raw handle, display-name, and platform-account-id
  capture, credentials, platform verification, child-declared identity, account
  creation, login success, connector authorization, AI decisions, policy
  decisions, native app control, and enforcement. Package subpath exports are now present; no runtime registry store, raw
  account identity capture, platform connector verification, parent UI, policy
  evaluator, runtime gate, native app support, connector, or enforcement is
  claimed.
- SOCIAL-07 now adds parent approval request/decision contracts in
  `packages/parent-domain/src/social-parent-approval.ts`. Requests and decisions
  use parent-domain family, child, device, actor, timestamp, and evidence refs,
  and cover social account signup, login, account-switch, and manual-required
  states. They remain contract-only and reject raw messages, raw account
  identity, credentials, notification delivery, UI rendering, child
  notification, policy execution, action execution, native app control,
  connector authorization, and enforcement. Package subpath exports are now present; no runtime approval store, parent/child
  UI, notification delivery, policy evaluator/executor, runtime gate, native app
  support, connector, or enforcement is claimed.
- SOCIAL-08 now adds route-only feed/reels/shorts classification contracts in
  `packages/activity-domain/src/browser-social-feed-route-classification.ts`.
  The classifier consumes validated managed social route evidence plus sanitized
  surface hints and distinguishes dynamic feeds, short-video feed surfaces, and
  exact single-short-video routes. It rejects feed content semantics,
  recommendation semantics, message content, AI decisions, policy decisions,
  native app control, connector access, and enforcement. The live proof now
  drives Playwright against real public Reddit, Twitch, TikTok, Instagram, and
  YouTube Shorts routes, persists screenshots plus redacted URL/title hashes,
  and parses the route-only captures through the classifier without storing page
  body, DOM, feed content, messages, credentials, or recommendation semantics.
  Package subpath exports are now present; no feed content parser,
  recommender analysis, policy evaluator, runtime gate, UI, native app support,
  connector, or enforcement is claimed.
- SOCIAL-09 now adds bounded video/social metadata extractor contracts in
  `packages/activity-domain/src/browser-social-video-metadata.ts`. The extractor
  consumes managed social video, post, or feed route evidence and metadata refs
  for title, description, author hash, thumbnail hash, duration, publish date,
  category, and restriction signals. Metadata can be available, partial, or
  manual-required, while page body capture, transcript text, message content,
  feed content semantics, content semantics, AI decisions, policy decisions,
  native app control, connector access, and enforcement are rejected. The live
  proof now captures real public YouTube Shorts, Vimeo, Reddit, and Instagram
  route surfaces, persists screenshots plus redacted URL/title/meta hashes, and
  parses bounded refs through the extractor without storing raw title text, meta
  values, page body, DOM, transcript text, or feed content. Package subpath exports are now present; no network
  fetcher, transcript parser, feed content parser, AI analysis, policy
  evaluator, runtime gate, UI, native app support, connector, or enforcement is
  claimed.
- SOCIAL-10 now adds social-specific AI analysis contracts in activity-domain:
  `browser-social-ai-analysis-values.ts`,
  `browser-social-ai-analysis-schemas.ts`, and
  `browser-social-ai-analysis-result-builder.ts`. The contracts define typed
  social analysis tasks, prompt-template boundaries, input evidence refs,
  candidate classifications, confidence, uncertainty, model runtime refs, and
  degraded states for managed-browser social routes. Inputs reject raw browser,
  page, feed, message, transcript, screenshot, native, and connector state;
  results reject final policy actions, enforcement, raw model text/content
  storage, native app control, connector claims, and inconsistent degraded
  states. The live-evidence proof consumes SOCIAL-09 public social/video
  metadata proof refs and emits degraded `model-unavailable` input/result rows
  without executing a model or claiming provider selection. Package subpath
  exports are now present; no AI model execution, runtime provider selection, SOCIAL-11
  risk/benefit signal model, policy evaluator, runtime gate, UI, native app
  support, connector, or enforcement is claimed.
- SOCIAL-11 now adds social risk/benefit signal model contracts in
  activity-domain: `browser-social-riskbenefit-values.ts` and
  `browser-social-riskbenefit-signals.ts`. Signal rows model candidate social
  risks and benefits with severity, confidence, evidence refs, and
  manual-required/unavailable states. Signal sets copy provenance from typed
  SOCIAL-10 AI analysis results while rejecting raw message/feed/page/model use,
  account identity verification claims, final policy decisions, connector/native
  claims, and enforcement. The live-evidence proof consumes SOCIAL-10 degraded
  AI result refs and emits unavailable risk/benefit signal sets without
  classifying content or claiming final policy/enforcement authority. Package subpath exports are now present; no policy compiler, runtime gate, UI, native app
  support, connector, or enforcement is claimed.
- SOCIAL-12 now adds parent-domain social policy compiler contracts in
  `social-policy-compiler-values.ts` and `social-policy-compiler.ts`. The
  compiler consumes parent-owned social evidence, signal-set, parent-rule,
  schedule, and time-budget refs and produces decision candidates for allow,
  warn, parent-review, block, manual-review, or unknown outcomes. Inputs reject raw signal payloads,
  raw model text, activity-domain object transfer, UI/runtime/enforcement,
  native app, and connector claims. Decision candidates remain non-final and
  non-enforcing while validating fallback and parent-approval reason
  requirements. Package subpath exports are now present; no runtime policy gate, UI, native app support, connector, or
  enforcement is claimed.
- `social-policy-schedule-time-budget-proof` now makes SOCIAL-12 schedule and
  time-budget context explicit with required refs and states for contract-only
  candidates. Manual-required or unavailable schedule/time-budget states remain
  non-final fallbacks; no runtime policy gate, applied schedule, applied budget,
  browser mutation, or enforcement is claimed.
- `social-policy-live-evidence-compiler-proof` now consumes SOCIAL-11
  live-evidence signal refs and emits non-final SOCIAL-12 parent-domain
  manual-review candidates. It proves the compiler boundary rejects final
  policy, runtime gate, UI, enforcement, native app, connector, raw signal
  payload, and raw model text claims while leaving runtime policy execution,
  applied schedules, applied budgets, browser mutation, and enforcement
  unclaimed.
- SOCIAL-13 now adds managed-browser account creation gate contracts in
  `packages/activity-domain/src/browser-social-account-creation-gate.ts`. Gate
  plans require matching route-only account-flow evidence and sanitized
  form-shape evidence, plus policy/approval refs as applicable. They can model
  allow-navigation, hold-for-parent-approval, block-submit, manual-review, and
  unknown-warn candidates while rejecting runtime browser pause/block claims,
  child/parent UI claims, final policy decisions, credentials, form submissions,
  account creation, native app control, connector claims, and enforcement.
  Package subpath exports are now present; no
  runtime browser blocking, UI, native app support, connector, or enforcement is
  claimed.
- SOCIAL-14 now adds managed-browser feed/short/video route gate contracts in
  `packages/activity-domain/src/browser-social-feed-video-route-gate-values.ts`,
  `packages/activity-domain/src/browser-social-feed-video-route-gate-guards.ts`,
  and `packages/activity-domain/src/browser-social-feed-video-route-gate.ts`.
  Gate plans combine typed feed route classification, bounded video metadata
  evidence, and policy/approval/time-limit refs to model allow, warn,
  parent-review, block, limit, manual-review, and unknown-warn route candidates.
  The contracts reject browser navigation block execution, redirects, CSS/DOM
  hiding, tab closing, applied time limits, child/parent UI, final policy
  decisions, feed/video content capture, recommendation modeling, native app
  control, connector claims, and enforcement. Live proof now runs
  `scripts/test/social-feed-video-live-route-gate-proof.mjs` against real public
  Reddit, Twitch, TikTok, Instagram, YouTube, and Vimeo surfaces, writes
  screenshots plus redacted proof JSON, validates five route-gate candidate
  plans through the built contracts, and records the YouTube short redirect as a
  non-planned live capture. Package subpath exports are now present; no runtime
  route gate, UI, native app support, connector, or enforcement is claimed.
- SOCIAL-15 now adds unmanaged social bypass detector contracts in
  `packages/activity-domain/src/browser-social-unmanaged-bypass-detector-values.ts`
  and `packages/activity-domain/src/browser-social-unmanaged-bypass-detector.ts`.
  The detector consumes redacted unmanaged/browser-like process evidence,
  confidence, suspected platform refs, and unmanaged fallback states to emit
  bypass-only evidence with managed-browser-required state. It rejects exact URL
  proof, managed-session boundaries, route evidence, social account proof,
  feed/video route proof, message content, account identity, native app control,
  connector claims, child/parent UI, process termination, managed browser
  relaunch, and enforcement. Package subpath exports are now present; no runtime blocking, UI, native app support,
  connector, or enforcement is claimed.
- SOCIAL-16 now adds Android native social app capability matrix contracts in
  `packages/parent-domain/src/social-android-native-app-capability-matrix-values.ts`
  and `packages/parent-domain/src/social-android-native-app-capability-matrix.ts`.
  The matrix covers package visibility, UsageStats foreground evidence,
  accessibility route hints, VPN/domain hints, device-owner app control, and
  managed-profile config. It keeps Android native social support app-level,
  permission-required, manual-required, unavailable, or not-implemented unless
  platform proof exists. It rejects native route proof, per-video/per-reel
  blocking, message content, account identity, accessibility content capture,
  device-owner enrollment, VPN content inspection, runtime adapter, connector,
  UI, and enforcement claims. The real host proof now records adb present, one
  booted Android emulator, known social package-id visibility queried only, and
  YouTube present on that emulator without persisting the raw installed package
  list. Package subpath exports are now present; no Android native app
  implementation, connector, UI, native route/content proof, or enforcement is
  claimed.
- SOCIAL-17 now adds iOS Screen Time/ManagedSettings social capability matrix
  contracts in
  `packages/parent-domain/src/social-ios-screen-time-capability-matrix-values.ts`
  and `packages/parent-domain/src/social-ios-screen-time-capability-matrix.ts`.
  The matrix covers FamilyControls authorization, application-token selection,
  web-domain-token selection, DeviceActivity monitor state, ManagedSettings
  application shields, and ManagedSettings web-domain shields. It keeps iOS
  native social support entitlement-required, token-selection-required, or
  manual-device-proof-required until Apple approval and device artifacts exist.
  It rejects entitlement approval, raw app identity, native route proof,
  per-video/per-reel blocking, message content, account identity, screen
  content capture, runtime adapter, connector, UI, and enforcement claims.
  Package subpath exports are now present; no
  iOS native app implementation, Apple entitlement, device proof, connector,
  UI, or enforcement is claimed.
- SOCIAL-18 now adds platform connector authorization boundary contracts in
  `packages/parent-domain/src/social-platform-connector-authorization-values.ts`
  and `packages/parent-domain/src/social-platform-connector-authorization.ts`.
  The boundary covers Google/YouTube supervision, Meta Family Center, TikTok
  Family Pairing, platform export/import, and parent-provided account refs as
  optional adjacent sources. Rows encode parent authorization, custody,
  expiry/revocation/manual-required state, scopes, and proof refs while keeping
  core gating independent. They reject token storage, OAuth client
  implementation, provider API calls, raw account data, message/feed content
  capture, account identity verification, policy decisions, AI runtime, UI,
  native app control, connector implementation, and enforcement. Package subpath exports are now present; no
  connector runtime, token store, provider API, UI, or enforcement is claimed.
  `social-platform-connector-authorization-proof` captures live public
  Google/YouTube supervision, Meta Family Center, and TikTok Family Pairing
  pages with Playwright screenshots and parses those proof refs through the
  five-row connector boundary while keeping connector implementation, token
  storage, OAuth, provider API calls, raw account/message/feed capture, UI
  delivery, native app control, policy execution, and enforcement unclaimed.
- SOCIAL-19 now adds parent-domain social decision memory-cache contracts in
  `packages/parent-domain/src/social-decision-memory-cache-values.ts` and
  `packages/parent-domain/src/social-decision-memory-cache.ts`. The initial
  activity-domain path was not used because codex-a currently owns
  `packages/activity-domain`. The snapshot covers account, video, and channel
  decision refs with cache keys, policy/child/rule keys, bounded TTL classes,
  source evidence refs, decision refs, invalidation reasons, and
  fresh/stale/miss/manual reuse rules. Fresh hits can be reused only when they
  have no invalidation reasons and cite decision refs; stale, miss, and
  manual-required rows cannot drive policy input. The contracts reject final
  policy decisions, runtime cache store claims, AI cache claims, raw
  account/video/message storage, connector data storage, UI, native app
  control, and enforcement. Package subpath exports are now present; no runtime cache, activity-domain export, UI,
  connector, native app control, policy execution, or enforcement is claimed.
- `social-decision-memory-live-evidence-proof` now consumes SOCIAL-12
  live-evidence policy candidate refs and emits a bounded ref-only SOCIAL-19
  memory snapshot with account miss, video fresh-hit, and channel stale-hit
  entries. It validates schema acceptance plus negative rejection for final
  policy, runtime cache store, AI cache, raw content, connector data, UI,
  native app, and enforcement claims while leaving runtime cache persistence,
  activity-domain exports, connector/native runtime, policy execution, and
  enforcement unclaimed.
- SOCIAL-20 now adds parent-domain parent social dashboard UX contracts in
  `packages/parent-domain/src/social-dashboard-ux-values.ts` and
  `packages/parent-domain/src/social-dashboard-ux.ts`, plus text-domain copy
  tokens in `packages/text-domain/src/social-dashboard-ux-text.ts`. The
  snapshot covers account approval queue, feed/video gates, native app
  capability, connector boundaries, decision memory, settings/custody, and
  manual-required gaps as section/action/status contracts. The real Browser
  route now requests the service-backed social dashboard read-model and captures
  desktop/mobile screenshots for the seven-row parent social snapshot. Package
  subpath exports are now present; no settings mutation, notification delivery,
  connector authorization, native app control, final policy execution, or
  enforcement is claimed.
- SOCIAL-21 now adds parent-domain child approval/block UX contracts in
  `packages/parent-domain/src/social-child-approval-block-ux-values.ts` and
  `packages/parent-domain/src/social-child-approval-block-ux.ts`, plus
  text-domain copy tokens in
  `packages/text-domain/src/social-child-approval-block-ux-text.ts`. The
  snapshot covers approval-request pending, blocked social route candidate,
  warning social route candidate, manual-review required, time-limit candidate,
  and native-app unavailable states as child-facing state/action contracts. The
  current proof maps those states into the shared child intervention renderer,
  serves them through the Rust child-agent endpoint, and captures screenshots.
  Package subpath exports are now present; no notification delivery, browser
  navigation block execution, applied time limits, final policy decisions,
  connector authorization, native app control, or enforcement is claimed.
- SOCIAL-22 now adds parent-domain social audit/explanation read-model
  contracts in
  `packages/parent-domain/src/social-audit-explanation-read-model-values.ts`
  and `packages/parent-domain/src/social-audit-explanation-read-model.ts`. The
  snapshot covers account approval, feed/video gate, native-app gap, connector
  boundary, decision memory, and manual-required gap rows with evidence links,
  policy refs, parent approval refs, memory refs, manual gap refs, and audit
  refs. The Rust service now answers the social audit/explanation read-model
  command with a schema-backed event consumed by the Browser route, and
  desktop/mobile screenshots prove the rendered parent explanation panel.
  Package subpath exports are now present; no logging-domain runtime store,
  notification delivery, raw account/video/message content, connector
  authorization, native app control, final policy execution, or enforcement is
  claimed.
- SOCIAL-23 now adds `scripts/test/social-platform-account-feed-proof-artifacts.mjs`,
  a proof artifact gate that verifies SOCIAL-01 through SOCIAL-22 checklist
  ownership, proof directory references, required proof files, social workpack
  README references, and feature/expectation coverage. It emits
  `test-results/social-platform-account-feed-proof-artifacts/proof.json` and
  `output/browser-plan-proof/social-23-tests-fixtures-playwright-manual-proof/01-social-proof-artifact-manifest.md`.
  The manifest records rendered parent dashboard, child intervention, and parent
  explanation proof coverage while the overall rollout stays partial.
  `social-source-custody-mutation-proof` now adds a Rust service WebSocket
  command/event proof for runtime custody mutation by applying a redacted-ref
  source custody settings snapshot. No runtime connector, native app control,
  final policy execution, enforcement, or product checklist claim is made.
- SOCIAL-24 now adds `scripts/test/social-platform-account-feed-rollout-gate.mjs`,
  a rollout/manual-required label gate that verifies SOCIAL-01 through SOCIAL-23
  checklist labels and required no-claim guard text. It emits
  `test-results/social-platform-account-feed-rollout-gate/proof.json` and
  `output/browser-plan-proof/social-24-rollout-manual-required-labels/01-rollout-manual-required-labels.md`.
  SOCIAL rollout state: partial/manual-required. Product checklist upgrade is
  not claimed. Service-backed dashboard and explanation read-model delivery plus
  child-agent-served intervention rendering are present, including the
  settings/custody dashboard row and runtime custody mutation proof, but
  connector/native runtime, final policy execution, enforcement, release
  readiness, and product completion remain unclaimed.
- `social-alert-report-intent-proof` now adds parent-domain social alert/report
  intent contracts and focused tests for high-risk signal, account approval,
  feed/video gate, weekly summary, manual-required, and unavailable states. It
  links dashboard panel refs, social explanation refs, evidence refs, policy
  refs, audit refs, optional parent report/action refs, and local-outbox refs
  without claiming raw account/video/message content, screenshots, provider
  delivery, report delivery, parent notification UI, final policy decisions, or
  enforcement.
- `social-report-writer-delivery-proof` now adds parent-domain social report
  writer delivery-readiness contracts and focused tests. It links social report
  intents into parent-owned report artifact and receipt rows while preserving
  explicit non-claims for external runtime report delivery, provider dispatch,
  provider receipt ingestion, raw social content, final policy execution, and
  enforcement.
- `social-applied-schedule-time-budget-proof` now adds parent-domain
  schedule/time-budget application-readiness contracts and focused tests. It
  links SOCIAL-12 compiler candidates into parent-owned schedule and budget
  evaluation rows plus a runtime handoff ref while preserving explicit
  non-claims for runtime-applied schedules, runtime time-budget application,
  browser gate execution, final policy execution, and enforcement.
- `social-parent-sensitivity-settings-proof` now adds parent-domain Parent sensitivity settings
  contracts and focused tests for high-risk alerts,
  feed/video review, account-flow review, connector data use, native-app gap
  review, and weekly summary sensitivity rows. Contract-only policy candidate
  input requires source/privacy refs, AI aggregate refs, dashboard refs,
  evidence refs, schedule refs, and time-budget refs; manual/unavailable rows
  stay out of policy input. The proof does not claim raw content custody,
  connector API calls, runtime settings UI, final policy decisions, or
  enforcement.
- `social-video-source-custody-settings-proof` now adds activity-domain social
  source custody settings contracts and focused tests for enabled redacted-ref
  use, parent-review connector refs, disabled/manual-required/unavailable
  source states, retention labels, and manual proof requirements. Contract-only
  policy candidate input requires source/privacy evidence refs and enabled
  custody state; parent-review, disabled, manual-required, and unavailable rows
  stay out of policy input. The proof does not claim raw message/video custody,
  screenshots, connector tokens/API calls, runtime settings UI, runtime custody
  mutation, final policy decisions, or enforcement.
- `social-source-custody-mutation-proof` now adds agent-protocol-domain and Rust
  protocol/service proof for service-backed source custody mutation. The command
  `agent.browser.social-source-custody.mutation.apply` returns
  `agent.browser.social-source-custody.mutation.applied` with a redacted-ref
  settings snapshot, `serviceMutationExecuted=true`, and
  `runtimeCustodyMutationApplied=true` while preserving no raw content custody,
  no connector API calls, no final policy decision, no enforcement, and no
  product completion claim.

## 21. Must-Not-Claim List

Do not claim:

```text
We know child created a fake account from one URL only.
We can read private messages without explicit permission/proof.
We can block individual reels inside native apps without adapter proof.
We can verify the child's real age from platform account data alone.
We can trust platform teen settings completely.
We can classify dynamic feeds permanently.
We can detect all social account creation across all browsers/apps.
We can use remote AI by default.
We can store raw messages/screenshots by default.
```

## 22. Minimum MVP

First strong MVP:

```text
Managed browser detects known social signup routes.
Managed browser detects generic signup/login/account-switch forms.
Parent policy says social account creation requires approval.
Child sees approval hold screen.
Parent sees approval request with evidence.
Parent can allow once, block, or approve account.
YouTube video vs Shorts route is detected.
Shorts/Reels/TikTok-style feeds can be blocked/limited by route in managed browser.
Educational YouTube video can be allowed after URL/metadata/AI analysis.
Unmanaged browser/social app remains bypass/app-level evidence only.
```

This is enough to stand out because Ocentra controls the account creation
attempt, not just after-the-fact screen time.

## 23. Done Signal

This feature is credible when:

```text
A child cannot create a new social account in the managed browser without parent approval.
Unknown or secondary social accounts trigger parent-review.
Social/video/feed routes are first-class policy targets.
YouTube educational video can be treated differently from YouTube Shorts/feed.
Platform limitations are visible.
Native app limitations are visible.
Unmanaged browser attempts are reported as bypass, not exact social proof.
AI classification cites evidence and cannot enforce directly.
Parent can see what happened and why.
Every action has evidence, policy, and audit refs.
```

Final rule:

```text
Do not trust the platform alone.
Do not trust AI alone.
Do not trust weak signals alone.
Gate social account creation with evidence, parent policy, and auditable approval.
```
