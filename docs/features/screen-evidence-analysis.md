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
- Local AI and policy can consume evidence summaries in dry-run paths.
- Product settings and quality proof are incomplete.
- Raw screen control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Opt-in UI, capture cadence, OCR/vision model quality, deletion proof,
confidence thresholds, and parent explanation UX remain.

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
