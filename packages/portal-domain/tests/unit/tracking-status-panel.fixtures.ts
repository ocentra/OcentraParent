import {
  GeneratedPortalActivityEventKind,
  GeneratedPortalActivityQuerySchemaVersion,
  GeneratedPortalAgentEvent,
  GeneratedPortalAgentProtocolRuntime,
  GeneratedPortalTrackingContracts,
} from '../../src/generated-portal-contracts';
import { TrackingStatusProofArtifacts } from '../../src/tracking-status-proof-artifacts';
import type { TrackingStatusLiveProjectionInput } from '../../src/tracking-status-panel';
import type { trackingRetentionSettingsHostedUiProof } from '../../src/tracking-retention-settings-hosted-ui-proof';

export const ExpectedTrackingStateTitles = [
  'Tracking off',
  'Permission required',
  'Stale last known',
  'Offline last known',
  'Low accuracy',
  'Nearby place ambiguous',
  'Policy alert',
  'Parent acknowledged',
  'Exception active',
  'Child check-in',
  'Temporary live',
  'Missing device',
  'Retention deleted',
] as const;

export const ExpectedTrackingProofArtifacts = [
  TrackingStatusProofArtifacts.ContractBoundary,
  TrackingStatusProofArtifacts.PermissionCapability,
  TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  TrackingStatusProofArtifacts.DeviceStatus,
  TrackingStatusProofArtifacts.RuntimeLocationEvidence,
  TrackingStatusProofArtifacts.NearbyPlace,
  TrackingStatusProofArtifacts.AlertSeverity,
  TrackingStatusProofArtifacts.ParentAcknowledgement,
  TrackingStatusProofArtifacts.ParentAcknowledgement,
  TrackingStatusProofArtifacts.ChildCheckIn,
  TrackingStatusProofArtifacts.TemporaryLiveMode,
  TrackingStatusProofArtifacts.MissingDeviceMode,
  TrackingStatusProofArtifacts.RetentionDelete,
] as const;

export const ExpectedRetentionDeletedRow = {
  title: 'Retention deleted',
  state: 'Retention deleted',
  proofTier: 'P1 fixture proof',
  evidence: 'UI fixture proof',
  proofArtifact: TrackingStatusProofArtifacts.RetentionDelete,
  missingProof: 'Manual proof required',
  productClaim: 'No product claim',
  historyVisibility: 'Deleted history hidden',
  deletedEvidence: 'Deleted evidence not rendered',
} as const;

export const TrackingRetentionSettingsWriteDefaults = GeneratedPortalTrackingContracts.RetentionSettingsWrite.Defaults;

export const TrackingReadModel = {
  schemaVersion: GeneratedPortalActivityQuerySchemaVersion,
  generatedAt: '2026-06-03T07:25:00Z',
  custodyLabel: 'child-device-query-store',
  limit: 20,
  returned: 2,
  activeRows: 1,
  tombstoneRows: 1,
  capabilityStatus: 'recent',
  latestEventId: 'tracking-retention-delete-1',
  latestObservedAt: '2026-06-03T07:26:00Z',
  latestActiveEventId: 'tracking-event-1',
  latestActiveObservedAt: '2026-06-03T07:24:00Z',
  latestTombstoneEventId: 'tracking-retention-delete-1',
  latestTombstoneObservedAt: '2026-06-03T07:26:00Z',
  activeKindCounts: [{ value: 'tracking.expected-place.evaluated', count: 1 }],
  activeDeviceCounts: [{ value: 'child-device-1', count: 1 }],
  activeCapabilityStatusCounts: [{ value: 'recent', count: 1 }],
  deletedEvidenceReferenceIds: ['location-evidence-1'],
  rows: [
    {
      schemaVersion: GeneratedPortalActivityQuerySchemaVersion,
      eventId: 'tracking-event-1',
      observedAt: '2026-06-03T07:24:00Z',
      deviceId: 'child-device-1',
      platform: 'android',
      observer: 'tracking-engine',
      kind: 'tracking.expected-place.evaluated',
      subjectKind: 'tracking-rule',
      subjectId: 'expected-place-school',
      subjectDisplayName: 'School',
      capabilityStatus: 'recent',
      queryVisibility: 'active',
      deletedAt: null,
      evidenceReferenceIds: ['tracking-evidence-1'],
      deletedEvidenceReferenceIds: [],
      evidence: [],
    },
    {
      schemaVersion: GeneratedPortalActivityQuerySchemaVersion,
      eventId: 'tracking-retention-delete-1',
      observedAt: '2026-06-03T07:26:00Z',
      deviceId: 'child-device-1',
      platform: 'android',
      observer: 'tracking-retention',
      kind: GeneratedPortalActivityEventKind.TrackingRetentionDeleted,
      subjectKind: 'location-evidence',
      subjectId: 'location-evidence-1',
      subjectDisplayName: null,
      capabilityStatus: 'recent',
      queryVisibility: 'tombstone',
      deletedAt: '2026-06-03T07:26:00Z',
      evidenceReferenceIds: ['location-evidence-1'],
      deletedEvidenceReferenceIds: ['location-evidence-1'],
      evidence: [],
    },
  ],
} as const;

