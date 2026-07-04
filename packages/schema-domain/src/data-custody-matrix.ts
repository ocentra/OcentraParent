/* thin adapter over Rust-generated data custody source-of-truth contracts */

import { ParentContractSchemaVersionSchema } from './family-reference-primitives';
import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { countProductionProofValues } from './proof-shape';
import {
  DataCustodyAuthoritySchema,
  DataCustodyClassId,
  DataCustodyClassIdSchema,
  DataCustodyDefaultLocationSchema,
  DataCustodyExposureSchema,
  DataCustodyHostingPolicySchema,
  DataCustodyOcentraHostingMode,
  DataCustodySourceOfTruthSchema,
  SeededDataCustodyClassIds,
} from './custody-boundary';
import {
  DataCustodySourceOfTruthContractRuntime,
  GeneratedDataCustodyAuthorities,
  GeneratedDataCustodyClassIds,
  GeneratedDataCustodyKnownGaps,
  GeneratedDataCustodyNonClaims,
  GeneratedDataCustodyOcentraHostingModes,
  GeneratedDataCustodySourceOfTruthContractProof,
  type GeneratedDataCustodyClassId,
  type GeneratedDataCustodyNonClaim,
  type GeneratedDataCustodySourceOfTruthContractProof as GeneratedDataCustodySourceOfTruthContractProofShape,
} from './generated/data-custody-source-of-truth-contracts';

export const DataCustodySourceOfTruthSchemaVersionSchema = withParser(
  Schema.Literal(DataCustodySourceOfTruthContractRuntime.SchemaVersion)
);
export const DataCustodySourceOfTruthMatrixIdSchema = brandedNonEmptyStringSchema('DataCustodySourceOfTruthMatrixId');
export const DataCustodySourceOfTruthMatrixRowIdSchema = brandedNonEmptyStringSchema(
  'DataCustodySourceOfTruthMatrixRowId'
);

export const RequiredDataCustodyClassIds = [...GeneratedDataCustodyClassIds] as const;
export const RequiredDataCustodyNonClaims = [...GeneratedDataCustodyNonClaims] as const;
export const HostedOcentraMetadataClassIds = [
  ...GeneratedDataCustodySourceOfTruthContractProof.allowedOcentraHostedMetadata,
] as const;
export const MustNeverBeHostedByDefaultClassIds = [
  ...GeneratedDataCustodySourceOfTruthContractProof.mustNeverBeHostedByDefault,
] as const;
export const DataCustodyKnownGaps = [...GeneratedDataCustodyKnownGaps] as const;
export const DataCustodyClaimSafeLanguage = [
  ...GeneratedDataCustodySourceOfTruthContractProof.claimSafeLanguage,
] as const;

type DataCustodyClassIdValue = Infer<typeof DataCustodyClassIdSchema>;
type DataCustodySourceOfTruthMatrixRowCandidate = Infer<typeof DataCustodySourceOfTruthMatrixRowBaseSchema>;

const AllowedHostedMetadataClassIdSet: ReadonlySet<DataCustodyClassIdValue> = new Set(HostedOcentraMetadataClassIds);
const MustNeverHostedByDefaultClassIdSet: ReadonlySet<DataCustodyClassIdValue> = new Set(
  MustNeverBeHostedByDefaultClassIds
);
const RawChildEvidenceClassIdSet: ReadonlySet<DataCustodyClassIdValue> = new Set([
  DataCustodyClassId.EvidenceJournalSegments,
  DataCustodyClassId.ScreenshotsAndScreenAnalysisImages,
  DataCustodyClassId.BrowserUrlHistory,
  DataCustodyClassId.NetworkAppGameEvidence,
  DataCustodyClassId.LocationTrackingEvidence,
  DataCustodyClassId.SupportBundlesContainingRawChildActivity,
]);
const DerivedOnlyClassIdSet: ReadonlySet<DataCustodyClassIdValue> = new Set(
  GeneratedDataCustodySourceOfTruthContractProof.rows
    .filter((row) => row.sourceOfTruth.kind === 'derived-from-data-classes')
    .map((row) => row.classId)
);
const EncryptionOptionalClassIdSet: ReadonlySet<DataCustodyClassIdValue> = new Set([
  DataCustodyClassId.LicenseDownloadUpdateMetadata,
  DataCustodyClassId.ShortLivedReportCompilerStatus,
  DataCustodyClassId.PublicWebsiteReleaseStatus,
  DataCustodyClassId.UniversalDecryptKeys,
]);

