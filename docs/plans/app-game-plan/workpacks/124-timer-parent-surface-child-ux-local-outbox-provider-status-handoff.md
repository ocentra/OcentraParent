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

- Add a Rust-owned App/Game bridge from WP123 child UX provider-preflight rows
  to the existing V0.8 notification provider-status boundary.
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

## Current Status - Phase 1/2 Complete; Phase 3 Open

Commit `dc99eb608` adds the Rust-owned provider-status handoff in
`ocentra-app-game-core`. It maps honest WP123 provider-adapter-required rows to
identity-bound V0.8 manual-required entries and unavailable rows to V0.8
unavailable entries. The handoff preserves scheduler, outbox, provider-channel,
readiness, manual-proof, preference, policy, evidence, and audit references while
keeping provider receipts empty and every delivery/runtime/credential/UI/child/
adapter/enforcement claim false.

Three contract cases cover manual-required mapping, unavailable mapping, and
fail-closed claimed/missing-context inputs. The complete App/Game contract suite
(92 tests), unit suite (10 tests), crate Clippy, focused Enforcer checks,
formatting, diff hygiene, hub guard, and pre-commit passed. Provider execution,
receipts, credentials, production retry and quiet-hours workers, parent UI,
child delivery, retained Phase 3 proof, and whole-plan gates remain open.
