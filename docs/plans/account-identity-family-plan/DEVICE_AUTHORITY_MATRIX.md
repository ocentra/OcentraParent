# Device Authority Matrix

## Scope

This matrix defines who can do what on household devices, and which extra checks are required.

## Authority Dimensions

- Actor identity
- Household membership
- Role
- Device membership
- Device trust state
- Session freshness
- Capability grant
- Revocation status
- Audit requirement
- Elevated confirmation

## Action Matrix

| Action | Allowed roles | Required device state | Required session state | Extra checks | Notes |
| --- | --- | --- | --- | --- | --- |
| Pair child device | parent-owner, co-parent-guardian | Trusted parent-controller | Fresh | Child profile bound, audit required | Starts child-device membership. |
| Revoke child device | parent-owner, co-parent-guardian | Trusted parent-controller | Fresh | Elevated confirmation, audit required | Revocation must be explicit. |
| View child status | parent-owner, co-parent-guardian, observer | Trusted parent-controller or parent-observer | Fresh not required | Audit not required | Read-only only. |
| Change policy | parent-owner, co-parent-guardian | Trusted parent-controller | Fresh | Child-profile/device scope bound, audit required | Not allowed from observer role. |
| Start remote view | parent-owner, co-parent-guardian, observer | Trusted parent-controller or parent-observer | Fresh | Capability grant, audit required | Observer may view only. |
| Start remote control | parent-owner, co-parent-guardian | Trusted parent-controller | Fresh | Capability grant, elevated confirmation, audit required | Never observer-only. |
| Export/delete data | parent-owner only | Trusted parent-controller | Fresh | Elevated confirmation, audit required | Support-admin does not get this by default. |
| Manage billing | parent-owner only | Trusted parent-controller | Fresh | Elevated confirmation, audit required | Billing is owner-only. |

## Explicit Denials

- Support-admin is denied direct device authority.
- Child-device-agent is denied parent device authority.
- Observer is denied write authority.
- A stale, revoked, disabled, or wrong-household device is denied before the action executes.

## Failure Conditions

- Any action that succeeds because the actor knows a device id is a failure.
- Any action that succeeds with a revoked or wrong-household device is a failure.
- Any action that writes policy or billing from an observer path is a failure.
