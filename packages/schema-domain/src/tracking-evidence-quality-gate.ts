import type { TrackingReadModel } from './tracking-read-model';
import type { TrackingRetentionDeleteProof, TrackingRetentionExportProof } from './tracking-retention-runtime';

export type TrackingEvidenceQualityGateName =
  | 'location-ui-evidence-refs'
  | 'geofence-rule-and-source-refs'
  | 'nearby-place-provider-context'
  | 'retention-delete-before-after-proof'
  | 'retention-export-before-after-proof';

export interface TrackingEvidenceQualityGateInput {
  readonly readModel: TrackingReadModel;
  readonly retentionDeleteProof: TrackingRetentionDeleteProof;
  readonly retentionExportProof: TrackingRetentionExportProof;
}

export interface TrackingEvidenceQualityGateResult {
  readonly passed: boolean;
  readonly satisfiedGates: readonly TrackingEvidenceQualityGateName[];
  readonly missingGates: readonly TrackingEvidenceQualityGateName[];
  readonly locationEvidenceReferenceCount: number;
  readonly geofenceTransitionCount: number;
  readonly nearbyPlaceResultCount: number;
  readonly retentionDeleteBeforeLocationRows: number;
  readonly retentionDeleteAfterLocationRows: number;
  readonly retentionExportSourceLocationRows: number;
  readonly retentionExportedLocationRows: number;
}

interface TrackingEvidenceQualityGateStatus {
  readonly name: TrackingEvidenceQualityGateName;
  readonly passed: boolean;
}

export function evaluateTrackingEvidenceQualityGate(
  input: TrackingEvidenceQualityGateInput
): TrackingEvidenceQualityGateResult {
  const locationEvidenceReferenceCount =
    countEvidenceRefs(input.readModel.locationRows) + countEvidenceRefs(input.readModel.timeline);
  const gateStatuses = trackingEvidenceQualityGateStatuses(input);
  const satisfiedGates = gateStatuses.filter((gate) => gate.passed).map((gate) => gate.name);
  const missingGates = gateStatuses.filter((gate) => !gate.passed).map((gate) => gate.name);

  return {
    passed: missingGates.length === 0,
    satisfiedGates,
    missingGates,
    locationEvidenceReferenceCount,
    geofenceTransitionCount: input.readModel.geofenceTransitions.length,
    nearbyPlaceResultCount: input.readModel.nearbyPlaceRows.length,
    retentionDeleteBeforeLocationRows: input.retentionDeleteProof.beforeLocationRows,
    retentionDeleteAfterLocationRows: input.retentionDeleteProof.afterLocationRows,
    retentionExportSourceLocationRows: input.retentionExportProof.sourceLocationRows,
    retentionExportedLocationRows: input.retentionExportProof.exportedLocationRows,
  };
}

function trackingEvidenceQualityGateStatuses(
  input: TrackingEvidenceQualityGateInput
): readonly TrackingEvidenceQualityGateStatus[] {
  return [
    locationUiEvidenceRefsGate(input.readModel),
    geofenceRuleAndSourceRefsGate(input.readModel),
    nearbyPlaceProviderContextGate(input.readModel),
    retentionDeleteBeforeAfterGate(input.retentionDeleteProof),
    retentionExportBeforeAfterGate(input.retentionExportProof),
  ];
}

function locationUiEvidenceRefsGate(readModel: TrackingReadModel): TrackingEvidenceQualityGateStatus {
  return {
    name: 'location-ui-evidence-refs',
    passed:
      readModel.locationRows.length > 0 &&
      readModel.timeline.length > 0 &&
      readModel.locationRows.every((row) => hasEvidence(row.evidence)) &&
      readModel.timeline.every((row) => hasEvidence(row.evidence)),
  };
}

function geofenceRuleAndSourceRefsGate(readModel: TrackingReadModel): TrackingEvidenceQualityGateStatus {
  return {
    name: 'geofence-rule-and-source-refs',
    passed:
      readModel.geofenceTransitions.length > 0 &&
      readModel.geofenceTransitions.every(
        (row) =>
          row.ruleId.length > 0 &&
          row.geofenceId.length > 0 &&
          row.locationEvidenceId.length > 0 &&
          hasEvidence(row.evidence)
      ),
  };
}

function nearbyPlaceProviderContextGate(readModel: TrackingReadModel): TrackingEvidenceQualityGateStatus {
  return {
    name: 'nearby-place-provider-context',
    passed:
      readModel.nearbyPlaceRows.length > 0 &&
      readModel.nearbyPlaceRows.every(
        (row) =>
          row.providerKind !== 'unavailable' &&
          row.queryRadiusMeters >= 0 &&
          row.distanceMeters !== null &&
          row.category.length > 0 &&
          row.confidence >= 0 &&
          row.ambiguityState.length > 0 &&
          hasEvidence(row.evidence)
      ),
  };
}

function retentionDeleteBeforeAfterGate(proof: TrackingRetentionDeleteProof): TrackingEvidenceQualityGateStatus {
  const deletedEvidenceIds = new Set(proof.deletedEvidenceIds);

  return {
    name: 'retention-delete-before-after-proof',
    passed:
      proof.deletedEvidenceIds.length > 0 &&
      proof.beforeLocationRows > proof.afterLocationRows &&
      proof.readModel.locationRows.every((row) => !deletedEvidenceIds.has(row.evidenceId)),
  };
}

function retentionExportBeforeAfterGate(proof: TrackingRetentionExportProof): TrackingEvidenceQualityGateStatus {
  return {
    name: 'retention-export-before-after-proof',
    passed:
      proof.exportAllowed &&
      proof.sourceLocationRows > 0 &&
      proof.exportedLocationRows === proof.sourceLocationRows &&
      proof.remoteSyncDefault === 'disabled',
  };
}

function hasEvidence(refs: readonly unknown[]) {
  return refs.length > 0;
}

function countEvidenceRefs(rows: readonly { readonly evidence: readonly unknown[] }[]) {
  return rows.reduce((total, row) => total + row.evidence.length, 0);
}
