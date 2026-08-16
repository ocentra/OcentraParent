/* generated from crates/schema/src/notification_local_outbox_ts.rs */

export const GeneratedRequiredNotificationLocalOutboxStates = [
  'queued-local',
  'deferred-quiet-hours',
  'retry-scheduled',
  'dead-lettered',
  'receipt-required',
  'manual-required',
] as const;

export const GeneratedRequiredNotificationLocalOutboxNonClaims = [
  'no-provider-delivery',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-raw-child-evidence',
  'no-sensitive-provider-metadata',
] as const;

export const GeneratedNotificationLocalOutboxForbiddenDetailFragments = [
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

export const GeneratedNotificationLocalOutboxKnownGaps = [
  'No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this rust-parent-runtime proof.',
  'No provider delivery execution, webhook receipt ingestion, credentials, cloud routing, or parent notification UI is claimed.',
  'No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the local outbox artifact.',
  'Quiet-hours scheduling, retry execution, dead-letter review, and receipt/manual-required handling remain adapter/runtime work.',
  'Durable production outbox storage, retention controls, parent-visible history, and physical provider smoke proof remain future work.',
] as const;

export const GeneratedNotificationLocalOutboxProviderChannels = ['push', 'email', 'sms', 'whatsapp', 'in-app'] as const;

export const GeneratedNotificationLocalOutboxProofTimestamp = '2026-06-04T01:31:47.023Z' as const;

export const GeneratedNotificationLocalOutboxProofFamily = {
  familyId: 'family-notification-local-outbox-proof-1',
} as const;

export const GeneratedNotificationLocalOutboxProofDevice = {
  deviceId: 'windows-child-device-notification-outbox-proof-1',
  childProfileId: 'child-notification-outbox-proof-1',
  label: 'Windows child device notification outbox proof',
  platform: 'windows',
} as const;

export const GeneratedNotificationLocalOutboxProofParentAction = {
  actionReferenceId: 'parent-action-notification-outbox-proof-1',
  actor: {
    actorId: 'parent-notification-outbox-proof-1',
    role: 'parent',
  },
  policyVersion: 'notification-local-outbox-proof-v1',
  createdAt: '2026-06-04T01:31:47.023Z',
} as const;

export const GeneratedNotificationLocalOutboxProofEvidenceRef = {
  evidenceReferenceId: 'notification-local-outbox-evidence-ref-1',
  kind: 'policy-decision',
  observedAt: '2026-06-04T01:31:47.023Z',
} as const;

export const GeneratedNotificationLocalOutboxProofRows = [
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

export const GeneratedRequiredNotificationLocalOutboxSchedulerStates = [
  'due-local',
  'held-quiet-hours',
  'retry-window-scheduled',
  'dead-letter-review',
  'receipt-required',
  'manual-required',
] as const;

export const GeneratedRequiredNotificationLocalOutboxSchedulerNonClaims = [
  'no-provider-delivery-execution',
  'no-provider-receipt-ingestion',
  'no-provider-credentials',
  'no-cloud-routing',
  'no-parent-notification-ui',
  'no-production-durable-outbox-storage',
  'no-sensitive-detail-storage',
] as const;

export const GeneratedNotificationLocalOutboxSchedulerProofTimestamp = '2026-06-04T02:28:51.667Z' as const;

export const GeneratedNotificationLocalOutboxSchedulerProofNow = '2026-06-04T02:28:51.667Z' as const;

export const GeneratedNotificationLocalOutboxSchedulerArtifactRef =
  'parent-owned-local-notification-outbox-scheduler-jsonl-ref' as const;

export const GeneratedNotificationLocalOutboxSchedulerKnownGaps = [
  'No push, email, SMS, WhatsApp, or in-app provider adapter is implemented by this rust-parent-runtime scheduler proof.',
  'No provider delivery execution, webhook receipt ingestion, provider credentials, cloud routing, or parent notification UI is claimed.',
  'No raw child evidence, raw URLs, titles, message text, screenshots, reports, provider tokens, or private paths are stored in the scheduler artifact.',
  'Scheduler decisions are deterministic rust-parent-runtime proof rows; no production timer loop, durable outbox database, provider retry worker, or receipt webhook is implemented.',
  'Parent-visible history, preferences, escalation controls, retention controls, and physical provider smoke proof remain future work.',
] as const;

export const GeneratedNotificationLocalOutboxSchedulerProofRows = [
  {
    sourceEntryId: 'notification-local-outbox-policy-violation-push-queued',
    schedulerState: 'due-local',
    schedulerDecisionRef: 'scheduler-due-policy-violation-ref',
    nextAttemptAt: '2026-06-04T02:28:51.667Z',
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
