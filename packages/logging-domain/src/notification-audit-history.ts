import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

const NonEmptyNotificationAuditHistoryText = Schema.String.pipe(Schema.minLength(1));

export const NotificationAuditHistoryReadModelIdSchema = NonEmptyNotificationAuditHistoryText.pipe(
  Schema.brand('NotificationAuditHistoryReadModelId')
);
export const NotificationAuditHistoryEntryIdSchema = NonEmptyNotificationAuditHistoryText.pipe(
  Schema.brand('NotificationAuditHistoryEntryId')
);
export const NotificationAuditHistoryReferenceSchema = NonEmptyNotificationAuditHistoryText.pipe(
  Schema.brand('NotificationAuditHistoryReference')
);
export const NotificationAuditHistoryRequirementSchema = NonEmptyNotificationAuditHistoryText.pipe(
  Schema.brand('NotificationAuditHistoryRequirement')
);
export const NotificationAuditHistoryTimestampSchema = NonEmptyNotificationAuditHistoryText.pipe(
  Schema.brand('NotificationAuditHistoryTimestamp')
);

export const NotificationAuditHistoryProviderStatusSchema = withParser(
  Schema.Literal('queued', 'delivered', 'failed', 'unavailable', 'manual-required')
);

export const NotificationAuditHistoryRetryLifecycleStateSchema = withParser(
  Schema.Literal(
    'not-scheduled',
    'receipt-required-contract',
    'retry-scheduled-contract',
    'manual-review-required',
    'provider-unavailable',
    'quiet-hours-deferred-contract'
  )
);

export const NotificationAuditHistoryQuietHoursStateSchema = withParser(
  Schema.Literal('allow', 'defer-noncritical', 'manual-required', 'unavailable')
);

export const NotificationAuditHistoryEscalationStateSchema = withParser(
  Schema.Literal('none', 'waiting-window', 'manual-required', 'unavailable')
);

export const NotificationAuditHistoryPayloadFieldSchema = withParser(
  Schema.Literal(
    'alert-id-ref',
    'family-scope-ref',
    'device-scope-ref',
    'severity',
    'reason-code',
    'provider-channel',
    'provider-status',
    'retry-lifecycle-state',
    'parent-action-link-ref',
    'audit-entry-ref'
  )
);

export const NotificationAuditHistoryCustodyStateSchema = withParser(Schema.Literal('no-ocentra-hosted-child-data'));

export const NotificationAuditHistoryRedactionStateSchema = withParser(
  Schema.Literal('minimal-operational-fields-only')
);

const RequiredPayloadFields = [
  'alert-id-ref',
  'family-scope-ref',
  'device-scope-ref',
  'severity',
  'reason-code',
  'provider-channel',
  'provider-status',
  'retry-lifecycle-state',
  'parent-action-link-ref',
  'audit-entry-ref',
] as const;

const NotificationAuditHistoryEntryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(1),
  auditEntryId: NotificationAuditHistoryEntryIdSchema,
  providerStatus: NotificationAuditHistoryProviderStatusSchema,
  retryLifecycleState: NotificationAuditHistoryRetryLifecycleStateSchema,
  quietHoursState: NotificationAuditHistoryQuietHoursStateSchema,
  escalationState: NotificationAuditHistoryEscalationStateSchema,
  payloadRedactionState: NotificationAuditHistoryRedactionStateSchema,
  childDataCustodyState: NotificationAuditHistoryCustodyStateSchema,
  notificationIntentRef: NotificationAuditHistoryReferenceSchema,
  providerStatusRef: NotificationAuditHistoryReferenceSchema,
  providerAttemptRef: NotificationAuditHistoryReferenceSchema,
  deliveryResultRef: NotificationAuditHistoryReferenceSchema,
  retryLifecycleRef: NotificationAuditHistoryReferenceSchema,
  redactionPolicyRef: NotificationAuditHistoryReferenceSchema,
  custodyPolicyRef: NotificationAuditHistoryReferenceSchema,
  auditRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  evidenceRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  receiptRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  retryRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  manualRequiredRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  quietHoursRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  escalationRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
  manualProofRequirements: Schema.Array(NotificationAuditHistoryRequirementSchema),
  redactionSafePayloadFields: Schema.Array(NotificationAuditHistoryPayloadFieldSchema),
  providerAdapterImplemented: Schema.Boolean,
  sendAttemptExecuted: Schema.Boolean,
  retryExecutionObserved: Schema.Boolean,
  webhookReceiptIngested: Schema.Boolean,
  providerCredentialPresent: Schema.Boolean,
  notificationHistoryUiClaimed: Schema.Boolean,
  rawChildDataIncluded: Schema.Boolean,
  rawEvidencePayloadIncluded: Schema.Boolean,
  sensitiveProviderPayloadIncluded: Schema.Boolean,
  ocentraHostedChildDataStored: Schema.Boolean,
  providerStoresChildEvidenceClaimed: Schema.Boolean,
  lastCheckedAt: NotificationAuditHistoryTimestampSchema,
});

type NotificationAuditHistoryEntryCandidate = Infer<typeof NotificationAuditHistoryEntryBaseSchema>;

export const NotificationAuditHistoryEntrySchema = withParser(
  NotificationAuditHistoryEntryBaseSchema.pipe(
    Schema.filter(
      (entry) =>
        notificationAuditHistoryEntryIsSafe(entry) ||
        'Expected notification audit/history logs to keep provider status, retry lifecycle, receipt/manual, quiet-hours, escalation, and redaction-safe refs without provider runtime, webhook, UI, credential, raw child data, or Ocentra-hosted custody claims'
    )
  )
);

export const NotificationAuditHistoryReadModelSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(1),
    readModelId: NotificationAuditHistoryReadModelIdSchema,
    generatedAt: NotificationAuditHistoryTimestampSchema,
    sourceContractRefs: Schema.Array(NotificationAuditHistoryReferenceSchema),
    entries: Schema.Array(NotificationAuditHistoryEntrySchema),
  }).pipe(
    Schema.filter(
      (readModel) =>
        new Set(readModel.entries.map((entry) => entry.auditEntryId)).size === readModel.entries.length ||
        'Expected notification audit/history entry ids to be unique'
    ),
    Schema.filter(
      (readModel) =>
        notificationAuditHistoryCoversProviderStatuses(readModel.entries) ||
        'Expected notification audit/history to cover queued, delivered, failed, unavailable, and manual-required provider status logs'
    ),
    Schema.filter(
      (readModel) =>
        notificationAuditHistoryCoversRetryLifecycle(readModel.entries) ||
        'Expected notification audit/history to cover receipt-required, retry-scheduled, manual-review, unavailable, quiet-hours-deferred, and not-scheduled retry lifecycle states'
    ),
    Schema.filter(
      (readModel) =>
        notificationAuditHistoryCoversQuietHoursAndEscalation(readModel.entries) ||
        'Expected notification audit/history to cover quiet-hours and escalation refs without deleting audit history'
    )
  )
);

function notificationAuditHistoryEntryIsSafe(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return (
    !notificationAuditHistoryHasClaimUpgrade(entry) &&
    notificationAuditHistoryHasRequiredRefs(entry) &&
    notificationAuditHistoryHasRequiredPayloadFields(entry) &&
    notificationAuditHistoryStatesAreCoherent(entry)
  );
}

function notificationAuditHistoryHasClaimUpgrade(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return [
    entry.providerAdapterImplemented,
    entry.sendAttemptExecuted,
    entry.retryExecutionObserved,
    entry.webhookReceiptIngested,
    entry.providerCredentialPresent,
    entry.notificationHistoryUiClaimed,
    entry.rawChildDataIncluded,
    entry.rawEvidencePayloadIncluded,
    entry.sensitiveProviderPayloadIncluded,
    entry.ocentraHostedChildDataStored,
    entry.providerStoresChildEvidenceClaimed,
  ].some(Boolean);
}

