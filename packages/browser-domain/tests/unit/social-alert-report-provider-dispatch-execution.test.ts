import { describe, expect, it } from 'vitest';
import { buildSocialAlertReportLocalOutboxBridgeReadModel } from '../../src/social-alert-report-local-outbox-bridge';
import {
  buildSocialAlertReportProviderDispatchExecutionReadModel,
  SocialAlertReportProviderDispatchExecutionReadModelSchema,
  SocialAlertReportProviderDispatchExecutionRowSchema,
  SocialAlertReportProviderDispatchExecutionState,
  summarizeSocialAlertReportProviderDispatchExecution,
} from '../../src/social-alert-report-provider-dispatch-execution';
import {
  buildSocialAlertReportProviderPreflightReadModel,
  SocialAlertReportProviderPreflightStatus,
} from '@ocentra-parent/schema-domain/social-alert-report-provider-preflight-proof';
import { buildSocialAlertReportProviderReceiptBoundaryReadModel } from '@ocentra-parent/schema-domain/social-alert-report-provider-receipt-boundary-proof';
import { buildSocialAlertReportProviderStatusHandoffReadModel } from '@ocentra-parent/schema-domain/social-alert-report-provider-status-handoff-proof';
import {
  SocialAlertReportAdapterDispatchState,
  SocialAlertReportDeliveryClaimState,
  SocialAlertReportIntentKind,
  SocialAlertReportIntentStatus,
  SocialAlertReportParentCopyToken,
  SocialAlertReportPayloadField,
  SocialAlertReportReasonCode,
} from '@ocentra-parent/schema-domain/social-alert-report-intent';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
  ParentPlatform,
} from '@ocentra-parent/schema-domain/family-reference-primitives';

const Timestamp = '2026-06-08T22:47:00Z';
const MinimalPayloadFields = Object.values(SocialAlertReportPayloadField);

const BridgeOptions = {
  family: { familyId: 'family-social-provider-dispatch' },
  parentAction: {
    actionReferenceId: 'parent-action-social-provider-dispatch',
    actor: { actorId: 'parent-social-provider-dispatch', role: ParentActorRole.Parent },
    policyVersion: 'policy-social-provider-dispatch-v1',
    createdAt: Timestamp,
  },
  generatedAt: Timestamp,
  bridgeId: 'social-alert-report-local-outbox-bridge-dispatch',
  outboxRootRef: 'parent-owned-social-provider-dispatch-outbox-root',
  outboxFileRef: 'parent-owned-social-provider-dispatch-outbox-jsonl-ref',
  localDataPathRef: 'parent-owned-social-provider-dispatch-local-data-path-ref',
} as const;

const BaseIntent = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  alertReportIntentId: 'social-provider-dispatch-high-risk',
  intentKind: SocialAlertReportIntentKind.HighRiskSignal,
  intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
  priority: 'urgent',
  severity: 'critical',
  device: {
    deviceId: 'device-social-provider-dispatch',
    childProfileId: 'child-social-provider-dispatch',
    label: 'Study Phone',
    platform: ParentPlatform.Android,
  },
  notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
  providerChannelPreference: 'in-app',
  parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
  parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
  parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
  dashboardPanelRefs: ['panel-feed-video-gates'],
  explanationSnapshotRef: 'social-explanation-snapshot-provider-dispatch',
  explanationEventRefs: ['social-explanation-event-provider-dispatch'],
  evidenceReferences: [
    {
      evidenceReferenceId: 'evidence-social-provider-dispatch',
      kind: ParentEvidenceReferenceKind.PolicyDecision,
      observedAt: Timestamp,
    },
  ],
  policyRefs: ['policy-ref-social-provider-dispatch'],
  auditRefs: ['audit-ref-social-provider-dispatch'],
  parentReportRef: null,
  parentActionRef: null,
  localOutboxRecordRef: 'local-outbox-social-provider-dispatch',
  providerAttemptRefs: [],
  providerReceiptRefs: [],
  manualProofRequirements: [],
  minimalPayloadFields: MinimalPayloadFields,
  deliveryClaimState: SocialAlertReportDeliveryClaimState.LocalOutboxOnly,
  rawAccountDataIncluded: false,
  rawVideoContentIncluded: false,
  rawMessageContentIncluded: false,
  screenshotIncluded: false,
  providerDeliveryAttempted: false,
  providerDeliveryObserved: false,
  providerReceiptIngested: false,
  cloudRoutingClaimed: false,
  parentNotificationUiClaimed: false,
  reportDeliveryClaimed: false,
  finalPolicyDecisionClaimed: false,
  enforcementClaimed: false,
  adapterDispatchState: SocialAlertReportAdapterDispatchState.NotDispatched,
  adapterActionClaimed: false,
  createdAt: Timestamp,
} as const;

