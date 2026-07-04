use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use ocentra_lan_core::lan_pairing::LanSignedChildAgentReplayGuard;
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;
use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

use super::device_roles::{
    default_device_role_read_model, device_role_read_model_from_env,
    lan_ai_provider_capabilities_from_env, non_empty_env,
};

impl LanPairingRuntime {
    pub fn empty() -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::empty())),
            challenges: Arc::new(Mutex::new(Vec::new())),
            controller_lease: Arc::new(Mutex::new(None)),
            signed_child_agent_replay_guard: Arc::new(Mutex::new(
                LanSignedChildAgentReplayGuard::new(),
            )),
            passive_discovery_listener_state: Arc::new(Mutex::new(
                LanPassiveDiscoveryListenerState::running(timestamp_now()),
            )),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            persistence: LanPairingRegistryPersistence::InMemory,
            local_child_device_id: None,
            signed_child_agent_parent_device_id: None,
            signed_child_agent_family_hash: None,
            signed_child_agent_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            device_roles: default_device_role_read_model(None),
            lan_ai_provider_capabilities: Vec::new(),
        }
    }

    pub fn from_env() -> Self {
        let local_child_device_id = non_empty_env(LanPairingText(
            constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV.to_string(),
        ));
        let signed_child_agent_parent_device_id = non_empty_env(LanPairingText(
            constants::lan_pairing::SIGNED_CHILD_AGENT_PARENT_DEVICE_ID_ENV.to_string(),
        ))
        .map(|value| value.0);
        let signed_child_agent_family_hash = non_empty_env(LanPairingText(
            constants::lan_pairing::SIGNED_CHILD_AGENT_FAMILY_HASH_ENV.to_string(),
        ))
        .map(|value| value.0);
        let signed_child_agent_route_id = non_empty_env(LanPairingText(
            constants::lan_pairing::SIGNED_CHILD_AGENT_ROUTE_ID_ENV.to_string(),
        ))
        .map(|value| value.0)
        .unwrap_or_else(|| constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string());
        let registry_path = lan_pairing_registry_path_from_env(local_child_device_id.clone());
        Self::persistent_json_with_context(
            &registry_path,
            local_child_device_id.map(|value| value.0),
            signed_child_agent_parent_device_id,
            signed_child_agent_family_hash,
            signed_child_agent_route_id,
        )
    }

    fn persistent_json_with_context(
        path: &Path,
        local_child_device_id: Option<String>,
        signed_child_agent_parent_device_id: Option<String>,
        signed_child_agent_family_hash: Option<String>,
        signed_child_agent_route_id: String,
    ) -> Self {
        Self {
            registry: Arc::new(Mutex::new(TrustedDeviceRegistry::load_json(path))),
            challenges: Arc::new(Mutex::new(Vec::new())),
            controller_lease: Arc::new(Mutex::new(None)),
            signed_child_agent_replay_guard: Arc::new(Mutex::new(
                LanSignedChildAgentReplayGuard::new(),
            )),
            passive_discovery_listener_state: Arc::new(Mutex::new(
                LanPassiveDiscoveryListenerState::running(timestamp_now()),
            )),
            lan_ai_provider_heartbeat: Arc::new(Mutex::new(None)),
            lan_ai_job_leases: Arc::new(Mutex::new(Vec::new())),
            persistence: LanPairingRegistryPersistence::LocalJsonRegistry(path.to_path_buf()),
            local_child_device_id,
            signed_child_agent_parent_device_id,
            signed_child_agent_family_hash,
            signed_child_agent_route_id,
            device_roles: device_role_read_model_from_env(),
            lan_ai_provider_capabilities: lan_ai_provider_capabilities_from_env()
                .into_iter()
                .map(|value| value.0)
                .collect(),
        }
    }
}

fn lan_pairing_registry_path_from_env(local_child_device_id: Option<LanPairingText>) -> PathBuf {
    std::env::var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH)
        .ok()
        .filter(|path| !path.trim().is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| default_lan_pairing_registry_path(local_child_device_id))
}

fn default_lan_pairing_registry_path(local_child_device_id: Option<LanPairingText>) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(default_lan_pairing_registry_file_name(
        local_child_device_id,
    ));
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    path
}

fn default_lan_pairing_registry_file_name(local_child_device_id: Option<LanPairingText>) -> String {
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    match sanitize_registry_file_segment(local_child_device_id.unwrap_or_else(|| {
        constants::lan_pairing::REGISTRY_FILE_DEFAULT_SEGMENT
            .to_string()
            .into()
    })) {
        Some(segment) => name.push_str(segment.as_str()),
        None => name.push_str(constants::lan_pairing::REGISTRY_FILE_DEFAULT_SEGMENT),
    }
    name
}

fn sanitize_registry_file_segment(value: LanPairingText) -> Option<String> {
    let sanitized = value
        .0
        .trim()
        .chars()
        .map(|character| match character {
            'a'..='z' | '0'..='9' => character,
            'A'..='Z' => character.to_ascii_lowercase(),
            '-' | '_' => character,
            _ => '-',
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if sanitized.is_empty() {
        None
    } else {
        Some(sanitized)
    }
}
