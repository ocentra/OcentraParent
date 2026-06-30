use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub const REPORT_QUERY_CUSTODY_SCHEMA_VERSION: &str = "report-query-custody-proof";

const PARENT_PLATFORM_WINDOWS: &str = "windows";
const PARENT_PLATFORM_LINUX: &str = "linux";
const PARENT_PLATFORM_MACOS: &str = "macos";
const PARENT_PLATFORM_ANDROID: &str = "android";
const PARENT_PLATFORM_IOS: &str = "ios";

const PARENT_ACTOR_ROLE_PARENT: &str = "parent";
const PARENT_ACTOR_ROLE_GUARDIAN: &str = "guardian";
const PARENT_ACTOR_ROLE_SYSTEM: &str = "system";

const PARENT_EVIDENCE_REFERENCE_KIND_JOURNAL_EVENT: &str = "journal-event";
const PARENT_EVIDENCE_REFERENCE_KIND_QUERY_STORE_SUMMARY: &str = "query-store-summary";
const PARENT_EVIDENCE_REFERENCE_KIND_ACTIVITY_EVENT: &str = "activity-event";
const PARENT_EVIDENCE_REFERENCE_KIND_POLICY_DECISION: &str = "policy-decision";
const PARENT_EVIDENCE_REFERENCE_KIND_LOCAL_AI_RESULT: &str = "local-ai-result";

const REPORT_QUERY_CUSTODY_STATE_DERIVED_FRESH: &str = "derivedFresh";
const REPORT_QUERY_CUSTODY_STATE_DERIVED_STALE: &str = "derivedStale";
const REPORT_QUERY_CUSTODY_STATE_PARTIALLY_REDACTED: &str = "partiallyRedacted";
const REPORT_QUERY_CUSTODY_STATE_DELETED_SOURCE: &str = "deletedSource";
const REPORT_QUERY_CUSTODY_STATE_SYNC_CONFLICT: &str = "syncConflict";
const REPORT_QUERY_CUSTODY_STATE_CURSOR_EXPIRED: &str = "cursorExpired";
const REPORT_QUERY_CUSTODY_STATE_RATE_LIMITED: &str = "rateLimited";

const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_FRESH: &str = "fresh";
const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_STALE: &str = "stale";
const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_DELETED: &str = "deleted";
const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_CONFLICTED: &str = "conflicted";
const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_EXPIRED: &str = "expired";
const REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_RATE_LIMITED: &str = "rate-limited";

const REPORT_QUERY_CUSTODY_BOUNDARY_PARENT_OWNED_CITATIONS_ONLY: &str =
    "parent-owned-citations-only";
const REPORT_QUERY_CUSTODY_PAYLOAD_REDACTION_FULLY_REDACTED: &str = "fully-redacted";
const REPORT_QUERY_CUSTODY_PAYLOAD_REDACTION_PARTIALLY_REDACTED: &str = "partially-redacted";

const REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_SQLITE_QUERY_ROW: &str = "sqlite-query-row";
const REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_NOTIFICATION_HISTORY: &str = "notification-history";
const REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_AUDIT_EVENT: &str = "audit-event";
const REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_GENERATED_SUMMARY: &str = "generated-summary";

const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_SECOND_TRUTH_STORE: &str = "no-second-truth-store";
const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_PORTAL_UI: &str = "no-portal-ui";
const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_RAW_CHILD_EVIDENCE: &str = "no-raw-child-evidence";
const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_UNBOUNDED_PAGINATION: &str = "no-unbounded-pagination";
const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_PROVIDER_ROUTING: &str = "no-provider-routing";
const REPORT_QUERY_CUSTODY_NON_CLAIM_NO_OCENTRA_HOSTED_FAMILY_DATA_CUSTODY: &str =
    "no-ocentra-hosted-family-data-custody";

