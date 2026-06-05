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
- Raw screen control settings are preserved as design inputs for both summary
  and live-view decisions.

## Current Gap

Need an explicit product decision: local summaries only, optional screenshots,
or live view. If screenshots/live view are built, Ocentra needs capture
permission, transport, custody, retention, audit, parent settings, and platform
proof.

## Checklist

- [x] Product decision: summaries only, screenshots, live view, or tiered modes.
- [ ] Parent opt-in and child/device capability status.
- [ ] Source label: local summary, screenshot, live, relay, cache,
      unavailable.
- [ ] Retention and deletion controls.
- [ ] Audit for capture/view/export.
- [ ] Remote route/custody model if away-from-home.
- [ ] Platform permission proof.
- [ ] Privacy/legal review before public claim.

## Next AI Instructions

Do not quietly add screenshot retention or remote live view. If working here,
make the mode explicit and keep it separate from local screen summaries.
