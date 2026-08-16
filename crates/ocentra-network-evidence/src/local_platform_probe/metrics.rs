use super::{
    NetworkLocalPlatformProbeHost, NetworkLocalPlatformProbeObservation,
    NetworkLocalPlatformProbeState,
};

pub(super) fn count_host(
    observations: &[NetworkLocalPlatformProbeObservation],
    host: NetworkLocalPlatformProbeHost,
) -> usize {
    observations
        .iter()
        .filter(|observation| observation.host == host)
        .count()
}

pub(super) fn count_apple_ci_unavailable(
    observations: &[NetworkLocalPlatformProbeObservation],
) -> usize {
    observations
        .iter()
        .filter(|observation| {
            matches!(
                observation.host,
                NetworkLocalPlatformProbeHost::MacOsCi | NetworkLocalPlatformProbeHost::IosCi
            ) && observation.probe_state == NetworkLocalPlatformProbeState::CiOnly
        })
        .count()
}
