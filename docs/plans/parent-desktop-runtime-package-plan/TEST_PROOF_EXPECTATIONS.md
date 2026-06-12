# Parent Desktop Runtime Package Plan Test and Proof Expectations

<!-- agent-capsule -->

> Agent Capsule
> Plan: `parent-desktop-runtime-package-plan`
> Doc: `Parent Desktop Runtime Package Plan Test and Proof Expectations`
> Kind: plan-local test and proof decision tree.
> Read when: After the assigned workpack/checklist row is known; use to choose required tests/proof.
> Stop rule: Do not continue into broader docs unless this file gives an explicit next path.
> Proves: only the local scope, status, route, or contract stated by this file and its named proof/checklist rows.
> Does not prove: sibling plan completion, implementation correctness, product status, PR readiness, or broad DONE unless routed proof says so.
> Proof rule: This file defines required local tests/proof; missing tests keep rows open.

<!-- /agent-capsule -->

Use this after the assigned desktop runtime/package workpack is known. Proof must separate shell, local service connection, LAN controller state, packaging, signing claims, diagnostics, and release branch boundaries.

## Where tests should live

When the desktop runtime implementation crate/package exists, tests belong under its test tree and proof output under its proof folder. Until then, colocate with the owning Tauri/runtime/package scripts and record paths in the workpack and `PROOF_INDEX.md`.

## Decision Tree

| If the assigned work is...                      | Read next                       | Expected tests or proof                                                                                                       |
| ----------------------------------------------- | ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| WP01 Tauri shell boundary                       | assigned workpack               | command contract tests, localhost/origin checks, no inline route/string drift, shell launch smoke.                            |
| WP02 local service connection                   | assigned workpack               | connection lifecycle, token/origin/header checks, retry/timeout, service unavailable states.                                  |
| WP03 LAN route/controller                       | assigned workpack               | route state transitions, stale/offline/degraded states, authZ, replay/idempotency.                                            |
| WP04-WP06 observer/mobile/custody labels        | assigned workpack               | read-only observer proof, custody/source labels, redaction, no privileged action leakage.                                     |
| WP07-WP10 installer/package/update/signing      | assigned workpack; release flow | package smoke, install/launch/uninstall, rollback, version/tag alignment, honest signing/notarization/store claim boundaries. |
| WP11-WP12 support/privacy docs                  | assigned workpack               | support bundle redaction, logging assertion, privacy doc sync proof.                                                          |
| WP13-WP18 launch/build/platform/CI/manual proof | assigned workpack               | dev/build script smoke, platform matrix, artifact proof, manual platform runbook evidence.                                    |
| WP19-WP20 checklist/PR/CI gate                  | `PROOF_INDEX.md`                | product checklist sync, selected risk rows, CI/rollout validation list.                                                       |

## Expected test/proof inventory

Use these names as proof intent labels in the assigned workpack/proof note. Implementers choose the actual crate/package test names after the owning implementation boundary exists.

- `runtime-package.install-upgrade-uninstall`: installer paths cover install, upgrade, uninstall, rollback, and cleanup.
- `runtime-package.service.lifecycle`: service start/stop/restart/crash recovery and log proof are recorded.
- `runtime-package.version.alignment`: app/package/version/tag/channel alignment prevents release drift.
- `runtime-package.platform-smoke.matrix`: Windows/Linux/macOS/mobile artifact smoke is honest about platform scope.
- `runtime-package.signing.manual-state`: signing/notarization/store/device-owner credentials are proved or marked manual-required.
- `runtime-package.launch.localhost-security`: launch and localhost service boundaries prove origin/port/security expectations.
- `runtime-package.no-product-readiness-claim`: package smoke does not move feature capability status by itself.

## Required proof contents

- Install/build/launch command logs when packaging is touched.
- Origin/header/token proof for local service calls.
- Artifact path, platform, version, and limitation notes.
- Rollback/uninstall evidence for package/update work.

## Failure conditions

Do not claim DONE or PR_READY if any apply:

- The expected test/proof row for the touched work type is missing.
- The implementation crate/package test folder does not exist and the missing expected location is not recorded.
- Only happy-path tests pass for a trust, policy, persistence, protocol, UI, AI, platform, security, performance, or observability boundary.
- A product/checklist row moved without command logs and proof artifact path.
- A manual-required/platform limitation was converted into a runtime capability claim.
- A proof artifact lacks negative cases, logs/traces where relevant, or exact workpack/checklist linkage.
- A sibling plan or broad source tree was read without a route reason recorded in the workpack/proof note.
