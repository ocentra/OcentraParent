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
