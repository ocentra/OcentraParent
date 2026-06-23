<!-- agent-capsule -->

> Agent Capsule
> Plan: `app-game-plan`
> Doc: `App + Game Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# App + Game Plan Agent Route

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects
`docs/plans/app-game-plan`.

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

App/game is the native app and native game evidence spine. It observes and normalizes local app/game facts, writes evidence, builds read models, and emits typed handoffs. It does not own AI classification, policy authority, enforcement authority, notification delivery, portal rendering, or platform custody beyond the assigned adapter surface.

Module roles:

```text
schema-domain: canonical shared app/game schema, brands, parser, evidence row, adapter readiness, classifier digest, policy target, platform proof, child UX, and handoff shapes when those shapes cross package, crate, app, or plan boundaries.
app-game-domain: TypeScript helper/projection and focused validation surface. It consumes canonical schema-domain shapes and must not become a broad aggregator of policy, enforcement, notification, portal, or production runtime behavior.
app-game-core: child-local Rust app/game observation, sessionization, evidence event, AI-request event, and policy-request event boundary. It should use event/protocol handoffs rather than importing sibling runtime crates.
agent-protocol and agent-service: wire/service boundaries only when the selected workpack names protocol, service handler, or read API proof.
parent-domain, policy-domain, enforcement-domain, notification-domain, portal-domain, and apps/portal: consumers/projections/handoff owners. They must not re-own app/game observation, sessionization, or platform adapter authority.
AI plan: classifier/digest consumer only. AI consumes stored evidence or structured digests and does not scan apps, games, launchers, windows, processes, or devices directly.
platform adapters: source observers for assigned OS/platform proof only. They produce typed observations and capability states; they do not decide policy or enforcement.
```

Direct imports are allowed only for neutral/shared infrastructure or explicit public helper surfaces:

```text
canonical schema-domain app/game/evidence/policy-reference/protocol/capability/logging shapes
neutral event/evidence/logging/protocol primitives
approved public app-game-domain helper exports when the selected workpack names app-game-domain scope
approved Rust runtime/parity crates when the selected workpack names Rust proof
pure common helpers that do not own feature behavior or side effects
```

Forbidden direct imports:

```text
sibling feature owner runtime behavior from AI, policy, enforcement, notification, portal, child-runtime, LAN, remote, setup, payment, or data-custody plans
private source files from another plan's owning package/crate
peer feature contracts when the shared shape should live in schema-domain or another neutral boundary
portal, policy, AI, or notification code that scans app/game source state instead of consuming app/game evidence/read models
policy or enforcement internals that execute app/game actions without typed app/game authority, source freshness, and adapter-readiness proof
```

If app/game needs AI, policy, enforcement, notification, portal, child-runtime, LAN, or remote behavior, it must use typed evidence, commands, events, requests, read models, and proof handoffs. If a shape is used by multiple feature owners, place or consume it through `schema-domain` or another neutral shared boundary. Do not solve cross-plan behavior by importing another feature's runtime internals.

## Default read order

1. [PLAN_STATE.md](PLAN_STATE.md) - current state, open gaps, default no-read list.
2. [NEXT_ACTIONS.md](NEXT_ACTIONS.md) - short resume/open-work list.
3. [WORKPACK_INDEX.md](WORKPACK_INDEX.md) - choose assigned workpack only.
4. [WORKPACK_FAMILIES.md](WORKPACK_FAMILIES.md) - classify the assigned workpack family only when owner path is unclear.
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

[app-game-control.md](../../features/app-game-control.md), [app-install-purchase-approval.md](../../features/app-install-purchase-approval.md), [enforcement-integrity-tamper.md](../../features/enforcement-integrity-tamper.md), [app-game-evidence.md](../../expectations/app-game-evidence.md), [enforcement.md](../../expectations/enforcement.md)

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
