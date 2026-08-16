use std::io;

use ocentra_lan_core::lan_mdns_advertiser::{
    child_instance, parent_instance, send_advertisements, send_goodbye,
    LanMdnsAdvertisementInstance, LanMdnsPacketSink,
};
use ocentra_lan_core::lan_pairing::{
    LanMdnsAdvertisementLifecycleAction, LanMdnsAdvertisementPlatformSupport,
};

use crate::lan_pairing::LanPairingRuntime;

use super::{
    lifecycle_state_for_action, support_state_for_platform, LanMdnsAdvertisementSyncState,
};

pub(super) fn sync_mdns_advertisements_with_sink(
    runtime: &LanPairingRuntime,
    sync_state: &mut LanMdnsAdvertisementSyncState,
    platform_support: LanMdnsAdvertisementPlatformSupport,
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    let parent_action = LanPairingRuntime::mdns_advertisement_lifecycle(
        runtime.signed_child_agent_family_hash.is_some(),
        sync_state.parent.is_some(),
        platform_support,
    )
    .lifecycle_action;
    sync_advertisement_slot(
        &mut sync_state.parent,
        parent_action,
        runtime
            .build_parent_mdns_advertisement(
                lifecycle_state_for_action(parent_action),
                support_state_for_platform(platform_support),
            )
            .as_ref()
            .map(parent_instance),
        sink,
    )?;

    let child_action = LanPairingRuntime::mdns_advertisement_lifecycle(
        runtime.local_child_device_id.is_some() && runtime.signed_child_agent_family_hash.is_some(),
        sync_state.child.is_some(),
        platform_support,
    )
    .lifecycle_action;
    sync_advertisement_slot(
        &mut sync_state.child,
        child_action,
        runtime
            .build_child_mdns_advertisement(
                lifecycle_state_for_action(child_action),
                support_state_for_platform(platform_support),
            )
            .as_ref()
            .map(child_instance),
        sink,
    )
}

fn sync_advertisement_slot(
    current: &mut Option<LanMdnsAdvertisementInstance>,
    lifecycle_action: LanMdnsAdvertisementLifecycleAction,
    next_instance: Option<LanMdnsAdvertisementInstance>,
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    match lifecycle_action {
        LanMdnsAdvertisementLifecycleAction::Start
        | LanMdnsAdvertisementLifecycleAction::Update => {
            sync_started_advertisement(current, next_instance, sink)
        }
        LanMdnsAdvertisementLifecycleAction::Stop
        | LanMdnsAdvertisementLifecycleAction::Degraded => stop_advertisement(current, sink),
    }
}

fn sync_started_advertisement(
    current: &mut Option<LanMdnsAdvertisementInstance>,
    next_instance: Option<LanMdnsAdvertisementInstance>,
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    if let Some(instance) = next_instance {
        send_advertisements(std::slice::from_ref(&instance), sink)?;
        *current = Some(instance);
        return Ok(());
    }
    stop_advertisement(current, sink)
}

fn stop_advertisement(
    current: &mut Option<LanMdnsAdvertisementInstance>,
    sink: &dyn LanMdnsPacketSink,
) -> io::Result<()> {
    if let Some(instance) = current.take() {
        send_goodbye(std::slice::from_ref(&instance), sink)?;
    }
    Ok(())
}
