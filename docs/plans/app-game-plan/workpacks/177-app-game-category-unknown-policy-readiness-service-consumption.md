# WP177 App/Game Category Unknown Policy Readiness Service Consumption

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

- `cmd /c npm run build --workspace @ocentra-parent/agent-protocol-domain`
- `cmd /c npm run test --workspace @ocentra-parent/agent-protocol-domain -- app-game-policy-readiness`
- `cmd /c npm run test --workspace @ocentra-parent/portal-domain -- app-game-policy-readiness-panel`
- `cargo test -p ocentra-parent-agent-protocol app_game_policy_readiness`
- `cargo test -p ocentra-parent-agent-service app_game_policy_readiness`
- `node scripts/test/app-game-category-unknown-policy-readiness-service-consumption-proof.mjs`

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
