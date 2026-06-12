<!-- agent-capsule -->

> Agent Capsule
> Doc: Task Router
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Task Router

Read this immediately after root `AGENTS.md`. Pick one route. Do not read all
flows. If more than one route seems possible, choose the route that owns the
next decision, then stop reading until that route points deeper.

Authoring rule: routing docs define goals, expected shapes, boundaries, proof,
and failure conditions. They must not spoon-feed implementation code or tell a
future worker exactly what code to write.

## Route decision

| Assignment says...                                                                                                       | Read                                                             |
| ------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------- |
| You are the primary coordinator, hub, integrator, reviewer, or merger                                                    | `PRIMARY_COORDINATOR_FLOW.md`                                    |
| You are `codex-a`, `codex-b`, another lane worker, or have hub inbox mail                                                | `WORKER_LANE_FLOW.md`                                            |
| You are starting/resuming a worktree or using multiple PCs                                                               | `WORKTREE_LANE_START.md`                                         |
| You are implementing or updating a feature, plan, UI, AI, policy, reporting, enforcement, platform, remote, or docs task | `PLAN_WORKER_FLOW.md`                                            |
| You are updating feature docs, expectation docs, roadmap, product checklist, or competitor parity                        | `PRODUCT_DOC_FLOW.md`                                            |
| You are touching contracts, schemas, TypeScript/Rust protocol boundaries, app/crate source, tests, or source shape       | `SOURCE_BOUNDARY_FLOW.md`                                        |
| You are preparing DONE, PR_READY, PR body, integration, or merge readiness                                               | `PR_DONE_FLOW.md`                                                |
| You are debugging CI, local validation, heavier gates, or test requirements                                              | `VALIDATION_FLOW.md`                                             |
| You need to decide which tests/proof artifacts apply to a workpack                                                       | `TEST_PROOF_DECISION_MATRIX.md` after the plan/workpack is known |
| You are touching release, installer, package previews, version tags, production branch                                   | `RELEASE_FLOW.md`                                                |

## Route conflict rule

When one task mentions several areas, choose the first owner that must make a
decision. Do not read every possible owner.

| Conflict                                      | Owning first read                                                                             | Stop condition                                                                    |
| --------------------------------------------- | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| Product feature versus implementation package | `PLAN_WORKER_FLOW.md`                                                                         | Stop at the selected plan unless the plan workpack names the package.             |
| Source contract versus UI usage               | `SOURCE_BOUNDARY_FLOW.md` after plan selection                                                | Stop at the contract owner before reading UI callers.                             |
| Evidence capture versus enforcement           | Evidence plan first                                                                           | Enforcement plan is read only when the assignment asks for policy/action handoff. |
| AI classification versus policy authority     | AI plan first for model/output proof; enforcement/policy plan first for action authority      | No AI doc may create enforcement authority.                                       |
| Browser URL versus native app/game activity   | Browser plan for URL/tab/profile work; app-game plan for native process/package/launcher work | Do not read both unless a workpack names a handoff.                               |
| Screen capture versus screen AI               | Screen plan for capture/custody; screen-ai pipeline for OCR/VLM/result validation             | Do not claim content understanding from capture proof.                            |
| Package/release versus product readiness      | Runtime package plan or release flow                                                          | Do not change product status from installer smoke alone.                          |

## Product/plan route shortcut

For feature or plan work:

1. Read `PLAN_WORKER_FLOW.md`.
2. Read `docs/PLAN_INDEX.md`.
3. Open only the selected plan folder's `AGENTS.md`.
4. Open the plan's `PLAN_STATE.md`, `NEXT_ACTIONS.md`, and `WORKPACK_INDEX.md`.
5. Open only the assigned workpack.
6. Use `TEST_PROOF_DECISION_MATRIX.md` to choose tests/proof only after the
   workpack and touched risk surfaces are known.

If no plan owns the work, use `docs/FEATURE_ROUTE_INDEX.md` to find the owning
feature doc, then ask the hub before broad implementation.
