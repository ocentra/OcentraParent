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
| 01       | scope correction and setup handoff contract         | route/proof boundary validation            | `output/parent-client-runtime-distribution-plan-proof/01-parent-client-scope-and-route-boundary/` |
| 02       | portal build, route, auth, cache, env separation    | real build + focused vitest + focused Playwright                         | `output/parent-client-runtime-distribution-plan-proof/02-parent-web-portal-distribution/` |
| 03       | desktop shell/package, service bridge, launch smoke | desktop smoke + service boundary           | `output/parent-client-runtime-distribution-plan-proof/03-parent-desktop-shell-package/` |
| 04       | Android package and device proof                    | device install + manual-required gap proof | `output/parent-client-runtime-distribution-plan-proof/04-parent-android-package/` |
| 05       | iOS package and provisioning proof                  | device install + manual-required gap proof | `output/parent-client-runtime-distribution-plan-proof/05-parent-ios-package/` |
| 06       | Rust-owned route snapshot and local-service boundary | focused cargo unit + cargo contract        | `output/parent-client-runtime-distribution-plan-proof/06-parent-local-service-route-bridge/` |
| 07       | signing/store/notarization matrix                   | docs validation + artifact checks          | `output/parent-client-runtime-distribution-plan-proof/07-parent-client-signing-store-matrix/` |
| 08       | update/rollback model                               | rollback / teardown / checksum checks      | `output/parent-client-runtime-distribution-plan-proof/08-parent-client-update-rollback/` |
| 09       | launch smoke matrix                                 | launch/degrade/manual-required checks      | `output/parent-client-runtime-distribution-plan-proof/09-parent-client-launch-smoke-matrix/` |
| 10       | setup handoff contract                              | request/response contract tests            | `output/parent-client-runtime-distribution-plan-proof/10-setup-handoff-contracts/` |
| 11       | proof, CI, and release gate                         | route-sync + PR-ready gate                 | `output/parent-client-runtime-distribution-plan-proof/11-proof-ci-release-gate/` |

## Proof rules

- Collect command logs, negative cases, and artifact pointers.
- Do not store proof inside the plan folder.
- Rust-owned contract and runtime truth stays upstream of any TS presentation edge.
- Keep setup proof and package proof separate.
- Do not claim parity from scaffold or launch smoke alone.
