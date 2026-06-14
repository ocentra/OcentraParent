# Setup Install Provisioning Plan Decisions

## SIP-001: Setup owns the journey, not adjacent implementations

The `setup-install-provisioning-plan` owns the first-run state graph and user journey. It does not own auth provider implementation, installer mechanics, package signing, LAN protocol internals, data custody implementation, or portal component internals.

## SIP-002: Public site data boundary

`family.ocentra.ca` is a public information, download, and account-entry surface by default. It must not collect child activity data. It may route to registration/login, invite/code entry, download, support, privacy, status, and install-help flows. Any account, contact, or analytics collection must be explicit, minimized, documented, and routed to the owning plan.

## SIP-003: Installed is not ready

Installed does not mean provisioned. Provisioned does not mean paired. Paired does not mean trusted. Trusted does not mean policy-ready. Policy-ready does not mean all platform capabilities are supported.

## SIP-004: Parent app and child agent are different products

Parent desktop/mobile app install and child agent install must be tracked separately on every platform.

## SIP-005: Platform states are explicit

Every platform/install surface must use one of:

- `notImplemented`
- `previewOnly`
- `manualRequired`
- `readyForTest`
- `productionReady`
- `unsupported`
- `blocked`

## SIP-006: Pairing is an authority transition

Pairing changes a child device from discovered/untrusted/pending into household-bound trusted state only after account, household, device authority, and protocol proof agree.

Pairing must reject:

- expired code
- revoked code
- replayed code
- wrong household
- wrong child profile
- wrong device
- anonymous device
- stale signed hello
- revoked child device
- missing parent role
- missing session freshness

## SIP-007: Child disclosure and permissions

Sensitive child-side features require parent-visible and child-visible disclosure and permission state before they can be called ready.

Sensitive areas:

- screen capture / screen analysis
- browser URL evidence
- app/game evidence
- network evidence
- location / tracking
- notifications
- remote view / control
- background service
- tamper / uninstall resistance
- device owner / managed profile

## SIP-008: Setup telemetry and diagnostics are not child activity

Setup diagnostics may include:

- install status
- version
- OS/platform
- permission state
- pairing state
- route state
- error codes
- manual-required reason
- support refs

Setup diagnostics must not include:

- child activity
- raw evidence
- screenshots
- browser URLs
- app usage
- network payload
- location history
- parent rules
- provider secrets
- raw logs with tokens

## SIP-009: Bootstrap codes are separate and single-purpose

The parent install bootstrap code and the child pairing bootstrap code are different authority tokens. They are single-purpose, short-lived, revocable, replay-protected, and never equivalent to login sessions.

The parent bootstrap code allows an authorized parent to download and install the correct parent bootstrap/full package path for a household setup flow. It must not grant child data access, policy authority, data export/delete authority, remote control authority, or child-device trust.

The child pairing bootstrap code allows a child device bootstrap installer to join a pending household child-device setup flow after the parent portal creates the pairing code. It must not grant parent account login, parent portal session, data export/delete authority, policy mutation authority, remote control authority, or trust without parent confirmation.
