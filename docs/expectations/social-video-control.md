<!-- agent-capsule -->

> Agent Capsule
> Doc: Social And Video Control Expectations
> Kind: expectation/acceptance documentation; read only when selected by feature doc, plan route, or assigned workpack.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Social And Video Control Expectations

Social and video control is a first-class product area. It must not be hidden
inside generic app blocking or vague AI claims.

## Parent Outcome

- Parent can define rules for social apps, messaging apps, video platforms,
  channels, video URLs, categories, schedules, and time budgets.
- Parent can choose allow, warn, time-limit, parent-review, or block actions where
  the platform and adapter support them.
- Parent can see why a social/video item was flagged, including evidence refs,
  confidence, source, model/runtime status, and parent rule references.
- Parent can tune alert sensitivity without giving Ocentra default custody of
  messages, screenshots, or video content.

## Child-Device Outcome

- Child-device agent captures only approved evidence types for the configured
  platform and setting.
- Local AI or deterministic policy uses typed evidence summaries, not raw
  unvalidated model text.
- Policy decisions degrade to unknown, warn, or parent-review when source evidence
  or model confidence is insufficient.
- Enforcement happens only through a typed policy decision and supported
  platform adapter.

## Evidence Sources

Possible evidence sources must be documented separately:

- browser URL/tab metadata;
- managed browser page/video metadata where available;
- app/game/social session evidence;
- local screen OCR/vision summary;
- local notification or message metadata where platform policy allows it;
- parent-provided URLs or channels;
- platform account connectors only when explicitly authorized by the parent.

Raw message capture, photos, videos, screenshots, or account data require a
separate privacy/security review and a visible parent setting before any product
claim.

## Contract Boundary

Expected contract families:

- `SocialPlatformTarget`
- `VideoTarget`
- `ChannelTarget`
- `SocialEvidenceSummary`
- `VideoEvidenceSummary`
- `SocialVideoSourcePrivacySummary`
- `SocialRiskSignal`
- `VideoRiskSignal`
- `SocialVideoPolicyRule`
- `SocialVideoAlert`
- `SocialVideoDecision`

Browser-plan social workpacks may create focused README/proof scaffolding before
contracts exist, but that scaffolding is not product proof. Product claims still
require schema-backed targets, evidence summaries, confidence/degraded states,
policy decisions, UI/manual proof where visible, and platform-specific
capability evidence.

Browser-managed social route evidence must cite managed URL-shape proof before
it can classify platform access, account signup, login, account switch, profile,
feed, short-video feed, video, post, livestream, messaging route, upload/post,
or settings/privacy routes. Unmanaged social browser use remains bypass-only,
and native app social state remains manual-required until platform-specific app
proof exists. Route contracts must not claim account identity, message content,
feed content semantics, AI decisions, policy decisions, connector access, native
app control, or enforcement.

Social/video source privacy evidence-summary contracts may cite existing
managed-browser social route refs, bounded social/video metadata refs,
parent-provided URL or channel refs, optional connector authorization refs,
screen-summary refs, and native/platform manual-required states. They may
declare custody labels, source types, confidence, degraded/manual-required
state, and permitted downstream uses for AI candidate input, policy candidate
input, parent explanation, manual review, or audit summary. They must not store
raw content, raw messages, raw video, screenshots, connector tokens, connector
API payloads, native app control state, final policy decisions, or enforcement
state. Current proof: `social-video-source-privacy-proof`.

Social/video AI signal aggregate contracts may link source/privacy summary refs
to existing bounded social AI analysis result refs, candidate risk/benefit
signal-set refs, and managed-browser route gate/action candidate refs. They may
carry aggregate confidence, degraded/manual-required state, custody labels,
recommended policy input candidates, and parent-review/action candidate refs.
They must not collect raw browser/page/feed/message/video content, screenshots,
connector tokens, connector API payloads, native app control state, final policy
decisions, alert delivery state, rendered UI state, or enforcement state.
Current proof: `social-video-ai-signal-aggregate-proof`.

