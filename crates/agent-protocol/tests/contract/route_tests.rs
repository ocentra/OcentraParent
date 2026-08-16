use super::AgentRoute;
use crate::transport::AgentRouteSecurityPolicy;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_network_route_serializes_to_typescript_contract_shape() {
    let serialized =
        serde_json::to_value(AgentRoute::LocalNetwork).expect_value("route serializes: {error:?}");

    assert_eq!(serialized, "local-network");
}

#[test]
fn local_network_route_security_rejects_anonymous_control() {
    let policy = AgentRouteSecurityPolicy {
        route: AgentRoute::LocalNetwork,
        requires_pairing: true,
        allows_anonymous_control: false,
    };

    let serialized =
        serde_json::to_value(policy).expect_value("route security serializes: {error:?}");

    assert_eq!(serialized["route"], "local-network");
    assert_eq!(serialized["requiresPairing"], true);
    assert_eq!(serialized["allowsAnonymousControl"], false);
}
