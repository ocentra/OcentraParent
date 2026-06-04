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
