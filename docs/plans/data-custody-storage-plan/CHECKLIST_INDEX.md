<!-- agent-capsule -->

> Agent Capsule
> Plan: `data-custody-storage-plan`
> Doc: `CHECKLIST_INDEX.md`
> Kind: checklist route index.
> Read when: Only when a workpack or DONE/PR_READY flow names checklist rows.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Checklist status changes require proof rows and feature/product status sync.

<!-- /agent-capsule -->

# Data Custody Storage Plan Checklist Index

Use this file to locate checklist intent without opening broad checklists.

| Area                      | Required status rule                                                                  |
| ------------------------- | ------------------------------------------------------------------------------------- |
| Source and acceptance map | Keep status open until feature, expectation, capability, and route rows agree.        |
| Current state and gaps    | Do not close gaps without proof artifact paths.                                       |
| Contracts and boundaries  | Expected schema/protocol/read-model shapes must have negative proof.                  |
| Runtime/user flow handoff | UI/runtime/service handoffs need logs, screenshots, traces, or manual-required proof. |
| Rollout and PR gate       | DONE/PR_READY requires selected proof intents and known gaps.                         |
