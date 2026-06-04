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
import { trackingStatusLiveSummary, trackingStatusProofRows } from '../src/tracking-status-panel';

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
  returned: 1,
  activeRows: 1,
  tombstoneRows: 0,
  capabilityStatus: 'recent',
  latestEventId: 'tracking-event-1',
  latestObservedAt: '2026-06-03T07:24:00Z',
  latestTombstoneEventId: null,
  latestTombstoneObservedAt: null,
  deletedEvidenceReferenceIds: [],
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
  ],
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

    expect(trackingStatusLiveSummary(liveActivity)).toEqual({
      title: 'Service read model',
      loadState: 'info',
      proofTier: 'P2 service proof',
      rowsReturned: '1',
      lastObserved: '2026-06-03T07:24:00Z',
      eventId: 'tracking-event-1',
      capability: 'recent',
      custody: 'child-device-query-store',
      device: 'child-device-1',
      platform: 'android',
      observer: 'tracking-engine',
      activityKind: 'tracking.expected-place.evaluated',
      subject: 'School',
      subjectId: 'expected-place-school',
      subjectKind: 'tracking-rule',
      evidenceReferences: 'tracking-evidence-1',
      parserReason: null,
      productClaim: 'No product claim',
    });
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
