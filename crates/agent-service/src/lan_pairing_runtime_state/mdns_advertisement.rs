use std::io;
use std::time::Duration;

use ocentra_lan_core::lan_mdns_advertiser::{
    child_instance, current_platform_support, derive_child_advertisement_id,
    derive_parent_advertisement_id, parent_instance, send_advertisements, send_goodbye,
    LanMdnsAdvertisementInstance, LanMdnsPacketSink, UdpMulticastMdnsPacketSink,
};
use ocentra_lan_core::lan_pairing::{
    LanMdnsAdvertisementLifecycleAction, LanMdnsAdvertisementPlatformSupport,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanChildMdnsAdvertisementInput, LanMdnsAdvertisementLifecycleState,
    LanMdnsAdvertisementSupportState, LanParentMdnsAdvertisement,
};

use crate::lan_pairing::LanPairingRuntime;

const MDNS_ADVERTISEMENT_INTERVAL: Duration = Duration::from_secs(30);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LanMdnsAdvertisementSyncState {
    pub(crate) parent: Option<LanMdnsAdvertisementInstance>,
    pub(crate) child: Option<LanMdnsAdvertisementInstance>,
}

pub(crate) fn spawn_lan_mdns_advertisement_runtime(runtime: LanPairingRuntime) {
    tokio::spawn(async move {
        let sink = UdpMulticastMdnsPacketSink;
        let mut interval = tokio::time::interval(MDNS_ADVERTISEMENT_INTERVAL);
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let mut sync_state = LanMdnsAdvertisementSyncState::default();
        let _ = runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            current_platform_support(),
            &sink,
        );
        loop {
            interval.tick().await;
            let _ = runtime.sync_mdns_advertisements_with_sink(
                &mut sync_state,
                current_platform_support(),
                &sink,
            );
        }
    });
}

impl LanPairingRuntime {
    pub(crate) fn sync_mdns_advertisements_with_sink(
        &self,
        sync_state: &mut LanMdnsAdvertisementSyncState,
        platform_support: LanMdnsAdvertisementPlatformSupport,
        sink: &dyn LanMdnsPacketSink,
    ) -> io::Result<()> {
        self.sync_parent_mdns_advertisement(sync_state, platform_support, sink)?;
        self.sync_child_mdns_advertisement(sync_state, platform_support, sink)?;
        Ok(())
    }

    fn sync_parent_mdns_advertisement(
        &self,
        sync_state: &mut LanMdnsAdvertisementSyncState,
        platform_support: LanMdnsAdvertisementPlatformSupport,
        sink: &dyn LanMdnsPacketSink,
    ) -> io::Result<()> {
        let support_state = support_state_for_platform(platform_support);
        let decision = Self::mdns_advertisement_lifecycle(
            self.signed_child_agent_family_hash.is_some(),
            sync_state.parent.is_some(),
            platform_support,
        );
        match decision.lifecycle_action {
            LanMdnsAdvertisementLifecycleAction::Start
            | LanMdnsAdvertisementLifecycleAction::Update => {
                if let Some(advertisement) = self.build_parent_mdns_advertisement(
                    lifecycle_state_for_action(decision.lifecycle_action),
                    support_state,
                ) {
                    let instance = parent_instance(&advertisement);
                    send_advertisements(std::slice::from_ref(&instance), sink)?;
                    sync_state.parent = Some(instance);
                } else if let Some(instance) = sync_state.parent.take() {
                    send_goodbye(std::slice::from_ref(&instance), sink)?;
                }
                Ok(())
            }
            LanMdnsAdvertisementLifecycleAction::Stop
            | LanMdnsAdvertisementLifecycleAction::Degraded => {
                if let Some(instance) = sync_state.parent.take() {
                    send_goodbye(std::slice::from_ref(&instance), sink)?;
                }
                Ok(())
            }
        }
    }

