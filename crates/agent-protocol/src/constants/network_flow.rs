pub const INDICATOR_ADAPTER_UNAVAILABLE: &str = "adapter-unavailable";
pub const INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE: &str = "encrypted-content-unavailable";
pub const INDICATOR_HIGH_VOLUME: &str = "high-volume";
pub const INDICATOR_NEW_DESTINATION: &str = "new-destination";
pub const INDICATOR_REPEATED_FAILURE: &str = "repeated-failure";
pub const INDICATOR_UNUSUAL_UNKNOWN_PROCESS: &str = "unusual-unknown-process";
pub const INDICATOR_VPN_PROXY_TUNNEL: &str = "vpn-proxy-tunnel";

pub const INDICATOR_LABEL_ADAPTER_UNAVAILABLE: &str = "Network adapter unavailable";
pub const INDICATOR_LABEL_ENCRYPTED_CONTENT_UNAVAILABLE: &str = "Encrypted content unavailable";
pub const INDICATOR_LABEL_HIGH_VOLUME: &str = "High network volume";
pub const INDICATOR_LABEL_REPEATED_FAILURE: &str = "Repeated connection failure";
pub const INDICATOR_LABEL_UNUSUAL_UNKNOWN_PROCESS: &str = "Unknown process attribution";
pub const INDICATOR_LABEL_VPN_PROXY_TUNNEL: &str = "VPN, proxy, or tunnel likely in use";
pub const LABEL_DESTINATION_UNKNOWN: &str = "Unknown destination";
pub const LABEL_PROCESS_UNKNOWN: &str = "Unknown process";

pub const EVENT_SCHEMA_VERSION: u16 = 1;
pub const EVENT_NETWORK_FLOW_OBSERVED: &str = "network.flow.observed";
pub const EVENT_NETWORK_FLOW_EVENTING_OBSERVED: &str = "network.flow.eventing.observed";
pub const EVENT_NETWORK_DOMAIN_OBSERVED: &str = "network.domain.observed";
pub const EVENT_NETWORK_ACTIVITY_CLASSIFIED: &str = "network.activity.classified";
pub const EVENT_NETWORK_REVIEW_REQUESTED: &str = "network.review.requested";
pub const EVENT_AI_ANALYSIS_REQUESTED: &str = "ai.analysis.requested";
pub const EVENT_AI_ANALYSIS_COMPLETED: &str = "ai.analysis.completed";
pub const EVENT_POLICY_EVALUATION_REQUESTED: &str = "policy.evaluation.requested";
pub const EVENT_POLICY_DECISION_COMPLETED: &str = "policy.decision.completed";
pub const EVENT_ENFORCEMENT_COMMAND_ISSUED: &str = "enforcement.command.issued";
pub const EVENT_ENFORCEMENT_RESULT_OBSERVED: &str = "enforcement.result.observed";
pub const EVENT_AUDIT_ENTRY_COMMITTED: &str = "audit.entry.committed";
pub const EVENT_PORTAL_READ_MODEL_UPDATED: &str = "portal.read_model.updated";

pub const TEST_DEVICE_REF: &str = "device.child.windows-1";
pub const TEST_FLOW_EVENT_REF: &str = "event.network.flow.observed.1";
pub const TEST_DOMAIN_EVENT_REF: &str = "event.network.domain.observed.1";
pub const TEST_CLASSIFICATION_EVENT_REF: &str = "event.network.activity.classified.1";
pub const TEST_AI_REQUEST_REF: &str = "event.ai.analysis.requested.1";
pub const TEST_AI_ANALYSIS_REF: &str = "event.ai.analysis.completed.1";
pub const TEST_POLICY_EVALUATION_REF: &str = "event.policy.evaluation.requested.1";
pub const TEST_POLICY_DECISION_REF: &str = "event.policy.decision.completed.1";
pub const TEST_ENFORCEMENT_COMMAND_REF: &str = "event.enforcement.command.issued.1";
pub const TEST_ENFORCEMENT_RESULT_REF: &str = "event.enforcement.result.observed.1";
pub const TEST_AUDIT_ENTRY_REF: &str = "event.audit.entry.committed.1";
pub const TEST_PORTAL_READ_MODEL_REF: &str = "event.portal.read-model.updated.1";
pub const TEST_FLOW_EVIDENCE_REF: &str = "evidence.network.flow.1";
pub const TEST_DOMAIN_EVIDENCE_REF: &str = "evidence.network.domain.1";
pub const TEST_PARENT_RULE_REF: &str = "policy.rule.network-domain.1";
pub const TEST_ADAPTER_CAPABILITY_REF: &str = "adapter.capability.network.dry-run.1";
pub const TEST_ROLLBACK_REF: &str = "rollback.network.command.1";
pub const TEST_PROMPT_TEMPLATE_REF: &str = "prompt.network-ai-audit.v1";
pub const TEST_BROKER_CUSTODY_PROOF_REF: &str = "broker.network.custody-proof.1";
pub const TEST_BROKER_PUBLISHER_AUTH_REF: &str = "broker.network.publisher-auth.1";
pub const TEST_BROKER_SUBSCRIBER_AUTH_REF: &str = "broker.network.subscriber-auth.1";
pub const TEST_BROKER_ENCRYPTION_REF: &str = "broker.network.encryption.1";
pub const TEST_BROKER_RETENTION_POLICY_REF: &str = "broker.network.retention-policy.1";
pub const TEST_BROKER_REPLAY_PLAN_REF: &str = "broker.network.replay-plan.1";
pub const TEST_BROKER_DELETION_PLAN_REF: &str = "broker.network.deletion-plan.1";
pub const TEST_BROKER_OFFSET_POLICY_REF: &str = "broker.network.offset-policy.1";
pub const TEST_BROKER_DEDUPE_POLICY_REF: &str = "broker.network.dedupe-policy.1";
pub const TEST_BROKER_CONFIG_REF: &str = "broker.network.config.1";
pub const TEST_BROKER_DROPPED_EVENT_AUDIT_REF: &str = "broker.network.dropped-event-audit.1";
pub const TEST_BROKER_ADAPTER_ACTION_LEDGER_REF: &str = "broker.network.adapter-action-ledger.1";
pub const TEST_FAMILY_HUB_IDENTITY_REF: &str = "family-hub.network.identity.1";
pub const TEST_FAMILY_HUB_RELAY_POLICY_REF: &str = "family-hub.network.relay-policy.1";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF: &str =
    "network.remote-delivery.event-chain-journal.10c";
