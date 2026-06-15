# Workpack 03 - Child macOS Service Package

<!-- agent-capsule -->

> Agent Capsule
> Plan: `child-agent-runtime-distribution-plan`
> Workpack: `03-child-macos-service-package`
> Kind: workpack route and proof contract.

<!-- /agent-capsule -->

Purpose: define the child macOS package, launchd lifecycle, notarization, and uninstall proof boundary.

## Owns

- macOS child package shape
- launchd lifecycle and restart state
- notarization and signing state for the child artifact
- uninstall, disable, and removal behavior on macOS

## Must prove

- the package launches through the macOS service boundary
- restart or recovery behavior is honest for the platform
- signing and notarization state are explicit per artifact
- provisioning or entitlement gaps are surfaced as manual-required states
- no child background-service claim exceeds macOS limits

## Failure conditions

- persistent service behavior is claimed without macOS proof
- notarization or provisioning gaps are hidden
- uninstall and disable behavior is not audited
- parent-client parity is implied from the child slice
