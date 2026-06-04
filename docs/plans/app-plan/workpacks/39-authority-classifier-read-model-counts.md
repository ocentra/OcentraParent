# WP39 - Authority Classifier Read-Model Counts

## Scope

Cross-record the shared app/game WP39 read-model count exposure for the native
app plan.

The native app plan gains explicit staged boundary row counts for evidence
claim, identity, approval authority/action-result, platform authority
matrix/rows, and AI classifier result rows through the shared app/game
app-use/games read-model payloads.

This does not claim product-complete native app classifier, policy, portal, or
enforcement control.

## Implementation

- Reuse the shared app/game `AppGameServiceReadModel` boundary rows.
- Expose explicit counts for those rows on existing app-use/games read-model
  DTOs.
- Preserve no-live-classifier, no-policy, no-portal-row, no-adapter, and
  unsupported-platform boundaries.

## Proof

Proof artifacts live in:

```text
output/app-plan-proof/39-authority-classifier-read-model-counts
```

The authoritative implementation proof is the shared app/game workpack:

```text
docs/plans/app-game-plan/workpacks/39-authority-classifier-read-model-counts.md
output/app-game-plan-proof/39-authority-classifier-read-model-counts
```

## Product Doc Decision

`docs/product-capability-checklist.md` is intentionally unchanged. Native app
product status remains blocked on live classifier/provider execution, policy
consumption, portal authority/classifier rows, adapter proof, and platform
support.
