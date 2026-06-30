import { describe, expect, it } from 'vitest';
import {
  CanonicalDataCustodySourceOfTruthMatrix,
  DataCustodyClaimSafeLanguage,
  DataCustodyKnownGaps,
  DataCustodySourceOfTruthContractProofSchema,
  DataCustodySourceOfTruthProofReadModel as DataCustodySourceOfTruthContractProofReadModel,
  DataCustodySourceOfTruthMatrixRowSchema,
  HostedOcentraMetadataClassIds,
  MustNeverBeHostedByDefaultClassIds,
  RequiredDataCustodyClassIds,
  RequiredDataCustodyNonClaims,
  summarizeDataCustodyAuthorities,
  summarizeDataCustodyOcentraHostingModes,
} from '@ocentra-parent/schema-domain/data-custody-matrix';
import { DataCustodyClassId } from '@ocentra-parent/schema-domain/custody-boundary';

describe('data custody source-of-truth contracts', () => {
  coversEveryActiveDataClassProof();
  hostedMetadataVsForbiddenHostingProof();
  derivedRowsStayDerivedAndTraceableProof();
  redactionAndEvidenceBoundaryProof();
  accountControlPlaneAndNonClaimsStayExplicitProof();
});

function coversEveryActiveDataClassProof(): void {
  it('covers every active data class exactly once and keeps authority and hosting counts explicit', () => {
    const proof = DataCustodySourceOfTruthContractProofSchema.parse(DataCustodySourceOfTruthContractProofReadModel);
    const authorityCounts = summarizeDataCustodyAuthorities(proof.rows);
    const hostingModeCounts = summarizeDataCustodyOcentraHostingModes(proof.rows);

    expect(CanonicalDataCustodySourceOfTruthMatrix.rows.map((row) => row.classId)).toEqual(RequiredDataCustodyClassIds);
    expect(proof.rows.map((row) => row.classId)).toEqual(RequiredDataCustodyClassIds);
    expect(new Set(proof.rows.map((row) => row.rowId)).size).toBe(RequiredDataCustodyClassIds.length);
    expect(Object.values(authorityCounts).reduce((sum, count) => sum + count, 0)).toBe(proof.rows.length);
    expect(hostingModeCounts['allowed-metadata-only']).toBe(6);
    expect(hostingModeCounts['short-lived-status-only']).toBe(1);
    expect(hostingModeCounts['public-release-only']).toBe(1);
    expect(hostingModeCounts.forbidden).toBe(proof.rows.length - 8);
  });
}

function hostedMetadataVsForbiddenHostingProof(): void {
  it('keeps Ocentra-hosted metadata allowlist explicit and rejects raw child evidence or rules from that allowlist', () => {
    const proof = DataCustodySourceOfTruthContractProofReadModel;

    expect(HostedOcentraMetadataClassIds).toEqual([
      'account-identity-metadata',
      'subscription-entitlement-metadata',
      'license-download-update-metadata',
      'device-registration-pairing-route-metadata',
      'minimal-notification-routing-metadata',
      'short-lived-report-compiler-status',
      'support-case-metadata',
      'public-website-release-status',
    ]);
    expect(MustNeverBeHostedByDefaultClassIds).toContain(DataCustodyClassId.EvidenceJournalSegments);
    expect(MustNeverBeHostedByDefaultClassIds).toContain(DataCustodyClassId.ParentRulesAndApprovalHistory);
    expect(MustNeverBeHostedByDefaultClassIds).toContain(DataCustodyClassId.UniversalDecryptKeys);
    expect(HostedOcentraMetadataClassIds).not.toContain(DataCustodyClassId.ParentRulesAndApprovalHistory);
    expect(HostedOcentraMetadataClassIds).not.toContain(DataCustodyClassId.BrowserUrlHistory);
    expect(proof.mustNeverBeHostedByDefault).toEqual(MustNeverBeHostedByDefaultClassIds);
    expect(
      DataCustodySourceOfTruthMatrixRowSchema.safeParse({
        ...rowFor(DataCustodyClassId.AccountIdentityMetadata),
        ocentraHostedByDefault: false,
      }).success
    ).toBe(false);
  });
}