pub const TEST_REMOTE_EVENT_CHAIN_REPLAY_REF: &str =
    "network.remote-delivery.event-chain-replay.10c";
pub const TEST_REMOTE_EVENT_CHAIN_EXPORT_REF: &str =
    "network.remote-delivery.event-chain-export.10c";
pub const TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.support-status.10c";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF: &str =
    "network.remote-delivery.event-chain.receipt-ledger.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF: &str =
    "network.remote-delivery.event-chain.local-receipt-ack.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_REPLAY_REF: &str =
    "network.remote-delivery.event-chain.receipt-replay.10d";
pub const TEST_REMOTE_EVENT_CHAIN_RECEIPT_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.event-chain.receipt-support-status.10d";
pub const TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF: &str =
    "network.remote-delivery.durable-envelope.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_STORE_REF: &str =
    "network.remote-delivery.durable-envelope-store.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF: &str =
    "network.remote-delivery.durable-envelope-replay.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF: &str =
    "network.remote-delivery.durable-envelope-delete-export.10e";
pub const TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.durable-envelope-support-status.10e";
pub const TEST_REMOTE_DELIVERY_STATUS_BRIDGE_REF: &str =
    "network.remote-delivery.status-bridge.10f";
pub const TEST_REMOTE_DELIVERY_OUTBOX_STATUS_BRIDGE_REF: &str =
    "network.remote-delivery.outbox-status-bridge.10h";
pub const TEST_REMOTE_DELIVERY_OUTBOX_REF: &str = "network.remote-delivery.outbox.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF: &str =
    "network.remote-delivery.outbox-handoff.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF: &str =
    "network.remote-delivery.outbox-replay.10g";
pub const TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF: &str =
    "network.remote-delivery.outbox-support-status.10g";
pub const TEST_REMOTE_DELIVERY_DISPATCH_READINESS_REF: &str =
    "network.remote-delivery.dispatch-readiness.10i";
pub const TEST_REMOTE_DELIVERY_TRANSPORT_REQUIREMENTS_REF: &str =
    "network.remote-delivery.transport-requirements.10i";
pub const TEST_REMOTE_DELIVERY_BROKER_DISPATCH_GATE_REF: &str =
    "network.remote-delivery.broker-dispatch-gate.10i";
pub const TEST_REMOTE_DELIVERY_FAMILY_HUB_DISPATCH_GATE_REF: &str =
    "network.remote-delivery.family-hub-dispatch-gate.10i";
pub const TEST_REMOTE_DELIVERY_NO_ENFORCEMENT_INVARIANT_REF: &str =
    "network.remote-delivery.no-enforcement-invariant.10j";
pub const TEST_REMOTE_DELIVERY_AVAILABLE_METADATA_REF: &str =
    "network.remote-delivery.available-metadata.10j";
pub const TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF: &str =
    "network.remote-delivery.transport-dispatch-state.10k";
pub const TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF: &str =
    "network.remote-delivery.dispatch-blocked-manual-required.10k";
pub const TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF: &str =
    "network.remote-delivery.future-transport-seam.10k";
pub const TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF: &str =
    "network.remote-delivery.fixture-transport.10l";
pub const TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF: &str =
    "network.remote-delivery.fixture-dispatch-attempt.10l";
pub const TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF: &str = "network.remote-delivery.fixture-ack.10l";
pub const TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF: &str =
    "network.remote-delivery.delete-export-propagation-readiness.10m";
pub const TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF: &str =
    "network.remote-delivery.remote-delete-readiness.10m";
pub const TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF: &str =
    "network.remote-delivery.remote-export-readiness.10m";
pub const TEST_REMOTE_DELIVERY_DELETE_EXPORT_STATUS_BRIDGE_REF: &str =
    "network.remote-delivery.delete-export-status-bridge.10n";
pub const TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF: &str =
    "network.remote-delivery.provider-route.10p";
pub const TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF: &str =
    "network.remote-delivery.child-device-route.10p";
pub const TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF: &str =
    "network.remote-delivery.provider-readiness.10p";
pub const TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF: &str =
    "network.remote-delivery.child-device-readiness.10p";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF: &str =
    "network.remote-delivery.cross-process-custody-status.10q";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF: &str =
    "network.remote-delivery.cross-process-replay-readiness.10q";
pub const TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF: &str =
    "network.remote-delivery.remote-retention-readiness.10q";
pub const TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF: &str =
    "network.remote-delivery.remote-delete-custody-readiness.10q";
pub const TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF: &str =
    "network.remote-delivery.remote-export-custody-readiness.10q";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF: &str =
    "network.remote-delivery.cross-process-replay.10r";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF: &str =
    "network.remote-delivery.cross-process-replay-store.10r";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF: &str =
    "network.remote-delivery.cross-process-replay-cursor.10r";
pub const TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STATUS_REF: &str =
    "network.remote-delivery.cross-process-replay-status.10s";
pub const TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF: &str =
    "network.remote-delivery.external-cross-process-transport.10t";
pub const TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF: &str =
    "network.remote-delivery.external-cross-process-transport-envelope.10t";
pub const TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF: &str =
    "network.remote-delivery.external-cross-process-transport-ack.10t";
pub const TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF: &str =
    "network.remote-delivery.external-cross-process-transport-status.10t";
pub const TEST_LIVE_CAPTURE_STATUS_REF: &str = "network.live-capture.status.13a";
pub const FIELD_NETWORK_LIVE_CAPTURE_STATUS: &str = "networkLiveCaptureStatus";
pub const FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS: &str = "networkLinuxNftablesLabStatus";
pub const FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS: &str = "networkWindowsFirewallLabStatus";
pub const FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS: &str = "networkWindowsWfpGateStatus";
pub const FIELD_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS: &str =
    "networkAndroidVpnServiceGateStatus";
pub const FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS: &str =
    "networkAppleNetworkExtensionGateStatus";
