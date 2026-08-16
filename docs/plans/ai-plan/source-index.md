# AI Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: current source ownership and known missing source families as of 2026-08-15.
> Does not prove: implementation completeness, passing tests, proof, CI, or PR readiness.

<!-- /agent-capsule -->

This index was reconciled against the current repository during the 48-workpack
source audit. Deleted historical TypeScript owners, deleted portal filenames,
and deleted proof-script paths were removed. Detailed workpack status is in
[CODE_AUDIT.md](CODE_AUDIT.md).

## Source docs

| Area | Source |
| --- | --- |
| Product AI expectations | `docs/expectations/ai.md` |
| Local AI safety feature | `docs/features/local-ai-safety-evaluator.md` |
| Parent assistant feature | `docs/features/parent-assistant-actions.md` |
| Local AI and TabAgent reuse | `docs/architecture/local-ai-and-tabagent-reuse.md` |
| Provider runtime boundary | `docs/architecture/local-ai-provider-runtime-boundary.md` |
| Evidence context builder | `docs/architecture/local-ai-evidence-context-builder.md` |
| Household AI provider mesh | `docs/plans/ai-plan/household-ai-provider-mesh-plan.md` |
| Browser, screen, app/game, tracking evidence owners | `docs/plans/browser-plan`, `docs/plans/screen-plan`, `docs/plans/app-game-plan`, `docs/plans/tracking-plan` |

## Current Rust contract owners

| Owner | Current source | Current tests |
| --- | --- | --- |
| AI runtime/result/reference contracts | `crates/agent-protocol/src/local_ai.rs`, `local_ai_runtime.rs`, `local_ai_runtime/`, `local_ai_runtime_boundary.rs`, `local_ai_runtime_provider_proof.rs` | `crates/agent-protocol/tests/unit/local_ai_runtime_tests.rs`, `local_provider_adapter_readiness_tests.rs`, `local_ai_runtime_provider_proof_tests.rs`, `policy_tests.rs` |
| AI context wire shapes | `crates/agent-protocol/src/schema_domain_ai_wire.rs`, `schema_domain_mirrors.rs` | `crates/agent-protocol/tests/contract/schema_domain_ai_wire.rs` |
| Parent Assistant contracts | `crates/agent-protocol/src/parent_assistant.rs`, `parent_assistant/` | `crates/agent-protocol/tests/unit/parent_assistant_tests.rs` |
| Activity memory graph | `crates/agent-protocol/src/activity_memory_graph.rs` | `crates/agent-protocol/tests/contract/activity_memory_graph_tests.rs` |
| Child-domain AI events | `crates/agent-protocol/src/child_domain_runtime.rs`, `child_domain_runtime/` | `crates/agent-protocol/tests/contract/child_domain_runtime_events.rs` |
| Screen evidence/mesh contracts | `crates/agent-protocol/src/screen_evidence.rs`, `screen_evidence/` | `crates/agent-protocol/tests/contract/screen_evidence_tests.rs` |

There is no `packages/ai-domain`. `packages/schema-domain` contains narrow
generated policy/custody/browser contracts, but it is not the canonical owner
of the general AI contract family described by older plan text.

## Current runtime and core owners

