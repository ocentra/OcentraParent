# WP177 App/Game Category Unknown Policy Readiness Service Consumption

<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `WP177 App/Game Category Unknown Policy Readiness Service Consumption`
> Kind: assigned workpack; read only when selected by hub or WORKPACK_INDEX.
> Read when: Only when this exact workpack is assigned or selected from WORKPACK_INDEX.md.
> Stop rule: Do not open sibling workpacks. Do not move product status unless this workpack and proof rows say so.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Before DONE, select tests in TEST_PROOF_EXPECTATIONS.md and update proof/checklist rows.

<!-- /agent-capsule -->

## Scope

Extend the existing app/game policy-readiness service command and parent-safe
portal-domain intent so category and unknown-state policy inputs are visible in
the same service-backed path as policy evidence, approval, platform authority,
and classifier context.

## Implementation

- Add category/unknown readiness fields to the Rust protocol read model:
  `categoryRoutingReady`, `unknownReviewRequired`,
  `categoryCandidateRowCount`, and `unknownReviewRowCount`.
- Add `categoryCandidate` and `unknownReview` readiness row kinds to the Rust
  protocol and TypeScript protocol parser.
- Derive category readiness from existing
  `AppGameServiceReadModel.inventory_rows[].category_candidates`.
- Derive unknown-review readiness from inventory, runtime, foreground, and
  launcher rows whose classification is `unknownProcess`, `possiblyGame`, or
  `launcherGameCandidate`.
- Render category candidate and unknown-review readiness rows plus
  category/unknown counts in the App/Game Sessions policy-readiness intent.
- Keep `adapterDispatchClaimed=false` and keep these rows as readiness/status
  inputs only.

## Validation

- `cargo test -p ocentra-parent-agent-protocol app_game_policy_readiness`:
  1 passed.
- `cargo test -p ocentra-parent-agent-service app_game_policy_readiness`:
  2 passed.
- `cargo test -p ocentra-parent-runtime-core --test integration
  app_game_sessions_route_load_attaches_rust_owned_app_game_panels`: 1 passed.
- Portal workspace run containing
  `tests/unit/app-game-policy-readiness-panel.test.ts`: 37 files and 154 tests
  passed.
- Focused Enforcer `architecture-policy`, `source-shape`, `required-tests`,
  `no-test-doubles`, `no-naked-domain-strings`, `validation-bypass`, and
  `reexports`: passed across the mapped protocol, service, parent-runtime, and
  portal source/test paths.

## No-Claim Boundaries

- Does not add finished parent approval UI.
- Does not add finished child request UI.
- Does not add live classifier/provider quality.
- Does not execute the runtime policy evaluator.
- Does not dispatch adapters, broad installed-app blocking, or platform
  enforcement.
- Does not change the shared product checklist because another lane owns that
  file.
- Does not edit the shared SVG renderer.

## Current Status - Phase 1/2 Complete; Phase 3 Open

The 2026-08-15 code-first audit verified that this bounded readiness/status
path is already implemented. Rust protocol carries the category/unknown fields
and row kinds; agent-service derives category candidates and unknown-review
rows from the live service read model; parent-runtime turns them into
parent-safe panel details; and the portal renders those rows and counts. The
focused validations above are green, and `adapterDispatchClaimed` remains an
explicit separate field instead of being inferred from readiness.

The previous audit incorrectly treated missing live compiler/evaluator
consumption as a WP177 Phase 1 failure. That consumption is still a real
whole-plan gap, but it is outside this workpack's explicit readiness-only scope
and no-claim boundaries. Retained proof and the whole-plan Phase 3 gates remain
open, so this workpack is not marked broadly DONE.
