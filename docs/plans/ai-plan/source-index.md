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

| Package                          | Current AI-relevant files                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `packages/parent-domain`         | `src/local-ai.ts`, `src/local-ai-primitives.ts`, `src/local-ai-runtime.ts`, `src/local-ai-runtime-provider-proof.ts`, `src/local-ai-provider-scheduler.ts`, `src/local-ai-model-artifacts.ts`, `src/local-ai-references.ts`, `src/local-ai-context.ts`, `src/local-ai-context-builder.ts`, `src/local-ai-context-primitives.ts`, `src/local-ai-context-result.ts`, `src/local-ai-context-selection.ts`, `src/local-ai-activity-memory-graph.ts`, `src/local-ai-activity-memory-graph-read.ts`, `src/local-ai-graph-reference-contract-proof.ts`, `src/local-ai-recent-memory-window-proof.ts`, `src/local-ai-remote-assistant-boundary-proof.ts`, `src/local-ai-result-journal-sqlite-proof.ts`, `src/local-ai-text-inference-dry-run-proof.ts`, `src/local-ai-text-llm-adapter-boundary-proof.ts`, `src/screen-ai-memory-graph-source-guard-proof.ts`, `src/screen-ai-model-artifact-manifest-proof.ts`, `src/screen-ai-model-runtime-backpressure-proof.ts`, `src/parent-assistant.ts`, `src/parent-assistant-run-state.ts`, `src/policy.ts`, `src/enforcement-policy-dispatch.ts` |
| `packages/agent-protocol-domain` | `src/parent-assistant-command.ts` and protocol command/response shapes that carry parent-assistant routing                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| `packages/portal-domain`         | `src/activity-memory-graph.ts`, `src/parent-assistant-chat.ts`, portal route and DOM contracts for AI-adjacent surfaces                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |

Screen-derived household mesh TypeScript contract/proof families now exist in
`packages/parent-domain` and `scripts/test` as proof-backed boundaries, while
production physical-LAN transport remains separate:

- `scripts/test/screen-ai-household-mesh-proof.mjs`;
- `scripts/test/household-mesh-event-bridge-proof.mjs`;
- `scripts/test/household-ai-provider-route-selection-proof.mjs`;
- `scripts/test/child-agent-ai-policy-authority-proof.mjs`;
- `packages/parent-domain/tests/local-ai-contract-completeness-proof.test.ts`;
- `packages/parent-domain/tests/local-ai-runtime-status-read-model-proof.test.ts`;
- `packages/parent-domain/tests/local-ai-provider-scheduler.test.ts`;
- `packages/parent-domain/tests/screen-ai-model-runtime-backpressure-proof.test.ts`.

## Rust Service And Core

| Crate                  | Current AI-relevant files                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `crates/agent-service` | `local_ai_runtime_status.rs`, `local_ai_runtime_status_unavailable.rs`, `local_ai_runtime_payload.rs`, `local_ai_runtime_config*.rs`, `local_ai_runtime_distribution*.rs`, `local_ai_runtime_install_plan*.rs`, `local_ai_runtime_cache_status.rs`, `local_ai_runtime_model_selection.rs`, `local_ai_runtime_acceleration_config.rs`, `local_ai_model_registry.rs`, `local_ai_chat_generation*.rs`, `local_ai_generation_payload.rs`, `local_ai_provider_scheduler*.rs`, `local_ai_runtime_provider_proof_read_model.rs`, `parent_assistant_*.rs`, `policy_preview_*.rs` |
| `crates/agent-core`    | `policy_dry_run_evaluator*.rs`, `activity_store_memory_graph*.rs`, `activity_store_screen_evidence*.rs`, `activity_store_browser*.rs`, `activity_store_app_game*.rs`, `activity_store_network_flow*.rs`, `activity_store_policy_preview*.rs`, `enforcement_policy_dispatch*.rs`                                                                                                                                                                                                                                                                                          |

Household mesh/eventing Rust runtime source is currently represented through the
reusable event bus plus screen-service bridge proof paths. Planned production
physical-LAN protocol/service families remain:

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
| Runtime/provider      | `scripts/test/local-ai-runtime-provider-proof.mjs`, `scripts/test/local-ai-provider-scheduler-proof.mjs`, `scripts/test/local-ai-runtime-status-read-model-proof.mjs`                                                             |
| Local chat            | `scripts/test/local-ai-chat-proof.mjs`                                                                                                                                                                                            |
| Parent assistant      | `scripts/test/parent-assistant-provider-routing-proof.mjs`, `scripts/test/parent-assistant-action-preview-proof.mjs`, `scripts/test/activity-parent-assistant-runtime-proof.mjs`                                                  |
| External/API boundary | `scripts/test/api-ai-provider-authorization-proof.mjs`                                                                                                                                                                            |
| Rules                 | `scripts/test/ai-rule-index.test.mjs`                                                                                                                                                                                             |
| Package tests         | `packages/parent-domain/tests/local-ai*.test.ts`, `packages/parent-domain/tests/screen-ai*.test.ts`, `packages/parent-domain/tests/parent-assistant.test.ts`                                                                      |
| Rust tests            | `crates/agent-service/src/local_ai_*_tests.rs`, `crates/agent-service/src/parent_assistant_*_tests.rs`, `crates/agent-core/src/activity_store_memory_graph*_tests.rs`, `crates/agent-core/src/policy_dry_run_evaluator*_tests.rs` |

Household mesh proof scripts now present:

- `scripts/test/household-mesh-event-bridge-proof.mjs`;
- `scripts/test/household-ai-provider-route-selection-proof.mjs`;
- `scripts/test/screen-ai-household-mesh-proof.mjs`;
- `scripts/test/child-agent-ai-policy-authority-proof.mjs`;
- `scripts/test/screen-ai-event-driven-runtime-proof.mjs`.

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
- Household AI provider mesh is proof-backed for the local screen-derived route:
  event export/import, child-owned claim/lease, worker-only provider results,
  no raw screen transfer, route selection, and child-agent policy authority are
  covered by retained proof artifacts. Physical household LAN transport, live
  provider gossip, lease expiry/dead-letter runtime behavior, production model
  execution, and portal mesh UI remain product gaps.

## Missing Index Items To Add During Implementation

- Exact portal AI page/source file map after the UI surface is consolidated.
- Exact production command/event names once the general AI job queue and AI
  result journal protocol are wired beyond screen-specific event proofs.
- Exact model artifact registry and local model cache files once product-grade
  model packaging lands.
- Exact OCR/VLM worker source paths once screen intelligence execution starts.
- Exact proof output paths for any remaining UI, physical LAN, and production
  model workpacks.
- Exact household AI provider mesh production transport files once implemented.
- Exact household mesh Rust protocol/service files once implemented.
