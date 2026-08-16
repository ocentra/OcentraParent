# First-Run Setup UI Expectations

## Parent-facing screens

The first-run UI must include:

- Public site / invite entry
- Create account / sign in
- Create or join household
- Parent install link / QR / code
- Parent bootstrap tutorial / agreement
- Parent bootstrap code entry
- Parent package download / install progress
- Parent portal guided setup start
- Create child profile
- Generate child pairing link / QR / code
- Child install instructions
- Waiting for child device
- Child detected / confirm trust
- Permission readiness checklist
- Policy baseline setup
- Data custody status
- Setup complete / setup blocked / manual required

## Child-facing/bootstrap screens

The child bootstrap flow must include:

- Child setup link/code entry
- What Ocentra Parent is
- Who manages this device
- What parent may see/control
- Data/privacy summary
- Permission explanation
- Agreement / consent / disclosure
- Code entry
- Download / install progress
- Permission checklist
- Waiting for parent confirmation
- Setup complete / failed / manual required

## Required UI states

- `notStarted`
- `publicSiteOnly`
- `accountRequired`
- `authenticatedNoHousehold`
- `householdCreated`
- `childProfileCreated`
- `parentAppMissing`
- `parentAppReady`
- `childAgentMissing`
- `childAgentInstalled`
- `childAgentRunning`
- `permissionMissing`
- `pairingPending`
- `pairingFailed`
- `childDeviceTrusted`
- `readinessIncomplete`
- `manualRequired`
- `setupComplete`

## Required degraded and error states

- `expiredInvite`
- `revokedInvite`
- `wrongHousehold`
- `sessionExpired`
- `downloadFailed`
- `installFailed`
- `serviceStopped`
- `permissionDenied`
- `unsupportedPlatform`
- `pairingExpired`
- `pairingReplayed`
- `offlineChild`
- `revokedDevice`
- `providerUnavailable`
- `manualProofRequired`

## UI no-claim language

Use:

- "Downloaded"
- "Installed"
- "Running"
- "Permission missing"
- "Pairing pending"
- "Trusted"
- "Ready for test"
- "Manual proof required"

Do not use:

- "Protected"
- "Fully installed"
- "Production ready"
- "All set"
- "Remote ready"
- "Monitoring active"
- "Works on mobile"

unless corresponding proof exists.
