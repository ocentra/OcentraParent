use serde::{Deserialize, Serialize};

mod enum_context;
mod enum_custody;
mod identifiers;
mod sample_rows;
mod text_parse;
mod text_types_actor;
mod text_types_core;
mod text_types_query;

pub const REPORT_QUERY_CUSTODY_SCHEMA_VERSION: &str = "report-query-custody-proof";
pub const REPORT_QUERY_CUSTODY_MAX_PAGE_SIZE: u32 = 100;

const REPORT_QUERY_CUSTODY_STATE_DERIVED_FRESH: &str = "derivedFresh";
const REPORT_QUERY_CUSTODY_STATE_DERIVED_STALE: &str = "derivedStale";
const REPORT_QUERY_CUSTODY_STATE_PARTIALLY_REDACTED: &str = "partiallyRedacted";
const REPORT_QUERY_CUSTODY_STATE_DELETED_SOURCE: &str = "deletedSource";
const REPORT_QUERY_CUSTODY_STATE_SYNC_CONFLICT: &str = "syncConflict";
const REPORT_QUERY_CUSTODY_STATE_CURSOR_EXPIRED: &str = "cursorExpired";
const REPORT_QUERY_CUSTODY_STATE_RATE_LIMITED: &str = "rateLimited";

const REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_SECOND_TRUTH_STORE: &str =
    "No uncontrolled second truth store is claimed for report or query custody.";
const REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_PORTAL_OR_RAW_CHILD: &str =
    "No portal rendering, provider routing, or raw child evidence handling is claimed.";
const REPORT_QUERY_CUSTODY_KNOWN_GAP_PAGINATION_DERIVED: &str = "Pagination is modeled as stable derived state over governed evidence, not a second report store.";
const REPORT_QUERY_CUSTODY_KNOWN_GAP_EXPLICIT_OUTCOMES: &str =
    "Delete, tombstone, stale, conflict, and rate-limit outcomes stay explicit and claim-safe.";
const REPORT_QUERY_CUSTODY_KNOWN_GAP_QUERY_STORE_SUMMARY_ONLY: &str =
    "Assistant and report citations stay inside query-store-summary evidence refs only.";

const REPORT_QUERY_CUSTODY_SAMPLE_CONTRACT_VERSION: &str = "v0.6";
const REPORT_QUERY_CUSTODY_SAMPLE_PROOF_TIMESTAMP: &str = "2026-06-28T15:55:00.000Z";
const REPORT_QUERY_CUSTODY_SAMPLE_REQUEST_ID: &str = "report-query-custody-request-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID: &str = "family-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_ACCOUNT_ID: &str = "parent-account-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_ID: &str =
    "windows-parent-device-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_LABEL: &str =
    "Windows parent device report query custody proof";
const REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTION_ID: &str =
    "parent-action-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_PARENT_AUTHORITY_ID: &str =
    "parent-authority-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTOR_ID: &str = "parent-report-query-custody-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_POLICY_VERSION: &str = "report-query-custody-proof-v1";
const REPORT_QUERY_CUSTODY_SAMPLE_REQUESTED_CURSOR: &str = "report-query-custody-cursor-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_FRESH_NEXT_CURSOR: &str = "derived-fresh-next-cursor";
const REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_STALE_NEXT_CURSOR: &str = "derived-stale-next-cursor";
const REPORT_QUERY_CUSTODY_SAMPLE_PARTIALLY_REDACTED_NEXT_CURSOR: &str =
    "partially-redacted-next-cursor";
const REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_REF: &str = "deleted-source-ref-1";
const REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_AT: &str = "2026-06-28T15:57:00.000Z";
const REPORT_QUERY_CUSTODY_SAMPLE_SYNC_CONFLICT_NEXT_CURSOR: &str = "sync-conflict-next-cursor";
const REPORT_QUERY_CUSTODY_SAMPLE_CONFLICT_REF: &str = "conflict-ref-1";
const REPORT_QUERY_CUSTODY_SAMPLE_CURSOR_EXPIRED_AT: &str = "2026-06-28T15:59:00.000Z";
const REPORT_QUERY_CUSTODY_SAMPLE_RATE_LIMITED_UNTIL_AT: &str = "2026-06-28T16:05:00.000Z";
const REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_CURSOR_REF: &str =
    "report-query-custody-source-cursor-proof-1";
