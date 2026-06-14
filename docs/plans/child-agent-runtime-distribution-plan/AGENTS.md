<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Doc: `AGENTS.md`
> Kind: plan route and local agent contract.
> Read when: First file inside this plan after PLAN_INDEX or FEATURE_ROUTE_INDEX selects the plan.
> Stop rule: Do not continue into sibling docs, broad folders, source trees, or historical checkpoints unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: Route changes require PLAN_STATE.md, WORKPACK_INDEX.md, TEST_PROOF_EXPECTATIONS.md, PLAN_INDEX.md, and FEATURE_ROUTE_INDEX.md to stay aligned.

<!-- /agent-capsule -->

# Child Agent Runtime Distribution Plan Agent Route

Canonical scope: child-agent runtime distribution. This plan owns child Windows, macOS, Linux, Android, and iOS package distribution, managed respawn where the platform supports it, tamper/uninstall proof, signing/store/device-owner matrix, and setup-device-trust handoff. Parent client distribution stays in `parent-client-runtime-distribution-plan`.

Use this file only when `docs/PLAN_INDEX.md` or a hub assignment selects `docs/plans/child-agent-runtime-distribution-plan`.

## High-density execution contract

- Task: work only the assigned slice for this plan.
- Route first from `PLAN_STATE.md`.
- Choose exactly one workpack.
- Do not inspect sibling plans unless the selected workpack names a handoff.
- Proof must contain command log, negative case, artifact path, updated row, and skipped-risk note when applicable.
- Do not claim child tamper/uninstall, device-owner, or respawn behavior from parent client proof.

## Research gate

Before DONE or PR_READY, inspect:

- the current child-service/runtime implementation in the repo
- the package scripts for Windows, macOS, Linux, Android, and iOS distribution
- platform docs for Windows service / restart semantics
- platform docs for macOS launchd and notarization
- platform docs for Linux service managers and package signing
- Android package, service, device-owner, and managed-profile guidance
- iOS distribution and supervision/provisioning limits

## Decision tree

| If the task is about...                               | Open                                                      |
| ----------------------------------------------------- | --------------------------------------------------------- |
| Child scope correction and setup-device-trust handoff | `workpacks/01-child-agent-scope-and-route-boundary.md`    |
| Child Windows package                                 | `workpacks/02-child-windows-service-package.md`           |
| Child macOS package                                   | `workpacks/03-child-macos-service-package.md`             |
| Child Linux package                                   | `workpacks/04-child-linux-service-package.md`             |
| Child Android package                                 | `workpacks/05-child-android-agent-package.md`             |
| Child iOS capability package                          | `workpacks/06-child-ios-agent-capability-package.md`      |
| Managed service respawn                               | `workpacks/07-child-managed-service-respawn.md`           |
| Parent-authorized uninstall                           | `workpacks/08-child-parent-authorized-uninstall.md`       |
| Signing/store/device-owner matrix                     | `workpacks/09-child-signing-store-device-owner-matrix.md` |
| Setup-device-trust handoff                            | `workpacks/10-setup-device-trust-handoff.md`              |
| Proof, CI, and release gate                           | `workpacks/11-proof-ci-release-gate.md`                   |

## Architecture decisions

- Child agent distribution is separate from parent client distribution.
- Windows, macOS, and Linux rows may support managed respawn and uninstall resistance through the platform service manager or package manager.
- Android rows may support device-owner or managed-profile states where the platform allows them.
- iOS rows must stay honest about supervision, provisioning, and background-service limits.
- Signing, store, and device-owner claims must be explicit per artifact.
- Parent-authorized uninstall is a platform and custody claim, not a stealth claim.

## Handoffs

- `setup-install-provisioning-plan` owns the setup journey and device-trust handoff into install state.
- `parent-client-runtime-distribution-plan` owns the parent client distribution boundary.
- `device-trust-bootstrap-plan` owns trusted-device bootstrap and local sealed trust.
- `app-plan` owns the child local-service/runtime implementation surface.

## Failure conditions

- Do not claim parent client parity from child package proof.
- Do not hide platform-specific device-owner or provisioning gaps.
- Do not claim respawn where the platform cannot prove it.
- Do not conflate uninstall resistance with stealth persistence.
- Do not hide manual-required gaps.
