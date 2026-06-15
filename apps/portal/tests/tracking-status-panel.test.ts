import { describe, expect, it } from 'vitest';
import { ActivityQuerySchemaVersion } from '@ocentra-parent/activity-domain/query';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import { PortalRoute, TrackingStatusProofArtifacts } from '@ocentra-parent/portal-domain/contracts';
import { resolveLiveActivityState } from '../src/live-activity-state';
import { shouldRenderTrackingStatusRoute } from '../src/TrackingStatusRoutePanel';
import { trackingChildCheckInProof, trackingChildRuntimeUiProof } from '../src/tracking-child-check-in-proof';
import { trackingEvidenceDrawerHostedUiProof } from '../src/tracking-evidence-drawer-hosted-ui-proof';
import { trackingNotificationParentSurfaceHostedUiProof } from '../src/tracking-notification-parent-surface-hosted-ui-proof';
import { trackingParentActionReadinessHostedUiProof } from '../src/tracking-parent-action-readiness-hosted-ui-proof';
import { trackingMissingDeviceHostedUiProof } from '../src/tracking-missing-device-hosted-ui-proof';
import { trackingReportExportHostedUiProof } from '../src/tracking-report-export-hosted-ui-proof';
import { trackingReportPolicyConsumerHostedUiProof } from '../src/tracking-report-policy-consumer-hosted-ui-proof';
import { trackingRetentionSettingsHostedUiProof } from '../src/tracking-retention-settings-hosted-ui-proof';
import {
  trackingFamilyDashboardHostedRollupProof,
  trackingStatusLiveSummary,
  trackingStatusProofRows,
  trackingStatusServiceDataCoverage,
  trackingUnsupportedManualPlatformProof,
} from '../src/tracking-status-panel';