const REPORT_QUERY_CUSTODY_SAMPLE_STABLE_SORT_KEY: &str = "report-query-custody-stable-sort-key";
const REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_ONE: &str = "report-query-custody-evidence-1";
const REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_TWO: &str = "report-query-custody-evidence-2";
const REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_REF_ONE: &str = "report-query-custody-source-1";
const REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_REF_TWO: &str = "report-query-custody-source-2";

const REPORT_QUERY_CUSTODY_PAGE_SIZE: u32 = 25;
const REPORT_QUERY_CUSTODY_EXPECT_CONTRACT_VERSION: &str = "contract version";
const REPORT_QUERY_CUSTODY_EXPECT_ACCOUNT_ID: &str = "account id";
const REPORT_QUERY_CUSTODY_EXPECT_FAMILY_ID: &str = "family id";
const REPORT_QUERY_CUSTODY_EXPECT_DEVICE_ID: &str = "device id";
const REPORT_QUERY_CUSTODY_EXPECT_DEVICE_LABEL: &str = "device label";
const REPORT_QUERY_CUSTODY_EXPECT_ACTOR_ID: &str = "actor id";
const REPORT_QUERY_CUSTODY_EXPECT_POLICY_VERSION: &str = "policy version";
const REPORT_QUERY_CUSTODY_EXPECT_EVIDENCE_ID: &str = "evidence id";
const REPORT_QUERY_CUSTODY_EXPECT_PARENT_ACTION_ID: &str = "parent action id";
const REPORT_QUERY_CUSTODY_EXPECT_PARENT_AUTHORITY_ID: &str = "parent authority id";
const REPORT_QUERY_CUSTODY_EXPECT_TIMESTAMP: &str = "timestamp";
const REPORT_QUERY_CUSTODY_EXPECT_REQUEST_ID: &str = "request id";
const REPORT_QUERY_CUSTODY_EXPECT_QUERY_CURSOR: &str = "query cursor";
const REPORT_QUERY_CUSTODY_EXPECT_CURSOR_REF: &str = "cursor ref";
const REPORT_QUERY_CUSTODY_EXPECT_SORT_KEY: &str = "sort key";
const REPORT_QUERY_CUSTODY_EXPECT_SOURCE_REF: &str = "source ref";
const REPORT_QUERY_CUSTODY_EXPECT_CONFLICT_REF: &str = "conflict ref";
const REPORT_QUERY_CUSTODY_EXPECT_DELETED_SOURCE_REF: &str = "deleted source ref";

pub type ParentPlatform = enum_context::ParentPlatform;
pub type ParentActorRole = enum_context::ParentActorRole;
pub type ParentEvidenceReferenceKind = enum_context::ParentEvidenceReferenceKind;
pub type ReportQueryCustodyState = enum_context::ReportQueryCustodyState;
pub type ReportQueryCustodySourceFreshness = enum_custody::ReportQueryCustodySourceFreshness;
pub type ReportQueryCustodyBoundary = enum_custody::ReportQueryCustodyBoundary;
pub type ReportQueryCustodyPayloadRedaction = enum_custody::ReportQueryCustodyPayloadRedaction;
pub type ReportQueryCustodySourceDataClass = enum_custody::ReportQueryCustodySourceDataClass;
pub type ReportQueryCustodyNonClaim = enum_custody::ReportQueryCustodyNonClaim;
pub type ReportQueryCustodyTombstoneState = enum_custody::ReportQueryCustodyTombstoneState;

