import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import {
  V3NotificationProviderChannelSchema,
  V3NotificationRuleReasonCodeSchema,
} from './notification-v3-provider-retry';

export const RequiredNotificationLocalOutboxStates = [
  'queued-local',
  'deferred-quiet-hours',
  'retry-scheduled',
  'dead-lettered',
  'receipt-required',
  'manual-required',
] as const;

export const RequiredNotificationLocalOutboxNonClaims = [
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-raw-child-evidence',
  'no-sensitive-provider-metadata',
] as const;

export const NotificationLocalOutboxForbiddenDetailFragments = [
  'http://',
  'https://',
  'screenshot-bytes',
  'raw-title-value',
  'raw-message-body',
  'sqlite-private-path',
  'oauth-secret',
  'provider-token',
  'report-body',
] as const;

export const NotificationLocalOutboxKnownGaps = [
  'No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this parent-domain proof.',
  'No provider delivery execution, webhook receipt ingestion, credentials, cloud routing, or parent notification UI is claimed.',
  'No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the local outbox artifact.',
  'Quiet-hours scheduling, retry execution, dead-letter review, and receipt/manual-required handling remain adapter/runtime work.',
  'Durable production outbox storage, retention controls, parent-visible history, and physical provider smoke proof remain future work.',
] as const;

export const NotificationLocalOutboxProviderChannels = ['push', 'email', 'sms', 'whatsapp', 'in-app'] as const;

export const NotificationLocalOutboxProofTimestamp = '2026-06-04T01:31:47.023Z';

export const NotificationLocalOutboxProofFamily = {
  familyId: 'family-notification-local-outbox-proof-1',
} as const;

export const NotificationLocalOutboxProofDevice = {
  deviceId: 'windows-child-device-notification-outbox-proof-1',
  childProfileId: 'child-notification-outbox-proof-1',
  label: 'Windows child device notification outbox proof',
  platform: 'windows',
} as const;

export const NotificationLocalOutboxProofParentAction = {
  actionReferenceId: 'parent-action-notification-outbox-proof-1',
  actor: { actorId: 'parent-notification-outbox-proof-1', role: 'parent' },
  policyVersion: 'notification-local-outbox-proof-v1',
  createdAt: NotificationLocalOutboxProofTimestamp,
} as const;

export const NotificationLocalOutboxProofEvidenceRef = {
  evidenceReferenceId: 'notification-local-outbox-evidence-ref-1',
  kind: 'policy-decision',
  observedAt: NotificationLocalOutboxProofTimestamp,
} as const;

export const NotificationLocalOutboxProofRows = [
  {
    entryId: 'notification-local-outbox-policy-violation-push-queued',
    state: 'queued-local',
    reasonCode: 'policy-violation',
    providerChannel: 'push',
    severity: 'urgent',
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerPayloadPreview: 'alert id, urgent severity, policy reason, evidence ref, policy ref, parent action link',
  },
  {
    entryId: 'notification-local-outbox-sync-failure-whatsapp-deferred',
    state: 'deferred-quiet-hours',
    reasonCode: 'sync-failure',
    providerChannel: 'whatsapp',
    severity: 'attention',
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: '2026-06-04T12:00:00.000Z',
    retryAttemptCount: 0,
    quietHoursRef: 'quiet-hours-defer-noncritical-ref',
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerPayloadPreview: 'alert id, sync failure reason, parent-owned storage ref, authenticated drill-in',
  },
  {
    entryId: 'notification-local-outbox-suspicious-unknown-email-retry',
    state: 'retry-scheduled',
    reasonCode: 'suspicious-unknown',
    providerChannel: 'email',
    severity: 'attention',
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: '2026-06-04T01:41:47.023Z',
    retryAttemptCount: 1,
    quietHoursRef: null,
    retryPolicyRef: 'retry-policy-exponential-backoff-ref',
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    providerPayloadPreview: 'alert id, suspicious unknown reason, evidence ref, retry window ref, parent action link',
  },
  {
    entryId: 'notification-local-outbox-provider-failure-sms-dead-letter',
    state: 'dead-lettered',
    reasonCode: 'provider-failure',
    providerChannel: 'sms',
    severity: 'info',
    deliveryClaimState: 'local-outbox-only',
    visibleAfterAt: null,
    retryAttemptCount: 3,
    quietHoursRef: null,
    retryPolicyRef: 'retry-policy-dead-letter-ref',
    deadLetterRef: 'dead-letter-provider-setup-required-ref',
    providerReceiptRef: null,
    manualProofRequirements: ['provider setup review required'],
    manualActionRequired: true,
    providerPayloadPreview: 'alert id, provider failure reason, dead letter ref, manual review link',
  },
  {
    entryId: 'notification-local-outbox-parent-request-in-app-receipt-required',
    state: 'receipt-required',
    reasonCode: 'parent-request',
    providerChannel: 'in-app',
    severity: 'attention',
    deliveryClaimState: 'provider-receipt-required',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: 'provider-receipt-required-ref',
    manualProofRequirements: ['real provider receipt artifact required before delivery can be claimed'],
    manualActionRequired: true,
    providerPayloadPreview: 'alert id, parent request reason, receipt required marker, parent action link',
  },
  {
    entryId: 'notification-local-outbox-device-offline-in-app-manual',
    state: 'manual-required',
    reasonCode: 'device-offline',
    providerChannel: 'in-app',
    severity: 'urgent',
    deliveryClaimState: 'manual-required',
    visibleAfterAt: null,
    retryAttemptCount: 0,
    quietHoursRef: null,
    retryPolicyRef: null,
    deadLetterRef: null,
    providerReceiptRef: null,
    manualProofRequirements: ['parent/provider preference setup required before send path can be enabled'],
    manualActionRequired: true,
    providerPayloadPreview: 'alert id, device offline reason, manual required marker, authenticated parent link',
  },
] as const;

const NotificationOutboxRetryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const NotificationLocalOutboxAdapterProofSchemaVersionSchema = withParser(
  Schema.Literal('notification-local-outbox-adapter-proof')
);

export const NotificationLocalOutboxStateSchema = withParser(Schema.Literal(...RequiredNotificationLocalOutboxStates));

export const NotificationLocalOutboxNonClaimSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxNonClaims)
);

export const NotificationLocalOutboxSeveritySchema = withParser(Schema.Literal('info', 'attention', 'urgent'));

export const NotificationLocalOutboxDeliveryClaimStateSchema = withParser(
  Schema.Literal('local-outbox-only', 'provider-receipt-required', 'manual-required')
);

export const NotificationLocalOutboxReadModelIdSchema = brandedNonEmptyStringSchema(
  'NotificationLocalOutboxReadModelId'
);
export const NotificationLocalOutboxEntryIdSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxEntryId');
export const NotificationLocalOutboxReferenceSchema = brandedNonEmptyStringSchema('NotificationLocalOutboxReference');
export const NotificationLocalOutboxPayloadPreviewSchema = brandedNonEmptyStringSchema(
  'NotificationLocalOutboxPayloadPreview'
);

const NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema = Schema.Struct({
  alertRef: NotificationLocalOutboxReferenceSchema,
  family: FamilyReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  severity: NotificationLocalOutboxSeveritySchema,
  reasonCode: V3NotificationRuleReasonCodeSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  evidenceRefs: Schema.Array(ParentEvidenceReferenceSchema),
  policyRefs: Schema.Array(NotificationLocalOutboxReferenceSchema),
  auditRefs: Schema.Array(NotificationLocalOutboxReferenceSchema),
  payloadTemplateRef: NotificationLocalOutboxReferenceSchema,
  providerPayloadPreview: NotificationLocalOutboxPayloadPreviewSchema,
  sensitiveDetailMinimized: Schema.Boolean,
  rawChildEvidenceIncluded: Schema.Boolean,
  rawUrlOrTitleIncluded: Schema.Boolean,
  rawMessageTextIncluded: Schema.Boolean,
  screenshotOrReportIncluded: Schema.Boolean,
});

export const NotificationLocalOutboxMinimalAlertEnvelopeSchema = withParser(
  NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema.pipe(
    Schema.filter(
      (envelope) =>
        notificationEnvelopeIsSafe(envelope) ||
        'Expected local notification outbox envelopes to carry minimal refs only, without raw child evidence, URLs, titles, message text, screenshots, reports, or forbidden payload fragments'
    )
  )
);

const NotificationLocalOutboxRecordBaseSchema = Schema.Struct({
  entryId: NotificationLocalOutboxEntryIdSchema,
  state: NotificationLocalOutboxStateSchema,
  envelope: NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  outboxFileRef: NotificationLocalOutboxReferenceSchema,
  localDataPathRef: NotificationLocalOutboxReferenceSchema,
  deliveryClaimState: NotificationLocalOutboxDeliveryClaimStateSchema,
  visibleAfterAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  retryAttemptCount: NotificationOutboxRetryCountSchema,
  quietHoursRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  retryPolicyRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  deadLetterRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  providerReceiptRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  manualProofRequirements: Schema.Array(NotificationLocalOutboxReferenceSchema),
  manualActionRequired: Schema.Boolean,
  providerDeliveryAttempted: Schema.Boolean,
  providerDeliveryObserved: Schema.Boolean,
  providerReceiptIngested: Schema.Boolean,
  providerCredentialsStored: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  sensitiveProviderMetadataStored: Schema.Boolean,
});

export const NotificationLocalOutboxRecordSchema = withParser(
  NotificationLocalOutboxRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        notificationOutboxRecordIsSafe(record) ||
        'Expected local outbox records to be filesystem/local-data-path refs only, with coherent defer/retry/dead-letter/receipt/manual states and no provider delivery or sensitive metadata claims'
    )
  )
);

