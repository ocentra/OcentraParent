/* generated from crates/schema/src/report_query_custody.rs */

import {
  GeneratedReportQueryCustodyNonClaims,
  GeneratedReportQueryCustodySourceDataClasses,
  GeneratedReportQueryCustodyStates,
  type GeneratedReportQueryCustodyContractProof,
  type GeneratedReportQueryCustodyRequest,
  type GeneratedReportQueryCustodyRow,
} from './generated-report-query-custody-contracts';

const allowedSourceDataClassSet = new Set<string>(GeneratedReportQueryCustodySourceDataClasses);

export function reportQueryCustodyRequestIsHonestGenerated(request: GeneratedReportQueryCustodyRequest): boolean {
  return (
    request.parentAuthorized &&
    request.parentOwnedSourceRequired &&
    !request.rawChildEvidenceRequested &&
    request.pageSize > 0 &&
    request.requestedDataClasses.length > 0 &&
    request.allowedSourceDataClasses.length > 0 &&
    request.sourceCitationRefs.length > 0 &&
    request.assistantCitationRefs.length > 0 &&
    request.notificationPayloadBoundary === 'parent-owned-citations-only' &&
    request.requestedDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    request.allowedSourceDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    request.sourceCitationRefs.every((citation) => citation.kind === 'query-store-summary') &&
    request.assistantCitationRefs.every((citation) => citation.kind === 'query-store-summary')
  );
}

export function reportQueryCustodyRowIsHonestGenerated(row: GeneratedReportQueryCustodyRow): boolean {
  return (
    row.claimSafe &&
    !row.secondTruthStoreClaimed &&
    !row.reportCacheMutated &&
    !row.rawChildEvidenceIncluded &&
    row.parentAuthorized &&
    row.parentOwnedSourceRequired &&
    row.pageSize > 0 &&
    row.pageIndex > 0 &&
    allowedSourceDataClassSet.has(row.sourceDataClass) &&
    row.requestedDataClasses.length > 0 &&
    row.allowedSourceDataClasses.length > 0 &&
    row.sourceCitationRefs.length > 0 &&
    row.assistantCitationRefs.length > 0 &&
    row.notificationPayloadBoundary === 'parent-owned-citations-only' &&
    row.requestedDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    row.allowedSourceDataClasses.every((dataClass) => allowedSourceDataClassSet.has(dataClass)) &&
    row.sourceCitationRefs.every((citation) => citation.kind === 'query-store-summary') &&
    row.assistantCitationRefs.every((citation) => citation.kind === 'query-store-summary') &&
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
    proof.rows.every((row) => row.pageSize === proof.request.pageSize) &&
    proof.rows.every((row) => row.notificationPayloadBoundary === proof.request.notificationPayloadBoundary) &&
    proof.rows.every((row, index) => row.pageIndex === index + 1) &&
    new Set(proof.rows.map((row) => row.cursorRef)).size === proof.rows.length
  );
}
