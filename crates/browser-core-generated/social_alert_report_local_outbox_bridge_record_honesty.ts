/* generated support for crates/browser-core/src/social_alert_report_local_outbox_bridge.rs */

const NotificationLocalOutboxForbiddenDetailFragments = [
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

const AdapterRecordClaimFlags = [
  'providerDeliveryAttempted',
  'providerDeliveryObserved',
  'providerReceiptIngested',
  'providerCredentialsStored',
  'cloudRoutingClaimed',
  'parentNotificationUiClaimed',
  'sensitiveProviderMetadataStored',
] as const;

type NotificationEnvelopeCandidate = {
  readonly sensitiveDetailMinimized: boolean;
  readonly rawChildEvidenceIncluded: boolean;
  readonly rawUrlOrTitleIncluded: boolean;
  readonly rawMessageTextIncluded: boolean;
  readonly screenshotOrReportIncluded: boolean;
  readonly evidenceRefs: ReadonlyArray<unknown>;
  readonly policyRefs: ReadonlyArray<unknown>;
  readonly auditRefs: ReadonlyArray<unknown>;
  readonly providerPayloadPreview: string;
};

type NotificationOutboxRecordCandidate = {
  readonly state: string;
  readonly outboxFileRef: string;
  readonly localDataPathRef: string;
  readonly deliveryClaimState: string;
  readonly visibleAfterAt: string | null;
  readonly retryAttemptCount: number;
  readonly quietHoursRef: string | null;
  readonly retryPolicyRef: string | null;
  readonly deadLetterRef: string | null;
  readonly providerReceiptRef: string | null;
  readonly manualProofRequirements: ReadonlyArray<unknown>;
  readonly manualActionRequired: boolean;
  readonly providerDeliveryAttempted: boolean;
  readonly providerDeliveryObserved: boolean;
  readonly providerReceiptIngested: boolean;
  readonly providerCredentialsStored: boolean;
  readonly cloudRoutingClaimed: boolean;
  readonly parentNotificationUiClaimed: boolean;
  readonly sensitiveProviderMetadataStored: boolean;
};

export function notificationEnvelopeIsSafe(envelope: NotificationEnvelopeCandidate): boolean {
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

export function notificationOutboxRecordIsSafe(record: NotificationOutboxRecordCandidate): boolean {
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

function textContainsForbiddenDetail(value: string): boolean {
  return NotificationLocalOutboxForbiddenDetailFragments.some((fragment) => value.includes(fragment));
}
