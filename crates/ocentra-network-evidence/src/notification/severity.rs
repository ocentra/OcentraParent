use super::NetworkParentNotificationSeverity;
use crate::{NetworkEvidencePolicyAction, NetworkEvidencePolicyMapping, NetworkEvidencePolicyMode};

pub(super) fn severity_for(
    mapping: &NetworkEvidencePolicyMapping,
) -> NetworkParentNotificationSeverity {
    match mapping.mode {
        NetworkEvidencePolicyMode::ObserveOnly => NetworkParentNotificationSeverity::Info,
        NetworkEvidencePolicyMode::ParentReview => NetworkParentNotificationSeverity::Review,
        NetworkEvidencePolicyMode::DryRun => dry_run_severity(mapping.mapped_action),
    }
}

fn dry_run_severity(action: NetworkEvidencePolicyAction) -> NetworkParentNotificationSeverity {
    match action {
        NetworkEvidencePolicyAction::Block | NetworkEvidencePolicyAction::Limit => {
            NetworkParentNotificationSeverity::Urgent
        }
        NetworkEvidencePolicyAction::WarnChild | NetworkEvidencePolicyAction::Monitor => {
            NetworkParentNotificationSeverity::Warning
        }
        NetworkEvidencePolicyAction::AskParent => NetworkParentNotificationSeverity::Review,
        NetworkEvidencePolicyAction::None => NetworkParentNotificationSeverity::Info,
    }
}
