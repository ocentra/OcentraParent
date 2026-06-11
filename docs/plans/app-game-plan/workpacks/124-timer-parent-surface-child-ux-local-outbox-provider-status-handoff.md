# WP124 Timer Parent-Surface Child UX Local Outbox Provider Status Handoff

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

- Parent-domain build.
- Focused parent-domain test for the child UX provider-status handoff.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
