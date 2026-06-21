import { describe, expect, it } from 'vitest';
import {
  dueNotificationLocalOutboxSchedulerRecords,
  NotificationLocalOutboxSchedulerKnownGaps,
  NotificationLocalOutboxSchedulerProofReadModel,
  NotificationLocalOutboxSchedulerProofSchema,
  NotificationLocalOutboxSchedulerRecordSchema,
  summarizeNotificationLocalOutboxSchedulerChannels,
  summarizeNotificationLocalOutboxSchedulerStates,
} from '@ocentra-parent/schema-domain/notification-local-outbox-scheduler-proof';

describe('notification local outbox scheduler proof', () => {
  acceptsTheSchedulerReadModel();
  rejectsProviderExecutionAndStorageOverclaims();
  rejectsSensitiveDetailsAndIncoherentSchedulerStates();
  rejectsProofsMissingSchedulerCoverageOrNonClaims();
});

function acceptsTheSchedulerReadModel(): void {
  it('covers due quiet-hours retry dead-letter receipt and manual scheduler states across provider channels', () => {
    const proof = NotificationLocalOutboxSchedulerProofSchema.parse(NotificationLocalOutboxSchedulerProofReadModel);
    const retry = schedulerRecordFor('retry-window-scheduled');
    const held = schedulerRecordFor('held-quiet-hours');

    expect(proof.readModelId).toBe('notification-local-outbox-scheduler-proof');
    expect(proof.sourceAdapterReadModelId).toBe('notification-local-outbox-adapter-proof');
    expect(summarizeNotificationLocalOutboxSchedulerStates(proof.records)).toEqual({
      'due-local': 1,
      'held-quiet-hours': 1,
      'retry-window-scheduled': 1,
      'dead-letter-review': 1,
      'receipt-required': 1,
      'manual-required': 1,
    });
    expect(summarizeNotificationLocalOutboxSchedulerChannels(proof.records)).toEqual({
      push: 1,
      email: 1,
      sms: 1,
      whatsapp: 1,
      'in-app': 2,
    });
    expect(dueNotificationLocalOutboxSchedulerRecords(proof.records).map((record) => record.sourceEntryId)).toEqual([
      'notification-local-outbox-policy-violation-push-queued',
    ]);
    expect(held.nextAttemptAt).toBe('2026-06-04T12:00:00.000Z');
    expect(held.quietHoursWindow?.endsAt).toBe('2026-06-04T12:00:00.000Z');
    expect(retry.nextAttemptAt).toBe('2026-06-04T02:38:51.667Z');
    expect(retry.retryWindow?.opensAt).toBe('2026-06-04T02:38:51.667Z');
    expect(retry.retryWindow?.attemptNumber).toBe(2);
    expect(NotificationLocalOutboxSchedulerKnownGaps).toContain(
      'Scheduler decisions are deterministic parent-domain proof rows; no production timer loop, durable outbox database, provider retry worker, or receipt webhook is implemented.'
    );
  });
}

function rejectsProviderExecutionAndStorageOverclaims(): void {
  it('rejects scheduler rows or proof documents that claim provider execution cloud UI credentials or durable storage', () => {
    const due = schedulerRecordFor('due-local');
    const proof = NotificationLocalOutboxSchedulerProofReadModel;

    for (const invalidRecord of [
      { ...due, providerDeliveryAttempted: true },
      { ...due, providerDeliveryObserved: true },
      { ...due, providerReceiptIngested: true },
      { ...due, providerCredentialsStored: true },
      { ...due, cloudRoutingClaimed: true },
      { ...due, parentNotificationUiClaimed: true },
      { ...due, productionDurableOutboxStorageClaimed: true },
      { ...due, sensitiveProviderMetadataStored: true },
    ]) {
      expect(NotificationLocalOutboxSchedulerRecordSchema.safeParse(invalidRecord).success).toBe(false);
    }

    for (const invalidProof of [
      { ...proof, providerDeliveryRuntimeClaimed: true },
      { ...proof, providerReceiptIngestionClaimed: true },
      { ...proof, providerCredentialsClaimed: true },
      { ...proof, cloudRoutingClaimed: true },
      { ...proof, parentNotificationUiClaimed: true },
      { ...proof, retryExecutionRuntimeClaimed: true },
      { ...proof, quietHoursTimerRuntimeClaimed: true },
      { ...proof, productionDurableOutboxStorageClaimed: true },
    ]) {
      expect(NotificationLocalOutboxSchedulerProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsSensitiveDetailsAndIncoherentSchedulerStates(): void {
  it('rejects raw detail leaks and mismatched due quiet-hours retry dead-letter receipt states', () => {
    const due = schedulerRecordFor('due-local');
    const held = schedulerRecordFor('held-quiet-hours');
    const retry = schedulerRecordFor('retry-window-scheduled');
    const deadLetter = schedulerRecordFor('dead-letter-review');
    const receipt = schedulerRecordFor('receipt-required');

    for (const invalidRecord of [
      { ...due, rawChildEvidenceIncluded: true },
      { ...due, rawUrlOrTitleIncluded: true },
      { ...due, rawMessageTextIncluded: true },
      { ...due, screenshotOrReportIncluded: true },
      { ...due, schedulerPayloadPreview: 'minimal alert https://sensitive.example/child-activity' },
      { ...due, nextAttemptAt: '2026-06-04T02:29:51.667Z' },
      { ...held, quietHoursWindow: null },
      { ...held, nextAttemptAt: '2026-06-04T11:59:59.000Z' },
      { ...retry, retryWindow: null },
      { ...retry, retryWindow: { ...retry.retryWindow, attemptNumber: 1 } },
      { ...deadLetter, deadLetterReviewRef: null },
      { ...receipt, providerReceiptRef: null },
    ]) {
      expect(NotificationLocalOutboxSchedulerRecordSchema.safeParse(invalidRecord).success).toBe(false);
    }
  });
}

function rejectsProofsMissingSchedulerCoverageOrNonClaims(): void {
  it('rejects proof documents that omit required scheduler states channels or non-claims', () => {
    const proof = NotificationLocalOutboxSchedulerProofReadModel;

    expect(
      NotificationLocalOutboxSchedulerProofSchema.safeParse({
        ...proof,
        records: proof.records.filter((record) => record.schedulerState !== 'dead-letter-review'),
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxSchedulerProofSchema.safeParse({
        ...proof,
        records: proof.records.filter((record) => record.providerChannel !== 'sms'),
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxSchedulerProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-cloud-routing'),
      }).success
    ).toBe(false);
  });
}

function schedulerRecordFor(schedulerState: string) {
  const record = NotificationLocalOutboxSchedulerProofReadModel.records.find(
    (candidate) => candidate.schedulerState === schedulerState
  );
  if (record === undefined) {
    throw new Error(`Missing local notification scheduler record: ${schedulerState}`);
  }
  return record;
}
