use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::{
    android_vpn_service_gate::{
        plan_network_android_vpn_service_gate, NetworkAndroidVpnServiceCapabilityState,
        NetworkAndroidVpnServiceGateInput,
    },
    apple_network_extension_gate::{
        plan_network_apple_network_extension_gate, NetworkAppleNetworkExtensionCapabilityState,
        NetworkAppleNetworkExtensionGateInput, NetworkAppleNetworkExtensionPlatform,
    },
    dns::types::NetworkEvidenceGrade,
    linux_adapter_gate::{
        plan_network_linux_adapter_gate, NetworkLinuxAdapterCapabilityState,
        NetworkLinuxAdapterGateInput, NetworkLinuxAdapterKind,
    },
    platform_claims::{
        build_network_platform_claim_manifest, NetworkPlatformClaimManifestError,
        NetworkPlatformClaimManifestInput, NetworkPlatformClaimProofSource,
        NetworkPlatformClaimState, NetworkPlatformClaimTarget, NetworkPlatformUnsupportedClaims,
    },
    policy::{
        map_network_evidence_grade_to_policy, NetworkEvidencePolicyAction,
        NetworkEvidencePolicyMapping, NetworkEvidencePolicyMappingInput,
    },
    windows_firewall_adapter::{
        plan_network_windows_firewall_adapter_proof, NetworkWindowsFirewallAdapterAction,
        NetworkWindowsFirewallAdapterProofInput, NetworkWindowsFirewallCapabilityState,
        NetworkWindowsFirewallProofState, NetworkWindowsFirewallTargetKind,
    },
    windows_wfp_gate::{
        plan_network_windows_wfp_gate, NetworkWindowsWfpGateCapabilityState,
        NetworkWindowsWfpGateInput,
    },
};

#[derive(Clone, Copy)]
struct Suffix(&'static str);

#[test]
fn platform_claim_manifest_names_fixture_platform_permission_and_device_refs() {
    let proof = build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
        manifest_ref: " network-platform-manifest-52 ".to_owned(),
        proof_sources: complete_platform_sources(),
        unsupported_claims: no_unsupported_claims(),
    })
    .expect_value("complete platform gates should produce a manifest");

    assert_eq!(proof.manifest_ref, "network-platform-manifest-52");
    assert_eq!(proof.entries.len(), 8);
    assert_eq!(proof.ready_claims, 8);
    assert_eq!(proof.manual_required_claims, 0);
    assert_eq!(proof.unavailable_claims, 0);
    assert!(proof.manual_followups.is_empty());
    assert!(proof.every_claim_names_platform);
    assert!(proof.every_claim_names_permission_or_manual_followup);
    assert!(proof.no_enforcement_commands_published);
    assert!(proof.no_live_adapter_execution_claimed);
    assert!(proof.ui_has_no_policy_authority);
    assert!(proof.every_claim_names_audit_ref);

    let windows_firewall = proof
        .entries
        .iter()
        .find(|entry| entry.target == NetworkPlatformClaimTarget::WindowsFirewall)
        .expect_value("Windows Firewall row should exist");
    assert_eq!(
        windows_firewall.device_or_os_refs,
        vec!["windows-os-scope-ref-52".to_owned()]
    );
    assert!(windows_firewall
        .adapter_capability_refs
        .contains(&"windows-firewall-domain-target-ref-52".to_owned()));
    assert!(windows_firewall
        .adapter_capability_refs
        .contains(&"windows-firewall-rule-ref-52".to_owned()));

    let targets: Vec<NetworkPlatformClaimTarget> =
        proof.entries.iter().map(|entry| entry.target).collect();
    assert_eq!(
        targets,
        vec![
            NetworkPlatformClaimTarget::WindowsFirewall,
            NetworkPlatformClaimTarget::WindowsWfp,
            NetworkPlatformClaimTarget::AndroidVpnService,
            NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs,
            NetworkPlatformClaimTarget::AppleNetworkExtensionIos,
            NetworkPlatformClaimTarget::LinuxNftables,
            NetworkPlatformClaimTarget::LinuxEbpf,
            NetworkPlatformClaimTarget::LinuxTun
        ]
    );
    assert!(proof
        .entries
        .iter()
        .all(|entry| entry.claim_state == NetworkPlatformClaimState::Ready));
}

