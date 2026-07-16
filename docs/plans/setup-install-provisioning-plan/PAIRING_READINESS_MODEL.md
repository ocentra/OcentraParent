# Pairing Readiness Model

## Purpose

Describe the parent bootstrap to child pairing authority chain as a state machine with explicit readiness and recovery.

## Authority chain

Public site or invite/code entry
-> account and household authority
-> parent bootstrap code
-> parent bootstrap installer
-> parent portal
-> child pairing bootstrap code
-> child bootstrap installer
-> parent confirmation
-> device trust
-> readiness evaluation

## Pairing states

- `notStarted`
- `codeGenerated`
- `codeDisplayed`
- `qrDisplayed`
- `linkDisplayed`
- `pendingAcceptance`
- `accepted`
- `expired`
- `revoked`
- `replayedRejected`
- `wrongHouseholdRejected`
- `wrongDeviceRejected`
- `anonymousRejected`
- `trusted`
- `untrusted`
- `offline`
- `recoveryRequired`
- `manualRequired`

## Readiness dimensions

- `accountReady`
- `householdReady`
- `parentAppReady`
- `childProfileReady`
- `childAgentInstalled`
- `childAgentRunning`
- `permissionsReady`
- `pairingReady`
- `deviceTrusted`
- `networkReachable`
- `dataCustodyReady`
- `policyBaselineReady`
- `recoveryPathReady`

## Recovery flows

- lost parent device
- child reinstall
- revoked child device
- wrong account
- offline child
- permission loss
- provider unavailable
- installer failure
- service stopped
- unsupported platform

## Rule

A child device is not trusted until the parent portal confirms or assigns it. Readiness is a matrix, not a boolean.
