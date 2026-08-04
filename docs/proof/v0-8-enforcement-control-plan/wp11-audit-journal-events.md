# WP11 Audit And Journal Events — Rejected Action Durability

## Scope

This proof covers the fail-safe authorization-rejection boundary in
`crates/agent-service/src/enforcement_api/enforcement_command_execution.rs`.
It does not claim the whole WP11 checklist complete.

## Durable rejection contract

After a command payload has parsed into a typed enforcement request, an
`authorize_enforcement_boundary` rejection is persisted as
`ActivityEventKind::EnforcementAuditRecorded` before the command returns
`AgentCommandRejected`. The durable activity fields retain the policy decision,
action/result/audit identifiers, target type, evidence references, capability
state, failed status, no-op adapter code, rollback-not-required state, and the
canonical rejection reason. The raw policy target value is deliberately not
persisted for a rejection. If persistence fails, the command returns the
storage failure rather than reporting an unrecorded rejection as ready.

The app-game dispatch read model treats an audit as adapter-execution evidence
only when its fields contain `ENFORCEMENT_AUDIT_EVENT`, the serialized typed
execution audit. Rejection records intentionally omit that field, so a
pre-execution rejection cannot be presented as adapter execution. A typed audit
whose actual execution fails remains visible; failed status alone is not used as
the discriminator.

## Query proof

The dedicated real-source regression creates a valid typed request whose
`block` action targets a device, which the enforcement boundary rejects with
`policy-target-mismatch`. It reads the persisted event through
`ActivityStore::latest_enforcement_audit_fields` and proves the audit identifier,
failed status, evidence reference, and rejection reason before accepting the
`AgentCommandRejected` envelope.

## Validation

```text
cargo test -p ocentra-parent-agent-service --test enforcement_runtime enforcement_rejection_journal_tests::rejected_action_is_persisted_as_a_durable_enforcement_audit -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_readback_excludes_rejected_enforcement_audits -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_payload_tests::app_game_adapter_dispatch_result_keeps_typed_executed_failure_evidence -- --exact
# passed: 1
npm run lint:architecture -- --files crates/agent-protocol/src/constants/sqlite.rs crates/agent-core/src/activity_store_enforcement_audit.rs crates/agent-service/src/enforcement_api/enforcement_command_execution.rs crates/agent-service/src/activity_api/app_game_adapter_dispatch_result_payload.rs crates/agent-core/tests/unit/activity_store_enforcement_audit_tests.rs crates/agent-service/tests/unit/enforcement_rejection_journal_tests.rs crates/agent-service/tests/unit/app_game_adapter_dispatch_result_service_tests.rs
# passed
```

## No-claim boundary

- Payload parse failures without a typed enforcement request remain outside this
  slice; they have no trusted audit identity to persist.
- This proof does not establish cross-process/device replay, retention/export,
  portal rendering, or the remaining WP11 transition/query coverage.
- It does not schedule or prove WP04 trusted dispatch.
