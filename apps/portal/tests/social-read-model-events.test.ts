import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  type AgentEventEnvelope,
} from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { AgentProtocolDefaults } from '@ocentra-parent/schema-domain/agent-protocol-defaults';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
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
  parseAgentSocialAlertReportParentSurfaceReadModelEvent,
  parseAgentSocialAlertReportReadModelEvent,
  parseAgentSocialDashboardReadModelEvent,
  parseAgentSocialParentNotificationDeliveryReadModelEvent,
} from '../src/social-read-model-events';

const Timestamp = '2026-06-21T11:20:00.000Z';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

describe('portal social read-model event parsers', () => {
  socialReadModelSnapshotParseTests();
  socialReadModelRejectionTests();
});

function socialReadModelSnapshotParseTests(): void {
  it('parses the social alert report payload from the canonical schema-domain snapshot', () => {
    const snapshot = socialAlertReportSnapshot();
    const parsed = parseAgentSocialAlertReportReadModelEvent(
      readModelEvent(
        AgentEvent.BrowserSocialAlertReportReadModelReported,
        AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel,
        snapshot
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: snapshot,
    });
  });

  it('parses the social alert parent-surface payload from the canonical schema-domain snapshot', () => {
    const snapshot = socialAlertReportParentSurfaceSnapshot();
    const parsed = parseAgentSocialAlertReportParentSurfaceReadModelEvent(
      readModelEvent(
        AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
        AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel,
        snapshot
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: snapshot,
    });
  });

  it('parses the social parent notification readiness payload from the canonical schema-domain snapshot', () => {
    const snapshot = socialParentNotificationDeliverySnapshot();
    const parsed = parseAgentSocialParentNotificationDeliveryReadModelEvent(
      readModelEvent(
        AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
        AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel,
        snapshot
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: snapshot,
    });
  });

  it('parses the social dashboard payload from the canonical schema-domain snapshot', () => {
    const snapshot = socialDashboardSnapshot();
    const parsed = parseAgentSocialDashboardReadModelEvent(
      readModelEvent(
        AgentEvent.BrowserSocialDashboardReadModelReported,
        AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel,
        snapshot
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: snapshot,
    });
  });
}

function socialReadModelRejectionTests(): void {
  socialAlertReportRejectionTests();
  socialSurfaceRejectionTests();
}

