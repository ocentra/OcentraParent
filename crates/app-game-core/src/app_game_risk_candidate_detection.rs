use ocentra_parent_agent_protocol::app_game::{
    AppGameInventoryEvidenceRow, APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
};
use serde::{Deserialize, Serialize};

use crate::app_game_category_risk_policy_routing::types::{
    AppGameCategoryProofState, AppGameCategoryRiskCandidate, AppGameCategoryRiskCandidateKind,
    AppGameCategoryRiskCandidateSource,
};
use crate::app_game_policy_target_compiler::references::AppGamePolicyEvidenceRef;
use crate::app_game_policy_target_compiler::types::AppGamePolicyCompilerRequestedAction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AppGameRiskSignal {
    VpnProxy,
    RemoteDesktop,
    DownloadTorrent,
    InstallerUpdater,
    AiChatbot,
    SocialVideoMessaging,
    UnknownRisk,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameRiskCandidateDetectionState {
    CatalogCandidate,
    InventoryCandidate,
    UnknownCandidate,
    NotRiskCandidate,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppGameRiskCandidateDetection {
    pub state: AppGameRiskCandidateDetectionState,
    pub signal: Option<AppGameRiskSignal>,
    pub candidate: Option<AppGameCategoryRiskCandidate>,
}

pub fn detect_app_game_risk_candidate(
    row: &AppGameInventoryEvidenceRow,
) -> AppGameRiskCandidateDetection {
    if let Some((category, signal)) = row.category_candidates.iter().find_map(|category| {
        risk_signal(category.category_kind.as_str()).map(|signal| (category, signal))
    }) {
        let category_proof_ref = category.evidence.iter().find_map(|evidence| {
            AppGamePolicyEvidenceRef::parse(evidence.evidence_id.clone()).ok()
        });
        let supporting_evidence_refs = row
            .evidence
            .iter()
            .filter_map(|evidence| {
                AppGamePolicyEvidenceRef::parse(evidence.evidence_id.clone()).ok()
            })
            .collect();

        return AppGameRiskCandidateDetection {
            state: if category.catalog_ref.is_some() {
                AppGameRiskCandidateDetectionState::CatalogCandidate
            } else {
                AppGameRiskCandidateDetectionState::InventoryCandidate
            },
            signal: Some(signal),
            candidate: Some(AppGameCategoryRiskCandidate {
                candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
                candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
                confidence_permille: confidence_permille(category.confidence),
                category_proof_state: category_proof_ref
                    .as_ref()
                    .map(|_| AppGameCategoryProofState::Active)
                    .unwrap_or(AppGameCategoryProofState::Missing),
                category_proof_ref,
                supporting_evidence_refs,
                ai_digest_ref: None,
                requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
            }),
        };
    }

    if row.classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS {
        return AppGameRiskCandidateDetection {
            state: AppGameRiskCandidateDetectionState::UnknownCandidate,
            signal: Some(AppGameRiskSignal::UnknownRisk),
            candidate: Some(AppGameCategoryRiskCandidate {
                candidate_kind: AppGameCategoryRiskCandidateKind::AppRisk,
                candidate_source: AppGameCategoryRiskCandidateSource::NativeInventory,
                confidence_permille: confidence_permille(row.confidence),
                category_proof_state: AppGameCategoryProofState::Missing,
                category_proof_ref: None,
                supporting_evidence_refs: row
                    .evidence
                    .iter()
                    .filter_map(|evidence| {
                        AppGamePolicyEvidenceRef::parse(evidence.evidence_id.clone()).ok()
                    })
                    .collect(),
                ai_digest_ref: None,
                requested_action: AppGamePolicyCompilerRequestedAction::AskParent,
            }),
        };
    }

    AppGameRiskCandidateDetection {
        state: AppGameRiskCandidateDetectionState::NotRiskCandidate,
        signal: None,
        candidate: None,
    }
}

fn risk_signal(category_kind: &str) -> Option<AppGameRiskSignal> {
    match category_kind {
        "vpnProxy" => Some(AppGameRiskSignal::VpnProxy),
        "remoteDesktop" => Some(AppGameRiskSignal::RemoteDesktop),
        "downloadTorrent" => Some(AppGameRiskSignal::DownloadTorrent),
        "installerUpdater" => Some(AppGameRiskSignal::InstallerUpdater),
        "aiChatbot" => Some(AppGameRiskSignal::AiChatbot),
        "socialVideoMessaging" => Some(AppGameRiskSignal::SocialVideoMessaging),
        "unknownRisk" => Some(AppGameRiskSignal::UnknownRisk),
        _ => None,
    }
}

fn confidence_permille(confidence: f64) -> u16 {
    if !confidence.is_finite() {
        return 0;
    }
    (confidence.clamp(0.0, 1.0) * 1_000.0).round() as u16
}