const DataCustodySourceOfTruthMatrixRowBaseSchema = Schema.Struct({
  rowId: DataCustodySourceOfTruthMatrixRowIdSchema,
  classId: DataCustodyClassIdSchema,
  classLabel: brandedNonEmptyStringSchema('DataCustodyClassLabel'),
  sourceOwner: brandedNonEmptyStringSchema('DataCustodySourceOwner'),
  sourceOfTruth: DataCustodySourceOfTruthSchema,
  custodyAuthority: DataCustodyAuthoritySchema,
  defaultLocation: DataCustodyDefaultLocationSchema,
  ocentraHostedByDefault: Schema.Boolean,
  mustNeverBeHostedByDefault: Schema.Boolean,
  encryptedBeforeUpload: Schema.Boolean,
  mayAppearInReports: Schema.Boolean,
  mayAppearInNotifications: Schema.Boolean,
  reportExposure: DataCustodyExposureSchema,
  notificationExposure: DataCustodyExposureSchema,
  rawChildEvidenceAllowed: Schema.Boolean,
  derivedUseOnly: Schema.Boolean,
  sensitive: Schema.Boolean,
  hostingPolicy: DataCustodyHostingPolicySchema,
  notes: brandedNonEmptyStringSchema('DataCustodyNotes'),
});

export const DataCustodySourceOfTruthMatrixRowSchema = withParser(
  DataCustodySourceOfTruthMatrixRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        dataCustodyHostedDefaultMatchesAllowList(row) ||
        'Expected only explicitly allowed metadata classes to be Ocentra-hosted by default'
    ),
    Schema.filter(
      (row) =>
        dataCustodyMustNeverHostedBoundaryIsHonest(row) ||
        'Expected must-never-host-by-default classes to stay non-hosted and claim-safe'
    ),
    Schema.filter(
      (row) =>
        dataCustodyDerivedSourceIsHonest(row) ||
        'Expected derived rows to cite one or more source classes and self rows to cite none'
    ),
    Schema.filter(
      (row) =>
        dataCustodyExposureIsClaimSafe(row) ||
        'Expected report and notification exposure to stay redacted, reference-only, or absent for sensitive classes'
    ),
    Schema.filter(
      (row) =>
        dataCustodyEncryptionBoundaryIsHonest(row) ||
        'Expected sensitive and cross-boundary custody rows to stay encrypted before upload unless explicitly public/status only'
    )
  )
);

const DataCustodySourceOfTruthMatrixBaseSchema = Schema.Struct({
  schemaVersion: DataCustodySourceOfTruthSchemaVersionSchema,
  matrixId: DataCustodySourceOfTruthMatrixIdSchema,
  rows: Schema.Array(DataCustodySourceOfTruthMatrixRowSchema),
});

export const DataCustodySourceOfTruthMatrixSchema = withParser(
  DataCustodySourceOfTruthMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        new Set(matrix.rows.map((row) => row.rowId)).size === matrix.rows.length ||
        'Expected data custody matrix row ids to be unique'
    ),
    Schema.filter(
      (matrix) =>
        new Set(matrix.rows.map((row) => row.classId)).size === SeededDataCustodyClassIds.length &&
        matrix.rows.length === SeededDataCustodyClassIds.length ||
        'Expected exactly one matrix row for each generated data custody class'
    )
  )
);

const DataCustodySourceOfTruthContractProofBaseSchema = Schema.Struct({
  schemaVersion: DataCustodySourceOfTruthSchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  matrixId: DataCustodySourceOfTruthMatrixIdSchema,
  rows: Schema.Array(DataCustodySourceOfTruthMatrixRowSchema),
  allowedOcentraHostedMetadata: Schema.Array(DataCustodyClassIdSchema),
  mustNeverBeHostedByDefault: Schema.Array(DataCustodyClassIdSchema),
  claimSafeLanguage: Schema.Array(brandedNonEmptyStringSchema('DataCustodyClaimSafeLanguage')),
  nonClaims: Schema.Array(withParser(Schema.Literal(...GeneratedDataCustodyNonClaims))),
  accountControlPlaneSeparated: Schema.Boolean,
  providerOwnedBillingIdentitySeparated: Schema.Boolean,
  ocentraIsDefaultChildDataStore: Schema.Boolean,
  providerAutoApplyClaimed: Schema.Boolean,
  supportDecryptByDefaultClaimed: Schema.Boolean,
  sqliteAsTruthLayerClaimed: Schema.Boolean,
  rawChildActivityHostedByDefaultClaimed: Schema.Boolean,
  updatedAt: brandedNonEmptyStringSchema('DataCustodyUpdatedAt'),
});