const NotificationLocalOutboxProofBaseSchema = Schema.Struct({
  schemaVersion: NotificationLocalOutboxAdapterProofSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  readModelId: NotificationLocalOutboxReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  outboxRootRef: NotificationLocalOutboxReferenceSchema,
  records: Schema.Array(NotificationLocalOutboxRecordSchema),
  nonClaims: Schema.Array(NotificationLocalOutboxNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  providerCredentialsClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
});

export const NotificationLocalOutboxAdapterProofSchema = withParser(
  NotificationLocalOutboxProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        notificationOutboxProofIsSafe(proof) ||
        'Expected notification local outbox proof to cover required local states/channels and keep provider delivery, receipt, credentials, cloud routing, and UI non-claims explicit'
    )
  )
);

type NotificationEnvelopeCandidate = Infer<typeof NotificationLocalOutboxMinimalAlertEnvelopeBaseSchema>;
type NotificationOutboxRecordCandidate = Infer<typeof NotificationLocalOutboxRecordBaseSchema>;
type NotificationOutboxProofCandidate = Infer<typeof NotificationLocalOutboxProofBaseSchema>;

export type NotificationLocalOutboxState = Infer<typeof NotificationLocalOutboxStateSchema>;
export type NotificationLocalOutboxNonClaim = Infer<typeof NotificationLocalOutboxNonClaimSchema>;
export type NotificationLocalOutboxSeverity = Infer<typeof NotificationLocalOutboxSeveritySchema>;
export type NotificationLocalOutboxDeliveryClaimState = Infer<typeof NotificationLocalOutboxDeliveryClaimStateSchema>;
export type NotificationLocalOutboxMinimalAlertEnvelope = Infer<
  typeof NotificationLocalOutboxMinimalAlertEnvelopeSchema
>;
export type NotificationLocalOutboxRecord = Infer<typeof NotificationLocalOutboxRecordSchema>;
export type NotificationLocalOutboxAdapterProof = Infer<typeof NotificationLocalOutboxAdapterProofSchema>;

const AdapterClaimFlags = [
  'providerDeliveryRuntimeClaimed',
  'providerReceiptIngestionClaimed',
  'providerCredentialsClaimed',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
] as const;

const AdapterRecordClaimFlags = [
  'providerDeliveryAttempted',
  'providerDeliveryObserved',
  'providerReceiptIngested',
  'providerCredentialsStored',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'sensitiveProviderMetadataStored',
] as const;

function notificationEnvelopeIsSafe(envelope: NotificationEnvelopeCandidate): boolean {
  return (
    envelope.sensitiveDetailMinimized &&
    !envelope.rawChildEvidenceIncluded &&
    !envelope.rawUrlOrTitleIncluded &&
    !envelope.rawMessageTextIncluded &&
    !envelope.screenshotOrReportIncluded &&
    envelope.evidenceRefs.length > 0 &&
    envelope.policyRefs.length > 0 &&
    envelope.auditRefs.length > 0 &&
    !textContainsForbiddenDetail(envelope.providerPayloadPreview)
  );
}

function notificationOutboxRecordIsSafe(record: NotificationOutboxRecordCandidate): boolean {
  return (
    !AdapterRecordClaimFlags.some((flag) => record[flag]) &&
    record.outboxFileRef.trim().length > 0 &&
    record.localDataPathRef.trim().length > 0 &&
    notificationOutboxStateIsCoherent(record)
  );
}

function notificationOutboxStateIsCoherent(record: NotificationOutboxRecordCandidate): boolean {
  if (record.state !== 'receipt-required' && record.providerReceiptRef !== null) {
    return false;
  }
  if (record.state === 'queued-local') {
    return record.visibleAfterAt === null && record.retryAttemptCount === 0 && !record.manualActionRequired;
  }
  if (record.state === 'deferred-quiet-hours') {
    return record.visibleAfterAt !== null && record.quietHoursRef !== null && !record.manualActionRequired;
  }
  if (record.state === 'retry-scheduled') {
    return record.retryAttemptCount > 0 && record.retryPolicyRef !== null && record.visibleAfterAt !== null;
  }
  return notificationOutboxTerminalStateIsCoherent(record);
}

function notificationOutboxTerminalStateIsCoherent(record: NotificationOutboxRecordCandidate): boolean {
  if (record.state === 'dead-lettered') {
    return record.deadLetterRef !== null && record.manualActionRequired && record.manualProofRequirements.length > 0;
  }
  if (record.state === 'receipt-required') {
    return (
      record.deliveryClaimState === 'provider-receipt-required' &&
      record.providerReceiptRef !== null &&
      record.manualActionRequired &&
      record.manualProofRequirements.length > 0
    );
  }
  return (
    record.state === 'manual-required' &&
    record.deliveryClaimState === 'manual-required' &&
    record.manualActionRequired &&
    record.manualProofRequirements.length > 0
  );
}

