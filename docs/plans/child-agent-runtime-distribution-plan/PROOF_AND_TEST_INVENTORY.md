# Proof and Test Inventory

| Workpack | Proof focus                                              | Test focus                                       | Proof location                                      |
| -------- | -------------------------------------------------------- | ------------------------------------------------ | --------------------------------------------------- |
| 01       | child scope correction and parent/child route boundary   | negative contract / route-boundary tests         | `output/child-agent-runtime-distribution-plan-proof/01-child-agent-scope-and-route-boundary/` |
| 02       | child Windows package and service lifecycle              | package + respawn + uninstall negative case      | `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/` |
| 03       | child macOS package and launchd lifecycle                | package + notarization + uninstall negative case | `output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/` |
| 04       | child Linux package and service-manager lifecycle        | package + restart/uninstall negative case        | `output/child-agent-runtime-distribution-plan-proof/04-child-linux-service-package/` |
| 05       | child Android package and device-owner gap proof         | package + install + lifecycle proof              | `output/child-agent-runtime-distribution-plan-proof/05-child-android-agent-package/` |
| 06       | child iOS capability package and provisioning proof      | package + manual-required gap proof              | `output/child-agent-runtime-distribution-plan-proof/06-child-ios-agent-capability-package/` |
| 07       | managed respawn and restart survival                     | platform-specific restart tests                  | `output/child-agent-runtime-distribution-plan-proof/07-child-managed-service-respawn/` |
| 08       | parent-authorized uninstall                              | uninstall / revocation negative cases            | `output/child-agent-runtime-distribution-plan-proof/08-child-parent-authorized-uninstall/` |
| 09       | signing / store / device-owner matrix                    | docs validation + artifact checks                | `output/child-agent-runtime-distribution-plan-proof/09-child-signing-store-device-owner-matrix/` |
| 10       | setup-device-trust handoff                               | request/response contract tests                  | `output/child-agent-runtime-distribution-plan-proof/10-setup-device-trust-handoff/` |
| 11       | proof, CI, and release gate                              | route-sync + PR-ready gate                       | `output/child-agent-runtime-distribution-plan-proof/11-proof-ci-release-gate/` |

## Proof rules

- Collect command logs, negative cases, and artifact pointers.
- Do not store proof inside the plan folder.
- Keep setup proof and package proof separate.
- Do not claim parent client parity from child package proof.
- If a workpack proof root is empty or only carries blocker text, keep that workpack open.
