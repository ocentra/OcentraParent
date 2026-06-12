<!-- agent-capsule -->

> Agent Capsule
> Doc: Ocentra Parent Docs Router
> Kind: global docs router/index; read to choose a smaller route, then stop.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Ocentra Parent Docs Router

This docs folder is organized for low-token agent work. Do not browse the tree
manually. Start from the appropriate index.

| Need                                 | Read                                          |
| ------------------------------------ | --------------------------------------------- |
| Choose a product/plan route          | `PLAN_INDEX.md`                               |
| Choose a feature route               | `FEATURE_ROUTE_INDEX.md` or `feature-list.md` |
| Find a source/reference doc          | `SOURCE_DOC_ROUTER.md`                        |
| Find historical proof/checkpoint     | `CHECKPOINT_INDEX.md`                         |
| Work as an agent/lane                | `agent/TASK_ROUTER.md`                        |
| Find plan status conflicts/staleness | `PLAN_HEALTH_INDEX.md`                        |
| Find any doc by path                 | `ALL_DOC_FILES_INDEX.md`                      |

Plan folders now contain short routing/state files. Agents should read a plan's
`AGENTS.md`, `PLAN_STATE.md`, and `WORKPACK_INDEX.md` before opening any large
checklist or historical proof file.
