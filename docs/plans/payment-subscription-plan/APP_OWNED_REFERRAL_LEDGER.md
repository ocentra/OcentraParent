# App-Owned Referral Ledger

Purpose: define the canonical referral history owned by the app.

## Ledger entries

| Entry | Meaning | Required fields |
| --- | --- | --- |
| `ReferralInvite` | A referral code or link was created. | `referralId`, `referrerAccountId`, `inviteType`, `createdAt` |
| `ReferralRelationship` | A referred household or parent relationship exists. | `referralId`, `referredAccountId`, `relationshipState`, `createdAt` |
| `ReferralQualification` | The referred household satisfied the qualification rules. | `referralId`, `qualifiedAt`, `qualificationReason` |
| `ReferralCredit` | Child-device credit was granted. | `referralId`, `creditUnits`, `grantedAt`, `expiresAt` |
| `ReferralCreditLedgerEntry` | A credit change or adjustment was recorded. | `referralId`, `deltaUnits`, `reason`, `actorRef`, `createdAt` |
| `ReferralAbuseSignal` | A fraud or abuse signal was detected. | `referralId`, `signalType`, `detailsRef`, `createdAt` |

## Rules

- Referral invites are separate from household invites.
- Referral credits are non-cash and non-transferable.
- Qualification and revocation decisions must be explainable from the ledger alone.
- Anti-abuse flags belong in the ledger, not just the UI.

## Failure conditions

- Do not let referral records contain child names, child activity, screenshots, or policy details.
- Do not collapse referral and household membership into one row type.
- Do not delete historical referral records when a referral loses qualification.
