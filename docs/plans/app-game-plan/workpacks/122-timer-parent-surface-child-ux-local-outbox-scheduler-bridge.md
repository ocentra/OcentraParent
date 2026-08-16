# WP122 Timer Parent-Surface Child UX Local Outbox Scheduler Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP122 Timer Parent-Surface Child UX Local Outbox Scheduler Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP121 parent-owned local outbox records into deterministic scheduler
proof rows.

## Scope

- Add a parent-domain bridge from WP121 child UX local outbox rows to the
  existing notification local-outbox scheduler JSONL schema.
- Reuse the shared scheduler proof record shape so child UX work stays on the
  same low-level evidence spine as app/game notification readiness.
- Schedule only deliverable child UX local outbox records as `due-local` proof
  rows.
- Keep manual-required and unavailable child UX local outbox rows unscheduled
  with blocked refs.
- Preserve scheduler evidence refs: source entry id/state, local outbox file
  ref, local data path ref, scheduler artifact ref, decision ref, reason code,
  channel, severity, and next-attempt timestamp.

## Non-Goals

- No child-device delivery runtime.
- No notification provider delivery or receipt ingestion.
- No retry execution runtime or quiet-hours timer runtime.
- No durable production scheduler/outbox storage.
- No parent notification UI claim.
- No adapter execution.
- No platform enforcement or broad blocking claim.
- No raw child evidence, URL/title, message text, screenshots, reports, private
  diagnostics, or raw private source rows.
- No package export while `packages/parent-domain/package.json` is owned by
  another lane.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- [x] Rust App/Game crate build and Clippy.
- [x] Focused Rust test for reopened WP121 records, deterministic due-local scheduling,
  and unsafe/manual state rejection.
- [x] Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.

## Current Status - Phase 1/2 Complete; Phase 3 Open

The 2026-08-15 live-code audit invalidated the historical `parent-domain`
completion claim. That package and its advertised JSONL scheduler bridge are not
present in the tracked tree. Commit `b2455cbae` now supplies the Rust owner: the
agent-protocol mirror exposes the shared scheduler record shape and
`ocentra-app-game-core` maps only honest `queued-local` WP121 rows to deterministic
`due-local` records. Manual/receipt/retry/dead-letter rows remain blocked and
unsafe delivery/private-data claims fail closed.

The bounded proof store writes atomically under an exclusive lock, marks the
parent-owned artifact only at persistence, reopens cleanly, treats exact replay
as idempotent, and rejects conflicting scheduler ids. Three scheduler tests plus
the shared schema test pass; the full App/Game crate remains green at 86 contract
plus 10 unit tests, and pre-commit also ran all 236 agent-protocol contract plus
71 unit tests. Dual-crate Clippy and eight focused Enforcer checks pass.
Provider/preference preflight, production retry/quiet-hours execution, service
composition, receipts, UI, child delivery, retained proof, and adapter execution
remain later workpacks/Phase 3.
