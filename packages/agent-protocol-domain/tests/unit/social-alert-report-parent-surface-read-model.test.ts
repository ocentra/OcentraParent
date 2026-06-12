import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentProtocolDefaults,
  parseAgentSocialAlertReportParentSurfaceReadModelEvent,
  SocialAlertReportParentSurfaceReadModelSnapshotSchema,
  type AgentEventEnvelope,
} from '../../src/contracts';

const Timestamp = '2026-06-08T13:50:00Z';

describe('social alert/report parent-surface read model adapter', () => {
  it('parses service-backed parent-surface rows without UI delivery claims', () => {
    const result = parseAgentSocialAlertReportParentSurfaceReadModelEvent(eventWithSnapshot(snapshot()));

    expect(result.ok).toBe(true);
    if (!result.ok) {
      return;
    }
    expect(result.value.rows).toHaveLength(3);
    expect(result.value.manualActionRequiredCount).toBe(2);
    expect(result.value.unavailableVisibleCount).toBe(1);
    expect(result.value.parentNotificationUiRendered).toBe(false);
    expect(result.value.parentNotificationPreferenceUiRendered).toBe(false);
    expect(result.value.providerDeliveryRuntimeClaimed).toBe(false);
    expect(result.value.adapterDispatchClaimed).toBe(false);
    expect(result.value.connectorNativeRuntimeClaimed).toBe(false);
    expect(result.value.finalPolicyExecutionClaimed).toBe(false);
    expect(result.value.enforcementClaimed).toBe(false);
  });

  it('rejects wrong event missing json invalid json and dishonest parent-surface claims', () => {
    expect(
      parseAgentSocialAlertReportParentSurfaceReadModelEvent({
        ...eventWithSnapshot(snapshot()),
        event: AgentEvent.HealthReported,
      }).ok
    ).toBe(false);
    expect(parseAgentSocialAlertReportParentSurfaceReadModelEvent(eventWithPayload({})).ok).toBe(false);
    expect(
      parseAgentSocialAlertReportParentSurfaceReadModelEvent(
        eventWithPayload({
          [AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel]: '{',
        })
      ).ok
    ).toBe(false);
    expect(
      SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [{ ...manualRow(), parentNotificationUiRendered: true }],
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        manualActionRequiredCount: 1,
      }).success
    ).toBe(false);
    expect(
      SocialAlertReportParentSurfaceReadModelSnapshotSchema.safeParse({
        ...snapshot(),
        rows: [{ ...manualRow(), drillInRefs: [] }],
      }).success
    ).toBe(false);
  });
});

function eventWithSnapshot(value: unknown): AgentEventEnvelope {
  return eventWithPayload({
    [AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel]: JSON.stringify(value),
  });
}

function eventWithPayload(payload: AgentEventEnvelope['payload']): AgentEventEnvelope {
  return {
    schemaVersion: 1,
    eventId: 'event-social-alert-report-parent-surface',
    correlationId: 'command-social-alert-report-parent-surface',
    sentAt: Timestamp,
    source: {
      peerId: 'agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal',
      role: 'portal',
    },
    event: AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
    severity: 'info',
    payload,
    snapshot: null,
  };
}

function snapshot() {
  return {
    schemaVersion: 'social-alert-report-parent-surface-read-model',
    intentId: 'social-alert-report-parent-surface-service',
    generatedAt: Timestamp,
    sourceProviderStatusHandoffId: 'social-provider-status-handoff-service',
    sourcePreferenceStatusHandoffId: 'social-preference-status-handoff-service',
    rows: [highRiskRow(), manualRow(), unavailableRow()],
    manualActionRequiredCount: 2,
    unavailableVisibleCount: 1,
    historyVisibleCount: 3,
    preferenceSetupRequiredCount: 2,
    parentSurfaceNonClaims: [
      'no-parent-notification-ui-rendered',
      'no-parent-notification-preference-ui-rendered',
      'no-parent-frequency-control-ui-rendered',
      'no-parent-notification-history-ui-rendered',
      'no-provider-delivery-execution',
      'no-provider-receipt-ingestion',
      'no-provider-credentials',
      'no-cloud-routing',
      'no-child-delivery',
      'no-quiet-hours-timer-runtime',
      'no-retry-worker-runtime',
      'no-production-durable-outbox-storage',
      'no-adapter-dispatch',
      'no-report-delivery-execution',
      'no-final-policy-execution',
      'no-connector-native-runtime',
      'no-enforcement',
    ],
    parentNotificationUiRendered: false,
    parentNotificationPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentNotificationHistoryUiRendered: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionClaimed: false,
    providerCredentialsClaimed: false,
    cloudRoutingClaimed: false,
    childDeliveryClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    retryExecutionRuntimeClaimed: false,
    productionDurableOutboxStorageClaimed: false,
    adapterDispatchClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    connectorNativeRuntimeClaimed: false,
    enforcementClaimed: false,
  };
}

