# WP125 Timer Parent-Surface Child UX Local Outbox Preference Preflight

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP122 scheduled child UX local outbox scheduler rows into parent
preference and quiet-hours preflight rows without claiming UI or delivery.

## Scope

- Add a parent-domain bridge from WP122 child UX local outbox scheduler rows to
  child UX parent preference preflight rows.
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

- Parent-domain build.
- Focused parent-domain test for the child UX preference preflight.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
