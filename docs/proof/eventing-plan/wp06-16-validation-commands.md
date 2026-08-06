# WP06 Validation Commands

| Command | Exit | Result | Notes |
| --- | ---: | --- | --- |
| `cargo test -p ocentra-eventing --test journal_replay` | 0 | pass | 22 passed; append/hash-chain/recovery, corruption, cursor/filter, projection-only safety, journal policy. |
| `cargo test -p ocentra-eventing --test contract topology_manifest` | 0 | pass | 4 passed; deterministic manifest and explicit topology classifications. |
| `cargo test -p ocentra-eventing --test contract compatibility_matrix` | 0 | pass | 3 passed; lineage semantics, deviations, and manual-required scope. |
| `cargo test -p ocentra-eventing --test unit production_shutdown` | 0 | pass | 5 passed; drain/dead-letter, cancellation, and shutdown lifecycle. |
| `cargo test -p ocentra-eventing --test version_skew` | 0 | pass | 2 passed; reject newer and older stored envelope schema versions. |
| `npm run lint:architecture -- --files crates/ocentra-eventing/src crates/ocentra-eventing/tests` | 0 | pass | Current repository architecture-policy gate. |
| `rg -n --glob '*.rs' 'EventBus::global|OnceLock<.*EventBus|Lazy<.*EventBus|static .*EventBus' crates/ocentra-eventing/src crates/ocentra-eventing/tests` | 1 | expected zero-match | No hidden global EventBus pattern found. |
| `cargo test -p ocentra-parent-agent-protocol --test contract enforcement_audit_boundary_tests --quiet` | 0 | pass | 2 passed; `EnforcementAuditJournalEvent` has the canonical Eventing contract, aggregate, and idempotency identity. |
| `cargo test -p ocentra-parent-agent-service --test enforcement_eventing_journal --quiet` | 0 | pass | 1 passed; nested sidecar parents are created before append, and an exact append retry preserves one hash-chained record with projection-only replay. |
| `cargo test -p ocentra-parent-agent-service --test enforcement_runtime production_command_retry_keeps_eventing_identity_and_command_correlation` | 0 | pass | The organized production-path harness executes `build_enforcement_audit_report_with_paths` twice through `record_eventing_enforcement_audit`; the nested sidecar retains only before/final records and each uses command `message_id` correlation plus stable `sent_at` identity. |
| `cargo test -p ocentra-parent-agent-service --test enforcement_runtime` | 0 | pass | 42 passed; the full service enforcement-runtime target compiles the Linux-stable harness path used by CI. |
| `cargo test -p ocentra-eventing --test journal_replay --quiet` | 0 | pass | 22 passed; reusable journal corruption, recovery, cursor/filter, projection-only, and idempotency coverage remains green. |
| `cargo clippy -p ocentra-parent-agent-protocol --all-targets -- -D warnings` | 0 | pass | Touched protocol crate and all targets are warning-free. |
| `cargo clippy -p ocentra-parent-agent-service --test enforcement_eventing_journal -- -D warnings` | 0 | pass | New consumer-handoff integration harness is warning-free. |
| `cargo clippy -p ocentra-parent-agent-service --all-targets -- -D warnings` | 0 | pass | CI-equivalent service Clippy gate passed after the production test harness path repair. |
| `npm run lint:architecture -- --files crates/agent-protocol/src/constants/enforcement.rs crates/agent-protocol/src/enforcement.rs crates/agent-protocol/tests/contract/enforcement_audit_boundary_tests.rs crates/agent-service/src/enforcement_api/enforcement_pre_action_journal.rs crates/agent-service/src/enforcement_api/enforcement_pre_action_journal/eventing_journal.rs crates/agent-service/src/enforcement_api/enforcement_command_execution.rs crates/agent-service/tests/enforcement_eventing_journal.rs crates/agent-service/tests/unit/enforcement_eventing_retry_production_tests.rs crates/agent-service/tests/unit/enforcement_runtime.rs` | 0 | pass | Scoped Rust source/test hard gate includes the production retry proof and its test harness. |

## Validation boundary

These commands prove Eventing WP06 generic journal/replay/topology mechanics
and the narrow typed enforcement audit-summary handoff. They do not prove
enforcement adapter dispatch, authorization, rollback, side effects, transport,
retention, deletion, or WP10.
