# WP121 Timer Parent-Surface Child UX Local Outbox Bridge

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP121 Timer Parent-Surface Child UX Local Outbox Bridge`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX path by converting
validated timer parent-surface child UX local artifact records into parent-owned
local outbox records.

## Scope

- Add a parent-domain bridge from WP118/WP120 child UX local artifact records to
  existing notification local-outbox JSONL records.
- Reuse the existing local-outbox schema so child UX handoff output is
  represented by one parent-owned evidence spine instead of a duplicate outbox
  shape.
- Queue only deliverable child UX local artifacts as `queued-local` records.
- Keep manual-required and unavailable child UX artifacts out of queued JSONL
  records with blocked refs.
- Preserve minimal payload refs: family/device scope, severity, reason code,
  evidence refs, policy refs, audit refs, parent action, and copy-token template
  refs.

## Non-Goals

- No child-device delivery runtime.
- No notification provider delivery or receipt ingestion.
- No scheduler runtime.
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
- Focused parent-domain test for the child UX local outbox bridge.
- Formatting, source-shape, no-test-doubles, `git diff --check`, lane guard, and
  hub guard before commit.
