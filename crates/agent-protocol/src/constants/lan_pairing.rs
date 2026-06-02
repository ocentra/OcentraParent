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
pub const NETWORK_NEIGHBOR_DEVICE_PREFIX: &str = "lan-device-";
pub const NETWORK_NEIGHBOR_LABEL_PREFIX: &str = "LAN ";
pub const NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME: &str = "unknown-host";
pub const PLATFORM_UNKNOWN: &str = "unknown";
pub const PLATFORM_ROUTER: &str = "router";
pub const PLATFORM_WINDOWS: &str = "windows";
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
pub const NBTSTAT_EXE: &str = "nbtstat";
pub const NBTSTAT_CACHE_ARG: &str = "-c";
pub const NBTSTAT_UNIQUE_MARKER: &str = "UNIQUE";
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
