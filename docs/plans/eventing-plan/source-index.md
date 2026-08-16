# Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Source Index`
> Kind: source ownership index; read only when source ownership is unclear.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not inspect broad source from here; use only the named package/crate path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This index records the source material for the reusable Rust event bus plan.
Use it before implementation so workers do not invent a second bus shape.

## Unity/C# Lineage Reference

The Unity/C# bus is the ancestor of the TypeScript package. Reuse the behavior
ideas, but replace Unity-specific shapes with Rust runtime ownership,
validation, and tests.

| Source                                                                                                            | Reusable Semantics                                                                                                                                                     |
| ----------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventBus.cs`                      | Central bus, sync/async subscribers, loose `force`, in-flight duplicate guard, queued events, batch retry, timeouts, clear, and queued drain on subscribe.             |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventRegistrar.cs`                | Registrar-owned subscription lifecycle and idempotent dispose/unsubscribe-all behavior.                                                                                |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\Events\EventArgsBase.cs`                                  | Timestamp, unique identifier, republish flag, dispose hook, and debug string. Rust keeps metadata but replaces disposal with explicit ownership/service cleanup rules. |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\Events\GameEvents\PlayerDecission\PlayerDecisionEvent.cs` | Base-event plus derived decision-event family; Rust should use typed enum/wrapper variants, not inheritance, downcasts, erased payloads, or loose strings.             |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\EventInfo.cs`                     | Publisher/subscriber usage mapping.                                                                                                                                    |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\ScriptInfo.cs`                    | Event health states: pass, fail, no subscriber, and no publisher.                                                                                                      |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\UsageInfo.cs`                     | Stored publisher/subscriber graph.                                                                                                                                     |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\Editor\EventBusManager.cs`        | Editor/source scanning that finds event publishers/subscribers and makes orphan event states visible.                                                                  |
| `E:\ocentra-games\References\Scripts\OcentraAI\LLMGames\LLMGamesCommon\EventBus\Editor\UsageInfoDrawer.cs`        | Human-readable usage table for event graph inspection.                                                                                                                 |

## Ocentra Games Reference

The Games package is a design reference only. Reuse the semantics; do not copy
the TypeScript implementation directly.

| Source                                                                                          | Reusable Semantics                                                                                                                                                                                                                                                                               |
| ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `E:\ocentra-games\packages\eventing-domain\src\core\EventBus.ts`                                | Subscribe, async subscribe, publish, publishAsync, queue when no handler exists, retry, TTL, max queue, async timeout, in-flight duplicate guard, and test injection.                                                                                                                            |
| `E:\ocentra-games\packages\eventing-domain\src\core\EventRegistrar.ts`                          | Registrar-owned subscriptions and dispose/unsubscribe lifecycle.                                                                                                                                                                                                                                 |
| `E:\ocentra-games\packages\eventing-domain\src\core\EventArgsBase.ts`                           | Timestamp, unique identifier, republish flag, target handler marker, static event type requirement, and dispose hook.                                                                                                                                                                            |
| `E:\ocentra-games\packages\eventing-domain\src\core\OperationResult.ts`                         | Success/failure result with attempts and error message.                                                                                                                                                                                                                                          |
| `E:\ocentra-games\packages\eventing-domain\src\core\OperationDeferred.ts`                       | One-shot deferred completion with timeout and double-settlement guard.                                                                                                                                                                                                                           |
| `E:\ocentra-games\packages\eventing-domain\src\testing\createTestEventBus.ts`                   | Isolated test bus construction.                                                                                                                                                                                                                                                                  |
| `E:\ocentra-games\packages\eventing-domain\src\events\**\*.ts` and `src\events\EventTypeMap.ts` | Full event taxonomy scan: 179 EventArgsBase event types observed in the explorer pass, no duplicate event type strings in that scan, generated maps/exports, many request/query events with deferred completion, republishable image events, and discoverability across shipped event contracts. |
| `E:\ocentra-games\packages\eventing-domain\tests\EventBus.spec.ts`                              | Failure, async parallel handling, queued event TTL behavior.                                                                                                                                                                                                                                     |
| `E:\ocentra-games\packages\eventing-domain\tests\EventRegistrar.spec.ts`                        | Registrar subscribe, publish, dispose, and factory behavior.                                                                                                                                                                                                                                     |
| `E:\ocentra-games\packages\eventing-domain\tests\EventArgsBase.spec.ts`                         | Event type enforcement, generated ids, timestamp, and disposal behavior.                                                                                                                                                                                                                         |
| `E:\ocentra-games\packages\eventing-domain\tests\OperationResult.spec.ts`                       | Result shape behavior.                                                                                                                                                                                                                                                                           |
| `E:\ocentra-games\packages\eventing-domain\src\events\assets\UploadAssetEvent.ts`               | Command event with payload plus deferred response.                                                                                                                                                                                                                                               |
| `E:\ocentra-games\packages\eventing-domain\src\events\assets\SyncToR2Event.ts`                  | Fire-and-complete command event pattern.                                                                                                                                                                                                                                                         |

