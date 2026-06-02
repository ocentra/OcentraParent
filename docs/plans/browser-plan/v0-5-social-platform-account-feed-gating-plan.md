# V0.5 Social Platform Account Feed And Gating Plan

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
unknown account login triggers ask-parent
secondary account signal triggers ask-parent
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
Unknown or secondary social accounts trigger ask-parent.
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
