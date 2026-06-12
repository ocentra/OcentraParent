import { describe, expect, it } from 'vitest';
import { NotificationAuditHistoryEntrySchema } from '../../src/notification-audit-history';
import {
  NotificationAuditHistoryHandoffReadModelSchema,
  NotificationAuditHistoryHandoffSourceRowSchema,
  NotificationAuditHistoryHandoffSourceStatus,
  buildNotificationAuditHistoryHandoffReadModel,
} from '../../src/notification-audit-history-handoff';

const Timestamp = '2026-06-05T02:17:00Z';
const Options = {
  handoffReadModelId: 'app-game-notification-audit-history-bridge-proof',
  generatedAt: Timestamp,
  sourceReadModelRef: 'app-game-notification-local-outbox-bridge-proof',
  sourceContractRefs: [
    'app-game-notification-local-outbox-bridge',
    'notification-audit-history-contract',
    'reports-notifications-sync-feature-doc',
  ],
} as const;

describe('notification audit history handoff', () => {
  registersAuditEntryMappingTest();
  registersRedactionBoundaryTest();
  registersUnsafeOverclaimRejectionTest();
});

function registersAuditEntryMappingTest() {
  it('builds audit-history entries for queued manual-required and unavailable source rows', () => {
    const readModel = buildNotificationAuditHistoryHandoffReadModel(Options, sourceRows());

    expect(readModel.queuedAuditEntryCount).toBe(2);
    expect(readModel.manualRequiredAuditEntryCount).toBe(1);
    expect(readModel.unavailableAuditEntryCount).toBe(1);
    expect(readModel.auditHistoryEntries.map((entry) => entry.providerStatus)).toEqual([
      'queued',
      'queued',
      'manual-required',
      'unavailable',
    ]);
    expect(readModel.auditHistoryEntries.map((entry) => entry.retryLifecycleState)).toEqual([
      'not-scheduled',
      'not-scheduled',
      'manual-review-required',
      'provider-unavailable',
    ]);
    expect(readModel.auditHistoryEntries.map((entry) => entry.notificationHistoryUiClaimed)).toEqual([
      false,
      false,
      false,
      false,
    ]);
    expect(readModel.sourceRows.map((row) => row.policyRefs)).toEqual([
      ['policy-ref-time-limit'],
      ['policy-ref-suspicious-unknown'],
      ['policy-ref-manual-required'],
      ['policy-ref-unavailable'],
    ]);
  });
}

function registersRedactionBoundaryTest() {
  it('keeps minimal payload fields and child data custody boundaries on each audit row', () => {
    const readModel = buildNotificationAuditHistoryHandoffReadModel(Options, sourceRows());

    for (const entry of readModel.auditHistoryEntries) {
      expect(entry.redactionSafePayloadFields).toEqual([
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
      ]);
      expect(entry.payloadRedactionState).toBe('minimal-operational-fields-only');
      expect(entry.childDataCustodyState).toBe('no-ocentra-hosted-child-data');
      expect(entry.rawChildDataIncluded).toBe(false);
      expect(entry.rawEvidencePayloadIncluded).toBe(false);
      expect(entry.ocentraHostedChildDataStored).toBe(false);
    }
  });
}

function registersUnsafeOverclaimRejectionTest() {
  it('rejects unsafe source rows and provider runtime overclaims', () => {
    const readModel = buildNotificationAuditHistoryHandoffReadModel(Options, sourceRows());
    const queued = readModel.auditHistoryEntries[0];

    expect(
      NotificationAuditHistoryHandoffSourceRowSchema.safeParse({
        ...sourceRows()[0],
        sourceOutboxRecordRef: null,
      }).success
    ).toBe(false);
    expect(
      NotificationAuditHistoryHandoffReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryRuntimeClaimed: true,
      }).success
    ).toBe(false);
    expect(
      NotificationAuditHistoryEntrySchema.safeParse({
        ...queued,
        sendAttemptExecuted: true,
      }).success
    ).toBe(false);
  });
}

function sourceRows() {
  return [
    sourceRow('time-limit', NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox),
    sourceRow('suspicious-unknown', NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox),
    sourceRow('manual-required', NotificationAuditHistoryHandoffSourceStatus.ManualRequired),
    sourceRow('unavailable', NotificationAuditHistoryHandoffSourceStatus.Unavailable),
  ] as const;
}

function sourceRow(suffix: string, sourceStatus: NotificationAuditHistoryHandoffSourceStatus) {
  const linked = sourceStatus === NotificationAuditHistoryHandoffSourceStatus.QueuedLocalOutbox;
  return {
    handoffEntryId: `app-game-notification-audit-${suffix}`,
    sourceStatus,
    sourceNotificationIntentRef: `notification-intent-${suffix}`,
    sourceOutboxRecordRef: linked ? `local-outbox-record-${suffix}` : null,
    providerChannelRef: linked ? 'in-app' : 'manual',
    reasonCodeRef: `app-game-${suffix}`,
    auditRefs: [`audit-ref-${suffix}`],
    evidenceRefs: [`evidence-ref-${suffix}`],
    policyRefs: [`policy-ref-${suffix}`],
    blockedReasonRefs: linked ? [] : [`manual-proof-required-${suffix}`],
  } as const;
}