#[test]
fn platform_claim_manifest_records_missing_permission_artifacts_as_manual_followup() {
    let proof = build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
        manifest_ref: "network-platform-manifest-52".to_owned(),
        proof_sources: vec![NetworkPlatformClaimProofSource::WindowsWfp(
            plan_network_windows_wfp_gate(NetworkWindowsWfpGateInput {
                administrator_permission_proof_ref: None,
                ..windows_wfp_input()
            })
            .expect_value("missing administrator proof should stay reportable"),
        )],
        unsupported_claims: no_unsupported_claims(),
    })
    .expect_value("manual-required platform gate should produce a manifest");

    assert_eq!(proof.ready_claims, 0);
    assert_eq!(proof.manual_required_claims, 1);
    assert_eq!(proof.manual_followups.len(), 1);
    assert_eq!(
        proof.manual_followups[0].missing_required_artifacts,
        vec!["windows-wfp.administrator-permission".to_owned()]
    );
    assert!(proof.every_claim_names_platform);
    assert!(proof.every_claim_names_permission_or_manual_followup);
}

#[test]
fn platform_claim_manifest_reports_unavailable_states_without_execution() {
    let proof = build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
        manifest_ref: "network-platform-manifest-52".to_owned(),
        proof_sources: vec![NetworkPlatformClaimProofSource::LinuxAdapter(
            plan_network_linux_adapter_gate(NetworkLinuxAdapterGateInput {
                capability_state: NetworkLinuxAdapterCapabilityState::Unavailable,
                permission_proof_ref: None,
                ..linux_input_for(NetworkLinuxAdapterKind::Tun, Suffix("unavailable"))
            })
            .expect_value("unavailable Linux adapter state should stay reportable"),
        )],
        unsupported_claims: no_unsupported_claims(),
    })
    .expect_value("unavailable platform gate should produce a manifest");

    assert_eq!(proof.ready_claims, 0);
    assert_eq!(proof.unavailable_claims, 1);
    assert_eq!(proof.manual_required_claims, 0);
    assert_eq!(
        proof.entries[0].target,
        NetworkPlatformClaimTarget::LinuxTun
    );
    assert_eq!(
        proof.entries[0].claim_state,
        NetworkPlatformClaimState::Unavailable
    );
    assert!(!proof.entries[0].adapter_authorized_by_proof);
    assert!(!proof.entries[0].enforcement_command_published);
    assert_eq!(proof.manual_followups.len(), 1);
    assert!(proof.manual_followups[0]
        .missing_required_artifacts
        .contains(&"linux-adapter.permission".to_owned()));
}

#[test]
fn platform_claim_manifest_rejects_broad_or_live_platform_claims() {
    assert_eq!(
        build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
            manifest_ref: "network-platform-manifest-52".to_owned(),
            proof_sources: complete_platform_sources(),
            unsupported_claims: NetworkPlatformUnsupportedClaims {
                generic_platform_support_claimed: true,
                ..no_unsupported_claims()
            },
        }),
        Err(NetworkPlatformClaimManifestError::GenericPlatformSupportClaimRejected)
    );
    assert_eq!(
        build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
            manifest_ref: "network-platform-manifest-52".to_owned(),
            proof_sources: complete_platform_sources(),
            unsupported_claims: NetworkPlatformUnsupportedClaims {
                live_adapter_execution_claimed: true,
                ..no_unsupported_claims()
            },
        }),
        Err(NetworkPlatformClaimManifestError::LiveAdapterExecutionClaimRejected)
    );
    assert_eq!(
        build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
            manifest_ref: "network-platform-manifest-52".to_owned(),
            proof_sources: complete_platform_sources(),
            unsupported_claims: NetworkPlatformUnsupportedClaims {
                ui_policy_authority_claimed: true,
                ..no_unsupported_claims()
            },
        }),
        Err(NetworkPlatformClaimManifestError::UiPolicyAuthorityClaimRejected)
    );
}