export const ExpectedTrackingLiveSummary = {
  title: 'Service read model',
  loadState: 'info',
  proofTier: 'P2 service proof',
  rowsReturned: '2',
  lastObserved: '2026-06-03T07:24:00Z',
  eventId: 'tracking-event-1',
  capability: 'recent',
  custody: 'child-device-query-store',
  evidenceReferences: 'tracking-evidence-1',
  parserReason: null,
  productClaim: 'No product claim',
  citations: [
    {
      title: 'School',
      eventId: 'tracking-event-1',
      observedAt: '2026-06-03T07:24:00Z',
      device: 'child-device-1',
      platform: 'android',
      observer: 'tracking-engine',
      activityKind: 'tracking.expected-place.evaluated',
      subject: 'tracking-rule | expected-place-school',
      status: 'active | recent',
      evidenceReferences: 'tracking-evidence-1',
      deletedEvidence: 'Not reported',
      productClaim: 'No product claim',
    },
    {
      title: GeneratedPortalActivityEventKind.TrackingRetentionDeleted,
      eventId: 'tracking-retention-delete-1',
      observedAt: '2026-06-03T07:26:00Z',
      device: 'child-device-1',
      platform: 'android',
      observer: 'tracking-retention',
      activityKind: GeneratedPortalActivityEventKind.TrackingRetentionDeleted,
      subject: 'location-evidence | location-evidence-1',
      status: 'tombstone | recent',
      evidenceReferences: 'location-evidence-1',
      deletedEvidence: 'location-evidence-1',
      productClaim: 'No product claim',
    },
  ],
} as const;

export const ExpectedTrackingServiceDataCoverage = {
  title: 'Service data coverage',
  loadState: 'info',
  proofTier: 'P2 service proof',
  rowsReturned: '2',
  rowVisibility: '1 | 1',
  lastObserved: '2026-06-03T07:26:00Z',
  eventId: 'tracking-retention-delete-1',
  deviceCounts: 'child-device-1 (1)',
  capability: 'recent (1)',
  custody: 'child-device-query-store',
  activityKinds: 'tracking.expected-place.evaluated (1)',
  evidenceReferences: 'tracking-evidence-1',
  deletedEvidence: 'location-evidence-1',
  productClaim: 'No product claim',
} as const;

export const ExpectedLegacyTrackingServiceDataCoverage = {
  ...ExpectedTrackingServiceDataCoverage,
  deviceCounts: 'child-device-1',
  capability: 'recent',
  activityKinds: 'tracking.expected-place.evaluated',
} as const;

export const ExpectedUnsupportedManualRows = [
  {
    title: 'Android background location manual required',
    supportState: 'manual-required',
    renderedState: 'manual-required',
  },
  {
    title: 'Android geofence transition manual required',
    supportState: 'manual-required',
    renderedState: 'manual-required',
  },
  {
    title: 'iOS background location manual required',
    supportState: 'manual-required',
    renderedState: 'manual-required',
  },
  {
    title: 'iOS geofence transition manual required',
    supportState: 'manual-required',
    renderedState: 'manual-required',
  },
  {
    title: 'Windows desktop OS location manual required',
    supportState: 'manual-required',
    renderedState: 'manual-required',
  },
  {
    title: 'Web child agent location unavailable',
    supportState: 'platform-unsupported',
    renderedState: 'unavailable',
  },
  {
    title: 'Authority hard-control proof required',
    supportState: 'real-device-required',
    renderedState: 'authority-required',
  },
] as const;