function derivedRowsStayDerivedAndTraceableProof(): void {
  it('marks read models and reports as derived rows with cited source classes instead of self truth', () => {
    const sqliteRow = rowFor(DataCustodyClassId.SqliteEvidenceReadModelDatabase);
    const reportRow = rowFor(DataCustodyClassId.GeneratedLongTermReports);
    const storageRow = rowFor(DataCustodyClassId.ParentOwnedStorageContents);

    expect(sqliteRow.sourceOfTruth.kind).toBe('derived-from-data-classes');
    expect(sqliteRow.sourceOfTruth.sourceClassIds).toEqual([DataCustodyClassId.EvidenceJournalSegments]);
    expect(sqliteRow.derivedUseOnly).toBe(true);
    expect(reportRow.sourceOfTruth.kind).toBe('derived-from-data-classes');
    expect(reportRow.sourceOfTruth.sourceClassIds).toEqual([
      DataCustodyClassId.SqliteEvidenceReadModelDatabase,
      DataCustodyClassId.LocalAiAndPolicyDecisions,
      DataCustodyClassId.ChildProfile,
    ]);
    expect(storageRow.sourceOfTruth.sourceClassIds).toContain(DataCustodyClassId.EvidenceJournalSegments);
    expect(
      DataCustodySourceOfTruthMatrixRowSchema.safeParse({
        ...sqliteRow,
        sourceOfTruth: {
          ...sqliteRow.sourceOfTruth,
          kind: 'self',
        },
      }).success
    ).toBe(false);
  });
}

function redactionAndEvidenceBoundaryProof(): void {
  it('keeps raw child evidence out of notification exposure and rejects hosted-by-default raw evidence rows', () => {
    const evidenceRow = rowFor(DataCustodyClassId.EvidenceJournalSegments);
    const screenshotRow = rowFor(DataCustodyClassId.ScreenshotsAndScreenAnalysisImages);
    const assistantRow = rowFor(DataCustodyClassId.AssistantChildEvidenceContext);

    expect(evidenceRow.reportExposure).toBe('allowed-references-only');
    expect(evidenceRow.notificationExposure).toBe('none');
    expect(evidenceRow.rawChildEvidenceAllowed).toBe(true);
    expect(screenshotRow.reportExposure).toBe('none');
    expect(screenshotRow.notificationExposure).toBe('none');
    expect(assistantRow.reportExposure).toBe('none');
    expect(assistantRow.notificationExposure).toBe('none');
    expect(
      DataCustodySourceOfTruthMatrixRowSchema.safeParse({
        ...evidenceRow,
        ocentraHostedByDefault: true,
      }).success
    ).toBe(false);
    expect(
      DataCustodySourceOfTruthMatrixRowSchema.safeParse({
        ...screenshotRow,
        notificationExposure: 'minimal',
      }).success
    ).toBe(false);
  });
}

function accountControlPlaneAndNonClaimsStayExplicitProof(): void {
  it('keeps account and provider separation explicit and rejects false positive hosting or support decrypt claims', () => {
    const proof = DataCustodySourceOfTruthContractProofReadModel;

    expect(proof.accountControlPlaneSeparated).toBe(true);
    expect(proof.providerOwnedBillingIdentitySeparated).toBe(true);
    expect(proof.ocentraIsDefaultChildDataStore).toBe(false);
    expect(proof.providerAutoApplyClaimed).toBe(false);
    expect(proof.supportDecryptByDefaultClaimed).toBe(false);
    expect(proof.sqliteAsTruthLayerClaimed).toBe(false);
    expect(proof.rawChildActivityHostedByDefaultClaimed).toBe(false);
    expect(proof.nonClaims).toEqual(RequiredDataCustodyNonClaims);
    expect(DataCustodyClaimSafeLanguage).toHaveLength(5);
    expect(DataCustodyKnownGaps).toContain(
      'Support decrypt-by-default remains false until product and key-custody decisions exist.'
    );
    expect(
      DataCustodySourceOfTruthContractProofSchema.safeParse({
        ...proof,
        supportDecryptByDefaultClaimed: true,
      }).success
    ).toBe(false);
    expect(
      DataCustodySourceOfTruthContractProofSchema.safeParse({
        ...proof,
        rawChildActivityHostedByDefaultClaimed: true,
      }).success
    ).toBe(false);
  });
}

function rowFor(classId: (typeof RequiredDataCustodyClassIds)[number]) {
  const row = DataCustodySourceOfTruthContractProofReadModel.rows.find((candidate) => candidate.classId === classId);

  if (row === undefined) {
    throw new Error(`missing data custody source-of-truth row: ${classId}`);
  }

  return row;
}