pub const TEST_WINDOWS_FIREWALL_LAB_STATUS_REF: &str = "network.windows-firewall.lab-status.38a";
pub const TEST_WINDOWS_FIREWALL_LAB_REF: &str = "network.windows-firewall.lab-execution.38a";
pub const TEST_WINDOWS_FIREWALL_ADAPTER_PLAN_REF: &str =
    "network.windows-firewall.adapter-plan.38a";
pub const TEST_WINDOWS_FIREWALL_POLICY_DECISION_REF: &str =
    "network.policy-decision.windows-firewall.38a";
pub const TEST_WINDOWS_FIREWALL_PARENT_RULE_REF: &str = "network.parent-rule.windows-firewall.38a";
pub const TEST_WINDOWS_FIREWALL_EVIDENCE_REF: &str = "network.evidence.windows-firewall.38a";
pub const TEST_WINDOWS_FIREWALL_LOCAL_AI_RESULT_REF: &str = "network.local-ai.windows-firewall.38a";
pub const TEST_WINDOWS_FIREWALL_CAPABILITY_PROOF_REF: &str =
    "network.windows-firewall.capability-proof.38a";
pub const TEST_WINDOWS_FIREWALL_OS_SCOPE_REF: &str = "network.windows-firewall.os-scope.38a";
pub const TEST_WINDOWS_FIREWALL_TARGET_REF: &str =
    "network.windows-firewall.target.remote-address.38a";
pub const TEST_WINDOWS_FIREWALL_RULE_REF: &str = "network.windows-firewall.rule.38a";
pub const TEST_WINDOWS_FIREWALL_AUTHORIZATION_REF: &str =
    "network.windows-firewall.authorization.38a";
pub const TEST_WINDOWS_FIREWALL_APPLY_ARTIFACT_REF: &str =
    "network.windows-firewall.apply-artifact.38a";
pub const TEST_WINDOWS_FIREWALL_RESULT_ARTIFACT_REF: &str =
    "network.windows-firewall.result-artifact.38a";
pub const TEST_WINDOWS_FIREWALL_ROLLBACK_ARTIFACT_REF: &str =
    "network.windows-firewall.rollback-artifact.38a";
pub const TEST_WINDOWS_FIREWALL_AUDIT_EVENT_REF: &str = "network.windows-firewall.audit-event.38a";
pub const TEST_WINDOWS_FIREWALL_RULE_NAME: &str = "OcentraParentNetworkLab-row38a";
pub const TEST_WINDOWS_FIREWALL_TARGET_REMOTE_ADDRESS: &str = "203.0.113.254";
pub const TEST_WINDOWS_FIREWALL_APPLY_RULE_COMMAND_REF: &str =
    "network.windows-firewall.command.apply-rule.38a";
pub const TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_COMMAND_REF: &str =
    "network.windows-firewall.command.verify-rule-present.38a";
pub const TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_COMMAND_REF: &str =
    "network.windows-firewall.command.rollback-rule.38a";
pub const TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_COMMAND_REF: &str =
    "network.windows-firewall.command.verify-rule-removed.38a";
pub const TEST_WINDOWS_FIREWALL_APPLY_RULE_OUTPUT_SHA256: &str =
    "sha256:network-windows-firewall-apply-rule-38a";
pub const TEST_WINDOWS_FIREWALL_VERIFY_PRESENT_OUTPUT_SHA256: &str =
    "sha256:network-windows-firewall-verify-rule-present-38a";
pub const TEST_WINDOWS_FIREWALL_ROLLBACK_RULE_OUTPUT_SHA256: &str =
    "sha256:network-windows-firewall-rollback-rule-38a";
pub const TEST_WINDOWS_FIREWALL_VERIFY_REMOVED_OUTPUT_SHA256: &str =
    "sha256:network-windows-firewall-verify-rule-removed-38a";
pub const TEST_WINDOWS_WFP_GATE_STATUS_REF: &str = "network.windows-wfp.gate-status.39";
pub const TEST_WINDOWS_WFP_GATE_REF: &str = "network.windows-wfp.gate.39";
pub const TEST_WINDOWS_WFP_POLICY_DECISION_REF: &str = "network.policy-decision.windows-wfp.39";
pub const TEST_WINDOWS_WFP_PARENT_RULE_REF: &str = "network.parent-rule.windows-wfp.39";
pub const TEST_WINDOWS_WFP_EVIDENCE_REF: &str = "network.evidence.windows-wfp.39";
pub const TEST_WINDOWS_WFP_LOCAL_AI_RESULT_REF: &str = "network.local-ai.windows-wfp.39";
pub const TEST_WINDOWS_WFP_TARGET_REF: &str = "network.windows-wfp.target.39";
pub const TEST_WINDOWS_WFP_PROVIDER_REF: &str = "network.windows-wfp.provider.39";
pub const TEST_WINDOWS_WFP_LAYER_REF: &str = "network.windows-wfp.layer.39";
pub const TEST_WINDOWS_WFP_ADMIN_PERMISSION_PROOF_REF: &str =
    "network.windows-wfp.admin-permission-proof.39";
pub const TEST_WINDOWS_WFP_DRIVER_SIGNING_PROOF_REF: &str =
    "network.windows-wfp.driver-signing-proof.39";
pub const TEST_WINDOWS_WFP_DRIVER_PACKAGE_PROOF_REF: &str =
    "network.windows-wfp.driver-package-proof.39";
pub const TEST_WINDOWS_WFP_PROVIDER_REGISTRATION_PLAN_REF: &str =
    "network.windows-wfp.provider-registration-plan.39";
pub const TEST_WINDOWS_WFP_LAYER_CAPABILITY_MATRIX_REF: &str =
    "network.windows-wfp.layer-capability-matrix.39";
pub const TEST_WINDOWS_WFP_ROLLBACK_PLAN_REF: &str = "network.windows-wfp.rollback-plan.39";
pub const TEST_WINDOWS_WFP_LAB_RESULT_ARTIFACT_REF: &str =
    "network.windows-wfp.lab-result-artifact.39";