export const ExpectedFamilyDashboardHostedRollupRows = [
  {
    title: 'Family active summary',
    status: 'rollup-ready',
    visibleChildren: '2',
    attentionItems: '1',
    retainedAuditItems: '0',
    evidence: 'tracking-family-dashboard-evidence-active-summary',
  },
  {
    title: 'Child attention summary',
    status: 'rollup-ready',
    visibleChildren: '1',
    attentionItems: '2',
    retainedAuditItems: '0',
    evidence: 'tracking-family-dashboard-evidence-child-attention',
  },
  {
    title: 'Retention audit summary',
    status: 'rollup-ready',
    visibleChildren: '0',
    attentionItems: '0',
    retainedAuditItems: '2',
    evidence: 'tracking-family-dashboard-evidence-retention-audit',
  },
] as const;

export const ExpectedReportExportHostedUiRows = [
  {
    title: 'Redacted report packet',
    status: 'report-export-read-model-ready',
    exportedRows: '6',
    redactedEvidenceRefs: '6',
    custody: 'parent-owned-redacted-report',
    evidence: 'tracking-report-export-evidence-redacted-report',
  },
  {
    title: 'Retention audit export packet',
    status: 'report-export-read-model-ready',
    exportedRows: '5',
    redactedEvidenceRefs: '5',
    custody: 'parent-owned-local-export',
    evidence: 'tracking-report-export-evidence-retention-audit',
  },
  {
    title: 'Family dashboard summary packet',
    status: 'report-export-read-model-ready',
    exportedRows: '3',
    redactedEvidenceRefs: '3',
    custody: 'parent-owned-redacted-report',
    evidence: 'tracking-report-export-evidence-family-dashboard',
  },
  {
    title: 'Policy drill-in export packet',
    status: 'report-export-read-model-ready',
    exportedRows: '2',
    redactedEvidenceRefs: '2',
    custody: 'parent-owned-redacted-report',
    evidence: 'tracking-report-export-evidence-policy-drill-in',
  },
] as const;

export const ExpectedReportExportHostedUiProof = {
  title: 'Report export read-model UI',
  body: 'Hosted route renders redacted report/export packet rows from existing read-model proof refs without exposing raw location payloads or claiming product-ready export.',
  proofTier: 'P2 service proof',
  rowsReturned: '4',
  proofArtifact: TrackingStatusProofArtifacts.ReportExportReadModel,
  boundary:
    'Hosted report/export packet rendering only; raw location payload export, service mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.',
  missingProof: 'Manual proof required',
  productClaim: 'No product claim',
  rawLocationPayloadClaimedRows: '0',
  serviceMutationClaimedRows: '0',
  platformRuntimeClaimedRows: '0',
  childDeviceDeliveryClaimedRows: '0',
  providerDeliveryClaimedRows: '0',
  notificationReceiptClaimedRows: '0',
  physicalDeviceClaimedRows: '0',
  authorityClaimedRows: '0',
  productClaimReadyRows: '0',
  rows: ExpectedReportExportHostedUiRows,
} as const;

export const ExpectedReportPolicyConsumerHostedUiRows = [
  {
    title: 'Parent report summary consumer',
    status: 'consumer-ready',
    storedJournalRef: 'tracking-journal-row-report-summary',
    storedReadModelRef: 'tracking-read-model-row-report-summary',
    evidence: 'tracking-report-policy-evidence-summary',
    reportSurface: 'parent-report-location-summary-row',
  },
  {
    title: 'Policy evidence drill-in consumer',
    status: 'consumer-ready',
    storedJournalRef: 'tracking-journal-row-policy-drill-in',
    storedReadModelRef: 'tracking-read-model-row-policy-drill-in',
    evidence: 'tracking-report-policy-evidence-decision',
    reportSurface: 'parent-policy-evidence-drill-in-row',
  },
  {
    title: 'Retention audit export consumer',
    status: 'consumer-ready',
    storedJournalRef: 'tracking-journal-row-retention-export',
    storedReadModelRef: 'tracking-read-model-row-retention-export',
    evidence: 'tracking-report-policy-evidence-retention',
    reportSurface: 'parent-retention-audit-export-row',
  },
] as const;

