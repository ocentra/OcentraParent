# Recovery Reset Model

This document defines the encrypted recovery and re-pair path.

## Rule

Recovery is not account login. Recovery is the process of restoring a trusted device or re-pairing a device after loss, revocation, tamper, or reset.

## Required shape

- Recovery material must be encrypted.
- Recovery must be household-bound.
- Recovery must be parent-authorized.
- Recovery must be able to rebuild trust state without exposing plaintext trust keys.
- Recovery must preserve revocation history.

## Recovery paths

| Situation | Expected behavior |
| --- | --- |
| Parent lost the trusted device | Use the encrypted recovery bundle or a reset path to bootstrap a new trusted device. |
| Child device replaced | Re-pair the child device after parent step-up. |
| Trust state revoked | Require a new bootstrap; stale trust cannot be resurrected. |
| Bundle is corrupted | Fail closed and ask for a fresh recovery or re-pair flow. |
| Wrong household tries to restore | Reject the restore. |

## Negative cases

- A recovery bundle cannot unlock a different household.
- A recovery bundle cannot ignore revocation.
- A partial restore cannot silently become a new trust root.
- A device reset cannot be treated as proof of ownership.

## UI implication

The UI should distinguish between sign-in, trust recovery, device reset, and household transfer. The user must know when they are rebuilding trust, not just logging in.