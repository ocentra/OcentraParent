# AI Plan State

<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan State`
> Kind: current state and open gaps.
> Read when: Immediately after plan AGENTS.md; use for current state and no-claim boundaries.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

Generated from the existing `ai-plan` docs. This is the default resume/status file; large historical docs are linked, not embedded.

## Scope

This folder is the single working plan location for local AI safety contracts, local model runtime, evidence context building, provider routing, AI job queues, memory and knowledge graph, TabAgent reuse, screen OCR/VLM routing, policy handoff, parent explanations, and later parent-approved remote assistant boundaries.

## WP03 source integration checkpoint — 2026-08-25

The Rust-owned WP03 source packet is integrated in the canonical consolidation
through `f9225e24a` (source packet branch
`origin/codex/ai-wp03-contract-repair5-aug25` at `9bc7574a`). The packet covers
the `crates/schema` AI contract family, its exporter, and the generated
`packages/schema-domain` edge surface. This is source evidence only: no general
production caller was found, and the three expected contract test roots remain
absent:

- `crates/schema/tests/contract/ai_contracts.rs`
- `crates/schema/tests/contract/ai_contracts_negative.rs`
- `packages/schema-domain/tests/contract/ai-contracts.test.ts`

Focused source review, tests, proof, CI, and DONE/READY state were not changed
by this checkpoint. Write the complete test family and establish a real caller
before treating WP03 as a closed implementation slice.

## Code-first Phase 1 audit (2026-08-15)

- Authoritative audit: [CODE_AUDIT.md](CODE_AUDIT.md).
- All 48 workpacks now have reviewed code/test topology in the engineering graph.
- 11 workpacks are Phase 1 complete for their bounded source/test scope: 01, 02,
  05, 06, 08, 10, 11, 24, 26, 32, and 37.
- 37 workpacks retain a production-code or expected-test gap. The workpack
  checkbox summary below is plan-document state, not this implementation result.
- Phase 2 focused test/Enforcer execution and Phase 3 proof regeneration were
  not run as part of this audit.

## Production reachability audit (2026-08-16)

This is a source/caller audit, not a completion promotion. A mapped source
file, checked row, focused test, or historical proof is not production
completion unless a shipped entrypoint reaches it with trusted input and a
material effect. The graph currently maps implementation roots, but it does
not establish that reachability or the completion contract for any workpack.

| WP | Reachable production code and effect | Remaining production gap |
| --- | --- | --- |
| 01 | No AI runtime entrypoint; plan/specification only. | No shipped local-first AI boundary to audit. |
| 02 | No AI runtime entrypoint; snapshot/specification only. | No shipped custody/runtime source. |
| 03 | `schema_domain_ai_wire` is self/test-referenced; no general AI caller. | One canonical cross-boundary contract family is missing for work items, results, journal, explanation, prompt, and remote state. |
| 04 | `agent-protocol` runtime/status and assistant shapes feed typed service/websocket reports. | Generated TS parity and complete negative decoder family are missing; source presence is not parity proof. |
| 05 | Local runtime status flows through the AI websocket command and portal status card. | No verified artifact/model readiness and execution authority beyond typed status. |
| 06 | Provider route metadata is selected by LAN AI routing; service exposes a typed provider-status read model. | No physical or multi-service provider authority is present. |
| 07 | Scheduler/queue is called by local chat, parent assistant, and screen analysis. | No neutral durable `AiWorkItem` lifecycle, replay, or dead-letter journal. |
| 08 | LAN AI route metadata and lease/status paths are reachable. | No physical household mesh discovery, lease, or multi-device runtime proof. |
| 09 | Parent-assistant runtime builds evidence context from command input and feeds local generation. | No general SQLite/read-model context builder carrying all custody states. |
| 10 | Typed activity/memory references are consumed by graph and assistant paths. | No general normalizer and edge validation for the full context contract. |
| 11 | `ActivityStore` parent-rule context feeds deterministic policy preview. | Generic AI context is not integrated into that boundary. |
| 12 | Prompt constants are consumed by parent-assistant runtime. | No task-keyed registry, deprecation policy, or fixture family. |
| 13 | Browser/app-game/tracking owners run deterministic classifiers. | No unified ambiguous-result escalation into a general AI route. |
| 14 | Websocket local chat/assistant reaches the configured llama.cpp process runner. | Runner accepts a raw prompt string; no validated context-only input boundary. |
| 15 | The same service path can execute configured local generation with typed timeout/empty/failure results. | No end-to-end validated context, output parser, journal, and enforcement-disabled flow. |
| 16 | Screen analysis invokes adapter parsing and redaction functions. | No general AI result validator, evidence-reference, or contradiction boundary. |
| 17 | Scheduler/runner returns typed timeout, unavailable, busy, and invalid-output states. | No durable failure journal or integrated fallback read model. |
| 18 | ActivityStore policy preview invokes the typed local-AI evaluator with parent-rule precedence. | No durable AI-result and policy-decision linkage; preview is not enforcement. |
| 19 | Local generation and screen analysis emit typed service events. | No canonical AI result journal with SQLite ingest, read, and replay. |
| 20 | Parent-assistant websocket reaches local answers/preview output and portal projection. | No unified explanation read model linking evidence, rules, model refs, and degraded state. |
| 21 | Protocol memory-reference types are used by AI/activity shapes. | No general consumer enforcing expiry, invalidation, or unsourced fail-closed behavior. |
| 22 | ActivityStore graph/history APIs produce an activity read model. | No bounded memory contract feeding validated AI context. |
| 23 | Semantic enums exist in protocol only. | No semantic index, embedding, rebuild, or invalidation caller. |
| 24 | Typed graph contracts feed the activity graph/read-model path. | Required focused validation is deferred; no new runtime gap was closed here. |
| 25 | ActivityStore graph builder/index/query code is reachable and materializes graph read models. | AI, policy, result, and action edges are absent. |
| 26 | TabAgent mapping/documentation has no shipped runtime caller. | No legal extraction/runtime reuse slice exists yet. |
| 27 | Local scheduler/runtime boundary is used by service scheduling. | No native transport reuse, reconnect, or invalid-payload boundary. |
| 28 | Model registry and runtime-install-plan types are read by configuration/distribution code. | No verified downloader, corruption handling, or resumable cache owner. |
| 29 | Activity graph index/query paths are reachable. | Expiry, invalidation, and journal-rebuild semantics are incomplete. |
| 30 | Env-gated screen runtime spawns queue/capture/adapter processing when configured. | No OCR engine or production OCR worker. |
| 31 | `screen-ai-core` router/logic has no production caller outside its own modules/tests. | Missing shipped guided-VLM worker and result owner. |
| 32 | LAN provider route, job lease, and child validation paths are reachable. | No physical multi-device/remote execution authority. |
| 33 | Browser URL/video code performs deterministic classification. | Ambiguous evidence is not routed into the AI spine. |
| 34 | Browser social/feed contracts and deterministic URL logic exist. | No complete target/result path joining fallback, confidence, and degraded state. |
| 35 | Browser-game compiler/evaluator is reachable for deterministic policy compilation. | No cloud ambiguity and approved AI fallback integration. |
| 36 | App/game classification and activity read models are reachable. | Unknown classification is not routed to an AI result path. |
| 37 | Tracking AI boundary consumes cited evidence/context and hands off deterministic results. | No complete child AI result and policy route beyond that boundary. |
| 38 | Screen runtime/queue/capture path is reachable when configured. | Screen-intelligence router has no caller; no OCR/VLM fallback owner. |
| 39 | Resource/acceleration configuration is used in runtime arguments/status. | No real hardware measurement, fit engine, or load/backpressure authority. |
| 40 | Registry/cache contracts are consumed by runtime configuration. | No downloader, integrity verifier, quarantine, or durable cache owner. |
| 41 | Runtime distribution/runner/config code is reachable. | No install, repair, or uninstall pipeline for model/runtime artifacts. |
| 42 | Generation config and parent-assistant runtime are reachable. | No per-task settings registry or settings-to-result linkage. |
| 43 | Portal AI cards render runtime/job/memory/remote/degraded projections. | Presentation has no unified policy/explanation/history production read model. |
| 44 | Parent-assistant API boundary is reachable and keeps remote paths disabled/degraded. | No trusted one-shot parent action consumer; projection/status is not action authority. |
| 45 | Local parent-assistant context/runtime path is reachable. | No remote adapter, approval, redacted bundle transport, or response validator. |
| 46 | Source rejection guards exist at agent/screen/tracking boundaries. | No complete integrated negative-gate owner across the AI route. |
| 47 | Scheduler/queue/status paths are reachable for local work. | No hardware-fit, backpressure, or complete mobile/school fallback behavior. |
| 48 | Rollout documentation only; no independent runtime entrypoint. | Rollout cannot begin while the preceding runtime gaps remain. |

### Reachability and topology corrections

- The current graph report has 48 AI rows: WP01, WP02, WP26, and WP48 are
  `no-source`/`no-code-required`; the remaining rows have mapped
  `code-and-tests` roots, but all completion contracts still lack reviewed
  implementation, test, proof, and checklist evidence in the report.
- `CODE_AUDIT.md`'s 11 Phase 1 bounded source/test rows and 37 incomplete rows
  describe source/test topology, not shipped runtime completion. Historical
  checked/open workpack labels retain that same limitation.
- `packages/ai-domain` does not exist; historical ownership text pointing to
  it is stale. Cross-boundary AI contracts remain Rust-owned/generated through
  the selected canonical boundary.
- `crates/agent-protocol/src/schema_domain_ai_wire.rs` has no production caller
  beyond its own module/tests. Its graph mapping must not be read as a shipped
  general AI contract.

No source slice is legally unblocked by this audit. The smallest future
production slices are, in dependency order, a canonical WP03 contract plus a
real consumer, WP07's durable lifecycle owner, WP09's custody-aware context
builder, the WP14/WP15 validated-context boundary, and WP19's durable result
journal. A generic bridge or DTO-only adapter would be unreachable scaffolding.

## Current ownership interpretation

```text
crates/schema or the owning Rust crate:
  Canonical shared AI contracts when AI shapes cross package, crate, app, or plan boundaries.