export const ExpectedReportPolicyConsumerHostedUiProof = {
  title: 'Report policy consumer UI',
  body: 'Hosted route renders parent report summary, policy drill-in, and retention audit consumer rows from stored journal/read-model refs without claiming product-ready report or policy execution.',
  proofTier: 'P2 service proof',
  rowsReturned: '3',
  proofArtifact: TrackingStatusProofArtifacts.ReportPolicyConsumer,
  boundary:
    'Hosted report/policy consumer rendering only; AI execution, product policy mutation, platform runtime, child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, production, and product readiness remain unclaimed.',
  missingProof: 'Manual proof required',
  productClaim: 'No product claim',
  aiExecutionClaimedRows: '0',
  policyMutationClaimedRows: '0',
  platformRuntimeClaimedRows: '0',
  childDeviceDeliveryClaimedRows: '0',
  providerDeliveryClaimedRows: '0',
  notificationReceiptClaimedRows: '0',
  physicalDeviceClaimedRows: '0',
  authorityClaimedRows: '0',
  productClaimReadyRows: '0',
  rows: ExpectedReportPolicyConsumerHostedUiRows,
} as const;

export const ExpectedNotificationParentSurfaceRows = [
  {
    title: 'Notification history ready',
    status: 'history-intent-ready',
    policyDecisionRef: 'tracking-decision-home-arrival',
    evidenceRefs: 'location-evidence-geofence-entry',
    providerAttemptRef: 'tracking-provider-attempt-home-arrival',
    receiptRequirementRefs: 'receipt-ingestion-required-home-arrival',
    preferenceRequirementRefs: 'parent-notification-preference-required-home-arrival',
    manualProofRequirements: 'provider-delivery-runtime-required | receipt-webhook-runtime-required',
    redactedSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-home-arrival',
  },
  {
    title: 'Manual notification action required',
    status: 'manual-action-required',
    policyDecisionRef: 'tracking-decision-left-expected-place',
    evidenceRefs: 'location-evidence-geofence-entry',
    providerAttemptRef: 'tracking-provider-attempt-left-school',
    receiptRequirementRefs: 'manual-receipt-required-left-school',
    preferenceRequirementRefs: 'quiet-hours-requirement-left-school',
    manualProofRequirements: 'manual-provider-review-required | quiet-hours-runtime-required',
    redactedSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-left-expected-place',
  },
  {
    title: 'Notification provider unavailable',
    status: 'provider-unavailable',
    policyDecisionRef: 'tracking-decision-provider-unavailable',
    evidenceRefs: 'location-evidence-geofence-entry',
    providerAttemptRef: 'tracking-provider-attempt-unavailable',
    receiptRequirementRefs: 'provider-receipt-unavailable',
    preferenceRequirementRefs: 'source-unavailable-preference-required',
    manualProofRequirements: 'provider-adapter-unavailable | manual-parent-history-review-required',
    redactedSummaryRef: 'tracking-notification-redacted-summary-tracking-alert-provider-unavailable',
  },
] as const;

