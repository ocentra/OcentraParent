<!-- agent-capsule -->

> Agent Capsule
> Plan: `ai-plan`
> Doc: `AI Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# AI Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/ai-plan`.

## High-density execution contract

Task: work only the assignment slice for this plan.
Context: `PLAN_STATE.md` is current state; `WORKPACK_INDEX.md` chooses one workpack; `TEST_PROOF_EXPECTATIONS.md` defines required local tests/proof.
Scope rule: one plan, one workpack, exact checklist rows. Sibling plans, full checklists, source inventories, and checkpoints are closed unless named by the selected route.
Implementation rule: code may move only after route, workpack, expected tests, and proof location are identified.
Test rule: expected tests are obligations, not suggestions. If the test crate/folder does not exist yet, record the missing location and keep the row open.
Proof rule: proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
Authoring rule: this plan describes outcomes, boundaries, expected tests, proof, and failure conditions; it must not prescribe implementation code except for minimal public contract or artifact-shape examples.
Failure condition: no DONE/PR_READY when tests are happy-path only, proof is missing, product status moved without evidence, or validation scope is not listed.

## Ownership, Import, And Boundary Contract

AI is an evidence consumer and evaluator, not a capture owner, policy authority, enforcement owner, or transport owner.

Module roles:

```text
crates/schema or the owning Rust crate: canonical shared AI schema, brands, parsers, literals, context/result/runtime/reference shapes, route/action/read-model DTOs, and encoded-shape owner when AI contracts cross package, crate, app, or plan boundaries.
schema-domain: temporary generated-validation or edge-decoder surface only where TypeScript still needs one during migration.
ai-domain: TypeScript package identity, helper/projection tests, and focused AI-domain validation surface; it must not re-own canonical shared AI contracts now centralized in Rust-owned schema surfaces.
child-ai-core: child-local AI runtime/evaluator boundary; it validates local context, provider results, and accepted AI output before policy can consume it.
screen-ai-core: screen AI worker/router boundary for screen-analysis jobs; it consumes screen evidence references and must not own general screen capture or retention.
agent-protocol and agent-service: protocol/transport boundaries only when a selected workpack names wire/service proof.
portal-domain and apps/portal: parent-visible AI status/explanation projections; they do not run child-safety AI or own policy/enforcement.
Browser, screen, tracking, network, and app-game plans: evidence/source owners only; AI consumes their evidence/read models and must not import their runtime behavior.
policy and enforcement plans: deterministic decision/action consumers; AI output is evidence input and cannot bypass parent-authored policy authority.
LAN/remote plans: provider/job transport or remote-assistant handoff surfaces only; they do not own AI safety classification or local child-policy authority.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
Rust-owned canonical AI, evidence, policy-reference, family-reference, protocol, capability, and logging shapes plus generated DTOs or temporary edge decoders
neutral event/evidence/logging/protocol primitives
approved public ai-domain helper exports when the selected workpack names ai-domain as helper/projection scope
approved Rust parity/runtime crates when the selected workpack names Rust proof
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports:

```text
sibling feature owner packages or crates for browser, screen, tracking, network, app/game, policy, enforcement, LAN, or remote runtime behavior
private source files from another plan's owning package/crate
peer feature contracts when the shared shape should live in crates/schema or another neutral Rust-owned boundary
AI runtime calls that scrape browser/screen/network/tracking/app state directly instead of consuming evidence/read-model/request results
policy or enforcement internals to turn a model label into an action without deterministic policy handoff
```

If AI needs more context from browser, screen, tracking, network, app/game, LAN, or remote surfaces, it must use typed evidence, read models, commands, events, requests, provider jobs, or proof handoffs. If a shape is shared by multiple feature owners, place or consume it through `crates/schema` or another neutral Rust-owned boundary. Use `schema-domain` only as a temporary generated-validation or edge-decoder surface while migration is still incomplete. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. Assigned workpack under `workpacks/`, if any.
5. [CHECKLIST_INDEX.md](CHECKLIST_INDEX.md) - exact checklist section/row lookup only.
6. [TEST_PROOF_EXPECTATIONS.md](TEST_PROOF_EXPECTATIONS.md) - local test/proof decision tree after the workpack is known.
7. [PROOF_INDEX.md](PROOF_INDEX.md) - only when validating proof or PR-ready claims.

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

[local-ai-safety-evaluator.md](../../features/local-ai-safety-evaluator.md), [parent-assistant-actions.md](../../features/parent-assistant-actions.md), [ai.md](../../expectations/ai.md), [parent-assistant-chat.md](../../expectations/parent-assistant-chat.md)

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

Read `PLAN_HEALTH.md` if you are making a broad completion/staleness claim. Update the assigned workpack, relevant checklist rows, proof references, and
feature/product docs as needed. Then follow `../../agent/PR_DONE_FLOW.md` and
`../../agent/VALIDATION_FLOW.md`.