function notificationAuditHistoryHasRequiredRefs(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return entry.auditRefs.length > 0 && entry.evidenceRefs.length > 0;
}

function notificationAuditHistoryHasRequiredPayloadFields(entry: NotificationAuditHistoryEntryCandidate): boolean {
  const fields = new Set(entry.redactionSafePayloadFields);
  return (
    fields.size === entry.redactionSafePayloadFields.length && RequiredPayloadFields.every((field) => fields.has(field))
  );
}

function notificationAuditHistoryStatesAreCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return (
    notificationAuditHistoryDeliveredStateIsCoherent(entry) &&
    notificationAuditHistoryRetryStateIsCoherent(entry) &&
    notificationAuditHistoryManualStateIsCoherent(entry) &&
    notificationAuditHistoryQuietHoursStateIsCoherent(entry) &&
    notificationAuditHistoryEscalationStateIsCoherent(entry)
  );
}

function notificationAuditHistoryDeliveredStateIsCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return (
    entry.providerStatus !== 'delivered' ||
    (entry.retryLifecycleState === 'receipt-required-contract' &&
      entry.receiptRefs.length > 0 &&
      entry.manualProofRequirements.length > 0)
  );
}

function notificationAuditHistoryRetryStateIsCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return (
    entry.retryLifecycleState !== 'retry-scheduled-contract' ||
    (entry.providerStatus === 'failed' && entry.retryRefs.length > 0 && entry.manualProofRequirements.length > 0)
  );
}

function notificationAuditHistoryManualStateIsCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return (
    entry.providerStatus !== 'manual-required' ||
    (entry.manualRequiredRefs.length > 0 && entry.manualProofRequirements.length > 0)
  );
}

function notificationAuditHistoryQuietHoursStateIsCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return entry.quietHoursState === 'allow' || entry.quietHoursRefs.length > 0;
}

function notificationAuditHistoryEscalationStateIsCoherent(entry: NotificationAuditHistoryEntryCandidate): boolean {
  return entry.escalationState === 'none' || entry.escalationRefs.length > 0;
}

function notificationAuditHistoryCoversProviderStatuses(entries: readonly NotificationAuditHistoryEntry[]): boolean {
  const statuses = new Set(entries.map((entry) => entry.providerStatus));
  return ['queued', 'delivered', 'failed', 'unavailable', 'manual-required'].every((status) =>
    statuses.has(status as NotificationAuditHistoryProviderStatus)
  );
}

function notificationAuditHistoryCoversRetryLifecycle(entries: readonly NotificationAuditHistoryEntry[]): boolean {
  const states = new Set(entries.map((entry) => entry.retryLifecycleState));
  return [
    'not-scheduled',
    'receipt-required-contract',
    'retry-scheduled-contract',
    'manual-review-required',
    'provider-unavailable',
    'quiet-hours-deferred-contract',
  ].every((state) => states.has(state as NotificationAuditHistoryRetryLifecycleState));
}

function notificationAuditHistoryCoversQuietHoursAndEscalation(
  entries: readonly NotificationAuditHistoryEntry[]
): boolean {
  const quietHours = new Set(entries.map((entry) => entry.quietHoursState));
  const escalation = new Set(entries.map((entry) => entry.escalationState));
  return (
    ['allow', 'defer-noncritical', 'manual-required', 'unavailable'].every((state) =>
      quietHours.has(state as NotificationAuditHistoryQuietHoursState)
    ) &&
    ['none', 'waiting-window', 'manual-required', 'unavailable'].every((state) =>
      escalation.has(state as NotificationAuditHistoryEscalationState)
    )
  );
}

