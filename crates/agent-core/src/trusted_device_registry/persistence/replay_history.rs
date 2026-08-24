use super::super::TrustedDeviceRegistry;

pub(super) fn merge_challenge_ids(
    current: &TrustedDeviceRegistry,
    persisted: &mut TrustedDeviceRegistry,
) {
    persisted.merge_accepted_challenge_ids(current.accepted_challenge_ids.iter().cloned());
}
