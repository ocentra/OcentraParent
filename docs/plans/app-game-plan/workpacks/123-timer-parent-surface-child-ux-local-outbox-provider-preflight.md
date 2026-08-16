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

- [x] Rust App/Game crate build and Clippy.
- [x] Focused Rust tests for WP122 due-local conversion, provider setup requirements,
  and manual/unsafe input rejection.
- [x] Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1/2 Complete; Phase 3 Open

The 2026-08-15 live-code audit invalidated the historical `parent-domain`
completion claim because that package and its child-UX provider-preflight bridge
are absent from the tracked tree. Commit `3f81b0200` now supplies the Rust owner
in `ocentra-app-game-core`: only a persisted, identity-bound `due-local` WP122 row
becomes `provider-adapter-required`, with scheduler/outbox/decision/channel,
reason, evidence, policy, and audit refs preserved plus three distinct adapter,
credential, and physical-smoke-proof requirements.

Manual and dead-letter scheduler states remain manual-required or unavailable;
unpersisted, mismatched, missing-evidence, duplicate-requirement, or claimed
delivery rows fail closed. Three focused preflight tests, all 89 App/Game
contract tests, 10 unit tests, Clippy, seven focused Enforcer checks, and
pre-commit pass. Provider execution, credential storage, receipts,
retry/quiet-hours runtime, cloud routing, service composition, UI, child
delivery, retained proof, and adapter dispatch remain later workpacks/Phase 3.
