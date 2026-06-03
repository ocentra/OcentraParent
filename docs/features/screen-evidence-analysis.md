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
- The desktop local adapter path now uses the shared `xcap` capture API on
  Windows/macOS and an X11 command backend on Linux. Windows proof captures
  active-window, selected-window, and parent-opt-in primary-display scopes with
  encrypted temporary custody and raw delete-after-success proof. Linux WSLg
  proof captures a real X11 selected window with encrypted custody and raw
  deletion. macOS live capture proof still requires platform execution evidence
  before parity is claimed.
- The screen-capture adapter crate now has a Rust trigger scheduler proof for
  parent-enabled managed-browser trigger inputs, native app foreground trigger
  inputs, cadence-due decisions, debounce, and disabled-parent suppression. The
  Windows trigger matrix proof captures real browser and Notepad windows after
  scheduler enqueue decisions and deletes raw temp images.
- `scripts/test/screen-ai-local-vlm-proof.mjs` now runs a local Qwen2-VL proof
  matrix over 16 real window captures plus a disabled no-capture/no-AI row. The
  matrix covers controlled browser video, education video, Vimeo-style video,
  social feed/chat, browser game, bypass tool, shopping, school/productivity,
  native app, controlled native game window, native owned-process time-limit,
  unknown low-confidence fallback, controlled violence text, and three timed
  cadence frames. Every captured row validates screen analysis, local AI safety
  result, deterministic policy dry-run, parent explanation artifact, and raw
  image deletion.
- `scripts/test/screen-ai-action-dispatch-proof.mjs` now links the
  screen-derived native owned-process time-limit policy decision into the real
  Windows Rust service time-limit adapter path. The proof preserves the screen
  policy decision ID and evidence refs through dispatch, restart recovery,
  parent cancel, expiry, and a real Windows process-termination result.
- Android child-agent scaffold now has emulator MediaProjection proof with
  explicit OS consent, foreground service, captured frame digest, and raw temp
  deletion. Physical Android parity and silent background capture are not
  claimed.
- Product settings and quality proof are incomplete.
- Raw screen control settings are preserved as design inputs, not
  product-complete implementation proof.

## Current Gap

Opt-in UI, service-owned background timer/foreground watchers, live external
URL/account operator proof, OCR/vision quality beyond controlled fixtures,
runtime service/read-model wiring, physical Android/iOS proof, live macOS
capture proof, Linux root/Wayland portal proof, browser/network/mobile/broad
block action adapters from screen-derived decisions, checklist status movement,
and production parent explanation UX remain. These are remaining screen+AI
delivery items, not external handoff excuses.

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
