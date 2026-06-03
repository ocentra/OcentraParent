# Pasted Content Coverage Audit

This audit maps the pasted AI planning requirements into this folder.

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

## Added Beyond Pasted Text

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
  and rollout gates.