export const ExpectedParentActionReadinessRows = [
  {
    title: 'Expected-place parent alert ready',
    status: 'alert-policy-ready',
    primaryActionRef: 'notify-parent',
    policyDecisionRef: 'expected-place-decision-school',
    evidenceRefs: 'expected-place-evidence-school-arrival',
    uiSurfaceRef: 'tracking-expected-place-ui-readiness-expected-place-decision-school',
    manualProofRequirements: 'hosted-read-only-parent-action-proof',
  },
  {
    title: 'Expected-place child check-in ready',
    status: 'check-in-policy-ready',
    primaryActionRef: 'ask-child-check-in',
    policyDecisionRef: 'expected-place-decision-late-bus',
    evidenceRefs: 'expected-place-evidence-late-bus',
    uiSurfaceRef: 'tracking-expected-place-ui-readiness-expected-place-decision-late-bus',
    manualProofRequirements: 'hosted-read-only-parent-action-proof',
  },
  {
    title: 'Expected-place suppressed no action',
    status: 'suppressed-no-action',
    primaryActionRef: 'no-action',
    policyDecisionRef: 'expected-place-decision-holiday',
    evidenceRefs: 'expected-place-evidence-holiday',
    uiSurfaceRef: 'tracking-expected-place-ui-readiness-expected-place-decision-holiday',
    manualProofRequirements: 'hosted-read-only-parent-action-proof',
  },
  {
    title: 'Expected-place manual review required',
    status: 'manual-required',
    primaryActionRef: 'manual-review',
    policyDecisionRef: 'expected-place-decision-low-accuracy',
    evidenceRefs: 'expected-place-evidence-low-accuracy',
    uiSurfaceRef: 'tracking-expected-place-ui-readiness-expected-place-decision-low-accuracy',
    manualProofRequirements: 'tracking-expected-place-manual-proof-expected-place-decision-low-accuracy',
  },
  {
    title: 'Parent acknowledgement recorded',
    status: 'acknowledgement-recorded',
    primaryActionRef: 'acknowledge-safe',
    policyDecisionRef: 'tracking-decision-safe',
    evidenceRefs: 'tracking-parent-action-evidence-1',
    uiSurfaceRef: 'tracking-parent-action-surface-tracking-alert-safe',
    manualProofRequirements: 'live-service-mutation-proof-required | rendered-portal-acknowledgement-ui-proof-required',
  },
  {
    title: 'Expected exception active',
    status: 'exception-active',
    primaryActionRef: 'mark-expected',
    policyDecisionRef: 'tracking-decision-expected',
    evidenceRefs: 'tracking-parent-action-evidence-2',
    uiSurfaceRef: 'tracking-parent-action-surface-tracking-alert-expected',
    manualProofRequirements: 'live-service-mutation-proof-required | rendered-portal-acknowledgement-ui-proof-required',
  },
  {
    title: 'False alarm recorded',
    status: 'false-alarm-recorded',
    primaryActionRef: 'mark-false-alarm',
    policyDecisionRef: 'tracking-decision-false-alarm',
    evidenceRefs: 'tracking-parent-action-evidence-3',
    uiSurfaceRef: 'tracking-parent-action-surface-tracking-alert-false-alarm',
    manualProofRequirements: 'live-service-mutation-proof-required | rendered-portal-acknowledgement-ui-proof-required',
  },
  {
    title: 'Child check-in action ready',
    status: 'child-check-in-request-ready',
    primaryActionRef: 'request-child-check-in',
    policyDecisionRef: 'tracking-decision-check-in',
    evidenceRefs: 'tracking-parent-action-evidence-4',
    uiSurfaceRef: 'tracking-parent-action-surface-tracking-alert-check-in',
    manualProofRequirements: 'child-device-runtime-proof-required | rendered-portal-acknowledgement-ui-proof-required',
  },
  {
    title: 'Critical escalation review ready',
    status: 'escalation-review-ready',
    primaryActionRef: 'escalate-manual-review',
    policyDecisionRef: 'tracking-decision-critical-review',
    evidenceRefs: 'tracking-parent-action-evidence-5',
    uiSurfaceRef: 'tracking-parent-action-surface-tracking-alert-critical-review',
    manualProofRequirements: 'critical-escalation-runtime-proof-required | second-guardian-provider-proof-required',
  },
] as const;

export const ExpectedMissingDeviceRows = [
  {
    title: 'Last-known only state',
    state: 'last-known-only',
    primaryBadge: 'last-known',
    contactState: 'contact-state-offline',
    lastKnownEvidenceRef: 'location-evidence-last-known-stale',
    deviceStatusEvidenceRef: 'device-status-offline-last-known',
    actionRefs: 'review-last-known | ask-child-check-in | call-child | mark-found',
    manualProofRequirements: 'hosted-read-only-missing-device-proof',
  },
  {
    title: 'Powered-off offline state',
    state: 'offline',
    primaryBadge: 'offline',
    contactState: 'contact-state-powered-off',
    lastKnownEvidenceRef: 'location-evidence-last-known-powered-off',
    deviceStatusEvidenceRef: 'device-status-powered-off',
    actionRefs: 'review-last-known | ask-child-check-in | call-child | mark-found',
    manualProofRequirements: 'powered-off-current-location-proof-forbidden | hosted-read-only-missing-device-proof',
  },
  {
    title: 'Contact requested state',
    state: 'contact-requested',
    primaryBadge: 'contact-requested',
    contactState: 'contact-state-online',
    lastKnownEvidenceRef: 'location-evidence-last-known-contact-requested',
    deviceStatusEvidenceRef: 'device-status-contact-action-queued',
    actionRefs: 'review-last-known | call-child | mark-found',
    manualProofRequirements: 'hosted-read-only-missing-device-proof',
  },
  {
    title: 'Manual platform proof state',
    state: 'manual-required',
    primaryBadge: 'manual-required',
    contactState: 'contact-state-unknown',
    lastKnownEvidenceRef: 'location-evidence-last-known-manual-required',
    deviceStatusEvidenceRef: 'device-status-platform-proof-required',
    actionRefs: 'review-last-known | manual-platform-proof',
    manualProofRequirements: 'os-lost-mode-api-proof-required | physical-device-proof-required',
  },
] as const;