    fn sync_child_mdns_advertisement(
        &self,
        sync_state: &mut LanMdnsAdvertisementSyncState,
        platform_support: LanMdnsAdvertisementPlatformSupport,
        sink: &dyn LanMdnsPacketSink,
    ) -> io::Result<()> {
        let support_state = support_state_for_platform(platform_support);
        let decision = Self::mdns_advertisement_lifecycle(
            self.local_child_device_id.is_some() && self.signed_child_agent_family_hash.is_some(),
            sync_state.child.is_some(),
            platform_support,
        );
        match decision.lifecycle_action {
            LanMdnsAdvertisementLifecycleAction::Start
            | LanMdnsAdvertisementLifecycleAction::Update => {
                if let Some(advertisement) = self.build_child_mdns_advertisement(
                    lifecycle_state_for_action(decision.lifecycle_action),
                    support_state,
                ) {
                    let instance = child_instance(&advertisement);
                    send_advertisements(std::slice::from_ref(&instance), sink)?;
                    sync_state.child = Some(instance);
                } else if let Some(instance) = sync_state.child.take() {
                    send_goodbye(std::slice::from_ref(&instance), sink)?;
                }
                Ok(())
            }
            LanMdnsAdvertisementLifecycleAction::Stop
            | LanMdnsAdvertisementLifecycleAction::Degraded => {
                if let Some(instance) = sync_state.child.take() {
                    send_goodbye(std::slice::from_ref(&instance), sink)?;
                }
                Ok(())
            }
        }
    }

    fn build_parent_mdns_advertisement(
        &self,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Option<LanParentMdnsAdvertisement> {
        let family_hash = self.signed_child_agent_family_hash.clone()?;
        LanParentMdnsAdvertisement::new(
            derive_parent_advertisement_id(family_hash.as_str()),
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            family_hash,
            self.mdns_pairing_state(),
            lifecycle_state,
            support_state,
        )
        .ok()
    }

    fn build_child_mdns_advertisement(
        &self,
        lifecycle_state: LanMdnsAdvertisementLifecycleState,
        support_state: LanMdnsAdvertisementSupportState,
    ) -> Option<LanChildMdnsAdvertisement> {
        let family_hash = self.signed_child_agent_family_hash.clone()?;
        let opaque_device_id = self.local_child_device_id.clone()?;
        LanChildMdnsAdvertisement::new(LanChildMdnsAdvertisementInput {
            advertisement_id: derive_child_advertisement_id(
                family_hash.as_str(),
                opaque_device_id.as_str(),
            ),
            opaque_device_id,
            protocol_version: constants::lan_pairing::SCHEMA_VERSION_TEXT.to_string(),
            family_hash,
            platform: std::env::consts::OS.to_string(),
            agent_version: env!("CARGO_PKG_VERSION").to_string(),
            pairing_state: self.mdns_pairing_state(),
            lifecycle_state,
            support_state,
        })
        .ok()
    }
}

fn support_state_for_platform(
    platform_support: LanMdnsAdvertisementPlatformSupport,
) -> LanMdnsAdvertisementSupportState {
    match platform_support {
        LanMdnsAdvertisementPlatformSupport::Supported => {
            LanMdnsAdvertisementSupportState::Supported
        }
        LanMdnsAdvertisementPlatformSupport::Degraded => LanMdnsAdvertisementSupportState::Degraded,
        LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform => {
            LanMdnsAdvertisementSupportState::UnsupportedPlatform
        }
    }
}

fn lifecycle_state_for_action(
    lifecycle_action: LanMdnsAdvertisementLifecycleAction,
) -> LanMdnsAdvertisementLifecycleState {
    match lifecycle_action {
        LanMdnsAdvertisementLifecycleAction::Start => LanMdnsAdvertisementLifecycleState::Start,
        LanMdnsAdvertisementLifecycleAction::Update => LanMdnsAdvertisementLifecycleState::Update,
        LanMdnsAdvertisementLifecycleAction::Stop => LanMdnsAdvertisementLifecycleState::Stop,
        LanMdnsAdvertisementLifecycleAction::Degraded => {
            LanMdnsAdvertisementLifecycleState::Degraded
        }
    }
}
