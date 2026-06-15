# Signed EntitlementSnapshot Model

Purpose: define the derived artifact the product can trust on-device after the server has decided the entitlement state.

## Snapshot fields

| Field                    | Meaning                                                  |
| ------------------------ | -------------------------------------------------------- |
| `householdId`            | Billing household or parent account scope.               |
| `parentAccountId`        | Parent who owns the billing relationship.                |
| `baseChildSeats`         | Starter bundle child-device count.                       |
| `paidChildSeats`         | Monthly paid add-on seats.                               |
| `referralSeats`          | Seats created by qualified referral credits.             |
| `graceSeats`             | Time-bound continuation seats.                           |
| `providerMode`           | Stripe, Razorpay, PayPal, store, or manual invoice mode. |
| `region`                 | Launch region or market bucket.                          |
| `deviceBindingId`        | Trusted device binding identifier.                       |
| `issuedAt` / `expiresAt` | Snapshot lifetime.                                       |
| `snapshotVersion`        | Server-side schema version.                              |
| `signature`              | Server signature over the snapshot payload.              |

## Validation rules

- The app must verify the signature before trusting the snapshot.
- The snapshot must match the household and device binding.
- Expired or revoked snapshots must be rejected.
- The snapshot must not contain child names, child activity, screenshots, or provider secrets.

## Failure conditions

- Do not treat the snapshot as the root of trust.
- Do not let a snapshot outlive the ledger state that generated it.
- Do not reuse a snapshot across households or devices.
