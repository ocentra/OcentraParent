use super::{NetworkEvidencePolicyAction, NetworkEvidencePolicyMode};
use crate::dns::types::NetworkEvidenceGrade;

pub(super) fn mapped_mode_and_action(
    grade: NetworkEvidenceGrade,
    requested_action: NetworkEvidencePolicyAction,
) -> (NetworkEvidencePolicyMode, NetworkEvidencePolicyAction) {
    match grade {
        NetworkEvidenceGrade::A => (
            NetworkEvidencePolicyMode::DryRun,
            dry_run_action(requested_action),
        ),
        NetworkEvidenceGrade::B => probable_mode_and_action(requested_action),
        NetworkEvidenceGrade::C => (
            NetworkEvidencePolicyMode::ParentReview,
            NetworkEvidencePolicyAction::AskParent,
        ),
        NetworkEvidenceGrade::D => (
            NetworkEvidencePolicyMode::ObserveOnly,
            NetworkEvidencePolicyAction::None,
        ),
    }
}

fn dry_run_action(requested_action: NetworkEvidencePolicyAction) -> NetworkEvidencePolicyAction {
    match requested_action {
        NetworkEvidencePolicyAction::None => NetworkEvidencePolicyAction::Monitor,
        action => action,
    }
}

fn probable_mode_and_action(
    requested_action: NetworkEvidencePolicyAction,
) -> (NetworkEvidencePolicyMode, NetworkEvidencePolicyAction) {
    match requested_action {
        NetworkEvidencePolicyAction::Block | NetworkEvidencePolicyAction::Limit => (
            NetworkEvidencePolicyMode::ParentReview,
            NetworkEvidencePolicyAction::AskParent,
        ),
        NetworkEvidencePolicyAction::None => (
            NetworkEvidencePolicyMode::DryRun,
            NetworkEvidencePolicyAction::Monitor,
        ),
        action => (NetworkEvidencePolicyMode::DryRun, action),
    }
}