const REPORT_QUERY_CUSTODY_TOMBSTONE_STATE_NOT_REQUIRED: &str = "not-required";
const REPORT_QUERY_CUSTODY_TOMBSTONE_STATE_WRITTEN: &str = "written";

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
const REPORT_QUERY_CUSTODY_EXPECT_TIMESTAMP: &str = "timestamp";
const REPORT_QUERY_CUSTODY_EXPECT_REQUEST_ID: &str = "request id";
const REPORT_QUERY_CUSTODY_EXPECT_QUERY_CURSOR: &str = "query cursor";
const REPORT_QUERY_CUSTODY_EXPECT_CURSOR_REF: &str = "cursor ref";
const REPORT_QUERY_CUSTODY_EXPECT_SORT_KEY: &str = "sort key";
const REPORT_QUERY_CUSTODY_EXPECT_SOURCE_REF: &str = "source ref";
const REPORT_QUERY_CUSTODY_EXPECT_CONFLICT_REF: &str = "conflict ref";
const REPORT_QUERY_CUSTODY_EXPECT_DELETED_SOURCE_REF: &str = "deleted source ref";

macro_rules! report_query_text_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                if value.trim().is_empty() {
                    None
                } else {
                    Some(Self(value))
                }
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

macro_rules! report_query_string_enum {
    ($name:ident { $($variant:ident => $value:expr),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)+
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }

        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                $(if value == $value { return Ok(Self::$variant); })+
                Err(serde::de::Error::unknown_variant(value.as_str(), &[$($value,)+]))
            }
        }
    };
}

report_query_string_enum!(ParentPlatform {
    Windows => PARENT_PLATFORM_WINDOWS,
    Linux => PARENT_PLATFORM_LINUX,
    Macos => PARENT_PLATFORM_MACOS,
    Android => PARENT_PLATFORM_ANDROID,
    Ios => PARENT_PLATFORM_IOS,
});

report_query_string_enum!(ParentActorRole {
    Parent => PARENT_ACTOR_ROLE_PARENT,
    Guardian => PARENT_ACTOR_ROLE_GUARDIAN,
    System => PARENT_ACTOR_ROLE_SYSTEM,
});

report_query_string_enum!(ParentEvidenceReferenceKind {
    JournalEvent => PARENT_EVIDENCE_REFERENCE_KIND_JOURNAL_EVENT,
    QueryStoreSummary => PARENT_EVIDENCE_REFERENCE_KIND_QUERY_STORE_SUMMARY,
    ActivityEvent => PARENT_EVIDENCE_REFERENCE_KIND_ACTIVITY_EVENT,
    PolicyDecision => PARENT_EVIDENCE_REFERENCE_KIND_POLICY_DECISION,
    LocalAiResult => PARENT_EVIDENCE_REFERENCE_KIND_LOCAL_AI_RESULT,
});

report_query_string_enum!(ReportQueryCustodyState {
    DerivedFresh => REPORT_QUERY_CUSTODY_STATE_DERIVED_FRESH,
    DerivedStale => REPORT_QUERY_CUSTODY_STATE_DERIVED_STALE,
    PartiallyRedacted => REPORT_QUERY_CUSTODY_STATE_PARTIALLY_REDACTED,
    DeletedSource => REPORT_QUERY_CUSTODY_STATE_DELETED_SOURCE,
    SyncConflict => REPORT_QUERY_CUSTODY_STATE_SYNC_CONFLICT,
    CursorExpired => REPORT_QUERY_CUSTODY_STATE_CURSOR_EXPIRED,
    RateLimited => REPORT_QUERY_CUSTODY_STATE_RATE_LIMITED,
});

report_query_string_enum!(ReportQueryCustodySourceFreshness {
    Fresh => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_FRESH,
    Stale => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_STALE,
    Deleted => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_DELETED,
    Conflicted => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_CONFLICTED,
    Expired => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_EXPIRED,
    RateLimited => REPORT_QUERY_CUSTODY_SOURCE_FRESHNESS_RATE_LIMITED,
});

report_query_string_enum!(ReportQueryCustodyBoundary {
    ParentOwnedCitationsOnly => REPORT_QUERY_CUSTODY_BOUNDARY_PARENT_OWNED_CITATIONS_ONLY,
});

report_query_string_enum!(ReportQueryCustodyPayloadRedaction {
    FullyRedacted => REPORT_QUERY_CUSTODY_PAYLOAD_REDACTION_FULLY_REDACTED,
    PartiallyRedacted => REPORT_QUERY_CUSTODY_PAYLOAD_REDACTION_PARTIALLY_REDACTED,
});