Social URL pattern libraries may map exact managed URL-shape classifications and
normalized social domains to route evidence, but they must reject unmanaged
browser rows and fake-domain rows. Domain and path patterns can prove route
shape only; they cannot prove child account identity, message or feed content,
parent policy decisions, UI delivery, native app control, platform connector
authorization, or enforcement.

Signup/login/account-switch evidence can be route-only or manual-required until
later DOM/form, account identity, parent approval, and policy/action rows prove
more. These contracts must not capture credentials, form field values, raw
messages, screenshots, account data, parent approval decisions, policy
decisions, native app control, connector access, UI delivery, or enforcement.

Managed form-shape detectors may use sanitized control-kind hints such as email
input, password input, submit button, or account-switch link. They must not
store raw DOM, field values, credentials, screenshots, form submissions, account
identity, parent approval decisions, policy decisions, platform connector data,
native app control, UI delivery, or enforcement state.

Social account identity registries may store unverified route-context refs,
manual-required state, or parent-declared hash refs. They must not store raw
handles, display names, platform account ids, credentials, child-declared
identity, platform verification, connector authorization, account creation
claims, login success claims, policy decisions, native app control, UI delivery,
or enforcement state.

Parent approval request/decision contracts may reference family, child, device,
actor, request, decision, and evidence ids. They remain contract-only until
runtime approval storage, parent/child UI, notification delivery, policy
execution, action execution, and enforcement proof exist. They must not store raw
messages, raw account data, credentials, connector authorization, native app
control, UI delivery, child notification, policy action execution, or
enforcement state by default.

Feed, reels, and shorts route classification may use managed social route
evidence plus sanitized surface hints to classify dynamic feed, short-video feed
surface, or exact single-short-video route shape. It must not infer feed content,
recommendations, messages, social graph state, AI decisions, policy decisions,
connector data, native app control, UI delivery, or enforcement state.

Social/video metadata extractors may store bounded metadata refs such as title,
description, author hash, thumbnail hash, duration, publish date, category, and
restriction signal refs. They must not store page body, transcript text,
messages, feed content, recommendations, social graph data, AI decisions, policy
decisions, connector data, native app control, UI delivery, or enforcement
state.

Managed-browser feed/short/video route gate-plan contracts may combine typed
feed route classification, bounded video metadata evidence, policy candidates,
parent approval refs, and time-limit refs to model route action candidates. They
must not claim browser navigation block execution, redirects, CSS/DOM hiding,
tab closing, applied time limits, UI delivery, final policy decisions, feed or
video content capture, recommendation modeling, native app control, connector
authority, or enforcement. Current proof:
`social-feed-video-live-route-gate-proof` captures real public social/video
surfaces with Playwright, persists screenshots plus redacted hashes/statuses,
and validates candidate route-gate plans without raw page-body, DOM, title,
credential, form-submit, feed-content, video-content, UI, final-policy, or
enforcement claims.

Unmanaged social bypass detector contracts may use redacted unmanaged or
browser-like process evidence to report bypass-only, managed-browser-required
social evidence. They must not claim exact URLs, managed social route evidence,
social account proof, feed/video route proof, messages, account identity, native
app control, connector authority, UI delivery, process termination, managed
browser relaunch, or enforcement.

Android native social app capability matrix contracts may represent package
visibility, UsageStats foreground evidence, accessibility route hints,
VPN/domain hints, device-owner app control, and managed-profile config as
app-level, permission-required, manual-required, unavailable, or not-implemented
states. They must not claim native social route proof, per-video or per-reel
blocking, message content, account identity, accessibility content capture,
device-owner enrollment, VPN content inspection, runtime adapters, connector
authority, UI delivery, or enforcement until platform proof exists.
Host/device proof may query only the known public social package ids needed for
the matrix and must not persist raw installed package lists, UI trees, logcat,
content, account identity, native route state, device-owner enrollment, runtime
adapter behavior, connector authority, UI delivery, or enforcement.

