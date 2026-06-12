<!-- agent-capsule -->

> Agent Capsule
> Doc: Source Doc Router
> Kind: global docs router/index; read to choose a smaller route, then stop.
> Read when: Only when this exact doc is named by the active route, index, feature doc, or assigned workpack.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the owning feature/plan/checklist/proof route that makes the claim current.

<!-- /agent-capsule -->

# Source Doc Router

Root-level capability guides, schema proposals, inventories, and UI notes are reference docs. Do not read all of them; route by keyword.

| Doc                                                                                                                        | Route                                                           |    Size | Read when                                                        |
| -------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------- | ------: | ---------------------------------------------------------------- |
| [app-control-capability-guide.md](plans/app-game-plan/workpacks/app-control-capability-guide.md)                           | app-game-plan, app-plan, v0-8-enforcement-control-plan          |  37,592 | Open only when the selected plan/workpack references this topic. |
| [app-control-schema-proposal.md](plans/app-game-plan/workpacks/app-control-schema-proposal.md)                             | app-game-plan, app-plan, v0-8-enforcement-control-plan          |  49,513 | Open only when the selected plan/workpack references this topic. |
| [app-control-settings-inventory.md](plans/app-game-plan/workpacks/app-control-settings-inventory.md)                       | app-game-plan, app-plan, v0-8-enforcement-control-plan          | 309,053 | Open only when the selected plan/workpack references this topic. |
| [browser-control-1057-settings-inventory.md](plans/browser-plan/workpacks/browser-control-1057-settings-inventory.md)      | browser-plan, v0-8-enforcement-control-plan                     | 629,370 | Open only when the selected plan/workpack references this topic. |
| [browser-control-coverage-matrix.md](plans/browser-plan/workpacks/browser-control-coverage-matrix.md)                      | browser-plan, v0-8-enforcement-control-plan                     |  13,822 | Open only when the selected plan/workpack references this topic. |
| [browser-control-schema-proposal.md](plans/browser-plan/workpacks/browser-control-schema-proposal.md)                      | browser-plan, v0-8-enforcement-control-plan                     |  52,153 | Open only when the selected plan/workpack references this topic. |
| [browser-policy-questionnaire-forest-v1.md](plans/browser-plan/workpacks/browser-policy-questionnaire-forest-v1.md)        | ai-plan, browser-plan                                           |  28,405 | Open only when the selected plan/workpack references this topic. |
| [browser-policy-settings-catalog.md](plans/browser-plan/workpacks/browser-policy-settings-catalog.md)                      | browser-plan                                                    |  46,035 | Open only when the selected plan/workpack references this topic. |
| [competitor-capability-map.md](competitor-capability-map.md)                                                               | reference                                                       |  15,149 | Open only when the selected plan/workpack references this topic. |
| [data and AI Ui plan.md](plans/data-custody-storage-plan/workpacks/data%20and%20AI%20Ui%20plan.md)                         | data-custody-storage-plan, ai-plan, lan-plan                    |  30,420 | Open only when the selected plan/workpack references this topic. |
| [device-location-tracking-capability-guide.md](plans/tracking-plan/workpacks/device-location-tracking-capability-guide.md) | tracking-plan                                                   |  32,365 | Open only when the selected plan/workpack references this topic. |
| [device-location-tracking-schema-proposal.md](plans/tracking-plan/workpacks/device-location-tracking-schema-proposal.md)   | tracking-plan                                                   |  45,563 | Open only when the selected plan/workpack references this topic. |
| [feature-expectations.md](feature-expectations.md)                                                                         | reference                                                       |   5,282 | Open only when the selected plan/workpack references this topic. |
| [feature-list.md](feature-list.md)                                                                                         | product index                                                   |  18,195 | Open only when the selected plan/workpack references this topic. |
| [full-platform-portal-ai-execution-plan.md](plans/ai-plan/workpacks/full-platform-portal-ai-execution-plan.md)             | ai-plan, lan-plan, portal-ux-household-surfaces-plan            |  45,092 | Open only when the selected plan/workpack references this topic. |
| [game-control-capability-guide.md](plans/app-game-plan/workpacks/game-control-capability-guide.md)                         | app-game-plan, v0-8-enforcement-control-plan                    |  37,002 | Open only when the selected plan/workpack references this topic. |
| [game-control-schema-proposal.md](plans/app-game-plan/workpacks/game-control-schema-proposal.md)                           | app-game-plan, v0-8-enforcement-control-plan                    |  57,710 | Open only when the selected plan/workpack references this topic. |
| [game-control-settings-inventory.md](plans/app-game-plan/workpacks/game-control-settings-inventory.md)                     | app-game-plan, v0-8-enforcement-control-plan                    |  28,005 | Open only when the selected plan/workpack references this topic. |
| [manage UI proof checklist.md](plans/portal-ux-household-surfaces-plan/workpacks/manage%20UI%20proof%20checklist.md)       | portal-ux-household-surfaces-plan, policy-control-plane-plan    |   5,646 | Open only when the selected plan/workpack references this topic. |
| [managed-unmanaged-browser.md](plans/browser-plan/workpacks/managed-unmanaged-browser.md)                                  | browser-plan                                                    |  31,870 | Open only when the selected plan/workpack references this topic. |
| [network-control-capability-guide.md](plans/network-plan/workpacks/network-control-capability-guide.md)                    | network-plan, v0-8-enforcement-control-plan                     |  34,401 | Open only when the selected plan/workpack references this topic. |
| [network-control-schema-proposal.md](plans/network-plan/workpacks/network-control-schema-proposal.md)                      | network-plan, v0-8-enforcement-control-plan                     |  46,311 | Open only when the selected plan/workpack references this topic. |
| [network-control-settings-inventory.md](plans/network-plan/workpacks/network-control-settings-inventory.md)                | network-plan, v0-8-enforcement-control-plan                     | 356,081 | Open only when the selected plan/workpack references this topic. |
| [policy Ui fix.md](plans/portal-ux-household-surfaces-plan/workpacks/policy%20Ui%20fix.md)                                 | portal-ux-household-surfaces-plan, policy-control-plane-plan    |  22,692 | Open only when the selected plan/workpack references this topic. |
| [portal and account Ui fix.md](plans/portal-ux-household-surfaces-plan/workpacks/portal%20and%20account%20Ui%20fix.md)     | portal-ux-household-surfaces-plan, account-identity-family-plan |  26,494 | Open only when the selected plan/workpack references this topic. |
| [product-capability-checklist.md](product-capability-checklist.md)                                                         | product index                                                   | 136,247 | Open only when the selected plan/workpack references this topic. |
| [product-constitution.md](product-constitution.md)                                                                         | reference                                                       |  11,021 | Open only when the selected plan/workpack references this topic. |
| [product-roadmap.md](product-roadmap.md)                                                                                   | product index                                                   |  62,086 | Open only when the selected plan/workpack references this topic. |
| [screen-control-settings-inventory.md](plans/screen-plan/workpacks/screen-control-settings-inventory.md)                   | screen-plan, v0-8-enforcement-control-plan                      | 473,192 | Open only when the selected plan/workpack references this topic. |
| [screen-evidence-analysis-capability-guide.md](plans/screen-plan/workpacks/screen-evidence-analysis-capability-guide.md)   | eventing-plan, screen-plan                                      |  31,198 | Open only when the selected plan/workpack references this topic. |
| [screen-evidence-analysis-schema-proposal.md](plans/screen-plan/workpacks/screen-evidence-analysis-schema-proposal.md)     | eventing-plan, screen-plan                                      |  62,865 | Open only when the selected plan/workpack references this topic. |
| [tracking-control-settings-inventory.md](plans/tracking-plan/workpacks/tracking-control-settings-inventory.md)             | tracking-plan, v0-8-enforcement-control-plan                    | 325,445 | Open only when the selected plan/workpack references this topic. |