report_query_string_enum!(ReportQueryCustodySourceDataClass {
    SqliteQueryRow => REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_SQLITE_QUERY_ROW,
    NotificationHistory => REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_NOTIFICATION_HISTORY,
    AuditEvent => REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_AUDIT_EVENT,
    GeneratedSummary => REPORT_QUERY_CUSTODY_SOURCE_DATA_CLASS_GENERATED_SUMMARY,
});

report_query_string_enum!(ReportQueryCustodyNonClaim {
    NoSecondTruthStore => REPORT_QUERY_CUSTODY_NON_CLAIM_NO_SECOND_TRUTH_STORE,
    NoPortalUi => REPORT_QUERY_CUSTODY_NON_CLAIM_NO_PORTAL_UI,
    NoRawChildEvidence => REPORT_QUERY_CUSTODY_NON_CLAIM_NO_RAW_CHILD_EVIDENCE,
    NoUnboundedPagination => REPORT_QUERY_CUSTODY_NON_CLAIM_NO_UNBOUNDED_PAGINATION,
    NoProviderRouting => REPORT_QUERY_CUSTODY_NON_CLAIM_NO_PROVIDER_ROUTING,
    NoOcentraHostedFamilyDataCustody =>
        REPORT_QUERY_CUSTODY_NON_CLAIM_NO_OCENTRA_HOSTED_FAMILY_DATA_CUSTODY,
});

report_query_string_enum!(ReportQueryCustodyTombstoneState {
    NotRequired => REPORT_QUERY_CUSTODY_TOMBSTONE_STATE_NOT_REQUIRED,
    Written => REPORT_QUERY_CUSTODY_TOMBSTONE_STATE_WRITTEN,
});

report_query_text_identifier!(ParentContractSchemaVersion);
report_query_text_identifier!(ParentAccountId);
report_query_text_identifier!(FamilyId);
report_query_text_identifier!(ChildProfileId);
report_query_text_identifier!(ParentDeviceId);
report_query_text_identifier!(ParentDeviceLabel);
report_query_text_identifier!(ParentActorId);
report_query_text_identifier!(ParentPolicyVersion);
report_query_text_identifier!(ParentEvidenceReferenceId);
report_query_text_identifier!(ParentActionReferenceId);
report_query_text_identifier!(ParentTimestamp);
report_query_text_identifier!(ReportQueryCustodyRequestId);
report_query_text_identifier!(ReportQueryCustodyQueryCursor);
report_query_text_identifier!(ReportQueryCustodyCursorRef);
report_query_text_identifier!(ReportQueryCustodySortKey);
report_query_text_identifier!(ReportQueryCustodySourceRef);
report_query_text_identifier!(ReportQueryCustodyConflictRef);
report_query_text_identifier!(ReportQueryCustodyDeletedSourceRef);

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: ParentEvidenceReferenceId,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: ParentTimestamp,
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
    pub parent_authorized: bool,
    pub parent_owned_source_required: bool,
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
    pub parent_authorized: bool,
    pub parent_owned_source_required: bool,
    pub raw_child_evidence_included: bool,
    pub report_cache_mutated: bool,
    pub second_truth_store_claimed: bool,
    pub claim_safe: bool,
}

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
    [
        REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_SECOND_TRUTH_STORE,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_NO_PORTAL_OR_RAW_CHILD,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_PAGINATION_DERIVED,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_EXPLICIT_OUTCOMES,
        REPORT_QUERY_CUSTODY_KNOWN_GAP_QUERY_STORE_SUMMARY_ONLY,
    ]
}

pub fn required_report_query_custody_non_claims() -> Vec<ReportQueryCustodyNonClaim> {
    vec![
        ReportQueryCustodyNonClaim::NoSecondTruthStore,
        ReportQueryCustodyNonClaim::NoPortalUi,
        ReportQueryCustodyNonClaim::NoRawChildEvidence,
        ReportQueryCustodyNonClaim::NoUnboundedPagination,
        ReportQueryCustodyNonClaim::NoProviderRouting,
        ReportQueryCustodyNonClaim::NoOcentraHostedFamilyDataCustody,
    ]
}

