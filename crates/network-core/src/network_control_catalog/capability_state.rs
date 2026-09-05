use super::{NetworkControlCapabilityState, NetworkControlEffectStatus};

pub fn capability_state_for(
    effect_status: NetworkControlEffectStatus,
) -> NetworkControlCapabilityState {
    match effect_status {
        NetworkControlEffectStatus::AlreadyRepresented => NetworkControlCapabilityState::Available,
        NetworkControlEffectStatus::NeedsEffectWiring => NetworkControlCapabilityState::Degraded,
        NetworkControlEffectStatus::ManualRequired => NetworkControlCapabilityState::ManualRequired,
        NetworkControlEffectStatus::PermissionRequired => {
            NetworkControlCapabilityState::PermissionRequired
        }
        NetworkControlEffectStatus::PermissionLimited => {
            NetworkControlCapabilityState::PermissionLimited
        }
        NetworkControlEffectStatus::FutureGap => NetworkControlCapabilityState::FutureGap,
        NetworkControlEffectStatus::Degraded => NetworkControlCapabilityState::Degraded,
        NetworkControlEffectStatus::Unavailable => NetworkControlCapabilityState::Unavailable,
        NetworkControlEffectStatus::ProofRequired => NetworkControlCapabilityState::Protected,
    }
}
