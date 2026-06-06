# AI Source Index

This file indexes the current Ocentra Parent AI surface. It is not a substitute
for the owning feature and expectation docs.

## Source Docs

| Area                        | Source                                                    |
| --------------------------- | --------------------------------------------------------- |
| Product AI expectations     | `docs/expectations/ai.md`                                 |
| Local AI safety feature     | `docs/features/local-ai-safety-evaluator.md`              |
| Parent assistant feature    | `docs/features/parent-assistant-actions.md`               |
| Local AI and TabAgent reuse | `docs/architecture/local-ai-and-tabagent-reuse.md`        |
| Provider runtime boundary   | `docs/architecture/local-ai-provider-runtime-boundary.md` |
| Evidence context builder    | `docs/architecture/local-ai-evidence-context-builder.md`  |
| Household AI provider mesh  | `docs/plans/ai-plan/household-ai-provider-mesh-plan.md`   |
| AI/UI runtime notes         | `docs/data and AI Ui plan.md`                             |
| Browser AI slices           | `docs/plans/browser-plan`                                 |
| Screen AI slices            | `docs/plans/screen-plan`                                  |
| App/game AI adjacency       | `docs/plans/app-game-plan`                                |
| Tracking AI adjacency       | `docs/plans/tracking-plan`                                |

## TypeScript Contracts

| Package                          | Current AI-relevant files                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `packages/parent-domain`         | `src/local-ai.ts`, `src/local-ai-primitives.ts`, `src/local-ai-runtime.ts`, `src/local-ai-runtime-provider-proof.ts`, `src/local-ai-provider-scheduler.ts`, `src/local-ai-model-artifacts.ts`, `src/local-ai-references.ts`, `src/local-ai-context.ts`, `src/local-ai-context-builder.ts`, `src/local-ai-context-primitives.ts`, `src/local-ai-context-result.ts`, `src/local-ai-context-selection.ts`, `src/local-ai-activity-memory-graph.ts`, `src/local-ai-activity-memory-graph-read.ts`, `src/parent-assistant.ts`, `src/parent-assistant-run-state.ts`, `src/policy.ts`, `src/enforcement-policy-dispatch.ts` |
| `packages/agent-protocol-domain` | `src/parent-assistant-command.ts` and protocol command/response shapes that carry parent-assistant routing                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `packages/portal-domain`         | `src/activity-memory-graph.ts`, `src/parent-assistant-chat.ts`, portal route and DOM contracts for AI-adjacent surfaces                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Planned household mesh TypeScript contract families:

- `packages/parent-domain/src/household-ai-provider-mesh*.ts`;
- `packages/parent-domain/src/ai-work*.ts`;
- `packages/parent-domain/src/ai-provider*.ts`;
- `packages/parent-domain/src/ai-work-claim*.ts`;
- `packages/parent-domain/src/ai-work-result*.ts`;
- matching `packages/parent-domain/tests/household-ai-provider-mesh*.test.ts`;
- matching `packages/parent-domain/tests/ai-work*.test.ts`;
- matching `packages/parent-domain/tests/ai-provider*.test.ts`.

## Rust Service And Core

| Crate                  | Current AI-relevant files                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `crates/agent-service` | `local_ai_runtime_status.rs`, `local_ai_runtime_status_unavailable.rs`, `local_ai_runtime_payload.rs`, `local_ai_runtime_config*.rs`, `local_ai_runtime_distribution*.rs`, `local_ai_runtime_install_plan*.rs`, `local_ai_runtime_cache_status.rs`, `local_ai_runtime_model_selection.rs`, `local_ai_runtime_acceleration_config.rs`, `local_ai_model_registry.rs`, `local_ai_chat_generation*.rs`, `local_ai_generation_payload.rs`, `local_ai_provider_scheduler*.rs`, `local_ai_runtime_provider_proof_read_model.rs`, `parent_assistant_*.rs`, `policy_preview_*.rs` |
| `crates/agent-core`    | `policy_dry_run_evaluator*.rs`, `activity_store_memory_graph*.rs`, `activity_store_screen_evidence*.rs`, `activity_store_browser*.rs`, `activity_store_app_game*.rs`, `activity_store_network_flow*.rs`, `activity_store_policy_preview*.rs`, `enforcement_policy_dispatch*.rs`                                                                                                                                                                                                                                                                                          |

Planned household mesh Rust protocol/service families:

