# Proof and Test Inventory

| Workpack | Proof focus                                              | Test focus                                       | Proof location                                      |
| -------- | -------------------------------------------------------- | ------------------------------------------------ | --------------------------------------------------- |
| 01       | scope correction and setup-device-trust handoff contract | negative contract / boundary tests               | `docs/proof/child-agent-runtime-distribution-plan/` |
| 02       | child Windows package and service lifecycle              | package + respawn + uninstall negative case      | `docs/proof/child-agent-runtime-distribution-plan/` |
| 03       | child macOS package and launchd lifecycle                | package + notarization + uninstall negative case | `docs/proof/child-agent-runtime-distribution-plan/` |
| 04       | child Linux package and service-manager lifecycle        | package + restart/uninstall negative case        | `docs/proof/child-agent-runtime-distribution-plan/` |
| 05       | child Android package and device-owner gap proof         | package + install + lifecycle proof              | `docs/proof/child-agent-runtime-distribution-plan/` |
| 06       | child iOS capability package and provisioning proof      | package + manual-required gap proof              | `docs/proof/child-agent-runtime-distribution-plan/` |
| 07       | managed respawn and restart survival                     | platform-specific restart tests                  | `docs/proof/child-agent-runtime-distribution-plan/` |
| 08       | parent-authorized uninstall                              | uninstall / revocation negative cases            | `docs/proof/child-agent-runtime-distribution-plan/` |
| 09       | signing / store / device-owner matrix                    | docs validation + artifact checks                | `docs/proof/child-agent-runtime-distribution-plan/` |
| 10       | setup-device-trust handoff                               | request/response contract tests                  | `docs/proof/child-agent-runtime-distribution-plan/` |
| 11       | proof, CI, and release gate                              | route-sync + PR-ready gate                       | `docs/proof/child-agent-runtime-distribution-plan/` |

## Proof rules

- Collect command logs, negative cases, and artifact pointers.
- Do not store proof inside the plan folder.
- Keep setup proof and package proof separate.
- Do not claim parent client parity from child package proof.
