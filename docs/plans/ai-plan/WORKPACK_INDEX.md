# AI Plan Workpack Index

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Workpack Index`
> Kind: workpack selector; use before opening any workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Use this index to open exactly one assigned workpack. Do not read every file in `workpacks/`.

## Code-first audit overlay

The status column below is checkbox/document state. It is not implementation
truth. Use [CODE_AUDIT.md](CODE_AUDIT.md) for the 2026-08-15 source/test review:
all 48 workpacks are graph-mapped; 11 are Phase 1 complete for bounded code/test
scope and 37 retain concrete code or expected-test gaps.

| Status  | Workpack                                                                                                                     |  Size | Boxes                 |
| ------- | ---------------------------------------------------------------------------------------------------------------------------- | ----: | --------------------- |
| checked | [01 - Source Index And Repo Reconciliation](workpacks/01-source-index-and-repo-reconciliation.md)                            | 2,049 | 5/5 checked; 0 open   |
| checked | [02 - Current AI Snapshot And Gap Map](workpacks/02-current-ai-snapshot-and-gap-map.md)                                      |   818 | 5/5 checked; 0 open   |
| open    | [03 - Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md)                            |   928 | 0/6 checked; 6 open   |
| open    | [04 - Rust Protocol Parity For AI Contracts](workpacks/04-rust-protocol-parity-for-ai-contracts.md)                          |   821 | 0/5 checked; 5 open   |
| open    | [05 - LocalModelRuntimeStatus Hardening](workpacks/05-local-model-runtime-status-hardening.md)                               |   822 | 0/5 checked; 5 open   |
| open    | [06 - LocalProviderCapability Hardening](workpacks/06-local-provider-capability-hardening.md)                                | 1,259 | 0/5 checked; 5 open   |
| open    | [07 - AI Job Queue Contract](workpacks/07-ai-job-queue-contract.md)                                                          | 1,633 | 1/11 checked; 10 open |
| open    | [08 - AI Provider Routing Contract](workpacks/08-ai-provider-routing-contract.md)                                            | 1,393 | 0/10 checked; 10 open |
| open    | [09 - Local Evidence Context Builder V1](workpacks/09-local-evidence-context-builder-v1.md)                                  |   751 | 0/6 checked; 6 open   |
| open    | [10 - Evidence Reference Normalization](workpacks/10-evidence-reference-normalization.md)                                    |   760 | 0/5 checked; 5 open   |
| open    | [11 - Parent Rule Context Builder](workpacks/11-parent-rule-context-builder.md)                                              |   706 | 0/5 checked; 5 open   |
| open    | [12 - Prompt Template Version Registry](workpacks/12-prompt-template-version-registry.md)                                    |   628 | 0/5 checked; 5 open   |
| open    | [13 - Deterministic No-Model Classifier Lane](workpacks/13-deterministic-no-model-classifier-lane.md)                        |   720 | 0/6 checked; 6 open   |
| open    | [14 - Local Text LLM Adapter Boundary](workpacks/14-local-text-llm-adapter-boundary.md)                                      |   773 | 0/6 checked; 6 open   |
| open    | [15 - Local Text LLM Execution Dry-Run Adapter](workpacks/15-local-text-llm-execution-dry-run-adapter.md)                    |   743 | 0/6 checked; 6 open   |
| open    | [16 - Output Parser And Schema Validator](workpacks/16-output-parser-and-schema-validator.md)                                |   663 | 0/6 checked; 6 open   |
| open    | [17 - Degraded Timeout Invalid-Output Handling](workpacks/17-degraded-timeout-invalid-output-handling.md)                    |   665 | 0/6 checked; 6 open   |
| open    | [18 - Deterministic Policy Evaluator Integration](workpacks/18-deterministic-policy-evaluator-integration.md)                |   761 | 0/5 checked; 5 open   |
| open    | [19 - AI Result Journal SQLite Ingest](workpacks/19-ai-result-journal-sqlite-ingest.md)                                      |   663 | 0/5 checked; 5 open   |
| open    | [20 - Parent Explanation Read Model](workpacks/20-parent-explanation-read-model.md)                                          |   771 | 0/6 checked; 6 open   |
| open    | [21 - Memory Reference Contract](workpacks/21-memory-reference-contract.md)                                                  |   676 | 0/5 checked; 5 open   |
| open    | [22 - Short-Window Recent Activity Memory](workpacks/22-short-window-recent-activity-memory.md)                              |   707 | 0/5 checked; 5 open   |
| open    | [23 - Evidence-Backed Semantic Memory](workpacks/23-evidence-backed-semantic-memory.md)                                      |   684 | 0/6 checked; 6 open   |
| open    | [24 - Knowledge Graph Reference Contract](workpacks/24-knowledge-graph-reference-contract.md)                                |   672 | 0/5 checked; 5 open   |
| open    | [25 - Minimal Graph Edges For Safety Context](workpacks/25-minimal-graph-edges-for-safety-context.md)                        |   691 | 0/6 checked; 6 open   |
| open    | [26 - TabAgent Code Audit And Reuse Map](workpacks/26-tabagent-code-audit-and-reuse-map.md)                                  | 2,264 | 6/7 checked; 1 open   |
| open    | [27 - TabAgent Native Bridge Reuse Candidate](workpacks/27-tabagent-native-bridge-reuse-candidate.md)                        |   779 | 0/5 checked; 5 open   |
| open    | [28 - TabAgent Model Lifecycle Cache Reuse Candidate](workpacks/28-tabagent-model-lifecycle-cache-reuse-candidate.md)        |   772 | 0/5 checked; 5 open   |
| open    | [29 - TabAgent Memory Graph Reuse Candidate](workpacks/29-tabagent-memory-graph-reuse-candidate.md)                          |   647 | 0/5 checked; 5 open   |
| open    | [30 - OCR Worker Lane](workpacks/30-ocr-worker-lane.md)                                                                      | 1,122 | 7/9 checked; 2 open   |
| open    | [31 - Guided VLM Worker Lane](workpacks/31-guided-vlm-worker-lane.md)                                                        |   880 | 0/8 checked; 8 open   |
| open    | [32 - Household AI Provider Mesh And Remote Assistant Boundary](workpacks/32-family-ai-hub-and-remote-assistant-boundary.md) | 1,542 | 0/11 checked; 11 open |
| open    | [33 - Browser URL Video AI Lane](workpacks/33-browser-url-video-ai-lane.md)                                                  |   770 | 0/6 checked; 6 open   |
| open    | [34 - Browser Social Feed Signup AI Lane](workpacks/34-browser-social-feed-signup-ai-lane.md)                                |   737 | 0/5 checked; 5 open   |
| open    | [35 - Browser Game Cloud Game AI Lane](workpacks/35-browser-game-cloud-game-ai-lane.md)                                      |   784 | 0/6 checked; 6 open   |
| open    | [36 - App Game Unknown Classifier Lane](workpacks/36-app-game-unknown-classifier-lane.md)                                    |   762 | 0/6 checked; 6 open   |
| open    | [37 - Tracking Location Safety Analysis Lane](workpacks/37-tracking-location-safety-analysis-lane.md)                        |   762 | 0/6 checked; 6 open   |
| open    | [38 - Screen OCR VLM Router Lane](workpacks/38-screen-ocr-vlm-router-lane.md)                                                |   963 | 0/9 checked; 9 open   |
| open    | [39 - Device Hardware Model Fit Lane](workpacks/39-device-hardware-model-fit-lane.md)                                        |   650 | 0/5 checked; 5 open   |
| open    | [40 - Model Catalog Artifact Integrity Lane](workpacks/40-model-catalog-artifact-integrity-lane.md)                          |   658 | 0/5 checked; 5 open   |
| open    | [41 - Llama GGUF Runtime Packaging Lane](workpacks/41-llama-gguf-runtime-packaging-lane.md)                                  |   667 | 0/6 checked; 6 open   |
| open    | [42 - Inference Settings Template Governance Lane](workpacks/42-inference-settings-template-governance-lane.md)              |   601 | 0/5 checked; 5 open   |
| open    | [43 - AI Activity Portal Surface Lane](workpacks/43-ai-activity-portal-surface-lane.md)                                      |   695 | 0/6 checked; 6 open   |
| open    | [44 - Provider API Authorization Custody Lane](workpacks/44-provider-api-authorization-custody-lane.md)                      |   663 | 0/5 checked; 5 open   |
| open    | [45 - Remote Redacted Report Assistant Lane](workpacks/45-remote-redacted-report-assistant-lane.md)                          |   658 | 0/5 checked; 5 open   |
| open    | [46 - Security Privacy Negative Gates Lane](workpacks/46-security-privacy-negative-gates-lane.md)                            |   759 | 0/8 checked; 8 open   |
| open    | [47 - Performance Resource Battery Proof Lane](workpacks/47-performance-resource-battery-proof-lane.md)                      | 1,026 | 0/8 checked; 8 open   |
| open    | [48 - Rollout Checklist And PR Gate](workpacks/48-rollout-checklist-and-pr-gate.md)                                          | 1,933 | 0/25 checked; 25 open |

## Workpack families and owner paths

Use this section to classify the assigned workpack before opening source. It is a routing aid, not permission to read every workpack in the family.

```text
Current state and route hygiene:
  01 source index and reconciliation
  02 current AI snapshot and gap map
  48 rollout checklist and PR gate
  Owners: docs/plans/ai-plan plus named proof roots. No source edits unless the workpack names them.