iOS Screen Time/ManagedSettings social capability matrix contracts may
represent FamilyControls authorization, application-token selection,
web-domain-token selection, DeviceActivity monitor state, ManagedSettings
application shields, and ManagedSettings web-domain shields as
entitlement-required, token-selection-required, manual-device-proof-required,
unavailable, or not-implemented states. They must not claim Apple entitlement
approval, raw app identity, native social route proof, per-video or per-reel
blocking, message content, account identity, screen content capture,
DeviceActivity or ManagedSettings runtime behavior, connector authority, UI
delivery, or enforcement until Apple approval and physical device proof exist.

Platform connector authorization boundary contracts may represent connector
provider options, parent authorization state, custody state, scopes,
expiry/revocation/manual-required state, and proof refs for Google/YouTube
supervision, Meta Family Center, TikTok Family Pairing, platform export/import,
and parent-provided account refs. They must not store tokens, implement OAuth
clients, call provider APIs, capture raw account/message/feed data, verify
account identity, become a core gating dependency, claim policy decisions, run
AI, render UI, control native apps, or enforce actions until separate connector,
custody, privacy, UI, and runtime proof exists.

Social decision memory-cache contracts may represent account, video, and channel
decision refs with policy, child, parent-rule, and subject cache keys, bounded
TTL classes, source evidence refs, decision refs, and invalidation reasons.
Fresh hits may feed policy input only when decision refs are present and no
invalidation reasons exist. Stale, miss, and manual-required rows must not drive
policy input. These contracts must not store raw account data, raw video content,
raw message content, connector data, raw model payloads, or final policy
decisions, and they must not claim runtime cache stores, AI caches, UI delivery,
native app control, or enforcement.

Parent social dashboard UX contracts may represent dashboard sections, sort
order, statuses, actions, severities, source evidence refs, and manual-required
gaps for account approval queues, feed/video gates, native app capability,
connector boundaries, and decision memory. These contracts must not claim
notification delivery, policy decisions, connector authorization, native app
control, or enforcement. The current rendered parent portal proof covers the
service-backed Browser-route social dashboard snapshot only; it does not prove
connector/native runtime, policy execution, notifications, or enforcement.

Parent-owned social notification/report delivery readiness may record
local delivery result refs only for report-ready rows that already have
parent-owned report artifact and receipt refs. Manual-required and unavailable rows must keep
local delivery result refs empty. This local result proof does not prove
external provider delivery, parent notification UI delivery, cloud routing,
final policy execution, connector/native runtime, browser mutation, child
intervention execution, or enforcement.

Child approval/block UX contracts may represent child-facing states and actions
for approval pending, blocked route candidates, warning candidates, manual
review, time-limit candidates, and native-app unavailable states. These
contracts must not claim rendered child UI, notification delivery, browser
navigation block execution, block-page rendering, applied time limits, final
policy decisions, connector authorization, native app control, or enforcement
from contracts alone. Current proof covers child-agent-served intervention
pages for the mapped social states only; browser navigation block execution,
notification delivery, applied limits, final policy execution, connector/native
runtime, and enforcement remain unclaimed until separately implemented and
tested.

Social audit/explanation read-model contracts may represent parent-readable and
audit-log rows for account approval, feed/video gates, native-app gaps,
connector boundaries, decision memory, and manual-required gaps. They may link
evidence refs, policy refs, parent approval refs, decision-memory refs, manual
gap refs, and audit refs. They must not claim runtime audit stores, rendered
explanation UI, notification delivery, raw account/video/message content,
connector authorization, native app control, final policy decisions, or
enforcement from contracts alone. Current proof covers the service-backed
Browser-route explanation read-model and rendered explanation panel only; a
runtime audit store, notification delivery, connector/native runtime, final
policy execution, and enforcement remain unclaimed until separately implemented
and tested.

Social alert/report intent read-model contracts may represent parent-visible
local-outbox alert intent rows and manual-required report rows from bounded
evidence, policy, dashboard, explanation, audit, and parent-action refs. The
service-backed Browser-route proof may claim only that the real portal rendered
the Rust service read model for the captured alert/report rows. It must not
claim provider dispatch, provider receipt, parent notification UI delivery,
report delivery, final policy execution, or enforcement from this proof.

