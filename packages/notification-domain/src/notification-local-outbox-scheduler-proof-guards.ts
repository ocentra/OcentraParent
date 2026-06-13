import {
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxProviderChannels,
} from './notification-local-outbox-adapter-proof-values';
import {
  RequiredNotificationLocalOutboxSchedulerNonClaims,
  RequiredNotificationLocalOutboxSchedulerStates,
} from '@ocentra-parent/notification-domain/notification-local-outbox-scheduler-proof-values';
import type {
  NotificationOutboxSchedulerProofCandidate,
  NotificationOutboxSchedulerRecordCandidate,
} from './notification-local-outbox-scheduler-proof-schemas';

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

function textContainsForbiddenDetail(text: string): boolean {
  const lowerText = text.toLowerCase();
  return NotificationLocalOutboxForbiddenDetailFragments.some((fragment) => lowerText.includes(fragment));
}
