use std::sync::atomic::{AtomicU64, Ordering};
use std::{
    fmt::{Debug, Display},
    path::{Path, PathBuf},
};

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingIntentKind, LanPairingProof, LanParentIntentEnvelope,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision,
};
use ocentra_parent_agent_protocol::policy_constants;

use ocentra_parent_agent_core::trusted_device_registry::TrustedDeviceRegistry;

use crate::test_text::TestText;

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

pub(crate) fn agent_event_option<T>(result: Option<T>) -> T {
    result.expect_value(constants::error::AGENT_EVENT_SERIALIZES)
}

pub(crate) fn proof(expires_at: impl Display) -> LanPairingProof {
    proof_for(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHALLENGE_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::PROOF_DIGEST,
        expires_at,
    )
}

pub(crate) fn selected_registry(expires_at: impl Display) -> TrustedDeviceRegistry {
    let mut registry = TrustedDeviceRegistry::empty();
    let active_proof = proof(expires_at);
    registry.accept_pairing_proof(
        &active_proof,
        child_device(),
        parent_device(),
        constants::lan_pairing::ISSUED_AT,
    );
    agent_event_result(registry.select_pairing(
        constants::lan_pairing::PAIRING_ID,
        constants::lan_pairing::CHILD_DEVICE_ID,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        constants::lan_pairing::EXPIRES_AT,
    ));
    let _ = registry.clear_selected_route_reachability();
    registry
}

pub(crate) fn proof_for(
    pairing_id: impl Display,
    challenge_id: impl Display,
    child_device_id: impl Display,
    route_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
) -> LanPairingProof {
    let pairing_id = TestText::from_display(pairing_id);
    let challenge_id = TestText::from_display(challenge_id);
    let child_device_id = TestText::from_display(child_device_id);
    let route_id = TestText::from_display(route_id);
    let proof_digest = TestText::from_display(proof_digest);
    let expires_at = TestText::from_display(expires_at);
    LanPairingProof {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: pairing_id.to_string(),
        challenge_id: challenge_id.to_string(),
        child_device_id: child_device_id.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        route_id: route_id.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        proof_digest: proof_digest.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: expires_at.to_string(),
    }
}

pub(crate) fn intent(
    intent_id: impl Display,
    target_child_device_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
) -> LanParentIntentEnvelope {
    intent_with_route(
        intent_id,
        target_child_device_id,
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK,
        proof_digest,
        expires_at,
    )
}

pub(crate) fn intent_with_route(
    intent_id: impl Display,
    target_child_device_id: impl Display,
    route_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
) -> LanParentIntentEnvelope {
    intent_for_pairing(
        intent_id,
        constants::lan_pairing::PAIRING_ID,
        target_child_device_id,
        route_id,
        proof_digest,
        expires_at,
    )
}

pub(crate) fn intent_for_pairing(
    intent_id: impl Display,
    pairing_id: impl Display,
    target_child_device_id: impl Display,
    route_id: impl Display,
    proof_digest: impl Display,
    expires_at: impl Display,
) -> LanParentIntentEnvelope {
    let intent_id = TestText::from_display(intent_id);
    let pairing_id = TestText::from_display(pairing_id);
    let target_child_device_id = TestText::from_display(target_child_device_id);
    let route_id = TestText::from_display(route_id);
    let proof_digest = TestText::from_display(proof_digest);
    let expires_at = TestText::from_display(expires_at);
    LanParentIntentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        intent_id: intent_id.to_string(),
        intent_kind: LanPairingIntentKind::RuleQuery,
        target_child_device_id: target_child_device_id.to_string(),
        route_id: route_id.to_string(),
        pairing_id: pairing_id.to_string(),
        proof_digest: proof_digest.to_string(),
        origin: constants::lan_pairing::ALLOWED_ORIGIN.to_string(),
        issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        expires_at: expires_at.to_string(),
        controller_lease_id: constants::lan_pairing::CONTROLLER_LEASE_ID.to_string(),
        controller_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        parent_actor_id: constants::lan_pairing::PARENT_ACTOR_ID.to_string(),
        parent_authority: LanPairingParentAuthority::ActiveController,
        controller_lease_issued_at: constants::lan_pairing::ISSUED_AT.to_string(),
        controller_lease_expires_at: constants::lan_pairing::CONTROLLER_LEASE_EXPIRES_AT
            .to_string(),
        evidence_references: Vec::new(),
    }
}

pub(crate) fn child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    )
}

pub(crate) fn second_child_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        constants::lan_pairing::SECOND_CHILD_DEVICE_ID.to_string(),
        Some(policy_constants::TEST_CHILD_PROFILE_ID.to_string()),
        policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    )
}

pub(crate) fn parent_device() -> LanPairingDeviceRef {
    LanPairingDeviceRef::new(
        constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        None,
        policy_constants::TEST_PARENT_DEVICE_LABEL.to_string(),
        policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
    )
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
