import type { TrackingLocationEvidence, TrackingRetentionPolicy } from './tracking-evidence';
import { TrackingReadModelSchema, type TrackingReadModel } from './tracking-read-model';

export interface TrackingRetentionDeleteInput {
  readonly readModel: TrackingReadModel;
  readonly generatedAt: TrackingReadModel['generatedAt'];
  readonly deletedEvidenceIds: readonly TrackingLocationEvidence['evidenceId'][];
}

export interface TrackingRetentionDeleteProof {
  readonly beforeLocationRows: TrackingReadModel['returned'];
  readonly afterLocationRows: TrackingReadModel['returned'];
  readonly deletedEvidenceIds: readonly TrackingLocationEvidence['evidenceId'][];
  readonly readModel: TrackingReadModel;
}

export interface TrackingRetentionExportInput {
  readonly readModel: TrackingReadModel;
  readonly generatedAt: TrackingReadModel['generatedAt'];
  readonly policy: TrackingRetentionPolicy;
}

export interface TrackingRetentionExportProof {
  readonly exportAllowed: TrackingRetentionPolicy['exportAllowed'];
  readonly sourceLocationRows: TrackingReadModel['returned'];
  readonly exportedLocationRows: TrackingReadModel['returned'];
  readonly custodyLabel: TrackingRetentionPolicy['custodyLabel'];
  readonly retentionMode: TrackingRetentionPolicy['mode'];
  readonly remoteSyncDefault: TrackingRetentionPolicy['remoteSyncDefault'];
  readonly readModel: TrackingReadModel;
}

export function applyTrackingRetentionDelete(input: TrackingRetentionDeleteInput): TrackingRetentionDeleteProof {
  const deleted = new Set(input.deletedEvidenceIds);
  const locationRows = input.readModel.locationRows.filter((row) => !deleted.has(row.evidenceId));
  const deviceStatusRows = input.readModel.deviceStatusRows.filter(
    (row) => row.lastLocationEvidenceId === null || !deleted.has(row.lastLocationEvidenceId)
  );
  const geofenceTransitions = input.readModel.geofenceTransitions.filter((row) => !deleted.has(row.locationEvidenceId));
  const expectedPlaceDecisions = input.readModel.expectedPlaceDecisions.filter(
    (row) => !deleted.has(row.locationEvidenceId)
  );
  const nearbyPlaceRows = input.readModel.nearbyPlaceRows.filter((row) => !deleted.has(row.locationEvidenceId));
  const timeline = input.readModel.timeline.filter((row) => !deleted.has(row.rowId));

  const readModel = TrackingReadModelSchema.parse({
    ...input.readModel,
    generatedAt: input.generatedAt,
    returned: timeline.length,
    locationRows,
    deviceStatusRows,
    geofenceTransitions,
    expectedPlaceDecisions,
    nearbyPlaceRows,
    timeline,
    capabilityStatus: locationRows.length === 0 ? 'stale' : input.readModel.capabilityStatus,
  });

  return {
    beforeLocationRows: input.readModel.locationRows.length,
    afterLocationRows: readModel.locationRows.length,
    deletedEvidenceIds: input.deletedEvidenceIds,
    readModel,
  };
}

export function applyTrackingRetentionExport(input: TrackingRetentionExportInput): TrackingRetentionExportProof {
  const locationRows = input.policy.exportAllowed
    ? input.readModel.locationRows.map((row) => ({
        ...row,
        custodyLabel: input.policy.custodyLabel,
        retentionMode: input.policy.mode,
      }))
    : [];
  const deviceStatusRows = input.policy.exportAllowed
    ? input.readModel.deviceStatusRows.map((row) => ({
        ...row,
        custodyLabel: input.policy.custodyLabel,
        retentionMode: input.policy.mode,
      }))
    : [];
  const timeline = input.policy.exportAllowed ? input.readModel.timeline : [];
  const readModel = TrackingReadModelSchema.parse({
    ...input.readModel,
    generatedAt: input.generatedAt,
    custodyLabel: input.policy.custodyLabel,
    capabilityStatus: input.policy.exportAllowed ? input.readModel.capabilityStatus : 'unavailable',
    returned: timeline.length,
    locationRows,
    deviceStatusRows,
    retentionPolicies: [input.policy],
    timeline,
  });

  return {
    exportAllowed: input.policy.exportAllowed,
    sourceLocationRows: input.readModel.locationRows.length,
    exportedLocationRows: readModel.locationRows.length,
    custodyLabel: input.policy.custodyLabel,
    retentionMode: input.policy.mode,
    remoteSyncDefault: input.policy.remoteSyncDefault,
    readModel,
  };
}
