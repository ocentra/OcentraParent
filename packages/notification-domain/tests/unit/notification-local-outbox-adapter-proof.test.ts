import { describe, expect, it } from 'vitest';
import {
  NotificationLocalOutboxAdapterProofReadModel,
  NotificationLocalOutboxAdapterProofSchema,
  NotificationLocalOutboxForbiddenDetailFragments,
  NotificationLocalOutboxKnownGaps,
  NotificationLocalOutboxMinimalAlertEnvelopeSchema,
  NotificationLocalOutboxRecordSchema,
  summarizeNotificationLocalOutboxChannels,
  summarizeNotificationLocalOutboxStates,
} from '@ocentra-parent/notification-domain/notification-local-outbox-adapter-proof';

describe('notification local outbox adapter proof', () => {
  acceptsTheLocalOutboxReadModel();
  rejectsProviderDeliveryAndCustodyOverclaims();
  rejectsSensitiveEnvelopeAndStateMismatches();
  rejectsProofsMissingStatesOrNonClaims();
});

function acceptsTheLocalOutboxReadModel(): void {
  it('covers local queued deferred retry dead-letter receipt and manual states across provider channels', () => {
    const proof = NotificationLocalOutboxAdapterProofSchema.parse(NotificationLocalOutboxAdapterProofReadModel);

    expect(proof.readModelId).toBe('notification-local-outbox-adapter-proof');
    expect(summarizeNotificationLocalOutboxStates(proof.records)).toEqual({
      'queued-local': 1,
      'deferred-quiet-hours': 1,
      'retry-scheduled': 1,
      'dead-lettered': 1,
      'receipt-required': 1,
      'manual-required': 1,
    });
    expect(summarizeNotificationLocalOutboxChannels(proof.records)).toEqual({
      push: 1,
      email: 1,
      sms: 1,
      whatsapp: 1,
      'in-app': 2,
    });
    expect(NotificationLocalOutboxKnownGaps).toContain(
      'No provider delivery execution, webhook receipt ingestion, credentials, cloud routing, or parent notification UI is claimed.'
    );
  });
}

function rejectsProviderDeliveryAndCustodyOverclaims(): void {
  it('rejects rows or proof documents that claim provider delivery receipts credentials cloud routing or UI', () => {
    const queued = recordFor('queued-local');
    const proof = NotificationLocalOutboxAdapterProofReadModel;

    for (const invalidRecord of [
      { ...queued, providerDeliveryAttempted: true },
      { ...queued, providerDeliveryObserved: true },
      { ...queued, providerReceiptIngested: true },
      { ...queued, providerCredentialsStored: true },
      { ...queued, cloudRoutingClaimed: true },
      { ...queued, parentNotificationUiClaimed: true },
      { ...queued, sensitiveProviderMetadataStored: true },
    ]) {
      expect(NotificationLocalOutboxRecordSchema.safeParse(invalidRecord).success).toBe(false);
    }

    for (const invalidProof of [
      { ...proof, providerDeliveryRuntimeClaimed: true },
      { ...proof, providerReceiptIngestionClaimed: true },
      { ...proof, providerCredentialsClaimed: true },
      { ...proof, cloudRoutingClaimed: true },
      { ...proof, parentNotificationUiClaimed: true },
    ]) {
      expect(NotificationLocalOutboxAdapterProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsSensitiveEnvelopeAndStateMismatches(): void {
  it('rejects sensitive envelope detail and incoherent quiet-hours retry dead-letter receipt states', () => {
    const queued = recordFor('queued-local');
    const deferred = recordFor('deferred-quiet-hours');
    const retry = recordFor('retry-scheduled');
    const deadLetter = recordFor('dead-lettered');
    const receipt = recordFor('receipt-required');

    for (const invalidEnvelope of [
      { ...queued.envelope, rawChildEvidenceIncluded: true },
      { ...queued.envelope, rawUrlOrTitleIncluded: true },
      { ...queued.envelope, rawMessageTextIncluded: true },
      { ...queued.envelope, screenshotOrReportIncluded: true },
      {
        ...queued.envelope,
        providerPayloadPreview: `minimal alert ${NotificationLocalOutboxForbiddenDetailFragments[1]}`,
      },
      { ...queued.envelope, evidenceRefs: [] },
    ]) {
      expect(NotificationLocalOutboxMinimalAlertEnvelopeSchema.safeParse(invalidEnvelope).success).toBe(false);
    }

    for (const invalidRecord of [
      { ...deferred, quietHoursRef: null },
      { ...retry, retryAttemptCount: 0 },
      { ...retry, retryPolicyRef: null },
      { ...deadLetter, deadLetterRef: null },
      { ...deadLetter, manualActionRequired: false },
      { ...receipt, providerReceiptRef: null },
      { ...queued, providerReceiptRef: 'unexpected-provider-receipt-ref' },
    ]) {
      expect(NotificationLocalOutboxRecordSchema.safeParse(invalidRecord).success).toBe(false);
    }
  });
}

function rejectsProofsMissingStatesOrNonClaims(): void {
  it('rejects proof documents that omit required local states channels or non-claims', () => {
    const proof = NotificationLocalOutboxAdapterProofReadModel;

    expect(
      NotificationLocalOutboxAdapterProofSchema.safeParse({
        ...proof,
        records: proof.records.filter((record) => record.state !== 'dead-lettered'),
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxAdapterProofSchema.safeParse({
        ...proof,
        records: proof.records.filter((record) => record.envelope.providerChannel !== 'sms'),
      }).success
    ).toBe(false);
    expect(
      NotificationLocalOutboxAdapterProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-cloud-routing'),
      }).success
    ).toBe(false);
  });
}

function recordFor(state: string) {
  const record = NotificationLocalOutboxAdapterProofReadModel.records.find((candidate) => candidate.state === state);
  if (record === undefined) {
    throw new Error(`Missing local notification outbox record: ${state}`);
  }
  return record;
}
