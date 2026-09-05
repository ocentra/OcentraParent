use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionLifecycleState;
use ocentra_parent_agent_protocol::lan_pairing_provider_selection::LanProviderSelectionPolicyDecision;

use crate::{
    app::lan_pairing::LanPairingRuntime,
    lan_pairing_provider_selection_read_model::provider_selection_read_model,
};

#[test]
fn provider_selection_read_model_refuses_unpaired_or_missing_provider_routes() {
    let runtime = LanPairingRuntime::empty();
    let read_model = provider_selection_read_model(&runtime);

    assert_eq!(read_model.selected_provider_route_id, None);
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.lifecycle_state
            == LanProviderSelectionLifecycleState::CandidateUnavailable
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::RefuseUnpairedProvider));
    assert!(read_model
        .candidates
        .iter()
        .any(|candidate| candidate.lifecycle_state
            == LanProviderSelectionLifecycleState::NotImplemented
            && candidate.policy_decision
                == LanProviderSelectionPolicyDecision::RequireCloudRelayDecision));
}
