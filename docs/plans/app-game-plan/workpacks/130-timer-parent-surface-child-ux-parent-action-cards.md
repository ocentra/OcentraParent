# WP130 Timer Parent-Surface Child UX Parent Action Cards

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP130 Timer Parent-Surface Child UX Parent Action Cards`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Goal

Advance the unified native app plus native game child UX evidence spine by
turning live child UX parent-surface intent records into parent-visible action
cards on the App/Game Sessions route.

## Scope

- Extend the portal-domain timer parent-surface intent with parent action rows
  derived from child UX parent-surface intent records.
- Render the parent action rows as App/Game Sessions cards backed by the real
  service read model event.
- Keep the cards parent-safe: target meaning, readable manual-action and
  preference-setup states, source/artifact refs, drill-in refs, manual-proof
  refs, and explicit no-claim adapter/child-delivery/platform states.

## Non-Goals

- No parent preference mutation, notification rule mutation, or frequency
  controls.
- No provider delivery, delivery receipts, receipt ingestion, provider
  credentials, cloud routing, retry workers, or quiet-hours runtime.
- No child runtime delivery.
- No adapter dispatch.
- No platform enforcement or broad blocking claim.
- No raw private source rows, raw target values, private diagnostics,
  screenshots, reports, or sensitive child evidence in the portal cards.
- No package export while package ownership remains elsewhere.
- No central product checklist update while `docs/product-capability-checklist.md`
  is owned by another lane.

## Validation

- Portal-domain build.
- Focused portal timer parent-surface panel test.
- Formatting, no-test-doubles, source-shape, `git diff --check`, lane guard, and
  hub guard before commit.
