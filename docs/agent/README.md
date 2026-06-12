<!-- agent-capsule -->

> Agent Capsule
> Doc: Agent Flow Directory
> Kind: agent flow documentation; read only when selected by root AGENTS or TASK_ROUTER.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Agent Flow Directory

This folder replaces the old single long root `AGENTS.md` with focused flows.
The root `AGENTS.md` points here first so Codex can choose only the flow needed
for the current task.

## Files

| File                               | Use when                                                                              |
| ---------------------------------- | ------------------------------------------------------------------------------------- |
| `TASK_ROUTER.md`                   | First file after root `AGENTS.md`; chooses the route.                                 |
| `PRIMARY_COORDINATOR_FLOW.md`      | You are assigning, reviewing, integrating, or merging lane work.                      |
| `WORKER_LANE_FLOW.md`              | You are a lane worker with hub/Ledger mail.                                           |
| `WORKTREE_LANE_START.md`           | You are starting/resuming a worktree or moving between PCs.                           |
| `HUB_LEDGER_MESSAGING.md`          | You need exact hub/Ledger commands or wakeup protocol.                                |
| `PLAN_WORKER_FLOW.md`              | You are doing product, feature, docs, UI, policy, AI, enforcement, or reporting work. |
| `PRODUCT_DOC_FLOW.md`              | You need product-doc update protocol and feature/expectation routing.                 |
| `SOURCE_BOUNDARY_FLOW.md`          | You touch shared contracts, TS/Rust boundaries, schema brands, literals, tests.       |
| `VALIDATION_FLOW.md`               | You need validation and CI-readiness rules.                                           |
| `TEST_PROOF_DECISION_MATRIX.md`    | You know the plan/workpack and need to choose required tests or proof by risk.        |
| `PR_DONE_FLOW.md`                  | You are reporting DONE/PR_READY or creating a PR.                                     |
| `LOCAL_DEV_PORTS.md`               | You need dev ports or visible demos.                                                  |
| `RELEASE_FLOW.md`                  | You touch packaging, versioning, production branch, installer publishing.             |
| `MIGRATED_ROOT_AGENTS_ORIGINAL.md` | Archive copy of the uploaded full root guide. Do not read by default.                 |
