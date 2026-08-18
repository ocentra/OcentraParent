/* generated from crates/schema/src/report_query_custody.rs */

import {
  GeneratedReportQueryCustodyMaxPageSize,
  GeneratedReportQueryCustodyNonClaims,
  GeneratedReportQueryCustodySourceDataClasses,
  GeneratedReportQueryCustodyStates,
  type GeneratedParentEvidenceReference,
  type GeneratedReportQueryCustodyContractProof,
  type GeneratedReportQueryCustodyRequest,
  type GeneratedReportQueryCustodyRow,
} from './generated-report-query-custody-contracts';

const allowedSourceDataClassSet = new Set<string>(GeneratedReportQueryCustodySourceDataClasses);

function reportQueryCustodyAuthorityReferenceIsConsistentGenerated(
  request: GeneratedReportQueryCustodyRequest
): boolean {
  const authority = request.parentAuthority;
  return (
    authority.authorityGeneration > 0 &&
    authority.familyId === request.family.familyId &&
    authority.parentAccountId === request.account.parentAccountId &&
    authority.deviceId === request.device.deviceId &&
    authority.childProfileId === request.device.childProfileId
  );
}

function reportQueryCustodyCitationIsBoundGenerated(
  citation: GeneratedParentEvidenceReference,
  request: GeneratedReportQueryCustodyRequest
): boolean {
  return (
    citation.familyId === request.family.familyId &&
    citation.childProfileId === request.device.childProfileId &&
    request.requestedDataClasses.includes(citation.sourceDataClass) &&
    request.allowedSourceDataClasses.includes(citation.sourceDataClass)
  );
}

export function reportQueryCustodyRequestIsHonestGenerated(request: GeneratedReportQueryCustodyRequest): boolean {
  return (
    reportQueryCustodyAuthorityReferenceIsConsistentGenerated(request) &&
    !request.rawChildEvidenceRequested &&
    request.pageSize > 0 &&
    request.pageSize <= GeneratedReportQueryCustodyMaxPageSize &&
    request.requestedDataClasses.length > 0 &&
    request.allowedSourceDataClasses.length > 0 &&
    request.sourceCitationRefs.length > 0 &&
    request.assistantCitationRefs.length > 0 &&
    request.notificationPayloadBoundary === 'parent-owned-citations-only' &&
    request.requestedDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    request.allowedSourceDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    request.sourceCitationRefs.every(
      (citation) => citation.kind === 'query-store-summary' && reportQueryCustodyCitationIsBoundGenerated(citation, request)
    ) &&
    request.assistantCitationRefs.every(
      (citation) => citation.kind === 'query-store-summary' && reportQueryCustodyCitationIsBoundGenerated(citation, request)
    )
  );
}

export function reportQueryCustodyRowIsHonestGenerated(row: GeneratedReportQueryCustodyRow): boolean {
  return (
    row.claimSafe &&
    !row.secondTruthStoreClaimed &&
    !row.reportCacheMutated &&
    !row.rawChildEvidenceIncluded &&
    row.parentAuthority.authorityGeneration > 0 &&
    row.pageSize > 0 &&
    row.pageSize <= GeneratedReportQueryCustodyMaxPageSize &&
    row.pageIndex > 0 &&
    allowedSourceDataClassSet.has(row.sourceDataClass) &&
    row.requestedDataClasses.length > 0 &&
    row.allowedSourceDataClasses.length > 0 &&
    row.sourceCitationRefs.length > 0 &&
    row.assistantCitationRefs.length > 0 &&
    row.notificationPayloadBoundary === 'parent-owned-citations-only' &&
    row.requestedDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    row.allowedSourceDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    row.sourceCitationRefs.every(
      (citation) =>
        citation.kind === 'query-store-summary' &&
        citation.familyId === row.parentAuthority.familyId &&
        citation.childProfileId === row.parentAuthority.childProfileId &&
        row.requestedDataClasses.includes(citation.sourceDataClass) &&
        row.allowedSourceDataClasses.includes(citation.sourceDataClass)
    ) &&
    row.assistantCitationRefs.every(
      (citation) =>
        citation.kind === 'query-store-summary' &&
        citation.familyId === row.parentAuthority.familyId &&
        citation.childProfileId === row.parentAuthority.childProfileId &&
        row.requestedDataClasses.includes(citation.sourceDataClass) &&
        row.allowedSourceDataClasses.includes(citation.sourceDataClass)
    ) &&
    reportQueryCustodyStateIsCoherentGenerated(row)
  );
}
function reportQueryCustodyStateHasExpectedShapeGenerated(
  row: GeneratedReportQueryCustodyRow,
  expectation: {
    readonly sourceFreshness: GeneratedReportQueryCustodyRow['sourceFreshness'];
    readonly payloadRedactionState: GeneratedReportQueryCustodyRow['payloadRedactionState'];
    readonly tombstoneState: GeneratedReportQueryCustodyRow['tombstoneState'];
    readonly nextCursorRef: boolean;
    readonly deletedSourceRef: boolean;
    readonly deletedSourceAt: boolean;
    readonly conflictRef: boolean;
    readonly cursorExpiredAt: boolean;
    readonly rateLimitedUntilAt: boolean;
  }
): boolean {
  return (
    row.sourceFreshness === expectation.sourceFreshness &&
    row.payloadRedactionState === expectation.payloadRedactionState &&
    row.tombstoneState === expectation.tombstoneState &&
    (row.nextCursorRef !== null) === expectation.nextCursorRef &&
    (row.deletedSourceRef !== null) === expectation.deletedSourceRef &&
    (row.deletedSourceAt !== null) === expectation.deletedSourceAt &&
    (row.conflictRef !== null) === expectation.conflictRef &&
    (row.cursorExpiredAt !== null) === expectation.cursorExpiredAt &&
    (row.rateLimitedUntilAt !== null) === expectation.rateLimitedUntilAt
  );
}

