import { describe, expect, it } from 'vitest';
import {
  ParentContractSchemaVersion,
  ParentEvidenceReferenceKind,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  DataCustodyBoundarySchema,
  DataCustodyBundleState,
  DataCustodyBundleType,
  DataCustodyClassId,
  DataCustodyRecoveryHandoffSchema,
  DataCustodyRecoveryHandoffState,
  DataCustodyRecoveryHandoffTarget,
  DataCustodySourceOfTruth,
  DataCustodySourceOfTruthSchema,
  DataCustodyRawPayloadState,
  DataCustodyRetentionDisposition,
  DataCustodyState,
  parseDataCustodyBoundary,
} from '../../src/custody-boundary';

const Boundary = {
  schemaVersion: ParentContractSchemaVersion.V0_6,
  family: {
    familyId: 'family-local-1',
  },
  evidence: {
    evidenceReferenceId: 'tracking-evidence-1',
    kind: ParentEvidenceReferenceKind.ActivityEvent,
    observedAt: '2026-06-01T00:00:00Z',
  },
  recordId: 'custody-record-1',
  storeRef: 'sqlite://tracking/evidence/1',
  custodyState: DataCustodyState.LocalOnly,
  rawPayloadState: DataCustodyRawPayloadState.Excluded,
  retentionPolicyId: 'tracking-retention-policy-default',
  retentionDisposition: DataCustodyRetentionDisposition.Retain,
} as const;

describe('data custody boundary contracts', () => {
  it('parses a parent-safe custody boundary with evidence refs only', () => {
    const parsed = parseDataCustodyBoundary(Boundary);

    expect(parsed.recordId).toBe('custody-record-1');
    expect(parsed.rawPayloadState).toBe(DataCustodyRawPayloadState.Excluded);
  });

  it('rejects missing custody record identity', () => {
    expect(
      DataCustodyBoundarySchema.safeParse({
        ...Boundary,
        recordId: '',
      }).success
    ).toBe(false);
  });

  it('parses canonical self and derived source-of-truth refs', () => {
    const selfOwned = DataCustodySourceOfTruth.self();
    const derived = DataCustodySourceOfTruth.derivedFromDataClass(
      DataCustodyClassId.EncryptedJournalSegment
    );

    expect(selfOwned.kind).toBe('self');
    expect(selfOwned.sourceClassId).toBe(null);
    expect(derived.kind).toBe('derived-from-data-class');
    expect(derived.sourceClassId).toBe(DataCustodyClassId.EncryptedJournalSegment);
  });

  it('rejects ambiguous source-of-truth refs', () => {
    expect(
      DataCustodySourceOfTruthSchema.safeParse({
        kind: 'self',
        sourceClassId: DataCustodyClassId.EncryptedJournalSegment,
      }).success
    ).toBe(false);

    expect(
      DataCustodySourceOfTruthSchema.safeParse({
        kind: 'derived-from-data-class',
        sourceClassId: null,
      }).success
    ).toBe(false);
  });

  it('parses non-mutating recovery preview and delete handoff contracts', () => {
    const previewHandoff = DataCustodyRecoveryHandoffSchema.parse({
      bundleType: DataCustodyBundleType.ImportPreview,
      bundleState: DataCustodyBundleState.BundlePreviewOnly,
      handoffTarget: DataCustodyRecoveryHandoffTarget.SetupRestorePreview,
      handoffState: DataCustodyRecoveryHandoffState.PreviewOnly,
      previewIsNonMutating: true,
      explicitParentConfirmationRequired: true,
      sourceOfTruthPreserved: true,
      tombstonesPreserved: true,
      deleteRequestRequired: false,
    });
    const deleteHandoff = DataCustodyRecoveryHandoffSchema.parse({
      bundleType: DataCustodyBundleType.Export,
      bundleState: DataCustodyBundleState.BundleWritten,
      handoffTarget: DataCustodyRecoveryHandoffTarget.ParentLocalDeleteRuntime,
      handoffState: DataCustodyRecoveryHandoffState.DeletePending,
      previewIsNonMutating: true,
      explicitParentConfirmationRequired: false,
      sourceOfTruthPreserved: true,
      tombstonesPreserved: true,
      deleteRequestRequired: true,
    });

    expect(previewHandoff.handoffState).toBe('preview-only');
    expect(deleteHandoff.handoffState).toBe('delete-pending');
  });

  it('rejects recovery handoffs that mutate preview state or omit delete requirements', () => {
    const preview = {
      bundleType: DataCustodyBundleType.ImportPreview,
      bundleState: DataCustodyBundleState.BundlePreviewOnly,
      handoffTarget: DataCustodyRecoveryHandoffTarget.SetupRestorePreview,
      handoffState: DataCustodyRecoveryHandoffState.PreviewOnly,
      previewIsNonMutating: true,
      explicitParentConfirmationRequired: true,
      sourceOfTruthPreserved: true,
      tombstonesPreserved: true,
      deleteRequestRequired: false,
    } as const;

    expect(
      DataCustodyRecoveryHandoffSchema.safeParse({
        ...preview,
        previewIsNonMutating: false,
      }).success
    ).toBe(false);
    expect(
      DataCustodyRecoveryHandoffSchema.safeParse({
        ...preview,
        explicitParentConfirmationRequired: false,
      }).success
    ).toBe(false);
    expect(
      DataCustodyRecoveryHandoffSchema.safeParse({
        ...preview,
        handoffTarget: DataCustodyRecoveryHandoffTarget.ParentLocalDeleteRuntime,
        handoffState: DataCustodyRecoveryHandoffState.DeleteConfirmed,
      }).success
    ).toBe(false);
    expect(
      DataCustodyRecoveryHandoffSchema.safeParse({
        ...preview,
        bundleState: DataCustodyBundleState.BundleWrongHousehold,
        handoffState: DataCustodyRecoveryHandoffState.ManualRequired,
      }).success
    ).toBe(false);
  });
});
