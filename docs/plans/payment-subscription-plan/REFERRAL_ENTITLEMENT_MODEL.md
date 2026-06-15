# Referral Entitlement Model

Purpose: define how referral invites create child-device credit without collapsing household invites into the same concept.

## Invite types

- Household invite: join an already paired household. This is identity and access management.
- Referral invite: bring a new paying household or parent into the product. This is growth accounting.

## Referral lifecycle

| State     | Meaning                                          | Next step                            |
| --------- | ------------------------------------------------ | ------------------------------------ |
| Issued    | Referral code or link exists.                    | Accepted or expired.                 |
| Accepted  | Referred parent or household started signup.     | Pending qualification.               |
| Qualified | The referred household met the activation rules. | Credit grant.                        |
| Credited  | Child-device credit is active.                   | Renewal or revocation.               |
| Revoked   | Qualification failed or abuse was detected.      | Grace or blocked expansion.          |
| Expired   | Credit window ended.                             | No future credit until re-qualified. |

## Anti-abuse rules

- No self-referrals.
- No duplicate referral credits for the same referred household.
- Same billing instrument, same payment fingerprint, or obvious device clusters should be treated as abuse signals, not entitlement signals.
- Thresholds must be configured in policy, not hard-coded into UI.

## Credit behavior

- Referral credits may unlock child-device entitlement while the referred household remains qualified.
- When the referred household loses qualification, future credit stops.
- Historical referral records remain intact for audit and support.
- Losing a referral does not delete billing history; it changes entitlement state.

## Failure conditions

- Do not treat referral codes as household invites.
- Do not give cash-equivalent payouts unless a later product decision explicitly adds them.
- Do not let referral metadata contain child data.
