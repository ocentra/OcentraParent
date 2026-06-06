# Tracking Retention Settings Service Mutation Proof Source Snapshot

- Branch: codex/tracking-retention-settings-service-mutation-proof
- Base commit at generation: a79e7643d935c623eada98afc04197c2a90ea675
- Git status at proof generation:

```text
M  crates/agent-protocol/src/constants.rs
M  crates/agent-protocol/src/constants/field.rs
M  crates/agent-protocol/src/lib.rs
A  crates/agent-protocol/src/tracking_retention_settings_service_mutation.rs
A  crates/agent-protocol/src/tracking_retention_settings_service_mutation_tests.rs
M  crates/agent-protocol/src/transport.rs
MM crates/agent-service/src/activity_api.rs
M  crates/agent-service/src/main.rs
AM crates/agent-service/src/tracking_retention_settings_service_mutation_payload.rs
A  crates/agent-service/src/tracking_retention_settings_service_mutation_service_tests.rs
MM crates/agent-service/src/websocket.rs
M  docs/features/location-geofence-device-status.md
M  docs/plans/tracking-plan/implementation-checklist.md
M  docs/plans/tracking-plan/workpacks/07-retention-and-custody-model.md
M  docs/plans/tracking-plan/workpacks/32-journal-sqlite-and-read-model-proof.md
A  output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-service-mutation-proof.json
A  output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/27-retention-settings-service-mutation-proof.json
A  output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/28-retention-settings-service-mutation-proof.json
A  output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/00-source-snapshot.md
A  output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/01-contract-proof.log
A  output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/13-security-negative-proof.log
A  output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/16-validation-commands.log
A  output/tracking-plan-proof/tracking-retention-settings-service-mutation-proof/proof.json
M  packages/agent-protocol-domain/src/contracts.ts
M  packages/agent-protocol-domain/src/defaults.ts
A  packages/agent-protocol-domain/src/tracking-retention-settings-service-mutation.ts
A  packages/agent-protocol-domain/tests/tracking-retention-settings-service-mutation.test.ts
A  packages/parent-domain/src/tracking-retention-settings-service-mutation-proof.ts
A  packages/parent-domain/tests/tracking-retention-settings-service-mutation-proof.test.ts
A  scripts/test/tracking-retention-settings-service-mutation-proof.mjs
A  test-results/tracking-retention-settings-service-mutation-proof/proof.json
A  test-results/tracking-retention-settings-service-mutation-proof/retention-settings-service-mutation.json
```

- Scope: protocol/domain/service command mutation proof for local tracking retention settings.
- Source inspected: location/geofence feature doc, WP07, WP32, agent-protocol-domain, parent-domain, agent-protocol, and agent-service READMEs.
