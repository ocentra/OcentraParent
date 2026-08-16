use super::{
    NetworkPlatformClaimEntry, NetworkPlatformClaimManualFollowup, NetworkPlatformClaimState,
};

pub(super) fn manual_followups(
    entries: &[NetworkPlatformClaimEntry],
) -> Vec<NetworkPlatformClaimManualFollowup> {
    entries
        .iter()
        .filter(|entry| !entry.missing_required_artifacts.is_empty())
        .map(|entry| NetworkPlatformClaimManualFollowup {
            target: entry.target,
            missing_required_artifacts: entry.missing_required_artifacts.clone(),
        })
        .collect()
}

pub(super) fn count_state(
    entries: &[NetworkPlatformClaimEntry],
    state: NetworkPlatformClaimState,
) -> usize {
    entries
        .iter()
        .filter(|entry| entry.claim_state == state)
        .count()
}
