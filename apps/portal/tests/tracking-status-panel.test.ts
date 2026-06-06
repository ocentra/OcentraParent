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

const TrackingRetentionSettingsWriteResult = {
  schemaVersion: AgentProtocolSchemaVersion,
  commandId: 'tracking-retention-settings-write-command',
  settingsKind: 'retention-window-setting',
  writeState: 'service-write-command-accepted',
  acceptedAt: '2026-06-06T19:40:00.000Z',
  sourceMutationProofRefs: [
    'output/tracking-plan-proof/07-retention-and-custody-model/20-retention-settings-mutation-proof.json',
  ],
  commandTransportClaimed: true,
  serviceWritePreflightClaimed: true,
  serviceMutationExecuted: false,
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
        commandTransportClaimedRows: '1',
        serviceWritePreflightClaimedRows: '1',
        serviceMutationExecutedRows: '0',
        platformRuntimeClaimedRows: '0',
        childDeviceDeliveryClaimedRows: '0',
        providerDeliveryClaimedRows: '0',
        notificationReceiptClaimedRows: '0',
        physicalDeviceClaimedRows: '0',
        authorityClaimedRows: '0',
        productClaimReadyRows: '0',
        parserReason: 'Not reported',
        boundary:
          'Portal command/result rendering only; service mutation execution, platform runtime, child-device delivery, provider delivery, physical-device proof, authority, and product readiness remain unclaimed.',
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
