# Pasted Content Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `eventing-plan`
> Doc: `Pasted Content Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.
> Snippet rule: fenced blocks in this document are contract/artifact/command examples only. They are not instructions to copy implementation code unless the surrounding section explicitly says the snippet is the public contract shape.

<!-- /agent-capsule -->

This audit records consolidation of the two 2026-06-03 pasted Rust event bus
planning inputs. The pasted text is treated as source context, not copied as the
final architecture.

It also records the follow-up lineage comparison against the Unity/C# event bus
that preceded the TypeScript package.

Attachments:

```text
C:\Users\sujan\.codex\attachments\800513e4-7e64-4fa3-8835-4180f7ec8b82\pasted-text.txt
C:\Users\sujan\.codex\attachments\ebee5dc4-0786-4445-a1f3-bb9e1f42c557\pasted-text.txt
C:\Users\sujan\.codex\attachments\9776d552-57cc-4803-8278-74ab914f1146\pasted-text.txt
```

## Coverage Map

| Pasted requirement                                                                      | Covered by                                                                                                                                                                                                                   | Notes                                                                                                                                                                          |
| --------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Use Ocentra Games eventing-domain as reference, not direct copy                         | [source index](source-index.md), [full-scope plan](01-rust-eventing-full-scope-plan.md)                                                                                                                                      | Semantics are preserved; TypeScript implementation shape is not copied.                                                                                                        |
| Build Rust-first reusable eventing                                                      | [README](README.md), [full-scope plan](01-rust-eventing-full-scope-plan.md), [crate API](02-crate-api-and-code-shape.md)                                                                                                     | The plan targets `crates/ocentra-eventing` as generic infrastructure.                                                                                                          |
| Parent/controller and child-agent should both use the shared bus                        | [README](README.md), [full-scope plan](01-rust-eventing-full-scope-plan.md), [taxonomy](03-event-taxonomy-and-parent-integration.md), [workpacks](05-implementation-workpacks.md)                                            | The shared crate runs in both Rust runtimes with separate local bus instances.                                                                                                 |
| Vite/TypeScript UI must not own business logic                                          | [README](README.md), [full-scope plan](01-rust-eventing-full-scope-plan.md), [taxonomy](03-event-taxonomy-and-parent-integration.md), [tests](04-tests-proof-and-validation.md), [workpacks](05-implementation-workpacks.md) | UI can send typed intents and render read models only.                                                                                                                         |
| Two-layer live bus plus durable journal                                                 | [full-scope plan](01-rust-eventing-full-scope-plan.md), [crate API](02-crate-api-and-code-shape.md), [tests](04-tests-proof-and-validation.md)                                                                               | Live bus and NDJSON/replay proof are separate but related layers.                                                                                                              |
| EventBus, EventRegistrar, EventArgsBase, OperationResult, OperationDeferred equivalents | [source index](source-index.md), [crate API](02-crate-api-and-code-shape.md), [workpacks](05-implementation-workpacks.md)                                                                                                    | Rust equivalents are `EventBus`, `EventRegistrar`, `DomainEvent`, `EventEnvelope`, `PublishReport`, and local request completion.                                              |
| Unity/C# original bus ideas                                                             | [source index](source-index.md), [lineage safety](07-lineage-preservation-and-migration-safety.md), [tests](04-tests-proof-and-validation.md), [workpacks](05-implementation-workpacks.md)                                   | Central bus, sync/async split, force/republish, queue drain on subscribe, usage graph, registrar lifecycle, and operation/deferred ideas are preserved as Rust-safe semantics. |
| Async, parallel, sequential, retry, TTL, queueing, target handler                       | [full-scope plan](01-rust-eventing-full-scope-plan.md), [crate API](02-crate-api-and-code-shape.md), [tests](04-tests-proof-and-validation.md)                                                                               | Dispatch modes and queue/dead-letter behavior are explicit.                                                                                                                    |
| Rust Effect-Schema-like brands, validation, and shapes                                  | [type-safety guide](06-type-safety-validation-and-ownership.md), [crate API](02-crate-api-and-code-shape.md), [tests](04-tests-proof-and-validation.md)                                                                      | Raw values are boundary-only; live dispatch uses validated newtypes and typed events.                                                                                          |
| Borrow, mutation, ownership, and await safety                                           | [type-safety guide](06-type-safety-validation-and-ownership.md), [crate API](02-crate-api-and-code-shape.md), [workpacks](05-implementation-workpacks.md)                                                                    | Handlers cannot mutate event payloads; no lock-held await; mutable state belongs in services/actors.                                                                           |
| Network to AI to policy to enforcement chain                                            | [taxonomy](03-event-taxonomy-and-parent-integration.md), [tests](04-tests-proof-and-validation.md), [workpacks](05-implementation-workpacks.md)                                                                              | Network consumes the reusable bus after core proof.                                                                                                                            |
| No AI/UI/network direct enforcement                                                     | [full-scope plan](01-rust-eventing-full-scope-plan.md), [taxonomy](03-event-taxonomy-and-parent-integration.md), [tests](04-tests-proof-and-validation.md), [workpacks](05-implementation-workpacks.md)                      | Enforcement commands require policy decision refs and adapter proof.                                                                                                           |
| `ocentra-eventing` is local runtime infrastructure only                                 | [current snapshot](current-eventing-snapshot.md), [taxonomy](03-event-taxonomy-and-parent-integration.md), [workpacks](05-implementation-workpacks.md)                                                                       | Household mesh, LAN, relay, and parent/child transport are consumer-layer bridges, not generic crate behavior.                                                                 |
| Household Mesh Bridge converts selected events to typed LAN messages                    | [current snapshot](current-eventing-snapshot.md), [taxonomy](03-event-taxonomy-and-parent-integration.md), AI [mesh plan](../ai-plan/household-ai-provider-mesh-plan.md)                                                     | Incoming LAN messages authenticate/authorize/validate before local republish; remote peers never publish directly into another runtime bus.                                    |
| AI provider mesh event taxonomy                                                         | [taxonomy](03-event-taxonomy-and-parent-integration.md), AI [mesh plan](../ai-plan/household-ai-provider-mesh-plan.md)                                                                                                       | Adds `ai.provider.*`, `ai.work.*`, `ai.work.claim.*`, `ai.work.lease.*`, and `ai.result.*` families.                                                                           |
| Detailed Rust starter code direction                                                    | [crate API](02-crate-api-and-code-shape.md)                                                                                                                                                                                  | The pasted starter code was strengthened into module/API/workpack requirements instead of being pasted as final source.                                                        |
| Solid tests and proof                                                                   | [tests](04-tests-proof-and-validation.md), [workpacks](05-implementation-workpacks.md)                                                                                                                                       | Tests require real Tokio/serde/tempfs behavior and exact assertions.                                                                                                           |

## Final Plan Adjustments Beyond The Paste

- The reusable crate is shared by Rust parent/controller and child-agent
  runtimes.
- Cross-process parent-to-child routing is a typed transport or journal/replay
  boundary, not shared in-memory state.
- The Vite/TypeScript portal surface is explicitly view/input only.
- Parent-specific event constants stay out of the generic crate.
- No test doubles are allowed in the eventing proof plan.
- Network Workpack 10 is a consumer of this plan, not a place to invent another
  bus.
- Household AI Provider Mesh is a consumer of this plan, not a reason to turn
  the generic eventing crate into a LAN broker, AI scheduler, policy engine, or
  enforcement runtime.
- Rust type safety is made explicit through validated newtypes, typed live
  envelopes, associated request responses, mutation guards, and lock-held-await
  proof.
- Unity/C# and TypeScript lineage is now protected through a conformance
  matrix, event topology proof, constrained force/republish policy, manual
  clock, shutdown lifecycle, and no payload-carried handle/source gates.
