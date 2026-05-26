pub const COMMAND_PROOF_SUBMIT: &str = "agent.lan-pairing.proof.submit";
pub const COMMAND_ROUTE_SELECT: &str = "agent.lan-pairing.route.select";
pub const COMMAND_ROUTE_REVOKE: &str = "agent.lan-pairing.route.revoke";
pub const COMMAND_STATUS_GET: &str = "agent.lan-pairing.status.get";
pub const EVENT_STATUS_REPORTED: &str = "agent.lan-pairing.status.reported";
pub const EVENT_AUDIT_REPORTED: &str = "agent.lan-pairing.audit.reported";
pub const LOCAL_CHILD_DEVICE_ID_ENV: &str = "OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID";
pub const SUPPORTED_WEBSOCKET_COMMANDS: &[&str] = &[
    COMMAND_PROOF_SUBMIT,
    COMMAND_ROUTE_SELECT,
    COMMAND_ROUTE_REVOKE,
    COMMAND_STATUS_GET,
];

pub const PLANNED_HTTP_ENDPOINT_DISCOVERY_ID: &str = "lan-pairing.discovery";
pub const PLANNED_HTTP_ENDPOINT_CHALLENGE_ID: &str = "lan-pairing.challenge";
pub const PLANNED_HTTP_ENDPOINT_PROOF_ID: &str = "lan-pairing.proof";
pub const PLANNED_HTTP_ENDPOINT_CONTROL_ID: &str = "lan-pairing.control";
pub const PLANNED_HTTP_ENDPOINT_REGISTRY_ID: &str = "lan-pairing.registry";
pub const PLANNED_HTTP_ENDPOINT_DISCOVERY_PATH: &str = "/api/lan-pairing/discovery";
pub const PLANNED_HTTP_ENDPOINT_CHALLENGE_PATH: &str = "/api/lan-pairing/challenge";
pub const PLANNED_HTTP_ENDPOINT_PROOF_PATH: &str = "/api/lan-pairing/proof";
pub const PLANNED_HTTP_ENDPOINT_CONTROL_PATH: &str = "/api/lan-pairing/control";
pub const PLANNED_HTTP_ENDPOINT_REGISTRY_PATH: &str = "/api/lan-pairing/registry";
pub const PLANNED_HTTP_ENDPOINT_PATHS: &[&str] = &[
    PLANNED_HTTP_ENDPOINT_DISCOVERY_PATH,
    PLANNED_HTTP_ENDPOINT_CHALLENGE_PATH,
    PLANNED_HTTP_ENDPOINT_PROOF_PATH,
    PLANNED_HTTP_ENDPOINT_CONTROL_PATH,
    PLANNED_HTTP_ENDPOINT_REGISTRY_PATH,
];

pub const SUPPORT_PLANNED_UNSUPPORTED: &str = "planned-unsupported";
pub const SUPPORT_WEBSOCKET_DIRECT: &str = "websocket-direct";
pub const ADDRESS_REF_UNPROVEN: &str = "lan-address-ref-unproven";
pub const ADDRESS_REF_DIRECT_WEBSOCKET: &str = "lan-address-ref-direct-websocket";
pub const CHALLENGE_ID_PREFIX: &str = "challenge-direct-";
pub const PROOF_DIGEST_PREVIEW_PREFIX: &str = "sha256:direct-preview:";
pub const ROUTE_REQUIREMENT_PAIRED_DEVICE: &str = "paired-device";
pub const ROUTE_REQUIREMENT_ALLOWED_ORIGIN: &str = "allowed-origin";
pub const ROUTE_REQUIREMENT_TARGET_DEVICE_MATCH: &str = "target-device-match";
pub const ROUTE_REQUIREMENT_ROUTE_ID_MATCH: &str = "route-id-match";
pub const ROUTE_REQUIREMENT_UNEXPIRED_INTENT: &str = "unexpired-intent";
pub const ROUTE_REQUIREMENT_NON_REPLAYED_INTENT: &str = "non-replayed-intent";
pub const ROUTE_REQUIREMENT_UNREVOKED_PAIRING: &str = "unrevoked-pairing";
pub const ROUTE_REQUIREMENT_SELECTED_DEVICE_REACHABLE: &str = "selected-device-reachable";
pub const ROUTE_REQUIREMENTS: &[&str] = &[
    ROUTE_REQUIREMENT_PAIRED_DEVICE,
    ROUTE_REQUIREMENT_ALLOWED_ORIGIN,
    ROUTE_REQUIREMENT_TARGET_DEVICE_MATCH,
    ROUTE_REQUIREMENT_ROUTE_ID_MATCH,
    ROUTE_REQUIREMENT_UNEXPIRED_INTENT,
    ROUTE_REQUIREMENT_NON_REPLAYED_INTENT,
    ROUTE_REQUIREMENT_UNREVOKED_PAIRING,
    ROUTE_REQUIREMENT_SELECTED_DEVICE_REACHABLE,
];
pub const MANUAL_PROOF_GAP_LAN_BIND: &str = "manual-lan-bind-proof";
pub const MANUAL_PROOF_GAP_FIREWALL: &str = "manual-firewall-proof";
pub const MANUAL_PROOF_GAP_PHYSICAL_DEVICE: &str = "manual-physical-device-proof";
pub const MANUAL_PROOF_GAPS: &[&str] = &[
    MANUAL_PROOF_GAP_LAN_BIND,
    MANUAL_PROOF_GAP_FIREWALL,
    MANUAL_PROOF_GAP_PHYSICAL_DEVICE,
];

