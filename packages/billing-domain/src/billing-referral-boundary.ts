import {
  type Infer,
  NonEmptyStringSchema,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  BillingAuditReferenceSchema,
  NonNegativeBillingCountSchema,
} from './billing-entitlement-values.js';

export const BillingReferralInviteStateSchema = withParser(
  Schema.Literal(
    'invite-created',
    'invite-opened',
    'qualified-credit-granted',
    'fraud-review'
  )
);

export const BillingReferralInviteResultStatusSchema = withParser(
  Schema.Literal('accepted', 'rejected', 'manual-review')
);

export const BillingReferralInviteRejectionReasonSchema = withParser(
  Schema.Literal(
    'self-referral-rejected',
    'same-household-rejected',
    'same-device-farm-rejected',
    'same-payment-method-manual-review',
    'fraud-review'
  )
);

export const BillingReferralSubjectSchema = brandedNonEmptyStringSchema(
  'BillingReferralSubject'
);
export const BillingReferralRequestIdSchema = brandedNonEmptyStringSchema(
  'BillingReferralRequestId'
);
export const BillingReferralInviteIdSchema = brandedNonEmptyStringSchema(
  'BillingReferralInviteId'
);

export const BillingReferralCodeSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        /^[A-Z0-9-]+$/u.test(value) ||
        'Expected billing referral codes to stay uppercase and hyphenated'
    )
  )
);

export const BillingReferralInvitedIdentifierSchema = withParser(
  NonEmptyStringSchema.pipe(
    Schema.filter(
      (value) =>
        !value.includes('child-') ||
        'Expected referral invite identifiers to stay free of child-private tokens'
    )
  )
);

export const BillingReferralInviteSummarySchema = withParser(
  Schema.Struct({
    inviteId: BillingReferralInviteIdSchema,
    inviteState: BillingReferralInviteStateSchema,
    referralCode: BillingReferralCodeSchema,
    invitedIdentifier: BillingReferralInvitedIdentifierSchema,
    auditReference: BillingAuditReferenceSchema,
    updatedAt: ParentTimestampSchema,
  })
);

export const BillingReferralSummarySchema = withParser(
  Schema.Struct({
    subject: BillingReferralSubjectSchema,
    referralCode: BillingReferralCodeSchema,
    availableCredits: NonNegativeBillingCountSchema,
    activeReferredParents: NonNegativeBillingCountSchema,
    pendingInvites: NonNegativeBillingCountSchema,
    invites: Schema.Array(BillingReferralInviteSummarySchema),
    auditReference: BillingAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (summary) =>
        summary.availableCredits === summary.activeReferredParents ||
        'Expected referral credits to match active referred parent count'
    )
  )
);

export const BillingReferralInviteResultSchema = withParser(
  Schema.Struct({
    requestId: BillingReferralRequestIdSchema,
    status: BillingReferralInviteResultStatusSchema,
    inviteState: Schema.Union(BillingReferralInviteStateSchema, Schema.Null),
    referralCode: Schema.Union(BillingReferralCodeSchema, Schema.Null),
    rejectionReason: Schema.Union(
      BillingReferralInviteRejectionReasonSchema,
      Schema.Null
    ),
    auditReference: BillingAuditReferenceSchema,
  }).pipe(
    Schema.filter(
      (result) =>
        result.status !== 'accepted' ||
        (result.inviteState !== null &&
          result.rejectionReason === null &&
          result.referralCode !== null) ||
        'Expected accepted referral invites to carry an invite state and referral code without a rejection reason'
    ),
    Schema.filter(
      (result) =>
        result.status !== 'rejected' ||
        ((result.rejectionReason === 'self-referral-rejected' ||
          result.rejectionReason === 'same-household-rejected' ||
          result.rejectionReason === 'same-device-farm-rejected') &&
          result.referralCode !== null) ||
        'Expected rejected referral abuse outcomes to stay distinct from manual-review reasons'
    ),
    Schema.filter(
      (result) =>
        result.status !== 'manual-review' ||
        (result.inviteState === 'fraud-review' &&
          (result.rejectionReason === 'same-payment-method-manual-review' ||
            result.rejectionReason === 'fraud-review') &&
          result.referralCode !== null) ||
        'Expected manual-review referral abuse outcomes to keep fraud-review state with an explicit review reason'
    )
  )
);

export type BillingReferralInviteState = Infer<
  typeof BillingReferralInviteStateSchema
>;
export type BillingReferralInviteResultStatus = Infer<
  typeof BillingReferralInviteResultStatusSchema
>;
export type BillingReferralInviteRejectionReason = Infer<
  typeof BillingReferralInviteRejectionReasonSchema
>;
export type BillingReferralInviteSummary = Infer<
  typeof BillingReferralInviteSummarySchema
>;
export type BillingReferralSummary = Infer<typeof BillingReferralSummarySchema>;
export type BillingReferralInviteResult = Infer<
  typeof BillingReferralInviteResultSchema
>;