/// Every custody proof must account for each explicit query/report outcome.
/// Keep this list in the Rust contract owner so runtime proof builders cannot
/// silently drift from the generated edge contract's required state set.
pub fn required_report_query_custody_states() -> [ReportQueryCustodyState; 7] {
    [
        ReportQueryCustodyState::DerivedFresh,
        ReportQueryCustodyState::DerivedStale,
        ReportQueryCustodyState::PartiallyRedacted,
        ReportQueryCustodyState::DeletedSource,
        ReportQueryCustodyState::SyncConflict,
        ReportQueryCustodyState::CursorExpired,
        ReportQueryCustodyState::RateLimited,
    ]
}

pub type ParentContractSchemaVersion = text_types_core::ParentContractSchemaVersion;
pub type ParentAccountId = text_types_core::ParentAccountId;
pub type FamilyId = text_types_core::FamilyId;
pub type ChildProfileId = text_types_core::ChildProfileId;
pub type ParentDeviceId = text_types_core::ParentDeviceId;
pub type ParentDeviceLabel = text_types_core::ParentDeviceLabel;
pub type ParentActorId = text_types_actor::ParentActorId;
pub type ParentPolicyVersion = text_types_actor::ParentPolicyVersion;
pub type ParentEvidenceReferenceId = text_types_actor::ParentEvidenceReferenceId;
pub type ParentActionReferenceId = text_types_actor::ParentActionReferenceId;
pub type ParentAuthorityReferenceId = text_types_actor::ParentAuthorityReferenceId;
pub type ParentTimestamp = text_types_actor::ParentTimestamp;
pub type ReportQueryCustodyRequestId = text_types_actor::ReportQueryCustodyRequestId;
pub type ReportQueryCustodyQueryCursor = text_types_query::ReportQueryCustodyQueryCursor;
pub type ReportQueryCustodyCursorRef = text_types_query::ReportQueryCustodyCursorRef;
pub type ReportQueryCustodySortKey = text_types_query::ReportQueryCustodySortKey;
pub type ReportQueryCustodySourceRef = text_types_query::ReportQueryCustodySourceRef;
pub type ReportQueryCustodyConflictRef = text_types_query::ReportQueryCustodyConflictRef;
pub type ReportQueryCustodyDeletedSourceRef = text_types_query::ReportQueryCustodyDeletedSourceRef;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActorReference {
    pub actor_id: ParentActorId,
    pub role: ParentActorRole,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentAccountReference {
    pub parent_account_id: ParentAccountId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyReference {
    pub family_id: FamilyId,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDeviceReference {
    pub device_id: ParentDeviceId,
    pub child_profile_id: Option<ChildProfileId>,
    pub label: ParentDeviceLabel,
    pub platform: ParentPlatform,
}

/// A Rust-owned authority reference is the narrow handoff from an
/// account/household authority snapshot into report/query custody. The
/// encoded reference is not itself proof of currentness; the storage owner
/// must bind it to the verified capability snapshot and validate its identity,
/// generation, and expiry before deriving a row. That validation does not
/// claim repository currentness after the snapshot was issued. The reference
/// deliberately carries the identity tuple so an authority for another
/// household, account, device, or child cannot be reused by shape alone.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportQueryCustodyParentAuthorityReference {
    pub authority_reference_id: ParentAuthorityReferenceId,
    pub family_id: FamilyId,
    pub parent_account_id: ParentAccountId,
    pub device_id: ParentDeviceId,
    pub child_profile_id: Option<ChildProfileId>,
    pub authority_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: ParentEvidenceReferenceId,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: ParentTimestamp,
    pub family_id: FamilyId,
    pub child_profile_id: Option<ChildProfileId>,
    pub source_data_class: ReportQueryCustodySourceDataClass,
    pub source_reference: ReportQueryCustodySourceRef,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentActionReference {
    pub action_reference_id: ParentActionReferenceId,
    pub actor: ParentActorReference,
    pub policy_version: ParentPolicyVersion,
    pub created_at: ParentTimestamp,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportQueryCustodyRequest {
    pub schema_version: String,
    pub request_id: ReportQueryCustodyRequestId,
    pub family: FamilyReference,
    pub account: ParentAccountReference,
    pub device: ParentDeviceReference,
    pub parent_action: ParentActionReference,
    pub requested_cursor: ReportQueryCustodyQueryCursor,
    pub page_size: u32,
    pub requested_data_classes: Vec<ReportQueryCustodySourceDataClass>,
    pub allowed_source_data_classes: Vec<ReportQueryCustodySourceDataClass>,
    pub source_citation_refs: Vec<ParentEvidenceReference>,
    pub assistant_citation_refs: Vec<ParentEvidenceReference>,
    pub notification_payload_boundary: ReportQueryCustodyBoundary,
    pub parent_authority: ReportQueryCustodyParentAuthorityReference,
    pub raw_child_evidence_requested: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportQueryCustodyRow {
    pub row_id: ReportQueryCustodySourceRef,
    pub request_id: ReportQueryCustodyRequestId,
    pub state: ReportQueryCustodyState,
    pub source_freshness: ReportQueryCustodySourceFreshness,
    pub source_data_class: ReportQueryCustodySourceDataClass,
    pub cursor_ref: ReportQueryCustodyCursorRef,
    pub source_cursor_ref: ReportQueryCustodyCursorRef,
    pub next_cursor_ref: Option<ReportQueryCustodyCursorRef>,
    pub page_index: u32,
    pub page_size: u32,
    pub stable_sort_key: ReportQueryCustodySortKey,
    pub requested_data_classes: Vec<ReportQueryCustodySourceDataClass>,
    pub allowed_source_data_classes: Vec<ReportQueryCustodySourceDataClass>,
    pub source_citation_refs: Vec<ParentEvidenceReference>,
    pub assistant_citation_refs: Vec<ParentEvidenceReference>,
    pub notification_payload_boundary: ReportQueryCustodyBoundary,
    pub payload_redaction_state: ReportQueryCustodyPayloadRedaction,
    pub tombstone_state: ReportQueryCustodyTombstoneState,
    pub deleted_source_ref: Option<ReportQueryCustodyDeletedSourceRef>,
    pub deleted_source_at: Option<ParentTimestamp>,
    pub conflict_ref: Option<ReportQueryCustodyConflictRef>,
    pub cursor_expired_at: Option<ParentTimestamp>,
    pub rate_limited_until_at: Option<ParentTimestamp>,
    pub parent_authority: ReportQueryCustodyParentAuthorityReference,
    pub raw_child_evidence_included: bool,
    pub report_cache_mutated: bool,
    pub second_truth_store_claimed: bool,
    pub claim_safe: bool,
}

/// Untrusted cross-boundary proof payload.
///
/// Deserialization proves only wire shape. Rust runtime callers must obtain a
/// sealed `ValidatedReportQueryCustodyProofSnapshot` from
/// `ocentra-storage-custody-core`; this DTO is never authority by itself.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportQueryCustodyContractProof {
    pub schema_version: String,
    pub contract_version: ParentContractSchemaVersion,
    pub request: ReportQueryCustodyRequest,
    pub rows: Vec<ReportQueryCustodyRow>,
    pub non_claims: Vec<ReportQueryCustodyNonClaim>,
    pub report_runtime_claimed: bool,
    pub portal_ui_claimed: bool,
    pub provider_routing_claimed: bool,
    pub ocentra_hosted_family_data_custody_claimed: bool,
    pub second_truth_store_claimed: bool,
    pub raw_child_evidence_claimed: bool,
    pub updated_at: ParentTimestamp,
}

pub fn report_query_custody_known_gaps() -> [&'static str; 5] {
    sample_rows::report_query_custody_known_gaps()
}

pub fn required_report_query_custody_non_claims() -> Vec<ReportQueryCustodyNonClaim> {
    sample_rows::required_report_query_custody_non_claims()
}

pub fn sample_report_query_custody_contract_proof() -> ReportQueryCustodyContractProof {
    sample_rows::sample_report_query_custody_contract_proof()
}