const ExpectedTrackingStateTitles = [
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

const ExpectedTrackingProofArtifacts = [
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

const ExpectedRetentionDeletedRow = {
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

const TrackingReadModel = {
  schemaVersion: ActivityQuerySchemaVersion,
  generatedAt: '2026-06-03T07:25:00Z',
  custodyLabel: 'child-device-query-store',
  limit: 20,
  returned: 2,
  activeRows: 1,
  tombstoneRows: 1,
  capabilityStatus: 'recent',
  latestEventId: 'tracking-event-1',
  latestObservedAt: '2026-06-03T07:24:00Z',
  latestTombstoneEventId: 'tracking-retention-delete-1',
  latestTombstoneObservedAt: '2026-06-03T07:26:00Z',
  deletedEvidenceReferenceIds: ['location-evidence-1'],
  rows: [
    {
      schemaVersion: ActivityQuerySchemaVersion,
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
      schemaVersion: ActivityQuerySchemaVersion,
      eventId: 'tracking-retention-delete-1',
      observedAt: '2026-06-03T07:26:00Z',
      deviceId: 'child-device-1',
      platform: 'android',
      observer: 'tracking-retention',
      kind: 'activity.tracking.retention.deleted',
      subjectKind: 'location-evidence',
      subjectId: 'location-evidence-1',
      subjectDisplayName: null,
      capabilityStatus: 'recent',
      queryVisibility: 'tombstone',
      deletedAt: '2026-06-03T07:26:00Z',
      evidenceReferenceIds: [],
      deletedEvidenceReferenceIds: ['location-evidence-1'],
      evidence: [],
    },
  ],
} as const;

const ExpectedTrackingLiveSummary = {
  title: 'Service read model',
  loadState: 'info',
  proofTier: 'P2 service proof',
  rowsReturned: '2',
  lastObserved: '2026-06-03T07:24:00Z',
  eventId: 'tracking-event-1',
  capability: 'recent',
  custody: 'child-device-query-store',
  evidenceReferences: 'tracking-evidence-1 | location-evidence-1',
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
      title: 'activity.tracking.retention.deleted',
      eventId: 'tracking-retention-delete-1',
      observedAt: '2026-06-03T07:26:00Z',
      device: 'child-device-1',
      platform: 'android',
      observer: 'tracking-retention',
      activityKind: 'activity.tracking.retention.deleted',
      subject: 'location-evidence | location-evidence-1',
      status: 'tombstone | recent',
      evidenceReferences: 'Not reported',
      deletedEvidence: 'location-evidence-1',
      productClaim: 'No product claim',
    },
  ],
} as const;

const ExpectedTrackingServiceDataCoverage = {
  title: 'Service data coverage',
  loadState: 'info',
  proofTier: 'P2 service proof',
  rowsReturned: '2',
  rowVisibility: '1 | 1',
  lastObserved: '2026-06-03T07:26:00Z',
  eventId: 'tracking-retention-delete-1',
  capability: 'recent',
  custody: 'child-device-query-store',
  activityKinds: 'tracking.expected-place.evaluated | activity.tracking.retention.deleted',
  evidenceReferences: 'tracking-evidence-1',
  deletedEvidence: 'location-evidence-1',
  productClaim: 'No product claim',
} as const;

const ExpectedUnsupportedManualRows = [
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

const ExpectedFamilyDashboardHostedRollupRows = [
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

const ExpectedReportExportHostedUiRows = [
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

const ExpectedReportExportHostedUiProof = {
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

const ExpectedReportPolicyConsumerHostedUiRows = [
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

const ExpectedReportPolicyConsumerHostedUiProof = {
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

const ExpectedNotificationParentSurfaceRows = [
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

const ExpectedParentActionReadinessRows = [
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

const ExpectedMissingDeviceRows = [
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

const TrackingRetentionSettingsWriteResult = {
  schemaVersion: AgentProtocolSchemaVersion,
  commandId: 'tracking-retention-settings-write-command',
  settingsKind: 'retention-window-setting',
  writeState: 'service-write-command-accepted',
  acceptedAt: '2026-06-06T19:40:00.000Z',
  sourceWriterIntentRefs: ['tracking-retention-settings-write-retention-window'],
  sourceReadModelProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
  ],
  sourceMutationProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
  ],
  appliedRetentionWindowHours: 168,
  appliedDeleteAfterAlertResolved: false,
  parentExportPrepared: false,
  remoteSyncEnabled: false,
  remoteAiEnabled: false,
  localServiceStateRevision: 1,
  localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
  durableSettingsPersisted: false,
  commandTransportClaimed: true,
  serviceWritePreflightClaimed: true,
  serviceMutationExecuted: true,
  portalWritableUiClaimed: false,
  platformRuntimeClaimed: false,
  childDeviceDeliveryClaimed: false,
  providerDeliveryClaimed: false,
  notificationReceiptClaimed: false,
  physicalDeviceClaimed: false,
  authorityClaimed: false,
  productClaimReady: false,
} as const;

describe('tracking status proof surface', () => {
  it('lists the first-target tracking states as fixture proof without product claims', () => {
    const rows = trackingStatusProofRows();

    expect(rows.map((row) => row.title)).toEqual(ExpectedTrackingStateTitles);
    expect(rows.every((row) => row.proofTier === 'P1 fixture proof')).toBe(true);
    expect(rows.every((row) => row.productClaim === 'No product claim')).toBe(true);
    expect(rows.every((row) => row.proofArtifact.startsWith('output/tracking-plan-proof/'))).toBe(true);
    expect(rows.map((row) => row.proofArtifact)).toEqual(ExpectedTrackingProofArtifacts);
    expect(rows.filter((row) => row.missingProof === 'Physical device proof required').map((row) => row.title)).toEqual(
      ['Permission required', 'Temporary live', 'Missing device']
    );
  });

  it('only attaches the proof surface to the live tracking product route', () => {
    expect(shouldRenderTrackingStatusRoute(PortalRoute.PolicyTracking)).toBe(true);
    expect(shouldRenderTrackingStatusRoute(PortalRoute.Overview)).toBe(false);
  });

  it('marks deleted location history as hidden without rendering deleted evidence ids', () => {
    const retentionRow = trackingStatusProofRows().find((row) => row.title === 'Retention deleted');

    expect(retentionRow).toEqual(ExpectedRetentionDeletedRow);
    expect(JSON.stringify(retentionRow)).not.toContain('location-evidence-1');
  });

  it('summarizes the live service-backed tracking read model without product completion claims', () => {
    const liveActivity = resolveLiveActivityState([trackingEvent(JSON.stringify(TrackingReadModel))]);

    expect(trackingStatusLiveSummary(liveActivity)).toEqual(ExpectedTrackingLiveSummary);
  });

  it('renders service-data coverage from the live read model without device or provider claims', () => {
    const liveActivity = resolveLiveActivityState([trackingEvent(JSON.stringify(TrackingReadModel))]);

    expect(trackingStatusServiceDataCoverage(liveActivity)).toEqual(ExpectedTrackingServiceDataCoverage);
  });

  it('renders evidence drawer proof from the selected citation without evaluator or dispatch claims', () => {
    const liveActivity = resolveLiveActivityState([trackingEvent(JSON.stringify(TrackingReadModel))]);
    const liveSummary = trackingStatusLiveSummary(liveActivity);

    expect(trackingEvidenceDrawerHostedUiProof(liveSummary.citations[0] ?? null)).toEqual({
      title: 'Evidence drawer proof',
      body: 'Hosted route renders a read-only evidence drawer from the selected service-backed citation without evaluating policy or dispatching actions.',
      proofTier: 'P2 service proof',
      drawerMode: 'read-only evidence drawer',
      sourceEventId: 'tracking-event-1',
      evidenceReferences: 'tracking-evidence-1',
      deletedEvidence: 'Not reported',
      proofArtifact: TrackingStatusProofArtifacts.HostedEvidenceDrawer,
      boundary:
        'Display-only evidence drill-in; policy evaluation, action dispatch, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      policyEvaluatorClaimedRows: '0',
      actionDispatchClaimedRows: '0',
      childDeviceDeliveryClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
    });
  });
});

describe('tracking retention settings hosted proof surface', () => {
  it('renders retention write preflight result without product-ready mutation claims', () => {
    const liveActivity = resolveLiveActivityState([
      trackingRetentionSettingsWriteEvent(JSON.stringify(TrackingRetentionSettingsWriteResult)),
    ]);

    expect(
      trackingRetentionSettingsHostedUiProof(liveActivity.activityTrackingRetentionSettingsWriteResult)
    ).toMatchObject({
      title: 'Retention settings read-model UI',
      writePreflight: {
        title: 'Retention write preflight result',
        commandId: 'tracking-retention-settings-write-command',
        settingsKind: 'retention-window-setting',
        writeState: 'service-write-command-accepted',
        acceptedAt: '2026-06-06T19:40:00.000Z',
        sourceMutationProofRefs:
          'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
        sourceWriterIntentRefs: 'tracking-retention-settings-write-retention-window',
        sourceReadModelProofRefs:
          'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
        appliedRetentionWindowHours: '168',
        appliedDeleteAfterAlertResolved: '0',
        parentExportPrepared: '0',
        remoteSyncEnabled: '0',
        remoteAiEnabled: '0',
        localServiceStateRevision: '1',
        localServiceStateSnapshotRef: 'agent-service-local-retention-settings-state',
        durableSettingsPersistedRows: '0',
        commandTransportClaimedRows: '1',
        serviceWritePreflightClaimedRows: '1',
        serviceMutationExecutedRows: '1',
        platformRuntimeClaimedRows: '0',
        childDeviceDeliveryClaimedRows: '0',
        providerDeliveryClaimedRows: '0',
        notificationReceiptClaimedRows: '0',
        physicalDeviceClaimedRows: '0',
        authorityClaimedRows: '0',
        productClaimReadyRows: '0',
        parserReason: 'Not reported',
        boundary:
          'Portal command/result rendering only; service mutation execution and local state revision are local proof, while durable product persistence, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
      },
    });
  });
});

describe('tracking dashboard and platform proof surface', () => {
  it('renders family dashboard rollup rows without provider, device, authority, or product-ready claims', () => {
    const proof = trackingFamilyDashboardHostedRollupProof();

    expect(proof).toEqual({
      title: 'Family dashboard tracking rollup',
      body: 'Hosted route renders family active, child attention, and retention audit rollups from existing tracking proof refs without claiming device delivery.',
      proofTier: 'P2 service proof',
      rowsReturned: '3',
      proofArtifact: TrackingStatusProofArtifacts.FamilyDashboardRollup,
      boundary:
        'Hosted dashboard rollup rendering only; child-device delivery, provider delivery, notification receipt ingestion, physical-device proof, authority, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      childDeviceDeliveryClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      notificationReceiptClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedFamilyDashboardHostedRollupRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:product ready|physical device proved|provider delivered)/iu);
  });

  it('renders report export read-model packets without raw export, mutation, runtime, or product claims', () => {
    const proof = trackingReportExportHostedUiProof();

    expect(proof).toEqual(ExpectedReportExportHostedUiProof);
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:raw location payload exported|service mutation executed|product-ready export delivered)/iu
    );
  });

  it('renders report policy consumer rows without AI, mutation, device, or product claims', () => {
    const proof = trackingReportPolicyConsumerHostedUiProof();

    expect(proof).toEqual(ExpectedReportPolicyConsumerHostedUiProof);
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:AI execution claimed|policy mutation executed|physical device proved|product ready)/iu
    );
  });

  it('renders unsupported/manual platform rows without invented capability or product claims', () => {
    const proof = trackingUnsupportedManualPlatformProof();

    expect(proof).toEqual({
      title: 'Unsupported/manual tracking platform proof',
      body: 'Unsupported platform and manual-required adapter rows render as degraded states without invented capability.',
      proofTier: 'P1 fixture proof',
      rowsReturned: '7',
      manualRequiredRows: '5',
      unavailableRows: '1',
      authorityRequiredRows: '1',
      fakeCapabilityRows: '0',
      productClaimReadyRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.UnsupportedManualPlatform,
      missingProof: 'Manual proof required',
      boundary:
        'Hosted render-state proof only; physical-device, authority, provider delivery, and product readiness remain unclaimed.',
      productClaim: 'No product claim',
      rows: ExpectedUnsupportedManualRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:product ready|physical device proved|authority proved)/iu);
  });
});

describe('tracking notification parent-surface hosted proof surface', () => {
  it('renders notification history rows without delivery, receipt, or product claims', () => {
    const proof = trackingNotificationParentSurfaceHostedUiProof();

    expect(proof).toEqual({
      title: 'Notification history intent UI',
      body: 'Hosted route renders parent notification history, manual action, and provider unavailable rows from existing tracking notification proof refs without claiming provider delivery or receipt runtime.',
      proofTier: 'P2 service proof',
      rowsReturned: '3',
      proofArtifact: TrackingStatusProofArtifacts.NotificationParentSurfaceHistory,
      boundary:
        'Hosted notification history rendering only; preference mutation, quiet-hours runtime, provider delivery, receipt ingestion, child-device delivery, physical-device proof, authority, production storage, adapter dispatch, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      renderedParentNotificationUiRows: '3',
      parentPreferenceMutationRows: '0',
      providerDeliveryClaimedRows: '0',
      receiptIngestionClaimedRows: '0',
      childDeviceDeliveryClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productionStorageClaimedRows: '0',
      adapterDispatchClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedNotificationParentSurfaceRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:provider delivered|receipt ingested|product ready)/iu);
  });
});

describe('tracking parent action readiness hosted proof surface', () => {
  it('renders expected-place and acknowledgement action rows without runtime, delivery, or product claims', () => {
    const proof = trackingParentActionReadinessHostedUiProof();

    expect(proof).toEqual({
      title: 'Parent action readiness UI',
      body: 'Hosted route renders expected-place alert policy and parent acknowledgement action readiness rows from existing tracking proof refs without claiming live mutation or delivery runtime.',
      proofTier: 'P2 service proof',
      expectedPlaceProofArtifact: TrackingStatusProofArtifacts.ExpectedPlaceAlertPolicy,
      acknowledgementProofArtifact: TrackingStatusProofArtifacts.ParentAcknowledgementActionReadiness,
      boundary:
        'Hosted parent action readiness rendering only; live service mutation, alert delivery, provider delivery, receipt ingestion, child-device runtime, physical-device proof, authority, production workers, adapter dispatch, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      expectedPlaceRows: '4',
      acknowledgementActionRows: '5',
      renderedParentActionRows: '9',
      liveServiceMutationRows: '0',
      providerDeliveryClaimedRows: '0',
      notificationReceiptClaimedRows: '0',
      childDeviceRuntimeClaimedRows: '0',
      physicalDeviceClaimedRows: '0',
      authorityClaimedRows: '0',
      productionWorkerClaimedRows: '0',
      adapterDispatchClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedParentActionReadinessRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(
      /(?:service mutation executed|provider delivered|receipt ingested|product ready)/iu
    );
  });
});

describe('tracking missing-device hosted proof surface', () => {
  it('renders missing-device state rows without current-location, physical-device, or product claims', () => {
    const proof = trackingMissingDeviceHostedUiProof();

    expect(proof).toEqual({
      title: 'Missing-device state UI',
      body: 'Hosted route renders last-known, offline, contact-requested, and manual-required missing-device rows from existing WP29 proof without claiming current location or OS lost-mode runtime.',
      proofTier: 'P2 service proof',
      sourceProofArtifact: TrackingStatusProofArtifacts.MissingDeviceMode,
      boundary:
        'Hosted missing-device rendering only; current location runtime, powered-off tracking, remote sync, provider delivery, physical-device proof, OS lost-mode APIs, authority, production workers, and product readiness remain unclaimed.',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
      renderedMissingDeviceRows: '4',
      lastKnownOnlyRows: '1',
      offlineRows: '1',
      contactRequestedRows: '1',
      manualRequiredRows: '1',
      currentLocationRuntimeClaimedRows: '0',
      poweredOffTrackingClaimedRows: '0',
      remoteSyncRuntimeClaimedRows: '0',
      providerDeliveryClaimedRows: '0',
      physicalDeviceProofClaimedRows: '0',
      osLostModeApiClaimedRows: '0',
      productClaimReadyRows: '0',
      rows: ExpectedMissingDeviceRows,
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:current location proved|lost mode executed|product ready)/iu);
  });
});

describe('tracking child check-in proof surface', () => {
  it('renders child check-in copy as calm fixture proof without delivery claims', () => {
    const proof = trackingChildCheckInProof();

    expect(proof).toEqual({
      title: 'Child check-in request',
      body: 'Your parent is asking you to check in. Are you safe?',
      proofTier: 'P1 fixture proof',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.ChildCheckIn,
      copyBoundary: 'Calm copy, no accusation',
      safeAction: "I'm safe",
      helpAction: 'Need help',
      shareLocationAction: 'Share current location',
      callParentAction: 'Call parent',
      deliveryBoundary: 'Child-device delivery not proved',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:trouble|lying|bad place|delivered|product ready)/iu);
  });

  it('renders child runtime UI copy without delivery or product claims', () => {
    const proof = trackingChildRuntimeUiProof();

    expect(proof).toEqual({
      title: 'Child runtime UI proof',
      body: 'Child sees a clear tracking request, safe response, help response, and location-share consent copy.',
      proofTier: 'P2 service proof',
      evidence: 'UI fixture proof',
      proofArtifact: TrackingStatusProofArtifacts.ChildRuntimeUi,
      disclosure: 'Tracking request disclosed',
      safeResponse: 'Safe response visible',
      helpResponse: 'Help response visible',
      locationShareConsent: 'Location share asks consent',
      runtimeBoundary: 'Hosted proof only, not child-agent delivery',
      deliveryBoundary: 'Child-device delivery not proved',
      missingProof: 'Manual proof required',
      productClaim: 'No product claim',
    });
    expect(JSON.stringify(proof)).not.toMatch(/(?:delivered|physical device proved|product ready)/iu);
  });
});

function trackingEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-read-model-event',
    correlationId: 'tracking-read-model-command',
    sentAt: '2026-06-03T07:25:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityTrackingReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}

function trackingRetentionSettingsWriteEvent(serializedResult: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'tracking-retention-settings-write-result-event',
    correlationId: 'tracking-retention-settings-write-command',
    sentAt: '2026-06-06T19:40:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityTrackingRetentionSettingsWriteReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityTrackingRetentionSettingsWriteResult]: serializedResult,
    },
    snapshot: null,
  });
}
