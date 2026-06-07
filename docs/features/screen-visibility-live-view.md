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
- The parent Settings route now renders those optional raw-retention and
  live-view readiness rows from the same domain proof using the real Rust agent
  and Vite portal path
  (`scripts/test/screen-optional-visibility-capability-status-portal-proof.mjs`,
  `output/screen-plan-proof/optional-visibility-capability-status-portal/proof-summary.json`,
  and
  `output/screen-plan-proof/optional-visibility-capability-status-portal/parent-optional-visibility-capability-status.png`).
  The UI proof shows disabled, manual-required, blocked, and capture-only
  permission evidence without enabling raw retention, live view, live
  transport, relay/cache, remote input, or privacy/legal approval.
- `ScreenLiveViewParentUiPersistenceProofSchema` and
  `scripts/test/screen-live-view-parent-ui-persistence-proof.mjs` now prove the
  parent Settings command/readiness evidence can be carried into live-view
  service-session and Rust runtime decisions as persisted parent opt-in state
  while still keeping product live view false.
- `scripts/test/screen-live-view-worker-startup-proof.mjs` now proves the Rust
  service worker startup gate exists and stays stopped unless runtime readiness,
  a real live-view prompt artifact, relay/cache execution when needed,
  physical-device parity, and privacy/legal approval are all proved.
- Raw screen control settings are preserved as design inputs for both summary
  and live-view decisions.

## Current Gap

Need runtime product implementation for optional screenshots or live view. The
contract preflight, parent opt-in/device status, and fail-closed
platform-permission gate exist, parent Settings can render readiness rows,
live-view parent UI persistence is carried into service/runtime proofs, and the
Rust worker startup gate stays fail-closed. Optional raw-retention persistence/
runtime, actual live-view permission prompts, live transport, relay/cache
execution, platform screenshots, physical-device parity, privacy/legal approval,
and a started production live-view worker remain before any product-complete
claim.

## Checklist

- [x] Product decision: summaries only, screenshots, live view, or tiered modes.
- [x] Parent opt-in and child/device capability status.
- [x] Source label: local summary, screenshot, live, relay, cache,
      unavailable.
- [x] Retention and deletion controls.
- [x] Audit for capture/view/export.
- [x] Remote route/custody model if away-from-home.
- [x] Parent Settings route renders optional raw-retention/live-view readiness
      rows without enabling those modes.
- [x] Parent UI persistence proof carries live-view opt-in state into
      service-session/runtime decisions without enabling product live view.
- [x] Rust service worker startup gate refuses to start without real platform,
      relay/cache when needed, physical parity, and privacy/legal proof.
- [~] Platform permission proof gate exists; real live-view prompt/platform
  screenshots remain.
- [ ] Runtime enablement, optional raw-retention persistence/runtime, live
      transport, relay/cache, physical parity, and platform prompt proof before
      product claim.
- [ ] Privacy/legal review before public claim.

## Next AI Instructions

Do not quietly add screenshot retention or remote live view. If working here,
make the mode explicit and keep it separate from local screen summaries.
