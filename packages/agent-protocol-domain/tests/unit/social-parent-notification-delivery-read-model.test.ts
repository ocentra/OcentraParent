import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentProtocolDefaults,
  parseAgentSocialParentNotificationDeliveryReadModelEvent,
  SocialParentNotificationDeliveryReadModelSnapshotSchema,
  type AgentEventEnvelope,
} from '../../src/contracts';

const Timestamp = '2026-06-08T11:45:00Z';

describe('social parent notification delivery read model adapter', () => {
  it('parses service-backed readiness rows without delivery claims', () => {
    const result = parseAgentSocialParentNotificationDeliveryReadModelEvent(eventWithSnapshot(snapshot()));

    expect(result.ok).toBe(true);
    if (!result.ok) {
      return;
    }
    expect(result.value.rows).toHaveLength(3);
    expect(result.value.parentReportStatusReadyCount).toBe(1);
    expect(result.value.manualRequiredCount).toBe(1);
    expect(result.value.unavailableCount).toBe(1);
    expect(result.value.parentNotificationUiDeliveryClaimed).toBe(false);
    expect(result.value.externalRuntimeReportDeliveryClaimed).toBe(false);
    expect(result.value.finalPolicyExecutionClaimed).toBe(false);
    expect(result.value.enforcementClaimed).toBe(false);
  });

  it('rejects wrong event missing json invalid json and unsafe readiness claims', () => {
    expect(
      parseAgentSocialParentNotificationDeliveryReadModelEvent({
        ...eventWithSnapshot(snapshot()),
        event: AgentEvent.HealthReported,
      }).ok
    ).toBe(false);
    expect(parseAgentSocialParentNotificationDeliveryReadModelEvent(eventWithPayload({})).ok).toBe(false);
    expect(
      parseAgentSocialParentNotificationDeliveryReadModelEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel]: '{',
        })
      ).ok
    ).toBe(false);
    expect(
      SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [{ ...readyRow(), parentNotificationUiDelivered: true }],
      }).success
    ).toBe(false);
    expect(
      SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        parentReportStatusReadyCount: 2,
      }).success
    ).toBe(false);
    expect(
      SocialParentNotificationDeliveryReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [{ ...readyRow(), reportReceiptRef: null }],
      }).success
    ).toBe(false);
  });
});

function eventWithSnapshot(value: unknown): AgentEventEnvelope {
  return eventWithPayload({
    [AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel]: JSON.stringify(value),
  });
}

function eventWithPayload(payload: AgentEventEnvelope['payload']): AgentEventEnvelope {
  return {
    schemaVersion: 1,
    eventId: 'event-social-parent-notification-delivery',
    correlationId: 'command-social-parent-notification-delivery',
    sentAt: Timestamp,
    source: {
      peerId: 'agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal',
      role: 'portal',
    },
    event: AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
    severity: 'info',
    payload,
    snapshot: null,
  };
}

function snapshot() {
  return {
    schemaVersion: 'social-parent-notification-delivery-read-model',
    readinessId: 'social-parent-notification-delivery-readiness-service',
    generatedAt: Timestamp,
    sourceReportWriterProofRef: 'social-report-writer-delivery-proof-service',
    rows: [readyRow(), manualRow(), unavailableRow()],
    nonClaims: [
      'no-parent-notification-ui-delivery',
      'no-external-runtime-report-delivery',
      'no-provider-delivery',
      'no-provider-receipt-ingestion',
      'no-final-policy-execution',
      'no-enforcement',
    ],
    parentReportStatusReadyCount: 1,
    manualRequiredCount: 1,
    unavailableCount: 1,
    parentNotificationUiDeliveryClaimed: false,
    externalRuntimeReportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  };
}

function readyRow() {
  return baseRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-ready-high-risk-service',
    parentVisibleReportStatusRef: 'social-parent-visible-report-status-high-risk-service',
    parentReportRef: 'social-parent-report-high-risk-service',
    reportArtifactRef: 'social-report-artifact-high-risk-service',
    reportReceiptRef: 'social-report-receipt-high-risk-service',
    manualProofRequirements: [],
    notificationDeliveryReadinessState: 'parent-report-status-ready',
    reportDeliveryExecutionState: 'parent-owned-report-ready',
    parentOwnedReportArtifactWritten: true,
    parentOwnedReportReceiptRecorded: true,
  });
}

function manualRow() {
  return baseRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-manual-required-service',
    parentVisibleReportStatusRef: 'social-parent-visible-report-status-manual-required-service',
    parentReportRef: null,
    reportArtifactRef: null,
    reportReceiptRef: null,
    manualProofRequirements: ['manual-parent-notification-ui-runtime-proof-required'],
    notificationDeliveryReadinessState: 'manual-required',
    reportDeliveryExecutionState: 'manual-required',
    parentOwnedReportArtifactWritten: false,
    parentOwnedReportReceiptRecorded: false,
  });
}

function unavailableRow() {
  return baseRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-unavailable-service',
    parentVisibleReportStatusRef: null,
    parentReportRef: null,
    reportArtifactRef: null,
    reportReceiptRef: null,
    manualProofRequirements: ['external-report-delivery-runtime-unavailable'],
    notificationDeliveryReadinessState: 'unavailable',
    reportDeliveryExecutionState: 'unavailable',
    parentOwnedReportArtifactWritten: false,
    parentOwnedReportReceiptRecorded: false,
  });
}

function baseRow(overrides: {
  readonly notificationDeliveryReadinessRowId: string;
  readonly parentVisibleReportStatusRef: string | null;
  readonly parentReportRef: string | null;
  readonly reportArtifactRef: string | null;
  readonly reportReceiptRef: string | null;
  readonly manualProofRequirements: readonly string[];
  readonly notificationDeliveryReadinessState: 'parent-report-status-ready' | 'manual-required' | 'unavailable';
  readonly reportDeliveryExecutionState: 'parent-owned-report-ready' | 'manual-required' | 'unavailable';
  readonly parentOwnedReportArtifactWritten: boolean;
  readonly parentOwnedReportReceiptRecorded: boolean;
}) {
  return {
    ...overrides,
    sourceReportWriterDeliveryRowRef: 'social-report-writer-delivery-row-service',
    sourceIntentRef: 'social-alert-report-high-risk-service',
    parentNotificationUiRef: null,
    sourceEvidenceRefs: ['evidence-social-route-gate'],
    sourcePolicyRefs: ['policy-ref-social-high-risk'],
    sourceAuditRefs: ['audit-ref-social-alert-report'],
    parentNotificationUiDelivered: false,
    externalRuntimeReportDeliveryClaimed: false,
    providerDeliveryAttempted: false,
    providerReceiptIngested: false,
    finalPolicyDecisionClaimed: false,
    enforcementClaimed: false,
    createdAt: Timestamp,
  };
}
