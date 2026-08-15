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

## Code-first Phase 1 audit (2026-08-15)

- Authoritative audit: [CODE_AUDIT.md](CODE_AUDIT.md).
- All 48 workpacks now have reviewed code/test topology in the engineering graph.
- 11 workpacks are Phase 1 complete for their bounded source/test scope: 01, 02,
  05, 06, 08, 10, 11, 24, 26, 32, and 37.
- 37 workpacks retain a production-code or expected-test gap. The workpack
  checkbox summary below is plan-document state, not this implementation result.
- Phase 2 focused test/Enforcer execution and Phase 3 proof regeneration were
  not run as part of this audit.

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