Canonical contract/schema family:
  03 contract boundary and Effect schemas
  04 Rust protocol parity for AI contracts
  05 LocalModelRuntimeStatus hardening
  06 LocalProviderCapability hardening
  10 evidence reference normalization
  12 prompt template version registry
  16 output parser and schema validator
  21 memory reference contract
  24 knowledge graph reference contract
  Owners: packages/schema-domain first; crates/agent-protocol or child-ai-core only when Rust/wire parity is assigned.

Runtime/provider/job family:
  07 AI job queue contract
  08 AI provider routing contract
  14 local text LLM adapter boundary
  15 local text LLM execution dry-run adapter
  17 degraded timeout invalid-output handling
  31 guided VLM worker lane
  32 household AI provider mesh and remote assistant boundary
  39 device hardware model fit lane
  40 model catalog artifact integrity lane
  41 Llama GGUF runtime packaging lane
  42 inference settings template governance lane
  Owners: child-ai-core for child-local runtime; schema-domain for shared job/provider contracts; LAN/remote only through explicit provider-job handoff, not direct runtime import.

Evidence context and memory family:
  09 local evidence context builder V1
  11 parent rule context builder
  19 AI result journal SQLite ingest
  20 parent explanation read model
  22 short-window recent activity memory
  23 evidence-backed semantic memory
  25 minimal graph edges for safety context
  29 TabAgent memory graph reuse candidate
  Owners: schema-domain for context/reference shapes; child-ai-core for local evaluation/read-model behavior; storage/eventing/evidence primitives only through neutral boundaries.

