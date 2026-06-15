# Parent Client Scope Correction

Purpose: correct the historical desktop-only naming without breaking the folder path.

## Canonical scope

The folder path remains `docs/plans/parent-desktop-runtime-package-plan/` for compatibility, but the canonical meaning is now `parent-client-runtime-distribution-plan`.

This plan owns:

- parent web portal distribution
- parent desktop shell/package distribution
- parent Android package distribution
- parent iOS package distribution
- parent client route bridge contracts
- signing, notarization, and store claims
- update and rollback
- launch smoke
- release artifacts and platform capability matrix

This plan does not own:

- child agent runtime/package distribution
- setup journey
- account provider implementation
- pairing protocol internals
- policy behavior
- billing provider behavior
- child capture/enforcement adapters

## Setup handoff contracts

Setup may request:

```text
parentInstallPackage(platform, channel, accountRef, householdRef, installCodeRef)
parentClientLaunch(platform, installState)
parentClientReadiness(platform, deviceTrustState, packageState, routeState)
```

This plan returns:

```text
artifactState
packageVersion
channel
signingState
storeState
launchTarget
installProofState
updateState
rollbackState
manualRequiredGaps
```

## Compatibility note

The historical desktop-only workpack tree remains in the folder for legacy proof references. New work should use the parent-client workpacks in `WORKPACK_INDEX.md`.