function highRiskRow() {
  return baseRow({
    surfaceRowId: 'social-parent-surface-provider-high-risk-service',
    sourceProviderHandoffRowId: 'social-provider-status-handoff-high-risk-service',
    sourcePreferenceHandoffRowId: 'social-preference-status-handoff-high-risk-service',
    sourceIntentRef: 'social-alert-report-intent-high-risk-service',
    notificationStatusRef: 'social-notification-status-high-risk-service',
    sourcePreferenceStatusRef: 'social-preference-status-high-risk-service',
    auditRefs: ['audit-ref-social-parent-surface-high-risk-service'],
    manualProofRequirements: ['manual-parent-surface-high-risk-runtime-proof-required'],
    parentSurfaceStatus: 'manual-action-required',
    historyVisibility: 'history-row-visible',
    preferenceVisibility: 'preference-setup-required',
  });
}

function manualRow() {
  return baseRow({
    surfaceRowId: 'social-parent-surface-manual-action-service',
    sourceProviderHandoffRowId: 'social-provider-status-handoff-manual-service',
    sourcePreferenceHandoffRowId: 'social-preference-status-handoff-manual-service',
    sourceIntentRef: 'social-alert-report-intent-manual-service',
    notificationStatusRef: 'social-notification-status-manual-service',
    sourcePreferenceStatusRef: 'social-preference-status-manual-service',
    auditRefs: ['audit-ref-social-parent-surface-manual-service'],
    manualProofRequirements: ['manual-parent-surface-runtime-proof-required'],
    parentSurfaceStatus: 'manual-action-required',
    historyVisibility: 'history-row-visible',
    preferenceVisibility: 'preference-setup-required',
  });
}

function unavailableRow() {
  return baseRow({
    surfaceRowId: 'social-parent-surface-unavailable-service',
    sourceProviderHandoffRowId: 'social-provider-status-handoff-unavailable-service',
    sourcePreferenceHandoffRowId: 'social-preference-status-handoff-unavailable-service',
    sourceIntentRef: 'social-alert-report-intent-unavailable-service',
    notificationStatusRef: 'social-notification-status-unavailable-service',
    sourcePreferenceStatusRef: 'social-preference-status-unavailable-service',
    auditRefs: ['audit-ref-social-parent-surface-unavailable-service'],
    manualProofRequirements: ['manual-parent-surface-unavailable-runtime-proof-required'],
    parentSurfaceStatus: 'unavailable-visible',
    historyVisibility: 'unavailable-row-visible',
    preferenceVisibility: 'preference-disabled-visible',
  });
}

function baseRow(overrides: {
  readonly surfaceRowId: string;
  readonly sourceProviderHandoffRowId: string;
  readonly sourcePreferenceHandoffRowId: string;
  readonly sourceIntentRef: string;
  readonly notificationStatusRef: string;
  readonly sourcePreferenceStatusRef: string;
  readonly auditRefs: readonly string[];
  readonly manualProofRequirements: readonly string[];
  readonly parentSurfaceStatus: 'manual-action-required' | 'unavailable-visible';
  readonly historyVisibility: 'history-row-visible' | 'unavailable-row-visible';
  readonly preferenceVisibility: 'preference-setup-required' | 'preference-disabled-visible';
}) {
  return {
    ...overrides,
    drillInRefs: [overrides.notificationStatusRef, overrides.sourcePreferenceStatusRef],
    minimalSurfacePayloadBoundary: 'parent-surface-status-ref-only',
    sensitiveDetailIncluded: false,
    parentNotificationUiRendered: false,
    parentNotificationPreferenceUiRendered: false,
    parentFrequencyControlUiRendered: false,
    parentNotificationHistoryUiRendered: false,
    providerDeliveryClaimed: false,
    providerReceiptClaimed: false,
    parentPreferenceMutationClaimed: false,
    childDeliveryClaimed: false,
    quietHoursTimerRuntimeClaimed: false,
    reportDeliveryExecutionClaimed: false,
    finalPolicyExecutionClaimed: false,
    adapterDispatchClaimed: false,
    enforcementClaimed: false,
  };
}
