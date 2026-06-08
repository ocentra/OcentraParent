# WP12 iOS Background Region Significant-Change Adapter Source Snapshot

- Branch: codex/tracking-plan-full-continuation-a
- Base commit at generation: 13615298fc1bbb27fe3455587b6e30a1beb81329
- Git status at proof generation:

```text
M docs/features/location-geofence-device-status.md
 M docs/plans/tracking-plan/implementation-checklist.md
 M docs/plans/tracking-plan/workpacks/11-ios-core-location-foreground-adapter.md
 M docs/plans/tracking-plan/workpacks/12-ios-background-region-significant-change-adapter.md
 M docs/plans/tracking-plan/workpacks/33-proof-gates-fixtures-rollout-and-pr-gate.md
 M output/tracking-plan-proof/01-source-index-and-repo-reconciliation/00-source-snapshot.md
 M output/tracking-plan-proof/01-source-index-and-repo-reconciliation/proof.json
 M output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/00-source-snapshot.md
 M output/tracking-plan-proof/02-current-tracking-snapshot-and-gap-map/proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/21-retention-settings-write-command-proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/22-retention-local-service-state-proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/23-retention-durable-settings-proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/24-retention-product-readiness-proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/25-retention-runtime-artifact-gate-proof.json
 M output/tracking-plan-proof/07-retention-and-custody-model/26-retention-product-settings-writable-execution-proof.json
 M output/tracking-plan-proof/11-ios-core-location-foreground-adapter/00-source-snapshot.md
 M output/tracking-plan-proof/11-ios-core-location-foreground-adapter/03-runtime-location-evidence.json
 M output/tracking-plan-proof/11-ios-core-location-foreground-adapter/16-validation-commands.log
 M output/tracking-plan-proof/11-ios-core-location-foreground-adapter/19-ios-location-manual-required-proof.json
 M output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/00-source-snapshot.md
 M output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/05-geofence-transition-proof.json
 M output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/16-validation-commands.log
 M output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/19-ios-location-manual-required-proof.json
 M output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/33-full-product-ui-local-runtime-artifact-capture-proof.json
 M output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-write-command-proof.json
 M output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/33-retention-local-service-state-proof.json
 M output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/34-retention-durable-settings-proof.json
 M output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/35-retention-product-readiness-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/27-ios-location-manual-required-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/33-retention-settings-write-command-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/40-retention-local-service-state-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/41-retention-durable-settings-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/43-retention-product-readiness-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/46-product-readiness-closure-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/49-physical-device-artifact-gate-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/60-retention-runtime-artifact-gate-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/63-real-runtime-handoff-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/64-retention-product-settings-writable-execution-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/65-claim-audit-proof.json
 M output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/66-full-product-ui-local-runtime-artifact-capture-proof.json
 M output/tracking-plan-proof/tracking-claim-audit-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-claim-audit-proof/proof.json
 M output/tracking-plan-proof/tracking-full-product-ui-local-runtime-artifact-capture-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json
 D output/tracking-plan-proof/tracking-ios-location-wp33-gate-proof/00-source-snapshot.md
 D output/tracking-plan-proof/tracking-ios-location-wp33-gate-proof/16-validation-commands.log
 D output/tracking-plan-proof/tracking-ios-location-wp33-gate-proof/proof.json
 M output/tracking-plan-proof/tracking-product-readiness-closure-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-product-readiness-closure-proof/proof.json
 M output/tracking-plan-proof/tracking-real-runtime-handoff-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-real-runtime-handoff-proof/manual-validation-runbook.md
 M output/tracking-plan-proof/tracking-real-runtime-handoff-proof/proof.json
 M output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-retention-product-settings-writable-execution-proof/proof.json
 M output/tracking-plan-proof/tracking-retention-runtime-artifact-gate-proof/00-source-snapshot.md
 M output/tracking-plan-proof/tracking-retention-runtime-artifact-gate-proof/proof.json
 M output/tracking-plan-proof/tracking-retention/product-settings-writable-execution.json
 M packages/parent-domain/src/tracking-ios-location-manual-required-proof.ts
 M packages/parent-domain/tests/tracking-ios-location-manual-required-proof.test.ts
 M scripts/test/tracking-ios-location-manual-required-proof.mjs
 M test-results/tracking-claim-audit-proof/proof.json
 M test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json
 M test-results/tracking-ios-location-manual-required-proof/proof.json
 M test-results/tracking-ios-location-manual-required-proof/tracking-ios-location-manual-required-read-model.json
 D test-results/tracking-ios-location-wp33-gate-proof/proof.json
 M test-results/tracking-physical-device-artifact-gate-proof/proof.json
 M test-results/tracking-product-readiness-closure-proof/proof.json
 M test-results/tracking-real-runtime-handoff-proof/proof.json
 M test-results/tracking-retention-durable-settings-proof/proof.json
 M test-results/tracking-retention-local-service-state-proof/proof.json
 M test-results/tracking-retention-product-readiness-proof/proof.json
 M test-results/tracking-retention-product-settings-writable-execution-proof/proof.json
 M test-results/tracking-retention-runtime-artifact-gate-proof/proof.json
 M test-results/tracking-retention-settings-write-command-proof/proof.json
 M test-results/tracking-source-reconciliation-gap-map-proof/proof.json
```

- Scope: parent-domain iOS Always authorization, region, significant-change, visits, and background relaunch gaps read model against existing simulator package/manual proof plans.
- Source inspected: location/geofence feature doc, location/geofence expectations, platform expectations, WP11 workpack, and WP12 workpack.
- Boundary: this proof keeps Core Location authorization, sample, region, background, entitlement, notification, physical-device, authority, and product-ready behavior manual-required until matching artifacts exist.
- Runtime artifact inventory: 0 present of 9 required; missing refs remain output/tracking-plan-proof/ios-core-location/when-in-use-authorization-state.json, output/tracking-plan-proof/ios-core-location/foreground-location-events.ndjson, output/tracking-plan-proof/ios-core-location/degraded-location-state.json, output/tracking-plan-proof/ios-region-monitoring/02-authorization-state.json, output/tracking-plan-proof/ios-region-monitoring/05-region-transitions.ndjson, output/tracking-plan-proof/ios-region-monitoring/significant-change-events.ndjson, output/tracking-plan-proof/ios-region-monitoring/visit-events.ndjson, output/tracking-plan-proof/ios-region-monitoring/background-terminated-relaunch-result.json, output/tracking-plan-proof/ios-region-monitoring/authority-entitlement-approval.json.
