pub const COMMAND_PROOF_SUBMIT: &str = "agent.lan-pairing.proof.submit";
pub const COMMAND_ROUTE_SELECT: &str = "agent.lan-pairing.route.select";
pub const COMMAND_ROUTE_REVOKE: &str = "agent.lan-pairing.route.revoke";
pub const COMMAND_STATUS_GET: &str = "agent.lan-pairing.status.get";
pub const COMMAND_BROWSER_DISCOVERY_SCAN: &str = "agent.lan-pairing.browser-discovery.scan";
pub const COMMAND_ADD_DEVICE_REQUEST: &str = "agent.lan-pairing.add-device.request";
pub const COMMAND_CONTROLLER_LEASE_RENEW: &str = "agent.lan-pairing.controller-lease.renew";
pub const COMMAND_CONTROLLER_LEASE_RELEASE: &str = "agent.lan-pairing.controller-lease.release";
pub const COMMAND_CONTROLLER_LEASE_TAKEOVER: &str = "agent.lan-pairing.controller-lease.takeover";
pub const COMMAND_LAN_AI_PROVIDER_STATUS_GET: &str = "agent.lan-ai.provider.status.get";
pub const COMMAND_LAN_AI_JOB_SUBMIT: &str = "agent.lan-ai.job.submit";
pub const EVENT_STATUS_REPORTED: &str = "agent.lan-pairing.status.reported";
pub const EVENT_BROWSER_DISCOVERY_REPORTED: &str = "agent.lan-pairing.browser-discovery.reported";
pub const EVENT_ADD_DEVICE_REPORTED: &str = "agent.lan-pairing.add-device.reported";
pub const EVENT_AUDIT_REPORTED: &str = "agent.lan-pairing.audit.reported";
pub const EVENT_LAN_AI_JOB_REPORTED: &str = "agent.lan-ai.job.reported";
pub const DEVICE_ROLES_ENV: &str = "OCENTRA_PARENT_DEVICE_ROLES";
pub const DEVICE_SURFACE_ENV: &str = "OCENTRA_PARENT_DEVICE_SURFACE";
pub const LAN_AI_PROVIDER_OPT_IN_ENV: &str = "OCENTRA_PARENT_LAN_AI_PROVIDER_OPT_IN";
pub const LAN_AI_PROVIDER_BUSY_ENV: &str = "OCENTRA_PARENT_LAN_AI_PROVIDER_BUSY";
pub const LAN_AI_PROVIDER_CAPABILITIES_ENV: &str = "OCENTRA_PARENT_LAN_AI_PROVIDER_CAPABILITIES";
pub const LOCAL_CHILD_DEVICE_ID_ENV: &str = "OCENTRA_PARENT_AGENT_LAN_CHILD_DEVICE_ID";
pub const SUPPORTED_WEBSOCKET_COMMANDS: &[&str] = &[
    COMMAND_PROOF_SUBMIT,
    COMMAND_ROUTE_SELECT,
    COMMAND_ROUTE_REVOKE,
    COMMAND_STATUS_GET,
    COMMAND_BROWSER_DISCOVERY_SCAN,
    COMMAND_ADD_DEVICE_REQUEST,
    COMMAND_CONTROLLER_LEASE_RENEW,
    COMMAND_CONTROLLER_LEASE_RELEASE,
    COMMAND_CONTROLLER_LEASE_TAKEOVER,
    COMMAND_LAN_AI_PROVIDER_STATUS_GET,
    COMMAND_LAN_AI_JOB_SUBMIT,
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
pub const SUPPORT_NETWORK_NEIGHBOR: &str = "network-neighbor";
pub const ADDRESS_REF_UNPROVEN: &str = "lan-address-ref-unproven";
pub const ADDRESS_REF_DIRECT_WEBSOCKET: &str = "lan-address-ref-direct-websocket";
pub const ADDRESS_REF_NETWORK_NEIGHBOR: &str = "lan-address-ref-network-neighbor";
pub const CHALLENGE_ID_PREFIX: &str = "challenge-direct-";
pub const PROOF_DIGEST_PREVIEW_PREFIX: &str = "sha256:direct-preview:";
pub const ROUTE_REQUIREMENT_PAIRED_DEVICE: &str = "paired-device";
pub const ROUTE_REQUIREMENT_ALLOWED_ORIGIN: &str = "allowed-origin";
pub const ROUTE_REQUIREMENT_TARGET_DEVICE_MATCH: &str = "target-device-match";
pub const ROUTE_REQUIREMENT_ROUTE_ID_MATCH: &str = "route-id-match";
pub const ROUTE_REQUIREMENT_UNEXPIRED_INTENT: &str = "unexpired-intent";
pub const ROUTE_REQUIREMENT_NON_REPLAYED_INTENT: &str = "non-replayed-intent";
pub const ROUTE_REQUIREMENT_UNREVOKED_PAIRING: &str = "unrevoked-pairing";
pub const ROUTE_REQUIREMENT_ACTIVE_CONTROLLER_LEASE: &str = "active-controller-lease";
pub const ROUTE_REQUIREMENT_SELECTED_DEVICE_REACHABLE: &str = "selected-device-reachable";
pub const ROUTE_REQUIREMENT_PARENT_WRITE_AUTHORITY: &str = "parent-write-authority";
pub const ROUTE_REQUIREMENT_LAN_AI_JOB_AUTHORIZED: &str = "lan-ai-job-authorized";
pub const ROUTE_REQUIREMENT_DISCOVERY_STATE_EXPLICIT: &str = "discovery-state-explicit";
pub const ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED: &str = "route-recovery-persisted";
pub const ROUTE_REQUIREMENTS: &[&str] = &[
    ROUTE_REQUIREMENT_PAIRED_DEVICE,
    ROUTE_REQUIREMENT_ALLOWED_ORIGIN,
    ROUTE_REQUIREMENT_TARGET_DEVICE_MATCH,
    ROUTE_REQUIREMENT_ROUTE_ID_MATCH,
    ROUTE_REQUIREMENT_UNEXPIRED_INTENT,
    ROUTE_REQUIREMENT_NON_REPLAYED_INTENT,
    ROUTE_REQUIREMENT_UNREVOKED_PAIRING,
    ROUTE_REQUIREMENT_ACTIVE_CONTROLLER_LEASE,
    ROUTE_REQUIREMENT_SELECTED_DEVICE_REACHABLE,
    ROUTE_REQUIREMENT_PARENT_WRITE_AUTHORITY,
    ROUTE_REQUIREMENT_LAN_AI_JOB_AUTHORIZED,
    ROUTE_REQUIREMENT_DISCOVERY_STATE_EXPLICIT,
    ROUTE_REQUIREMENT_ROUTE_RECOVERY_PERSISTED,
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
pub const LOCAL_AGENT_DEVICE_ID: &str = "local-dev-agent";
pub const LOCAL_AGENT_LABEL: &str = "local-dev-agent";
pub const LOCAL_AGENT_STATUS: &str = "ocentra-local-service";
pub const LAN_SCAN_SOURCE_LOCAL_SERVICE: &str = "local-service";
pub const LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR: &str = "windows-neighbor-table";
pub const TEST_LAN_IP: &str = "192.168.2.42";
pub const TEST_LAN_MAC: &str = "54-27-1e-97-c3-31";
pub const TEST_ROUTER_IP: &str = "192.168.2.1";
pub const TEST_ROUTER_MAC: &str = "00-11-22-33-44-55";
pub const TEST_HOSTNAME: &str = "GAMEDEV";
pub const TEST_NETWORK_INTERFACE: &str = "Ethernet 2";
pub const TEST_NAMED_NEIGHBOR_ROW_PARSE_EXPECT: &str = "named neighbor row parses";
pub const TEST_UNNAMED_NEIGHBOR_ROW_PARSE_EXPECT: &str = "unnamed neighbor row parses";
pub const TEST_NEIGHBOR_CACHE_LOCK_EXPECT: &str = "cache lock";
pub const TEST_NETBIOS_STATUS_ROW_SEPARATOR: &str = " ";
pub const NETWORK_NEIGHBOR_DEVICE_PREFIX: &str = "lan-device-";
pub const CANONICAL_DEVICE_MAC_PREFIX: &str = "lan-physical-mac-";
pub const CANONICAL_DEVICE_IP_PREFIX: &str = "lan-physical-ip-";
pub const CANONICAL_DEVICE_ID_PREFIX: &str = "lan-physical-device-";
pub const LAN_EVIDENCE_ID_PREFIX: &str = "lan-evidence-";
pub const LAN_EVIDENCE_KEY_IP_PREFIX: &str = "ip:";
pub const LAN_EVIDENCE_KEY_MAC_PREFIX: &str = "mac:";
pub const LAN_EVIDENCE_KEY_HOSTNAME_PREFIX: &str = "hostname:";
pub const LAN_EVIDENCE_KEY_INTERFACE_PREFIX: &str = "interface:";
pub const LAN_EVIDENCE_KEY_AGENT_PREFIX: &str = "agent:";
pub const LAN_EVIDENCE_KEY_TRUSTED_PREFIX: &str = "trusted:";
pub const LAN_EVIDENCE_KEY_ROUTE_PREFIX: &str = "route:";
pub const LAN_EVIDENCE_KEY_ROUTER_PREFIX: &str = "router:";
pub const LAN_EVIDENCE_KEY_PARENT_DECISION_PREFIX: &str = "parent-decision:";
pub const REGISTRY_KEY_HOUSEHOLD_DEVICE_DECISIONS: &str = "householdDeviceDecisions";
pub const HOUSEHOLD_ACTION_ID_FIELD: &str = "householdActionId";
pub const HOUSEHOLD_ACTION_KIND_FIELD: &str = "householdActionKind";
pub const HOUSEHOLD_DECISION_ACTION_KIND_FIELD: &str = "actionKind";
pub const HOUSEHOLD_ACTION_CHILD_PROFILE_ID_FIELD: &str = "childProfileId";
pub const HOUSEHOLD_ACTION_DISPLAY_NAME_FIELD: &str = "displayName";
pub const HOUSEHOLD_ACTION_DEVICE_KIND_FIELD: &str = "deviceKind";
pub const HOUSEHOLD_ACTION_REVOKED_AT_FIELD: &str = "revokedAt";
pub const HOUSEHOLD_ACTION_ASSIGN: &str = "assign";
pub const HOUSEHOLD_ACTION_RENAME: &str = "rename";
pub const HOUSEHOLD_ACTION_IGNORE: &str = "ignore";
pub const HOUSEHOLD_ACTION_RESTORE: &str = "restore";
pub const HOUSEHOLD_ACTION_TRUST: &str = "trust";
pub const HOUSEHOLD_DEVICE_KIND_MOBILE: &str = "mobile";
pub const HOUSEHOLD_DEVICE_KIND_DESKTOP: &str = "desktop";
pub const HOUSEHOLD_DEVICE_KIND_LAPTOP: &str = "laptop";
pub const HOUSEHOLD_DEVICE_KIND_TABLET: &str = "tablet";
pub const HOUSEHOLD_DEVICE_KIND_ROUTER: &str = "router";
pub const HOUSEHOLD_DEVICE_KIND_UNKNOWN: &str = "unknown";
pub const HOUSEHOLD_DEVICE_KINDS: &[&str] = &[
    HOUSEHOLD_DEVICE_KIND_MOBILE,
    HOUSEHOLD_DEVICE_KIND_DESKTOP,
    HOUSEHOLD_DEVICE_KIND_LAPTOP,
    HOUSEHOLD_DEVICE_KIND_TABLET,
    HOUSEHOLD_DEVICE_KIND_ROUTER,
    HOUSEHOLD_DEVICE_KIND_UNKNOWN,
];
pub const HOUSEHOLD_ACTION_ID: &str = "household-action-1";
pub const HOUSEHOLD_RESTORE_ACTION_ID: &str = "household-action-restore-1";
pub const HOUSEHOLD_RENAMED_DEVICE_LABEL: &str = "Bedroom Windows PC";
pub const PRODUCTION_PROOF_FIELD_SUMMARY: &str = "productionHouseholdProof";
pub const SIGNED_DISCOVERY_RELAY_FIELD_SUMMARY: &str = "signedDiscoveryRelaySpine";
pub const LAN_SOURCE_MATRIX_FIELD_SUMMARY: &str = "lanDiscoverySourceMatrix";
pub const LAN_SOURCE_MATRIX_FIELD_WORKPACK_ROWS: &str = "workpackRows";
pub const LAN_SOURCE_MATRIX_FIELD_SOURCE_ROWS: &str = "sourceRows";
pub const LAN_SOURCE_MATRIX_FIELD_WORKPACK_ID: &str = "workpackId";
pub const LAN_SOURCE_MATRIX_FIELD_SOURCE: &str = "source";
pub const LAN_SOURCE_MATRIX_FIELD_STATUS: &str = "status";
pub const LAN_SOURCE_MATRIX_FIELD_AUTHORITY: &str = "authority";
pub const LAN_SOURCE_MATRIX_FIELD_UI_SURFACE: &str = "uiSurface";
pub const LAN_SOURCE_MATRIX_FIELD_CAN_CONFIRM: &str = "canConfirmChildAgent";
pub const LAN_SOURCE_MATRIX_FIELD_REQUIRED_ARTIFACT: &str = "requiredArtifactSummary";
pub const PRODUCTION_PROOF_FIELD_STATUS_ROWS: &str = "statusRows";
pub const PRODUCTION_PROOF_FIELD_PROOF_STATE: &str = "proofState";
pub const PRODUCTION_PROOF_FIELD_NOT_IMPLEMENTED: &str = "notImplemented";
pub const PRODUCTION_PROOF_FIELD_CLAIMS_NOT_PROVED: &str = "claimsNotProved";
pub const SIGNED_DISCOVERY_RELAY_FIELD_ADAPTER_ROWS: &str = "adapterRows";
pub const SIGNED_DISCOVERY_RELAY_FIELD_ADAPTER: &str = "adapter";
pub const SIGNED_DISCOVERY_RELAY_FIELD_SIGNED_PROOF_ROWS: &str = "signedProofRows";
pub const SIGNED_DISCOVERY_RELAY_FIELD_REJECTION_REASON: &str = "rejectionReason";
pub const SIGNED_DISCOVERY_RELAY_FIELD_RELAY_CACHE_ROWS: &str = "relayCacheRows";
pub const SIGNED_DISCOVERY_RELAY_FIELD_CUSTODY_LABEL: &str = "custodyLabel";
pub const SIGNED_DISCOVERY_RELAY_FIELD_ROUTE_SAFETY_ROWS: &str = "routeSafetyRows";
pub const SIGNED_DISCOVERY_RELAY_FIELD_CHECK: &str = "check";
pub const SIGNED_DISCOVERY_RELAY_ADAPTER_SIGNED_CHILD_AGENT_HELLO: &str =
    "signed-child-agent-hello";
pub const SIGNED_DISCOVERY_RELAY_CUSTODY_NO_CHILD_DATA: &str = "no-ocentra-child-data-custody";
pub const SIGNED_DISCOVERY_RELAY_ROUTE_CHECK_SELECTED_CUSTODY: &str = "selected-route-custody";
pub const PRODUCTION_PROOF_CAPABILITY_SIGNED_HELLO: &str = "signed-lan-hello";
pub const PRODUCTION_PROOF_CAPABILITY_SIGNED_HEARTBEAT: &str = "signed-lan-heartbeat";
pub const PRODUCTION_PROOF_CAPABILITY_PASSIVE_NEIGHBOR: &str = "passive-neighbor-discovery";
pub const PRODUCTION_PROOF_CAPABILITY_ROUTER_NEIGHBOR: &str = "router-neighbor-discovery";
pub const PRODUCTION_PROOF_CAPABILITY_MDNS: &str = "mdns-name-discovery";
pub const PRODUCTION_PROOF_CAPABILITY_SSDP: &str = "ssdp-name-discovery";
pub const PRODUCTION_PROOF_CAPABILITY_ROUTER_DHCP: &str = "router-dhcp-name-discovery";
pub const PRODUCTION_PROOF_CAPABILITY_TRUSTED_REGISTRY: &str = "trusted-registry";
pub const PRODUCTION_PROOF_CAPABILITY_PARENT_ASSIGNMENT: &str = "parent-assignment";
pub const PRODUCTION_PROOF_CAPABILITY_PARENT_RENAME: &str = "parent-rename";
pub const PRODUCTION_PROOF_CAPABILITY_PARENT_IGNORE: &str = "parent-ignore";
pub const PRODUCTION_PROOF_CAPABILITY_PARENT_REVOCATION: &str = "parent-revocation";
pub const PRODUCTION_PROOF_CAPABILITY_ROUTE_CUSTODY: &str = "route-custody";
pub const PRODUCTION_PROOF_CAPABILITY_STALE_SELECTED: &str = "stale-selected-device";
pub const PRODUCTION_PROOF_CAPABILITY_OFFLINE_SELECTED: &str = "offline-selected-device";
pub const PRODUCTION_PROOF_CAPABILITY_RELAY_ROUTE: &str = "relay-route";
pub const PRODUCTION_PROOF_CAPABILITY_CACHE_ROUTE: &str = "cache-route";
pub const PRODUCTION_PROOF_CAPABILITY_SECOND_PHYSICAL_AGENT: &str = "second-physical-child-agent";
pub const PRODUCTION_PROOF_CAPABILITY_ANDROID_PARITY: &str = "android-child-agent-parity";
pub const PRODUCTION_PROOF_CAPABILITY_IOS_PARITY: &str = "ios-child-agent-parity";
pub const PRODUCTION_PROOF_CAPABILITY_STORE_SIGNING: &str = "store-signing";
pub const PRODUCTION_PROOF_STATE_CI_MECHANICAL: &str = "ci-mechanical-proof";
pub const PRODUCTION_PROOF_STATE_MANUAL_REQUIRED: &str = "manual-required";
pub const PRODUCTION_PROOF_STATE_NOT_IMPLEMENTED: &str = "not-implemented";
pub const PRODUCTION_PROOF_LABEL_SIGNED_HELLO: &str =
    "signed LAN hello is manual-required until a second installed child agent signs a family-scoped hello";
pub const PRODUCTION_PROOF_LABEL_SIGNED_HEARTBEAT: &str =
    "signed LAN heartbeat is manual-required until a second installed child agent emits a signed heartbeat";
pub const PRODUCTION_PROOF_LABEL_PASSIVE_NEIGHBOR: &str =
    "Windows neighbor table scan feeds passive LAN evidence into the read model";
pub const PRODUCTION_PROOF_LABEL_ROUTER_NEIGHBOR: &str =
    "router or infrastructure classification is read-model state, not child-agent proof";
pub const PRODUCTION_PROOF_LABEL_MDNS: &str =
    "mDNS name discovery remains manual-required until service discovery packets are parsed";
pub const PRODUCTION_PROOF_LABEL_SSDP: &str =
    "SSDP name discovery remains manual-required until UPnP discovery packets are parsed";
pub const PRODUCTION_PROOF_LABEL_ROUTER_DHCP: &str =
    "router DHCP name discovery remains manual-required until router integration is proven";
pub const PRODUCTION_PROOF_LABEL_TRUSTED_REGISTRY: &str =
    "trusted registry entries are service-backed local state";
pub const PRODUCTION_PROOF_LABEL_PARENT_ASSIGNMENT: &str =
    "parent assignment decisions are durable registry state when present";
pub const PRODUCTION_PROOF_LABEL_PARENT_RENAME: &str =
    "parent rename decisions are durable registry state when present";
pub const PRODUCTION_PROOF_LABEL_PARENT_IGNORE: &str =
    "parent ignore decisions revoke enrollable surfaces without deleting evidence";
pub const PRODUCTION_PROOF_LABEL_PARENT_REVOCATION: &str =
    "revoked pairings remain explicit before route control is accepted";
pub const PRODUCTION_PROOF_LABEL_ROUTE_CUSTODY: &str =
    "selected route custody is reported separately from discovery";
pub const PRODUCTION_PROOF_LABEL_STALE_SELECTED: &str =
    "stale selected-device state is explicit when selected route evidence is stale";
pub const PRODUCTION_PROOF_LABEL_OFFLINE_SELECTED: &str =
    "offline selected-device state is explicit when selected route evidence is offline";
pub const PRODUCTION_PROOF_LABEL_RELAY_ROUTE: &str =
    "cloud relay route is not implemented in this LAN proof";
pub const PRODUCTION_PROOF_LABEL_CACHE_ROUTE: &str =
    "parent cache route is not implemented in this LAN proof";
pub const PRODUCTION_PROOF_LABEL_SECOND_PHYSICAL_AGENT: &str =
    "second physical child-agent proof is manual-required";
pub const PRODUCTION_PROOF_LABEL_ANDROID_PARITY: &str =
    "Android child-agent parity is manual-required until real device artifacts exist";
pub const PRODUCTION_PROOF_LABEL_IOS_PARITY: &str =
    "iOS child-agent parity is manual-required until entitlement and device artifacts exist";
pub const PRODUCTION_PROOF_LABEL_STORE_SIGNING: &str =
    "store signing remains manual-required until signing and store artifacts exist";
pub const PRODUCTION_PROOF_ARTIFACT_SIGNED_HELLO: &str =
    "Attach signed hello payload, nonce, signature, family hash, route id, and service log from a second physical child agent.";
pub const PRODUCTION_PROOF_ARTIFACT_SIGNED_HEARTBEAT: &str =
    "Attach signed heartbeat payload, expiry/replay evidence, route id, and service log from a second physical child agent.";
pub const PRODUCTION_PROOF_ARTIFACT_MDNS: &str =
    "Attach captured mDNS/DNS-SD discovery packet or parser fixture from the household LAN.";
pub const PRODUCTION_PROOF_ARTIFACT_SSDP: &str =
    "Attach captured SSDP/UPnP discovery packet or parser fixture from the household LAN.";
pub const PRODUCTION_PROOF_ARTIFACT_ROUTER_DHCP: &str =
    "Attach router DHCP name evidence or router integration proof from the household network.";
pub const PRODUCTION_PROOF_ARTIFACT_SECOND_PHYSICAL_AGENT: &str =
    "Attach proof JSON and logs from two distinct physical hosts on the same router or subnet.";
pub const PRODUCTION_PROOF_ARTIFACT_ANDROID_PARITY: &str =
    "Attach real Android child-agent device proof, permissions, package, and transport artifacts.";
pub const PRODUCTION_PROOF_ARTIFACT_IOS_PARITY: &str =
    "Attach real iOS entitlement, device, package, and transport artifacts.";
pub const PRODUCTION_PROOF_ARTIFACT_STORE_SIGNING: &str =
    "Attach signing, Play/TestFlight/App Store, installer, and checksum artifacts before release claims.";
pub const PRODUCTION_PROOF_CLAIM_PASSIVE_NEIGHBOR: &str =
    "passive Windows neighbor evidence is represented in typed LAN read-model state";
pub const PRODUCTION_PROOF_CLAIM_REGISTRY_ROUTE: &str =
    "trusted registry, route custody, stale/offline, and parent decisions are represented in typed LAN read-model state";
pub const PRODUCTION_PROOF_NON_CLAIM_PHYSICAL: &str =
    "physical household LAN readiness remains manual-required until two physical child-agent hosts and router/firewall artifacts are attached";
pub const PRODUCTION_PROOF_NON_CLAIM_SIGNED: &str =
    "signed LAN hello and heartbeat remain manual-required until a second installed child agent signs them";
pub const PRODUCTION_PROOF_NON_CLAIM_CLOUD: &str =
    "cloud relay routing storage and authentication are not implemented in this LAN proof";
pub const PRODUCTION_PROOF_NON_CLAIM_ANDROID: &str =
    "Android child-agent parity remains manual-required until real device permission and transport artifacts are attached";
pub const PRODUCTION_PROOF_NON_CLAIM_IOS: &str =
    "iOS child-agent parity remains manual-required until entitlement device and transport artifacts are attached";
pub const PRODUCTION_PROOF_NON_CLAIM_STORE: &str =
    "store signing remains manual-required until signing store and release artifacts are attached";
pub const SIGNED_DISCOVERY_RELAY_NON_CLAIM_PARENT_STORAGE: &str =
    "parent-owned storage is unavailable until a parent-selected storage adapter exists";
pub const LAN_SOURCE_MATRIX_CLAIM_READ_MODEL: &str =
    "all LAN plan workpacks are represented in a service-backed source matrix read model";
pub const LAN_SOURCE_MATRIX_CLAIM_WEAK_SOURCES: &str =
    "weak LAN discovery sources cannot confirm child identity or assign child profiles";
pub const LAN_SOURCE_MATRIX_NON_CLAIM_PACKET_MODE: &str =
    "packet-mode ARP sweep and passive listeners remain gated until packet driver artifacts exist";
pub const LAN_SOURCE_MATRIX_NON_CLAIM_PHYSICAL: &str =
    "physical household LAN completion remains manual-required until real two-host proof is attached";
pub const LAN_SOURCE_MATRIX_NON_CLAIM_MDNS_SSDP: &str =
    "mDNS/SSDP advertisement and responder behavior remains manual-required until fixtures and LAN captures exist";
pub const LAN_SOURCE_MATRIX_ARTIFACT_PACKET_MODE: &str =
    "Attach packet-driver or controlled packet IO proof with selected interface, subnet cap, timing, and malformed packet fixtures.";
pub const LAN_SOURCE_MATRIX_ARTIFACT_MDNS_SSDP: &str =
    "Attach mDNS/DNS-SD and SSDP/UPnP fixtures or LAN captures with sanitized host/service names.";
pub const LAN_SOURCE_MATRIX_ARTIFACT_SIGNED_CHILD: &str =
    "Attach signed child-agent hello and heartbeat payloads with nonce, family hash, route id, and replay rejection logs.";
pub const LAN_SOURCE_MATRIX_ARTIFACT_PHYSICAL: &str =
    "Attach two physical child-agent host proof, router/firewall reachability, screenshots, and generated proof JSON.";
pub const LAN_SOURCE_MATRIX_TITLE_01: &str = "Contract boundary and Effect schemas";
pub const LAN_SOURCE_MATRIX_TITLE_02: &str = "Evidence model and device record";
pub const LAN_SOURCE_MATRIX_TITLE_03: &str = "Interface detection";
pub const LAN_SOURCE_MATRIX_TITLE_04: &str = "Neighbor table ingestion";
pub const LAN_SOURCE_MATRIX_TITLE_05: &str = "Targeted ARP checks";
pub const LAN_SOURCE_MATRIX_TITLE_06: &str = "Bounded ARP sweep";
pub const LAN_SOURCE_MATRIX_TITLE_07: &str = "Passive discovery listeners";
pub const LAN_SOURCE_MATRIX_TITLE_08: &str = "mDNS and DNS-SD discovery";
pub const LAN_SOURCE_MATRIX_TITLE_09: &str = "SSDP and UPnP discovery";
pub const LAN_SOURCE_MATRIX_TITLE_10: &str = "NetBIOS, LLMNR, and reverse DNS";
pub const LAN_SOURCE_MATRIX_TITLE_11: &str = "Light service probing";
pub const LAN_SOURCE_MATRIX_TITLE_12: &str = "OUI and vendor lookup";
pub const LAN_SOURCE_MATRIX_TITLE_13: &str = "Merge and de-duplication engine";
pub const LAN_SOURCE_MATRIX_TITLE_14: &str = "Explainable classification";
pub const LAN_SOURCE_MATRIX_TITLE_15: &str = "Household device store";
pub const LAN_SOURCE_MATRIX_TITLE_16: &str = "Read models and LAN events";
pub const LAN_SOURCE_MATRIX_TITLE_17: &str = "Parent and child mDNS advertisements";
pub const LAN_SOURCE_MATRIX_TITLE_18: &str = "Signed child hello and heartbeat";
pub const LAN_SOURCE_MATRIX_TITLE_19: &str = "Assignment, revocation, and audit";
pub const LAN_SOURCE_MATRIX_TITLE_20: &str = "Proof gates, fixtures, and rollout";
pub const LAN_SOURCE_MATRIX_WORKPACK_ID_SIGNED_CHILD_HELLO: &str = "18";
pub const NETWORK_NEIGHBOR_LABEL_PREFIX: &str = "LAN ";
pub const NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME: &str = "unknown-host";
pub const PLATFORM_UNKNOWN: &str = "unknown";
pub const PLATFORM_ROUTER: &str = "router";
pub const PLATFORM_WINDOWS: &str = "windows";
pub const CHILD_AGENT_CAPABILITY_DIRECT_WEBSOCKET: &str = "direct-websocket";
pub const CHILD_AGENT_CAPABILITY_DEVICE_INVENTORY: &str = "device-inventory";
pub const CHILD_AGENT_CAPABILITY_PAIRING_ROUTE: &str = "pairing-route";
pub const SURFACE_DEVICES: &str = "devices";
pub const SURFACE_POLICY: &str = "policy";
pub const SURFACE_BROWSER: &str = "browser";
pub const SURFACE_APP: &str = "app";
pub const SURFACE_SCREEN: &str = "screen";
pub const SURFACE_NETWORK: &str = "network";
pub const SURFACE_ACTIVITY: &str = "activity";
pub const SURFACE_TRACKING: &str = "tracking";
pub const SURFACE_AI: &str = "ai";
pub const POWERSHELL_EXE: &str = "powershell";
pub const POWERSHELL_NO_PROFILE_ARG: &str = "-NoProfile";
pub const POWERSHELL_EXECUTION_POLICY_ARG: &str = "-ExecutionPolicy";
pub const POWERSHELL_BYPASS_ARG: &str = "Bypass";
pub const POWERSHELL_COMMAND_ARG: &str = "-Command";
pub const POWERSHELL_LAN_NEIGHBOR_COMMAND: &str = "[Console]::OutputEncoding=[Text.UTF8Encoding]::UTF8; $cache = @{}; try { Get-DnsClientCache | Where-Object { $_.Data -match '^[0-9]+\\.[0-9]+\\.[0-9]+\\.[0-9]+$' -and $_.Entry } | ForEach-Object { $cache[$_.Data] = $_.Entry } } catch {}; Get-NetNeighbor -AddressFamily IPv4 | Where-Object { $_.IPAddress -and $_.LinkLayerAddress } | ForEach-Object { [pscustomobject]@{ IPAddress = $_.IPAddress; LinkLayerAddress = $_.LinkLayerAddress; State = $_.State; InterfaceAlias = $_.InterfaceAlias; Hostname = $cache[$_.IPAddress] } } | ConvertTo-Json -Compress";
pub const POWERSHELL_LOCAL_NETWORK_IDENTITY_COMMAND: &str = "[Console]::OutputEncoding=[Text.UTF8Encoding]::UTF8; Get-NetIPConfiguration | Where-Object { $_.IPv4Address -and $_.NetAdapter.Status -eq 'Up' -and $_.InterfaceAlias -notlike 'vEthernet*' -and $_.InterfaceAlias -notlike 'Loopback*' } | ForEach-Object { [pscustomobject]@{ IPAddress = $_.IPv4Address.IPAddress; InterfaceAlias = $_.InterfaceAlias; MacAddress = $_.NetAdapter.MacAddress } } | Select-Object -First 1 | ConvertTo-Json -Compress";
pub const POWERSHELL_CPU_COMMAND: &str = "[Console]::OutputEncoding=[Text.UTF8Encoding]::UTF8; Get-CimInstance Win32_Processor | Select-Object -First 1 Name,NumberOfCores,NumberOfLogicalProcessors | ConvertTo-Json -Compress";
pub const POWERSHELL_GPU_COMMAND: &str = "[Console]::OutputEncoding=[Text.UTF8Encoding]::UTF8; Get-CimInstance Win32_VideoController | Select-Object Name,DriverVersion,AdapterRAM | ConvertTo-Json -Compress";
pub const POWERSHELL_COMPUTER_SYSTEM_COMMAND: &str = "[Console]::OutputEncoding=[Text.UTF8Encoding]::UTF8; Get-CimInstance Win32_ComputerSystem | Select-Object TotalPhysicalMemory,Manufacturer,Model,Name | ConvertTo-Json -Compress";
pub const LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS: u64 = 3000;
pub const NBTSTAT_EXE: &str = "nbtstat";
pub const NBTSTAT_CACHE_ARG: &str = "-c";
pub const NBTSTAT_ADAPTER_STATUS_ARG: &str = "-A";
pub const NBTSTAT_UNIQUE_MARKER: &str = "UNIQUE";
pub const NBTSTAT_GROUP_MARKER: &str = "GROUP";
pub const NBTSTAT_WORKSTATION_SERVICE_MARKER: &str = "<00>";
pub const NBTSTAT_SERVER_SERVICE_MARKER: &str = "<20>";
pub const NVIDIA_SMI_EXE: &str = "nvidia-smi";
pub const NVIDIA_SMI_QUERY_ARG: &str = "--query-gpu=name,driver_version,memory.total";
pub const NVIDIA_SMI_FORMAT_ARG: &str = "--format=csv,noheader,nounits";
pub const HARDWARE_VALUE_SEPARATOR: &str = " | ";
pub const JSON_KEY_IP_ADDRESS: &str = "IPAddress";
pub const JSON_KEY_LINK_LAYER_ADDRESS: &str = "LinkLayerAddress";
pub const JSON_KEY_STATE: &str = "State";
pub const JSON_KEY_INTERFACE_ALIAS: &str = "InterfaceAlias";
pub const JSON_KEY_HOSTNAME: &str = "Hostname";
pub const JSON_KEY_NAME: &str = "Name";
pub const JSON_KEY_NUMBER_OF_CORES: &str = "NumberOfCores";
pub const JSON_KEY_NUMBER_OF_LOGICAL_PROCESSORS: &str = "NumberOfLogicalProcessors";
pub const JSON_KEY_DRIVER_VERSION: &str = "DriverVersion";
pub const JSON_KEY_ADAPTER_RAM: &str = "AdapterRAM";
pub const JSON_KEY_TOTAL_PHYSICAL_MEMORY: &str = "TotalPhysicalMemory";
pub const JSON_KEY_MANUFACTURER: &str = "Manufacturer";
pub const JSON_KEY_MODEL: &str = "Model";
pub const JSON_KEY_MAC_ADDRESS: &str = "MacAddress";
pub const MAC_DASH: &str = "-";
pub const MAC_ZERO_COMPACT: &str = "000000000000";
pub const MAC_BROADCAST_COMPACT: &str = "ffffffffffff";
pub const MAC_IPV4_MULTICAST_PREFIX_COMPACT: &str = "01005e";
pub const WINDOWS_NEIGHBOR_STATE_REACHABLE_NUMBER: &str = "5";
pub const WINDOWS_NEIGHBOR_STATE_PERMANENT_NUMBER: &str = "6";
pub const WINDOWS_NEIGHBOR_STATE_STALE_NUMBER: &str = "4";
pub const WINDOWS_NEIGHBOR_STATE_REACHABLE: &str = "reachable";
pub const WINDOWS_NEIGHBOR_STATE_PERMANENT: &str = "permanent";
pub const WINDOWS_NEIGHBOR_STATE_STALE: &str = "stale";
pub const CPU_CORES_LABEL: &str = " cores";
pub const CPU_LOGICAL_SEPARATOR: &str = " / ";
pub const CPU_LOGICAL_LABEL: &str = " logical";
pub const MEMORY_GIB_LABEL: &str = " GiB";
pub const MEMORY_MIB_LABEL: &str = " MiB";
pub const NVIDIA_DRIVER_SEPARATOR: &str = " driver ";
pub const NVIDIA_VRAM_LABEL: &str = " VRAM";
pub const PARENT_DEVICE_ID: &str = "parent-device-1";
pub const SECOND_PARENT_DEVICE_ID: &str = "parent-device-2";
pub const PARENT_ACTOR_ID: &str = "parent-actor-1";
pub const SECOND_PARENT_ACTOR_ID: &str = "parent-actor-2";
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
pub const CONTROLLER_LEASE_ID: &str = "controller-lease-1";
pub const SECOND_CONTROLLER_LEASE_ID: &str = "controller-lease-2";
pub const THIRD_CONTROLLER_LEASE_ID: &str = "controller-lease-3";
pub const RULE_QUERY_INTENT_ID: &str = "intent-rule-query";
pub const RULE_UPDATE_INTENT_ID: &str = "intent-rule-update";
pub const APPROVAL_DECISION_INTENT_ID: &str = "intent-approval-decision";
pub const CONTROLLER_LEASE_RENEW_INTENT_ID: &str = "intent-controller-lease-renew";
pub const CONTROLLER_LEASE_RELEASE_INTENT_ID: &str = "intent-controller-lease-release";
pub const CONTROLLER_LEASE_TAKEOVER_INTENT_ID: &str = "intent-controller-lease-takeover";
pub const LAN_AI_PROVIDER_STATUS_INTENT_ID: &str = "intent-lan-ai-provider-status";
pub const LAN_AI_JOB_INTENT_ID: &str = "intent-lan-ai-job";
pub const OBSERVER_RULE_QUERY_INTENT_ID: &str = "intent-observer-rule-query";
pub const OBSERVER_RULE_UPDATE_INTENT_ID: &str = "intent-observer-rule-update";
pub const OLD_CONTROLLER_AFTER_TAKEOVER_INTENT_ID: &str = "intent-old-controller-after-takeover";
pub const LAN_AI_JOB_ID: &str = "lan-ai-job-1";
pub const LAN_AI_CLAIM_ID_PREFIX: &str = "lan-ai-claim-";
pub const LAN_AI_LEASE_ID_PREFIX: &str = "lan-ai-lease-";
pub const LAN_AI_MAX_LEASE_ATTEMPTS: usize = 3;
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
pub const CONTROLLER_LEASE_EXPIRES_AT: &str = "2099-05-23T14:44:00.000Z";
pub const CONTROLLER_LEASE_EXPIRED_AT: &str = "2026-05-23T14:38:00.000Z";
pub const OBSERVED_AT: &str = "2026-05-23T14:41:00.000Z";
pub const REGISTRY_FILE_PREFIX: &str = "ocentra-parent-lan-registry-";
pub const REGISTRY_FILE_EXTENSION: &str = "json";
pub const RAW_MARKER_ACTIVITY_SQLITE: &str = "activity.sqlite";
pub const RAW_MARKER_ACTIVITY_NDJSON: &str = "activity.ndjson";
pub const RAW_MARKER_DECRYPTED_EVIDENCE: &str = "decryptedEvidence";
pub const RAW_MARKER_JOURNAL_PATH: &str = "journalPath";
pub const RAW_MARKER_RAW_EVIDENCE: &str = "rawEvidence";
pub const RAW_MARKER_RAW_PROMPT: &str = "rawPrompt";
pub const RAW_MARKER_RAW_PROOF_SECRET: &str = "rawProofSecret";
pub const RAW_MARKER_RAW_TOKEN: &str = "rawToken";
pub const RAW_MARKER_SQLITE_PATH: &str = "sqlitePath";