pub const TEST_WINDOWS_WFP_AUDIT_EVENT_REF: &str = "network.windows-wfp.audit-event.39";
pub const WFP_BOUNDARY_RESEARCH_ONLY_REQUESTED: &str = "research-only-requested";
pub const WFP_BOUNDARY_CAPABILITY_MANUAL_REQUIRED: &str = "capability-manual-required";
pub const WFP_BOUNDARY_CAPABILITY_UNAVAILABLE: &str = "capability-unavailable";
pub const WFP_BOUNDARY_EVIDENCE_GRADE_BELOW_PROOF_THRESHOLD: &str =
    "evidence-grade-below-proof-threshold";
pub const WFP_BOUNDARY_POLICY_NOT_WFP_APPROVED: &str = "policy-not-wfp-approved";
pub const WFP_BOUNDARY_MISSING_REQUIRED_ARTIFACT: &str = "missing-required-artifact";
pub const WFP_ARTIFACT_ADMINISTRATOR_PERMISSION_PROOF: &str = "administrator-permission-proof";
pub const WFP_ARTIFACT_DRIVER_SIGNING_PROOF: &str = "driver-signing-proof";
pub const WFP_ARTIFACT_DRIVER_PACKAGE_PROOF: &str = "driver-package-proof";
pub const WFP_ARTIFACT_PROVIDER_REGISTRATION_PLAN: &str = "provider-registration-plan";
pub const WFP_ARTIFACT_LAYER_CAPABILITY_MATRIX: &str = "layer-capability-matrix";
pub const WFP_ARTIFACT_ROLLBACK_PLAN: &str = "rollback-plan";
pub const WFP_ARTIFACT_LAB_RESULT_ARTIFACT: &str = "lab-result-artifact";
pub const WFP_ARTIFACT_AUDIT_EVENT: &str = "audit-event";
pub const TEST_ANDROID_VPN_SERVICE_GATE_STATUS_REF: &str =
    "network.android-vpn-service.gate-status.40";
pub const TEST_ANDROID_VPN_SERVICE_GATE_REF: &str = "network.android-vpn-service.gate.40";
pub const TEST_ANDROID_VPN_SERVICE_POLICY_DECISION_REF: &str =
    "network.policy-decision.android-vpn-service.40";
pub const TEST_ANDROID_VPN_SERVICE_PARENT_RULE_REF: &str =
    "network.parent-rule.android-vpn-service.40";
pub const TEST_ANDROID_VPN_SERVICE_EVIDENCE_REF: &str = "network.evidence.android-vpn-service.40";
pub const TEST_ANDROID_VPN_SERVICE_LOCAL_AI_RESULT_REF: &str =
    "network.local-ai.android-vpn-service.40";
pub const TEST_ANDROID_VPN_SERVICE_PACKAGE_REF: &str = "network.android-vpn-service.package.40";
pub const TEST_ANDROID_VPN_SERVICE_REF: &str = "network.android-vpn-service.service.40";
pub const TEST_ANDROID_VPN_SERVICE_DECLARATION_REF: &str =
    "network.android-vpn-service.declaration.40";
pub const TEST_ANDROID_VPN_USER_CONSENT_PROOF_REF: &str =
    "network.android-vpn-service.user-consent-proof.40";
pub const TEST_ANDROID_VPN_PHYSICAL_DEVICE_PROOF_REF: &str =
    "network.android-vpn-service.physical-device-proof.40";
pub const TEST_ANDROID_VPN_PACKAGE_IDENTITY_PROOF_REF: &str =
    "network.android-vpn-service.package-identity-proof.40";
pub const TEST_ANDROID_VPN_VIRTUAL_INTERFACE_PROOF_REF: &str =
    "network.android-vpn-service.virtual-interface-proof.40";
pub const TEST_ANDROID_VPN_TRAFFIC_OBSERVATION_PROOF_REF: &str =
    "network.android-vpn-service.traffic-observation-proof.40";
pub const TEST_ANDROID_VPN_ROLLBACK_PLAN_REF: &str = "network.android-vpn-service.rollback-plan.40";
pub const TEST_ANDROID_VPN_AUDIT_EVENT_REF: &str = "network.android-vpn-service.audit-event.40";
pub const TEST_ANDROID_VPN_DEVICE_OWNER_PROOF_REF: &str =
    "network.android-vpn-service.device-owner-proof.40";
pub const ANDROID_VPN_BOUNDARY_RESEARCH_ONLY_REQUESTED: &str = "research-only-requested";
pub const ANDROID_VPN_BOUNDARY_CAPABILITY_MANUAL_REQUIRED: &str = "capability-manual-required";
pub const ANDROID_VPN_BOUNDARY_CAPABILITY_UNAVAILABLE: &str = "capability-unavailable";
pub const ANDROID_VPN_BOUNDARY_EVIDENCE_GRADE_BELOW_PROOF_THRESHOLD: &str =
    "evidence-grade-below-proof-threshold";
pub const ANDROID_VPN_BOUNDARY_POLICY_NOT_VPN_SERVICE_APPROVED: &str =
    "policy-not-vpn-service-approved";
pub const ANDROID_VPN_BOUNDARY_MISSING_REQUIRED_ARTIFACT: &str = "missing-required-artifact";
pub const ANDROID_VPN_ARTIFACT_VPN_SERVICE_DECLARATION: &str = "vpn-service-declaration";
pub const ANDROID_VPN_ARTIFACT_USER_CONSENT_PROOF: &str = "user-consent-proof";
pub const ANDROID_VPN_ARTIFACT_PHYSICAL_DEVICE_PROOF: &str = "physical-device-proof";
pub const ANDROID_VPN_ARTIFACT_PACKAGE_IDENTITY_PROOF: &str = "package-identity-proof";
pub const ANDROID_VPN_ARTIFACT_VIRTUAL_INTERFACE_PROOF: &str = "virtual-interface-proof";
pub const ANDROID_VPN_ARTIFACT_TRAFFIC_OBSERVATION_PROOF: &str = "traffic-observation-proof";
pub const ANDROID_VPN_ARTIFACT_ROLLBACK_PLAN: &str = "rollback-plan";
pub const ANDROID_VPN_ARTIFACT_AUDIT_EVENT: &str = "audit-event";
pub const ANDROID_VPN_ARTIFACT_DEVICE_OWNER_PROOF: &str = "device-owner-proof";
pub const TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF: &str =
    "network.apple-network-extension.gate-status.41";
