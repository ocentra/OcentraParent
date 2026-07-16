use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

pub(crate) fn provider_id_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED
            .to_string()
            .into()
    }
}

pub(crate) fn execution_state_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::EXECUTION_STATE_DRY_RUN_READY
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::EXECUTION_STATE_DISABLED
            .to_string()
            .into()
    }
}

pub(crate) fn provider_source_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY
        || provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED
    {
        constants::local_ai_runtime::PROVIDER_SOURCE_LOCAL_CONFIG
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::PROVIDER_SOURCE_UNAVAILABLE
            .to_string()
            .into()
    }
}

pub(crate) fn readiness_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_READY
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::ADAPTER_READINESS_STATE_NOT_READY
            .to_string()
            .into()
    }
}

pub(crate) fn unavailable_reason_for_status(provider_status: &LanPairingText) -> LanPairingText {
    if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_AVAILABLE {
        constants::value::EMPTY.to_string().into()
    } else if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_BUSY {
        constants::local_ai_runtime::DEGRADED_OVERLOADED
            .to_string()
            .into()
    } else if provider_status.0 == constants::value::LAN_AI_PROVIDER_STATUS_DEGRADED {
        constants::local_ai_runtime::DEGRADED_PROVIDER_UNAVAILABLE
            .to_string()
            .into()
    } else {
        constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED
            .to_string()
            .into()
    }
}
