# Repo Audit Workpack Index

Choose one structural workpack. Do not open all workpacks unless acting as the coordinator.

| Status | Workpack | Owns | Does not own | Output |
| --- | --- | --- | --- | --- |
| open | [WP01 Test Topology Inventory](workpacks/01-test-topology-inventory.md) | Real tests, empty scaffolds, inline source tests. | Moving tests or rewriting test frameworks. | Test topology inventory and recommended cleanup queue. |
| open | [WP02 CI Package Coverage Matrix](workpacks/02-ci-package-coverage-matrix.md) | Mapping crates/packages/apps to local commands and CI jobs. | Implementing feature tests. | CI/package coverage matrix and missing-gate list. |
| open | [WP03 Architecture Policy Reconciliation](workpacks/03-architecture-policy-reconciliation.md) | Re-export/barrel policy truth and cleanup/exceptions decision. | Bulk source rewrite without approved policy. | Architecture policy decision and scoped gate plan. |
| open | [WP04 Ownership Drift Map](workpacks/04-ownership-drift-map.md) | Misplaced code/tests and broad frontage packages. | Feature implementation. | Owner/frontage/move-candidate map. |
| open | [WP05 DRY Common-Core Candidate Map](workpacks/05-dry-common-core-candidate-map.md) | Repeated logic and safe extraction candidates. | Extracting before tests exist. | Duplicate pattern table and pre-extraction proof requirements. |
| open | [WP06 Plan Thread Review Gate](workpacks/06-plan-thread-review-gate.md) | Applying structural audit to pasted plan reports. | Fixing plan code directly. | Per-plan verdict table and next-slice recommendations. |

## Dependency order

```text
WP01 -> WP02 -> WP03 -> WP04 -> WP05 -> WP06
```

WP04 and WP05 can run after WP01/WP02 produce enough inventory. WP06 consumes this baseline while plan reports are pasted.