function notificationOutboxProofIsSafe(proof: NotificationOutboxProofCandidate): boolean {
  return (
    requiredOutboxStatesAreCovered(proof.records) &&
    requiredOutboxChannelsAreCovered(proof.records) &&
    RequiredNotificationLocalOutboxNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    AdapterClaimFlags.every((flag) => proof[flag] === false)
  );
}

function requiredOutboxStatesAreCovered(records: ReadonlyArray<NotificationOutboxRecordCandidate>): boolean {
  return RequiredNotificationLocalOutboxStates.every((state) => records.some((record) => record.state === state));
}

function requiredOutboxChannelsAreCovered(records: ReadonlyArray<NotificationOutboxRecordCandidate>): boolean {
  return NotificationLocalOutboxProviderChannels.every((channel) =>
    records.some((record) => record.envelope.providerChannel === channel)
  );
}

type OutboxInput = (typeof NotificationLocalOutboxProofRows)[number];

function outboxRecord(input: OutboxInput): NotificationLocalOutboxRecord {
  const { providerPayloadPreview, ...recordInput } = input;
  return NotificationLocalOutboxRecordSchema.parse({
    ...recordInput,
    envelope: {
      alertRef: `notification-alert-${input.entryId}`,
      family: NotificationLocalOutboxProofFamily,
      device: NotificationLocalOutboxProofDevice,
      parentAction: NotificationLocalOutboxProofParentAction,
      severity: input.severity,
      reasonCode: input.reasonCode,
      providerChannel: input.providerChannel,
      evidenceRefs: [NotificationLocalOutboxProofEvidenceRef],
      policyRefs: ['notification-policy-ref-1'],
      auditRefs: [`notification-audit-${input.entryId}`],
      payloadTemplateRef: `notification-minimal-template-${input.reasonCode}`,
      providerPayloadPreview,
      sensitiveDetailMinimized: true,
      rawChildEvidenceIncluded: false,
      rawUrlOrTitleIncluded: false,
      rawMessageTextIncluded: false,
      screenshotOrReportIncluded: false,
    },
    outboxFileRef: 'local-notification-outbox-jsonl-ref',
    localDataPathRef: 'parent-owned-local-notification-outbox-data-path-ref',
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    sensitiveProviderMetadataStored: false,
  });
}

export const NotificationLocalOutboxAdapterProofReadModel = NotificationLocalOutboxAdapterProofSchema.parse({
  schemaVersion: 'notification-local-outbox-adapter-proof',
  contractVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'notification-local-outbox-adapter-proof',
  generatedAt: NotificationLocalOutboxProofTimestamp,
  outboxRootRef: 'parent-owned-local-notification-outbox-root',
  nonClaims: RequiredNotificationLocalOutboxNonClaims,
  providerDeliveryRuntimeClaimed: false,
  providerReceiptIngestionClaimed: false,
  providerCredentialsClaimed: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  records: NotificationLocalOutboxProofRows.map((row) => outboxRecord(row)),
});

export const decodeNotificationLocalOutboxRecord = (input: unknown): NotificationLocalOutboxRecord =>
  NotificationLocalOutboxRecordSchema.parse(input);

export const decodeNotificationLocalOutboxAdapterProof = (input: unknown): NotificationLocalOutboxAdapterProof =>
  NotificationLocalOutboxAdapterProofSchema.parse(input);

export const RequiredNotificationLocalOutboxSchedulerStates = [
  'due-local',
  'held-quiet-hours',
  'retry-window-scheduled',
  'dead-letter-review',
  'receipt-required',
  'manual-required',
] as const;

export const RequiredNotificationLocalOutboxSchedulerNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-production-durable-outbox-storage',
  'no-sensitive-detail-storage',
] as const;

export const NotificationLocalOutboxSchedulerProofTimestamp = '2026-06-04T02:28:51.667Z';
export const NotificationLocalOutboxSchedulerProofNow = '2026-06-04T02:28:51.667Z';
export const NotificationLocalOutboxSchedulerArtifactRef = 'parent-owned-local-notification-outbox-scheduler-jsonl-ref';

export const NotificationLocalOutboxSchedulerKnownGaps = [
  'No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this parent-domain scheduler proof.',
  'No provider delivery execution, webhook receipt ingestion, provider credentials, cloud routing, or parent notification UI is claimed.',
  'No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the scheduler artifact.',
  'Scheduler decisions are deterministic parent-domain proof rows; no production timer loop, durable outbox database, provider retry worker, or receipt webhook is implemented.',
  'Parent-visible history, preferences, escalation controls, retention controls, and physical provider smoke proof remain future work.',
] as const;

