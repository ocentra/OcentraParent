/* thin adapter over Rust-owned generated notification v3 provider retry contracts */

import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  GeneratedV3NotificationDeliveryAttemptStates,
  GeneratedV3NotificationDeliveryResultStates,
  GeneratedV3NotificationEscalationDecisions,
  GeneratedV3NotificationParentPreferenceStates,
  GeneratedV3NotificationProviderChannels,
  GeneratedV3NotificationQuietHoursDecisions,
  GeneratedV3NotificationRuleProviderRetryContractReadModel,
  GeneratedV3NotificationRuleReasonCodes,
  GeneratedV3NotificationRetryPolicyStates,
  generatedV3NotificationRuleProviderRetryContractEntryIsHonest,
  generatedV3NotificationRuleProviderRetryContractReadModelIsHonest,
} from './generated-notification-v3-provider-retry';

export const V3NotificationRuleProviderRetryContractReadModelIdSchema = brandedNonEmptyStringSchema(
  'V3NotificationRuleProviderRetryContractReadModelId'
);
export const V3NotificationRuleProviderRetryContractEntryIdSchema = brandedNonEmptyStringSchema(
  'V3NotificationRuleProviderRetryContractEntryId'
);
export const V3NotificationRuleProviderRetryContractReferenceSchema = brandedNonEmptyStringSchema(
  'V3NotificationRuleProviderRetryContractReference'
);
export const V3NotificationRuleProviderRetryContractRequirementSchema = brandedNonEmptyStringSchema(
  'V3NotificationRuleProviderRetryContractRequirement'
);
export const V3NotificationRuleProviderRetryContractTextSchema = brandedNonEmptyStringSchema(
  'V3NotificationRuleProviderRetryContractText'
);

export const V3NotificationRuleReasonCodeSchema = withParser(Schema.Literal(...GeneratedV3NotificationRuleReasonCodes));

export const V3NotificationProviderChannelSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationProviderChannels)
);

export const V3NotificationDeliveryAttemptStateSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationDeliveryAttemptStates)
);

export const V3NotificationDeliveryResultStateSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationDeliveryResultStates)
);

export const V3NotificationRetryPolicyStateSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationRetryPolicyStates)
);

export const V3NotificationQuietHoursDecisionSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationQuietHoursDecisions)
);

export const V3NotificationEscalationDecisionSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationEscalationDecisions)
);

export const V3NotificationParentPreferenceStateSchema = withParser(
  Schema.Literal(...GeneratedV3NotificationParentPreferenceStates)
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

export const V3NotificationRuleProviderRetryContractEntrySchema = withParser(
  V3NotificationRuleProviderRetryContractEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        generatedV3NotificationRuleProviderRetryContractEntryIsHonest(entry) ||
        'Expected V3 notification rule/provider retry contract rows to reference rule, reason, channel, attempt/result, retry, quiet-hours, escalation, preference, audit, and evidence contracts without claiming implemented provider delivery, observed receipts, raw evidence payloads, or provider child-evidence storage'
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
      (readModel) =>
        generatedV3NotificationRuleProviderRetryContractReadModelIsHonest(readModel) ||
        'Expected V3 notification rule/provider retry contract to cover queued, receipt-required, retryable failure, permanent failure, manual-required, and not-sent delivery results with retry policy states'
    )
  )
);

export type V3NotificationRuleProviderRetryContractReadModelId =
  typeof V3NotificationRuleProviderRetryContractReadModelIdSchema.Type;
export type V3NotificationRuleProviderRetryContractEntryId =
  typeof V3NotificationRuleProviderRetryContractEntryIdSchema.Type;
export type V3NotificationRuleProviderRetryContractReference =
  typeof V3NotificationRuleProviderRetryContractReferenceSchema.Type;
export type V3NotificationRuleProviderRetryContractRequirement =
  typeof V3NotificationRuleProviderRetryContractRequirementSchema.Type;
export type V3NotificationRuleProviderRetryContractText = typeof V3NotificationRuleProviderRetryContractTextSchema.Type;
export type V3NotificationRuleReasonCode = Infer<typeof V3NotificationRuleReasonCodeSchema>;
export type V3NotificationProviderChannel = Infer<typeof V3NotificationProviderChannelSchema>;
export type V3NotificationDeliveryAttemptState = Infer<typeof V3NotificationDeliveryAttemptStateSchema>;
export type V3NotificationDeliveryResultState = Infer<typeof V3NotificationDeliveryResultStateSchema>;
export type V3NotificationRetryPolicyState = Infer<typeof V3NotificationRetryPolicyStateSchema>;
export type V3NotificationQuietHoursDecision = Infer<typeof V3NotificationQuietHoursDecisionSchema>;
export type V3NotificationEscalationDecision = Infer<typeof V3NotificationEscalationDecisionSchema>;
export type V3NotificationParentPreferenceState = Infer<typeof V3NotificationParentPreferenceStateSchema>;
export type V3NotificationRuleProviderRetryContractEntry = Infer<
  typeof V3NotificationRuleProviderRetryContractEntrySchema
>;
export type V3NotificationRuleProviderRetryContractReadModel = Infer<
  typeof V3NotificationRuleProviderRetryContractReadModelSchema
>;

export const V3NotificationRuleProviderRetryContractReadModel =
  V3NotificationRuleProviderRetryContractReadModelSchema.parse(
    GeneratedV3NotificationRuleProviderRetryContractReadModel
  );