const reportQueryCustodyStateExpectations = {
  derivedFresh: {
    sourceFreshness: 'fresh',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: true,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: false,
    cursorExpiredAt: false,
    rateLimitedUntilAt: false,
  },
  derivedStale: {
    sourceFreshness: 'stale',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: true,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: false,
    cursorExpiredAt: false,
    rateLimitedUntilAt: false,
  },
  partiallyRedacted: {
    sourceFreshness: 'stale',
    payloadRedactionState: 'partially-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: true,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: false,
    cursorExpiredAt: false,
    rateLimitedUntilAt: false,
  },
  deletedSource: {
    sourceFreshness: 'deleted',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'written',
    nextCursorRef: false,
    deletedSourceRef: true,
    deletedSourceAt: true,
    conflictRef: false,
    cursorExpiredAt: false,
    rateLimitedUntilAt: false,
  },
  syncConflict: {
    sourceFreshness: 'conflicted',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: true,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: true,
    cursorExpiredAt: false,
    rateLimitedUntilAt: false,
  },
  cursorExpired: {
    sourceFreshness: 'expired',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: false,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: false,
    cursorExpiredAt: true,
    rateLimitedUntilAt: false,
  },
  rateLimited: {
    sourceFreshness: 'rate-limited',
    payloadRedactionState: 'fully-redacted',
    tombstoneState: 'not-required',
    nextCursorRef: false,
    deletedSourceRef: false,
    deletedSourceAt: false,
    conflictRef: false,
    cursorExpiredAt: false,
    rateLimitedUntilAt: true,
  },
} as const;

export function reportQueryCustodyStateIsCoherentGenerated(row: GeneratedReportQueryCustodyRow): boolean {
  return reportQueryCustodyStateHasExpectedShapeGenerated(row, reportQueryCustodyStateExpectations[row.state]);
}
function reportQueryCustodyPaginationIsContinuousGenerated(
  proof: GeneratedReportQueryCustodyContractProof
): boolean {
  const rows = proof.rows;
  const first = rows[0];
  return (
    first !== undefined &&
    first.cursorRef.toString() === proof.request.requestedCursor.toString() &&
    new Set(rows.map((row) => row.rowId)).size === rows.length &&
    new Set(rows.map((row) => row.cursorRef)).size === rows.length &&
    new Set(rows.map((row) => row.sourceCursorRef)).size === 1 &&
    new Set(rows.map((row) => row.stableSortKey)).size === rows.length &&
    rows.every((row, index) => row.pageIndex === index + 1) &&
    rows.every((row, index) => {
      if (row.nextCursorRef === null) {
        return true;
      }
      if (index < rows.length - 1) {
        const next = rows[index + 1];
        return next !== undefined && row.nextCursorRef === next.cursorRef;
      }
      return !rows
        .slice(0, index + 1)
        .some((seen) => seen.cursorRef === row.nextCursorRef);
    }) &&
    rows.every((row, index) => {
      if (index === 0) {
        return true;
      }
      const previous = rows[index - 1];
      return previous !== undefined && previous.sourceCursorRef === row.sourceCursorRef;
    }) &&
    rows.every((row, index) => {
      if (index === 0) {
        return true;
      }
      const previous = rows[index - 1];
      return (
        previous !== undefined &&
        previous.stableSortKey.toString() < row.stableSortKey.toString()
      );
    })
  );
}

export function reportQueryCustodyProofIsHonestGenerated(proof: GeneratedReportQueryCustodyContractProof): boolean {
  return (
    GeneratedReportQueryCustodyStates.every((state) => proof.rows.some((row) => row.state === state)) &&
    GeneratedReportQueryCustodyNonClaims.every((nonClaim) => proof.nonClaims.includes(nonClaim)) &&
    reportQueryCustodyRequestIsHonestGenerated(proof.request) &&
    proof.reportRuntimeClaimed === false &&
    proof.portalUiClaimed === false &&
    proof.providerRoutingClaimed === false &&
    proof.ocentraHostedFamilyDataCustodyClaimed === false &&
    proof.secondTruthStoreClaimed === false &&
    proof.rawChildEvidenceClaimed === false &&
    proof.rows.every((row) => reportQueryCustodyRowIsHonestGenerated(row)) &&
    proof.rows.every((row) => row.requestId === proof.request.requestId) &&
    proof.rows.every(
      (row) =>
        row.parentAuthority.authorityReferenceId ===
          proof.request.parentAuthority.authorityReferenceId &&
        row.parentAuthority.familyId === proof.request.family.familyId &&
        row.parentAuthority.parentAccountId === proof.request.account.parentAccountId &&
        row.parentAuthority.deviceId === proof.request.device.deviceId &&
        row.parentAuthority.childProfileId === proof.request.device.childProfileId
    ) &&
    proof.rows.every((row) => row.pageSize === proof.request.pageSize) &&
    proof.rows.every((row) => row.notificationPayloadBoundary === proof.request.notificationPayloadBoundary) &&
    reportQueryCustodyPaginationIsContinuousGenerated(proof)
  );
}
