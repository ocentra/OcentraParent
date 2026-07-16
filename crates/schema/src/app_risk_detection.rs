use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

macro_rules! app_risk_detection_identifier {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                (!value.trim().is_empty()).then_some(Self(value))
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

macro_rules! app_risk_detection_string_enum {
    ($name:ident { $($variant:ident = $value:literal),+ $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[repr(u8)]
        pub enum $name {
            $(#[serde(rename = $value)] $variant,)+
        }

        impl $name {
            pub const fn as_str(&self) -> &'static str {
                const VALUES: &[&str] = &[$($value),+];
                VALUES[*self as usize]
            }
        }
    };
}

app_risk_detection_identifier!(ParentEvidenceReferenceId);
app_risk_detection_identifier!(AppRiskDetectionCandidateId);
app_risk_detection_identifier!(AppRiskDetectionInventoryEntryRef);
app_risk_detection_identifier!(AppRiskDetectionIdentityRef);
app_risk_detection_identifier!(AppRiskDetectionSourceRef);
app_risk_detection_identifier!(AppRiskDetectionLocalAiDigestRef);
app_risk_detection_identifier!(AppRiskDetectionMatrixId);

pub const APP_RISK_DETECTION_SCHEMA_VERSION: &str = "v0.6";
const APP_RISK_DETECTION_PARENT_EVIDENCE_REFERENCE_ID_EXPECTATION: &str =
    "parent evidence reference id";
const APP_RISK_DETECTION_CANDIDATE_ID_EXPECTATION: &str = "app risk detection candidate id";
const APP_RISK_DETECTION_INVENTORY_ENTRY_REF_EXPECTATION: &str =
    "app risk detection inventory entry ref";
const APP_RISK_DETECTION_IDENTITY_REF_EXPECTATION: &str = "app risk detection identity ref";
const APP_RISK_DETECTION_SOURCE_REF_EXPECTATION: &str = "app risk detection source ref";
const APP_RISK_DETECTION_LOCAL_AI_DIGEST_REF_EXPECTATION: &str =
    "app risk detection local ai digest ref";
const APP_RISK_DETECTION_MATRIX_ID_EXPECTATION: &str = "app risk detection matrix id";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentEvidenceReference {
    pub evidence_reference_id: ParentEvidenceReferenceId,
    pub kind: ParentEvidenceReferenceKind,
    pub observed_at: String,
}

app_risk_detection_string_enum!(ParentEvidenceReferenceKind {
    ActivityEvent = "activity-event",
});

app_risk_detection_string_enum!(ParentPlatform {
    Windows = "windows",
    Linux = "linux",
    Macos = "macos",
    Android = "android",
    Ios = "ios",
});

app_risk_detection_string_enum!(AppRiskDetectionRiskSignal {
    VpnProxy = "vpnProxy",
    RemoteDesktop = "remoteDesktop",
    DownloadTorrent = "downloadTorrent",
    InstallerUpdater = "installerUpdater",
    AiChatbot = "aiChatbot",
    SocialVideoMessaging = "socialVideoMessaging",
    UnknownRisk = "unknownRisk",
});

app_risk_detection_string_enum!(AppRiskDetectionSourceKind {
    KnownCatalog = "knownCatalog",
    ExecutableName = "executableName",
    PublisherMetadata = "publisherMetadata",
    ExecutableHash = "executableHash",
    LocalAiDigest = "localAiDigest",
    ParentOverride = "parentOverride",
});

app_risk_detection_string_enum!(AppRiskDetectionCandidateState {
    CatalogMatch = "catalogMatch",
    HeuristicCandidate = "heuristicCandidate",
    AiCandidate = "aiCandidate",
    ParentReviewCandidate = "parentReviewCandidate",
    ParentDisplayOverride = "parentDisplayOverride",
});

app_risk_detection_string_enum!(AppRiskDetectionPublisherTrustState {
    KnownPublisher = "knownPublisher",
    UnknownPublisher = "unknownPublisher",
    MissingPublisher = "missingPublisher",
    UnverifiedPublisher = "unverifiedPublisher",
    ParentTrusted = "parentTrusted",
});

app_risk_detection_string_enum!(AppRiskDetectionPolicyCandidateAction {
    None = "none",
    Observe = "observe",
    Warn = "warn",
    AskParent = "askParent",
    ManualReview = "manualReview",
});

app_risk_detection_string_enum!(AppRiskDetectionConfidenceBand {
    High = "high",
    Medium = "medium",
    Low = "low",
    Review = "review",
});

app_risk_detection_string_enum!(AppRiskDetectionPolicyTargetKind {
    RiskApp = "risk-app",
});

app_risk_detection_string_enum!(AppRiskDetectionAskParentRouting {
    Available = "available",
    ManualReview = "manual-review",
    NotRouted = "not-routed",
});

app_risk_detection_string_enum!(AppRiskDetectionSurfaceState {
    RiskDisclosureReady = "riskdisclosure-ready",
});

app_risk_detection_string_enum!(AppRiskDetectionNoContentClaimState {
    NoContentCaptured = "no-content-captured",
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRiskDetectionParentOverride {
    pub parent_display_label: String,
    pub policy_candidate_action: AppRiskDetectionPolicyCandidateAction,
    pub raw_identity_changed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRiskDetectionSurfaceDisclosure {
    pub surface_state: AppRiskDetectionSurfaceState,
    pub confidence_percent: u8,
    pub source_evidence_count: u8,
    pub no_content_claim_state: AppRiskDetectionNoContentClaimState,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRiskDetectionCandidate {
    pub schema_version: String,
    pub candidate_id: AppRiskDetectionCandidateId,
    pub platform: ParentPlatform,
    pub inventory_entry_ref: Option<AppRiskDetectionInventoryEntryRef>,
    pub identity_ref: Option<AppRiskDetectionIdentityRef>,
    pub risk_signal: AppRiskDetectionRiskSignal,
    pub source_kind: AppRiskDetectionSourceKind,
    pub candidate_state: AppRiskDetectionCandidateState,
    pub publisher_trust_state: AppRiskDetectionPublisherTrustState,
    pub confidence: f64,
    pub confidence_band: AppRiskDetectionConfidenceBand,
    pub evidence_references: Vec<ParentEvidenceReference>,
    pub source_refs: Vec<AppRiskDetectionSourceRef>,
    pub local_ai_digest_ref: Option<AppRiskDetectionLocalAiDigestRef>,
    pub parent_override: Option<AppRiskDetectionParentOverride>,
    pub policy_candidate_action: AppRiskDetectionPolicyCandidateAction,
    pub policy_target_kind: AppRiskDetectionPolicyTargetKind,
    pub ask_parent_routing: AppRiskDetectionAskParentRouting,
    pub not_direct_enforcement: bool,
    pub no_content_claim: bool,
    pub surface_disclosure: AppRiskDetectionSurfaceDisclosure,
    pub last_checked_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRiskDetectionMatrix {
    pub schema_version: String,
    pub matrix_id: AppRiskDetectionMatrixId,
    pub generated_at: String,
    pub candidates: Vec<AppRiskDetectionCandidate>,
}

const GENERATED_AT: &str = "2026-06-03T10:55:00.000Z";

pub fn sample_app_risk_detection_matrix() -> AppRiskDetectionMatrix {
    AppRiskDetectionMatrix {
        schema_version: APP_RISK_DETECTION_SCHEMA_VERSION.to_owned(),
        matrix_id: app_risk_detection_matrix_id("app-riskdetection-proof-matrix"),
        generated_at: GENERATED_AT.to_owned(),
        candidates: vec![
            known_catalog(
                "known-vpn-proxy-risk",
                AppRiskDetectionRiskSignal::VpnProxy,
                0.94,
                AppRiskDetectionPolicyCandidateAction::Warn,
            ),
            known_catalog(
                "known-remote-desktop-risk",
                AppRiskDetectionRiskSignal::RemoteDesktop,
                0.92,
                AppRiskDetectionPolicyCandidateAction::AskParent,
            ),
            known_catalog(
                "known-download-torrent-risk",
                AppRiskDetectionRiskSignal::DownloadTorrent,
                0.9,
                AppRiskDetectionPolicyCandidateAction::Warn,
            ),
            known_catalog(
                "known-ai-chatbot-risk",
                AppRiskDetectionRiskSignal::AiChatbot,
                0.86,
                AppRiskDetectionPolicyCandidateAction::ManualReview,
            ),
            heuristic_candidate(
                "unknown-vpn-name-candidate",
                AppRiskDetectionRiskSignal::VpnProxy,
                AppRiskDetectionSourceKind::ExecutableName,
                0.42,
            ),
            heuristic_candidate(
                "unknown-publisher-hash-candidate",
                AppRiskDetectionRiskSignal::UnknownRisk,
                AppRiskDetectionSourceKind::ExecutableHash,
                0.38,
            ),
            ai_digest_candidate(),
            parent_override_candidate(),
        ],
    }
}

pub fn required_app_risk_detection_source_kinds() -> &'static [AppRiskDetectionSourceKind] {
    &[
        AppRiskDetectionSourceKind::KnownCatalog,
        AppRiskDetectionSourceKind::ExecutableName,
        AppRiskDetectionSourceKind::PublisherMetadata,
        AppRiskDetectionSourceKind::ExecutableHash,
        AppRiskDetectionSourceKind::LocalAiDigest,
        AppRiskDetectionSourceKind::ParentOverride,
    ]
}

fn known_catalog(
    candidate_id: &str,
    risk_signal: AppRiskDetectionRiskSignal,
    confidence: f64,
    policy_candidate_action: AppRiskDetectionPolicyCandidateAction,
) -> AppRiskDetectionCandidate {
    risk_candidate(RiskCandidateInput {
        candidate_id: candidate_id.to_string(),
        risk_signal,
        source_kind: AppRiskDetectionSourceKind::KnownCatalog,
        publisher_trust_state: AppRiskDetectionPublisherTrustState::KnownPublisher,
        confidence,
        confidence_band: AppRiskDetectionConfidenceBand::High,
        policy_candidate_action,
        inventory_entry_ref: Some(app_risk_detection_inventory_entry_ref(format!(
            "inventory-{candidate_id}"
        ))),
        identity_ref: Some(app_risk_detection_identity_ref(format!(
            "identity-{candidate_id}"
        ))),
        source_refs: Some(vec![app_risk_detection_source_ref(format!(
            "source-{candidate_id}"
        ))]),
        local_ai_digest_ref: None,
        parent_override: None,
    })
}

fn heuristic_candidate(
    candidate_id: &str,
    risk_signal: AppRiskDetectionRiskSignal,
    source_kind: AppRiskDetectionSourceKind,
    confidence: f64,
) -> AppRiskDetectionCandidate {
    risk_candidate(RiskCandidateInput {
        candidate_id: candidate_id.to_string(),
        risk_signal,
        source_kind,
        publisher_trust_state: AppRiskDetectionPublisherTrustState::UnknownPublisher,
        confidence,
        confidence_band: AppRiskDetectionConfidenceBand::Review,
        policy_candidate_action: AppRiskDetectionPolicyCandidateAction::ManualReview,
        inventory_entry_ref: None,
        identity_ref: None,
        source_refs: Some(vec![app_risk_detection_source_ref(format!(
            "source-{candidate_id}"
        ))]),
        local_ai_digest_ref: None,
        parent_override: None,
    })
}

fn ai_digest_candidate() -> AppRiskDetectionCandidate {
    risk_candidate(RiskCandidateInput {
        candidate_id: "local-ai-social-video-messaging-risk".to_string(),
        risk_signal: AppRiskDetectionRiskSignal::SocialVideoMessaging,
        source_kind: AppRiskDetectionSourceKind::LocalAiDigest,
        publisher_trust_state: AppRiskDetectionPublisherTrustState::KnownPublisher,
        confidence: 0.73,
        confidence_band: AppRiskDetectionConfidenceBand::Medium,
        policy_candidate_action: AppRiskDetectionPolicyCandidateAction::AskParent,
        inventory_entry_ref: None,
        identity_ref: None,
        source_refs: Some(vec![app_risk_detection_source_ref(
            "source-local-ai-social-video-messaging-risk",
        )]),
        local_ai_digest_ref: Some(app_risk_detection_local_ai_digest_ref(
            "local-ai-digest-social-video-messaging",
        )),
        parent_override: None,
    })
}

fn parent_override_candidate() -> AppRiskDetectionCandidate {
    risk_candidate(RiskCandidateInput {
        candidate_id: "parent-display-override-ai-tool".to_string(),
        risk_signal: AppRiskDetectionRiskSignal::AiChatbot,
        source_kind: AppRiskDetectionSourceKind::ParentOverride,
        publisher_trust_state: AppRiskDetectionPublisherTrustState::ParentTrusted,
        confidence: 0.8,
        confidence_band: AppRiskDetectionConfidenceBand::Medium,
        policy_candidate_action: AppRiskDetectionPolicyCandidateAction::Observe,
        inventory_entry_ref: None,
        identity_ref: None,
        source_refs: Some(vec![app_risk_detection_source_ref(
            "source-parent-display-override-ai-tool",
        )]),
        local_ai_digest_ref: None,
        parent_override: Some(AppRiskDetectionParentOverride {
            parent_display_label: "Homework AI tool".to_string(),
            policy_candidate_action: AppRiskDetectionPolicyCandidateAction::Observe,
            raw_identity_changed: false,
        }),
    })
}

struct RiskCandidateInput {
    candidate_id: String,
    risk_signal: AppRiskDetectionRiskSignal,
    source_kind: AppRiskDetectionSourceKind,
    publisher_trust_state: AppRiskDetectionPublisherTrustState,
    confidence: f64,
    confidence_band: AppRiskDetectionConfidenceBand,
    policy_candidate_action: AppRiskDetectionPolicyCandidateAction,
    inventory_entry_ref: Option<AppRiskDetectionInventoryEntryRef>,
    identity_ref: Option<AppRiskDetectionIdentityRef>,
    source_refs: Option<Vec<AppRiskDetectionSourceRef>>,
    local_ai_digest_ref: Option<AppRiskDetectionLocalAiDigestRef>,
    parent_override: Option<AppRiskDetectionParentOverride>,
}

fn risk_candidate(input: RiskCandidateInput) -> AppRiskDetectionCandidate {
    let RiskCandidateInput {
        candidate_id,
        risk_signal,
        source_kind,
        publisher_trust_state,
        confidence,
        confidence_band,
        policy_candidate_action,
        inventory_entry_ref,
        identity_ref,
        source_refs,
        local_ai_digest_ref,
        parent_override,
    } = input;

    const CANDIDATE_STATE_BY_SOURCE_KIND: [AppRiskDetectionCandidateState; 6] = [
        AppRiskDetectionCandidateState::CatalogMatch,
        AppRiskDetectionCandidateState::HeuristicCandidate,
        AppRiskDetectionCandidateState::HeuristicCandidate,
        AppRiskDetectionCandidateState::HeuristicCandidate,
        AppRiskDetectionCandidateState::AiCandidate,
        AppRiskDetectionCandidateState::ParentDisplayOverride,
    ];
    const ASK_PARENT_ROUTING_BY_ACTION: [AppRiskDetectionAskParentRouting; 5] = [
        AppRiskDetectionAskParentRouting::NotRouted,
        AppRiskDetectionAskParentRouting::NotRouted,
        AppRiskDetectionAskParentRouting::NotRouted,
        AppRiskDetectionAskParentRouting::Available,
        AppRiskDetectionAskParentRouting::ManualReview,
    ];

    AppRiskDetectionCandidate {
        schema_version: APP_RISK_DETECTION_SCHEMA_VERSION.to_owned(),
        candidate_id: app_risk_detection_candidate_id(candidate_id.clone()),
        platform: ParentPlatform::Windows,
        inventory_entry_ref,
        identity_ref,
        risk_signal,
        source_kind,
        candidate_state: CANDIDATE_STATE_BY_SOURCE_KIND[source_kind as usize],
        publisher_trust_state,
        confidence,
        confidence_band,
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: parent_evidence_reference_id(format!("evidence-{candidate_id}")),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: GENERATED_AT.to_owned(),
        }],
        source_refs: source_refs.unwrap_or_else(|| {
            vec![app_risk_detection_source_ref(format!(
                "source-{candidate_id}"
            ))]
        }),
        local_ai_digest_ref,
        parent_override,
        policy_candidate_action,
        policy_target_kind: AppRiskDetectionPolicyTargetKind::RiskApp,
        ask_parent_routing: ASK_PARENT_ROUTING_BY_ACTION[policy_candidate_action as usize],
        not_direct_enforcement: true,
        no_content_claim: true,
        surface_disclosure: AppRiskDetectionSurfaceDisclosure {
            surface_state: AppRiskDetectionSurfaceState::RiskDisclosureReady,
            confidence_percent: (confidence * 100.0).round() as u8,
            source_evidence_count: 1,
            no_content_claim_state: AppRiskDetectionNoContentClaimState::NoContentCaptured,
        },
        last_checked_at: GENERATED_AT.to_owned(),
    }
}

fn parent_evidence_reference_id(value: impl Into<String>) -> ParentEvidenceReferenceId {
    crate::schema_option_or_unreachable(
        ParentEvidenceReferenceId::parse(value),
        APP_RISK_DETECTION_PARENT_EVIDENCE_REFERENCE_ID_EXPECTATION,
    )
}

fn app_risk_detection_candidate_id(value: impl Into<String>) -> AppRiskDetectionCandidateId {
    crate::schema_option_or_unreachable(
        AppRiskDetectionCandidateId::parse(value),
        APP_RISK_DETECTION_CANDIDATE_ID_EXPECTATION,
    )
}

fn app_risk_detection_inventory_entry_ref(
    value: impl Into<String>,
) -> AppRiskDetectionInventoryEntryRef {
    crate::schema_option_or_unreachable(
        AppRiskDetectionInventoryEntryRef::parse(value),
        APP_RISK_DETECTION_INVENTORY_ENTRY_REF_EXPECTATION,
    )
}

fn app_risk_detection_identity_ref(value: impl Into<String>) -> AppRiskDetectionIdentityRef {
    crate::schema_option_or_unreachable(
        AppRiskDetectionIdentityRef::parse(value),
        APP_RISK_DETECTION_IDENTITY_REF_EXPECTATION,
    )
}

fn app_risk_detection_source_ref(value: impl Into<String>) -> AppRiskDetectionSourceRef {
    crate::schema_option_or_unreachable(
        AppRiskDetectionSourceRef::parse(value),
        APP_RISK_DETECTION_SOURCE_REF_EXPECTATION,
    )
}

fn app_risk_detection_local_ai_digest_ref(
    value: impl Into<String>,
) -> AppRiskDetectionLocalAiDigestRef {
    crate::schema_option_or_unreachable(
        AppRiskDetectionLocalAiDigestRef::parse(value),
        APP_RISK_DETECTION_LOCAL_AI_DIGEST_REF_EXPECTATION,
    )
}

fn app_risk_detection_matrix_id(value: impl Into<String>) -> AppRiskDetectionMatrixId {
    crate::schema_option_or_unreachable(
        AppRiskDetectionMatrixId::parse(value),
        APP_RISK_DETECTION_MATRIX_ID_EXPECTATION,
    )
}