export const NotificationLocalOutboxSchedulerProofRows = [
  {
    sourceEntryId: 'notification-local-outbox-policy-violation-push-queued',
    schedulerState: 'due-local',
    schedulerDecisionRef: 'scheduler-due-policy-violation-ref',
    nextAttemptAt: NotificationLocalOutboxSchedulerProofNow,
    quietHoursWindow: null,
    retryWindow: null,
    deadLetterReviewRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    schedulerPayloadPreview: 'alert id, push channel, policy reason, evidence ref, scheduler due marker',
  },
  {
    sourceEntryId: 'notification-local-outbox-sync-failure-whatsapp-deferred',
    schedulerState: 'held-quiet-hours',
    schedulerDecisionRef: 'scheduler-quiet-hours-sync-failure-ref',
    nextAttemptAt: '2026-06-04T12:00:00.000Z',
    quietHoursWindow: {
      quietHoursWindowRef: 'quiet-hours-household-night-window-ref',
      startsAt: '2026-06-04T02:00:00.000Z',
      endsAt: '2026-06-04T12:00:00.000Z',
      holdReasonRef: 'quiet-hours-noncritical-sync-failure-hold-ref',
    },
    retryWindow: null,
    deadLetterReviewRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    schedulerPayloadPreview:
      'alert id, whatsapp channel, sync failure reason, quiet-hours hold, authenticated drill-in',
  },
  {
    sourceEntryId: 'notification-local-outbox-suspicious-unknown-email-retry',
    schedulerState: 'retry-window-scheduled',
    schedulerDecisionRef: 'scheduler-retry-suspicious-unknown-ref',
    nextAttemptAt: '2026-06-04T02:38:51.667Z',
    quietHoursWindow: null,
    retryWindow: {
      retryWindowRef: 'retry-window-exponential-backoff-attempt-2-ref',
      opensAt: '2026-06-04T02:38:51.667Z',
      closesAt: '2026-06-04T02:43:51.667Z',
      attemptNumber: 2,
      maxAttempts: 3,
    },
    deadLetterReviewRef: null,
    providerReceiptRef: null,
    manualProofRequirements: [],
    manualActionRequired: false,
    schedulerPayloadPreview: 'alert id, email channel, suspicious unknown reason, retry window, parent action link',
  },
  {
    sourceEntryId: 'notification-local-outbox-provider-failure-sms-dead-letter',
    schedulerState: 'dead-letter-review',
    schedulerDecisionRef: 'scheduler-dead-letter-provider-failure-ref',
    nextAttemptAt: null,
    quietHoursWindow: null,
    retryWindow: null,
    deadLetterReviewRef: 'dead-letter-provider-setup-review-ref',
    providerReceiptRef: null,
    manualProofRequirements: ['provider setup review required before retry worker can be enabled'],
    manualActionRequired: true,
    schedulerPayloadPreview: 'alert id, sms channel, provider failure reason, dead-letter review, manual link',
  },
  {
    sourceEntryId: 'notification-local-outbox-parent-request-in-app-receipt-required',
    schedulerState: 'receipt-required',
    schedulerDecisionRef: 'scheduler-receipt-required-parent-request-ref',
    nextAttemptAt: null,
    quietHoursWindow: null,
    retryWindow: null,
    deadLetterReviewRef: null,
    providerReceiptRef: 'provider-receipt-required-ref',
    manualProofRequirements: ['real provider receipt artifact required before delivered state can be claimed'],
    manualActionRequired: true,
    schedulerPayloadPreview: 'alert id, in-app channel, parent request reason, receipt required, parent action link',
  },
  {
    sourceEntryId: 'notification-local-outbox-device-offline-in-app-manual',
    schedulerState: 'manual-required',
    schedulerDecisionRef: 'scheduler-manual-required-device-offline-ref',
    nextAttemptAt: null,
    quietHoursWindow: null,
    retryWindow: null,
    deadLetterReviewRef: null,
    providerReceiptRef: null,
    manualProofRequirements: ['parent/provider preference setup required before notification worker can be enabled'],
    manualActionRequired: true,
    schedulerPayloadPreview:
      'alert id, in-app channel, device offline reason, manual required, authenticated parent link',
  },
] as const;

const NotificationSchedulerAttemptCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const NotificationLocalOutboxSchedulerProofSchemaVersionSchema = withParser(
  Schema.Literal('notification-local-outbox-scheduler-proof')
);

export const NotificationLocalOutboxSchedulerStateSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxSchedulerStates)
);

export const NotificationLocalOutboxSchedulerNonClaimSchema = withParser(
  Schema.Literal(...RequiredNotificationLocalOutboxSchedulerNonClaims)
);

export const NotificationLocalOutboxSchedulerReadModelIdSchema = brandedNonEmptyStringSchema(
  'NotificationLocalOutboxSchedulerReadModelId'
);
export const NotificationLocalOutboxSchedulerEntryIdSchema = brandedNonEmptyStringSchema(
  'NotificationLocalOutboxSchedulerEntryId'
);