Social alert/report local outbox bridge contracts may serialize only
local-outbox-eligible social alert/report intents into the shared
parent-owned `NotificationLocalOutboxRecord` JSONL schema and reread those
records through the same parser. Manual-required and unavailable rows may appear
in the bridge read model but must not produce queued JSONL records. This bridge
must not claim provider dispatch, provider receipt, scheduler runtime, parent
notification UI delivery, report delivery execution, final policy execution,
connector/native runtime, or enforcement.

Social alert/report parent-surface intent contracts may project provider-status
and preference/quiet-hours status handoff rows into parent-visible
manual-action-required and unavailable-visible surface intent rows with
notification status refs, preference status refs, quiet-hours state, audit refs,
and manual proof requirements. They may prepare data for a future authenticated
parent notification surface, but must not claim rendered parent notification,
preference, frequency-control, or notification-history UI, provider dispatch,
provider receipt, child delivery, quiet-hours timer execution, report delivery
execution, final policy execution, connector/native runtime, or enforcement.

Service-backed social alert/report parent-surface read models may expose those
manual-action-required and unavailable-visible rows through the Rust WebSocket
command/event path and the existing Browser route only as status projection.
The service path must use the shared local eventing request/subscriber pattern
and must derive the parent-surface projection from provider-status and
preference-status handoff subscribers rather than a narrower duplicate row
source. It must preserve no-claim fields for parent notification UI delivery,
preference UI delivery, notification history UI, provider delivery, provider
receipt ingestion, provider credentials, cloud routing, child delivery,
quiet-hours timer runtime, retry-worker runtime, production durable outbox
storage, adapter dispatch, report delivery execution, final policy execution,
connector/native runtime, browser mutation, unmanaged exact URL support, and
enforcement.

Social proof artifact gates may verify checklist ownership, proof folders,
required source/security/validation/UI-marker files, README references, and
feature/expectation coverage. They may cite rendered Playwright/screenshot
proof only for the exact UI route and state actually captured. They must not
claim runtime connector behavior, native app control, final policy execution,
enforcement, or product completion from contract-only rows or unavailable-state
shells.

Social alert/report scheduler bridge rows may move parent-owned local outbox
records into the existing notification local outbox scheduler JSONL schema only
when the source row is already linked. Manual-required and unavailable rows must
remain visible but unscheduled. The bridge may prove quiet-hours/preference
handoff readiness and audit refs only; it must not claim provider delivery,
receipt ingestion, quiet-hours timer execution, retry worker execution,
parent/child notification UI delivery, report delivery execution, final policy
execution, connector/native runtime, enforcement, or product completion.

Social alert/report preference preflight rows may consume social alert/report
scheduler bridge rows and require parent notification preference,
frequency-control, and quiet-hours policy proof before delivery can be claimed.
Manual-required and unavailable rows must stay blocked. This preflight must not
claim parent notification preference UI, notification history UI, quiet-hours
timer execution, provider delivery, child delivery, report delivery execution,
final policy execution, connector/native runtime, enforcement, or product
completion.

Social alert/report preference-status handoff rows may project those preference
preflight rows into the existing V3 notification rule/provider/retry preference
and quiet-hours status entries. Scheduled and manual-required rows must remain
manual-required until parent preference and quiet-hours proof exists;
unavailable rows must remain disabled/not-sent. The handoff must not claim
parent notification preference UI, notification history UI, parent notification
UI, quiet-hours timer execution, provider delivery, child delivery, report
delivery execution, final policy execution, connector/native runtime,
enforcement, or product completion.

Social alert/report audit-history bridge rows may map social local outbox rows
into the existing logging-domain notification audit-history handoff. Linked
rows may become queued audit-history entries; manual-required and unavailable
rows must become blocked/manual entries. This bridge must preserve redaction-safe
payload fields and no Ocentra-hosted child data custody. It must not claim
provider delivery, receipt ingestion, credentials, parent notification history
UI, child delivery, retry or quiet-hours runtime execution, report delivery
execution, final policy execution, connector/native runtime, enforcement, or
product completion.

