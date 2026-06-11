# WP126 Timer Parent-Surface Child UX Local Outbox Preference Status Handoff

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning WP125 preference-preflight rows into existing V3 notification
rule/provider/retry status entries without claiming parent UI mutation or
delivery.

## Scope

- Add a parent-domain bridge from WP125 child UX preference-preflight rows to
  the existing V3 notification rule/provider/retry contract.
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

- Parent-domain build.
- Focused parent-domain test for the child UX preference-status handoff.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
