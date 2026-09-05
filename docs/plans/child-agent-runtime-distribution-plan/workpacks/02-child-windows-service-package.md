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

## Live source truth

Status: source partial; implementation correction and all test/validation/proof gates remain open.

Committed source includes the `ocentra-child-agent-service` binary, durable journal/tombstone/removal paths, startup recovery, typed readiness, bounded in-process ingress, the Windows builder, MSI authoring, and WinSW definitions with child artifact/service values.

The installed binary still calls default startup, which supplies no `ChildAgentTrustBindingSource`. It therefore remains fail-closed at `TrustBindingManualRequired`. `ChildAgentIngress` is an in-process queue, not an authenticated product transport, and health is not exposed through a shipped external endpoint. The package source/config filenames and deferred lifecycle harness/workflow inputs also retain parent-era identities even where their current contents use child values.

## Required production source outcome

- consume WP10's reviewed trusted-startup, authenticated-ingress, and external-health boundary;
- use canonical child-owned Windows package/service/updater source identities end to end;
- preserve durable custody and explicit removal/audit semantics across install/update/remove;
- keep service-manager lifecycle failure states observable rather than inferred from static MSI/WinSW declarations.

Implementation dependency: Child WP10 reviewed implementation. Normal READY/DONE remains blocked by strict completion gates.

## Expected test-source gap

- current, missing, stale, and revoked trust startup behavior;
- authenticated command ingress and external health reachability;
- child-labelled elevated install/start/stop/restart/uninstall/respawn and reboot recovery;
- durable custody/removal-state preservation and cleanup boundaries;
- negative identity, stale legacy artifact, and service-manager failure cases.

Historical proof under `output/child-agent-runtime-distribution-plan-proof/02-child-windows-service-package/` and `test-results/windows-package-lifecycle-proof/...` is review input only. It does not close this source gap.
