import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

export const PolicyControlApprovalNotificationBoundaryReadModelIdSchema =
  brandedNonEmptyStringSchema('PolicyControlApprovalNotificationBoundaryReadModelId');
export const PolicyControlApprovalNotificationBoundaryEntryIdSchema =
  brandedNonEmptyStringSchema('PolicyControlApprovalNotificationBoundaryEntryId');
export const PolicyControlApprovalNotificationBoundaryReferenceSchema =
  brandedNonEmptyStringSchema('PolicyControlApprovalNotificationBoundaryReference');
export const PolicyControlApprovalNotificationBoundaryRequirementSchema =
  brandedNonEmptyStringSchema('PolicyControlApprovalNotificationBoundaryRequirement');

const BoundaryCountSchema = Schema.Number.pipe(Schema.nonNegative(), Schema.int());
const NullableBoundaryReferenceSchema = Schema.Union(
  PolicyControlApprovalNotificationBoundaryReferenceSchema,
  Schema.Null
);

export const PolicyControlApprovalNotificationStateSchema = withParser(
  Schema.Literal(
    'preview-only',
    'pending-parent-review',
    'approved',
    'denied',
    'modified',
    'expired-request',
    'replay-rejected'
  )
);

export const PolicyControlApprovalNotificationOriginSchema = withParser(
  Schema.Literal('child-request', 'assistant-draft')
);

export const PolicyControlApprovalNotificationKindSchema = withParser(
  Schema.Literal('ask-parent', 'temporary-override', 'bonus-time')
);

export const PolicyControlApprovalNotificationOverrideKindSchema = withParser(
  Schema.Literal('temporary-allow', 'temporary-block', 'bonus-time')
);

const PolicyControlApprovalNotificationBoundaryEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  entryId: PolicyControlApprovalNotificationBoundaryEntryIdSchema,
  notificationState: PolicyControlApprovalNotificationStateSchema,
  origin: PolicyControlApprovalNotificationOriginSchema,
  approvalKind: PolicyControlApprovalNotificationKindSchema,
  requestRef: PolicyControlApprovalNotificationBoundaryReferenceSchema,
  approvalRef: NullableBoundaryReferenceSchema,
  notificationIntentRef: PolicyControlApprovalNotificationBoundaryReferenceSchema,
  overrideRef: NullableBoundaryReferenceSchema,
  overrideKind: Schema.Union(PolicyControlApprovalNotificationOverrideKindSchema, Schema.Null),
  auditRefs: Schema.Array(PolicyControlApprovalNotificationBoundaryReferenceSchema),
  policyContextRefs: Schema.Array(PolicyControlApprovalNotificationBoundaryReferenceSchema),
  manualProofRequirements: Schema.Array(PolicyControlApprovalNotificationBoundaryRequirementSchema),
  parentConfirmationRequired: Schema.Boolean,
  parentReviewed: Schema.Boolean,
  portalQueueVisible: Schema.Boolean,
  policyMutationClaimed: Schema.Literal(false),
  enforcementMutationClaimed: Schema.Literal(false),
  providerDeliveryClaimed: Schema.Literal(false),
  lastUpdatedAt: ParentTimestampSchema,
});

type PolicyControlApprovalNotificationBoundaryEntryCandidate = Infer<
  typeof PolicyControlApprovalNotificationBoundaryEntryBaseSchema
>;

export const PolicyControlApprovalNotificationBoundaryEntrySchema = withParser(
  PolicyControlApprovalNotificationBoundaryEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        policyControlApprovalNotificationEntryIsHonest(entry) ||
        'Expected policy control approval notification rows to preserve preview-only assistant drafts, require parent confirmation before approval/mutation, keep audit refs attached, and avoid provider/policy/enforcement delivery claims'
    )
  )
);

export const PolicyControlApprovalNotificationBoundaryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: PolicyControlApprovalNotificationBoundaryReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(PolicyControlApprovalNotificationBoundaryReferenceSchema),
    returned: BoundaryCountSchema,
    previewOnlyCount: BoundaryCountSchema,
    pendingParentReviewCount: BoundaryCountSchema,
    approvedCount: BoundaryCountSchema,
    deniedCount: BoundaryCountSchema,
    modifiedCount: BoundaryCountSchema,
    expiredRequestCount: BoundaryCountSchema,
    replayRejectedCount: BoundaryCountSchema,
    providerDeliveryClaimed: Schema.Literal(false),
    policyMutationClaimed: Schema.Literal(false),
    enforcementMutationClaimed: Schema.Literal(false),
    assistantAutoApprovalClaimed: Schema.Literal(false),
    entries: Schema.Array(PolicyControlApprovalNotificationBoundaryEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.entryId)).size === readModel.entries.length ||
        'Expected policy control approval notification entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        policyControlApprovalNotificationCountsMatch(readModel) ||
        'Expected policy control approval notification counts to match row states'
    ),
    Schema.filter(
      (readModel) =>
        policyControlApprovalNotificationStatesAreCovered(readModel.entries) ||
        'Expected policy control approval notification boundary to cover preview-only, pending-parent-review, approved, denied, modified, expired-request, and replay-rejected states'
    )
  )
);

