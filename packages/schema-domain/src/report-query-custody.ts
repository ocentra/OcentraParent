/* thin edge adapter over Rust-owned report/query custody contracts */

import { type Infer, Schema, withParser } from './effect';
import {
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import {
  ParentAccountIdSchema,
  ParentContractSchemaVersionSchema,
  ChildProfileIdSchema,
  FamilyIdSchema,
  ParentDeviceIdSchema,
  ParentEvidenceReferenceIdSchema,
  ParentTimestampSchema,
  type ChildProfileId as ChildProfileIdType,
  type FamilyId as FamilyIdType,
  type ParentAccountId as ParentAccountIdType,
  type ParentActionReferenceId as ParentActionReferenceIdType,
  type ParentActorId as ParentActorIdType,
  type ParentDeviceId as ParentDeviceIdType,
  type ParentDeviceLabel as ParentDeviceLabelType,
  type ParentEvidenceReferenceId as ParentEvidenceReferenceIdType,
  type ParentPolicyVersion as ParentPolicyVersionType,
  type ParentTimestamp as ParentTimestampType,
} from './family-reference-primitives';
import * as Generated from './generated-report-query-custody-contracts';
import {
  reportQueryCustodyProofIsHonestGenerated,
  reportQueryCustodyRequestIsHonestGenerated,
  reportQueryCustodyRowIsHonestGenerated,
} from './generated-report-query-custody-contract-rules';

export const ReportQueryCustodySchemaVersionSchema = withParser(
  Schema.Literal(Generated.ReportQueryCustodyContractRuntime.SchemaVersion)
);

export const RequiredReportQueryCustodyStates = [...Generated.GeneratedReportQueryCustodyStates] as const;
export const RequiredReportQueryCustodyNonClaims = [...Generated.GeneratedReportQueryCustodyNonClaims] as const;
export const ReportQueryCustodyKnownGaps = [...Generated.GeneratedReportQueryCustodyKnownGaps] as const;

export const ReportQueryCustodyBoundarySchema = withParser(
  Schema.Literal(...Generated.GeneratedReportQueryCustodyBoundaries)
);
export const ReportQueryCustodyPayloadRedactionSchema = withParser(
  Schema.Literal(...Generated.GeneratedReportQueryCustodyPayloadRedactionStates)
);
export const ReportQueryCustodySourceFreshnessSchema = withParser(
  Schema.Literal(...Generated.GeneratedReportQueryCustodySourceFreshnessStates)
);
export const ReportQueryCustodyStateSchema = withParser(Schema.Literal(...Generated.GeneratedReportQueryCustodyStates));
export const ReportQueryCustodySourceDataClassSchema = withParser(
  Schema.Literal(...Generated.GeneratedReportQueryCustodySourceDataClasses)
);
export const ReportQueryCustodyTombstoneStateSchema = withParser(
  Schema.Literal(...Generated.GeneratedReportQueryCustodyTombstoneStates)
);

export const ReportQueryCustodyRequestIdSchema = Generated.GeneratedReportQueryCustodyRequestIdSchema;
export const ReportQueryCustodyQueryCursorSchema = Generated.GeneratedReportQueryCustodyQueryCursorSchema;
export const ReportQueryCustodyCursorRefSchema = Generated.GeneratedReportQueryCustodyCursorRefSchema;
export const ReportQueryCustodySortKeySchema = Generated.GeneratedReportQueryCustodySortKeySchema;
export const ReportQueryCustodySourceRefSchema = Generated.GeneratedReportQueryCustodySourceRefSchema;
export const ReportQueryCustodyConflictRefSchema = Generated.GeneratedReportQueryCustodyConflictRefSchema;
export const ReportQueryCustodyDeletedSourceRefSchema = Generated.GeneratedReportQueryCustodyDeletedSourceRefSchema;
export const ReportQueryCustodyParentAuthorityReferenceIdSchema =
  Generated.GeneratedParentAuthorityReferenceIdSchema;
const ReportQueryCustodyPositiveCountSchema = Schema.Number.pipe(Schema.int(), Schema.positive());

export type ParentAccountId = ParentAccountIdType;
export type FamilyId = FamilyIdType;
export type ChildProfileId = ChildProfileIdType;
export type ParentDeviceId = ParentDeviceIdType;
export type ParentDeviceLabel = ParentDeviceLabelType;
export type ParentActorId = ParentActorIdType;
export type ParentPolicyVersion = ParentPolicyVersionType;
export type ParentEvidenceReferenceId = ParentEvidenceReferenceIdType;
export type ParentActionReferenceId = ParentActionReferenceIdType;
export type ParentTimestamp = ParentTimestampType;
export type ReportQueryCustodyRequestId = Generated.GeneratedReportQueryCustodyRequestId;
export type ReportQueryCustodyQueryCursor = Generated.GeneratedReportQueryCustodyQueryCursor;
export type ReportQueryCustodyCursorRef = Generated.GeneratedReportQueryCustodyCursorRef;
export type ReportQueryCustodySortKey = Generated.GeneratedReportQueryCustodySortKey;
export type ReportQueryCustodySourceRef = Generated.GeneratedReportQueryCustodySourceRef;
export type ReportQueryCustodyConflictRef = Generated.GeneratedReportQueryCustodyConflictRef;
export type ReportQueryCustodyDeletedSourceRef = Generated.GeneratedReportQueryCustodyDeletedSourceRef;

const ReportQueryCustodyParentAuthorityReferenceSchema = Schema.Struct({
  authorityReferenceId: ReportQueryCustodyParentAuthorityReferenceIdSchema,
  familyId: FamilyIdSchema,
  parentAccountId: ParentAccountIdSchema,
  deviceId: ParentDeviceIdSchema,
  childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
  authorityGeneration: ReportQueryCustodyPositiveCountSchema,
});

const ReportQueryCustodyAllowedCitationRefSchema = Schema.Struct({
  evidenceReferenceId: ParentEvidenceReferenceIdSchema,
  kind: Schema.Literal('query-store-summary'),
  observedAt: ParentTimestampSchema,
  familyId: FamilyIdSchema,
  childProfileId: Schema.Union(ChildProfileIdSchema, Schema.Null),
  sourceDataClass: ReportQueryCustodySourceDataClassSchema,
  sourceReference: ReportQueryCustodySourceRefSchema,
});

const ReportQueryCustodyRequestBaseSchema = Schema.Struct({
  schemaVersion: ReportQueryCustodySchemaVersionSchema,
  requestId: ReportQueryCustodyRequestIdSchema,
  family: FamilyReferenceSchema,
  account: ParentAccountReferenceSchema,
  device: ParentDeviceReferenceSchema,
  parentAction: ParentActionReferenceSchema,
  requestedCursor: ReportQueryCustodyQueryCursorSchema,
  pageSize: ReportQueryCustodyPositiveCountSchema,
  requestedDataClasses: Schema.Array(ReportQueryCustodySourceDataClassSchema),
  allowedSourceDataClasses: Schema.Array(ReportQueryCustodySourceDataClassSchema),
  sourceCitationRefs: Schema.Array(ReportQueryCustodyAllowedCitationRefSchema),
  assistantCitationRefs: Schema.Array(ReportQueryCustodyAllowedCitationRefSchema),
  notificationPayloadBoundary: ReportQueryCustodyBoundarySchema,
  parentAuthority: ReportQueryCustodyParentAuthorityReferenceSchema,
  rawChildEvidenceRequested: Schema.Literal(false),
});

export const ReportQueryCustodyRequestSchema = withParser(
  ReportQueryCustodyRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        reportQueryCustodyRequestIsHonestGenerated(request as Generated.GeneratedReportQueryCustodyRequest) ||
        'Expected report and query requests to stay parent-authorized, citation-bound, and free of raw child evidence'
    )
  )
);