pub const TEST_APPLE_NETWORK_EXTENSION_GATE_REF: &str = "network.apple-network-extension.gate.41";
pub const TEST_APPLE_NETWORK_EXTENSION_POLICY_DECISION_REF: &str =
    "network.policy-decision.apple-network-extension.41";
pub const TEST_APPLE_NETWORK_EXTENSION_PARENT_RULE_REF: &str =
    "network.parent-rule.apple-network-extension.41";
pub const TEST_APPLE_NETWORK_EXTENSION_EVIDENCE_REF: &str =
    "network.evidence.apple-network-extension.41";
pub const TEST_APPLE_NETWORK_EXTENSION_LOCAL_AI_RESULT_REF: &str =
    "network.local-ai.apple-network-extension.41";
pub const TEST_APPLE_NETWORK_EXTENSION_BUNDLE_REF: &str =
    "network.apple-network-extension.bundle.41";
pub const TEST_APPLE_NETWORK_EXTENSION_REF: &str = "network.apple-network-extension.extension.41";
pub const TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF: &str =
    "network.apple-network-extension.developer-team-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF: &str =
    "network.apple-network-extension.entitlement-approval-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_PROVISIONING_PROFILE_PROOF_REF: &str =
    "network.apple-network-extension.provisioning-profile-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_SIGNING_PROOF_REF: &str =
    "network.apple-network-extension.signing-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_DEVICE_OR_TESTFLIGHT_PROOF_REF: &str =
    "network.apple-network-extension.device-or-testflight-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF: &str =
    "network.apple-network-extension.declaration.41";
pub const TEST_APPLE_NETWORK_EXTENSION_CONFIGURATION_PROOF_REF: &str =
    "network.apple-network-extension.configuration-proof.41";
pub const TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF: &str =
    "network.apple-network-extension.rollback-plan.41";
pub const TEST_APPLE_NETWORK_EXTENSION_AUDIT_EVENT_REF: &str =
    "network.apple-network-extension.audit-event.41";
pub const TEST_APPLE_NETWORK_EXTENSION_SUPERVISION_OR_MDM_PROOF_REF: &str =
    "network.apple-network-extension.supervision-or-mdm-proof.41";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_RESEARCH_ONLY_REQUESTED: &str =
    "research-only-requested";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_CAPABILITY_MANUAL_REQUIRED: &str =
    "capability-manual-required";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_CAPABILITY_UNAVAILABLE: &str = "capability-unavailable";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_EVIDENCE_GRADE_BELOW_PROOF_THRESHOLD: &str =
    "evidence-grade-below-proof-threshold";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_POLICY_NOT_NETWORK_EXTENSION_APPROVED: &str =
    "policy-not-network-extension-approved";
pub const APPLE_NETWORK_EXTENSION_BOUNDARY_MISSING_REQUIRED_ARTIFACT: &str =
    "missing-required-artifact";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_DEVELOPER_TEAM_PROOF: &str = "developer-team-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_ENTITLEMENT_APPROVAL_PROOF: &str =
    "entitlement-approval-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_PROVISIONING_PROFILE_PROOF: &str =
    "provisioning-profile-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_SIGNING_PROOF: &str = "signing-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_DEVICE_OR_TESTFLIGHT_PROOF: &str =
    "device-or-testflight-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_NETWORK_EXTENSION_DECLARATION: &str =
    "network-extension-declaration";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_EXTENSION_CONFIGURATION_PROOF: &str =
    "extension-configuration-proof";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_ROLLBACK_PLAN: &str = "rollback-plan";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_AUDIT_EVENT: &str = "audit-event";
pub const APPLE_NETWORK_EXTENSION_ARTIFACT_SUPERVISION_OR_MDM_PROOF: &str =
    "supervision-or-mdm-proof";
pub const TEST_LINUX_NFTABLES_LAB_STATUS_REF: &str = "network.linux-nftables.lab-status.42a";
pub const TEST_LINUX_NFTABLES_LAB_REF: &str = "network.linux-nftables.lab-execution.42a";
pub const TEST_LINUX_ADAPTER_GATE_REF: &str = "network.linux-adapter.gate.42a";
pub const TEST_LINUX_ADAPTER_POLICY_DECISION_REF: &str = "network.policy-decision.linux.42a";
pub const TEST_LINUX_ADAPTER_PARENT_RULE_REF: &str = "network.parent-rule.linux.42a";
pub const TEST_LINUX_ADAPTER_EVIDENCE_REF: &str = "network.evidence.linux.42a";
pub const TEST_LINUX_ADAPTER_LOCAL_AI_RESULT_REF: &str = "network.local-ai.linux.42a";
pub const TEST_LINUX_ADAPTER_CAPABILITY_PROOF_REF: &str =
    "network.linux-adapter.capability-proof.42a";
pub const TEST_LINUX_DISTRO_REF: &str = "network.linux.distro.42a";
pub const TEST_LINUX_KERNEL_REF: &str = "network.linux.kernel.42a";
pub const TEST_LINUX_DISTRO_KERNEL_PROOF_REF: &str = "network.linux.distro-kernel-proof.42a";
pub const TEST_LINUX_ADAPTER_PERMISSION_PROOF_REF: &str = "network.linux.permission-proof.42a";
pub const TEST_LINUX_ADAPTER_API_CAPABILITY_PROOF_REF: &str =
    "network.linux.adapter-api-capability-proof.42a";
pub const TEST_LINUX_ADAPTER_PLAN_PROOF_REF: &str = "network.linux.adapter-plan-proof.42a";
pub const TEST_LINUX_SERVICE_MANAGER_SCOPE_PROOF_REF: &str =
    "network.linux.service-manager-scope-proof.42a";
pub const TEST_LINUX_ROLLBACK_PLAN_REF: &str = "network.linux.rollback-plan.42a";
pub const TEST_LINUX_LAB_RESULT_ARTIFACT_REF: &str = "network.linux.lab-result-artifact.42a";
pub const TEST_LINUX_ADAPTER_AUDIT_EVENT_REF: &str = "network.linux.audit-event.42a";
pub const TEST_LINUX_NFTABLES_TABLE_NAME: &str = "ocentra_parent_lab_row42a";
pub const TEST_LINUX_NFTABLES_CHAIN_NAME: &str = "ocentra_parent_lab_chain_row42a";
pub const TEST_LINUX_NFTABLES_TARGET_REMOTE_ADDRESS: &str = "203.0.113.253";
pub const TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF: &str =
    "network.linux-nftables.command.create-table.42a";
