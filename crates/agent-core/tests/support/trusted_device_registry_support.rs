use std::sync::atomic::{AtomicU64, Ordering};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};

static TEMP_REGISTRY_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct TestPath(pub(crate) PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl From<&TestPath> for TestPath {
    fn from(value: &TestPath) -> Self {
        value.clone()
    }
}

pub(crate) fn agent_event_result<T, E>(result: Result<T, E>) -> T
where
    E: Debug,
{
    result.expect_value(constants::error::AGENT_EVENT_SERIALIZES)
}

pub(crate) fn household_decision() -> LanHouseholdDeviceDecision {
    LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: constants::lan_pairing::HOUSEHOLD_ACTION_ID.to_string(),
        action_kind: LanHouseholdDeviceActionKind::Rename,
        canonical_device_id: constants::lan_pairing::LOCAL_AGENT_DEVICE_ID.to_string(),
        child_profile_id: None,
        display_name: Some(constants::lan_pairing::HOUSEHOLD_RENAMED_DEVICE_LABEL.to_string()),
        device_kind: Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_DESKTOP.to_string()),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        decided_at: constants::lan_pairing::OBSERVED_AT.to_string(),
        revoked_at: None,
    }
}

pub(crate) fn temp_registry_path() -> TestPath {
    let mut name = String::from(constants::lan_pairing::REGISTRY_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push_str(
        &TEMP_REGISTRY_COUNTER
            .fetch_add(1, Ordering::Relaxed)
            .to_string(),
    );
    let mut path = std::env::temp_dir();
    path.push(name);
    path.set_extension(constants::lan_pairing::REGISTRY_FILE_EXTENSION);
    TestPath(path)
}
