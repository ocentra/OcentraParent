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

## Current Status - Phase 1/2 Complete; Phase 3 Open

Commit `64fe263e5` adds the Rust-owned preference-status handoff in
`ocentra-app-game-core` and corrects WP125 to retain safe provider channel,
reason, and severity enums on blocked rows. Parent-preference-required and
manual-required rows become manual setup/manual-required status entries;
unavailable rows become channel-disabled/not-sent entries. All rows preserve
their scheduler identity, safe provider context, preference/quiet-hours/rule/
intent/retry/escalation refs, evidence, policy, audit, and manual-proof refs.

The handoff embeds the existing App/Game parent-surface preference-status row,
keeps provider receipts absent, and leaves preference mutation, frequency UI,
quiet-hours/retry execution, provider delivery/receipts/credentials, cloud
routing, parent UI, child delivery, adapter dispatch, and enforcement false.
Two focused contract tests cover manual/unavailable mappings and claimed,
malformed, or duplicate-context rejection. The complete App/Game contract suite
(96 tests), unit suite (10 tests), crate Clippy, seven focused Enforcer checks,
formatting, diff hygiene, hub guard, and pre-commit passed. Retained Phase 3
proof and whole-plan gates remain open.
