# TabAgent Source Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `TabAgent Source Index`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

TabAgent and TabAgentServer are inspected reference systems only. They do not
define Ocentra Parent contracts, policy, custody, storage, or UI behavior.

## Local Reference Root

`E:\Desktop\TabAgent`

## Inspected Files

| Area                       | Files                                                                                            | What To Study                                                                                                                   | Ocentra Boundary                                                                   |
| -------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| Native bridge              | `src\Controllers\NativeHostManager.ts`, `src\types\native.ts`                                    | Persistent native host connection, message ids, queueing, reconnect events, request/response envelopes, native action types     | Ocentra local service command protocol and route status contracts                  |
| Browser-side model manager | `src\backgroundModelManager.ts`                                                                  | Transformer environment setup, model state, custom fetch/cache path, progress events, generation state, unload/restore behavior | Ocentra local model runtime and provider lifecycle, not browser-owned safety logic |
| Native model service       | `src\Controllers\services\NativeModelService.ts`                                                 | Pull/list/load/unload model lifecycle and response handling                                                                     | Ocentra model artifact registry and runtime status                                 |
| Native inference service   | `src\Controllers\services\NativeInferenceService.ts`                                             | Prompt/generation request boundary and stop/halt patterns                                                                       | Ocentra local inference adapter with typed context and typed result parsing        |
| IndexedDB model cache      | `src\DB\idbModel.ts`                                                                             | Manifest entries, quant status, chunked cache, cache stats, model file rewriting                                                | Ocentra model cache only, never evidence storage                                   |
| IndexedDB knowledge graph  | `src\DB\idbKnowledgeGraph.ts`                                                                    | Graph node/edge/index patterns and local persistence                                                                            | Ocentra source-cited memory/graph references                                       |
| Rust native router         | `TabAgentServer\Rust\native-messaging\src\router.rs`, `protocol.rs`                              | Route metadata, route registration, dispatch, request id handling, parse/write protocol                                         | Ocentra Rust protocol parity and service route dispatcher                          |
| Rust model routes          | `TabAgentServer\Rust\native-messaging\src\routes\models.rs`, `generation.rs`                     | Model list/load/generation route shapes and test structure                                                                      | Ocentra provider routing and generation payload contracts                          |
| Rust model cache           | `TabAgentServer\Rust\model-cache\src\lib.rs`                                                     | Minimal model cache crate boundary                                                                                              | Ocentra model artifact/cache crate or module                                       |
| Rust execution providers   | `TabAgentServer\Rust\execution-providers\src\lib.rs`                                             | Provider enum, availability, execution provider selection, hardware adapter shape                                               | Ocentra provider capability and resource scheduler                                 |
| Rust graph/storage         | `TabAgentServer\Rust\knowledge-graph\src\lib.rs`, `TabAgentServer\Rust\storage\src\knowledge.rs` | Graph memory edges, storage API, query patterns                                                                                 | Ocentra graph references with source evidence requirements                         |

## Local File Size Snapshot

| File                                                            | Lines |
| --------------------------------------------------------------- | ----: |
| `src\Controllers\NativeHostManager.ts`                          |   340 |
| `src\types\native.ts`                                           |   177 |
| `src\backgroundModelManager.ts`                                 |  1351 |
| `src\Controllers\services\NativeModelService.ts`                |   166 |
| `src\Controllers\services\NativeInferenceService.ts`            |   150 |
| `src\DB\idbModel.ts`                                            |  1427 |
| `src\DB\idbKnowledgeGraph.ts`                                   |   528 |
| `TabAgentServer\Rust\native-messaging\src\router.rs`            |   366 |
| `TabAgentServer\Rust\native-messaging\src\protocol.rs`          |   319 |
| `TabAgentServer\Rust\native-messaging\src\routes\models.rs`     |   527 |
| `TabAgentServer\Rust\native-messaging\src\routes\generation.rs` |   178 |
| `TabAgentServer\Rust\model-cache\src\lib.rs`                    |    27 |
| `TabAgentServer\Rust\execution-providers\src\lib.rs`            |   236 |
| `TabAgentServer\Rust\knowledge-graph\src\lib.rs`                |   679 |
| `TabAgentServer\Rust\storage\src\knowledge.rs`                  |    57 |

## Reuse Decisions

- Reuse lessons, not raw product meaning.
- Translate native bridge ideas into Ocentra-owned typed commands, responses,
  route ids, queue state, and degraded-state contracts.
- Translate model lifecycle ideas into local runtime status, artifact registry,
  model cache, progress, and provider capability contracts.
- Translate graph ideas into source-cited local derived references. A graph edge
  cannot become household truth unless it cites stored evidence, policy version,
  or parent action.
- Keep TabAgent browser behavior separate from child-device safety policy.
- Keep TabAgent model cache separate from Ocentra evidence journal and SQLite
  query store.
- Do not copy TabAgent string ids, UI, persona, remote behavior, or broad agent
  workflows into Ocentra Parent.

## Audit Checklist

- [ ] Confirm each candidate file is still present before extracting logic.
- [ ] Map every reused idea to an Ocentra TypeScript contract first.
- [ ] Add Rust parity before Rust runtime consumes extracted TabAgent contract
      logic.
- [ ] Prove local route status, unavailable, timeout, and invalid payload cases
      for any extracted TabAgent route logic.
- [ ] Prove model cache corruption never deletes or corrupts evidence.
- [ ] Prove memory/graph refs without source evidence are rejected.
- [ ] Document every copied/extracted module with source, license, changed
      ownership, and deleted unused behavior.

Current reconciliation note: the candidate file list was rechecked against
`E:\Desktop\TabAgent` on the unified screen-AI branch and all indexed files are
present. Ocentra has translated the reference ideas into local AI runtime,
provider scheduler, model artifact manifest, memory/graph, context-builder,
remote boundary, and screen mesh proof contracts. No TabAgent code has been
copied or extracted in this branch, so copied-module license/ownership notes
plus extraction-specific Rust parity and route proofs remain open until an
actual extraction happens.
