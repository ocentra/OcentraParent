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
persisted for a rejection. Its durable activity event key is a distinct stable
rejection-prefixed key; the original `ENFORCEMENT_AUDIT_EVENT_ID` remains a
reference field, so a corrected retry can persist its final execution audit
under that original key. If persistence fails, the command returns the
storage failure rather than reporting an unrecorded rejection as ready.

The app-game dispatch read model treats an audit as adapter-execution evidence
only when its fields contain `ENFORCEMENT_AUDIT_EVENT`, the serialized typed
execution audit, and the app-game dispatch result read-model identifier.
That identifier is derived by the service from the
`AgentActivityAppGameAdapterDispatchExecute` command boundary, not accepted
from caller payload fields. It is stamped only on the final execution audit,
not the durable pre-action `would-enforce` / `no-op` audit. Rejection records
intentionally omit the typed audit field, so neither a pre-execution rejection
nor a pre-action audit can be presented as adapter execution. A typed audit
whose actual execution fails remains visible; failed status alone is not used
as the discriminator.

## Query proof

The dedicated real-source regression creates a valid typed request whose
`block` action targets a device, which the enforcement boundary rejects with
`policy-target-mismatch`. It reads the persisted event through
`ActivityStore::latest_enforcement_audit_fields` and proves the audit identifier,
failed status, evidence reference, and rejection reason before accepting the
`AgentCommandRejected` envelope.

The retry regression records that rejected request, then sends a corrected
process dry-run with the same original audit identifier. It proves both the
rejection-prefixed activity key and the final audit key are durable, while the
second command reports `AgentEnforcementAuditReported`.

When otherwise valid audit records share the same timestamp, enforcement-audit
queries use SQLite's persisted insertion order (`rowid`) as the tie-breaker.
The regression inserts a lexically later executed audit followed by a
rejection-prefixed audit at the same timestamp and proves the later persisted
rejection is selected.

## Validation

```text
cargo test -p ocentra-parent-agent-service --test enforcement_runtime enforcement_rejection_journal_tests::rejected_action_is_persisted_as_a_durable_enforcement_audit -- --exact
cargo test -p ocentra-parent-agent-core --test unit activity_store_enforcement_audit_tests::activity_store_uses_persisted_insert_order_for_equal_time_enforcement_audits -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test enforcement_runtime enforcement_rejection_journal_tests::rejected_audit_does_not_dedupe_a_corrected_retry_final_audit -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_readback_excludes_rejected_enforcement_audits -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_payload_tests::app_game_adapter_dispatch_result_keeps_typed_executed_failure_evidence -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_payload_tests::app_game_adapter_dispatch_result_rejects_unowned_execution_audit_evidence -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_result_command_ignores_unowned_store_audit_evidence -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_readback_skips_newer_rejected_audit_for_typed_execution -- --exact
# passed: 1
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_does_not_mark_pre_action_audit_as_execution_evidence -- --exact
# passed: 1
npm run lint:architecture -- --files crates/agent-service/src/enforcement_api/enforcement_command_execution.rs crates/agent-service/tests/unit/app_game_adapter_dispatch_result_service_tests.rs
# passed
cargo test -p ocentra-parent-agent-service --test app_game_activity_read_models app_game_adapter_dispatch_result_service_tests::app_game_adapter_dispatch_execute_command_runs_scoped_enforcement_and_readback -- --exact
# passed: 1
npm run lint:architecture -- --base origin/main --head HEAD
# passed
```

## No-claim boundary

- Payload parse failures without a typed enforcement request remain outside this
  slice; they have no trusted audit identity to persist.
- This proof does not establish cross-process/device replay, retention/export,
  portal rendering, or the remaining WP11 transition/query coverage.
- It does not schedule or prove WP04 trusted dispatch.