const NotificationLocalOutboxQuietHoursWindowSchema = Schema.Struct({
  quietHoursWindowRef: NotificationLocalOutboxReferenceSchema,
  startsAt: ParentTimestampSchema,
  endsAt: ParentTimestampSchema,
  holdReasonRef: NotificationLocalOutboxReferenceSchema,
});

const NotificationLocalOutboxRetryWindowSchema = Schema.Struct({
  retryWindowRef: NotificationLocalOutboxReferenceSchema,
  opensAt: ParentTimestampSchema,
  closesAt: ParentTimestampSchema,
  attemptNumber: NotificationSchedulerAttemptCountSchema,
  maxAttempts: NotificationSchedulerAttemptCountSchema,
});

const NotificationLocalOutboxSchedulerRecordBaseSchema = Schema.Struct({
  schedulerEntryId: NotificationLocalOutboxSchedulerEntryIdSchema,
  sourceEntryId: NotificationLocalOutboxEntryIdSchema,
  sourceState: NotificationLocalOutboxStateSchema,
  schedulerState: NotificationLocalOutboxSchedulerStateSchema,
  reasonCode: V3NotificationRuleReasonCodeSchema,
  providerChannel: V3NotificationProviderChannelSchema,
  severity: NotificationLocalOutboxSeveritySchema,
  schedulerDecisionRef: NotificationLocalOutboxReferenceSchema,
  schedulerArtifactRef: NotificationLocalOutboxReferenceSchema,
  sourceOutboxFileRef: NotificationLocalOutboxReferenceSchema,
  localDataPathRef: NotificationLocalOutboxReferenceSchema,
  schedulerNowAt: ParentTimestampSchema,
  nextAttemptAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  quietHoursWindow: Schema.Union(NotificationLocalOutboxQuietHoursWindowSchema, Schema.Null),
  retryWindow: Schema.Union(NotificationLocalOutboxRetryWindowSchema, Schema.Null),
  deadLetterReviewRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  providerReceiptRef: Schema.Union(NotificationLocalOutboxReferenceSchema, Schema.Null),
  manualProofRequirements: Schema.Array(NotificationLocalOutboxReferenceSchema),
  manualActionRequired: Schema.Boolean,
  parentOwnedArtifactWritten: Schema.Boolean,
  rawChildEvidenceIncluded: Schema.Boolean,
  rawUrlOrTitleIncluded: Schema.Boolean,
  rawMessageTextIncluded: Schema.Boolean,
  screenshotOrReportIncluded: Schema.Boolean,
  providerDeliveryAttempted: Schema.Boolean,
  providerDeliveryObserved: Schema.Boolean,
  providerReceiptIngested: Schema.Boolean,
  providerCredentialsStored: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  productionDurableOutboxStorageClaimed: Schema.Boolean,
  sensitiveProviderMetadataStored: Schema.Boolean,
  schedulerPayloadPreview: NotificationLocalOutboxPayloadPreviewSchema,
});

export const NotificationLocalOutboxSchedulerRecordSchema = withParser(
  NotificationLocalOutboxSchedulerRecordBaseSchema.pipe(
    Schema.filter(
      (record) =>
        notificationOutboxSchedulerRecordIsSafe(record) ||
        'Expected local notification scheduler rows to use parent-owned artifact refs, deterministic next-at/retry windows, coherent manual/receipt/dead-letter states, and no provider delivery or sensitive-detail claims'
    )
  )
);

const NotificationLocalOutboxSchedulerProofBaseSchema = Schema.Struct({
  schemaVersion: NotificationLocalOutboxSchedulerProofSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  readModelId: NotificationLocalOutboxSchedulerReadModelIdSchema,
  generatedAt: ParentTimestampSchema,
  schedulerNowAt: ParentTimestampSchema,
  schedulerArtifactRootRef: NotificationLocalOutboxReferenceSchema,
  sourceAdapterReadModelId: NotificationLocalOutboxReferenceSchema,
  records: Schema.Array(NotificationLocalOutboxSchedulerRecordSchema),
  nonClaims: Schema.Array(NotificationLocalOutboxSchedulerNonClaimSchema),
  providerDeliveryRuntimeClaimed: Schema.Boolean,
  providerReceiptIngestionClaimed: Schema.Boolean,
  providerCredentialsClaimed: Schema.Boolean,
  cloudRoutingClaimed: Schema.Boolean,
  parentNotificationUiClaimed: Schema.Boolean,
  retryExecutionRuntimeClaimed: Schema.Boolean,
  quietHoursTimerRuntimeClaimed: Schema.Boolean,
  productionDurableOutboxStorageClaimed: Schema.Boolean,
});

export const NotificationLocalOutboxSchedulerProofSchema = withParser(
  NotificationLocalOutboxSchedulerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        notificationOutboxSchedulerProofIsSafe(proof) ||
        'Expected notification local outbox scheduler proof to cover required scheduler states/channels, write only parent-owned artifacts, and keep provider delivery, retry execution, quiet-hours timer, UI, cloud, credential, and durable storage non-claims explicit'
    )
  )
);