Social alert/report provider dispatch execution rows may prepare a local
redaction-safe dispatch packet only when a parsed provider receipt-boundary row
is `provider-dispatch-required` and a matching parsed
`NotificationLocalOutboxRecord` exists. Manual-required and provider-unavailable
rows must remain packetless and visible. This local packet boundary must not
claim external provider delivery, delivered notification receipts, provider
webhook runtime, provider credentials, cloud routing, parent notification UI
delivery, child delivery, report delivery execution, final policy execution,
connector/native runtime, enforcement, or product completion.

Rollout/manual-required gates may label rows as partial/manual-required only.
They must preserve product completion as unclaimed until notification delivery,
connector/native runtime, final policy execution, enforcement, release
readiness, and product checklist upgrade proof exists.

Managed-browser social policy execution may be claimed only for an
Ocentra-managed browser session when a non-final social policy decision
candidate is chained to real child-agent intervention endpoint evidence, a
managed target ref, live-surface capture before mutation, browser mutation,
child intervention execution, and screenshot refs. It must preserve no-claim
boundaries for unmanaged browsers, broad OS enforcement, external provider
delivery, connector/native runtime, Apple platform support, raw URL custody,
raw page custody, and product completion. Current proof:
`social-managed-browser-policy-execution-proof`.

Social AI analysis contracts may consume typed social route, metadata, feed,
account-flow, account-identity, screen-summary, parent-rule, and memory refs to
produce candidate classifications, confidence, uncertainty, model runtime refs,
and degraded states. They must not collect raw browser state, page body,
transcripts, messages, feed content, screenshots, connector state, or native app
state, and they must not claim final policy actions, enforcement, UI delivery,
raw model text/content storage, connector authority, native app control, or
runtime AI execution.

Social risk/benefit signal models may turn typed social AI analysis outputs into
candidate signal rows with canonical risk or benefit kind, severity, state,
confidence, and evidence refs. They must not use raw messages, feed content,
page body, raw model text, connector data, or native app state, and they must not
claim account identity verification, final policy decisions, UI delivery,
native app control, connector authority, runtime gates, or enforcement.

Social parent policy compiler contracts may consume parent-owned evidence refs,
signal-set refs, rule refs, and schedule refs to produce decision candidates for
social targets. They must not import activity-domain objects, store raw signal
payloads or raw model text, render UI, execute runtime gates, control native
apps, authorize connectors, claim final policy decisions, or enforce actions.

Managed-browser social account gate-plan contracts may combine route-only
account-flow evidence, sanitized form-shape evidence, policy candidates, and
parent approval refs to model account navigation or submit candidates. They must
not claim browser runtime pause/block execution, UI delivery, final policy
decisions, credential capture, form submission, account creation, native app
control, connector authority, or enforcement.

## Acceptance

- Social/video targets are first-class policy targets.
- Rules support schedules and time budgets.
- Parent-facing explanations cite evidence and confidence.
- Unsupported platforms show unavailable/manual-required states.
- Video analysis is not claimed complete until the product proves actual input,
  model/runtime path, confidence handling, policy action, and audit output.

## Validation Gates

- TypeScript schema tests for targets, evidence summaries, risk signals, rules,
  decisions, alerts, and degraded states.
- `social-video-source-privacy-proof` for source/privacy evidence-summary refs,
  custody flags, confidence/degraded/manual-required states, downstream-use
  limits, package export visibility, and no-raw-content guarantees.
- Integration tests with real stored evidence summaries.
- Portal tests for rule authoring and explanation state when UI exists.
- Platform/manual proof for any source that depends on app, account, browser,
  notification, accessibility, or screen permissions.

## Non-Goals

- Do not secretly collect messages or media.
- Do not claim video semantic analysis from URL metadata alone.
- Do not enforce social/video rules from raw AI text.
- Do not hide platform limitations.

## Done Signal

A parent can configure social/video rules, see evidence-backed explanations, and
receive warnings/limits/parent-review/block behavior only where the configured
source and platform adapter are proved.
