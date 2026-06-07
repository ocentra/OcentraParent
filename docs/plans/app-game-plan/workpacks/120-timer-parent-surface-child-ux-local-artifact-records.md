# WP120 Timer Parent-Surface Child UX Local Artifact Records

## Goal

Advance the unified native app plus native game child UX path by carrying
schema-backed local artifact record rows through the live timer parent-surface
read model and portal summary.

## Scope

- Extend the Rust app/game timer parent-surface protocol read model with
  structured child UX local artifact records.
- Derive each artifact record from ready action-result rows that have both
  child reason and child status references.
- Keep record payloads parent-safe: source result id, target domain,
  child-reason refs, child-status refs, and explicit false delivery/adapter/
  platform/raw-private-source claims.
- Extend the TypeScript agent-protocol parser and portal-domain timer
  parent-surface summary to render artifact record source ids and target
  domains.

## Non-Goals

- No child-device UI runtime.
- No notification provider delivery or receipt ingestion.
- No local child-device transport.
- No adapter execution.
- No platform enforcement or broad blocking claim.
- No raw target executable/package values in the artifact records.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Rust protocol and service tests for app/game timer parent-surface read model.
- Agent-protocol-domain parser tests.
- Portal timer parent-surface panel tests.
- Package builds for agent-protocol-domain and portal-domain.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
