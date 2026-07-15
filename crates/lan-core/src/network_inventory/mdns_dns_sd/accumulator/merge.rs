use super::super::advertisement::is_selected_service_type;
use super::super::{MdnsDnsSdPacket, MdnsDnsSdSrvRecord, MdnsRecordData, MDNS_SERVICE_ENUMERATION};
use super::MdnsDnsSdDiscoveryAccumulator;

impl MdnsDnsSdDiscoveryAccumulator {
    pub fn merge(&mut self, packet: MdnsDnsSdPacket) {
        for record in packet.records {
            merge_record(self, record.name, record.data);
        }
    }
}

fn merge_record(
    accumulator: &mut MdnsDnsSdDiscoveryAccumulator,
    record_name: String,
    data: MdnsRecordData,
) {
    match data {
        MdnsRecordData::Ptr(target) => merge_ptr_record(accumulator, record_name, target),
        MdnsRecordData::Srv {
            target_hostname,
            port,
        } => {
            accumulator.srv_records.insert(
                record_name,
                MdnsDnsSdSrvRecord {
                    target_hostname,
                    port,
                },
            );
        }
        MdnsRecordData::Txt(entries) => {
            accumulator.txt_records.insert(record_name, entries);
        }
        MdnsRecordData::A(address) | MdnsRecordData::Aaaa(address) => {
            accumulator
                .host_addresses
                .entry(record_name.to_ascii_lowercase())
                .or_default()
                .insert(address);
        }
        MdnsRecordData::Unknown => {}
    }
}

fn merge_ptr_record(
    accumulator: &mut MdnsDnsSdDiscoveryAccumulator,
    record_name: String,
    target: String,
) {
    if record_name.eq_ignore_ascii_case(MDNS_SERVICE_ENUMERATION) {
        if is_selected_service_type(&target) {
            accumulator.service_types.insert(target);
        }
        return;
    }
    if !is_selected_service_type(&record_name) {
        return;
    }
    let instance_key = (record_name, target.clone());
    accumulator
        .instances
        .entry(instance_key)
        .or_default()
        .instance_name = target;
}
