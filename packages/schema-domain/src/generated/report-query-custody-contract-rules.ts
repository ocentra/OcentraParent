/* generated from crates/schema/src/report_query_custody.rs */

import {
  GeneratedReportQueryCustodyNonClaims,
  GeneratedReportQueryCustodySourceDataClasses,
  GeneratedReportQueryCustodyStates,
  type GeneratedReportQueryCustodyContractProof,
  type GeneratedReportQueryCustodyRequest,
  type GeneratedReportQueryCustodyRow,
} from './report-query-custody-contracts';

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

export function reportQueryCustodyStateIsCoherentGenerated(row: GeneratedReportQueryCustodyRow): boolean {
  if (row.state === 'derivedFresh') {
    return (
      row.sourceFreshness === 'fresh' &&
      row.payloadRedactionState === 'fully-redacted' &&
      row.tombstoneState === 'not-required' &&
      row.nextCursorRef !== null &&
      row.deletedSourceRef === null &&
      row.deletedSourceAt === null &&
      row.conflictRef === null &&
      row.cursorExpiredAt === null &&
      row.rateLimitedUntilAt === null
    );
  }

  if (row.state === 'derivedStale') {
    return (
      row.sourceFreshness === 'stale' &&
      row.payloadRedactionState === 'fully-redacted' &&
      row.tombstoneState === 'not-required' &&
      row.nextCursorRef !== null &&
      row.deletedSourceRef === null &&
      row.deletedSourceAt === null &&
      row.conflictRef === null &&
      row.cursorExpiredAt === null &&
      row.rateLimitedUntilAt === null
    );
  }

  if (row.state === 'partiallyRedacted') {
    return (
      row.sourceFreshness === 'stale' &&
      row.payloadRedactionState === 'partially-redacted' &&
      row.tombstoneState === 'not-required' &&
      row.nextCursorRef !== null &&
      row.deletedSourceRef === null &&
      row.deletedSourceAt === null &&
      row.conflictRef === null &&
      row.cursorExpiredAt === null &&
      row.rateLimitedUntilAt === null
    );
  }

  if (row.state === 'deletedSource') {
    return (
      row.sourceFreshness === 'deleted' &&
      row.payloadRedactionState === 'fully-redacted' &&
      row.tombstoneState === 'written' &&
      row.nextCursorRef === null &&
      row.deletedSourceRef !== null &&
      row.deletedSourceAt !== null &&
      row.conflictRef === null &&
      row.cursorExpiredAt === null &&
      row.rateLimitedUntilAt === null
    );
  }

  if (row.state === 'syncConflict') {
    return (
      row.sourceFreshness === 'conflicted' &&
      row.payloadRedactionState === 'fully-redacted' &&
      row.tombstoneState === 'not-required' &&
      row.nextCursorRef !== null &&
      row.deletedSourceRef === null &&
      row.deletedSourceAt === null &&
      row.conflictRef !== null &&
      row.cursorExpiredAt === null &&
      row.rateLimitedUntilAt === null
    );
  }

  if (row.state === 'cursorExpired') {
    return (
      row.sourceFreshness === 'expired' &&
      row.payloadRedactionState === 'fully-redacted' &&
      row.tombstoneState === 'not-required' &&
      row.nextCursorRef === null &&
      row.deletedSourceRef === null &&
      row.deletedSourceAt === null &&
      row.conflictRef === null &&
      row.cursorExpiredAt !== null &&
      row.rateLimitedUntilAt === null
    );
  }

  return (
    row.state === 'rateLimited' &&
    row.sourceFreshness === 'rate-limited' &&
    row.payloadRedactionState === 'fully-redacted' &&
    row.tombstoneState === 'not-required' &&
    row.nextCursorRef === null &&
    row.deletedSourceRef === null &&
    row.deletedSourceAt === null &&
    row.conflictRef === null &&
    row.cursorExpiredAt === null &&
    row.rateLimitedUntilAt !== null
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
    proof.rows.every((row) => row.pageSize === proof.request.pageSize) &&
    proof.rows.every((row) => row.notificationPayloadBoundary === proof.request.notificationPayloadBoundary) &&
    proof.rows.every((row, index) => row.pageIndex === index + 1) &&
    new Set(proof.rows.map((row) => row.cursorRef)).size === proof.rows.length
  );
}
