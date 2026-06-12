# Pasted Content Coverage Audit

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `Pasted Content Coverage Audit`
> Kind: plan reference document; read only when routed by AGENTS, DOC_INDEX, or workpack.
> Read when: Only when named by the plan route, selected workpack, or index row.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

This audit maps pasted AI planning requirements into this folder. The
2026-06-06 planning update reconciles the older "family AI hub" wording into
the event-driven Household AI Provider Mesh model: `ocentra-eventing` is local
runtime infrastructure only, cross-device coordination belongs to a Household
Mesh Bridge, providers are worker-only, and the evidence-owning child agent
keeps evidence truth, result validation, policy authority, enforcement handoff,
audit, and read models.

| Pasted requirement                                 | Covered by                                                                 |
| -------------------------------------------------- | -------------------------------------------------------------------------- |
| Create `docs/plans/ai-plan`                        | This folder                                                                |
| Make README                                        | `README.md`                                                                |
| Make source index                                  | `source-index.md`                                                          |
| Make current snapshot                              | `current-ai-snapshot.md`                                                   |
| V0.6 contracts plan                                | `v0-6-local-ai-contracts-plan.md`, workpacks 03-04                         |
| V0.7 runtime/dry-run plan                          | `v0-7-local-ai-runtime-and-dry-run-plan.md`, workpacks 05, 14-19           |
| V0.7 context builder plan                          | `v0-7-ai-context-builder-plan.md`, workpacks 09-12                         |
| V0.7 model routing/queue plan                      | `v0-7-ai-model-routing-and-queue-plan.md`, workpacks 07-08                 |
| Household AI provider mesh                         | `household-ai-provider-mesh-plan.md`, workpacks 07, 08, 32, 47, 48         |
| V0.7 memory graph plan                             | `v0-7-ai-memory-graph-plan.md`, workpacks 21-25                            |
| V0.7 TabAgent reuse plan                           | `v0-7-tabagent-reuse-plan.md`, `tabagent-source-index.md`, workpacks 26-29 |
| V0.8 policy enforcement handoff                    | `v0-8-policy-enforcement-handoff-plan.md`, workpack 18                     |
| V1 screen OCR/VLM                                  | `v1-screen-ocr-vlm-plan.md`, workpacks 30-31, 38                           |
| V4 remote parent assistant                         | `v4-remote-parent-assistant-plan.md`, workpacks 32, 44-45                  |
| Test blueprint                                     | `v0-7-ai-test-blueprint.md`                                                |
| UI/UX guide                                        | `ui-ux-requirements-guide.md`                                              |
| Implementation checklist                           | `implementation-checklist.md`                                              |
| Workpacks                                          | `workpacks/01` through `workpacks/48`                                      |
| AI local child-device safety first                 | `README.md`, `current-ai-snapshot.md`                                      |
| AI execution may use trusted household providers   | `README.md`, `household-ai-provider-mesh-plan.md`, model routing plan      |
| Child agent retains policy/action authority        | `README.md`, `household-ai-provider-mesh-plan.md`, workpack 32             |
| Event bus is local, not LAN-wide                   | `household-ai-provider-mesh-plan.md`, eventing plan snapshot/taxonomy      |
| Mesh bridge owns selected LAN export/import        | `household-ai-provider-mesh-plan.md`, eventing plan snapshot/taxonomy      |
| AI consumes typed evidence, rules, context, memory | `README.md`, context-builder and memory docs                               |
| AI does not scan directly                          | `README.md`, `v0-7-ai-test-blueprint.md`, workpack 46                      |
| AI output is evidence, not authority               | `README.md`, policy handoff plan                                           |
| Parent policy decides actions                      | `README.md`, policy handoff plan                                           |
| Enforcement consumes policy only                   | `v0-8-policy-enforcement-handoff-plan.md`                                  |
| Remote/API disabled for normal child safety        | `README.md`, remote plan                                                   |
| TabAgent reuse behind Ocentra contracts            | `tabagent-source-index.md`, TabAgent reuse plan                            |
| Gemma/local text lane guidance                     | `model-and-runtime-candidate-strategy.md`, workpacks 14-15                 |
| OCR/VLM split                                      | `v1-screen-ocr-vlm-plan.md`, workpacks 30-31                               |
| Browser AI                                         | workpacks 33-35                                                            |
| App/game AI                                        | workpack 36                                                                |
| Tracking AI                                        | workpack 37                                                                |
| Screen AI                                          | workpacks 30-31, 38                                                        |
| Proof/validation stronger than other plans         | `v0-7-ai-test-blueprint.md`, `proof-pack-template.md`, workpacks 46-48     |
| Real capture-driven AI analysis proof              | `real-ai-analysis-and-pipeline-proof-matrix.md`, workpacks 30, 31, 38, 48  |
| Separate final pipeline pass after screen and AI   | `../screen-ai-pipeline-plan/README.md`                                     |
| Mesh-aware screen AI pipeline variant              | `../screen-ai-pipeline-plan/README.md`, pipeline proof matrix              |

## Added Beyond Pasted Text

- `household-ai-provider-mesh-plan.md` so same-device local AI, trusted
  household provider execution, dormant mobile fallback, and parent-approved
  remote assistant are separate roles instead of one vague hub.
- `tabagent-source-index.md` with actual local TabAgent file sizes and reuse
  decisions.
- `model-and-runtime-candidate-strategy.md` so model choice, current local text
  lane, OCR, VLM, embeddings, and remote assistant are not mixed.
- `proof-pack-template.md` so each AI implementation slice reports exact proof.
- `real-ai-analysis-and-pipeline-proof-matrix.md` so AI PR-ready requires real
  browser/app/timed-cadence analysis artifacts when screen-derived AI is in
  scope.
- `../screen-ai-pipeline-plan/README.md` so final capture plus analysis plus
  policy/action proof is a required post-screen/post-AI pass, not hidden inside
  either separate PR.
- Workpacks 39-48 for hardware/model fit, model integrity, llama/GGUF runtime,
  inference/template governance, UI/activity, provider/API custody, remote
  report assistant, security/privacy negative gates, performance/resource proof,
  and rollout gates, including mesh topology, claim/lease, child-agent
  authority, and no-raw-screen-transfer proof gates.
