use super::{AgentCommandName, AgentEventName};
use crate::transport::AgentPairingProof;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn lan_runtime_stream_command_and_event_names_serialize_to_contract_shape() {
    let command = serde_json::to_value(AgentCommandName::AgentLanRuntimeEventChainStreamGet)
        .expect_value("LAN runtime stream command serializes");
    let event = serde_json::to_value(AgentEventName::AgentLanRuntimeEventChainStreamReported)
        .expect_value("LAN runtime stream event serializes");

    assert_eq!(command, "agent.lan.runtime.event-chain.stream.get");
    assert_eq!(event, "agent.lan.runtime.event-chain.stream.reported");
}

#[test]
fn pairing_proof_serializes_without_raw_pairing_token() {
    let proof = AgentPairingProof {
        pairing_id: "pairing-local-dev".to_string(),
        device_id: "local-dev-agent".to_string(),
        parent_peer_id: "portal-dev".to_string(),
        issued_at: "2026-05-19T00:00:00Z".to_string(),
        expires_at: "2026-05-19T00:05:00Z".to_string(),
        token_hash: "sha256:local-dev-token-hash".to_string(),
    };

    let serialized = serde_json::to_value(proof).expect_value("pairing proof serializes");

    assert_eq!(serialized["tokenHash"], "sha256:local-dev-token-hash");
    assert_eq!(serialized.get("rawToken"), None);
}