#[test]
fn platform_claim_manifest_rejects_proof_source_that_publishes_enforcement_command() {
    let mut proof = plan_network_windows_wfp_gate(windows_wfp_input())
        .expect_value("complete WFP input should build proof");
    proof.enforcement_command_published = true;

    assert_eq!(
        build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
            manifest_ref: "network-platform-manifest-52".to_owned(),
            proof_sources: vec![NetworkPlatformClaimProofSource::WindowsWfp(proof)],
            unsupported_claims: no_unsupported_claims(),
        }),
        Err(
            NetworkPlatformClaimManifestError::ProofSourcePublishedEnforcementCommand(
                NetworkPlatformClaimTarget::WindowsWfp
            )
        )
    );
}

#[test]
fn platform_claim_manifest_rejects_non_ready_adapter_authorization() {
    let mut proof = plan_network_windows_firewall_adapter_proof(windows_firewall_input())
        .expect_value("complete Windows Firewall input should build proof");
    proof.proof_state = NetworkWindowsFirewallProofState::DryRun;
    proof.adapter_apply_authorized = true;

    assert_eq!(
        build_network_platform_claim_manifest(NetworkPlatformClaimManifestInput {
            manifest_ref: "network-platform-manifest-52".to_owned(),
            proof_sources: vec![NetworkPlatformClaimProofSource::WindowsFirewall(proof)],
            unsupported_claims: no_unsupported_claims(),
        }),
        Err(
            NetworkPlatformClaimManifestError::ProofSourceAuthorizesNonReadyAdapter(
                NetworkPlatformClaimTarget::WindowsFirewall
            )
        )
    );
}

fn complete_platform_sources() -> Vec<NetworkPlatformClaimProofSource> {
    vec![
        NetworkPlatformClaimProofSource::WindowsFirewall(
            plan_network_windows_firewall_adapter_proof(windows_firewall_input())
                .expect_value("complete Windows Firewall input should build proof"),
        ),
        NetworkPlatformClaimProofSource::WindowsWfp(
            plan_network_windows_wfp_gate(windows_wfp_input())
                .expect_value("complete WFP input should build proof"),
        ),
        NetworkPlatformClaimProofSource::AndroidVpnService(
            plan_network_android_vpn_service_gate(android_input())
                .expect_value("complete Android input should build proof"),
        ),
        NetworkPlatformClaimProofSource::AppleNetworkExtension(
            plan_network_apple_network_extension_gate(apple_input_for(
                NetworkAppleNetworkExtensionPlatform::MacOs,
                Suffix("macos"),
            ))
            .expect_value("complete Apple macOS input should build proof"),
        ),
        NetworkPlatformClaimProofSource::AppleNetworkExtension(
            plan_network_apple_network_extension_gate(apple_input_for(
                NetworkAppleNetworkExtensionPlatform::Ios,
                Suffix("ios"),
            ))
            .expect_value("complete Apple iOS input should build proof"),
        ),
        NetworkPlatformClaimProofSource::LinuxAdapter(
            plan_network_linux_adapter_gate(linux_input_for(
                NetworkLinuxAdapterKind::Nftables,
                Suffix("nftables"),
            ))
            .expect_value("complete Linux nftables input should build proof"),
        ),
        NetworkPlatformClaimProofSource::LinuxAdapter(
            plan_network_linux_adapter_gate(linux_input_for(
                NetworkLinuxAdapterKind::Ebpf,
                Suffix("ebpf"),
            ))
            .expect_value("complete Linux eBPF input should build proof"),
        ),
        NetworkPlatformClaimProofSource::LinuxAdapter(
            plan_network_linux_adapter_gate(linux_input_for(
                NetworkLinuxAdapterKind::Tun,
                Suffix("tun"),
            ))
            .expect_value("complete Linux TUN input should build proof"),
        ),
    ]
}

fn no_unsupported_claims() -> NetworkPlatformUnsupportedClaims {
    NetworkPlatformUnsupportedClaims {
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        generic_platform_support_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
        ui_policy_authority_claimed: false,
    }
}

fn policy_mapping() -> NetworkEvidencePolicyMapping {
    map_network_evidence_grade_to_policy(NetworkEvidencePolicyMappingInput {
        policy_decision_ref: "policy-decision-ref-52".to_owned(),
        parent_rule_ref: "parent-rule-ref-52".to_owned(),
        evidence_refs: vec!["network-evidence-ref-52".to_owned()],
        local_ai_result_ref: Some("local-ai-result-ref-52".to_owned()),
        evidence_grade: NetworkEvidenceGrade::A,
        requested_action: NetworkEvidencePolicyAction::Block,
        adapter_capability_proof_ref: Some("adapter-capability-ref-52".to_owned()),
    })
    .expect_value("policy mapping should parse")
}