pub const SCHEMA_VERSION: u16 = 1;
pub const SCHEMA_VERSION_TEXT: &str = "v0.9";
pub const ROUTE_ID_LOCAL_NETWORK: &str = "lan-route-local-network";
pub const ROUTE_ID_SECOND_LOCAL_NETWORK: &str = "lan-route-second-local-network";
pub const ROUTE_ID_UNSUPPORTED: &str = "lan-route-unsupported";
pub const CHILD_DEVICE_ID: &str = "child-device-1";
pub const SECOND_CHILD_DEVICE_ID: &str = "child-device-2";
pub const PARENT_DEVICE_ID: &str = "parent-device-1";
pub const PARENT_PEER_ID: &str = "portal-dev";
pub const PAIRING_ID: &str = "pairing-1";
pub const SECOND_PAIRING_ID: &str = "pairing-2";
pub const CHALLENGE_ID: &str = "challenge-1";
pub const SECOND_CHALLENGE_ID: &str = "challenge-2";
pub const PROOF_DIGEST: &str = "sha256:proof-digest";
pub const SECOND_PROOF_DIGEST: &str = "sha256:second-proof-digest";
pub const OTHER_PROOF_DIGEST: &str = "sha256:other-proof-digest";
pub const INTENT_ID: &str = "intent-1";
pub const SECOND_INTENT_ID: &str = "intent-2";
pub const RULE_QUERY_INTENT_ID: &str = "intent-rule-query";
pub const RULE_UPDATE_INTENT_ID: &str = "intent-rule-update";
pub const APPROVAL_DECISION_INTENT_ID: &str = "intent-approval-decision";
pub const SELECT_INTENT_ID: &str = "intent-select-1";
pub const SECOND_SELECT_INTENT_ID: &str = "intent-select-2";
pub const SELECT_BACK_INTENT_ID: &str = "intent-select-back";
pub const REVOKE_INTENT_ID: &str = "intent-revoke-1";
pub const REPLAYED_INTENT_ID: &str = "intent-replayed";
pub const AUDIT_EVENT_ID: &str = "lan-audit-1";
pub const EVIDENCE_REFERENCE_ID: &str = "activity-event-lan-control-1";
pub const SECOND_EVIDENCE_REFERENCE_ID: &str = "activity-event-lan-control-2";
pub const ALLOWED_ORIGIN: &str = "http://127.0.0.1:4478";
pub const WRONG_ORIGIN: &str = "http://127.0.0.1:9478";
pub const ISSUED_AT: &str = "2026-05-23T14:40:00.000Z";
pub const EXPIRES_AT: &str = "2099-05-23T14:45:00.000Z";
pub const EXPIRED_AT: &str = "2026-05-23T14:39:00.000Z";
pub const OBSERVED_AT: &str = "2026-05-23T14:41:00.000Z";
pub const REGISTRY_FILE_PREFIX: &str = "ocentra-parent-lan-registry-";
pub const REGISTRY_FILE_EXTENSION: &str = "json";