pub const TEST_LINUX_NFTABLES_CREATE_CHAIN_COMMAND_REF: &str =
    "network.linux-nftables.command.create-chain.42a";
pub const TEST_LINUX_NFTABLES_ADD_RULE_COMMAND_REF: &str =
    "network.linux-nftables.command.add-rule.42a";
pub const TEST_LINUX_NFTABLES_VERIFY_RULE_COMMAND_REF: &str =
    "network.linux-nftables.command.verify-rule-present.42a";
pub const TEST_LINUX_NFTABLES_DELETE_TABLE_COMMAND_REF: &str =
    "network.linux-nftables.command.delete-table.42a";
pub const TEST_LINUX_NFTABLES_VERIFY_REMOVED_COMMAND_REF: &str =
    "network.linux-nftables.command.verify-table-removed.42a";
pub const TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-create-table-42a";
pub const TEST_LINUX_NFTABLES_CREATE_CHAIN_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-create-chain-42a";
pub const TEST_LINUX_NFTABLES_ADD_RULE_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-add-rule-42a";
pub const TEST_LINUX_NFTABLES_VERIFY_RULE_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-verify-rule-present-42a";
pub const TEST_LINUX_NFTABLES_DELETE_TABLE_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-delete-table-42a";
pub const TEST_LINUX_NFTABLES_VERIFY_REMOVED_OUTPUT_SHA256: &str =
    "sha256:network-linux-nftables-verify-table-removed-42a";
pub const TEST_LIVE_CAPTURE_ROW13_STATUS_REF: &str = "network.live-capture.proof-gate.13";
pub const TEST_LIVE_CAPTURE_EXECUTION_STATUS_REF: &str =
    "network.live-capture.execution-status.13b";
pub const TEST_LIVE_CAPTURE_STORAGE_STATUS_REF: &str =
    "network.live-capture.raw-storage-custody.03a";
pub const TEST_LIVE_CAPTURE_WINDOWS_PROOF_REF: &str = "network.live-capture.windows-npcap.13";
pub const TEST_LIVE_CAPTURE_MANUAL_PROOF_REF: &str = "network.live-capture.manual-required.13";
pub const TEST_LIVE_CAPTURE_LINUX_PROOF_REF: &str = "network.live-capture.linux-libpcap.13";
pub const TEST_LIVE_CAPTURE_MACOS_PROOF_REF: &str = "network.live-capture.macos-bpf-libpcap.13";
pub const TEST_LIVE_CAPTURE_INTERFACE_REF: &str = "network.live-capture.interface.13";
pub const TEST_LIVE_CAPTURE_DRIVER_REF: &str = "network.live-capture.driver-proof.13";
pub const TEST_LIVE_CAPTURE_PERMISSION_REF: &str = "network.live-capture.permission-proof.13";
pub const TEST_LIVE_CAPTURE_BOUNDED_REF: &str = "network.live-capture.bounded-capture.13";
pub const TEST_LIVE_CAPTURE_CLEAN_STOP_REF: &str = "network.live-capture.clean-stop.13";
pub const TEST_LIVE_CAPTURE_QUOTA_REF: &str = "network.live-capture.quota-rotation.13";
pub const TEST_LIVE_CAPTURE_RETENTION_REF: &str = "network.live-capture.retention-delete-export.13";
pub const TEST_LIVE_CAPTURE_CUSTODY_REF: &str = "network.live-capture.custody.13";
pub const TEST_LIVE_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF: &str =
    "network.live-capture.private-traffic-exclusion.13";
pub const TEST_LIVE_CAPTURE_WINDOWS_EXECUTION_REF: &str =
    "network.live-capture.execution.windows-npcap.13b";
pub const TEST_LIVE_CAPTURE_MANUAL_EXECUTION_REF: &str =
    "network.live-capture.execution.manual-required.13b";
pub const TEST_LIVE_CAPTURE_LINUX_EXECUTION_REF: &str =
    "network.live-capture.execution.linux-libpcap.13b";
pub const TEST_LIVE_CAPTURE_MACOS_EXECUTION_REF: &str =
    "network.live-capture.execution.macos-bpf-libpcap.13b";
pub const TEST_LIVE_CAPTURE_DRIVER_INVOCATION_REF: &str =
    "network.live-capture.driver-invocation.13b";
pub const TEST_LIVE_CAPTURE_INTERFACE_OBSERVATION_REF: &str =
    "network.live-capture.interface-observation.13b";
pub const TEST_LIVE_CAPTURE_EXECUTION_PERMISSION_REF: &str = "network.live-capture.permission.13b";
pub const TEST_LIVE_CAPTURE_BOUNDED_WINDOW_REF: &str = "network.live-capture.bounded-window.13b";
pub const TEST_LIVE_CAPTURE_EXECUTION_CLEAN_STOP_REF: &str = "network.live-capture.clean-stop.13b";
pub const TEST_LIVE_CAPTURE_EXECUTION_CUSTODY_REF: &str = "network.live-capture.custody.13b";
pub const TEST_LIVE_CAPTURE_EXECUTION_RETENTION_REF: &str =
    "network.live-capture.retention-delete-export.13b";
pub const TEST_LIVE_CAPTURE_METADATA_SANITIZATION_REF: &str =
    "network.live-capture.metadata-sanitization.13b";
pub const TEST_LIVE_CAPTURE_EXECUTION_PRIVATE_TRAFFIC_EXCLUSION_REF: &str =
    "network.live-capture.private-traffic-exclusion.13b";
