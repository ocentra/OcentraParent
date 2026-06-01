# Social And Video Control Expectations

Social and video control is a first-class product area. It must not be hidden
inside generic app blocking or vague AI claims.

## Parent Outcome

- Parent can define rules for social apps, messaging apps, video platforms,
  channels, video URLs, categories, schedules, and time budgets.
- Parent can choose allow, warn, time-limit, ask-parent, or block actions where
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
- Policy decisions degrade to unknown, warn, or ask-parent when source evidence
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
- `SocialRiskSignal`
- `VideoRiskSignal`
- `SocialVideoPolicyRule`
- `SocialVideoAlert`
- `SocialVideoDecision`

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
receive warnings/limits/ask-parent/block behavior only where the configured
source and platform adapter are proved.