function socialAlertReportRejectionTests(): void {
  it('rejects wrong events, missing fields, invalid json, and dishonest alert-report payloads', () => {
    const event = readModelEvent(
      AgentEvent.BrowserSocialAlertReportReadModelReported,
      AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel,
      socialAlertReportSnapshot()
    );

    expect(
      parseAgentSocialAlertReportReadModelEvent({
        ...event,
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(
      parseAgentSocialAlertReportReadModelEvent({
        ...event,
        payload: {},
      })
    ).toEqual({
      ok: false,
      reason: 'missing-json-field',
    });
    expect(
      parseAgentSocialAlertReportReadModelEvent({
        ...event,
        payload: {
          [AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel]: '{',
        },
      })
    ).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentSocialAlertReportReadModelEvent(
        readModelEvent(
          AgentEvent.BrowserSocialAlertReportReadModelReported,
          AgentProtocolDefaults.Field.BrowserSocialAlertReportReadModel,
          {
            ...socialAlertReportSnapshot(),
            intents: [{ ...socialAlertReportIntent(), providerDeliveryAttempted: true }],
          }
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
}

function socialSurfaceRejectionTests(): void {
  it('rejects dishonest parent-surface, readiness, and dashboard payload claims', () => {
    expect(
      parseAgentSocialAlertReportParentSurfaceReadModelEvent(
        readModelEvent(
          AgentEvent.BrowserSocialAlertReportParentSurfaceReadModelReported,
          AgentProtocolDefaults.Field.BrowserSocialAlertReportParentSurfaceReadModel,
          {
            ...socialAlertReportParentSurfaceSnapshot(),
            rows: [{ ...socialParentSurfaceManualRow(), parentNotificationUiRendered: true }],
          }
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentSocialParentNotificationDeliveryReadModelEvent(
        readModelEvent(
          AgentEvent.BrowserSocialParentNotificationDeliveryReadModelReported,
          AgentProtocolDefaults.Field.BrowserSocialParentNotificationDeliveryReadModel,
          {
            ...socialParentNotificationDeliverySnapshot(),
            rows: [{ ...socialParentNotificationReadyRow(), parentNotificationUiDelivered: true }],
          }
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentSocialDashboardReadModelEvent(
        readModelEvent(
          AgentEvent.BrowserSocialDashboardReadModelReported,
          AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel,
          {
            ...socialDashboardSnapshot(),
            panels: socialDashboardSnapshot().panels.map((panel) =>
              panel.panelKind === 'feed-video-gates' ? { ...panel, policyDecisionClaimed: true } : panel
            ),
          }
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
}

function readModelEvent(event: AgentEventEnvelope['event'], payloadField: string, value: unknown): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: `event-${event}`,
    correlationId: `command-${event}`,
    sentAt: Timestamp,
    source: Source,
    target: Target,
    event,
    severity: 'info',
    payload: {
      [payloadField]: JSON.stringify(value),
    },
    snapshot: null,
  });
}

function socialAlertReportSnapshot() {
  return {
    schemaVersion: 'social-alert-report-read-model',
    familyId: 'family-social-alert-report-service',
    childProfileId: 'child-social-alert-report-service',
    generatedAt: Timestamp,
    intents: [socialAlertReportIntent()],
    providerStatusRows: [socialAlertReportProviderStatusRow()],
    claimBoundaries: {
      providerDelivery: 'not-claimed',
      reportDelivery: 'not-claimed',
      parentNotificationUi: 'not-claimed',
      finalPolicyDecision: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function socialAlertReportProviderStatusRow() {
  return {
    statusEntryId: 'social-provider-status-social-alert-report-high-risk-service',
    sourceIntentRef: 'social-alert-report-high-risk-service',
    sourcePreflightStatus: 'provider-adapter-required',
    providerStatus: 'manual-required',
    statusProofState: 'manual-action-required',
    deliveryClaimState: 'not-observed',
    providerAttemptRef: 'social-provider-attempt-not-started-social-alert-report-high-risk-service',
    readinessRefs: [
      'provider-adapter-required-social-alert-report-high-risk-service',
      'provider-credentials-required-social-alert-report-high-risk-service',
      'provider-smoke-proof-required-social-alert-report-high-risk-service',
    ],
    providerReceiptRefs: [],
    manualProofRequirements: [
      'provider-adapter-required-social-alert-report-high-risk-service',
      'provider-credentials-required-social-alert-report-high-risk-service',
      'provider-smoke-proof-required-social-alert-report-high-risk-service',
    ],
    providerDeliveryImplemented: false,
    providerDeliveryObserved: false,
    deliveredNotificationClaimed: false,
    sensitiveProviderPayloadClaimed: false,
    providerStoresChildEvidenceClaimed: false,
    lastCheckedAt: Timestamp,
  };
}

function socialAlertReportIntent() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    alertReportIntentId: 'social-alert-report-high-risk-service',
    intentKind: SocialAlertReportIntentKind.HighRiskSignal,
    intentStatus: SocialAlertReportIntentStatus.LocalOutboxEligible,
    priority: 'urgent',
    severity: 'critical',
    device: {
      deviceId: 'device-social-alert-report',
      childProfileId: 'child-social-alert-report-service',
      label: 'Study Phone',
      platform: 'android',
    },
    notificationReasonCode: SocialAlertReportReasonCode.HighRiskSignal,
    providerChannelPreference: 'in-app',
    parentTitleToken: SocialAlertReportParentCopyToken.HighRiskTitle,
    parentBodyToken: SocialAlertReportParentCopyToken.HighRiskBody,
    parentActionToken: SocialAlertReportParentCopyToken.OpenParentReviewAction,
    dashboardPanelRefs: ['panel-feed-video-gates'],
    explanationSnapshotRef: 'social-explanation-snapshot-alert-report',
    explanationEventRefs: ['social-explanation-event-feed-video-gate'],
    evidenceReferences: [
      {
        evidenceReferenceId: 'evidence-social-route-gate',
        kind: ParentEvidenceReferenceKind.PolicyDecision,
        observedAt: Timestamp,
      },
    ],
    policyRefs: ['policy-ref-social-high-risk'],
    auditRefs: ['audit-ref-social-alert-report'],
    parentReportRef: null,
    parentActionRef: {
      actionReferenceId: 'parent-action-social-review',
      actor: {
        actorId: 'parent-local-account',
        role: ParentActorRole.Parent,
      },
      policyVersion: 'policy-social-alert-report-v1',
      createdAt: Timestamp,
    },
    localOutboxRecordRef: 'local-outbox-social-alert-report',
    providerAttemptRefs: [],
    providerReceiptRefs: [],
    manualProofRequirements: [],
    minimalPayloadFields: Object.values(SocialAlertReportPayloadField),
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
  };
}

function socialAlertReportParentSurfaceSnapshot() {
  return {
    schemaVersion: 'social-alert-report-parent-surface-read-model',
    intentId: 'social-alert-report-parent-surface-service',
    generatedAt: Timestamp,
    sourceProviderStatusHandoffId: 'social-provider-status-handoff-service',
    sourcePreferenceStatusHandoffId: 'social-preference-status-handoff-service',
    rows: [socialParentSurfaceHighRiskRow(), socialParentSurfaceManualRow(), socialParentSurfaceUnavailableRow()],
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

function socialParentSurfaceHighRiskRow() {
  return socialParentSurfaceRow({
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

function socialParentSurfaceManualRow() {
  return socialParentSurfaceRow({
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

function socialParentSurfaceUnavailableRow() {
  return socialParentSurfaceRow({
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

function socialParentSurfaceRow(overrides: {
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

function socialParentNotificationDeliverySnapshot() {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    readinessId: 'social-parent-notification-delivery-readiness-service',
    generatedAt: Timestamp,
    sourceReportWriterProofRef: 'social-report-writer-delivery-proof-service',
    rows: [
      socialParentNotificationReadyRow(),
      socialParentNotificationManualRow(),
      socialParentNotificationUnavailableRow(),
    ],
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
    parentLocalDeliveryResultCount: 1,
    parentNotificationUiDeliveryClaimed: false,
    externalRuntimeReportDeliveryClaimed: false,
    finalPolicyExecutionClaimed: false,
    enforcementClaimed: false,
  };
}

function socialParentNotificationReadyRow() {
  return socialParentNotificationRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-ready-high-risk-service',
    parentVisibleReportStatusRef: 'social-parent-visible-report-status-high-risk-service',
    parentLocalDeliveryResultRef: 'social-parent-local-delivery-result-high-risk-service',
    parentReportRef: 'social-parent-report-high-risk-service',
    reportArtifactRef: 'social-report-artifact-high-risk-service',
    reportReceiptRef: 'social-report-receipt-high-risk-service',
    manualProofRequirements: [],
    notificationDeliveryReadinessState: 'parent-report-status-ready',
    reportDeliveryExecutionState: 'parent-owned-report-ready',
    parentLocalDeliveryResultRecorded: true,
    parentOwnedReportArtifactWritten: true,
    parentOwnedReportReceiptRecorded: true,
  });
}

function socialParentNotificationManualRow() {
  return socialParentNotificationRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-manual-required-service',
    parentVisibleReportStatusRef: 'social-parent-visible-report-status-manual-required-service',
    parentLocalDeliveryResultRef: null,
    parentReportRef: null,
    reportArtifactRef: null,
    reportReceiptRef: null,
    manualProofRequirements: ['manual-parent-notification-ui-runtime-proof-required'],
    notificationDeliveryReadinessState: 'manual-required',
    reportDeliveryExecutionState: 'manual-required',
    parentLocalDeliveryResultRecorded: false,
    parentOwnedReportArtifactWritten: false,
    parentOwnedReportReceiptRecorded: false,
  });
}

function socialParentNotificationUnavailableRow() {
  return socialParentNotificationRow({
    notificationDeliveryReadinessRowId: 'social-parent-notification-unavailable-service',
    parentVisibleReportStatusRef: null,
    parentLocalDeliveryResultRef: null,
    parentReportRef: null,
    reportArtifactRef: null,
    reportReceiptRef: null,
    manualProofRequirements: ['external-report-delivery-runtime-unavailable'],
    notificationDeliveryReadinessState: 'unavailable',
    reportDeliveryExecutionState: 'unavailable',
    parentLocalDeliveryResultRecorded: false,
    parentOwnedReportArtifactWritten: false,
    parentOwnedReportReceiptRecorded: false,
  });
}

function socialParentNotificationRow(overrides: {
  readonly notificationDeliveryReadinessRowId: string;
  readonly parentVisibleReportStatusRef: string | null;
  readonly parentLocalDeliveryResultRef: string | null;
  readonly parentReportRef: string | null;
  readonly reportArtifactRef: string | null;
  readonly reportReceiptRef: string | null;
  readonly manualProofRequirements: readonly string[];
  readonly notificationDeliveryReadinessState: 'parent-report-status-ready' | 'manual-required' | 'unavailable';
  readonly reportDeliveryExecutionState: 'parent-owned-report-ready' | 'manual-required' | 'unavailable';
  readonly parentLocalDeliveryResultRecorded: boolean;
  readonly parentOwnedReportArtifactWritten: boolean;
  readonly parentOwnedReportReceiptRecorded: boolean;
}) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
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

function socialDashboardSnapshot() {
  return {
    schemaVersion: 'social-dashboard-ux-contract',
    familyId: 'family-social-dashboard',
    childProfileId: 'child-social-dashboard',
    generatedAt: Timestamp,
    panels: [
      socialDashboardPanel('account-approval-queue', 'ready-for-review', 'open-parent-approval', [
        'parent-review-needed',
      ]),
      socialDashboardPanel('feed-video-gates', 'ready-for-review', 'review-feed-gate', ['feed-video-gate-candidate']),
      socialDashboardPanel('native-app-capability', 'manual-required', 'review-native-capability', [
        'native-app-manual-required',
      ]),
      socialDashboardPanel('connector-boundaries', 'manual-required', 'review-connector-boundary', [
        'connector-boundary-manual-required',
      ]),
      socialDashboardPanel('decision-memory', 'contract-only', 'review-memory-entry', ['memory-contract-only']),
      socialDashboardPanel('settings-custody', 'manual-required', 'review-settings-custody', [
        'settings-custody-runtime-gap',
      ]),
      socialDashboardPanel('manual-required-gaps', 'manual-required', 'manual-review', ['platform-proof-gap']),
    ],
    claimBoundaries: {
      renderedPortalUi: 'not-claimed',
      notificationDelivery: 'not-claimed',
      runtimeDataFetch: 'not-claimed',
      policyDecision: 'not-claimed',
      nativeAppControl: 'not-claimed',
      connectorAuthorization: 'not-claimed',
      enforcement: 'not-claimed',
    },
  };
}

function socialDashboardPanel(
  panelKind:
    | 'account-approval-queue'
    | 'feed-video-gates'
    | 'native-app-capability'
    | 'connector-boundaries'
    | 'decision-memory'
    | 'settings-custody'
    | 'manual-required-gaps',
  status: 'ready-for-review' | 'manual-required' | 'contract-only',
  primaryAction:
    | 'open-parent-approval'
    | 'review-feed-gate'
    | 'review-native-capability'
    | 'review-connector-boundary'
    | 'review-memory-entry'
    | 'review-settings-custody'
    | 'manual-review',
  reasons: readonly (
    | 'parent-review-needed'
    | 'feed-video-gate-candidate'
    | 'native-app-manual-required'
    | 'connector-boundary-manual-required'
    | 'memory-contract-only'
    | 'settings-custody-runtime-gap'
    | 'platform-proof-gap'
  )[]
) {
  return {
    panelId: `social-dashboard-${panelKind}`,
    panelKind,
    status,
    primaryAction,
    severity: status === 'manual-required' ? 'warning' : 'info',
    sortOrder: socialDashboardPanelSortOrder(panelKind),
    sourceEvidenceRefs: [`parent-evidence-${panelKind}`],
    reasons,
    renderedUiClaimed: false,
    notificationClaimed: false,
    runtimeDataFetchClaimed: false,
    policyDecisionClaimed: false,
    nativeAppControlClaimed: false,
    connectorAuthorizationClaimed: false,
    enforcementClaimed: false,
  };
}

function socialDashboardPanelSortOrder(
  panelKind:
    | 'account-approval-queue'
    | 'feed-video-gates'
    | 'native-app-capability'
    | 'connector-boundaries'
    | 'decision-memory'
    | 'settings-custody'
    | 'manual-required-gaps'
): number {
  return {
    'account-approval-queue': 0,
    'feed-video-gates': 1,
    'native-app-capability': 2,
    'connector-boundaries': 3,
    'decision-memory': 4,
    'settings-custody': 5,
    'manual-required-gaps': 6,
  }[panelKind];
}
