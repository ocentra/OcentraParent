# Workpack 04 - Child Linux Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `04-child-linux-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child Linux package, service-manager lifecycle, and package proof boundary.

## Owns

- Linux child package shape
- service-manager install and restart truth
- package signing and distribution state
- uninstall and daemon cleanup behavior on Linux

## Must prove

- the package installs through the intended Linux distribution path
- the service manager start/stop/restart state is honest
- platform-specific package manager gaps are explicit
- respawn or recovery claims match Linux service-manager proof
- no generic "Linux support" claim hides distro limits

## Failure conditions

- respawn is claimed without service-manager proof
- package proof is used to claim macOS or Windows readiness
- distro-specific gaps are hidden
- manual-required states are omitted
