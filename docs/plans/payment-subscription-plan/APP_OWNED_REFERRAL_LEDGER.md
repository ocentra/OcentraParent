# App-Owned Referral Ledger

Purpose: define the canonical referral history owned by the app.

Current Rust owner: referral-credit totals that affect entitlement delivery must be consumed by the Rust-owned entitlement snapshot derivation, not recreated in TypeScript.

## Ledger entries

| Entry | Meaning | Required fields |
| --- | --- | --- |
| `ReferralInvite` | A referral code or link was created. | `referralId`, `referrerAccountId`, `inviteType`, `createdAt` |
| `ReferralRelationship` | A referred household or parent relationship exists. | `referralId`, `referredAccountId`, `relationshipState`, `createdAt` |
| `ReferralQualification` | The referred household satisfied the qualification rules. | `referralId`, `qualifiedAt`, `qualificationReason` |
| `ReferralCredit` | Child-device credit was granted. | `referralId`, `creditUnits`, `grantedAt`, `expiresAt` |
| `ReferralCreditLedgerEntry` | A credit change or adjustment was recorded. | `referralId`, `deltaUnits`, `reason`, `actorRef`, `createdAt` |
| `ReferralCreditProjection` | Current qualified referral-credit total consumed by entitlement delivery. | `referrerAccountId`, `activeReferralCredits`, `revokedReferralCredits`, `updatedAt` |
| `ReferralAbuseSignal` | A fraud or abuse signal was detected. | `referralId`, `signalType`, `detailsRef`, `createdAt` |

## Rules

- Referral invites are separate from household invites.
- Referral credits are non-cash and non-transferable.
- Qualification and revocation decisions must be explainable from the ledger alone.
- Anti-abuse flags belong in the ledger, not just the UI.
- Referral credit loss must recalculate the effective child-device limit before the next signed entitlement snapshot is issued.

## Failure conditions

- Do not let referral records contain child names, child activity, screenshots, or policy details.
- Do not collapse referral and household membership into one row type.
- Do not delete historical referral records when a referral loses qualification.
