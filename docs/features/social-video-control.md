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
- First-class social/video product contracts and runtime proof are incomplete.

## Current Gap

Ocentra does not yet have complete social/message/video sources, privacy
settings, alert contracts, confidence handling, platform proof, or parent UI.

## Checklist

- [ ] Social platform and video target contracts.
- [ ] Source permissions and custody settings.
- [ ] URL/video/channel/app evidence summaries.
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
