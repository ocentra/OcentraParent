#![forbid(unsafe_code)]

use super::{
    LanMdnsAdvertisementLifecycleAction, LanMdnsAdvertisementLifecycleDecision,
    LanMdnsAdvertisementLifecycleInput, LanMdnsAdvertisementPlatformSupport,
};

pub(super) fn evaluate_lan_mdns_advertisement_lifecycle(
    input: LanMdnsAdvertisementLifecycleInput,
) -> LanMdnsAdvertisementLifecycleDecision {
    LanMdnsAdvertisementLifecycleDecision {
        lifecycle_action: lifecycle_action(input),
        hint_only: true,
        platform_support: input.platform_support,
    }
}

fn lifecycle_action(
    input: LanMdnsAdvertisementLifecycleInput,
) -> LanMdnsAdvertisementLifecycleAction {
    if !input.desired_present {
        return LanMdnsAdvertisementLifecycleAction::Stop;
    }
    match input.platform_support {
        LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform
        | LanMdnsAdvertisementPlatformSupport::Degraded => {
            LanMdnsAdvertisementLifecycleAction::Degraded
        }
        LanMdnsAdvertisementPlatformSupport::Supported => {
            if input.running {
                LanMdnsAdvertisementLifecycleAction::Update
            } else {
                LanMdnsAdvertisementLifecycleAction::Start
            }
        }
    }
}
