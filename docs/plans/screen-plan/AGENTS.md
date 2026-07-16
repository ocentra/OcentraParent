<!-- agent-capsule -->

> Agent Capsule
> Plan: `screen-plan`
> Doc: `Screen Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Screen Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/screen-plan`.

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

This plan owns local screen capture mechanics, capture scope/trigger definitions, platform adapter contracts, protected-surface behavior, screen evidence contracts, temporary encrypted queue behavior, local deletion proof, optional raw screenshot retention gates, optional live-view preflight gates, child disclosure, screen-specific portal/status projections, and proof routing for screen-owned work. It does not own full screen-AI pipeline completion, shared AI provider/runtime behavior, policy authority, enforcement execution, product custody policy, remote-access relay/session authority, browser/app/network/tracking source truth, or broad portal UX completion.

Module roles:

```text
screen-plan: local screen evidence/capture/custody/settings/live-view-boundary owner and screen proof router.
screen-domain: public screen capture, screen evidence, OCR, VLM, disclosure, settings, screen-intelligence-router, and handoff contract package.
screen-ai-pipeline-plan: multi-hop screen -> AI -> policy/action integration proof and live-operator product-path proof.
ai-plan and schema-domain: shared AI context/result/provider/degradation contracts and AI runtime/model behavior when selected.
policy-control-plane-plan: policy source truth, parent-rule precedence, preview/dry-run semantics, and deterministic policy authority.
v0-8-enforcement-control-plan: enforcement adapter execution, rollback, and supported runtime proof.
data-custody-storage-plan: product retention/export/delete/privacy/custody policy and parent-owned storage semantics.
portal-ux-household-surfaces-plan: rendered parent UX, screenshots, route proof, and no-fake-data presentation.
remote-access-plan: remote live-access capability, relay/session semantics, standing grants, and remote proof route.
browser-plan, app-game-plan, network-plan, and tracking-plan: source-trigger/source-truth owners for their domains.
agent-protocol, agent-service, and agent-core: protocol/service/queue/journal/read-model seams when the selected workpack names them.
```

Direct imports are allowed only for explicit public helper surfaces:

```text
schema-domain canonical screen/evidence/custody/capability/status shapes
screen-domain public contracts, screen intelligence, disclosure, routing, and handoff guard surfaces
agent-protocol-domain/agent-protocol public screen read models, events, and command seams when selected
portal-domain public screen projection contracts when selected
data-custody/policy/enforcement/AI/domain public handoff contracts only when the selected workpack names them
neutral event/evidence/logging helpers that do not own screen product behavior
```

Forbidden direct imports and claims:

```text
screen capture internals imported to bypass screen-plan adapter boundaries
screen proof upgraded into screen-AI pipeline completion
local capture proof upgraded into OCR/VLM quality or AI safety proof
screen summary upgraded into policy authority or enforcement execution
policy dry-run upgraded into adapter runtime proof
raw screenshot retention enabled without custody/legal/parent opt-in proof
live-view preflight or local loopback proof upgraded into product live-view readiness
screen live-view proof upgraded into remote-access relay/session readiness
remote summary export upgraded into raw screenshot remote upload
portal screenshot upgraded into screen runtime proof
mock screenshot or fixture proof upgraded into product proof
raw image path, OCR text, VLM output, or child-private payload leaked into unredacted logs
```

If screen work needs AI, policy, enforcement, custody, portal UX, remote access, browser/app/network/tracking source truth, or service runtime behavior, it must use typed handoffs, retained proof roots, and explicit no-claim boundaries. Do not solve cross-plan behavior by importing another feature owner's runtime internals.

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

[screen-evidence-analysis.md](../../features/screen-evidence-analysis.md), [screen-visibility-live-view.md](../../features/screen-visibility-live-view.md), [screen-evidence.md](../../expectations/screen-evidence.md)

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