pub const TEST_RAW_CAPTURE_MANIFEST_REF: &str = "network.raw-capture.manifest.03a";
pub const TEST_RAW_CAPTURE_STORAGE_LOCATION_REF: &str = "network.raw-capture.storage-location.03a";
pub const TEST_RAW_CAPTURE_ENCRYPTION_REF: &str = "network.raw-capture.encryption-at-rest.03a";
pub const TEST_RAW_CAPTURE_QUOTA_REF: &str = "network.raw-capture.quota-rotation.03a";
pub const TEST_RAW_CAPTURE_RETENTION_REF: &str = "network.raw-capture.retention-policy.03a";
pub const TEST_RAW_CAPTURE_DELETE_EXPORT_REF: &str = "network.raw-capture.delete-export.03a";
pub const TEST_RAW_CAPTURE_CUSTODY_CHAIN_REF: &str = "network.raw-capture.custody-chain.03a";
pub const TEST_RAW_CAPTURE_PRIVATE_TRAFFIC_EXCLUSION_REF: &str =
    "network.raw-capture.private-traffic-exclusion.03a";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_PATH_PREFIX: &str =
    "ocentra-network-remote-event-chain-journal";
pub const TEST_REMOTE_EVENT_CHAIN_JOURNAL_EXTENSION: &str = "ndjson";
pub const UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL: &str = "network-only-no-exact-url";
pub const UNSUPPORTED_CLAIM_DECRYPTED_HTTPS_PAYLOAD: &str = "decrypted-https-payload";
pub const UNAVAILABLE_REASON_MANUAL_REQUIRED: &str = "manual-required";

pub const TARGET_NETWORK_OBSERVER: &str = "network-observer";
pub const TARGET_DOMAIN_OBSERVER: &str = "network-domain-observer";
pub const TARGET_ACTIVITY_CLASSIFIER: &str = "network-activity-classifier";
pub const TARGET_NETWORK_REVIEW: &str = "network-review-request";
pub const TARGET_AI_ANALYZER: &str = "network-ai-analyzer";
pub const TARGET_POLICY_ENGINE: &str = "network-policy-engine";
pub const TARGET_ENFORCEMENT_DRY_RUN: &str = "network-enforcement-dry-run";
pub const TARGET_AUDIT_WRITER: &str = "network-audit-writer";
pub const TARGET_PORTAL_READ_MODEL: &str = "network-portal-read-model";

pub const PRODUCT_PATH_ACTION_RESULT_REF_PREFIX: &str = "network.product-path.action-result.";
pub const PRODUCT_PATH_ADAPTER_APPLY_REF_PREFIX: &str = "network.product-path.adapter-apply.";
pub const PRODUCT_PATH_ADAPTER_AUTHORIZATION_REF_PREFIX: &str =
    "network.product-path.adapter-authorization.";
pub const PRODUCT_PATH_ADAPTER_CAPABILITY_REF_PREFIX: &str =
    "network.product-path.adapter-capability.";
pub const PRODUCT_PATH_ADAPTER_RESULT_REF_PREFIX: &str = "network.product-path.adapter-result.";
pub const PRODUCT_PATH_AI_AUDIT_REF_PREFIX: &str = "network.product-path.ai-audit.";
pub const PRODUCT_PATH_AI_BASELINE_REF: &str = "network.product-path.ai-baseline.v1";
pub const PRODUCT_PATH_AI_DETECTION_REF_PREFIX: &str = "network.product-path.ai-detection.";
pub const PRODUCT_PATH_AI_EVALUATION_RUN_REF_PREFIX: &str = "network.product-path.ai-evaluation.";
pub const PRODUCT_PATH_AI_FIXTURE_REF_PREFIX: &str = "network.product-path.ai-fixture.";
pub const PRODUCT_PATH_AI_FIXTURE_SET_REF: &str = "network.product-path.ai-fixture-set.v1";
pub const PRODUCT_PATH_AI_MODEL_CARD_REF: &str = "network.product-path.ai-model-card.v1";
pub const PRODUCT_PATH_AI_MODEL_VERSION_REF: &str = "network.product-path.ai-model-version.v1";
pub const PRODUCT_PATH_ANALYZER_ALERT_REF_PREFIX: &str = "network.product-path.analyzer-alert.";
pub const PRODUCT_PATH_AUDIT_EVENT_REF_PREFIX: &str = "network.product-path.audit.";
pub const PRODUCT_PATH_CAPTURE_REF_PREFIX: &str = "network.product-path.capture.";
pub const PRODUCT_PATH_CASCADE_REF_PREFIX: &str = "network.product-path.cascade.";
pub const PRODUCT_PATH_CHILD_PROFILE_REF: &str = "network.product-path.child-profile.local";
pub const PRODUCT_PATH_DELETION_REF_PREFIX: &str = "network.product-path.delete.";
pub const PRODUCT_PATH_DNS_ADAPTER_PLAN_REF_PREFIX: &str = "network.product-path.dns-adapter-plan.";
pub const PRODUCT_PATH_EXPORT_REF_PREFIX: &str = "network.product-path.export.";
pub const PRODUCT_PATH_HOUSEHOLD_POLICY_REF: &str = "network.product-path.household-policy.local";
pub const PRODUCT_PATH_INGEST_REF_PREFIX: &str = "network.product-path.ingest.";
pub const PRODUCT_PATH_MODEL_RUNTIME_REF: &str = "network.product-path.local-model-runtime";
pub const PRODUCT_PATH_NARRATIVE_TEMPLATE_REF: &str = "network.product-path.narrative-template.v1";
pub const PRODUCT_PATH_PARENT_RULE_REF: &str = "network.product-path.parent-rule.network-domain";
pub const PRODUCT_PATH_POLICY_CONTEXT_REF_PREFIX: &str = "network.product-path.policy-context.";
pub const PRODUCT_PATH_POLICY_DECISION_REF_PREFIX: &str = "network.product-path.policy-decision.";
pub const PRODUCT_PATH_PORTAL_READ_MODEL_REF_PREFIX: &str =
    "network.product-path.portal-read-model.";
