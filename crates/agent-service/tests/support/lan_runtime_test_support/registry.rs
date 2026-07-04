use std::{
    fmt::Display,
    path::Path as TestPath,
    sync::{Arc, Mutex},
};

use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::DeviceRoleRuntimeReadModel;

use crate::lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime};
use crate::test_text::TestText;

pub(crate) fn persistent_json(path: impl AsRef<TestPath>) -> LanPairingRuntime {
    let path = path.as_ref();
    let mut runtime = LanPairingRuntime::empty();
    runtime.registry = Arc::new(Mutex::new(TrustedDeviceRegistry::load_json(path)));
    runtime.persistence = LanPairingRegistryPersistence::LocalJsonRegistry(path.to_path_buf());
    runtime
}

pub(crate) fn empty_with_local_child_device_id(
    local_child_device_id: Option<TestText>,
) -> LanPairingRuntime {
    let mut runtime = LanPairingRuntime::empty();
    runtime.local_child_device_id = local_child_device_id.map(|text| text.0);
    runtime
}

pub(crate) fn empty_with_signed_child_agent_context(
    local_child_device_id: Option<TestText>,
    parent_device_id: impl Display,
    family_hash: impl Display,
    route_id: impl Display,
) -> LanPairingRuntime {
    let mut runtime = empty_with_local_child_device_id(local_child_device_id);
    runtime.signed_child_agent_parent_device_id = Some(parent_device_id.to_string());
    runtime.signed_child_agent_family_hash = Some(family_hash.to_string());
    runtime.signed_child_agent_route_id = route_id.to_string();
    runtime
}

pub(crate) fn empty_with_device_role_read_model(
    device_roles: DeviceRoleRuntimeReadModel,
) -> LanPairingRuntime {
    let mut runtime = LanPairingRuntime::empty();
    runtime.device_roles = device_roles;
    runtime.lan_ai_provider_capabilities = vec![
        constants::local_ai_runtime::CAPABILITY_CHAT_COMPLETION.to_string(),
        constants::local_ai_runtime::CAPABILITY_SUMMARIZATION.to_string(),
    ];
    runtime
}
