# WP123 Timer Parent-Surface Child UX Local Outbox Provider Preflight

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

- Parent-domain build.
- Focused parent-domain test for the child UX local outbox provider preflight.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