export type NotificationAuditHistoryReadModelId = typeof NotificationAuditHistoryReadModelIdSchema.Type;
export type NotificationAuditHistoryEntryId = typeof NotificationAuditHistoryEntryIdSchema.Type;
export type NotificationAuditHistoryReference = typeof NotificationAuditHistoryReferenceSchema.Type;
export type NotificationAuditHistoryRequirement = typeof NotificationAuditHistoryRequirementSchema.Type;
export type NotificationAuditHistoryTimestamp = typeof NotificationAuditHistoryTimestampSchema.Type;
export type NotificationAuditHistoryProviderStatus = Infer<typeof NotificationAuditHistoryProviderStatusSchema>;
export type NotificationAuditHistoryRetryLifecycleState = Infer<
  typeof NotificationAuditHistoryRetryLifecycleStateSchema
>;
export type NotificationAuditHistoryQuietHoursState = Infer<typeof NotificationAuditHistoryQuietHoursStateSchema>;
export type NotificationAuditHistoryEscalationState = Infer<typeof NotificationAuditHistoryEscalationStateSchema>;
export type NotificationAuditHistoryPayloadField = Infer<typeof NotificationAuditHistoryPayloadFieldSchema>;
export type NotificationAuditHistoryCustodyState = Infer<typeof NotificationAuditHistoryCustodyStateSchema>;
export type NotificationAuditHistoryRedactionState = Infer<typeof NotificationAuditHistoryRedactionStateSchema>;
export type NotificationAuditHistoryEntry = Infer<typeof NotificationAuditHistoryEntrySchema>;
export type NotificationAuditHistoryReadModel = Infer<typeof NotificationAuditHistoryReadModelSchema>;

type NotificationAuditHistoryEntryInput = {
  auditEntryId: string;
  providerStatus: NotificationAuditHistoryProviderStatus;
  retryLifecycleState: NotificationAuditHistoryRetryLifecycleState;
  quietHoursState: NotificationAuditHistoryQuietHoursState;
  escalationState: NotificationAuditHistoryEscalationState;
  receiptRefs: readonly string[];
  retryRefs: readonly string[];
  manualRequiredRefs: readonly string[];
  quietHoursRefs: readonly string[];
  escalationRefs: readonly string[];
  manualProofRequirements: readonly string[];
};

const generatedAt = '2026-06-03T07:18:54.002Z';