| Owner | Current source | Current tests |
| --- | --- | --- |
| Local runtime/config/status/install metadata | `crates/agent-service/src/local_ai_runtime_status*.rs`, `local_ai_runtime_config*.rs`, `local_ai_runtime_distribution*.rs`, `local_ai_runtime_install_plan.rs`, `local_ai_model_registry.rs` | `crates/agent-service/tests/unit/local_ai_runtime*.rs`, `local_ai_model_registry_tests.rs` |
| Local text execution | `crates/agent-service/src/local_ai_chat_generation*.rs`, `local_ai_generation_payload.rs` | `crates/agent-service/tests/unit/local_ai_chat_generation*.rs` |
| Singleton scheduler | `crates/agent-service/src/local_ai_provider_scheduler*.rs` | `crates/agent-service/tests/unit/local_ai_provider_scheduler_tests.rs` |
| Parent Assistant runtime/API/context | `crates/agent-service/src/parent_assistant_*.rs`, `parent_assistant_api/`, `parent_assistant_runtime/`, `parent_assistant_evidence_context/` | `crates/agent-service/tests/unit/parent_assistant*.rs`, `parent_assistant_runtime_tests/` |
| LAN AI job lifecycle | `crates/agent-service/src/lan_ai_job_submit_transition.rs`, `lan_pairing/lan_ai_job*.rs`, `lan_pairing_runtime_state/job_leases.rs` | `crates/agent-service/tests/unit/lan_pairing/lan_ai_job*.rs`, `lan_ai_provider_heartbeat.rs`, `lan_ai_route_metadata.rs` |
| Policy dry-run evaluator | `crates/agent-core/src/policy_dry_run_evaluator.rs`, `policy_dry_run_evaluator/` | `crates/agent-core/tests/unit/policy_dry_run_evaluator_*_tests.rs` |
| Activity memory graph persistence | `crates/agent-core/src/activity_store_memory_graph*.rs` | `crates/agent-core/tests/unit/activity_store_memory_graph*_tests.rs` |
| Parent-rule context | `crates/agent-core/src/activity_store_parent_rule_context.rs`, `activity_store_policy_preview_parent_rules.rs` | `crates/agent-core/tests/unit/activity_store_policy_preview_parent_rule_tests.rs` |
| Household provider selection and screen mesh | `crates/agent-core/src/household_ai_provider_route.rs`, `screen_household_mesh_runtime*.rs` | `crates/agent-core/tests/unit/household_ai_provider_route_tests.rs`, `screen_household_mesh_runtime_tests.rs` |
| Child AI boundary | `crates/child-ai-core/src/child_domain_analysis.rs`, `tracking_boundary.rs` | `crates/child-ai-core/tests/contract/child_domain_policy_handoff.rs`, `tests/security/tracking_boundary.rs` |
| Screen AI routing/evidence-only boundary | `crates/screen-ai-core/src/screen_ai_pipeline*`, `screen_intelligence_router/`, `screen_intelligence_router_logic/` | `crates/screen-ai-core/tests/unit/pipeline_decision.rs`, contract tests |
| Tracking AI validation | `crates/tracking-core/src/ai_boundary.rs` | `crates/tracking-core/tests/ai-boundary/ai_result_boundary.rs` |

## Current portal owners

| Owner | Current source | Current tests |
| --- | --- | --- |
| AI runtime/job/memory/remote cards | `packages/portal-domain/src/local-ai-runtime-panel.ts`, `activity-memory-graph.ts`, `apps/portal/src/AiRuntimeRoutePanel.tsx` | `packages/portal-domain/tests/unit/local-ai-runtime-panel.test.ts`, `apps/portal/tests/local-ai/ai-runtime-route-panel.test.ts` |
| Parent Assistant chat | `packages/portal-domain/src/parent-assistant-chat.ts`, `parent-assistant-chat-impl.ts` and the vendor parent portal surface | `apps/portal/tests/e2e/assistant-chat-ui-proof.spec.ts` |

## Feature evidence owners consumed by AI

- Browser URL/social/video/game classification: `crates/browser-core`.
- App/game evidence and deterministic classification: `crates/app-game-core`,
  `crates/agent-core`, and the agent-protocol app-game authority contract.
- Tracking nearby-place request/result validation: `crates/tracking-core` and
  `crates/child-ai-core`.
- Screen capture/queue/deletion: agent-service screen runtime modules; AI does
  not own general capture.
- Deterministic policy/enforcement authority: policy and enforcement plans; AI
  result remains evidence only.

## Verified missing production source families

- General neutral `AiWorkItem`/`AiWorkState` and durable replay ledger.
- Generic stored-evidence `LocalAiEvidenceContext` builder.
- Task-keyed prompt/template and inference-settings registries.
- Canonical AI-result journal plus SQLite ingest/replay/read model.
- Semantic memory index and expiry/invalidation implementation.
- Complete policy/result/action graph edge families.
- Owned OCR engine and guided VLM worker.
- Real model download/extract/checksum/license/corruption/resume pipeline.
- Trusted one-shot remote provider authorization and remote execution adapter.
- Unified AI plus policy explanation read model.

## Deleted historical paths removed from this index

The previous index named nonexistent `packages/parent-domain/src/local-ai*.ts`,
`packages/agent-protocol-domain/src/parent-assistant-command.ts`,
`apps/portal/src/local-ai-runtime-details.ts`,
`apps/portal/src/activity-memory-graph-panel.ts`, and multiple nonexistent
`scripts/test/*ai*proof.mjs` files. They are not current implementation or proof
evidence and must not be used to close a workpack.
