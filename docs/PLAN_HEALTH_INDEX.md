<!-- agent-capsule -->

> Agent Capsule
> Doc: Plan Health Index
> Kind: global docs router/index; read to choose a smaller route, then stop.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Plan Health Index

This index highlights documentation state conflicts that can mislead Codex. It is not a default work file unless claiming DONE/PR_READY for a broad plan.

Checklist markers in plan docs were intentionally reset in this pass. Treat the counts below as a historical snapshot, and use [PLAN_QUALITY_MATRIX.md](PLAN_QUALITY_MATRIX.md) for the current documentation-quality view.

| Plan                                      | Snapshot                                                                 | Checklist                   | Workpacks                                                                    | Health file                                                                  |
| ----------------------------------------- | ------------------------------------------------------------------------ | --------------------------- | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| `ai-plan`                                 | current-ai-snapshot.md                                                   | 168/169 checked             | 47/48 open                                                                   | [PLAN_HEALTH.md](plans/ai-plan/PLAN_HEALTH.md)                               |
| `app-game-plan`                           | current-app-game-snapshot.md                                             | 211/229 checked             | 6 open reference/control-routing workpacks; many implementation rows checked | [PLAN_HEALTH.md](plans/app-game-plan/PLAN_HEALTH.md)                         |
| `app-plan`                                | current-app-snapshot.md                                                  | 102/267 checked             | 7/95 open                                                                    | [PLAN_HEALTH.md](plans/app-plan/PLAN_HEALTH.md)                              |
| `browser-plan`                            | current-browser-snapshot.md                                              | 97/138 checked              | 6 open reference/control-routing workpacks; 24 implementation rows checked   | [PLAN_HEALTH.md](plans/browser-plan/PLAN_HEALTH.md)                          |
| `eventing-plan`                           | current-eventing-snapshot.md                                             | 137/138 checked             | 12/12 open route workpacks                                                   | [PLAN_HEALTH.md](plans/eventing-plan/PLAN_HEALTH.md)                         |
| `lan-plan`                                | current-lan-snapshot.md                                                  | 9/74 checked                | 20/20 open                                                                   | [PLAN_HEALTH.md](plans/lan-plan/PLAN_HEALTH.md)                              |
| `network-plan`                            | current-network-snapshot.md                                              | 127/128 checked             | 8/8 open route workpacks                                                     | [PLAN_HEALTH.md](plans/network-plan/PLAN_HEALTH.md)                          |
| `parent-client-runtime-distribution-plan` | execution-grade route established; workpacks present                     | no implementation checklist | 11/11 open                                                                   | [PLAN_HEALTH.md](plans/parent-desktop-runtime-package-plan/PLAN_HEALTH.md)   |
| `child-agent-runtime-distribution-plan`   | execution-grade route established; workpacks present                     | no implementation checklist | 11/11 open                                                                   | [PLAN_HEALTH.md](plans/child-agent-runtime-distribution-plan/PLAN_HEALTH.md) |
| `portal-ux-household-surfaces-plan`       | missing                                                                  | no implementation checklist | 19/20 open                                                                   | [PLAN_HEALTH.md](plans/portal-ux-household-surfaces-plan/PLAN_HEALTH.md)     |
| `screen-ai-pipeline-plan`                 | missing                                                                  | 134/135 checked             | 2/10 open                                                                    | [PLAN_HEALTH.md](plans/screen-ai-pipeline-plan/PLAN_HEALTH.md)               |
| `screen-plan`                             | current-screen-snapshot.md                                               | 100/100 checked             | 22/40 open                                                                   | [PLAN_HEALTH.md](plans/screen-plan/PLAN_HEALTH.md)                           |
| `tracking-plan`                           | current-tracking-snapshot.md                                             | 79/111 checked              | 28/33 open                                                                   | [PLAN_HEALTH.md](plans/tracking-plan/PLAN_HEALTH.md)                         |
| `v0-8-enforcement-control-plan`           | missing                                                                  | no implementation checklist | 18/20 open                                                                   | [PLAN_HEALTH.md](plans/v0-8-enforcement-control-plan/PLAN_HEALTH.md)         |
| `setup-install-provisioning-plan`         | first-pass; research gate required before implementation                 | no implementation checklist | 6/6 research workpacks open                                                  | [PLAN_HEALTH.md](plans/setup-install-provisioning-plan/PLAN_HEALTH.md)       |
| `account-identity-family-plan`            | first-pass; research gate required before implementation                 | no implementation checklist | 6/6 research workpacks open                                                  | [PLAN_HEALTH.md](plans/account-identity-family-plan/PLAN_HEALTH.md)          |
| `data-custody-storage-plan`               | first-pass; privacy/storage research gate required before implementation | no implementation checklist | 7/7 research workpacks open                                                  | [PLAN_HEALTH.md](plans/data-custody-storage-plan/PLAN_HEALTH.md)             |
| `policy-control-plane-plan`               | first-pass; cross-plan policy handoff research required                  | no implementation checklist | 6/6 research workpacks open                                                  | [PLAN_HEALTH.md](plans/policy-control-plane-plan/PLAN_HEALTH.md)             |
| `payment-subscription-plan`               | first-pass; Stripe/Cloudflare/Firebase decision gate required            | no implementation checklist | 7/7 research workpacks open                                                  | [PLAN_HEALTH.md](plans/payment-subscription-plan/PLAN_HEALTH.md)             |
| `remote-access-plan`                      | first-pass; remote screen/control/security research required             | no implementation checklist | 6/6 research workpacks open                                                  | [PLAN_HEALTH.md](plans/remote-access-plan/PLAN_HEALTH.md)                    |
