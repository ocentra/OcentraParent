use std::collections::{BTreeMap, BTreeSet};

use ocentra_parent_agent_protocol::constants;

use super::super::advertisement::{
    parse_child_mdns_advertisement, parse_parent_mdns_advertisement,
};
use super::super::text::display_name_from_instance_name;
use super::super::{MdnsDnsSdDiscovery, MdnsDnsSdSrvRecord, MdnsDnsSdTxtRecord};
use super::MdnsDnsSdServiceAccumulator;

pub fn append_first_mdns_summary_detail(summary: &mut String, discovery: &MdnsDnsSdDiscovery) {
    if let Some(instance) = discovery.service_instances.first() {
        append_first_instance_summary(summary, instance);
        return;
    }
    if let Some(service_type) = discovery.service_types.first() {
        summary.push_str("; first service type=");
        summary.push_str(service_type);
    }
}

pub fn populate_instance_details(
    host_addresses: &BTreeMap<String, BTreeSet<String>>,
    srv_records: &BTreeMap<String, MdnsDnsSdSrvRecord>,
    txt_records: &BTreeMap<String, Vec<MdnsDnsSdTxtRecord>>,
    service_type: &str,
    instance_name: &str,
    instance: &mut MdnsDnsSdServiceAccumulator,
) {
    populate_instance_display_name(service_type, instance_name, instance);
    populate_instance_srv_details(host_addresses, srv_records, instance_name, instance);
    populate_instance_txt_details(txt_records, service_type, instance_name, instance);
}

pub fn extend_addresses_for_srv_target(
    host_addresses: &BTreeMap<String, BTreeSet<String>>,
    instance: &mut MdnsDnsSdServiceAccumulator,
    srv_record: &MdnsDnsSdSrvRecord,
) {
    if let Some(target_hostname) = srv_record.target_hostname.as_ref() {
        if let Some(addresses) = host_addresses.get(&target_hostname.to_ascii_lowercase()) {
            instance.addresses.extend(addresses.iter().cloned());
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

fn append_first_instance_summary(summary: &mut String, instance: &super::MdnsDnsSdServiceInstance) {
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
}

fn populate_instance_display_name(
    service_type: &str,
    instance_name: &str,
    instance: &mut MdnsDnsSdServiceAccumulator,
) {
    if let Some(display_name) = display_name_from_instance_name(instance_name, service_type) {
        instance.display_name = Some(display_name);
    }
}

fn populate_instance_srv_details(
    host_addresses: &BTreeMap<String, BTreeSet<String>>,
    srv_records: &BTreeMap<String, MdnsDnsSdSrvRecord>,
    instance_name: &str,
    instance: &mut MdnsDnsSdServiceAccumulator,
) {
    if let Some(srv_record) = srv_records.get(instance_name) {
        instance.target_hostname = srv_record.target_hostname.clone();
        instance.port = srv_record.port;
        extend_addresses_for_srv_target(host_addresses, instance, srv_record);
    }
}

fn populate_instance_txt_details(
    txt_records: &BTreeMap<String, Vec<MdnsDnsSdTxtRecord>>,
    service_type: &str,
    instance_name: &str,
    instance: &mut MdnsDnsSdServiceAccumulator,
) {
    if let Some(entries) = txt_records.get(instance_name) {
        instance.txt_records = entries.clone();
        populate_mdns_advertisements(service_type, entries, instance);
    }
}
