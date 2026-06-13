import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';

export const V3NotificationRuleProviderRetryContractReadModelIdSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractReadModelId');
export const V3NotificationRuleProviderRetryContractEntryIdSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractEntryId');
export const V3NotificationRuleProviderRetryContractReferenceSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractReference');
export const V3NotificationRuleProviderRetryContractRequirementSchema =
  brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractRequirement');
export const V3NotificationRuleProviderRetryContractTextSchema = brandedNonEmptyStringSchema('V3NotificationRuleProviderRetryContractText');

export const V3NotificationRuleReasonCodeSchema = withParser(
  Schema.Literal(
    'policy-violation',
    'parent-request',
    'suspicious-unknown',
    'device-offline',
    'sync-failure',
    'provider-failure'
  )
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

type V3NotificationRuleProviderRetryContractEntryCandidate = Infer<
  typeof V3NotificationRuleProviderRetryContractEntryBaseSchema
>;

export const V3NotificationRuleProviderRetryContractEntrySchema = withParser(
  V3NotificationRuleProviderRetryContractEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        notificationRuleProviderRetryContractEntryIsHonest(entry) ||
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
        new Set(readModel.entries.map((entry) => entry.contractEntryId)).size === readModel.entries.length ||
        'Expected V3 notification rule/provider retry contract entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        notificationRuleProviderRetryContractCoversReasonCodes(readModel.entries) ||
        'Expected V3 notification rule/provider retry contract to cover policy violation, ask parent, suspicious unknown, device offline, sync failure, and provider failure reason codes'
    ),
    Schema.filter(
      (readModel) =>
        notificationRuleProviderRetryContractCoversProviderChannels(readModel.entries) ||
        'Expected V3 notification rule/provider retry contract to cover push, email, SMS, WhatsApp, and in-app provider channel contracts'
    ),
    Schema.filter(
      (readModel) =>
        notificationRuleProviderRetryContractCoversDeliveryAndRetry(readModel.entries) ||
        'Expected V3 notification rule/provider retry contract to cover queued, receipt-required, retryable failure, permanent failure, manual-required, and not-sent delivery results with retry policy states'
    )
  )
);

function notificationRuleProviderRetryContractEntryIsHonest(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    !notificationRuleProviderRetryContractHasRuntimeClaim(entry) &&
    notificationRuleProviderRetryContractHasRequiredRefs(entry) &&
    notificationRuleProviderRetryContractDeliveryStateIsCoherent(entry)
  );
}

function notificationRuleProviderRetryContractHasRuntimeClaim(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return [
    entry.providerAdapterImplemented,
    entry.deliveryAttemptExecuted,
    entry.providerReceiptObserved,
    entry.rawEvidenceInProviderPayload,
    entry.providerStoresChildEvidenceClaimed,
  ].some(Boolean);
}

function notificationRuleProviderRetryContractHasRequiredRefs(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
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

function notificationRuleProviderRetryContractDeliveryStateIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    notificationRuleProviderRetryContractQuietHoursIsCoherent(entry) &&
    notificationRuleProviderRetryContractParentPreferenceIsCoherent(entry) &&
    notificationRuleProviderRetryContractRetryableFailureIsCoherent(entry) &&
    notificationRuleProviderRetryContractReceiptRequiredIsCoherent(entry) &&
    notificationRuleProviderRetryContractPermanentFailureIsCoherent(entry) &&
    notificationRuleProviderRetryContractNonReceiptRowsAreCoherent(entry)
  );
}

function notificationRuleProviderRetryContractQuietHoursIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    entry.quietHoursDecision !== 'defer-noncritical' ||
    (entry.deliveryAttemptState === 'suppressed-quiet-hours' &&
      entry.retryPolicyState === 'quiet-hours-deferred' &&
      entry.parentPreferenceState === 'quiet-hours-active')
  );
}

function notificationRuleProviderRetryContractParentPreferenceIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    entry.parentPreferenceState !== 'channel-disabled' ||
    (entry.deliveryAttemptState === 'provider-disabled' &&
      entry.deliveryResultState === 'not-sent' &&
      entry.retryPolicyState === 'provider-disabled')
  );
}

function notificationRuleProviderRetryContractRetryableFailureIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    entry.deliveryResultState !== 'retryable-failure' ||
    (entry.retryPolicyState === 'exponential-backoff' && entry.manualProofRequirements.length > 0)
  );
}

function notificationRuleProviderRetryContractReceiptRequiredIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    entry.deliveryResultState !== 'receipt-required' ||
    (entry.providerReceiptRefs.length > 0 &&
      entry.manualProofRequirements.length > 0 &&
      entry.deliveryAttemptState === 'queued')
  );
}

