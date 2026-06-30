use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanChildMdnsAdvertisement, LanParentMdnsAdvertisement,
};

use super::advertisement::{
    is_selected_service_type, parse_child_mdns_advertisement, parse_parent_mdns_advertisement,
};
use super::packet::parse_mdns_packet;
use super::text::display_name_from_instance_name;
use super::{
    MdnsDnsSdDiscovery, MdnsDnsSdPacket, MdnsDnsSdServiceInstance, MdnsDnsSdSrvRecord,
    MdnsDnsSdTxtRecord, MdnsRecordData, MDNS_SERVICE_ENUMERATION,
};

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

pub fn append_first_mdns_summary_detail(summary: &mut String, discovery: &MdnsDnsSdDiscovery) {
    if let Some(instance) = discovery.service_instances.first() {
        summary.push_str("; first service=");
        summary.push_str(&instance.service_type);
        if let Some(display_name) = instance.display_name.as_ref() {
            summary.push_str("; display=");
            summary.push_str(display_name);
        }
        if let Some(target_hostname) = instance.target_hostname.as_ref() {
            summary.push_str("; target=");
            summary.push_str(target_hostname);
        }
        if let Some(address) = instance.addresses.first() {
            summary.push_str("; address=");
            summary.push_str(address);
        }
    } else if let Some(service_type) = discovery.service_types.first() {
        summary.push_str("; first service type=");
        summary.push_str(service_type);
    }
}

#[derive(Default)]
pub struct MdnsDnsSdDiscoveryAccumulator {
    service_types: BTreeSet<String>,
    instances: BTreeMap<(String, String), MdnsDnsSdServiceAccumulator>,
    host_addresses: BTreeMap<String, BTreeSet<String>>,
    srv_records: BTreeMap<String, MdnsDnsSdSrvRecord>,
    txt_records: BTreeMap<String, Vec<MdnsDnsSdTxtRecord>>,
}

impl MdnsDnsSdDiscoveryAccumulator {
    pub fn merge(&mut self, packet: MdnsDnsSdPacket) {
        for record in packet.records {
            match record.data {
                MdnsRecordData::Ptr(target) => self.merge_ptr_record(record.name, target),
                MdnsRecordData::Srv {
                    target_hostname,
                    port,
                } => {
                    self.srv_records.insert(
                        record.name,
                        MdnsDnsSdSrvRecord {
                            target_hostname,
                            port,
                        },
                    );
                }
                MdnsRecordData::Txt(entries) => {
                    self.txt_records.insert(record.name, entries);
                }
                MdnsRecordData::A(address) | MdnsRecordData::Aaaa(address) => {
                    self.host_addresses
                        .entry(record.name.to_ascii_lowercase())
                        .or_default()
                        .insert(address);
                }
                MdnsRecordData::Unknown => {}
            }
        }
    }

    pub fn finalize(self, observed_at: String) -> MdnsDnsSdDiscovery {
        let mut service_instances = Vec::new();
        for ((service_type, instance_name), mut instance) in self.instances {
            if !is_selected_service_type(&service_type) {
                continue;
            }
            instance.service_type = service_type.clone();
            instance.instance_name = instance_name.clone();
            self.populate_instance_details(&service_type, &instance_name, &mut instance);
            service_instances.push(instance.into_service_instance());
        }

        MdnsDnsSdDiscovery {
            observed_at,
            service_types: self.service_types.into_iter().collect(),
            service_instances,
        }
    }

    fn merge_ptr_record(&mut self, record_name: String, target: String) {
        if record_name.eq_ignore_ascii_case(MDNS_SERVICE_ENUMERATION) {
            if is_selected_service_type(&target) {
                self.service_types.insert(target);
            }
            return;
        }
        if !is_selected_service_type(&record_name) {
            return;
        }
        let instance_name_key = target.clone();
        self.instances
            .entry((record_name.clone(), instance_name_key))
            .or_default()
            .instance_name = target;
    }

    fn populate_instance_details(
        &self,
        service_type: &str,
        instance_name: &str,
        instance: &mut MdnsDnsSdServiceAccumulator,
    ) {
        if let Some(display_name) = display_name_from_instance_name(instance_name, service_type) {
            instance.display_name = Some(display_name);
        }
        if let Some(srv_record) = self.srv_records.get(instance_name) {
            instance.target_hostname = srv_record.target_hostname.clone();
            instance.port = srv_record.port;
            self.extend_addresses_for_srv_target(instance, srv_record);
        }
        if let Some(entries) = self.txt_records.get(instance_name) {
            instance.txt_records = entries.clone();
            populate_mdns_advertisements(service_type, entries, instance);
        }
    }

    fn extend_addresses_for_srv_target(
        &self,
        instance: &mut MdnsDnsSdServiceAccumulator,
        srv_record: &MdnsDnsSdSrvRecord,
    ) {
        if let Some(target_hostname) = srv_record.target_hostname.as_ref() {
            if let Some(addresses) = self
                .host_addresses
                .get(&target_hostname.to_ascii_lowercase())
            {
                instance.addresses.extend(addresses.iter().cloned());
            }
        }
    }
}

pub fn populate_mdns_advertisements(
    service_type: &str,
    entries: &[MdnsDnsSdTxtRecord],
    instance: &mut MdnsDnsSdServiceAccumulator,
) {
    if service_type.eq_ignore_ascii_case(constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE) {
        instance.parent_advertisement = parse_parent_mdns_advertisement(entries);
    }
    if service_type.eq_ignore_ascii_case(constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE) {
        instance.child_advertisement = parse_child_mdns_advertisement(entries);
    }
}

#[derive(Default)]
pub struct MdnsDnsSdServiceAccumulator {
    service_type: String,
    instance_name: String,
    display_name: Option<String>,
    target_hostname: Option<String>,
    port: Option<u16>,
    addresses: BTreeSet<String>,
    txt_records: Vec<MdnsDnsSdTxtRecord>,
    parent_advertisement: Option<LanParentMdnsAdvertisement>,
    child_advertisement: Option<LanChildMdnsAdvertisement>,
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