export const NotificationAuditHistoryReadModel = NotificationAuditHistoryReadModelSchema.parse({
  schemaVersion: 1,
  readModelId: 'notification-audit-history-contract-proof',
  generatedAt,
  sourceContractRefs: [
    'reports-notifications-sync-feature-doc',
    'notification-feature-expectations-contract-boundary',
    'v3-notification-rule-provider-retry-contract',
    'v0-8-notification-provider-status-boundary',
    'data-custody-notification-metadata-boundary',
  ],
  entries: [
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-provider-queued',
      providerStatus: 'queued',
      retryLifecycleState: 'not-scheduled',
      quietHoursState: 'allow',
      escalationState: 'none',
      receiptRefs: [],
      retryRefs: [],
      manualRequiredRefs: [],
      quietHoursRefs: [],
      escalationRefs: [],
      manualProofRequirements: [],
    }),
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-delivered-receipt-required',
      providerStatus: 'delivered',
      retryLifecycleState: 'receipt-required-contract',
      quietHoursState: 'defer-noncritical',
      escalationState: 'waiting-window',
      receiptRefs: ['provider-receipt-required-ref'],
      retryRefs: [],
      manualRequiredRefs: [],
      quietHoursRefs: ['quiet-hours-defer-noncritical-ref'],
      escalationRefs: ['escalation-waiting-window-ref'],
      manualProofRequirements: ['provider receipt artifact before delivered history can be claimed'],
    }),
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-failed-retry-scheduled',
      providerStatus: 'failed',
      retryLifecycleState: 'retry-scheduled-contract',
      quietHoursState: 'allow',
      escalationState: 'manual-required',
      receiptRefs: [],
      retryRefs: ['retry-policy-exponential-backoff-ref'],
      manualRequiredRefs: ['provider-error-artifact-required-ref'],
      quietHoursRefs: [],
      escalationRefs: ['escalation-manual-review-ref'],
      manualProofRequirements: ['provider error artifact before retry execution can be claimed'],
    }),
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-failed-manual-review',
      providerStatus: 'failed',
      retryLifecycleState: 'manual-review-required',
      quietHoursState: 'manual-required',
      escalationState: 'manual-required',
      receiptRefs: [],
      retryRefs: [],
      manualRequiredRefs: ['manual-review-required-ref'],
      quietHoursRefs: ['quiet-hours-manual-required-ref'],
      escalationRefs: ['escalation-manual-required-ref'],
      manualProofRequirements: ['parent/provider preference review before retry lifecycle can be claimed'],
    }),
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-provider-unavailable',
      providerStatus: 'unavailable',
      retryLifecycleState: 'provider-unavailable',
      quietHoursState: 'unavailable',
      escalationState: 'unavailable',
      receiptRefs: [],
      retryRefs: [],
      manualRequiredRefs: ['provider-configuration-required-ref'],
      quietHoursRefs: ['quiet-hours-unavailable-ref'],
      escalationRefs: ['escalation-unavailable-ref'],
      manualProofRequirements: [
        'provider configuration and credential review before provider availability can be claimed',
      ],
    }),
    notificationAuditHistoryEntry({
      auditEntryId: 'notification-audit-manual-quiet-hours-deferred',
      providerStatus: 'manual-required',
      retryLifecycleState: 'quiet-hours-deferred-contract',
      quietHoursState: 'defer-noncritical',
      escalationState: 'waiting-window',
      receiptRefs: [],
      retryRefs: ['quiet-hours-deferred-retry-ref'],
      manualRequiredRefs: ['quiet-hours-parent-preference-required-ref'],
      quietHoursRefs: ['quiet-hours-defer-noncritical-ref'],
      escalationRefs: ['escalation-waiting-window-ref'],
      manualProofRequirements: ['parent quiet-hours preference artifact before deferred send can be claimed'],
    }),
  ],
});

function notificationAuditHistoryEntry(input: NotificationAuditHistoryEntryInput): NotificationAuditHistoryEntry {
  return NotificationAuditHistoryEntrySchema.parse({
    schemaVersion: 1,
    payloadRedactionState: 'minimal-operational-fields-only',
    childDataCustodyState: 'no-ocentra-hosted-child-data',
    notificationIntentRef: 'notification-intent-audit-history-ref',
    providerStatusRef: 'notification-provider-status-ref',
    providerAttemptRef: 'notification-provider-attempt-ref',
    deliveryResultRef: 'notification-delivery-result-ref',
    retryLifecycleRef: 'notification-retry-lifecycle-ref',
    redactionPolicyRef: 'notification-redaction-policy-ref',
    custodyPolicyRef: 'notification-data-custody-policy-ref',
    auditRefs: ['notification-audit-history-ref'],
    evidenceRefs: ['authenticated-evidence-drill-in-ref'],
    redactionSafePayloadFields: [...RequiredPayloadFields],
    providerAdapterImplemented: false,
    sendAttemptExecuted: false,
    retryExecutionObserved: false,
    webhookReceiptIngested: false,
    providerCredentialPresent: false,
    notificationHistoryUiClaimed: false,
    rawChildDataIncluded: false,
    rawEvidencePayloadIncluded: false,
    sensitiveProviderPayloadIncluded: false,
    ocentraHostedChildDataStored: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: generatedAt,
    ...input,
  });
}

export const decodeNotificationAuditHistoryEntry = Schema.decodeUnknownSync(NotificationAuditHistoryEntrySchema);
export const decodeNotificationAuditHistoryReadModel = Schema.decodeUnknownSync(
  NotificationAuditHistoryReadModelSchema
);
