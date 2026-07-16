# Workpack 02 - Child Windows Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `02-child-windows-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Windows package, service lifecycle, and respawn proof boundary.

## Owns

- Windows child package shape
- service install, start, stop, and restart state
- Windows respawn and recovery truth
- uninstall and cleanup behavior on Windows

## Must prove

- the package installs and launches as a child-agent artifact
- the service lifecycle is honest under start, stop, and restart
- respawn is only claimed when the platform service manager proves it
- uninstall or revoke removes the child authority state as expected
- no parent-client parity claim is made from this slice

## Failure conditions

- respawn is claimed without service-manager proof
- uninstall leaves trusted child behavior behind
- package proof is used to claim parent-client readiness
- manual-required states are hidden

## Execution truth

Current WP02 execution state is `blocked / proof-present`.

What is real:
- The Windows child-agent package artifact is real. `npm run release:package:windows` built the MSI, latest MSI alias, manifest, checksum sidecars, and bootstrap installer from the real workspace.
- The Windows proof harness now records distinct install, start, stop, restart, respawn, and uninstall-authority-cleanup states.
- Respawn is only modeled from Windows service-manager failure-action state and is not inferred from package build, WiX authoring, or WinSW XML alone.

What is blocked/manual-required on this host:
- Install, start, stop, restart, uninstall, and live respawn execution remain blocked by `admin-required`.
- The host proof run was non-elevated, so the lifecycle harness correctly left those states at `not-run` instead of inventing success.
- Reboot recovery remains manual-required and unproved in this workpack.

Non-claims that remain explicit:
- Parent-client parity is not claimed from this slice.
- Parent-authorized revoke parity is not claimed from this slice; that remains separate uninstall/revocation scope.

Proof root:
- `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/`
- Current blocked install-attempt proof: `test-results/windows-package-lifecycle-proof/2026-06-28T20-18-36-351Z/proof.json`
- Current artifact-only proof: `test-results/windows-package-lifecycle-proof/2026-06-28T20-18-36-351Z/proof.json`
