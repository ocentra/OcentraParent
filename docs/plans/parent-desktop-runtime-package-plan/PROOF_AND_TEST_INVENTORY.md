# Proof and Test Inventory

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `PROOF_AND_TEST_INVENTORY.md`
> Kind: plan reference document.

<!-- /agent-capsule -->

Proof lives outside the plan folder. Use the designated local artifact path for the selected workpack.

| Workpack | Proof focus                                         | Test focus                                 | Proof location                                    |
| -------- | --------------------------------------------------- | ------------------------------------------ | ------------------------------------------------- |
| 01       | scope correction and setup handoff contract         | negative contract / boundary tests         | `docs/proof/parent-desktop-runtime-package-plan/` |
| 02       | portal build, route, auth, cache, env separation    | build + unit + e2e                         | `docs/proof/parent-desktop-runtime-package-plan/` |
| 03       | desktop shell/package, service bridge, launch smoke | desktop smoke + service boundary           | `docs/proof/parent-desktop-runtime-package-plan/` |
| 04       | Android package and device proof                    | device install + manual-required gap proof | `docs/proof/parent-desktop-runtime-package-plan/` |
| 05       | iOS package and provisioning proof                  | device install + manual-required gap proof | `docs/proof/parent-desktop-runtime-package-plan/` |
| 06       | local-service route bridge                          | contract + negative route tests            | `docs/proof/parent-desktop-runtime-package-plan/` |
| 07       | signing/store/notarization matrix                   | docs validation + artifact checks          | `docs/proof/parent-desktop-runtime-package-plan/` |
| 08       | update/rollback model                               | rollback / teardown / checksum checks      | `docs/proof/parent-desktop-runtime-package-plan/` |
| 09       | launch smoke matrix                                 | launch/degrade/manual-required checks      | `docs/proof/parent-desktop-runtime-package-plan/` |
| 10       | setup handoff contract                              | request/response contract tests            | `docs/proof/parent-desktop-runtime-package-plan/` |
| 11       | proof, CI, and release gate                         | route-sync + PR-ready gate                 | `docs/proof/parent-desktop-runtime-package-plan/` |

## Proof rules

- Collect command logs, negative cases, and artifact pointers.
- Do not store proof inside the plan folder.
- Keep setup proof and package proof separate.
- Do not claim parity from scaffold or launch smoke alone.