pub fn sample_report_query_custody_contract_proof() -> ReportQueryCustodyContractProof {
    let proof_timestamp = timestamp(REPORT_QUERY_CUSTODY_SAMPLE_PROOF_TIMESTAMP);
    let request = ReportQueryCustodyRequest {
        schema_version: REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        request_id: request_id(REPORT_QUERY_CUSTODY_SAMPLE_REQUEST_ID),
        family: FamilyReference {
            family_id: family_id(REPORT_QUERY_CUSTODY_SAMPLE_FAMILY_ID),
        },
        account: ParentAccountReference {
            parent_account_id: account_id(REPORT_QUERY_CUSTODY_SAMPLE_ACCOUNT_ID),
        },
        device: ParentDeviceReference {
            device_id: parent_device_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_ID),
            child_profile_id: None,
            label: parent_device_label(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_DEVICE_LABEL),
            platform: ParentPlatform::Windows,
        },
        parent_action: ParentActionReference {
            action_reference_id: parent_action_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTION_ID),
            actor: ParentActorReference {
                actor_id: parent_actor_id(REPORT_QUERY_CUSTODY_SAMPLE_PARENT_ACTOR_ID),
                role: ParentActorRole::Parent,
            },
            policy_version: policy_version(REPORT_QUERY_CUSTODY_SAMPLE_POLICY_VERSION),
            created_at: proof_timestamp.clone(),
        },
        requested_cursor: query_cursor(REPORT_QUERY_CUSTODY_SAMPLE_REQUESTED_CURSOR),
        page_size: REPORT_QUERY_CUSTODY_PAGE_SIZE,
        requested_data_classes: allowed_data_classes(),
        allowed_source_data_classes: allowed_data_classes(),
        source_citation_refs: allowed_citation_refs(&proof_timestamp),
        assistant_citation_refs: allowed_citation_refs(&proof_timestamp),
        notification_payload_boundary: ReportQueryCustodyBoundary::ParentOwnedCitationsOnly,
        parent_authorized: true,
        parent_owned_source_required: true,
        raw_child_evidence_requested: false,
    };

    ReportQueryCustodyContractProof {
        schema_version: REPORT_QUERY_CUSTODY_SCHEMA_VERSION.to_string(),
        contract_version: contract_version(REPORT_QUERY_CUSTODY_SAMPLE_CONTRACT_VERSION),
        request: request.clone(),
        rows: sample_report_query_custody_rows(&request),
        non_claims: required_report_query_custody_non_claims(),
        report_runtime_claimed: false,
        portal_ui_claimed: false,
        provider_routing_claimed: false,
        ocentra_hosted_family_data_custody_claimed: false,
        second_truth_store_claimed: false,
        raw_child_evidence_claimed: false,
        updated_at: proof_timestamp,
    }
}

fn sample_report_query_custody_rows(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    let mut rows = sample_report_query_custody_rows_primary(request);
    rows.extend(sample_report_query_custody_rows_secondary(request));
    rows
}

fn sample_report_query_custody_rows_primary(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    [
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::DerivedFresh,
            source_freshness: ReportQueryCustodySourceFreshness::Fresh,
            source_data_class: ReportQueryCustodySourceDataClass::SqliteQueryRow,
            page_index: 1,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_FRESH_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::DerivedStale,
            source_freshness: ReportQueryCustodySourceFreshness::Stale,
            source_data_class: ReportQueryCustodySourceDataClass::GeneratedSummary,
            page_index: 2,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DERIVED_STALE_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::PartiallyRedacted,
            source_freshness: ReportQueryCustodySourceFreshness::Stale,
            source_data_class: ReportQueryCustodySourceDataClass::NotificationHistory,
            page_index: 3,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_PARTIALLY_REDACTED_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::PartiallyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::DeletedSource,
            source_freshness: ReportQueryCustodySourceFreshness::Deleted,
            source_data_class: ReportQueryCustodySourceDataClass::AuditEvent,
            page_index: 4,
            next_cursor_ref: None,
            deleted_source_ref: Some(deleted_source_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_REF,
            )),
            deleted_source_at: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_DELETED_SOURCE_AT)),
            conflict_ref: None,
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::Written,
        }),
    ]
    .iter()
    .map(sample_row)
    .collect()
}