function notificationRuleProviderRetryContractPermanentFailureIsCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return (
    entry.deliveryResultState !== 'permanent-failure' ||
    (entry.retryPolicyState === 'manual-review' && entry.manualProofRequirements.length > 0)
  );
}

function notificationRuleProviderRetryContractNonReceiptRowsAreCoherent(
  entry: V3NotificationRuleProviderRetryContractEntryCandidate
): boolean {
  return entry.deliveryResultState === 'receipt-required' || entry.providerReceiptRefs.length === 0;
}

function notificationRuleProviderRetryContractCoversReasonCodes(
  entries: readonly V3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const reasonCodes = new Set(entries.map((entry) => entry.reasonCode));
  return [
    'policy-violation',
    'parent-request',
    'suspicious-unknown',
    'device-offline',
    'sync-failure',
    'provider-failure',
  ].every((reasonCode) => reasonCodes.has(reasonCode as V3NotificationRuleReasonCode));
}

function notificationRuleProviderRetryContractCoversProviderChannels(
  entries: readonly V3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const channels = new Set(entries.map((entry) => entry.providerChannel));
  return ['push', 'email', 'sms', 'whatsapp', 'in-app'].every((channel) =>
    channels.has(channel as V3NotificationProviderChannel)
  );
}

function notificationRuleProviderRetryContractCoversDeliveryAndRetry(
  entries: readonly V3NotificationRuleProviderRetryContractEntry[]
): boolean {
  const deliveryResults = new Set(entries.map((entry) => entry.deliveryResultState));
  const retryPolicies = new Set(entries.map((entry) => entry.retryPolicyState));
  const quietHours = new Set(entries.map((entry) => entry.quietHoursDecision));
  const escalation = new Set(entries.map((entry) => entry.escalationDecision));
  const preferences = new Set(entries.map((entry) => entry.parentPreferenceState));

  return (
    ['not-sent', 'queued', 'retryable-failure', 'permanent-failure', 'receipt-required', 'manual-required'].every(
      (state) => deliveryResults.has(state as V3NotificationDeliveryResultState)
    ) &&
    ['no-retry', 'exponential-backoff', 'quiet-hours-deferred', 'manual-review', 'provider-disabled'].every((state) =>
      retryPolicies.has(state as V3NotificationRetryPolicyState)
    ) &&
    ['allow', 'defer-noncritical', 'emergency-override', 'manual-required'].every((state) =>
      quietHours.has(state as V3NotificationQuietHoursDecision)
    ) &&
    ['none', 'wait-window', 'escalate-parent', 'manual-review'].every((state) =>
      escalation.has(state as V3NotificationEscalationDecision)
    ) &&
    ['enabled', 'quiet-hours-active', 'channel-disabled', 'manual-setup-required'].every((state) =>
      preferences.has(state as V3NotificationParentPreferenceState)
    )
  );
}

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

type V3NotificationRuleProviderRetryContractEntryInput = {
  contractEntryId: string;
  reasonCode: V3NotificationRuleReasonCode;
  providerChannel: V3NotificationProviderChannel;
  deliveryAttemptState: V3NotificationDeliveryAttemptState;
  deliveryResultState: V3NotificationDeliveryResultState;
  retryPolicyState: V3NotificationRetryPolicyState;
  quietHoursDecision: V3NotificationQuietHoursDecision;
  escalationDecision: V3NotificationEscalationDecision;
  parentPreferenceState: V3NotificationParentPreferenceState;
  notificationRuleRef: string;
  notificationIntentRef: string;
  deliveryAttemptRef: string;
  deliveryResultRef: string;
  retryPolicyRef: string;
  quietHoursPolicyRef: string;
  escalationPolicyRef: string;
  parentPreferenceRef: string;
  auditRefs: readonly string[];
  evidenceRefs: readonly string[];
  providerReceiptRefs: readonly string[];
  manualProofRequirements: readonly string[];
  minimalProviderPayloadBoundary: string;
};

const generatedAt = '2026-06-02T15:18:13.000Z';

const SourceReadModelIds = {
  ReportsNotificationsSync: 'reports-notifications-sync-provider-status',
  IntegrityAlertStatusBridge: 'v0-8-integrity-alert-status-bridge',
  DataCustody: 'data-custody-provider-boundary',
  NotificationFeatureExpectations: 'notification-feature-expectations-contract-boundary',
} as const;