export type NotificationOutboxSchedulerRecordCandidate = Infer<typeof NotificationLocalOutboxSchedulerRecordBaseSchema>;
export type NotificationOutboxSchedulerProofCandidate = Infer<typeof NotificationLocalOutboxSchedulerProofBaseSchema>;

export type NotificationLocalOutboxSchedulerState = Infer<typeof NotificationLocalOutboxSchedulerStateSchema>;
export type NotificationLocalOutboxSchedulerNonClaim = Infer<typeof NotificationLocalOutboxSchedulerNonClaimSchema>;
export type NotificationLocalOutboxSchedulerRecord = Infer<typeof NotificationLocalOutboxSchedulerRecordSchema>;
export type NotificationLocalOutboxSchedulerProof = Infer<typeof NotificationLocalOutboxSchedulerProofSchema>;

const SchedulerClaimFlags = [
  'providerDeliveryRuntimeClaimed',
  'providerReceiptIngestionClaimed',
  'providerCredentialsClaimed',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'retryExecutionRuntimeClaimed',
  'quietHoursTimerRuntimeClaimed',
  'productionDurableOutboxStorageClaimed',
] as const;

const SchedulerRecordClaimFlags = [
  'rawChildEvidenceIncluded',
  'rawUrlOrTitleIncluded',
  'rawMessageTextIncluded',
  'screenshotOrReportIncluded',
  'providerDeliveryAttempted',
  'providerDeliveryObserved',
  'providerReceiptIngested',
  'providerCredentialsStored',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'productionDurableOutboxStorageClaimed',
  'sensitiveProviderMetadataStored',
] as const;

export function notificationOutboxSchedulerRecordIsSafe(record: NotificationOutboxSchedulerRecordCandidate): boolean {
  return (
    record.parentOwnedArtifactWritten &&
    !SchedulerRecordClaimFlags.some((flag) => record[flag]) &&
    record.schedulerArtifactRef.trim().length > 0 &&
    record.sourceOutboxFileRef.trim().length > 0 &&
    record.localDataPathRef.trim().length > 0 &&
    !textContainsForbiddenDetail(record.schedulerPayloadPreview) &&
    notificationOutboxSchedulerStateIsCoherent(record)
  );
}

export function notificationOutboxSchedulerProofIsSafe(proof: NotificationOutboxSchedulerProofCandidate): boolean {
  return (
    requiredSchedulerStatesAreCovered(proof.records) &&
    requiredSchedulerChannelsAreCovered(proof.records) &&
    RequiredNotificationLocalOutboxSchedulerNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    SchedulerClaimFlags.every((flag) => proof[flag] === false)
  );
}

function notificationOutboxSchedulerStateIsCoherent(record: NotificationOutboxSchedulerRecordCandidate): boolean {
  if (record.schedulerState === 'due-local') {
    return record.nextAttemptAt === record.schedulerNowAt && noHoldRetryOrManualRefs(record);
  }
  if (record.schedulerState === 'held-quiet-hours') {
    return (
      record.quietHoursWindow !== null &&
      record.nextAttemptAt === record.quietHoursWindow.endsAt &&
      record.retryWindow === null &&
      !record.manualActionRequired
    );
  }
  if (record.schedulerState === 'retry-window-scheduled') {
    return (
      record.retryWindow !== null &&
      record.retryWindow.attemptNumber > 1 &&
      record.retryWindow.attemptNumber <= record.retryWindow.maxAttempts &&
      record.nextAttemptAt === record.retryWindow.opensAt &&
      !record.manualActionRequired
    );
  }
  return notificationOutboxSchedulerTerminalStateIsCoherent(record);
}

function notificationOutboxSchedulerTerminalStateIsCoherent(
  record: NotificationOutboxSchedulerRecordCandidate
): boolean {
  if (record.schedulerState === 'dead-letter-review') {
    return terminalManualStateIsCoherent(record) && record.deadLetterReviewRef !== null;
  }
  if (record.schedulerState === 'receipt-required') {
    return terminalManualStateIsCoherent(record) && record.providerReceiptRef !== null;
  }
  return record.schedulerState === 'manual-required' && terminalManualStateIsCoherent(record);
}

function noHoldRetryOrManualRefs(record: NotificationOutboxSchedulerRecordCandidate): boolean {
  return (
    record.quietHoursWindow === null &&
    record.retryWindow === null &&
    record.deadLetterReviewRef === null &&
    record.providerReceiptRef === null &&
    record.manualProofRequirements.length === 0 &&
    !record.manualActionRequired
  );
}

function terminalManualStateIsCoherent(record: NotificationOutboxSchedulerRecordCandidate): boolean {
  return (
    record.nextAttemptAt === null &&
    record.quietHoursWindow === null &&
    record.retryWindow === null &&
    record.manualActionRequired &&
    record.manualProofRequirements.length > 0
  );
}

