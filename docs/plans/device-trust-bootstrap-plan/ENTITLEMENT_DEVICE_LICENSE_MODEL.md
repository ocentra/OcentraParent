# Entitlement Device License Model

This document defines the signed entitlement snapshot and how it unlocks a trusted device.

## Fields

| Field | Purpose |
| --- | --- |
| account ref | Binds the entitlement to the account. |
| household ref | Binds the entitlement to the household. |
| device ref | Binds the entitlement to the trusted device. |
| plan / tier | Identifies the purchased or allowed tier. |
| features | Lists enabled product capabilities. |
| channel | Describes the delivery channel or release track. |
| issued at | Shows when the snapshot was minted. |
| expires at | Sets freshness and refresh behavior. |
| grace window | Defines short offline grace, if any. |
| revocation ref | Links the snapshot to revocation state. |
| signature | Proves the snapshot came from the server. |

## Model rules

- The entitlement snapshot is not a trust root. It only works when device trust is already valid.
- A copied binary or copied config cannot replace a signed entitlement snapshot.
- A revoked or expired snapshot must fail closed or enter a clearly labeled grace state.
- Offline grace must be explicit and bounded.
- The local cache must be replaceable by a fresh signed snapshot.

## Negative cases

- Wrong device must not unlock the entitlement.
- Wrong household must not unlock the entitlement.
- Stale or replayed signatures must not unlock the entitlement.
- Revocation must override local cache.
- License state alone must never open device trust or child control.