export const V3NotificationRuleProviderRetryContractReadModel =
  V3NotificationRuleProviderRetryContractReadModelSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readModelId: 'v3-notification-rule-provider-retry-contract',
    generatedAt,
    sourceReadModelIds: Object.values(SourceReadModelIds),
    entries: [
      v3Entry({
        contractEntryId: 'notification-rule-policy-violation-push-queued',
        reasonCode: 'policy-violation',
        providerChannel: 'push',
        deliveryAttemptState: 'queued',
        deliveryResultState: 'queued',
        retryPolicyState: 'no-retry',
        quietHoursDecision: 'emergency-override',
        escalationDecision: 'escalate-parent',
        parentPreferenceState: 'enabled',
        notificationRuleRef: 'notification-rule-policy-violation-ref',
        notificationIntentRef: 'notification-intent-policy-violation-ref',
        deliveryAttemptRef: 'delivery-attempt-policy-violation-push-ref',
        deliveryResultRef: 'delivery-result-policy-violation-queued-ref',
        retryPolicyRef: 'retry-policy-no-retry-critical-ref',
        quietHoursPolicyRef: 'quiet-hours-emergency-override-ref',
        escalationPolicyRef: 'escalation-policy-escalate-parent-ref',
        parentPreferenceRef: 'parent-preference-push-enabled-ref',
        auditRefs: ['notification-audit-policy-violation-ref'],
        evidenceRefs: ['policy-decision-evidence-ref', 'authenticated-drill-in-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: [],
        minimalProviderPayloadBoundary:
          'Critical policy-violation payload carries only alert id, severity, reason code, evidence ref, policy ref, and parent action link.',
      }),
      v3Entry({
        contractEntryId: 'notification-rule-parent-request-in-app-receipt-required',
        reasonCode: 'parent-request',
        providerChannel: 'in-app',
        deliveryAttemptState: 'queued',
        deliveryResultState: 'receipt-required',
        retryPolicyState: 'no-retry',
        quietHoursDecision: 'allow',
        escalationDecision: 'wait-window',
        parentPreferenceState: 'enabled',
        notificationRuleRef: 'notification-rule-parent-request-ref',
        notificationIntentRef: 'notification-intent-parent-request-ref',
        deliveryAttemptRef: 'delivery-attempt-parent-request-in-app-ref',
        deliveryResultRef: 'delivery-result-parent-request-receipt-required-ref',
        retryPolicyRef: 'retry-policy-no-retry-parent-action-ref',
        quietHoursPolicyRef: 'quiet-hours-allow-parent-action-ref',
        escalationPolicyRef: 'escalation-policy-wait-window-ref',
        parentPreferenceRef: 'parent-preference-in-app-enabled-ref',
        auditRefs: ['notification-audit-parent-request-ref'],
        evidenceRefs: ['parent-request-ref', 'authenticated-parent-action-ref'],
        providerReceiptRefs: ['provider-receipt-parent-action-required-ref'],
        manualProofRequirements: ['real in-app receipt artifact before parent notification delivery can be claimed'],
        minimalProviderPayloadBoundary:
          'Ask-parent payload carries intent ref and parent action link; sensitive child detail remains behind authenticated parent surfaces.',
      }),
      v3Entry({
        contractEntryId: 'notification-rule-suspicious-unknown-email-retryable-failure',
        reasonCode: 'suspicious-unknown',
        providerChannel: 'email',
        deliveryAttemptState: 'retry-scheduled',
        deliveryResultState: 'retryable-failure',
        retryPolicyState: 'exponential-backoff',
        quietHoursDecision: 'allow',
        escalationDecision: 'manual-review',
        parentPreferenceState: 'manual-setup-required',
        notificationRuleRef: 'notification-rule-suspicious-unknown-ref',
        notificationIntentRef: 'notification-intent-suspicious-unknown-ref',
        deliveryAttemptRef: 'delivery-attempt-suspicious-unknown-email-ref',
        deliveryResultRef: 'delivery-result-suspicious-unknown-retryable-ref',
        retryPolicyRef: 'retry-policy-exponential-backoff-ref',
        quietHoursPolicyRef: 'quiet-hours-allow-suspicious-unknown-ref',
        escalationPolicyRef: 'escalation-policy-manual-review-ref',
        parentPreferenceRef: 'parent-preference-email-setup-required-ref',
        auditRefs: ['notification-audit-suspicious-unknown-ref'],
        evidenceRefs: ['classified-evidence-ref', 'notification-intent-audit-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['provider error artifact before retry execution can be claimed'],
        minimalProviderPayloadBoundary:
          'Suspicious-unknown payload avoids raw observation details and carries only reason, severity, evidence ref, and authenticated drill-in.',
      }),
      v3Entry({
        contractEntryId: 'notification-rule-device-offline-sms-permanent-failure',
        reasonCode: 'device-offline',
        providerChannel: 'sms',
        deliveryAttemptState: 'failed-final',
        deliveryResultState: 'permanent-failure',
        retryPolicyState: 'manual-review',
        quietHoursDecision: 'manual-required',
        escalationDecision: 'manual-review',
        parentPreferenceState: 'manual-setup-required',
        notificationRuleRef: 'notification-rule-device-offline-ref',
        notificationIntentRef: 'notification-intent-device-offline-ref',
        deliveryAttemptRef: 'delivery-attempt-device-offline-sms-ref',
        deliveryResultRef: 'delivery-result-device-offline-permanent-failure-ref',
        retryPolicyRef: 'retry-policy-manual-review-ref',
        quietHoursPolicyRef: 'quiet-hours-manual-required-ref',
        escalationPolicyRef: 'escalation-policy-manual-review-ref',
        parentPreferenceRef: 'parent-preference-sms-setup-required-ref',
        auditRefs: ['notification-audit-device-offline-ref'],
        evidenceRefs: ['device-health-status-ref', 'offline-window-evidence-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: [
          'provider failure artifact and parent preference setup before SMS retry can be claimed',
        ],
        minimalProviderPayloadBoundary:
          'Device-offline SMS payload carries device scope, reason code, and action link only; raw child activity is excluded.',
      }),
      v3Entry({
        contractEntryId: 'notification-rule-sync-failure-whatsapp-quiet-hours-deferred',
        reasonCode: 'sync-failure',
        providerChannel: 'whatsapp',
        deliveryAttemptState: 'suppressed-quiet-hours',
        deliveryResultState: 'manual-required',
        retryPolicyState: 'quiet-hours-deferred',
        quietHoursDecision: 'defer-noncritical',
        escalationDecision: 'wait-window',
        parentPreferenceState: 'quiet-hours-active',
        notificationRuleRef: 'notification-rule-sync-failure-ref',
        notificationIntentRef: 'notification-intent-sync-failure-ref',
        deliveryAttemptRef: 'delivery-attempt-sync-failure-whatsapp-ref',
        deliveryResultRef: 'delivery-result-sync-failure-quiet-hours-deferred-ref',
        retryPolicyRef: 'retry-policy-quiet-hours-deferred-ref',
        quietHoursPolicyRef: 'quiet-hours-defer-noncritical-ref',
        escalationPolicyRef: 'escalation-policy-wait-window-sync-ref',
        parentPreferenceRef: 'parent-preference-whatsapp-quiet-hours-ref',
        auditRefs: ['notification-audit-sync-failure-ref'],
        evidenceRefs: ['sync-failure-state-ref', 'parent-owned-storage-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['quiet-hours preference artifact before deferred provider send can be claimed'],
        minimalProviderPayloadBoundary:
          'Sync-failure payload is deferable and references parent-owned storage state without embedding report or raw evidence content.',
      }),
      v3Entry({
        contractEntryId: 'notification-rule-provider-failure-in-app-channel-disabled',
        reasonCode: 'provider-failure',
        providerChannel: 'in-app',
        deliveryAttemptState: 'provider-disabled',
        deliveryResultState: 'not-sent',
        retryPolicyState: 'provider-disabled',
        quietHoursDecision: 'allow',
        escalationDecision: 'none',
        parentPreferenceState: 'channel-disabled',
        notificationRuleRef: 'notification-rule-provider-failure-ref',
        notificationIntentRef: 'notification-intent-provider-failure-ref',
        deliveryAttemptRef: 'delivery-attempt-provider-disabled-ref',
        deliveryResultRef: 'delivery-result-not-sent-provider-disabled-ref',
        retryPolicyRef: 'retry-policy-provider-disabled-ref',
        quietHoursPolicyRef: 'quiet-hours-allow-provider-failure-ref',
        escalationPolicyRef: 'escalation-policy-none-provider-disabled-ref',
        parentPreferenceRef: 'parent-preference-channel-disabled-ref',
        auditRefs: ['notification-audit-provider-disabled-ref'],
        evidenceRefs: ['provider-configuration-state-ref', 'notification-routing-status-ref'],
        providerReceiptRefs: [],
        manualProofRequirements: ['provider enablement and credential review before send or retry can be claimed'],
        minimalProviderPayloadBoundary:
          'Provider-failure row is an audit and preference state only; no provider payload is sent while the channel is disabled.',
      }),
    ],
  });

function v3Entry(
  input: V3NotificationRuleProviderRetryContractEntryInput
): V3NotificationRuleProviderRetryContractEntry {
  return V3NotificationRuleProviderRetryContractEntrySchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    providerAdapterImplemented: false,
    deliveryAttemptExecuted: false,
    providerReceiptObserved: false,
    rawEvidenceInProviderPayload: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeV3NotificationRuleProviderRetryContractEntry = Schema.decodeUnknownSync(
  V3NotificationRuleProviderRetryContractEntrySchema
);
export const decodeV3NotificationRuleProviderRetryContractReadModel = Schema.decodeUnknownSync(
  V3NotificationRuleProviderRetryContractReadModelSchema
);