schema-domain:
  Narrow generated-validation/edge-decoder compatibility surface only. It does not currently own the general AI contract family.

packages/ai-domain:
  Does not exist. Do not route work to this historical/aspirational owner.

child-ai-core:
  Child-local AI runtime/evaluator boundary for context validation, provider result validation, degraded states, and accepted AI output.

screen-ai-core:
  Screen AI worker/router boundary. It consumes screen evidence references and does not own general screen capture, retention, or policy.

agent-protocol and agent-service:
  Wire/service transport boundaries when a selected workpack names protocol or service proof.

portal-domain and apps/portal:
  Parent-visible AI status/explanation projections. They do not own child-local evaluation or policy actions.

Browser, screen, tracking, network, and app/game plans:
  Evidence/source owners. AI consumes their evidence/read-model/request results and must not import their runtime behavior.

Policy and enforcement plans:
  Deterministic decision/action owners. AI output is evidence input, not authority to act.
```

## Current coupling risks

```text
- Historical `ai-domain` ownership text is stale because `packages/ai-domain` does not exist. Shared shapes must remain Rust-owned/generated through the selected canonical boundary.
- `child-ai-core` currently depends on tracking-core. Treat it as migration debt unless the selected workpack records a temporary compatibility reason and a replacement event/read-model route.
- Bridge work for browser, screen, tracking, network, or app/game must use evidence/read-model/request results instead of direct runtime calls.
```

## Resume route

1. Read this file.
2. Read `NEXT_ACTIONS.md` when starting/resuming.
3. Read `WORKPACK_INDEX.md`.
4. Open only the assigned workpack.
5. Use `CHECKLIST_INDEX.md` for exact checklist sections.
6. Use `PROOF_INDEX.md` for proof artifacts.

## Current snapshot source

- Snapshot: [current-ai-snapshot.md](current-ai-snapshot.md)
- Code/test audit: [CODE_AUDIT.md](CODE_AUDIT.md)

## What is already present / proved

- AI expectation doc and local-first custody boundary.
- Local AI and TabAgent reuse architecture.
- Local AI provider runtime boundary.
- Local AI evidence context builder spec.
- Local AI safety evaluator feature doc.
- Parent assistant feature doc and provider routing proof.
- Browser URL/video, social/feed, and browser-game AI planning inside `browser-plan`.
- Screen capture and screen-intelligence planning inside `screen-plan`.
- Runtime/provider status, local AI chat generation, provider scheduler, and parent assistant proof scripts.
- Reusable Rust eventing infrastructure in `crates/ocentra-eventing`, with existing network and parent/child consumer examples that AI work should follow instead of adding direct capture-to-AI coupling.
- LAN AI job submit/status and legacy screen family-hub proof primitives exist, but the target architecture is the Household AI Provider Mesh. Those existing primitives are precursors only: they do not yet prove decentralized provider discovery, claim/lease, child-agent result validation, duplicate prevention, retry/dead-letter, mobile dormant/fallback policy, child-agent-only policy authority, or physical household LAN execution.
- Activity memory graph contracts and read-model proof pieces.

## Open gaps / missing product runtime

- Product-grade local model configuration and artifact selection.
- Verified local model artifact download, integrity, cache, and retention flow.
- Product-grade local inference execution path for safety decisions.
- Cross-slice AI job queue and resource scheduler.
- Household AI provider mesh contracts and runtime proof. Existing local provider scheduler proof does not prove cross-device provider discovery, claim/lease, idempotency, result validation, two-device LAN execution, no raw screenshot transfer, or child-agent policy authority.
- Model quality validation and confidence calibration.
- OCR execution path.
- Guided local VLM execution path.
- Evidence-backed memory/graph minimal product implementation.
- Full TabAgent code reuse audit and extraction plan.
- Parent explanation UI that cites evidence, rules, model/runtime refs, and degraded states.
- Real stored-evidence validation for browser, app/game, tracking, LAN, network, and screen slices.

## Current proof interpretation

```text
Historical checklist rows are status input, not completion authority.
Current workpack boxes and assigned proof roots are the active execution source.
A checked checklist row cannot close an open workpack without current proof artifacts and command logs.
Mock provider, dry-run adapter, docs-only proof, or a schema-only unit test cannot prove local model runtime readiness.
A feature-evidence bridge proof proves only the handoff route unless it includes the owning feature evidence source and AI validation result.
An AI result is not a policy/enforcement decision until deterministic policy consumes it through the owned handoff.
```

## Checklist summary

- Full checklist: [implementation-checklist.md](implementation-checklist.md) (not default context).
- Checkbox rows detected: 169 total, 168 checked, 1 unchecked.
- Checklist index: [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md).

## Workpack summary

- Workpacks indexed: 48.
- Workpacks with open checkboxes: 46.
- Workpacks with all detected boxes checked: 2.
- Workpacks with no checkbox status: 0.

### Active/open workpacks

- [48 - Rollout Checklist And PR Gate](workpacks/48-rollout-checklist-and-pr-gate.md) - 0/25 checked, 25 open.
- [32 - Household AI Provider Mesh And Remote Assistant Boundary](workpacks/32-family-ai-hub-and-remote-assistant-boundary.md) - 0/11 checked, 11 open.
- [07 - AI Job Queue Contract](workpacks/07-ai-job-queue-contract.md) - 1/11 checked, 10 open.
- [08 - AI Provider Routing Contract](workpacks/08-ai-provider-routing-contract.md) - 0/10 checked, 10 open.
- [38 - Screen OCR VLM Router Lane](workpacks/38-screen-ocr-vlm-router-lane.md) - 0/9 checked, 9 open.
- [31 - Guided VLM Worker Lane](workpacks/31-guided-vlm-worker-lane.md) - 0/8 checked, 8 open.
- [46 - Security Privacy Negative Gates Lane](workpacks/46-security-privacy-negative-gates-lane.md) - 0/8 checked, 8 open.
- [47 - Performance Resource Battery Proof Lane](workpacks/47-performance-resource-battery-proof-lane.md) - 0/8 checked, 8 open.
- [03 - Contract Boundary And Effect Schemas](workpacks/03-contract-boundary-and-effect-schemas.md) - 0/6 checked, 6 open.
- [09 - Local Evidence Context Builder V1](workpacks/09-local-evidence-context-builder-v1.md) - 0/6 checked, 6 open.
- [13 - Deterministic No-Model Classifier Lane](workpacks/13-deterministic-no-model-classifier-lane.md) - 0/6 checked, 6 open.
- [14 - Local Text LLM Adapter Boundary](workpacks/14-local-text-llm-adapter-boundary.md) - 0/6 checked, 6 open.
- [15 - Local Text LLM Execution Dry-Run Adapter](workpacks/15-local-text-llm-execution-dry-run-adapter.md) - 0/6 checked, 6 open.
- [16 - Output Parser And Schema Validator](workpacks/16-output-parser-and-schema-validator.md) - 0/6 checked, 6 open.
- [17 - Degraded Timeout Invalid-Output Handling](workpacks/17-degraded-timeout-invalid-output-handling.md) - 0/6 checked, 6 open.
- [20 - Parent Explanation Read Model](workpacks/20-parent-explanation-read-model.md) - 0/6 checked, 6 open.
- [23 - Evidence-Backed Semantic Memory](workpacks/23-evidence-backed-semantic-memory.md) - 0/6 checked, 6 open.
- [25 - Minimal Graph Edges For Safety Context](workpacks/25-minimal-graph-edges-for-safety-context.md) - 0/6 checked, 6 open.
- [33 - Browser URL Video AI Lane](workpacks/33-browser-url-video-ai-lane.md) - 0/6 checked, 6 open.
- [35 - Browser Game Cloud Game AI Lane](workpacks/35-browser-game-cloud-game-ai-lane.md) - 0/6 checked, 6 open.
- [36 - App Game Unknown Classifier Lane](workpacks/36-app-game-unknown-classifier-lane.md) - 0/6 checked, 6 open.
- [37 - Tracking Location Safety Analysis Lane](workpacks/37-tracking-location-safety-analysis-lane.md) - 0/6 checked, 6 open.
- [41 - Llama GGUF Runtime Packaging Lane](workpacks/41-llama-gguf-runtime-packaging-lane.md) - 0/6 checked, 6 open.
- [43 - AI Activity Portal Surface Lane](workpacks/43-ai-activity-portal-surface-lane.md) - 0/6 checked, 6 open.

## Default no-read list

- `README_FULL_ORIGINAL.md` unless you need historical full README context.
- Full `implementation-checklist.md` unless `CHECKLIST_INDEX.md` names exact section/row.
- All workpacks; use `WORKPACK_INDEX.md`.
- Source inventories and pasted-content audits unless source ownership is unclear.
- Historical checkpoint/proof docs unless `PROOF_INDEX.md` or the assigned workpack names them.

## Health / consistency

- See `PLAN_HEALTH.md` before claiming the whole plan is complete or stale.

## HID Execution Guard (added 2026-06-12)

- Scope and completion source:
  - follow [PLAN_HID_MATRIX.md](../../PLAN_HID_MATRIX.md) execution slice, then this plan's assigned WORKPACK_INDEX.md and NEXT_ACTIONS.md.
  - do not mark this plan complete from checklist deltas alone.
- Before any checked update, attach:
  - a real test run log (or explicit known blocker) from the assigned implementation boundary,
  - a proof manifest under docs/proof/ai-plan/.
- Required proof manifest names:
  - docs/proof/ai-plan/slice-01-*.md
  - docs/proof/ai-plan/slice-02-*.md
  - docs/proof/ai-plan/slice-03-*.md
  - each proof file must include commands, pass/fail,
    negative-cases, and manual-required notes.
- Failure rule: no PR-ready claim until replay/idempotency, authZ/replay, and rollback/teardown proofs are present for the assigned slice.

## HID execution blueprint

Continue execution from: [PLAN_EXECUTION_BLUEPRINT.md](PLAN_EXECUTION_BLUEPRINT.md).
Update this plan only via the blueprint and matching workpack checklist.
