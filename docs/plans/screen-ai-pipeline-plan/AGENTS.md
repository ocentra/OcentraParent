<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-ai-pipeline-plan`
> Doc: `Screen AI Pipeline Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Screen AI Pipeline Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/screen-ai-pipeline-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.
Use `WORKPACK_FAMILIES.md` only when the selected workpack owner/proof family is unclear.

## Ownership, Import, And Boundary Contract

This plan owns the integration pipeline from real screen-trigger/capture evidence into local AI analysis, schema-valid AI results, deterministic policy handoff, action dry-run proof, journal/read-model/portal projection, custody/delete proof, live-operator proof, performance/backpressure proof, and final rollout proof. It does not own raw capture mechanics, shared AI provider/runtime, policy authority, enforcement execution, data custody policy, portal UX ownership, browser/app/network/tracking source truth, or remote access.

Module roles:

```text
screen-ai-pipeline-plan: multi-hop integration proof, scenario routing, proof shape, no-claim boundaries, and rollout gate for screen -> AI -> policy/action path.
screen-plan and screen-domain: screen capture, screen evidence contracts, OCR/VLM/screen intelligence contracts, protected surfaces, disclosure, and screen settings. screen-domain is a real contract package, not just metadata.
ai-plan and crates/schema: shared AI/runtime/evidence-context/model-result contracts. ai-domain is package identity and focused tests; canonical shared AI contracts live in crates/schema.
policy-control-plane-plan: deterministic policy source truth, parent rule precedence, preview/delivery/approval semantics, and policy decision boundary.
v0-8-enforcement-control-plan: enforcement action authority, adapter execution, rollback, and supported runtime proof.
data-custody-storage-plan: deletion, retention, export, privacy, and custody rules for raw images, queue artifacts, results, and screenshots.
portal-ux-household-surfaces-plan and portal-domain/apps/portal: parent-visible projection, screenshots, route/UI proof, and no-fake-data presentation.
browser-plan, app-game-plan, network-plan, tracking-plan, and screen-plan: domain source truth, triggers, evidence provenance, and target-specific runtime behavior.
agent-protocol, agent-service, and agent-core: protocol/service/journal/read-model seams only when selected by the workpack.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
crates/schema canonical screen/AI/evidence/policy/action/custody shapes
screen-domain public screen evidence/intelligence/router/disclosure contracts when selected
agent-protocol-domain/agent-protocol public read models, events, or command seams when selected
portal-domain public projection contracts when selected
domain-plan public handoff contracts only when the selected workpack names them
neutral event/evidence/logging helpers that do not own product behavior
```

Forbidden direct imports and claims:

```text
screen capture internals imported to bypass screen-plan ownership
AI/provider internals imported to bypass ai-plan/crates/schema contracts
AI result upgraded into policy authority
policy decision upgraded into enforcement execution
policy dry-run upgraded into adapter/runtime proof
local capture proof upgraded into full screen-AI product proof
mock/fixture proof upgraded into product proof
live-operator artifact-gate proof upgraded into live capture rerun proof
portal screenshot upgraded into pipeline/runtime proof
raw image path, prompt, OCR text, VLM output, or child-private payload leaked into unredacted logs
retention/custody proof claimed without deletion/retention artifacts
```

If screen-AI work needs raw capture, model provider behavior, policy authority, enforcement action, custody, portal rendering, domain trigger truth, or remote access behavior, it must use typed handoffs, retained proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) only when owner/proof family is unclear.
5. Assigned workpack under `workpacks/`, if any.
6. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
7. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
8. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

## Local decision tree

- If the hub assignment names a workpack, open only that workpack, then use `TEST_PROOF_EXPECTATIONS.md` to choose expected tests/proof for that work type.
- If the assignment names a checklist row but no workpack, use `CHECKLIST_INDEX.md` to locate the row, then use `TEST_PROOF_EXPECTATIONS.md` for required tests/proof.
- If the assignment is docs/status only, use `DOC_INDEX.md` and the docs/status rows in `TEST_PROOF_EXPECTATIONS.md`; do not inspect source or sibling plans unless the row names them.
- If the assignment touches source, contracts, runtime, UI, AI, platform, security, persistence, or observability, read `../../agent/SOURCE_BOUNDARY_FLOW.md` only after the local workpack is known.
- If the assignment is PR_READY or DONE, read `TEST_PROOF_EXPECTATIONS.md`, `PROOF_INDEX.md`, `PLAN_HEALTH.md` only for broad claims, then `../../agent/PR_DONE_FLOW.md`.
- If `TEST_PROOF_EXPECTATIONS.md` says a required test/proof is missing, keep the row open and report the missing test/proof instead of claiming completion.

## Local work loop

1. Read only the route files above and the assigned workpack/checklist row.
2. Identify the intended implementation crate/package or current owning package/crate if the per-plan implementation crate is not created yet.
3. Make the narrow code/doc change.
4. Run the lightest relevant compile/lint/type/schema check for the touched area before expanding scope.
5. Add or update the tests named by `TEST_PROOF_EXPECTATIONS.md`; if the expected test folder/crate does not exist yet, record the missing location and keep the row open.
6. Run the focused tests/proof commands, then run broader validation only when `VALIDATION_FLOW.md` or PR_READY scope requires it.
7. Update workpack/checklist/proof docs with exact test names, command logs, proof artifacts, skipped checks, and remaining gaps.

## Product docs for this plan

[screen-evidence-analysis.md](../../features/screen-evidence-analysis.md), [screen-visibility-live-view.md](../../features/screen-visibility-live-view.md), [local-ai-safety-evaluator.md](../../features/local-ai-safety-evaluator.md), [screen-evidence.md](../../expectations/screen-evidence.md), [ai.md](../../expectations/ai.md)

## Validation and proof choice

After the assigned workpack is known, use [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) first, then [TEST_PROOF_DECISION_MATRIX.md](../../agent/TEST_PROOF_DECISION_MATRIX.md) only for global risk escalation. Record the selected rows in DONE/PR_READY. Do not read unrelated proof docs, and do not close checklist rows with happy-path-only proof when auth, protocol, persistence, UI, AI, platform, security, performance, or observability risk is touched.

## Do not read by default

- `implementation-checklist.md` as a whole.
- all `workpacks/*.md`.
- `README_FULL_ORIGINAL.md`.
- `source-index.md` or pasted-content audits unless source ownership is unclear.
- sibling plan folders.
- global checkpoints unless `PROOF_INDEX.md` names them for your proof.

## Before DONE / PR_READY

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and `../../agent/VALIDATION_FLOW.md`.
