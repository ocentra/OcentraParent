# WP38 - Service Authority Classifier Surface Evidence

## Scope

Carry the WP31 staged app/game evidence-claim, identity, approval
authority/action-result, platform authority matrix, and AI classifier result
row refs into the existing `agent-service` app-use/games read-model evidence
vectors.

This workpack does not add live classifier/provider execution, dedicated
authority/classifier service events, portal rows, policy consumption, adapter
execution, broad blocking, or platform support claims.

## Implementation

- Add a shared helper that appends boundary-row refs from
  `AppGameServiceReadModel` into activity-surface evidence vectors.
- Preserve embedded evidence refs from evidence-claim and identity rows.
- Add local-db-row refs for approval authority/action-result, platform matrix,
  platform matrix child rows, classifier run ids, and classifier source
  evidence refs.
- Reuse the helper from both app-use and games read-model paths.
- Add focused service tests proving both surfaces preserve the staged refs while
  continuing to expose only the existing read-model rows.

## Proof

- `cargo test -p ocentra-parent-agent-service app_game_boundary_evidence`
- `cargo fmt --check`
- `npm run lint:schema-boundaries`
- `git diff --check`

Proof artifacts live in:

```text
output/app-game-plan-proof/38-service-authority-classifier-surface-evidence
```

## No-Claim Boundaries

- Boundary refs are evidence transport only.
- Service evidence-vector exposure is not a dedicated classifier event stream.
- No live model/provider execution is claimed.
- No portal authority/classifier row, policy decision, adapter execution, broad
  app blocking, or platform support claim is added.

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. WP38 moves
staged boundary refs into existing service read-model evidence vectors only;
product status should not move until live classifier/provider execution, policy
consumption, portal rows, adapter proof, and platform support exist.