const ReportQueryCustodyRowBaseSchema = Schema.Struct({
  rowId: ReportQueryCustodySourceRefSchema,
  requestId: ReportQueryCustodyRequestIdSchema,
  state: ReportQueryCustodyStateSchema,
  sourceFreshness: ReportQueryCustodySourceFreshnessSchema,
  sourceDataClass: ReportQueryCustodySourceDataClassSchema,
  cursorRef: ReportQueryCustodyCursorRefSchema,
  sourceCursorRef: ReportQueryCustodyCursorRefSchema,
  nextCursorRef: Schema.Union(ReportQueryCustodyCursorRefSchema, Schema.Null),
  pageIndex: ReportQueryCustodyPositiveCountSchema,
  pageSize: ReportQueryCustodyPositiveCountSchema,
  stableSortKey: ReportQueryCustodySortKeySchema,
  requestedDataClasses: Schema.Array(ReportQueryCustodySourceDataClassSchema),
  allowedSourceDataClasses: Schema.Array(ReportQueryCustodySourceDataClassSchema),
  sourceCitationRefs: Schema.Array(ReportQueryCustodyAllowedCitationRefSchema),
  assistantCitationRefs: Schema.Array(ReportQueryCustodyAllowedCitationRefSchema),
  notificationPayloadBoundary: ReportQueryCustodyBoundarySchema,
  payloadRedactionState: ReportQueryCustodyPayloadRedactionSchema,
  tombstoneState: ReportQueryCustodyTombstoneStateSchema,
  deletedSourceRef: Schema.Union(ReportQueryCustodyDeletedSourceRefSchema, Schema.Null),
  deletedSourceAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  conflictRef: Schema.Union(ReportQueryCustodyConflictRefSchema, Schema.Null),
  cursorExpiredAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  rateLimitedUntilAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  parentAuthority: ReportQueryCustodyParentAuthorityReferenceSchema,
  rawChildEvidenceIncluded: Schema.Literal(false),
  reportCacheMutated: Schema.Boolean,
  secondTruthStoreClaimed: Schema.Boolean,
  claimSafe: Schema.Boolean,
});

