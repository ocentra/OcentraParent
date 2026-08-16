# Setup State Machine

## Purpose

This document defines the setup bootstrap graph from public entry to parent-confirmed household readiness.

## End-to-end flow

Public site visit
-> invite/link/code entry
-> account/login handoff
-> household create/join
-> parent bootstrap code
-> parent bootstrap installer
-> parent portal launch
-> child pairing bootstrap code
-> child bootstrap installer
-> parent confirmation
-> device trust
-> readiness evaluation
-> recovery or manual-required

## Parent entry states

- `publicSite`
- `inviteLinkOpened`
- `manualCodeEntry`
- `qrCodeScanned`
- `inviteExpired`
- `inviteRevoked`
- `inviteAlreadyUsed`
- `inviteWrongHousehold`
- `accountRequired`
- `accountCreated`
- `signedIn`
- `authenticatedNoHousehold`
- `householdCreated`
- `householdJoined`
- `householdRoleResolved`

## Parent bootstrap states

- `parentInstallLinkReady`
- `parentInstallQrReady`
- `parentBootstrapDownloaded`
- `parentBootstrapLaunched`
- `parentBootstrapTutorialShown`
- `parentBootstrapConsentAccepted`
- `parentBootstrapCodeRequired`
- `parentBootstrapCodeSubmitted`
- `parentBootstrapCodeValid`
- `parentBootstrapCodeExpired`
- `parentBootstrapCodeRevoked`
- `parentBootstrapCodeWrongAccount`
- `parentBootstrapCodeWrongHousehold`
- `parentBootstrapCodeWrongRole`
- `parentFullPackageSelected`
- `parentFullPackageDownloading`
- `parentFullPackageVerified`
- `parentFullPackageInstallStarted`
- `parentFullPackageInstallSucceeded`
- `parentFullPackageInstallFailed`
- `parentPortalLaunched`

## Guided setup states

- `guidedSetupStarted`
- `childProfileRequired`
- `childProfileCreated`
- `childDeviceRequired`
- `childPairingCodeGenerated`
- `childPairingQrDisplayed`
- `childPairingLinkDisplayed`
- `waitingForChildDevice`

## Child bootstrap states

- `childLinkOpened`
- `childBootstrapDownloaded`
- `childBootstrapLaunched`
- `childBootstrapTutorialShown`
- `childDisclosureShown`
- `childConsentAccepted`
- `childPairingCodeRequired`
- `childPairingCodeSubmitted`
- `childPairingCodeValid`
- `childPairingCodeExpired`
- `childPairingCodeRevoked`
- `childPairingCodeReplayed`
- `childPairingCodeWrongHousehold`
- `childPairingCodeWrongProfile`
- `childPairingCodeWrongRole`
- `childFullPackageSelected`
- `childFullPackageDownloading`
- `childFullPackageVerified`
- `childInstallStarted`
- `childInstallSucceeded`
- `childInstallFailed`
- `childServiceRegistered`
- `childServiceRunning`
- `childPermissionChecklistStarted`
- `childPermissionGranted`
- `childPermissionDenied`
- `childPermissionMissing`
- `childSignedHelloEmitted`

## Parent-child trust states

- `parentPortalChildDetected`
- `childPendingParentConfirmation`
- `childProfileAssigned`
- `childDeviceTrusted`
- `childDeviceRejected`
- `childDeviceRevoked`
- `childDeviceOffline`
- `childDeviceStale`
- `policyBaselinePending`
- `policyBaselineApplied`
- `dataCustodyStatusKnown`
- `setupReady`
- `setupReadyWithManualGaps`
- `setupBlocked`
- `manualRequired`

## Code model

### ParentInstallBootstrapCode

Purpose:

- allows an authorized parent to download and install the correct parent bootstrap/full package path for a household setup flow.

Scope:

- account
- household
- role
- platform/channel
- expiry
- single-use or bounded-use
- download/install intent

Must not grant:

- child data access
- policy authority
- data export/delete authority
- remote control authority
- child-device trust

### ChildAgentPairingBootstrapCode

Purpose:

- allows a child device bootstrap installer to join a pending household child-device setup flow after the parent portal creates the pairing code.

Scope:

- household
- child profile or pending child-device slot
- child-device-agent role
- platform/channel
- expiry
- single-use
- pairing intent

Must not grant:

- parent account login
- parent portal session
- data export/delete authority
- policy mutation authority
- remote control authority
- trust without parent confirmation

## Cloud/setup backend role

The setup backend may be Cloudflare Worker/Wrangler-backed, but the plan must stay provider-neutral enough to change later.

It may own:

- public route handling
- invite/code verification
- bootstrap code status
- download manifest selection
- release/channel metadata
- minimal install telemetry/status
- rate-limit state
- redacted audit refs

It must not own:

- child activity data
- raw child evidence
- parent rules as source of truth
- full child device logs
- pairing secrets in plaintext logs
- universal decrypt keys