pub const PRODUCT_PATH_QUEUE_JOB_REF_PREFIX: &str = "network.product-path.local-ai-job.";
pub const PRODUCT_PATH_QUEUE_REF: &str = "network.product-path.local-ai-queue";
pub const PRODUCT_PATH_RETENTION_REF_PREFIX: &str = "network.product-path.retention.";
pub const PRODUCT_PATH_RISK_BUDGET_REF: &str = "network.product-path.risk-budget.local";
pub const PRODUCT_PATH_RISK_EVALUATION_REF_PREFIX: &str = "network.product-path.risk-evaluation.";
pub const PRODUCT_PATH_ROLLBACK_REF_PREFIX: &str = "network.product-path.rollback.";
pub const PRODUCT_PATH_SUMMARY_REF_PREFIX: &str = "network.product-path.summary.";
pub const PRODUCT_PATH_TOMBSTONE_REF_PREFIX: &str = "network.product-path.tombstone.";
pub const PRODUCT_PATH_TRIGGER_REF_PREFIX: &str = "network.product-path.trigger.";
pub const PRODUCT_PATH_TYPED_EVENT_REF_PREFIX: &str = "network.product-path.typed-event.";

pub const SUBSCRIBER_NETWORK_OBSERVER: &str = "network-runtime-observer-subscriber";
pub const SUBSCRIBER_DOMAIN_OBSERVER: &str = "network-runtime-domain-subscriber";
pub const SUBSCRIBER_ACTIVITY_CLASSIFIER: &str = "network-runtime-classifier-subscriber";
pub const SUBSCRIBER_NETWORK_REVIEW: &str = "network-runtime-review-subscriber";
pub const SUBSCRIBER_AI_REQUEST: &str = "network-runtime-ai-request-subscriber";
pub const SUBSCRIBER_AI_COMPLETE: &str = "network-runtime-ai-complete-subscriber";
pub const SUBSCRIBER_POLICY_REQUEST: &str = "network-runtime-policy-request-subscriber";
pub const SUBSCRIBER_POLICY_DECISION: &str = "network-runtime-policy-decision-subscriber";
pub const SUBSCRIBER_ENFORCEMENT_COMMAND: &str = "network-runtime-enforcement-command-subscriber";
pub const SUBSCRIBER_ENFORCEMENT_RESULT: &str = "network-runtime-enforcement-result-subscriber";
pub const SUBSCRIBER_AUDIT_ENTRY: &str = "network-runtime-audit-entry-subscriber";
pub const SUBSCRIBER_PORTAL_READ_MODEL: &str = "network-runtime-portal-read-model-subscriber";

pub const RUNTIME_COMPONENT_NETWORK_SPINE: &str = "network-runtime-spine";
pub const RUNTIME_INSTANCE_LOCAL_CHILD_AGENT: &str = "local-child-agent";
pub const AGGREGATE_NETWORK_FLOW_PREFIX: &str = "network-flow-";
pub const CORRELATION_NETWORK_RUNTIME_PREFIX: &str = "network-runtime-correlation-";
pub const IDEMPOTENCY_NETWORK_RUNTIME_PREFIX: &str = "network-runtime-idempotency-";
pub const IDEMPOTENCY_NETWORK_REVIEW_PREFIX: &str = "network-review-idempotency-";
pub const REQUEST_NETWORK_REVIEW_PREFIX: &str = "network-review-request-";
pub const REQUEST_NETWORK_REVIEW_TIMEOUT_MS: u64 = 50;
pub const ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES: &str = "network runtime chain publishes";
pub const ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES_DEGRADED: &str =
    "network runtime chain publishes degraded state";
pub const ERROR_NETWORK_RUNTIME_QUEUE_DRAINS: &str = "network runtime queued flow drains";
pub const ERROR_NETWORK_RUNTIME_QUEUE_OVERFLOW_DEAD_LETTERS: &str =
    "network runtime queue overflow dead letters";
pub const ERROR_NETWORK_RUNTIME_QUEUE_TTL_EXPIRES: &str =
    "network runtime queue ttl expires before dispatch";
pub const ERROR_NETWORK_RUNTIME_QUEUE_IDEMPOTENCY_REJECTS: &str =
    "network runtime queue idempotency rejects duplicates";
pub const ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES: &str = "network runtime payload decodes";
pub const ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES: &str = "network runtime review request completes";
pub const ERROR_NETWORK_RUNTIME_BROKER_DELIVERY_SEMANTICS: &str =
    "network runtime broker delivery semantics proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DELIVERY_STATUS: &str =
    "network runtime remote delivery status proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_EVENT_CHAIN_JOURNAL: &str =
    "network runtime remote event-chain journal proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_RECEIPT_LEDGER: &str =
    "network runtime remote receipt ledger proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DURABLE_ENVELOPE: &str =
    "network runtime remote durable envelope proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_HANDOFF: &str =
    "network runtime remote outbox handoff proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_OUTBOX_STATUS_BRIDGE: &str =
    "network runtime remote outbox status bridge proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DISPATCH_READINESS: &str =
    "network runtime remote dispatch readiness proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_NO_ENFORCEMENT_INVARIANT: &str =
    "network runtime remote no-enforcement invariant proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE: &str =
    "network runtime remote transport dispatch state proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_FIXTURE_TRANSPORT: &str =
    "network runtime remote fixture transport proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_DELETE_EXPORT_PROPAGATION: &str =
    "network runtime remote delete export propagation proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_PROVIDER_CHILD_READINESS: &str =
    "network runtime remote provider child readiness proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_CUSTODY_READINESS: &str =
    "network runtime remote cross-process custody readiness proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY: &str =
    "network runtime remote cross-process replay proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY_STATUS_BRIDGE: &str =
    "network runtime remote cross-process replay status bridge proof";
pub const ERROR_NETWORK_RUNTIME_REMOTE_EXTERNAL_CROSS_PROCESS_TRANSPORT: &str =
    "network runtime remote external cross-process transport proof";
pub const ERROR_NETWORK_LIVE_CAPTURE_STATUS: &str = "network live capture status proof";
pub const ERROR_NETWORK_LINUX_NFTABLES_LAB_STATUS: &str = "network Linux nftables lab status proof";
pub const ERROR_NETWORK_WINDOWS_FIREWALL_LAB_STATUS: &str =
    "network Windows firewall lab status proof";
pub const ERROR_NETWORK_WINDOWS_WFP_GATE_STATUS: &str = "network Windows WFP gate status proof";
pub const ERROR_NETWORK_ANDROID_VPN_SERVICE_GATE_STATUS: &str =
    "network Android VpnService gate status proof";
pub const ERROR_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS: &str =
    "network Apple Network Extension gate status proof";