function requiredSchedulerStatesAreCovered(
  records: ReadonlyArray<NotificationOutboxSchedulerRecordCandidate>
): boolean {
  return RequiredNotificationLocalOutboxSchedulerStates.every((state) =>
    records.some((record) => record.schedulerState === state)
  );
}

function requiredSchedulerChannelsAreCovered(
  records: ReadonlyArray<NotificationOutboxSchedulerRecordCandidate>
): boolean {
  return NotificationLocalOutboxProviderChannels.every((channel) =>
    records.some((record) => record.providerChannel === channel)
  );
}

type SchedulerInput = (typeof NotificationLocalOutboxSchedulerProofRows)[number];

function sourceRecordFor(sourceEntryId: SchedulerInput['sourceEntryId']): NotificationLocalOutboxRecord {
  const record = NotificationLocalOutboxAdapterProofReadModel.records.find(
    (candidate) => candidate.entryId === sourceEntryId
  );
  if (record === undefined) {
    throw new Error(`Missing notification local outbox source record: ${sourceEntryId}`);
  }
  return record;
}

function schedulerRecord(input: SchedulerInput): NotificationLocalOutboxSchedulerRecord {
  const source = sourceRecordFor(input.sourceEntryId);

  return NotificationLocalOutboxSchedulerRecordSchema.parse({
    schedulerEntryId: `notification-local-outbox-scheduler-${input.schedulerState}-${source.entryId}`,
    sourceEntryId: source.entryId,
    sourceState: source.state,
    schedulerState: input.schedulerState,
    reasonCode: source.envelope.reasonCode,
    providerChannel: source.envelope.providerChannel,
    severity: source.envelope.severity,
    schedulerDecisionRef: input.schedulerDecisionRef,
    schedulerArtifactRef: NotificationLocalOutboxSchedulerArtifactRef,
    sourceOutboxFileRef: source.outboxFileRef,
    localDataPathRef: source.localDataPathRef,
    schedulerNowAt: NotificationLocalOutboxSchedulerProofNow,
    nextAttemptAt: input.nextAttemptAt,
    quietHoursWindow: input.quietHoursWindow,
    retryWindow: input.retryWindow,
    deadLetterReviewRef: input.deadLetterReviewRef,
    providerReceiptRef: input.providerReceiptRef,
    manualProofRequirements: input.manualProofRequirements,
    manualActionRequired: input.manualActionRequired,
    parentOwnedArtifactWritten: true,
    rawChildEvidenceIncluded: false,
    rawUrlOrTitleIncluded: false,
    rawMessageTextIncluded: false,
    screenshotOrReportIncluded: false,
    providerDeliveryAttempted: false,
    providerDeliveryObserved: false,
    providerReceiptIngested: false,
    providerCredentialsStored: false,
    cloudRoutingClaimed: false,
    parentNotificationUiClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    sensitiveProviderMetadataStored: false,
    schedulerPayloadPreview: input.schedulerPayloadPreview,
  });
}

export const NotificationLocalOutboxSchedulerProofReadModel = NotificationLocalOutboxSchedulerProofSchema.parse({
  schemaVersion: 'notification-local-outbox-scheduler-proof',
  contractVersion: ParentContractSchemaVersion.V0_6,
  readModelId: 'notification-local-outbox-scheduler-proof',
  generatedAt: NotificationLocalOutboxSchedulerProofTimestamp,
  schedulerNowAt: NotificationLocalOutboxSchedulerProofNow,
  schedulerArtifactRootRef: 'parent-owned-local-notification-outbox-scheduler-root',
  sourceAdapterReadModelId: NotificationLocalOutboxAdapterProofReadModel.readModelId,
  records: NotificationLocalOutboxSchedulerProofRows.map((row) => schedulerRecord(row)),
  nonClaims: RequiredNotificationLocalOutboxSchedulerNonClaims,
  providerDeliveryRuntimeClaimed: false,
  providerReceiptIngestionClaimed: false,
  providerCredentialsClaimed: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  retryExecutionRuntimeClaimed: false,
  quietHoursTimerRuntimeClaimed: false,
  productionDurableOutboxStorageClaimed: false,
});

export const decodeNotificationLocalOutboxSchedulerRecord = (input: unknown): NotificationLocalOutboxSchedulerRecord =>
  NotificationLocalOutboxSchedulerRecordSchema.parse(input);

export const decodeNotificationLocalOutboxSchedulerProof = (input: unknown): NotificationLocalOutboxSchedulerProof =>
  NotificationLocalOutboxSchedulerProofSchema.parse(input);

function textContainsForbiddenDetail(text: string): boolean {
  const lowerText = text.toLowerCase();
  return NotificationLocalOutboxForbiddenDetailFragments.some((fragment) => lowerText.includes(fragment));
}
