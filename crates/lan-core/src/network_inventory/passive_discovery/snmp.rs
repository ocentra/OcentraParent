use super::super::service_identity::snmp::parse_allowed_snmp_response;
use super::labels::compact_identifier;
use super::text::compact_summary;

pub fn passive_allowed_snmp_response_summary(payload: &[u8]) -> Option<String> {
    let observation = parse_allowed_snmp_response(payload)?;
    let mut parts = Vec::new();
    if let Some(sys_name) = observation.sys_name {
        parts.push(format!("sys-name={sys_name}"));
    }
    if let Some(sys_descr) = observation.sys_descr {
        parts.push(format!("sys-descr={sys_descr}"));
    }
    if parts.is_empty() {
        return None;
    }
    Some(compact_summary(format!(
        "allowed SNMP response: {}",
        parts.join("; ")
    )))
}

pub fn passive_allowed_snmp_response_device_id(payload: &[u8]) -> Option<String> {
    let observation = parse_allowed_snmp_response(payload)?;
    observation
        .sys_name
        .map(|value| compact_identifier(&value))
        .filter(|value| !value.is_empty())
}
