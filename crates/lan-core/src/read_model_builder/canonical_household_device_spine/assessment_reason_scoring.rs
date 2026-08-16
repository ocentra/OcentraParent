use super::super::assessment_reasons::MergeDecisionReason;

pub(super) fn merge_score(reasons: &[MergeDecisionReason]) -> u16 {
    reasons.iter().map(|reason| score(*reason)).sum()
}

pub(super) fn is_manual_required(reason: MergeDecisionReason) -> bool {
    matches!(
        reason,
        MergeDecisionReason::SharedIpAddress
            | MergeDecisionReason::SharedHostname
            | MergeDecisionReason::SharedVendor
            | MergeDecisionReason::SharedDeviceType
    )
}

fn score(reason: MergeDecisionReason) -> u16 {
    match reason {
        MergeDecisionReason::SameCanonicalDeviceId => 120,
        MergeDecisionReason::SharedInstallId | MergeDecisionReason::SharedPairingId => 110,
        MergeDecisionReason::SharedStableMac => 100,
        MergeDecisionReason::SharedMdnsInstanceName | MergeDecisionReason::SharedSsdpUdn => 95,
        MergeDecisionReason::SharedLocalServiceIdentityAnchor => 90,
        MergeDecisionReason::SharedIpAddress => 25,
        MergeDecisionReason::SharedHostname => 20,
        MergeDecisionReason::SharedVendor => 10,
        MergeDecisionReason::SharedDeviceType => 12,
        MergeDecisionReason::ConflictingOcentraDeviceId
        | MergeDecisionReason::ConflictingChildProfileId => 0,
    }
}
