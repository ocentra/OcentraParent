import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';

export const V3NotificationRuleProviderRetryContractReadModelIdSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractReadModelId');
export const V3NotificationRuleProviderRetryContractEntryIdSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractEntryId');
export const V3NotificationRuleProviderRetryContractReferenceSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractReference');
export const V3NotificationRuleProviderRetryContractRequirementSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractRequirement');
export const V3NotificationRuleProviderRetryContractTextSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractText');

export const V3NotificationRuleReasonCodeSchema = withParser(
  Schema.Literal('policy-violation', 'parent-request', 'suspicious-unknown', 'device-offline', 'sync-failure', 'provider-failure')
);
export const V3NotificationProviderChannelSchema = withParser(
  Schema.Literal('push', 'email', 'sms', 'whatsapp', 'in-app')
);
export const V3NotificationDeliveryAttemptStateSchema = withParser(
  Schema.Literal('eligible', 'queued', 'suppressed-quiet-hours', 'retry-scheduled', 'failed-final', 'provider-disabled')
);
export const V3NotificationDeliveryResultStateSchema = withParser(
  Schema.Literal('not-sent', 'queued', 'retryable-failure', 'permanent-failure', 'receipt-required', 'manual-required')
);
export const V3NotificationRetryPolicyStateSchema = withParser(
  Schema.Literal('no-retry', 'exponential-backoff', 'quiet-hours-deferred', 'manual-review', 'provider-disabled')
);
export const V3NotificationQuietHoursDecisionSchema = withParser(
  Schema.Literal('allow', 'defer-noncritical', 'emergency-override', 'manual-required')
);
export const V3NotificationEscalationDecisionSchema = withParser(
  Schema.Literal('none', 'wait-window', 'escalate-parent', 'manual-review')
);
export const V3NotificationParentPreferenceStateSchema = withParser(
  Schema.Literal('enabled', 'quiet-hours-active', 'channel-disabled', 'manual-setup-required')
);

const V3NotificationRuleProviderRetryContractEntryBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  contractEntryId: V3NotificationRuleProviderRetryContractEntryIdSchema,
  reasonCode: V3NotificationRuleReasonCodeSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  deliveryAttemptState: V3NotificationDeliveryAttemptStateSchema,
  deliveryResultState: V3NotificationDeliveryResultStateSchema,
  retryPolicyState: V3NotificationRetryPolicyStateSchema,
  quietHoursDecision: V3NotificationQuietHoursDecisionSchema,
  escalationDecision: V3NotificationEscalationDecisionSchema,
  parentPreferenceState: V3NotificationParentPreferenceStateSchema,
  notificationRuleRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  notificationIntentRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  deliveryAttemptRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  deliveryResultRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  retryPolicyRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  quietHoursPolicyRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  escalationPolicyRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  parentPreferenceRef: V3NotificationRuleProviderRetryContractReferenceSchema,
  auditRefs: Schema.Array(V3NotificationRuleProviderRetryContractReferenceSchema),
  evidenceRefs: Schema.Array(V3NotificationRuleProviderRetryContractReferenceSchema),
  providerReceiptRefs: Schema.Array(V3NotificationRuleProviderRetryContractReferenceSchema),
  manualProofRequirements: Schema.Array(V3NotificationRuleProviderRetryContractRequirementSchema),
  minimalProviderPayloadBoundary: V3NotificationRuleProviderRetryContractTextSchema,
  providerAdapterImplemented: Schema.Boolean,
  deliveryAttemptExecuted: Schema.Boolean,
  providerReceiptObserved: Schema.Boolean,
  rawEvidenceInProviderPayload: Schema.Boolean,
  providerStoresChildEvidenceClaimed: Schema.Boolean,
  lastCheckedAt: ParentTimestampSchema,
});

type V3NotificationRuleProviderRetryContractEntryCandidate = Infer<typeof V3NotificationRuleProviderRetryContractEntryBaseSchema>;

export const V3NotificationRuleProviderRetryContractEntrySchema = withParser(
  V3NotificationRuleProviderRetryContractEntryBaseSchema.pipe(
    Schema.filter(
      (entry) => notificationRuleProviderRetryContractEntryIsHonest(entry) || 'Expected honest notification provider retry row'
    )
  )
);

export const V3NotificationRuleProviderRetryContractReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    readModelId: V3NotificationRuleProviderRetryContractReadModelIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelIds: Schema.Array(V3NotificationRuleProviderRetryContractReferenceSchema),
    entries: Schema.Array(V3NotificationRuleProviderRetryContractEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) => new Set(readModel.entries.map((entry) => entry.contractEntryId)).size === readModel.entries.length ||
        'Expected unique notification provider retry entry ids'
    )
  )
);

function notificationRuleProviderRetryContractEntryIsHonest(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    ![
      entry.providerAdapterImplemented,
      entry.deliveryAttemptExecuted,
      entry.providerReceiptObserved,
      entry.rawEvidenceInProviderPayload,
      entry.providerStoresChildEvidenceClaimed,
    ].some(Boolean) &&
    entry.auditRefs.length > 0 &&
    entry.evidenceRefs.length > 0 &&
    entry.notificationRuleRef.trim().length > 0 &&
    entry.notificationIntentRef.trim().length > 0 &&
    entry.deliveryAttemptRef.trim().length > 0 &&
    entry.deliveryResultRef.trim().length > 0 &&
    entry.retryPolicyRef.trim().length > 0 &&
    entry.quietHoursPolicyRef.trim().length > 0 &&
    entry.escalationPolicyRef.trim().length > 0 &&
    entry.parentPreferenceRef.trim().length > 0 &&
    entry.minimalProviderPayloadBoundary.trim().length > 0
  );
}

export type V3NotificationRuleProviderRetryContractReadModelId = typeof V3NotificationRuleProviderRetryContractReadModelIdSchema.Type;
export type V3NotificationRuleProviderRetryContractEntryId = typeof V3NotificationRuleProviderRetryContractEntryIdSchema.Type;
export type V3NotificationRuleProviderRetryContractReference = typeof V3NotificationRuleProviderRetryContractReferenceSchema.Type;
export type V3NotificationRuleProviderRetryContractRequirement = typeof V3NotificationRuleProviderRetryContractRequirementSchema.Type;
export type V3NotificationRuleProviderRetryContractText = typeof V3NotificationRuleProviderRetryContractTextSchema.Type;
export type V3NotificationRuleReasonCode = Infer<typeof V3NotificationRuleReasonCodeSchema>;
export type V3NotificationProviderChannel = Infer<typeof V3NotificationProviderChannelSchema>;
export type V3NotificationDeliveryAttemptState = Infer<typeof V3NotificationDeliveryAttemptStateSchema>;
export type V3NotificationDeliveryResultState = Infer<typeof V3NotificationDeliveryResultStateSchema>;
export type V3NotificationRetryPolicyState = Infer<typeof V3NotificationRetryPolicyStateSchema>;
export type V3NotificationQuietHoursDecision = Infer<typeof V3NotificationQuietHoursDecisionSchema>;
export type V3NotificationEscalationDecision = Infer<typeof V3NotificationEscalationDecisionSchema>;
export type V3NotificationParentPreferenceState = Infer<typeof V3NotificationParentPreferenceStateSchema>;
export type V3NotificationRuleProviderRetryContractEntry = Infer<typeof V3NotificationRuleProviderRetryContractEntrySchema>;
export type V3NotificationRuleProviderRetryContractReadModel = Infer<typeof V3NotificationRuleProviderRetryContractReadModelSchema>;
