use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanParentMdnsAdvertisement,
};

use super::advertisement::is_selected_service_type;
use super::packet::parse_mdns_packet;
use super::{MdnsDnsSdDiscovery, MdnsDnsSdServiceInstance, MdnsDnsSdSrvRecord, MdnsDnsSdTxtRecord};

#[path = "accumulator_detail.rs"]
mod accumulator_detail;

use accumulator_detail::{append_first_mdns_summary_detail, populate_instance_details};

mod merge;

pub fn parse_mdns_packets(packets: &[Vec<u8>], observed_at: String) -> Option<MdnsDnsSdDiscovery> {
    let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
    for packet in packets {
        if let Some(parsed_packet) = parse_mdns_packet(packet) {
            accumulator.merge(parsed_packet);
        }
    }
    Some(accumulator.finalize(observed_at))
}

pub fn passive_mdns_dns_sd_summary(payload: &[u8]) -> Option<String> {
    let discovery = discovery_from_single_packet(payload)?;
    if discovery.service_types.is_empty() && discovery.service_instances.is_empty() {
        return None;
    }
    let mut summary = format!(
        "mDNS DNS-SD packet: {} service type(s), {} instance(s)",
        discovery.service_types.len(),
        discovery.service_instances.len()
    );
    append_first_mdns_summary_detail(&mut summary, &discovery);
    Some(summary)
}

pub fn passive_mdns_dns_sd_device_id(payload: &[u8]) -> Option<String> {
    let discovery = discovery_from_single_packet(payload)?;
    discovery.service_instances.iter().find_map(|instance| {
        instance
            .child_advertisement
            .as_ref()
            .map(|advertisement| advertisement.opaque_device_id.clone())
    })
}

pub fn discovery_from_single_packet(payload: &[u8]) -> Option<MdnsDnsSdDiscovery> {
    let parsed_packet = parse_mdns_packet(payload)?;
    let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
    accumulator.merge(parsed_packet);
    Some(accumulator.finalize(String::new()))
}

#[derive(Default)]
pub struct MdnsDnsSdDiscoveryAccumulator {
    pub(super) service_types: BTreeSet<String>,
    pub(super) instances: BTreeMap<(String, String), MdnsDnsSdServiceAccumulator>,
    pub(super) host_addresses: BTreeMap<String, BTreeSet<String>>,
    pub(super) srv_records: BTreeMap<String, MdnsDnsSdSrvRecord>,
    pub(super) txt_records: BTreeMap<String, Vec<MdnsDnsSdTxtRecord>>,
}

impl MdnsDnsSdDiscoveryAccumulator {
    pub fn finalize(self, observed_at: String) -> MdnsDnsSdDiscovery {
        let MdnsDnsSdDiscoveryAccumulator {
            service_types,
            instances,
            host_addresses,
            srv_records,
            txt_records,
        } = self;
        let mut service_instances = Vec::new();
        for ((service_type, instance_name), mut instance) in instances {
            if !is_selected_service_type(&service_type) {
                continue;
            }
            instance.service_type = service_type.clone();
            instance.instance_name = instance_name.clone();
            populate_instance_details(
                &host_addresses,
                &srv_records,
                &txt_records,
                &service_type,
                &instance_name,
                &mut instance,
            );
            service_instances.push(instance.into_service_instance());
        }

        MdnsDnsSdDiscovery {
            observed_at,
            service_types: service_types.into_iter().collect(),
            service_instances,
        }
    }
}

#[derive(Default)]
pub struct MdnsDnsSdServiceAccumulator {
    pub(super) service_type: String,
    pub(super) instance_name: String,
    pub(super) display_name: Option<String>,
    pub(super) target_hostname: Option<String>,
    pub(super) port: Option<u16>,
    pub(super) addresses: BTreeSet<String>,
    pub(super) txt_records: Vec<MdnsDnsSdTxtRecord>,
    pub(super) parent_advertisement: Option<LanParentMdnsAdvertisement>,
    pub(super) child_advertisement: Option<LanChildMdnsAdvertisement>,
}

impl MdnsDnsSdServiceAccumulator {
    fn into_service_instance(self) -> MdnsDnsSdServiceInstance {
        MdnsDnsSdServiceInstance {
            service_type: self.service_type,
            instance_name: self.instance_name,
            display_name: self.display_name,
            target_hostname: self.target_hostname,
            port: self.port,
            addresses: self.addresses.into_iter().collect(),
            txt_records: self.txt_records,
            parent_advertisement: self.parent_advertisement,
            child_advertisement: self.child_advertisement,
        }
    }
}