## Pasted Planning Inputs

| Source                                                                                   | Covered Area                                                                                                                                                          |
| ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `C:\Users\sujan\.codex\attachments\800513e4-7e64-4fa3-8835-4180f7ec8b82\pasted-text.txt` | Rust-first Parent direction, two-layer live bus plus durable journal, crate split, event namespaces, network to AI to policy to enforcement chain, and worker prompt. |
| `C:\Users\sujan\.codex\attachments\ebee5dc4-0786-4445-a1f3-bb9e1f42c557\pasted-text.txt` | Reusable crate shape, module list, starter Rust API sketch, tests, NDJSON journal direction, and solid-v1 checklist.                                                  |

## Ocentra Parent Rules

| Source                                                   | Constraint                                                                                                                               |
| -------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `AGENTS.md`                                              | Keep changes narrow, contract-first, validated, and hub/lane aware.                                                                      |
| `.ocentra-ai\rules\ocentra-parent-rules.mdc`             | Route through feature docs and rule files before implementation.                                                                         |
| `.ocentra-ai\rules\ocentra-parent-domain-boundaries.mdc` | Shared meaning and event names live in domain/protocol crates, not app/service local strings.                                            |
| `.ocentra-ai\rules\ocentra-parent-rust-service.mdc`      | Tokio async service, no blocking IO in async handlers, lock scopes stay small, platform-neutral contracts stay outside platform modules. |
| `.ocentra-ai\rules\ocentra-parent-test-rules.mdc`        | Tests use real contracts, real Rust serde structs, real Tokio/local transports/filesystem paths, and no mocks/fakes/stubs/spies.         |
| `.ocentra-ai\rules\ocentra-parent-source-shape.mdc`      | Split code by ownership before files become mixed-concern runtime piles.                                                                 |
| `.ocentra-ai\rules\ocentra-parent-validation.mdc`        | Use focused validation during work and exact validation reports at handoff.                                                              |

## Parent Product Sources

| Source                                       | Relevance                                                                                                               |
| -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `docs/features/child-agent-local-service.md` | Owns the Rust child-agent authority boundary that will eventually consume eventing.                                     |
| `docs/features/network-domain-control.md`    | Network work already references `ocentra-eventing`; treat it as a consumer dependency, not a future crate placeholder. |
| `docs/expectations/network-flow-evidence.md` | Network event contracts must preserve evidence boundaries and no overclaiming.                                          |
| `docs/plans/network-plan/README.md`          | Network plan should consume the reusable Rust bus instead of inventing a network-only bus.                              |
| `apps/portal`                                | Vite/TypeScript view surface only; it must not own evidence, policy, AI, enforcement, cascade, or audit business logic. |
| `crates/agent-service`                       | Rust runtime boundary for parent/controller and child-agent service orchestration that should use the shared event bus. |

## Current Rust Workspace

| Source                  | Current Role                                                                                                                                                                            |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Cargo.toml`            | Workspace members now include `crates/ocentra-eventing`; any plan text claiming the reusable crate is not implemented yet is stale.                                                    |
| `crates/agent-protocol` | Parent/child/network protocol constants and serde structs; focused contract tests, not historical plan text, prove current eventing adoption.                                          |
| `crates/agent-core`     | Runtime core already contains `ocentra_eventing` consumer surfaces and remains the main local-bus consumer boundary.                                                                   |
| `crates/agent-service`  | Local service orchestration and read-model/API boundaries depend on typed eventing/protocol contracts; verify through focused proof instead of historical closure wording.              |

## Not Implementation Proof

- A plan file.
- A pasted code sketch.
- A TypeScript eventing reference.
- A Rust trait without dispatch, queue, retry, timeout, dead-letter, journal,
  and test proof.
- A bus that only works for Parent-specific events.
- A bus whose tests use fake services or weak assertions.