- `crates/agent-protocol/ai_work.rs`;
- `crates/agent-protocol/ai_provider_mesh.rs`;
- `crates/agent-protocol/ai_work_claim.rs`;
- `crates/agent-protocol/ai_work_result.rs`;
- protocol constants for `ai.provider.*`, `ai.work.*`,
  `ai.work.claim.*`, `ai.work.lease.*`, and `ai.result.*`;
- `crates/agent-service/household_mesh_bridge.rs`;
- `crates/agent-service/household_mesh_transport.rs`;
- `crates/agent-service/ai_provider_advertisement.rs`;
- `crates/agent-service/ai_provider_selection.rs`;
- `crates/agent-service/ai_work_ledger.rs`;
- `crates/agent-service/ai_work_event_handlers.rs`;
- `crates/agent-service/ai_work_claim_lease.rs`;
- `crates/agent-service/ai_result_validation.rs`.

## Portal Surfaces

| App           | Current AI-relevant files                                                                                                                                                                  |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `apps/portal` | `src/local-ai-runtime-details.ts`, `src/activity-memory-graph-panel.ts`, parent assistant chat/runtime integration files, policy preview panel files, AI-adjacent device/activity surfaces |

## Proof Scripts And Tests

| Area                  | Current proof/test files                                                                                                                                                                                                          |
| --------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Runtime/provider      | `scripts/test/local-ai-runtime-provider-proof.mjs`, `scripts/test/local-ai-provider-scheduler-proof.mjs`                                                                                                                          |
| Local chat            | `scripts/test/local-ai-chat-proof.mjs`                                                                                                                                                                                            |
| Parent assistant      | `scripts/test/parent-assistant-provider-routing-proof.mjs`, `scripts/test/parent-assistant-action-preview-proof.mjs`, `scripts/test/activity-parent-assistant-runtime-proof.mjs`                                                  |
| External/API boundary | `scripts/test/api-ai-provider-authorization-proof.mjs`                                                                                                                                                                            |
| Rules                 | `scripts/test/ai-rule-index.test.mjs`                                                                                                                                                                                             |
| Package tests         | `packages/parent-domain/tests/local-ai*.test.ts`, `packages/parent-domain/tests/parent-assistant.test.ts`                                                                                                                         |
| Rust tests            | `crates/agent-service/src/local_ai_*_tests.rs`, `crates/agent-service/src/parent_assistant_*_tests.rs`, `crates/agent-core/src/activity_store_memory_graph*_tests.rs`, `crates/agent-core/src/policy_dry_run_evaluator*_tests.rs` |

Planned household mesh proof scripts:

- `scripts/test/household-ai-provider-mesh-contract-proof.mjs`;
- `scripts/test/household-mesh-event-bridge-proof.mjs`;
- `scripts/test/household-ai-provider-claim-lease-proof.mjs`;
- `scripts/test/household-ai-provider-result-validation-proof.mjs`;
- `scripts/test/child-agent-ai-policy-authority-proof.mjs`;
- `scripts/test/mobile-dormant-ai-provider-proof.mjs`;
- `scripts/test/no-raw-screen-transfer-mesh-proof.mjs`;
- `scripts/test/ai-mesh-event-topology-proof.mjs`.

## Current Claim Boundaries

- AI contracts and status/readiness surfaces exist.
- Local text generation proof exists, but product-grade model quality and
  safety-decision execution still require stronger validation.
- Provider scheduler proof exists, but the AI job queue must be hardened into a
  cross-slice product boundary.
- Context builder contracts and specs exist, but every consuming slice must
  prove real stored-evidence context.
- Memory/graph proof pieces exist, but derived memory must remain source-cited
  and must not replace the encrypted journal and SQLite read models.
- Parent assistant routing exists, but remote/API AI remains outside normal
  child-device safety.
- Household AI provider mesh is planned, but the current local provider
  scheduler and LAN AI job primitives do not yet prove decentralized
  cross-device claim/lease, idempotency, result validation, no raw screen
  transfer, or child-agent policy authority.

## Missing Index Items To Add During Implementation

- Exact portal AI page/source file map after the UI surface is consolidated.
- Exact command/event names once AI job queue and AI result journal protocol are
  wired.
- Exact model artifact registry and local model cache files once product-grade
  model packaging lands.
- Exact OCR/VLM worker source paths once screen intelligence execution starts.
- Exact proof output paths for each AI workpack.
- Exact household AI provider mesh TypeScript contract files once implemented.
- Exact household mesh Rust protocol/service files once implemented.
