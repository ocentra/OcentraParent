<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-client-runtime-distribution-plan`
> Doc: `Parent Client Runtime Distribution Plan Agent Route`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX.md selects the plan.
> Stop rule: Do not continue into sibling plans unless the selected workpack names a handoff.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: If this file changes status or claims, update the assigned workpack, checklist row, and proof path.

<!-- /agent-capsule -->

# Parent Client Runtime Distribution Plan Agent Route

Canonical scope: parent client runtime distribution. This folder keeps the historical `parent-desktop-runtime-package-plan` path, but it now covers parent web, parent desktop, parent Android, and parent iOS distribution boundaries. Child agent runtime/package distribution belongs to `child-agent-runtime-distribution-plan`.

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/parent-desktop-runtime-package-plan`.

## High-density execution contract

- Task: work only the assigned slice for this plan.
- Route first from `PLAN_STATE.md`.
- Choose exactly one workpack.
- Do not inspect sibling plans unless the selected workpack names a handoff.
- Proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
- Do not claim desktop, web, Android, or iOS parity from scaffold or launch smoke alone.

## Research gate

Before DONE or PR_READY, inspect:

- the current repo parent client surfaces in `apps/portal`
- the parent desktop launch and mobile proof scripts in `package.json`
- official docs for Tauri v2 distribution and security capabilities
- Windows signing / MSI / MSIX guidance
- macOS signing / notarization / Developer ID guidance
- Linux package / signing guidance
- Android App Bundle / Play App Signing guidance
- iOS signing / provisioning / TestFlight / App Store guidance
- Cloudflare Pages or Worker deploy docs if the web portal is hosted there

## Decision tree

| If the task is about...                          | Open                                                     |
| ------------------------------------------------ | -------------------------------------------------------- |
| Parent client scope correction and setup handoff | `workpacks/01-parent-client-scope-and-route-boundary.md` |
| Parent web portal distribution                   | `workpacks/02-parent-web-portal-distribution.md`         |
| Parent desktop shell/package                     | `workpacks/03-parent-desktop-shell-package.md`           |
| Parent Android package                           | `workpacks/04-parent-android-package.md`                 |
| Parent iOS package                               | `workpacks/05-parent-ios-package.md`                     |
| Parent local-service route bridge                | `workpacks/06-parent-local-service-route-bridge.md`      |
| Signing/store/notarization matrix                | `workpacks/07-parent-client-signing-store-matrix.md`     |
| Update and rollback                              | `workpacks/08-parent-client-update-rollback.md`          |
| Launch smoke matrix                              | `workpacks/09-parent-client-launch-smoke-matrix.md`      |
| Setup handoff contracts                          | `workpacks/10-setup-handoff-contracts.md`                |
| Proof, CI, and release gate                      | `workpacks/11-proof-ci-release-gate.md`                  |

## Architecture decisions

- Parent client distribution owns parent web, parent desktop, parent Android, and parent iOS artifacts only.
- Child agent package/runtime distribution is a separate plan.
- Setup is a handoff into install state, not package proof.
- Parent web portal is a parent client and must have build, route, auth, cache, and env separation proof.
- Parent desktop shell/package is not product readiness.
- Parent Android and iOS scaffold states remain manual-required until real build, device, and store proof exists.
- Signing, notarization, and store claims must be explicit per artifact.
- Route bridge behavior must stay separate from setup and package claims.

## Handoffs

- `setup-install-provisioning-plan` owns journey, install code, and readiness state.
- `child-agent-runtime-distribution-plan` owns child agent packages and tamper/uninstall proof.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap and local sealed trust.
- `portal-ux-household-surfaces-plan` owns the generic household shell; this plan owns distribution proof for the client artifacts.

## Failure conditions

- Do not claim mobile parity from scaffold-only proof.
- Do not claim desktop readiness from launch smoke alone.
- Do not claim setup completion from package creation alone.
- Do not hide manual-required gaps.
- Do not conflate parent client distribution with child agent runtime distribution.