export const TrackingRetentionSettingsWriteResult = {
  schemaVersion: GeneratedPortalAgentProtocolRuntime.SchemaVersion,
  commandId: TrackingRetentionSettingsWriteDefaults.CommandId,
  settingsKind: TrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow,
  writeState: TrackingRetentionSettingsWriteDefaults.WriteStateAccepted,
  acceptedAt: '2026-06-06T19:40:00.000Z',
  sourceWriterIntentRefs: [TrackingRetentionSettingsWriteDefaults.WriterIntentRef],
  sourceReadModelProofRefs: [TrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[0]],
  sourceMutationProofRefs: [TrackingRetentionSettingsWriteDefaults.MutationProofRef],
  appliedRetentionWindowHours: 168,
  appliedDeleteAfterAlertResolutionState: 'retain-after-alert-resolved',
  parentExportState: 'not-prepared',
  remoteSyncState: 'disabled',
  remoteAiState: 'disabled',
  localServiceStateRevision: 1,
  localServiceStateSnapshotRef: TrackingRetentionSettingsWriteDefaults.LocalServiceStateSnapshotRef,
  durableSettingsStoreRef: TrackingRetentionSettingsWriteDefaults.DurableSettingsStoreRef,
  durableSettingsPersistenceState: 'persisted',
  commandTransportClaimState: 'claimed',
  serviceWritePreflightClaimState: 'claimed',
  serviceMutationExecutionState: 'claimed',
  portalWritableUiClaimState: 'unclaimed',
  platformRuntimeClaimState: 'unclaimed',
  childDeviceDeliveryClaimState: 'unclaimed',
  providerDeliveryClaimState: 'unclaimed',
  notificationReceiptClaimState: 'unclaimed',
  physicalDeviceClaimState: 'unclaimed',
  authorityClaimState: 'unclaimed',
  productClaimState: 'unclaimed',
} as const;

export function legacyTrackingReadModel() {
  const {
    latestActiveEventId: _latestActiveEventId,
    latestActiveObservedAt: _latestActiveObservedAt,
    activeKindCounts: _activeKindCounts,
    activeDeviceCounts: _activeDeviceCounts,
    activeCapabilityStatusCounts: _activeCapabilityStatusCounts,
    ...legacy
  } = TrackingReadModel;

  return {
    ...legacy,
    latestEventId: 'tracking-event-1',
    latestObservedAt: '2026-06-03T07:24:00Z',
  };
}

export function trackingProjectionInput(readModel = TrackingReadModel): TrackingStatusLiveProjectionInput {
  return {
    activityTrackingReadModelEvent: {
      event: GeneratedPortalAgentEvent.ActivityTrackingReadModelReported,
      eventId: 'tracking-read-model-event',
      correlationId: 'tracking-read-model-command',
      sentAt: '2026-06-03T07:25:01Z',
      sourcePeerId: 'agent-service',
      sourceRole: 'agent-service',
      targetPeerId: 'portal-dev',
      targetRole: 'portal',
      severity: 'info',
    },
    activityTrackingReadModel: {
      ok: true,
      value: readModel,
    },
  };
}

export function parsedTrackingRetentionSettingsWriteResult(): Parameters<
  typeof trackingRetentionSettingsHostedUiProof
>[0] {
  return {
    parseState: 'parsed',
    value: TrackingRetentionSettingsWriteResult,
  };
}
