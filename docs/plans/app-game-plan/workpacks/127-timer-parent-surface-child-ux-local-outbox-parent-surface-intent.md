# WP127 Timer Parent-Surface Child UX Local Outbox Parent Surface Intent

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP127 Timer Parent-Surface Child UX Local Outbox Parent Surface Intent`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
combining child UX provider-status and preference-status rows into redacted
parent-surface intent rows without claiming rendered UI or delivery.

## Scope

- Add a parent-domain parent-surface intent read model for child UX local outbox
  setup rows.
- Consume WP124 child UX provider-status handoff rows and WP126 child UX
  preference-status handoff rows.
- Preserve scheduler, outbox, provider, preference, quiet-hours, drill-in,
  audit, and manual proof refs.
- Keep manual-required and unavailable rows parent-visible as setup/status
  intent only.

## Non-Goals

- No rendered parent notification UI, preference UI, frequency control UI, or
  preference mutation.
- No provider push, email, SMS, WhatsApp, in-app, or child-device delivery
  execution.
- No provider credentials, provider templates, webhooks, delivery receipts, or
  receipt ingestion.
- No production retry workers, quiet-hours timers, durable outbox storage, or
  cloud routing.
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
- Focused parent-domain test for the child UX parent-surface intent.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