export const DataCustodySourceOfTruthContractProofSchema = withParser(
  DataCustodySourceOfTruthContractProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        dataCustodyProofCoversEveryClass(proof) ||
        'Expected the custody source-of-truth proof to cover every generated data class exactly once'
    ),
    Schema.filter(
      (proof) =>
        dataCustodyHostedMetadataProofIsHonest(proof) ||
        'Expected hosted metadata and must-never-host sets to match the row-level custody matrix'
    ),
    Schema.filter(
      (proof) =>
        dataCustodyNonClaimsStayExplicit(proof) ||
        'Expected all custody non-claims to remain explicit and false claim flags to stay false'
    ),
    Schema.filter(
      (proof) =>
        proof.accountControlPlaneSeparated &&
        proof.providerOwnedBillingIdentitySeparated &&
        proof.claimSafeLanguage.length >= 5 ||
        'Expected account/provider separation and claim-safe language to remain explicit'
    )
  )
);

export type DataCustodySourceOfTruthMatrixId = Infer<typeof DataCustodySourceOfTruthMatrixIdSchema>;
export type DataCustodySourceOfTruthMatrixRowId = Infer<typeof DataCustodySourceOfTruthMatrixRowIdSchema>;
export type DataCustodySourceOfTruthMatrixRow = Infer<typeof DataCustodySourceOfTruthMatrixRowSchema>;
export type DataCustodySourceOfTruthMatrix = Infer<typeof DataCustodySourceOfTruthMatrixSchema>;
export type DataCustodySourceOfTruthContractProof = Infer<typeof DataCustodySourceOfTruthContractProofSchema>;
export type DataCustodyNonClaim = GeneratedDataCustodyNonClaim;

export const DataCustodySourceOfTruthProofReadModel = DataCustodySourceOfTruthContractProofSchema.parse(
  GeneratedDataCustodySourceOfTruthContractProof
);

export const CanonicalDataCustodySourceOfTruthMatrix = DataCustodySourceOfTruthMatrixSchema.parse({
  schemaVersion: DataCustodySourceOfTruthProofReadModel.schemaVersion,
  matrixId: DataCustodySourceOfTruthProofReadModel.matrixId,
  rows: DataCustodySourceOfTruthProofReadModel.rows,
});

export function parseDataCustodySourceOfTruthMatrix(input: unknown): DataCustodySourceOfTruthMatrix {
  return DataCustodySourceOfTruthMatrixSchema.parse(input);
}

export function parseDataCustodySourceOfTruthContractProof(
  input: unknown
): DataCustodySourceOfTruthContractProof {
  return DataCustodySourceOfTruthContractProofSchema.parse(input);
}

export function getDataCustodySourceOfTruthMatrixRow(
  classId: DataCustodyClassIdValue
): DataCustodySourceOfTruthMatrixRow {
  const row = CanonicalDataCustodySourceOfTruthMatrix.rows.find((candidate) => candidate.classId === classId);

  if (!row) {
    throw new Error(`Missing canonical data custody row for ${classId}`);
  }

  return row;
}

export function summarizeDataCustodyAuthorities(
  rows: ReadonlyArray<DataCustodySourceOfTruthMatrixRow>
): Record<Infer<typeof DataCustodyAuthoritySchema>, number> {
  return countProductionProofValues(rows.map((row) => row.custodyAuthority), GeneratedDataCustodyAuthorities);
}

export function summarizeDataCustodyOcentraHostingModes(
  rows: ReadonlyArray<DataCustodySourceOfTruthMatrixRow>
): Record<Infer<typeof DataCustodyHostingPolicySchema>['ocentraHostingMode'], number> {
  return countProductionProofValues(
    rows.map((row) => row.hostingPolicy.ocentraHostingMode),
    GeneratedDataCustodyOcentraHostingModes
  );
}

function dataCustodyHostedDefaultMatchesAllowList(row: DataCustodySourceOfTruthMatrixRowCandidate): boolean {
  return AllowedHostedMetadataClassIdSet.has(row.classId) ? row.ocentraHostedByDefault : !row.ocentraHostedByDefault;
}

