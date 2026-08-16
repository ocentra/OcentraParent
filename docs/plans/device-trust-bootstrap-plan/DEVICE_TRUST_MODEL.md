# Device Trust Model

This document defines the trust authority boundary for parent and child devices.

## Definitions

- Parent account: the human account that can approve device trust, step-up, recovery, and revoke/remove actions.
- Household: the family unit that owns the trust relationship and entitlement state.
- Parent device: a device that has been bootstrapped as trusted for that household.
- Child device: a device that has been paired into the household and may receive standing access or policy delivery, but cannot own the trust root.
- Trust material: sealed local keys, signed entitlement snapshots, and recovery bundle metadata.
- Trust proof: evidence that a device is allowed to act for a household after bootstrap.

## Trust states

| State | Meaning | Allowed next states |
| --- | --- | --- |
| Untrusted | Device has no household trust. | Bootstrap pending, removed. |
| Bootstrap pending | A one-time install or pair flow has started. | Trusted, rejected, timeout. |
| Trusted | Device holds a sealed trust token and can act within its boundary. | Revoked, reset required, refresh. |
| Revoked | Parent removed the device or revoked trust. | Bootstrap pending, removed. |
| Reset required | Trust state exists but must be re-paired after recovery or tamper. | Bootstrap pending, revoked. |

## Invariants

- Login alone does not create device trust.
- Child devices do not mint or own the trust root.
- Trust material is sealed locally or re-derived from an encrypted recovery bundle.
- Trust revocation wins over stale local state.
- Signed entitlement state can unlock features only after the trust state is valid.
- A copied binary, copied config file, or copied database is not trust proof.

## High-risk actions that require step-up

- First parent install trust.
- First child install trust.
- Child device pairing.
- Policy change.
- Remote control grant.
- Data export, delete, or restore.
- Support access grant.
- Co-parent trusted device approval.
- Device revoke or restore.
- Household transfer.
- License or device-trust reset.
- Uninstall authorization.
- Tamper recovery.

## Negative cases

- Wrong household cannot activate a trusted device.
- Wrong device cannot use another device's sealed trust.
- Revoked trust cannot be revived by a stale local cache.
- Login without device trust cannot unlock high-risk actions.