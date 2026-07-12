use std::env;

use ocentra_parent_agent_protocol::constants;

use super::ServiceIdentityProbeSettings;

pub(super) fn runtime_service_identity_probe_settings() -> ServiceIdentityProbeSettings {
    ServiceIdentityProbeSettings {
        allow_wsd_identity_query: env_flag_enabled(
            constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        ),
        allow_snmp_identity_query: env_flag_enabled(
            constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        ),
        allow_os_fingerprint: false,
    }
}

pub(super) fn env_flag_enabled(name: &str) -> bool {
    matches!(
        env::var(name),
        Ok(value)
            if matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
    )
}