export const ReportQueryCustodyRowSchema = withParser(
  ReportQueryCustodyRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportQueryCustodyRowIsHonestGenerated(row as Generated.GeneratedReportQueryCustodyRow) ||
        'Expected report and query rows to stay derived, redacted, pagination-stable, tombstone-aware, and claim-safe'
    )
  )
);

const ReportQueryCustodyProofBaseSchema = Schema.Struct({
  schemaVersion: ReportQueryCustodySchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  request: ReportQueryCustodyRequestSchema,
  rows: Schema.Array(ReportQueryCustodyRowSchema),
  nonClaims: Schema.Array(withParser(Schema.Literal(...Generated.GeneratedReportQueryCustodyNonClaims))),
  reportRuntimeClaimed: Schema.Literal(false),
  portalUiClaimed: Schema.Literal(false),
  providerRoutingClaimed: Schema.Literal(false),
  ocentraHostedFamilyDataCustodyClaimed: Schema.Literal(false),
  secondTruthStoreClaimed: Schema.Literal(false),
  rawChildEvidenceClaimed: Schema.Literal(false),
  updatedAt: ParentTimestampSchema,
});

export const ReportQueryCustodyProofSchema = withParser(
  ReportQueryCustodyProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        reportQueryCustodyProofIsHonestGenerated(proof as Generated.GeneratedReportQueryCustodyContractProof) ||
        'Expected report and query custody proof to cover all required states, redaction, pagination, tombstones, and claim-safe boundaries'
    )
  )
);

export type ReportQueryCustodyState = Generated.GeneratedReportQueryCustodyState;
export type ReportQueryCustodyNonClaim = Generated.GeneratedReportQueryCustodyNonClaim;
export type ReportQueryCustodyRequest = Infer<typeof ReportQueryCustodyRequestSchema>;
export type ReportQueryCustodyRow = Infer<typeof ReportQueryCustodyRowSchema>;
export type ReportQueryCustodyProof = Infer<typeof ReportQueryCustodyProofSchema>;

export const ReportQueryCustodyProofReadModel = ReportQueryCustodyProofSchema.parse(
  Generated.GeneratedReportQueryCustodyContractProof
);

export function summarizeReportQueryCustodyStates(
  rows: ReadonlyArray<ReportQueryCustodyRow>
): Record<ReportQueryCustodyState, number> {
  const counts = Object.fromEntries(RequiredReportQueryCustodyStates.map((state) => [state, 0])) as Record<
    ReportQueryCustodyState,
    number
  >;
  for (const row of rows) {
    counts[row.state] += 1;
  }
  return counts;
}
