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

## Live source truth

Status: source partial; implementation correction and all test/validation/proof gates remain open.

The macOS builder produces child-named package artifacts, installs the child binary under child paths, and writes a launchd payload whose values use the child identity. The checked-in plist source filename remains parent-labelled. The package is unsigned; Apple signing, notarization, stapling, disable/remove cleanup, and a fail-closed lifecycle result are not implemented.

The launchd declaration can start the binary, but default child-service startup supplies no current Device Trust source. No authenticated product ingress or external health endpoint is composed. `RunAtLoad` and `KeepAlive` are declarations, not runtime health or bounded respawn proof.

## Required production source outcome

- consume WP10's reviewed trusted-startup, authenticated-ingress, and external-health boundary;
- use canonical child-owned macOS package/plist identity end to end;
- make signing/notarization inputs and rejection states explicit without embedding credentials;
- own disable, removal, restart/backoff, and cleanup outcomes without upgrading launchd declarations into proof.

Implementation dependency: Child WP10 reviewed implementation. Normal READY/DONE remains strict.

## Expected test-source gap

- canonical child artifact/plist identity;
- current/missing/revoked trust startup and external health;
- real-host install, launchd start, restart, deliberate stop, disable, uninstall, and cleanup;
- signing/notarization success and fail-closed missing/invalid authority states;
- restart-loop and hidden-persistence negative cases.

Historical contract/proof runners and `output/child-agent-runtime-distribution-plan-proof/03-child-macos-service-package/` do not close these source or host-test gaps.