fn windows_firewall_input() -> NetworkWindowsFirewallAdapterProofInput {
    NetworkWindowsFirewallAdapterProofInput {
        firewall_adapter_plan_ref: "windows-firewall-plan-ref-52".to_owned(),
        policy_mapping: policy_mapping(),
        requested_action: NetworkWindowsFirewallAdapterAction::BlockOutbound,
        windows_os_scope_ref: "windows-os-scope-ref-52".to_owned(),
        target_kind: NetworkWindowsFirewallTargetKind::RemoteAddress,
        target_ref: "windows-firewall-domain-target-ref-52".to_owned(),
        firewall_rule_ref: "windows-firewall-rule-ref-52".to_owned(),
        capability_state: NetworkWindowsFirewallCapabilityState::Supported,
        adapter_authorization_ref: Some("windows-firewall-adapter-auth-ref-52".to_owned()),
        adapter_capability_proof_ref: Some("windows-firewall-capability-ref-52".to_owned()),
        apply_artifact_ref: Some("windows-firewall-apply-ref-52".to_owned()),
        result_artifact_ref: Some("windows-firewall-result-ref-52".to_owned()),
        rollback_artifact_ref: Some("windows-firewall-rollback-ref-52".to_owned()),
        audit_event_ref: Some("windows-firewall-audit-ref-52".to_owned()),
        dry_run: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        host_firewall_mutation_claimed: false,
        netsh_command_invoked: false,
        powershell_command_invoked: false,
    }
}

fn windows_wfp_input() -> NetworkWindowsWfpGateInput {
    NetworkWindowsWfpGateInput {
        wfp_gate_ref: "windows-wfp-gate-ref-52".to_owned(),
        policy_mapping: policy_mapping(),
        target_ref: "windows-wfp-target-ref-52".to_owned(),
        wfp_provider_ref: "windows-wfp-provider-ref-52".to_owned(),
        wfp_layer_ref: "windows-wfp-layer-ref-52".to_owned(),
        capability_state: NetworkWindowsWfpGateCapabilityState::LabReady,
        administrator_permission_proof_ref: Some("windows-admin-permission-ref-52".to_owned()),
        driver_signing_proof_ref: Some("windows-driver-signing-ref-52".to_owned()),
        driver_package_proof_ref: Some("windows-driver-package-ref-52".to_owned()),
        provider_registration_plan_ref: Some("windows-provider-registration-ref-52".to_owned()),
        layer_capability_matrix_ref: Some("windows-layer-capability-ref-52".to_owned()),
        rollback_plan_ref: Some("windows-wfp-rollback-ref-52".to_owned()),
        lab_result_artifact_ref: Some("windows-wfp-lab-result-ref-52".to_owned()),
        audit_event_ref: Some("windows-wfp-audit-ref-52".to_owned()),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        live_driver_install_claimed: false,
        callout_registration_claimed: false,
        packet_block_claimed: false,
        kernel_payload_inspection_claimed: false,
        command_invocation_claimed: false,
    }
}

fn android_input() -> NetworkAndroidVpnServiceGateInput {
    NetworkAndroidVpnServiceGateInput {
        android_vpn_service_gate_ref: "android-vpn-gate-ref-52".to_owned(),
        policy_mapping: policy_mapping(),
        package_ref: "android-package-ref-52".to_owned(),
        vpn_service_ref: "android-vpn-service-ref-52".to_owned(),
        capability_state: NetworkAndroidVpnServiceCapabilityState::PhysicalDeviceReady,
        vpn_service_declaration_ref: Some("android-vpn-declaration-ref-52".to_owned()),
        user_consent_proof_ref: Some("android-user-consent-ref-52".to_owned()),
        physical_device_proof_ref: Some("android-physical-device-ref-52".to_owned()),
        package_identity_proof_ref: Some("android-package-identity-ref-52".to_owned()),
        virtual_interface_proof_ref: Some("android-virtual-interface-ref-52".to_owned()),
        traffic_observation_proof_ref: Some("android-traffic-observation-ref-52".to_owned()),
        rollback_plan_ref: Some("android-rollback-ref-52".to_owned()),
        audit_event_ref: Some("android-audit-ref-52".to_owned()),
        device_owner_required: true,
        device_owner_proof_ref: Some("android-device-owner-ref-52".to_owned()),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        emulator_only_product_support_claimed: false,
        live_vpn_tunnel_claimed: false,
        packet_block_claimed: false,
        app_package_correlation_claimed: false,
    }
}

