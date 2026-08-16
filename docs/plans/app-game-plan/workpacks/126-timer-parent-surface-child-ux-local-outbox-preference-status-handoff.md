# WP126 Timer Parent-Surface Child UX Local Outbox Preference Status Handoff

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP126 Timer Parent-Surface Child UX Local Outbox Preference Status Handoff`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP125 preference-preflight rows into existing V3 notification
rule/provider/retry status entries without claiming parent UI mutation or
delivery.

## Scope

- Add a Rust-owned App/Game bridge from WP125 child UX preference-preflight rows
  to the existing V3 notification rule/provider/retry status contract.
- Convert parent-preference-required and manual-required preflight rows into
  manual setup status entries.
- Convert unavailable preflight rows into disabled/not-sent status entries.
- Preserve scheduler, outbox, provider-channel, reason, preference,
  quiet-hours, evidence, audit, and manual proof refs where the source row
  provides them.

## Non-Goals

- No parent preference UI, frequency control UI, parent notification UI, or
  preference mutation.
- No quiet-hours timer runtime or retry worker runtime.
- No provider push, email, SMS, WhatsApp, in-app, or child-device delivery
  execution.
- No provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- No production durable outbox storage or cloud routing.
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
- Focused Rust tests for manual, unavailable, and malformed preference-status
  handoff rows.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1 Active

The 2026-08-15 live-code audit invalidated the historical `parent-domain`
completion claim. That package is absent, and the broad timer service/portal
files previously mapped by the graph contain only fixture/read-model consumers,
not a producer from WP125. The current Rust owner is `ocentra-app-game-core`,
where WP125 now supplies typed preference-preflight rows and the existing
App/Game parent-surface contract supplies the downstream preference-status
shape.

This workpack is active for a bounded Rust handoff. Parent-preference-required
and manual-required rows must map to manual setup status; unavailable rows must
map to disabled/not-sent status. The producer must preserve source scheduler,
outbox, provider channel, reason, preference, quiet-hours, evidence, audit, and
manual-proof refs and reject unsafe claims or malformed source shapes. The audit
also found that WP125 must retain safe provider-channel/reason/severity enums on
blocked rows so this handoff does not invent them. Preference mutation,
quiet-hours/retry execution, delivery, receipts, credentials, cloud routing,
parent UI, child delivery, adapter dispatch, and enforcement remain unclaimed.
