# Screen Evidence Analysis

## Parent Outcome

Parents can optionally use local screen summaries to understand visible activity
when browser/app/network evidence is insufficient, while keeping raw images out
of Ocentra custody by default.

## Ocentra Requirement

Screen evidence is opt-in, local-first, temporary, encrypted, summarized, and
deleted according to visible retention rules. Policy consumes summaries and
evidence refs, not retained screenshots or raw model text.

## Roadmap And Expectations

- Roadmap: V0.5.3 screen evidence, V0.7 local AI policy, V5 policy product.
- Expectations: [screen evidence](../expectations/screen-evidence.md),
  [AI](../expectations/ai.md), [data custody](../expectations/data-custody.md).
- Supporting docs:
  [screen settings inventory](../screen-control-settings-inventory.md).
- Modules: `packages/activity-domain`, `packages/parent-domain`,
  `crates/agent-core`, `crates/agent-service`.

## Competitor Pressure

See [Competitor Capability Map](../competitor-capability-map.md), especially
screenshots/live screen, video safety, and local-first privacy.

Some competitors use screenshots or screen visibility. Ocentra's default
position is more privacy-preserving: local summaries first, raw image retention
only with explicit parent settings.

## Current Ocentra State

- Local screen-analysis queue direction and contracts exist.
- `@ocentra-parent/activity-domain` now proves parent opt-in settings, cadence
  and trigger gates, temporary encrypted queue custody, deletion/expiry/delete
  failure states, raw-image non-retention, confidence/unknown handling, and
  policy eligibility with focused contract tests plus
  `scripts/test/screen-evidence-settings-retention-proof.mjs`.
- Local AI and policy can consume evidence summaries in dry-run paths.
- Windows local adapter proof now captures active-window, selected-window, and
  parent-opt-in primary-display scopes with encrypted temporary custody and raw
  delete-after-success proof.
- Android child-agent scaffold now has emulator MediaProjection proof with
  explicit OS consent, foreground service, captured frame digest, and raw temp
  deletion. Physical Android parity and silent background capture are not
  claimed.
- Product settings and quality proof are incomplete.
- Raw screen control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Opt-in UI, product screenshot capture scheduling, OCR/vision model quality,
runtime service/read-model wiring, physical Android/iOS/macOS/Linux proof,
enforcement handoff, checklist status movement, and parent explanation UX
remain.

## Checklist

- [ ] Parent opt-in setting.
- [ ] Capability/status contract.
- [ ] Encrypted temporary image queue.
- [ ] Local OCR/vision summary.
- [ ] Image deletion and retention state.
- [ ] Confidence and unknown handling.
- [ ] Policy decision references summary evidence.
- [ ] Portal explanation and audit.

## Next AI Instructions

Never route raw screen images to Ocentra cloud by default. Treat summaries,
confidence, deletion state, and custody labels as required product fields.
