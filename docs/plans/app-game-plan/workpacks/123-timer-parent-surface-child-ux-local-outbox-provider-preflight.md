# WP123 Timer Parent-Surface Child UX Local Outbox Provider Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP123 Timer Parent-Surface Child UX Local Outbox Provider Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP122 scheduled child UX local outbox rows into provider-preflight rows
that state exactly what setup is still required before delivery can be claimed.

## Scope

- Add a parent-domain bridge from WP122 child UX local outbox scheduler rows to
  provider-preflight rows.
- Reuse the existing app/game notification provider-preflight pattern for
  provider-adapter-required, manual-required, and unavailable rows.
- Convert scheduled child UX scheduler rows into provider-adapter-required rows
  with scheduler, outbox, decision, channel, reason, adapter, credential, and
  smoke-proof refs preserved.
- Keep manual-required and unavailable rows blocked before provider preflight
  with manual proof requirements.

## Non-Goals

- No provider push, email, SMS, WhatsApp, in-app, or child-device delivery
  execution.
- No provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- No production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
- No parent notification UI.
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
- Focused Rust tests for WP122 due-local conversion, provider setup requirements,
  and manual/unsafe input rejection.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1 Active

The 2026-08-15 live-code audit invalidated the historical `parent-domain`
completion claim because that package and its child-UX provider-preflight bridge
are absent from the tracked tree. WP122 now supplies real Rust scheduler records,
and the checked-in social-alert generated surface demonstrates the intended
provider-adapter/credential/smoke-proof requirement pattern, but no current Rust
owner projects an App/Game child-UX scheduler record into that fail-closed
preflight boundary.

This workpack is active for a bounded Rust provider-preflight contract and
projection in `ocentra-app-game-core`. Only a valid `due-local` row may become
`provider-adapter-required`; it must preserve scheduler/outbox/decision/channel,
reason, evidence, policy, and audit refs while generating explicit adapter,
credential, and physical smoke-proof requirements. Non-due and unsafe rows stay
blocked. Provider execution, credential storage, receipts, retry/quiet-hours
runtime, cloud routing, UI, child delivery, and adapter dispatch remain later
workpacks.
