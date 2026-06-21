import type {
  TrackingLocationEvidence,
  TrackingRetentionPolicy,
} from './tracking-evidence';
import type { TrackingReadModel } from './tracking-read-model';

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