function policyControlApprovalNotificationEntryIsHonest(
  entry: PolicyControlApprovalNotificationBoundaryEntryCandidate
): boolean {
  if (entry.auditRefs.length === 0 || entry.policyContextRefs.length === 0) {
    return false;
  }

  switch (entry.notificationState) {
    case 'preview-only':
      return (
        entry.origin === 'assistant-draft' &&
        entry.parentConfirmationRequired &&
        !entry.parentReviewed &&
        entry.portalQueueVisible &&
        entry.approvalRef === null &&
        entry.overrideRef === null &&
        entry.overrideKind === null
      );
    case 'pending-parent-review':
      return (
        !entry.parentConfirmationRequired &&
        !entry.parentReviewed &&
        entry.portalQueueVisible &&
        entry.approvalRef === null &&
        entry.overrideRef === null &&
        entry.overrideKind === null
      );
    case 'approved':
    case 'modified':
      return (
        !entry.parentConfirmationRequired &&
        entry.parentReviewed &&
        entry.portalQueueVisible &&
        entry.approvalRef !== null &&
        entry.overrideRef !== null &&
        entry.overrideKind !== null
      );
    case 'denied':
      return (
        !entry.parentConfirmationRequired &&
        entry.parentReviewed &&
        entry.portalQueueVisible &&
        entry.approvalRef !== null &&
        entry.overrideRef === null &&
        entry.overrideKind === null
      );
    case 'expired-request':
    case 'replay-rejected':
      return (
        !entry.parentConfirmationRequired &&
        !entry.parentReviewed &&
        !entry.portalQueueVisible &&
        entry.approvalRef === null &&
        entry.overrideRef === null &&
        entry.overrideKind === null
      );
  }
}

type PolicyControlApprovalNotificationBoundaryReadModelCounts = {
  returned: number;
  previewOnlyCount: number;
  pendingParentReviewCount: number;
  approvedCount: number;
  deniedCount: number;
  modifiedCount: number;
  expiredRequestCount: number;
  replayRejectedCount: number;
  entries: readonly PolicyControlApprovalNotificationBoundaryEntry[];
};

function policyControlApprovalNotificationCountsMatch(
  readModel: PolicyControlApprovalNotificationBoundaryReadModelCounts
): boolean {
  return (
    readModel.returned === readModel.entries.length &&
    readModel.previewOnlyCount === countEntries(readModel.entries, 'preview-only') &&
    readModel.pendingParentReviewCount === countEntries(readModel.entries, 'pending-parent-review') &&
    readModel.approvedCount === countEntries(readModel.entries, 'approved') &&
    readModel.deniedCount === countEntries(readModel.entries, 'denied') &&
    readModel.modifiedCount === countEntries(readModel.entries, 'modified') &&
    readModel.expiredRequestCount === countEntries(readModel.entries, 'expired-request') &&
    readModel.replayRejectedCount === countEntries(readModel.entries, 'replay-rejected')
  );
}

function policyControlApprovalNotificationStatesAreCovered(
  entries: readonly PolicyControlApprovalNotificationBoundaryEntry[]
): boolean {
  const states = new Set(entries.map((entry) => entry.notificationState));
  return [
    'preview-only',
    'pending-parent-review',
    'approved',
    'denied',
    'modified',
    'expired-request',
    'replay-rejected',
  ].every((state) => states.has(state as PolicyControlApprovalNotificationState));
}

function countEntries(
  entries: readonly PolicyControlApprovalNotificationBoundaryEntry[],
  notificationState: PolicyControlApprovalNotificationState
): number {
  return entries.filter((entry) => entry.notificationState === notificationState).length;
}

export type PolicyControlApprovalNotificationState = Infer<
  typeof PolicyControlApprovalNotificationStateSchema
>;
export type PolicyControlApprovalNotificationOrigin = Infer<
  typeof PolicyControlApprovalNotificationOriginSchema
>;
export type PolicyControlApprovalNotificationKind = Infer<
  typeof PolicyControlApprovalNotificationKindSchema
>;
export type PolicyControlApprovalNotificationOverrideKind = Infer<
  typeof PolicyControlApprovalNotificationOverrideKindSchema
>;
export type PolicyControlApprovalNotificationBoundaryEntry = Infer<
  typeof PolicyControlApprovalNotificationBoundaryEntrySchema
>;
export type PolicyControlApprovalNotificationBoundaryReadModelShape = {
  schemaVersion: Infer<typeof ParentContractSchemaVersionSchema>;
  readModelId: string;
  generatedAt: string;
  sourceReadModelIds: readonly string[];
  returned: number;
  previewOnlyCount: number;
  pendingParentReviewCount: number;
  approvedCount: number;
  deniedCount: number;
  modifiedCount: number;
  expiredRequestCount: number;
  replayRejectedCount: number;
  providerDeliveryClaimed: false;
  policyMutationClaimed: false;
  enforcementMutationClaimed: false;
  assistantAutoApprovalClaimed: false;
  entries: readonly PolicyControlApprovalNotificationBoundaryEntry[];
};

