use super::report_query_custody::{
    report_query_custody_known_gaps, sample_report_query_custody_contract_proof,
    REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
};

const REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT: &str = "report query custody proof json";
const REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR: &str = "\n";

pub fn report_query_custody_contracts_typescript() -> String {
    let proof_json = crate::schema_result_or_unreachable(
        serde_json::to_string_pretty(&sample_report_query_custody_contract_proof()),
        REPORT_QUERY_CUSTODY_PROOF_JSON_EXPECT,
    );
    let known_gaps = report_query_custody_known_gaps()
        .iter()
        .map(|gap| format!("  {:?},", gap))
        .collect::<Vec<_>>()
        .join(REPORT_QUERY_CUSTODY_KNOWN_GAPS_SEPARATOR);

    format!(
        r#"/* generated from crates/schema/src/report_query_custody.rs */

export const ReportQueryCustodyContractRuntime = {{
  SchemaVersion: '{schema_version}',
}} as const;

export type GeneratedParentContractSchemaVersion = 'v0.6';
export type GeneratedParentAccountId = string;
export type GeneratedFamilyId = string;
export type GeneratedChildProfileId = string;
export type GeneratedParentDeviceId = string;
export type GeneratedParentDeviceLabel = string;
export type GeneratedParentActorId = string;
export type GeneratedParentPolicyVersion = string;
export type GeneratedParentEvidenceReferenceId = string;
export type GeneratedParentActionReferenceId = string;
export type GeneratedParentTimestamp = string;
export type GeneratedReportQueryCustodyRequestId = string;
export type GeneratedReportQueryCustodyQueryCursor = string;
export type GeneratedReportQueryCustodyCursorRef = string;
export type GeneratedReportQueryCustodySortKey = string;
export type GeneratedReportQueryCustodySourceRef = string;
export type GeneratedReportQueryCustodyConflictRef = string;
export type GeneratedReportQueryCustodyDeletedSourceRef = string;

export type GeneratedParentPlatform = 'windows' | 'linux' | 'macos' | 'android' | 'ios';
export type GeneratedParentActorRole = 'parent' | 'guardian' | 'system';
export type GeneratedParentEvidenceReferenceKind =
  | 'journal-event'
  | 'query-store-summary'
  | 'activity-event'
  | 'policy-decision'
  | 'local-ai-result';
export type GeneratedReportQueryCustodyState =
  | 'derivedFresh'
  | 'derivedStale'
  | 'partiallyRedacted'
  | 'deletedSource'
  | 'syncConflict'
  | 'cursorExpired'
  | 'rateLimited';
export type GeneratedReportQueryCustodySourceFreshness =
  | 'fresh'
  | 'stale'
  | 'deleted'
  | 'conflicted'
  | 'expired'
  | 'rate-limited';
export type GeneratedReportQueryCustodyBoundary = 'parent-owned-citations-only';
export type GeneratedReportQueryCustodyPayloadRedaction = 'fully-redacted' | 'partially-redacted';
export type GeneratedReportQueryCustodySourceDataClass =
  | 'sqlite-query-row'
  | 'notification-history'
  | 'audit-event'
  | 'generated-summary';
export type GeneratedReportQueryCustodyNonClaim =
  | 'no-second-truth-store'
  | 'no-portal-ui'
  | 'no-raw-child-evidence'
  | 'no-unbounded-pagination'
  | 'no-provider-routing'
  | 'no-ocentra-hosted-family-data-custody';
export type GeneratedReportQueryCustodyTombstoneState = 'not-required' | 'written';

export const GeneratedParentPlatforms = ['windows', 'linux', 'macos', 'android', 'ios'] as const satisfies readonly GeneratedParentPlatform[];
export const GeneratedParentActorRoles = ['parent', 'guardian', 'system'] as const satisfies readonly GeneratedParentActorRole[];
export const GeneratedParentEvidenceReferenceKinds = [
  'journal-event',
  'query-store-summary',
  'activity-event',
  'policy-decision',
  'local-ai-result',
] as const satisfies readonly GeneratedParentEvidenceReferenceKind[];
export const GeneratedReportQueryCustodyStates = [
  'derivedFresh',
  'derivedStale',
  'partiallyRedacted',
  'deletedSource',
  'syncConflict',
  'cursorExpired',
  'rateLimited',
] as const satisfies readonly GeneratedReportQueryCustodyState[];
export const GeneratedReportQueryCustodySourceFreshnessStates = [
  'fresh',
  'stale',
  'deleted',
  'conflicted',
  'expired',
  'rate-limited',
] as const satisfies readonly GeneratedReportQueryCustodySourceFreshness[];
export const GeneratedReportQueryCustodyBoundaries = [
  'parent-owned-citations-only',
] as const satisfies readonly GeneratedReportQueryCustodyBoundary[];
export const GeneratedReportQueryCustodyPayloadRedactionStates = [
  'fully-redacted',
  'partially-redacted',
] as const satisfies readonly GeneratedReportQueryCustodyPayloadRedaction[];
export const GeneratedReportQueryCustodySourceDataClasses = [
  'sqlite-query-row',
  'notification-history',
  'audit-event',
  'generated-summary',
] as const satisfies readonly GeneratedReportQueryCustodySourceDataClass[];
export const GeneratedReportQueryCustodyNonClaims = [
  'no-second-truth-store',
  'no-portal-ui',
  'no-raw-child-evidence',
  'no-unbounded-pagination',
  'no-provider-routing',
  'no-ocentra-hosted-family-data-custody',
] as const satisfies readonly GeneratedReportQueryCustodyNonClaim[];
export const GeneratedReportQueryCustodyTombstoneStates = [
  'not-required',
  'written',
] as const satisfies readonly GeneratedReportQueryCustodyTombstoneState[];

export interface GeneratedParentActorReference {{
  actorId: GeneratedParentActorId;
  role: GeneratedParentActorRole;
}}

export interface GeneratedParentAccountReference {{
  parentAccountId: GeneratedParentAccountId;
}}

export interface GeneratedFamilyReference {{
  familyId: GeneratedFamilyId;
}}

export interface GeneratedParentDeviceReference {{
  deviceId: GeneratedParentDeviceId;
  childProfileId: GeneratedChildProfileId | null;
  label: GeneratedParentDeviceLabel;
  platform: GeneratedParentPlatform;
}}

export interface GeneratedParentEvidenceReference {{
  evidenceReferenceId: GeneratedParentEvidenceReferenceId;
  kind: GeneratedParentEvidenceReferenceKind;
  observedAt: GeneratedParentTimestamp;
}}

export interface GeneratedParentActionReference {{
  actionReferenceId: GeneratedParentActionReferenceId;
  actor: GeneratedParentActorReference;
  policyVersion: GeneratedParentPolicyVersion;
  createdAt: GeneratedParentTimestamp;
}}

export interface GeneratedReportQueryCustodyRequest {{
  schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  requestId: GeneratedReportQueryCustodyRequestId;
  family: GeneratedFamilyReference;
  account: GeneratedParentAccountReference;
  device: GeneratedParentDeviceReference;
  parentAction: GeneratedParentActionReference;
  requestedCursor: GeneratedReportQueryCustodyQueryCursor;
  pageSize: number;
  requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  parentAuthorized: boolean;
  parentOwnedSourceRequired: boolean;
  rawChildEvidenceRequested: boolean;
}}

export interface GeneratedReportQueryCustodyRow {{
  rowId: GeneratedReportQueryCustodySourceRef;
  requestId: GeneratedReportQueryCustodyRequestId;
  state: GeneratedReportQueryCustodyState;
  sourceFreshness: GeneratedReportQueryCustodySourceFreshness;
  sourceDataClass: GeneratedReportQueryCustodySourceDataClass;
  cursorRef: GeneratedReportQueryCustodyCursorRef;
  sourceCursorRef: GeneratedReportQueryCustodyCursorRef;
  nextCursorRef: GeneratedReportQueryCustodyCursorRef | null;
  pageIndex: number;
  pageSize: number;
  stableSortKey: GeneratedReportQueryCustodySortKey;
  requestedDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  allowedSourceDataClasses: readonly GeneratedReportQueryCustodySourceDataClass[];
  sourceCitationRefs: readonly GeneratedParentEvidenceReference[];
  assistantCitationRefs: readonly GeneratedParentEvidenceReference[];
  notificationPayloadBoundary: GeneratedReportQueryCustodyBoundary;
  payloadRedactionState: GeneratedReportQueryCustodyPayloadRedaction;
  tombstoneState: GeneratedReportQueryCustodyTombstoneState;
  deletedSourceRef: GeneratedReportQueryCustodyDeletedSourceRef | null;
  deletedSourceAt: GeneratedParentTimestamp | null;
  conflictRef: GeneratedReportQueryCustodyConflictRef | null;
  cursorExpiredAt: GeneratedParentTimestamp | null;
  rateLimitedUntilAt: GeneratedParentTimestamp | null;
  parentAuthorized: boolean;
  parentOwnedSourceRequired: boolean;
  rawChildEvidenceIncluded: boolean;
  reportCacheMutated: boolean;
  secondTruthStoreClaimed: boolean;
  claimSafe: boolean;
}}

export interface GeneratedReportQueryCustodyContractProof {{
  schemaVersion: typeof ReportQueryCustodyContractRuntime.SchemaVersion;
  contractVersion: GeneratedParentContractSchemaVersion;
  request: GeneratedReportQueryCustodyRequest;
  rows: readonly GeneratedReportQueryCustodyRow[];
  nonClaims: readonly GeneratedReportQueryCustodyNonClaim[];
  reportRuntimeClaimed: boolean;
  portalUiClaimed: boolean;
  providerRoutingClaimed: boolean;
  ocentraHostedFamilyDataCustodyClaimed: boolean;
  secondTruthStoreClaimed: boolean;
  rawChildEvidenceClaimed: boolean;
  updatedAt: GeneratedParentTimestamp;
}}

export const GeneratedReportQueryCustodyKnownGaps = [
{known_gaps}
] as const;

export const GeneratedReportQueryCustodyContractProof = {proof_json} as const satisfies GeneratedReportQueryCustodyContractProof;
"#,
        schema_version = REPORT_QUERY_CUSTODY_SCHEMA_VERSION,
        proof_json = proof_json,
        known_gaps = known_gaps,
    )
}

pub fn report_query_custody_contract_rules_typescript() -> String {
    r#"/* generated from crates/schema/src/report_query_custody.rs */

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
"#
    .to_owned()
}
