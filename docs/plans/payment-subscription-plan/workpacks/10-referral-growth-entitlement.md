# Workpack 10: Referral Growth and Entitlement

Purpose: define referral qualification, anti-abuse, credit grant/revoke, and grace behavior.

## Owns

- `REFERRAL_ENTITLEMENT_MODEL.md`
- PSP-009 and PSP-010

## Must prove

- Referral qualification is explicit.
- Self-referral and duplicate referral cases are rejected.
- Qualified referrals grant child-device credit.
- Revocation changes entitlement without deleting history.

## Proof path

- Use `docs/proof/payment-subscription-plan/wp10/` or the owning crate's local proof directory.

## Failure conditions

- The workpack fails if referral and household invites are conflated.
- The workpack fails if referral credit can become cash-like by accident.
- The workpack fails if abuse signals are not auditable.