fn sample_report_query_custody_rows_secondary(
    request: &ReportQueryCustodyRequest,
) -> Vec<ReportQueryCustodyRow> {
    [
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::SyncConflict,
            source_freshness: ReportQueryCustodySourceFreshness::Conflicted,
            source_data_class: ReportQueryCustodySourceDataClass::SqliteQueryRow,
            page_index: 5,
            next_cursor_ref: Some(cursor_ref(
                REPORT_QUERY_CUSTODY_SAMPLE_SYNC_CONFLICT_NEXT_CURSOR,
            )),
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: Some(conflict_ref(REPORT_QUERY_CUSTODY_SAMPLE_CONFLICT_REF)),
            state_timestamp: None,
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::CursorExpired,
            source_freshness: ReportQueryCustodySourceFreshness::Expired,
            source_data_class: ReportQueryCustodySourceDataClass::GeneratedSummary,
            page_index: 6,
            next_cursor_ref: None,
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_CURSOR_EXPIRED_AT)),
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
        sample_row(ReportQueryCustodySampleRowInput {
            request: &request,
            state: ReportQueryCustodyState::RateLimited,
            source_freshness: ReportQueryCustodySourceFreshness::RateLimited,
            source_data_class: ReportQueryCustodySourceDataClass::NotificationHistory,
            page_index: 7,
            next_cursor_ref: None,
            deleted_source_ref: None,
            deleted_source_at: None,
            conflict_ref: None,
            state_timestamp: Some(timestamp(REPORT_QUERY_CUSTODY_SAMPLE_RATE_LIMITED_UNTIL_AT)),
            payload_redaction_state: ReportQueryCustodyPayloadRedaction::FullyRedacted,
            tombstone_state: ReportQueryCustodyTombstoneState::NotRequired,
        }),
    ]
    .iter()
    .map(sample_row)
    .collect()
}

struct ReportQueryCustodySampleRowInput<'a> {
    request: &'a ReportQueryCustodyRequest,
    state: ReportQueryCustodyState,
    source_freshness: ReportQueryCustodySourceFreshness,
    source_data_class: ReportQueryCustodySourceDataClass,
    page_index: u32,
    next_cursor_ref: Option<ReportQueryCustodyCursorRef>,
    deleted_source_ref: Option<ReportQueryCustodyDeletedSourceRef>,
    deleted_source_at: Option<ParentTimestamp>,
    conflict_ref: Option<ReportQueryCustodyConflictRef>,
    state_timestamp: Option<ParentTimestamp>,
    payload_redaction_state: ReportQueryCustodyPayloadRedaction,
    tombstone_state: ReportQueryCustodyTombstoneState,
}

fn sample_row(input: ReportQueryCustodySampleRowInput<'_>) -> ReportQueryCustodyRow {
    let ReportQueryCustodySampleRowInput {
        request,
        state,
        source_freshness,
        source_data_class,
        page_index,
        next_cursor_ref,
        deleted_source_ref,
        deleted_source_at,
        conflict_ref,
        state_timestamp,
        payload_redaction_state,
        tombstone_state,
    } = input;

    let is_rate_limited = state == ReportQueryCustodyState::RateLimited;
    let is_cursor_expired = state == ReportQueryCustodyState::CursorExpired;

    ReportQueryCustodyRow {
        row_id: source_ref(format!("report-query-custody-row-{}", state.as_str())),
        request_id: request.request_id.clone(),
        state,
        source_freshness,
        source_data_class,
        cursor_ref: cursor_ref(format!("report-query-custody-cursor-{}", state.as_str())),
        source_cursor_ref: cursor_ref(REPORT_QUERY_CUSTODY_SAMPLE_SOURCE_CURSOR_REF),
        next_cursor_ref,
        page_index,
        page_size: request.page_size,
        stable_sort_key: sort_key(REPORT_QUERY_CUSTODY_SAMPLE_STABLE_SORT_KEY),
        requested_data_classes: request.requested_data_classes.clone(),
        allowed_source_data_classes: request.allowed_source_data_classes.clone(),
        source_citation_refs: request.source_citation_refs.clone(),
        assistant_citation_refs: request.assistant_citation_refs.clone(),
        notification_payload_boundary: request.notification_payload_boundary,
        payload_redaction_state,
        tombstone_state,
        deleted_source_ref,
        deleted_source_at,
        conflict_ref,
        cursor_expired_at: if is_cursor_expired {
            state_timestamp.clone()
        } else {
            None
        },
        rate_limited_until_at: if is_rate_limited {
            state_timestamp
        } else {
            None
        },
        parent_authorized: true,
        parent_owned_source_required: true,
        raw_child_evidence_included: false,
        report_cache_mutated: false,
        second_truth_store_claimed: false,
        claim_safe: true,
    }
}