TabAgent reuse family:
  26 TabAgent code audit and reuse map
  27 TabAgent native bridge reuse candidate
  28 TabAgent model lifecycle cache reuse candidate
  29 TabAgent memory graph reuse candidate
  Owners: docs and selected adapter boundary only. TabAgent must not redefine Ocentra Parent portal, policy, evidence, or child-agent authority.

Feature-evidence bridge family:
  30 OCR worker lane
  33 browser URL video AI lane
  34 browser social feed signup AI lane
  35 browser game cloud game AI lane
  36 app game unknown classifier lane
  37 tracking location safety analysis lane
  38 screen OCR VLM router lane
  Owners: evidence-producing plans own capture/runtime; AI consumes evidence/read models/requests and emits validated AI results. No direct imports from browser/screen/tracking/network/app-game runtime internals.

Policy handoff and portal explanation family:
  13 deterministic no-model classifier lane
  18 deterministic policy evaluator integration
  20 parent explanation read model
  43 AI activity portal surface lane
  45 remote redacted report assistant lane
  Owners: AI emits schema-valid evidence/classification/explanation; policy/enforcement own deterministic decisions/actions; portal renders status/explanation only.

Security, privacy, custody, and performance gates:
  44 provider API authorization custody lane
  46 security privacy negative gates lane
  47 performance resource battery proof lane
  48 rollout checklist and PR gate
  Owners: selected workpack proof root plus strict no-claim boundaries. Remote/API paths remain opt-in and outside the normal child safety blocking path.
```

If the workpack belongs to multiple families, use the strictest test/proof path. When a feature bridge needs source facts, consume the owning feature's evidence/read-model/request result; do not import or call its runtime implementation.
