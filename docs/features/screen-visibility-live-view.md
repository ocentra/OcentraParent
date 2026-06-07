# Screen Visibility And Live View

## Parent Outcome

Parents can understand what is visible on the child device when needed, while
knowing whether the product is showing local summaries, saved screenshots, live
view, parent cache, remote relay, or unavailable state.

## Ocentra Requirement

Ocentra's default posture is local screen summaries, not cloud screenshots. If
the product adds screenshots or live view, it must be explicit, opt-in,
audited, retention-limited, and platform-proved.

## Roadmap And Expectations

- Roadmap: V0.5.3 screen evidence, V2 remote access, V5 parent product, V6
  mobile agents.
- Expectations: [screen evidence](../expectations/screen-evidence.md),
  [remote access](../expectations/roadmap-v2-parent-owned-remote-access-cloud-relay.md),
  [data custody](../expectations/data-custody.md),
  [platforms](../expectations/platforms.md).
- Supporting docs:
  [screen settings inventory](../screen-control-settings-inventory.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-service`, platform folders.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
screenshots/live screen, video safety, reports/digests, and remote parent
access.

Some competitors provide screenshots or live viewing. Ocentra must either
compete directly or clearly position local summaries as the privacy-preserving
alternative.

## Current Ocentra State

- Local screen evidence summaries are tracked.
- Temporary local image queue/deletion expectations exist.
- Remote screenshot/live view is not product-complete and has not been accepted
  as a default feature.
- The local screen-summary boundary now has a contract/proof
  (`ScreenEvidenceRemoteBoundarySettingSchema` plus
  `scripts/test/screen-evidence-settings-retention-proof.mjs`) that keeps
  raw screenshot retention, live view, and raw remote upload disabled. Parent
  approved redacted summary export is the only accepted remote mode, and it must
  use parent-owned-export custody.
- The Settings route writable-intent proof now shows that enabling local
  screen-summary drafts still keeps raw screenshot retention, live view, and raw
  remote upload disabled in the rendered parent UI.
- Optional raw screenshot retention and live view now have separate preflight
  contracts and proof in `@ocentra-parent/activity-domain`
  (`ScreenRawScreenshotRetentionOptInSettingSchema`,
  `ScreenLiveViewOptInSettingSchema`, and
  `scripts/test/screen-optional-retention-live-preflight-proof.mjs`). Raw
  retention modes require explicit parent approval, audit ref, custody, TTL,
  delete proof, and no raw remote upload. Live-view modes require explicit
  parent approval, viewer audit, platform proof ref, LAN or relay transport
  label, no frame retention, no session recording, and no remote input.
- The live-view platform-permission gate now has a focused
  `ScreenLiveViewPlatformPermissionGateSchema` proof
  (`scripts/test/screen-live-view-platform-permission-proof.mjs` and
  `output/screen-plan-proof/live-view-platform-permission/proof-summary.json`).
  It consumes the real Android MediaProjection capture-consent proof as
  capture-only evidence and proves that capture permission cannot make live view
  product-ready without live-view permission-prompt evidence, viewer audit, live
  transport proof, no frame retention, and no remote input.
- Optional raw screenshot retention and live view now have an explicit
  child/device capability status contract
  (`ScreenOptionalVisibilityCapabilityStatusSchema` plus
  `scripts/test/screen-optional-visibility-capability-status-proof.mjs` and
  `output/screen-plan-proof/optional-visibility-capability-status/proof-summary.json`).
  It renders disabled, manual-required, and blocked readiness states for parent
  opt-in modes, rejects "ready" raw retention without runtime and deletion
  proof, and rejects live-view readiness when the only platform evidence is
  capture consent.
- Raw screen control settings are preserved as design inputs for both summary
  and live-view decisions.

## Current Gap

Need runtime product implementation for optional screenshots or live view. The
contract preflight, parent opt-in/device status, and fail-closed
platform-permission gate exist, but service runtime enablement, parent
retention/live-view UI, actual live-view permission prompts, live transport,
relay/cache execution, platform screenshots, and privacy/legal approval remain
before any product-complete claim.

## Checklist

- [x] Product decision: summaries only, screenshots, live view, or tiered modes.
- [x] Parent opt-in and child/device capability status.
- [x] Source label: local summary, screenshot, live, relay, cache,
      unavailable.
- [x] Retention and deletion controls.
- [x] Audit for capture/view/export.
- [x] Remote route/custody model if away-from-home.
- [~] Platform permission proof gate exists; real live-view prompt/platform
  screenshots remain.
- [ ] Runtime enablement, live transport, and parent UI persistence before
      product claim.
- [ ] Privacy/legal review before public claim.

## Next AI Instructions

Do not quietly add screenshot retention or remote live view. If working here,
make the mode explicit and keep it separate from local screen summaries.