describe('social alert/report provider dispatch execution', () => {
  it('prepares local dispatch packets only for provider-dispatch-required rows with parsed outbox records', () => {
    const readModel = buildDispatchExecutionReadModel();
    const summary = summarizeSocialAlertReportProviderDispatchExecution(readModel);
    const ready = readModel.rows[0];

    expect(summary.localDispatchPacketReadyCount).toBe(1);
    expect(summary.manualRequiredCount).toBe(1);
    expect(summary.providerUnavailableCount).toBe(1);
    expect(ready?.dispatchExecutionState).toBe(
      SocialAlertReportProviderDispatchExecutionState.LocalDispatchPacketReady
    );
    expect(ready?.dispatchPacket?.outboxEntryRef).toBe('local-outbox-social-provider-dispatch');
    expect(ready?.dispatchPacket?.providerChannel).toBe('in-app');
    expect(ready?.dispatchPacket?.rawChildEvidenceIncluded).toBe(false);
    expect(ready?.dispatchPacket?.rawUrlOrTitleIncluded).toBe(false);
    expect(ready?.providerDeliveryAttempted).toBe(false);
    expect(summary.providerDeliveryAttempted).toBe(false);
    expect(summary.providerReceiptIngested).toBe(false);
    expect(summary.enforcementClaimed).toBe(false);
  });

  it('keeps manual and unavailable rows packetless and visible for manual follow-up', () => {
    const readModel = buildDispatchExecutionReadModel();
    const manual = readModel.rows[1];
    const unavailable = readModel.rows[2];

    expect(manual?.dispatchExecutionState).toBe(SocialAlertReportProviderDispatchExecutionState.ManualRequired);
    expect(manual?.dispatchPacket).toBeNull();
    expect(manual?.manualProofRequirements).toEqual(['manual-proof-social-provider-dispatch-required']);
    expect(unavailable?.dispatchExecutionState).toBe(
      SocialAlertReportProviderDispatchExecutionState.ProviderUnavailable
    );
    expect(unavailable?.dispatchPacket).toBeNull();
    expect(unavailable?.manualProofRequirements).toEqual([
      'social-provider-dispatch-provider-unavailable-social-provider-dispatch-unavailable',
    ]);
  });

  it('rejects forged packets, provider delivery overclaims, and missing local outbox records', () => {
    const readModel = buildDispatchExecutionReadModel();
    const ready = readModel.rows[0];

    expect(
      SocialAlertReportProviderDispatchExecutionReadModelSchema.safeParse({
        ...readModel,
        providerDeliveryAttempted: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderDispatchExecutionRowSchema.safeParse({
        ...ready,
        providerDeliveryObserved: true,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportProviderDispatchExecutionRowSchema.safeParse({
        ...ready,
        dispatchPacket: {
          ...ready?.dispatchPacket,
          outboxEntryRef: 'forged-outbox-entry-ref',
        },
      }).success
    ).toBe(false);

    const missingOutbox = buildSocialAlertReportProviderDispatchExecutionReadModel(
      { generatedAt: Timestamp, dispatchExecutionId: 'social-provider-dispatch-missing-outbox' },
      buildReceiptBoundaryReadModel(),
      []
    );
    expect(missingOutbox.localDispatchPacketReadyCount).toBe(0);
    expect(missingOutbox.manualRequiredCount).toBe(2);
  });
});

function buildDispatchExecutionReadModel() {
  return buildSocialAlertReportProviderDispatchExecutionReadModel(
    {
      generatedAt: Timestamp,
      dispatchExecutionId: 'social-alert-report-provider-dispatch-execution',
    },
    buildReceiptBoundaryReadModel(),
    buildLocalOutboxRecords()
  );
}

function buildLocalOutboxRecords() {
  const bridge = buildSocialAlertReportLocalOutboxBridgeReadModel(BridgeOptions, [
    BaseIntent,
    manualRequiredIntent(),
    unavailableIntent(),
  ]);

  return bridge.rows.flatMap((row) => (row.outboxRecord === null ? [] : [row.outboxRecord]));
}

function buildReceiptBoundaryReadModel() {
  const preflight = buildSocialAlertReportProviderPreflightReadModel(
    {
      generatedAt: Timestamp,
      providerPreflightId: 'social-alert-report-provider-preflight-for-dispatch',
      sourceContractRefs: ['social-alert-report-intent', 'social-alert-report-provider-dispatch-execution'],
    },
    [BaseIntent, manualRequiredIntent(), unavailableIntent()]
  );
  const statusHandoff = buildSocialAlertReportProviderStatusHandoffReadModel(
    {
      generatedAt: Timestamp,
      handoffId: 'social-alert-report-provider-status-handoff-for-dispatch',
      sourceContractRefs: [
        'social-alert-report-provider-preflight-proof',
        'v0-8-notification-provider-status-boundary',
      ],
    },
    preflight
  );

  expect(statusHandoff.rows.map((row) => row.sourcePreflightStatus)).toEqual([
    SocialAlertReportProviderPreflightStatus.ProviderAdapterRequired,
    SocialAlertReportProviderPreflightStatus.ManualRequired,
    SocialAlertReportProviderPreflightStatus.Unavailable,
  ]);

  return buildSocialAlertReportProviderReceiptBoundaryReadModel(
    {
      generatedAt: Timestamp,
      receiptBoundaryId: 'social-alert-report-provider-receipt-boundary-for-dispatch',
      sourceContractRefs: [
        'social-alert-report-provider-status-handoff-proof',
        'v0-8-notification-provider-status-boundary',
      ],
    },
    statusHandoff
  );
}

function manualRequiredIntent() {
  return {
    ...BaseIntent,
    alertReportIntentId: 'social-provider-dispatch-manual-required',
    intentKind: SocialAlertReportIntentKind.ManualRequired,
    intentStatus: SocialAlertReportIntentStatus.ManualRequired,
    priority: 'attention',
    severity: 'warning',
    notificationReasonCode: SocialAlertReportReasonCode.ManualRequired,
    parentTitleToken: SocialAlertReportParentCopyToken.ManualRequiredTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.ManualRequiredBody,
    parentActionToken: SocialAlertReportParentCopyToken.ReviewManuallyAction,
    localOutboxRecordRef: null,
    deliveryClaimState: SocialAlertReportDeliveryClaimState.ManualRequired,
    manualProofRequirements: ['manual-proof-social-provider-dispatch-required'],
  } as const;
}

function unavailableIntent() {
  return {
    ...manualRequiredIntent(),
    alertReportIntentId: 'social-provider-dispatch-unavailable',
    intentKind: SocialAlertReportIntentKind.CapabilityUnavailable,
    intentStatus: SocialAlertReportIntentStatus.Unavailable,
    notificationReasonCode: SocialAlertReportReasonCode.CapabilityUnavailable,
    parentTitleToken: SocialAlertReportParentCopyToken.UnavailableTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.UnavailableBody,
    manualProofRequirements: ['manual-proof-social-provider-dispatch-unavailable'],
  } as const;
}
