# WP125 Timer Parent-Surface Child UX Local Outbox Preference Preflight

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP125 Timer Parent-Surface Child UX Local Outbox Preference Preflight`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP122 scheduled child UX local outbox scheduler rows into parent
preference and quiet-hours preflight rows without claiming UI or delivery.

## Scope

- Add a Rust-owned App/Game bridge from WP122 child UX local outbox scheduler
  rows to child UX parent preference preflight rows.
- Convert scheduled child UX rows into parent-preference-required rows with
  parent preference, notification frequency, and quiet-hours setup refs.
- Keep manual-required and unavailable scheduler rows blocked before preference
  setup.
- Preserve scheduler, outbox, scheduler decision, provider-channel, reason, and
  manual proof refs where the source row provides them.

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
- Focused Rust tests for ready, blocked, and malformed child UX preference
  preflight rows.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1/2 Complete; Phase 3 Open

Commit `e550e55c2` adds the Rust-owned preference-preflight projection in
`ocentra-app-game-core`. Honest persisted `due-local` scheduler rows become
parent-preference-required rows with distinct parent preference, notification
frequency, and quiet-hours requirements. Manual and dead-letter scheduler rows
remain explicitly manual-required or unavailable without exposing provider or
outbox identity as ready input.

The projection preserves scheduler, outbox, provider-channel, reason, severity,
evidence, policy, and audit references and keeps preference mutation,
frequency-control UI, quiet-hours runtime, provider delivery/receipts/
credentials, cloud routing, parent UI, child delivery, adapter dispatch, and
enforcement unclaimed. Two focused contract tests cover ready/manual/unavailable
routing plus unpersisted, mismatched, claimed, and duplicate-requirement failures.
The complete App/Game contract suite (94 tests), unit suite (10 tests), crate
Clippy, seven focused Enforcer checks, formatting, diff hygiene, hub guard, and
pre-commit passed. Retained Phase 3 proof and whole-plan gates remain open.