function dataCustodyMustNeverHostedBoundaryIsHonest(row: DataCustodySourceOfTruthMatrixRowCandidate): boolean {
  if (MustNeverHostedByDefaultClassIdSet.has(row.classId)) {
    return (
      row.mustNeverBeHostedByDefault &&
      !row.ocentraHostedByDefault &&
      row.hostingPolicy.ocentraHostingMode === DataCustodyOcentraHostingMode.Forbidden
    );
  }

  return !row.mustNeverBeHostedByDefault;
}

function dataCustodyDerivedSourceIsHonest(row: DataCustodySourceOfTruthMatrixRowCandidate): boolean {
  if (DerivedOnlyClassIdSet.has(row.classId)) {
    return row.sourceOfTruth.kind === 'derived-from-data-classes' && row.derivedUseOnly;
  }

  return row.sourceOfTruth.kind === 'self' && !row.derivedUseOnly;
}

function dataCustodyExposureIsClaimSafe(row: DataCustodySourceOfTruthMatrixRowCandidate): boolean {
  const reportExposureMatches = row.mayAppearInReports
    ? row.reportExposure !== 'none'
    : row.reportExposure === 'none';
  const notificationExposureMatches = row.mayAppearInNotifications
    ? row.notificationExposure !== 'none'
    : row.notificationExposure === 'none';

  if (!reportExposureMatches || !notificationExposureMatches) {
    return false;
  }

  if (RawChildEvidenceClassIdSet.has(row.classId)) {
    return (
      row.notificationExposure === 'none' &&
      (row.reportExposure === 'none' || row.reportExposure === 'allowed-references-only')
    );
  }

  return !row.rawChildEvidenceAllowed || row.notificationExposure === 'none';
}

function dataCustodyEncryptionBoundaryIsHonest(row: DataCustodySourceOfTruthMatrixRowCandidate): boolean {
  if (EncryptionOptionalClassIdSet.has(row.classId)) {
    return true;
  }

  if (row.ocentraHostedByDefault || row.hostingPolicy.parentOwnedStorageAllowed || row.hostingPolicy.providerMetadataAllowed) {
    return row.encryptedBeforeUpload;
  }

  return !row.sensitive || row.encryptedBeforeUpload;
}

function dataCustodyProofCoversEveryClass(
  proof: GeneratedDataCustodySourceOfTruthContractProofShape
): boolean {
  return (
    proof.rows.length === SeededDataCustodyClassIds.length &&
    new Set(proof.rows.map((row) => row.classId)).size === SeededDataCustodyClassIds.length
  );
}

function dataCustodyHostedMetadataProofIsHonest(
  proof: GeneratedDataCustodySourceOfTruthContractProofShape
): boolean {
  const hostedRows = proof.rows
    .filter((row: GeneratedDataCustodySourceOfTruthContractProofShape['rows'][number]) => row.ocentraHostedByDefault)
    .map((row: GeneratedDataCustodySourceOfTruthContractProofShape['rows'][number]) => row.classId);
  const mustNeverRows = proof.rows
    .filter(
      (row: GeneratedDataCustodySourceOfTruthContractProofShape['rows'][number]) => row.mustNeverBeHostedByDefault
    )
    .map((row: GeneratedDataCustodySourceOfTruthContractProofShape['rows'][number]) => row.classId);

  return (
    arraysMatchAsSets(hostedRows, proof.allowedOcentraHostedMetadata) &&
    arraysMatchAsSets(mustNeverRows, proof.mustNeverBeHostedByDefault)
  );
}

function dataCustodyNonClaimsStayExplicit(
  proof: GeneratedDataCustodySourceOfTruthContractProofShape
): boolean {
  return (
    GeneratedDataCustodyNonClaims.every((nonClaim: GeneratedDataCustodyNonClaim) => proof.nonClaims.includes(nonClaim)) &&
    !proof.ocentraIsDefaultChildDataStore &&
    !proof.providerAutoApplyClaimed &&
    !proof.supportDecryptByDefaultClaimed &&
    !proof.sqliteAsTruthLayerClaimed &&
    !proof.rawChildActivityHostedByDefaultClaimed
  );
}

function arraysMatchAsSets(
  left: ReadonlyArray<GeneratedDataCustodyClassId>,
  right: ReadonlyArray<GeneratedDataCustodyClassId>
): boolean {
  return left.length === right.length && left.every((value) => right.includes(value));
}