fn allowed_data_classes() -> Vec<ReportQueryCustodySourceDataClass> {
    vec![
        ReportQueryCustodySourceDataClass::SqliteQueryRow,
        ReportQueryCustodySourceDataClass::NotificationHistory,
        ReportQueryCustodySourceDataClass::AuditEvent,
        ReportQueryCustodySourceDataClass::GeneratedSummary,
    ]
}

fn allowed_citation_refs(timestamp: &ParentTimestamp) -> Vec<ParentEvidenceReference> {
    vec![
        ParentEvidenceReference {
            evidence_reference_id: evidence_id(REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_ONE),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp.clone(),
        },
        ParentEvidenceReference {
            evidence_reference_id: evidence_id(REPORT_QUERY_CUSTODY_SAMPLE_EVIDENCE_ID_TWO),
            kind: ParentEvidenceReferenceKind::QueryStoreSummary,
            observed_at: timestamp.clone(),
        },
    ]
}

fn contract_version(value: &str) -> ParentContractSchemaVersion {
    crate::schema_option_or_unreachable(
        ParentContractSchemaVersion::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_CONTRACT_VERSION,
    )
}

fn account_id(value: &str) -> ParentAccountId {
    crate::schema_option_or_unreachable(
        ParentAccountId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_ACCOUNT_ID,
    )
}

fn family_id(value: &str) -> FamilyId {
    crate::schema_option_or_unreachable(
        FamilyId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_FAMILY_ID,
    )
}

fn parent_device_id(value: &str) -> ParentDeviceId {
    crate::schema_option_or_unreachable(
        ParentDeviceId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_DEVICE_ID,
    )
}

fn parent_device_label(value: &str) -> ParentDeviceLabel {
    crate::schema_option_or_unreachable(
        ParentDeviceLabel::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_DEVICE_LABEL,
    )
}

fn parent_actor_id(value: &str) -> ParentActorId {
    crate::schema_option_or_unreachable(
        ParentActorId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_ACTOR_ID,
    )
}

fn policy_version(value: &str) -> ParentPolicyVersion {
    crate::schema_option_or_unreachable(
        ParentPolicyVersion::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_POLICY_VERSION,
    )
}

fn evidence_id(value: &str) -> ParentEvidenceReferenceId {
    crate::schema_option_or_unreachable(
        ParentEvidenceReferenceId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_EVIDENCE_ID,
    )
}

fn parent_action_id(value: &str) -> ParentActionReferenceId {
    crate::schema_option_or_unreachable(
        ParentActionReferenceId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_PARENT_ACTION_ID,
    )
}

fn timestamp(value: &str) -> ParentTimestamp {
    crate::schema_option_or_unreachable(
        ParentTimestamp::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_TIMESTAMP,
    )
}

fn request_id(value: &str) -> ReportQueryCustodyRequestId {
    crate::schema_option_or_unreachable(
        ReportQueryCustodyRequestId::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_REQUEST_ID,
    )
}

fn query_cursor(value: &str) -> ReportQueryCustodyQueryCursor {
    crate::schema_option_or_unreachable(
        ReportQueryCustodyQueryCursor::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_QUERY_CURSOR,
    )
}

fn cursor_ref(value: impl Into<String>) -> ReportQueryCustodyCursorRef {
    crate::schema_option_or_unreachable(
        ReportQueryCustodyCursorRef::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_CURSOR_REF,
    )
}

fn sort_key(value: &str) -> ReportQueryCustodySortKey {
    crate::schema_option_or_unreachable(
        ReportQueryCustodySortKey::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_SORT_KEY,
    )
}

fn source_ref(value: impl Into<String>) -> ReportQueryCustodySourceRef {
    crate::schema_option_or_unreachable(
        ReportQueryCustodySourceRef::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_SOURCE_REF,
    )
}

fn conflict_ref(value: &str) -> ReportQueryCustodyConflictRef {
    crate::schema_option_or_unreachable(
        ReportQueryCustodyConflictRef::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_CONFLICT_REF,
    )
}

fn deleted_source_ref(value: &str) -> ReportQueryCustodyDeletedSourceRef {
    crate::schema_option_or_unreachable(
        ReportQueryCustodyDeletedSourceRef::parse(value),
        REPORT_QUERY_CUSTODY_EXPECT_DELETED_SOURCE_REF,
    )
}
