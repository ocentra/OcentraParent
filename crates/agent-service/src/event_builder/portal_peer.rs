pub fn portal_peer() -> ocentra_parent_agent_protocol::transport::AgentPeer {
    ocentra_parent_agent_protocol::transport::AgentPeer {
        peer_id: ocentra_parent_agent_protocol::constants::peer::PORTAL_DEV.to_string(),
        role: ocentra_parent_agent_protocol::transport::AgentPeerRole::Portal,
    }
}
