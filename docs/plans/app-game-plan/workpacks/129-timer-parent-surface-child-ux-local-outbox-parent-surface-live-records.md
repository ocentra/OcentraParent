# WP129 Timer Parent-Surface Child UX Local Outbox Parent Surface Live Records

## Goal

Advance the unified native app plus native game child UX evidence spine by
surfacing structured child UX local outbox parent-surface intent records through
the live timer parent-surface protocol, service read model, and parent portal
summary.

## Scope

- Extend the shared timer parent-surface read model with redacted child UX
  parent-surface intent records.
- Mirror the contract in Rust protocol types.
- Derive live service records from replayed child UX local artifact/action
  result rows.
- Render parent-safe source, artifact, target-domain, drill-in, and manual-proof
  references in the existing App/Game Sessions timer parent-surface panel.

## Non-Goals

- No rendered parent notification UI, parent preference UI, frequency controls,
  or preference mutation.
- No provider delivery, delivery receipts, receipt ingestion, provider
  credentials, cloud routing, retry workers, or quiet-hours runtime.
- No child runtime delivery.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, private diagnostics,
  screenshots, reports, or sensitive child evidence in the portal summary.
- No package export while package ownership remains elsewhere.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Focused agent-protocol-domain timer parent-surface parser test.
- Focused Rust protocol timer parent-surface serialization test.
- Focused agent-service timer parent-surface payload test.
- Portal-domain build.
- Focused portal timer parent-surface panel test.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
