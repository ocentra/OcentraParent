use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenHouseholdMeshInput {
    pub queue_job_id: String,
    pub screen_evidence_ref: String,
    pub payload_ref: String,
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub provider_result_ref: String,
    pub policy_decision_ref: String,
    pub observed_at: String,
}

impl ScreenHouseholdMeshInput {
    pub fn proof_fixture() -> Self {
        Self {
            queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            screen_evidence_ref: constants::screen_flow::SCREEN_SUMMARY_EVENT_REF.to_string(),
            payload_ref: constants::screen_flow::TEST_SCREEN_MESH_PAYLOAD_REF.to_string(),
            provider_peer_id: constants::screen_flow::TEST_SCREEN_MESH_PROVIDER_PEER_ID.to_string(),
            claim_id: constants::screen_flow::TEST_SCREEN_MESH_CLAIM_ID.to_string(),
            lease_id: constants::screen_flow::TEST_SCREEN_MESH_LEASE_ID.to_string(),
            provider_result_ref: constants::screen_flow::TEST_SCREEN_MESH_RESULT_REF.to_string(),
            policy_decision_ref: constants::activity_store::TEST_POLICY_DECISION_ID.to_string(),
            observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenHouseholdMeshResultSubmission {
    pub provider_peer_id: String,
    pub claim_id: String,
    pub lease_id: String,
    pub screen_evidence_ref: String,
    pub custody_label: String,
    pub duplicate_result: bool,
    pub completed_after_lease_expiry: bool,
    pub raw_screenshot_transferred: bool,
    pub raw_screenshot_retained_by_provider: bool,
    pub provider_policy_event_attempted: bool,
    pub provider_enforcement_event_attempted: bool,
}

impl ScreenHouseholdMeshResultSubmission {
    pub fn accepted_for(input: &ScreenHouseholdMeshInput) -> Self {
        Self {
            provider_peer_id: input.provider_peer_id.clone(),
            claim_id: input.claim_id.clone(),
            lease_id: input.lease_id.clone(),
            screen_evidence_ref: input.screen_evidence_ref.clone(),
            custody_label: constants::value::LAN_PROVIDER_CUSTODY_LOCAL_NETWORK_AI_PROVIDER
                .to_string(),
            duplicate_result: false,
            completed_after_lease_expiry: false,
            raw_screenshot_transferred: false,
            raw_screenshot_retained_by_provider: false,
            provider_policy_event_attempted: false,
            provider_enforcement_event_attempted: false,
        }
    }
}
