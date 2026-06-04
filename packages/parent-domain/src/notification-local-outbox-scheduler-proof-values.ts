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