const generatedAt = '2026-06-13T21:24:00.000Z';

export const PolicyControlApprovalNotificationBoundarySample =
  PolicyControlApprovalNotificationBoundaryReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'policy-control-approval-notification-boundary',
    generatedAt,
    sourceReadModelIds: [
      'policy-control-request-lifecycle',
      'policy-control-approval-audit-log',
      'policy-control-parent-approval-queue',
    ],
    returned: 7,
    previewOnlyCount: 1,
    pendingParentReviewCount: 1,
    approvedCount: 1,
    deniedCount: 1,
    modifiedCount: 1,
    expiredRequestCount: 1,
    replayRejectedCount: 1,
    providerDeliveryClaimed: false,
    policyMutationClaimed: false,
    enforcementMutationClaimed: false,
    assistantAutoApprovalClaimed: false,
    entries: [
      entry({
        entryId: 'policy-control-preview-only-assistant-draft',
        notificationState: 'preview-only',
        origin: 'assistant-draft',
        approvalKind: 'ask-parent',
        portalQueueVisible: true,
        parentConfirmationRequired: true,
      }),
      entry({
        entryId: 'policy-control-pending-parent-review-child-request',
        notificationState: 'pending-parent-review',
        origin: 'child-request',
        approvalKind: 'ask-parent',
        portalQueueVisible: true,
        parentConfirmationRequired: false,
      }),
      entry({
        entryId: 'policy-control-approved-bonus-time',
        notificationState: 'approved',
        origin: 'child-request',
        approvalKind: 'bonus-time',
        portalQueueVisible: true,
        parentConfirmationRequired: false,
        parentReviewed: true,
        approvalRef: 'policy-approval-approved-ref',
        overrideRef: 'policy-override-approved-ref',
        overrideKind: 'bonus-time',
      }),
      entry({
        entryId: 'policy-control-denied-temporary-override',
        notificationState: 'denied',
        origin: 'child-request',
        approvalKind: 'temporary-override',
        portalQueueVisible: true,
        parentConfirmationRequired: false,
        parentReviewed: true,
        approvalRef: 'policy-approval-denied-ref',
      }),
      entry({
        entryId: 'policy-control-modified-ask-parent',
        notificationState: 'modified',
        origin: 'assistant-draft',
        approvalKind: 'ask-parent',
        portalQueueVisible: true,
        parentConfirmationRequired: false,
        parentReviewed: true,
        approvalRef: 'policy-approval-modified-ref',
        overrideRef: 'policy-override-modified-ref',
        overrideKind: 'temporary-allow',
      }),
      entry({
        entryId: 'policy-control-expired-request',
        notificationState: 'expired-request',
        origin: 'child-request',
        approvalKind: 'ask-parent',
        portalQueueVisible: false,
        parentConfirmationRequired: false,
      }),
      entry({
        entryId: 'policy-control-replay-rejected',
        notificationState: 'replay-rejected',
        origin: 'child-request',
        approvalKind: 'bonus-time',
        portalQueueVisible: false,
        parentConfirmationRequired: false,
      }),
    ],
  });

type PolicyControlApprovalNotificationBoundaryEntryInput = {
  entryId: string;
  notificationState: PolicyControlApprovalNotificationState;
  origin: PolicyControlApprovalNotificationOrigin;
  approvalKind: PolicyControlApprovalNotificationKind;
  portalQueueVisible: boolean;
  parentConfirmationRequired: boolean;
  parentReviewed?: boolean;
  approvalRef?: string;
  overrideRef?: string;
  overrideKind?: PolicyControlApprovalNotificationOverrideKind;
};

function entry(
  input: PolicyControlApprovalNotificationBoundaryEntryInput
): PolicyControlApprovalNotificationBoundaryEntry {
  return PolicyControlApprovalNotificationBoundaryEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    requestRef: `${input.entryId}-request-ref`,
    notificationIntentRef: `${input.entryId}-notification-intent-ref`,
    auditRefs: [`${input.entryId}-audit-ref`],
    policyContextRefs: [`${input.entryId}-policy-context-ref`],
    manualProofRequirements: [],
    parentReviewed: input.parentReviewed ?? false,
    policyMutationClaimed: false,
    enforcementMutationClaimed: false,
    providerDeliveryClaimed: false,
    lastUpdatedAt: generatedAt,
    approvalRef: input.approvalRef ?? null,
    overrideRef: input.overrideRef ?? null,
    overrideKind: input.overrideKind ?? null,
    ...input,
  });
}

export const decodePolicyControlApprovalNotificationBoundaryEntry = Schema.decodeUnknownSync(
  PolicyControlApprovalNotificationBoundaryEntrySchema
);
export const decodePolicyControlApprovalNotificationBoundaryReadModel = Schema.decodeUnknownSync(
  PolicyControlApprovalNotificationBoundaryReadModelSchema
);
