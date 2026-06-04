# WP38 - Service Authority Classifier Surface Evidence

## Scope

Cross-record the shared app/game WP38 service authority/classifier surface
evidence bridge for the native app plan.

The native app plan gains service read-model evidence refs for staged
evidence-claim, identity, approval authority/action-result, platform authority
matrix, and AI classifier result rows through the shared app/game app-use/games
path, without claiming product-complete native app classifier or enforcement
control.

## Implementation

- Reuse the shared app/game `AppGameServiceReadModel` boundary rows.
- Preserve those rows as evidence refs on existing app-use/games read-model
  DTOs.
- Preserve no-live-classifier, no-policy, no-portal-row, no-adapter, and
  unsupported-platform boundaries.

## Proof

Proof artifacts live in:

```text
output/app-plan-proof/38-service-authority-classifier-surface-evidence
```

The authoritative implementation proof is the shared app/game workpack:

```text
docs/plans/app-game-plan/workpacks/38-service-authority-classifier-surface-evidence.md
output/app-game-plan-proof/38-service-authority-classifier-surface-evidence
```

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. Native app
product status remains blocked on live classifier/provider execution, policy
consumption, dedicated portal rows, adapter proof, and platform support.