fn apple_input_for(
    platform: NetworkAppleNetworkExtensionPlatform,
    suffix: Suffix,
) -> NetworkAppleNetworkExtensionGateInput {
    NetworkAppleNetworkExtensionGateInput {
        apple_network_extension_gate_ref: format!(
            "apple-network-extension-gate-ref-52-{}",
            suffix.0
        ),
        policy_mapping: policy_mapping(),
        platform,
        bundle_ref: format!("apple-bundle-ref-52-{}", suffix.0),
        network_extension_ref: format!("apple-network-extension-ref-52-{}", suffix.0),
        capability_state: NetworkAppleNetworkExtensionCapabilityState::AppleDeviceReady,
        developer_team_proof_ref: Some(format!("apple-developer-team-ref-52-{}", suffix.0)),
        entitlement_approval_proof_ref: Some(format!("apple-entitlement-ref-52-{}", suffix.0)),
        provisioning_profile_proof_ref: Some(format!("apple-provisioning-ref-52-{}", suffix.0)),
        signing_proof_ref: Some(format!("apple-signing-ref-52-{}", suffix.0)),
        device_or_testflight_proof_ref: Some(format!("apple-device-ref-52-{}", suffix.0)),
        network_extension_declaration_ref: Some(format!(
            "apple-extension-declaration-ref-52-{}",
            suffix.0
        )),
        extension_configuration_proof_ref: Some(format!(
            "apple-extension-config-ref-52-{}",
            suffix.0
        )),
        rollback_plan_ref: Some(format!("apple-rollback-ref-52-{}", suffix.0)),
        audit_event_ref: Some(format!("apple-audit-ref-52-{}", suffix.0)),
        supervision_required: true,
        supervision_or_mdm_proof_ref: Some(format!("apple-supervision-mdm-ref-52-{}", suffix.0)),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        simulator_only_product_support_claimed: false,
        live_network_extension_claimed: false,
        packet_block_claimed: false,
        app_level_control_claimed: false,
    }
}

fn linux_input_for(
    adapter_kind: NetworkLinuxAdapterKind,
    suffix: Suffix,
) -> NetworkLinuxAdapterGateInput {
    NetworkLinuxAdapterGateInput {
        linux_adapter_gate_ref: format!("linux-adapter-gate-ref-52-{}", suffix.0),
        policy_mapping: policy_mapping(),
        adapter_kind,
        distro_ref: format!("linux-distro-ref-52-{}", suffix.0),
        kernel_ref: format!("linux-kernel-ref-52-{}", suffix.0),
        capability_state: NetworkLinuxAdapterCapabilityState::DistroReady,
        distro_kernel_proof_ref: Some(format!("linux-distro-kernel-ref-52-{}", suffix.0)),
        permission_proof_ref: Some(format!("linux-permission-ref-52-{}", suffix.0)),
        adapter_api_capability_proof_ref: Some(format!("linux-api-capability-ref-52-{}", suffix.0)),
        adapter_plan_proof_ref: Some(format!("linux-adapter-plan-ref-52-{}", suffix.0)),
        service_manager_scope_proof_ref: Some(format!("linux-service-manager-ref-52-{}", suffix.0)),
        rollback_plan_ref: Some(format!("linux-rollback-ref-52-{}", suffix.0)),
        lab_result_artifact_ref: Some(format!("linux-lab-result-ref-52-{}", suffix.0)),
        audit_event_ref: Some(format!("linux-audit-ref-52-{}", suffix.0)),
        research_only: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        generic_linux_support_claimed: false,
        live_adapter_install_claimed: false,
        packet_filtering_claimed: false,
        kernel_hook_loaded_claimed: false,
        tun_interface_mutation_claimed: false,
        service_manager_install_claimed: false,
    }
}
