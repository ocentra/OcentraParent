use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityObservationMode,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::network_capture::NetworkObservation;

pub fn network_fields(observation: &NetworkObservation) -> LogFields {
    let mut fields = base_fields(observation);
    insert_optional_text(
        &mut fields,
        constants::field::DESTINATION_DOMAIN,
        &observation.destination_domain,
    );
    insert_optional_text(
        &mut fields,
        constants::field::DESTINATION_IP,
        &observation.destination_ip,
    );
    insert_optional_u16(
        &mut fields,
        constants::field::DESTINATION_PORT,
        observation.destination_port,
    );
    insert_optional_text(
        &mut fields,
        constants::field::LOCAL_IP,
        &observation.local_ip,
    );
    insert_optional_u16(
        &mut fields,
        constants::field::LOCAL_PORT,
        observation.local_port,
    );
    insert_optional_u32(&mut fields, constants::field::PID, observation.pid);
    insert_optional_text(
        &mut fields,
        constants::field::PROCESS_NAME,
        &observation.process_name,
    );
    fields.insert(
        constants::field::ASSOCIATED_PID_COUNT.to_string(),
        LogFieldValue::Number(observation.associated_pid_count as f64),
    );
    fields
}

pub fn network_subject_id(observation: &NetworkObservation) -> String {
    let mut subject_id = if observation.status == ActivityCaptureCapabilityStatus::Available {
        String::from(constants::activity_capture::NETWORK_SUBJECT_ID_PREFIX)
    } else {
        String::from(constants::activity_capture::NETWORK_STATUS_SUBJECT_ID_PREFIX)
    };
    append_network_identity(&mut subject_id, observation);
    subject_id
}

pub fn network_display_name(observation: &NetworkObservation) -> Option<String> {
    if let Some(domain) = &observation.destination_domain {
        return Some(domain.clone());
    }
    if let (Some(ip), Some(port)) = (&observation.destination_ip, observation.destination_port) {
        return Some(endpoint_display(ip, port));
    }
    if let (Some(ip), Some(port)) = (&observation.local_ip, observation.local_port) {
        return Some(endpoint_display(ip, port));
    }
    None
}

fn base_fields(observation: &NetworkObservation) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::OBSERVATION_MODE.to_string(),
        LogFieldValue::String(
            ActivityObservationMode::NetworkSnapshot
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(observation.status.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::ADAPTER_ID.to_string(),
        LogFieldValue::String(constants::activity_capture::NETWORK_ADAPTER_ID.to_string()),
    );
    fields.insert(
        constants::field::DOMAIN_ATTRIBUTION_STATUS.to_string(),
        LogFieldValue::String(
            observation
                .domain_attribution_status()
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::PROCESS_ATTRIBUTION_STATUS.to_string(),
        LogFieldValue::String(
            observation
                .process_attribution_status()
                .as_protocol_str()
                .to_string(),
        ),
    );
    insert_optional_protocol(fields, observation)
}

fn insert_optional_protocol(mut fields: LogFields, observation: &NetworkObservation) -> LogFields {
    if let Some(protocol) = &observation.protocol {
        fields.insert(
            constants::field::NETWORK_PROTOCOL.to_string(),
            LogFieldValue::String(protocol.as_protocol_str().to_string()),
        );
    }
    if let Some(state) = &observation.tcp_state {
        fields.insert(
            constants::field::TCP_STATE.to_string(),
            LogFieldValue::String(state.as_protocol_str().to_string()),
        );
    }
    fields
}

fn append_network_identity(subject_id: &mut String, observation: &NetworkObservation) {
    if let Some(domain) = &observation.destination_domain {
        subject_id.push_str(domain);
        return;
    }
    if let (Some(ip), Some(port)) = (&observation.destination_ip, observation.destination_port) {
        push_endpoint(subject_id, ip, port);
        return;
    }
    if let (Some(ip), Some(port)) = (&observation.local_ip, observation.local_port) {
        push_endpoint(subject_id, ip, port);
        return;
    }
    subject_id.push_str(observation.status.as_protocol_str());
}

fn push_endpoint(target: &mut String, ip: &str, port: u16) {
    target.push_str(ip);
    target.push(constants::delimiter::HYPHEN);
    target.push_str(&port.to_string());
}

fn endpoint_display(ip: &str, port: u16) -> String {
    let mut display = String::from(ip);
    display.push(constants::delimiter::COLON);
    display.push_str(&port.to_string());
    display
}

fn insert_optional_u16(fields: &mut LogFields, key: &str, value: Option<u16>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::Number(f64::from(value)));
    }
}

fn insert_optional_u32(fields: &mut LogFields, key: &str, value: Option<u32>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::Number(f64::from(value)));
    }
}

fn insert_optional_text(fields: &mut LogFields, key: &str, value: &Option<String>) {
    if let Some(value) = value {
        fields.insert(key.to_string(), LogFieldValue::String(value.clone()));
    }
}
