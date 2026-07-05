/* thin adapter over Rust-generated report query custody contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { countProductionProofValues } from './proof-shape';
import {
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActionReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import {
  ParentContractSchemaVersionSchema,
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
import {
  GeneratedReportQueryCustodyContractProof,
  GeneratedReportQueryCustodyKnownGaps,
  GeneratedReportQueryCustodyNonClaims,
  GeneratedReportQueryCustodyPayloadRedactionStates,
  GeneratedReportQueryCustodySourceDataClasses,
  GeneratedReportQueryCustodySourceFreshnessStates,
  GeneratedReportQueryCustodyStates,
  GeneratedReportQueryCustodyTombstoneStates,
  ReportQueryCustodyContractRuntime,
  type GeneratedReportQueryCustodyContractProof as GeneratedReportQueryCustodyContractProofShape,
  type GeneratedReportQueryCustodyRequest,
  type GeneratedReportQueryCustodyRow,
} from './generated/report-query-custody-contracts';
import {
  reportQueryCustodyProofIsHonestGenerated,
  reportQueryCustodyRequestIsHonestGenerated,
  reportQueryCustodyRowIsHonestGenerated,
} from './generated/report-query-custody-contract-rules';

export const ReportQueryCustodySchemaVersionSchema = withParser(
  Schema.Literal(ReportQueryCustodyContractRuntime.SchemaVersion)
);

export const RequiredReportQueryCustodyStates = [...GeneratedReportQueryCustodyStates] as const;
export const RequiredReportQueryCustodyNonClaims = [...GeneratedReportQueryCustodyNonClaims] as const;
export const ReportQueryCustodyKnownGaps = [...GeneratedReportQueryCustodyKnownGaps] as const;

const ReportQueryCustodyBoundarySchema = withParser(Schema.Literal('parent-owned-citations-only'));
const ReportQueryCustodyPayloadRedactionSchema = withParser(
  Schema.Literal(...GeneratedReportQueryCustodyPayloadRedactionStates)
);
const ReportQueryCustodySourceFreshnessSchema = withParser(
  Schema.Literal(...GeneratedReportQueryCustodySourceFreshnessStates)
);
const ReportQueryCustodyStateSchema = withParser(Schema.Literal(...GeneratedReportQueryCustodyStates));
const ReportQueryCustodySourceDataClassSchema = withParser(
  Schema.Literal(...GeneratedReportQueryCustodySourceDataClasses)
);
const ReportQueryCustodyTombstoneStateSchema = withParser(
  Schema.Literal(...GeneratedReportQueryCustodyTombstoneStates)
);

const ReportQueryCustodyRequestIdSchema = brandedNonEmptyStringSchema('ReportQueryCustodyRequestId');
const ReportQueryCustodyQueryCursorSchema = brandedNonEmptyStringSchema('ReportQueryCustodyQueryCursor');
const ReportQueryCustodyCursorRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodyCursorRef');
const ReportQueryCustodySortKeySchema = brandedNonEmptyStringSchema('ReportQueryCustodySortKey');
const ReportQueryCustodySourceRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodySourceRef');
const ReportQueryCustodyConflictRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodyConflictRef');
const ReportQueryCustodyDeletedSourceRefSchema = brandedNonEmptyStringSchema('ReportQueryCustodyDeletedSourceRef');
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
export type ReportQueryCustodyRequestId = typeof ReportQueryCustodyRequestIdSchema.Type;
export type ReportQueryCustodyQueryCursor = typeof ReportQueryCustodyQueryCursorSchema.Type;
export type ReportQueryCustodyCursorRef = typeof ReportQueryCustodyCursorRefSchema.Type;
export type ReportQueryCustodySortKey = typeof ReportQueryCustodySortKeySchema.Type;
export type ReportQueryCustodySourceRef = typeof ReportQueryCustodySourceRefSchema.Type;
export type ReportQueryCustodyConflictRef = typeof ReportQueryCustodyConflictRefSchema.Type;
export type ReportQueryCustodyDeletedSourceRef = typeof ReportQueryCustodyDeletedSourceRefSchema.Type;

const ReportQueryCustodyAllowedCitationRefSchema = withParser(
  ParentEvidenceReferenceSchema.pipe(
    Schema.filter(
      (citation) =>
        citation.kind === 'query-store-summary' ||
        'Expected report and assistant citations to stay within query-store-summary evidence only'
    )
  )
);

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
  parentAuthorized: Schema.Boolean,
  parentOwnedSourceRequired: Schema.Boolean,
  rawChildEvidenceRequested: Schema.Boolean,
});

export const ReportQueryCustodyRequestSchema = withParser(
  ReportQueryCustodyRequestBaseSchema.pipe(
    Schema.filter(
      (request) =>
        reportQueryCustodyRequestIsHonestGenerated(request as GeneratedReportQueryCustodyRequest) ||
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
  parentAuthorized: Schema.Boolean,
  parentOwnedSourceRequired: Schema.Boolean,
  rawChildEvidenceIncluded: Schema.Literal(false),
  reportCacheMutated: Schema.Boolean,
  secondTruthStoreClaimed: Schema.Boolean,
  claimSafe: Schema.Boolean,
});

export const ReportQueryCustodyRowSchema = withParser(
  ReportQueryCustodyRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportQueryCustodyRowIsHonestGenerated(row as GeneratedReportQueryCustodyRow) ||
        'Expected report and query rows to stay derived, redacted, pagination-stable, tombstone-aware, and claim-safe'
    )
  )
);

const ReportQueryCustodyProofBaseSchema = Schema.Struct({
  schemaVersion: ReportQueryCustodySchemaVersionSchema,
  contractVersion: ParentContractSchemaVersionSchema,
  request: ReportQueryCustodyRequestSchema,
  rows: Schema.Array(ReportQueryCustodyRowSchema),
  nonClaims: Schema.Array(withParser(Schema.Literal(...GeneratedReportQueryCustodyNonClaims))),
  reportRuntimeClaimed: Schema.Boolean,
  portalUiClaimed: Schema.Boolean,
  providerRoutingClaimed: Schema.Boolean,
  ocentraHostedFamilyDataCustodyClaimed: Schema.Boolean,
  secondTruthStoreClaimed: Schema.Boolean,
  rawChildEvidenceClaimed: Schema.Boolean,
  updatedAt: ParentTimestampSchema,
});

export const ReportQueryCustodyProofSchema = withParser(
  ReportQueryCustodyProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        reportQueryCustodyProofIsHonestGenerated(proof as GeneratedReportQueryCustodyContractProofShape) ||
        'Expected report and query custody proof to cover all required states, redaction, pagination, tombstones, and claim-safe boundaries'
    )
  )
);

export type ReportQueryCustodyState = Infer<typeof ReportQueryCustodyStateSchema>;
export type ReportQueryCustodyNonClaim = Infer<
  typeof ReportQueryCustodyProofBaseSchema
>['nonClaims'][number];
export type ReportQueryCustodyRequest = Infer<typeof ReportQueryCustodyRequestSchema>;
export type ReportQueryCustodyRow = Infer<typeof ReportQueryCustodyRowSchema>;
export type ReportQueryCustodyProof = Infer<typeof ReportQueryCustodyProofSchema>;

export const ReportQueryCustodyProofReadModel = ReportQueryCustodyProofSchema.parse(
  GeneratedReportQueryCustodyContractProof
);

export function summarizeReportQueryCustodyStates(
  rows: ReadonlyArray<ReportQueryCustodyRow>
): Record<ReportQueryCustodyState, number> {
  return countProductionProofValues(rows.map((row) => row.state), RequiredReportQueryCustodyStates);
}
