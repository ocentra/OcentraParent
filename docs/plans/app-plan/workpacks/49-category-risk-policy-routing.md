# WP49 - Category/Risk Policy Routing

## Scope

Cross-record the shared app/game WP49 category/risk policy-routing proof for the
native app plan.

This workpack routes native app category and risk candidates into the Rust
policy compiler only with active category proof, supporting evidence refs,
source/confidence disclosure, and no adapter dispatch.

It does not add live classifier/provider execution, service runtime policy
evaluation, portal UI, notifications, child request delivery, broad app
blocking, platform support, or platform adapter execution.

## Implementation

- Reuse the shared parent-domain category/risk policy-routing contract.
- Route native app categories to app-category policy targets.
- Route risk candidates to risk-app policy targets without hard adapter action
  claims.
- Require local AI candidate routes to cite AI digest refs.
- Keep manual-review candidates manual-required.
- Preserve the no-adapter boundary with `adapterDispatchState:
not-dispatched`.
- `compile_app_game_category_risk_candidate` composes the validated route with
  the compiler and returns the compiler trace/decision without enabling
  enforcement handoff.

2026-08-16 app-plan-code-pass: the Rust route-to-compiler composition is
code-drafted and unvalidated. Agent-service now consumes the shared Rust risk
detection boundary into the typed policy-readiness surface as a
`categoryRiskRouting` row. Candidate rows remain `manual-required` because the
live service read model has no parent-authored rule, device, or local-user
context from which to build a compiler request. Tests, proof, checklist
closure, CI, and compiler service consumption remain deferred.

## Production ownership

- `crates/app-game-core/src/app_game_category_risk_policy_routing.rs`
- `crates/app-game-core/src/app_game_category_risk_policy_routing_types.rs`
- `crates/app-game-core/src/app_game_risk_candidate_detection.rs`
- `crates/app-game-core/src/app_game_policy_target_compiler.rs`
- `crates/agent-protocol/src/app_game_policy_readiness.rs`
- `crates/agent-service/src/activity_api/app_game_policy_readiness_payload.rs`
- `crates/agent-service/src/activity_api/app_game_policy_readiness_sources.rs`
- `crates/parent-runtime-core/src/parent_ui_bridge/app_game_readiness_labels.rs`

## Proof

- Validation and proof commands are deferred by the production-code-only
  phase; no tests, proof runners, Enforcer checks, CI, or PR validation were
  run.

Proof artifacts live in:

```text
output/app-plan-proof/49-category-risk-policy-routing
```

## No-Claim Boundaries

- Category and risk candidates remain policy inputs, not final decisions.
- Risk candidates cannot request block-launch or other hard adapter actions.
- Local AI category routes require digest refs and cannot dispatch adapters.
- Stale category proof is rejected before compile-ready routing.
- Compiler runtime service consumption, portal category/risk authoring,
  child UX, notifications,
  platform adapters, and broad app blocking remain gaps.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged because
primary owns central checklist edits during the merge wave. WP49 moves native
app category quality and risk-app policy-routing composition forward at the
Rust app-game boundary, but product status should not move until runtime
service policy consumption, portal category/risk UI, live classifier/provider
execution, notification/child request UX, broad app blocking, and platform
proof are complete.
