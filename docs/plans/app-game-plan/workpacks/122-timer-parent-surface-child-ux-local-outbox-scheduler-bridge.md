# WP122 Timer Parent-Surface Child UX Local Outbox Scheduler Bridge

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

- Parent-domain build.
- Focused parent-domain test for the child UX local outbox scheduler bridge.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
