# WP124 Timer Parent-Surface Child UX Local Outbox Provider Status Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP124 Timer Parent-Surface Child UX Local Outbox Provider Status Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP123 provider-preflight rows into existing V0.8 provider-status
boundary rows without claiming delivery.

## Scope

- Add a parent-domain bridge from WP123 child UX provider-preflight rows to the
  existing V0.8 notification provider-status boundary.
- Convert provider-adapter-required and manual-required preflight rows into
  manual-required provider-status entries.
- Convert unavailable preflight rows into unavailable provider-status entries.
- Preserve scheduler, outbox, provider-channel, readiness, and manual proof refs
  where the source row provides them.

## Non-Goals

- No provider push, email, SMS, WhatsApp, in-app, or child-device delivery
  execution.
- No provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- No production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- No parent notification UI/history/preferences claim.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw child evidence, URL/title, message text, screenshots, reports, private
  diagnostics, or raw private source rows.
- No package export while `packages/parent-domain/package.json` is owned by
  another lane.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Rust App/Game crate build and Clippy.
- Focused Rust tests for WP123 status conversion, V0.8 boundary parity, and
  delivery-claim rejection.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1 Active

The 2026-08-15 live-code audit invalidated the historical `parent-domain`
completion claim. The tracked Rust tree contains the canonical V0.8 notification
provider-status boundary and WP123 now owns App/Game child-UX provider-preflight
rows, but no current Rust bridge binds those two contracts. The existing
App/Game parent-surface status types are fixture/read-model inputs, not a producer
from WP123.

This workpack is active for a bounded Rust handoff in `ocentra-app-game-core`.
Provider-adapter-required and manual-required preflight rows must become honest
V0.8 manual-required entries; unavailable rows must become unavailable entries.
The bridge must preserve scheduler/outbox/channel/readiness/manual-proof/audit
refs and keep provider delivery, receipts, credentials, sensitive payloads,
cloud routing, UI, child delivery, adapter dispatch, and enforcement unclaimed.
