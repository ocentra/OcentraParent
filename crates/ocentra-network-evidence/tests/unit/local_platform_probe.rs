use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::adapter_capability_status::*;
use ocentra_network_evidence::local_platform_probe::*;
use ocentra_network_evidence::platform_claims::*;

#[derive(Clone, Copy)]
enum CapabilityRef {
    Missing,
    Present(&'static str),
}

struct EvidenceRef(&'static str);

#[test]
fn local_platform_probe_aligns_windows_android_linux_and_apple_status_rows() {
    let proof =
        build_network_local_platform_probe_proof(input_with_observations(expected_observations()))
            .expect_value("local platform observations should align with adapter status");

    assert_eq!(proof.probe_ref, "local-platform-probe-ref-54");
    assert_eq!(proof.adapter_status_ref, "adapter-capability-status-ref-54");
    assert_eq!(proof.target_count, 8);
    assert_eq!(proof.windows_probe_count, 2);
    assert_eq!(proof.android_probe_count, 1);
    assert_eq!(proof.linux_probe_count, 3);
    assert_eq!(proof.apple_ci_unavailable_count, 2);
    assert!(proof.every_observation_matches_adapter_status);
    assert!(proof.read_only_probes_do_not_execute_adapters);
    assert!(!proof.local_platform_support_claimed);
    assert!(proof.no_live_adapter_execution_claimed);
    assert!(proof.no_enforcement_commands_published);
    assert!(proof.ui_has_no_policy_authority);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
}

#[test]
fn local_platform_probe_rejects_adapter_execution_attempts() {
    let mut observations = expected_observations();
    observations[0].adapter_execution_attempted = true;

    assert_eq!(
        build_network_local_platform_probe_proof(input_with_observations(observations)),
        Err(
            NetworkLocalPlatformProbeError::AdapterExecutionAttemptRejected(
                NetworkPlatformClaimTarget::WindowsFirewall
            )
        )
    );
}

#[test]
fn local_platform_probe_rejects_product_support_and_content_claims() {
    let mut unsupported = no_unsupported_claims();
    unsupported.production_platform_support_claimed = true;

    assert_eq!(
        build_network_local_platform_probe_proof(NetworkLocalPlatformProbeInput {
            unsupported_claims: unsupported,
            ..input_with_observations(expected_observations())
        }),
        Err(NetworkLocalPlatformProbeError::ProductionPlatformSupportClaimRejected)
    );

    let mut observations = expected_observations();
    observations[2].exact_url_claimed = true;
    assert_eq!(
        build_network_local_platform_probe_proof(input_with_observations(observations)),
        Err(NetworkLocalPlatformProbeError::ExactUrlClaimRejected)
    );
}

#[test]
fn local_platform_probe_rejects_status_mismatches() {
    let mut observations = expected_observations();
    observations[3].capability_status = NetworkAdapterCapabilityStatusState::ManualRequired;

    assert_eq!(
        build_network_local_platform_probe_proof(input_with_observations(observations)),
        Err(NetworkLocalPlatformProbeError::CapabilityStatusMismatch(
            NetworkPlatformClaimTarget::LinuxNftables
        ))
    );
}

#[test]
fn local_platform_probe_rejects_lab_ready_state_for_manual_status() {
    let mut observations = expected_observations();
    observations[4].probe_state = NetworkLocalPlatformProbeState::LabReady;

    assert_eq!(
        build_network_local_platform_probe_proof(input_with_observations(observations)),
        Err(
            NetworkLocalPlatformProbeError::ProbeStateDoesNotSupportCapability(
                NetworkPlatformClaimTarget::LinuxEbpf
            )
        )
    );
}

#[test]
fn local_platform_probe_requires_read_only_probe_execution_refs() {
    let mut observations = expected_observations();
    observations[0].read_only_probe_executed = false;

    assert_eq!(
        build_network_local_platform_probe_proof(input_with_observations(observations)),
        Err(
            NetworkLocalPlatformProbeError::ReadOnlyProbeExecutionRefMissing(
                NetworkPlatformClaimTarget::WindowsFirewall
            )
        )
    );
}

fn input_with_observations(
    observations: Vec<NetworkLocalPlatformProbeObservation>,
) -> NetworkLocalPlatformProbeInput {
    NetworkLocalPlatformProbeInput {
        probe_ref: " local-platform-probe-ref-54 ".to_owned(),
        adapter_status: adapter_status(),
        observations,
        unsupported_claims: no_unsupported_claims(),
    }
}

fn adapter_status() -> NetworkAdapterCapabilityStatusProof {
    build_network_adapter_capability_status(NetworkAdapterCapabilityStatusInput {
        status_ref: " adapter-capability-status-ref-54 ".to_owned(),
        platform_manifest: manifest(),
        portal_status_proof_ref: Some(" portal-status-proof-ref-54 ".to_owned()),
        generic_platform_support_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
        ui_policy_authority_claimed: false,
        broader_platform_capability_ux_claimed: false,
    })
    .expect_value("manifest should produce adapter status")
}

fn manifest() -> NetworkPlatformClaimManifestProof {
    let entries = vec![
        entry(
            NetworkPlatformClaimTarget::WindowsFirewall,
            NetworkPlatformClaimState::DryRun,
            CapabilityRef::Present("windows-firewall.profile-state-ref"),
        ),
        entry(
            NetworkPlatformClaimTarget::WindowsWfp,
            NetworkPlatformClaimState::ManualRequired,
            CapabilityRef::Missing,
        ),
        entry(
            NetworkPlatformClaimTarget::AndroidVpnService,
            NetworkPlatformClaimState::ManualRequired,
            CapabilityRef::Missing,
        ),
        entry(
            NetworkPlatformClaimTarget::LinuxNftables,
            NetworkPlatformClaimState::Ready,
            CapabilityRef::Present("linux-adapter.nft-wsl-tool-ref"),
        ),
        entry(
            NetworkPlatformClaimTarget::LinuxEbpf,
            NetworkPlatformClaimState::ManualRequired,
            CapabilityRef::Missing,
        ),
        entry(
            NetworkPlatformClaimTarget::LinuxTun,
            NetworkPlatformClaimState::ManualRequired,
            CapabilityRef::Missing,
        ),
        entry(
            NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs,
            NetworkPlatformClaimState::Unavailable,
            CapabilityRef::Missing,
        ),
        entry(
            NetworkPlatformClaimTarget::AppleNetworkExtensionIos,
            NetworkPlatformClaimState::Unavailable,
            CapabilityRef::Missing,
        ),
    ];
    let manual_followups = entries
        .iter()
        .filter(|entry| !entry.missing_required_artifacts.is_empty())
        .map(|entry| NetworkPlatformClaimManualFollowup {
            target: entry.target,
            missing_required_artifacts: entry.missing_required_artifacts.clone(),
        })
        .collect();

    NetworkPlatformClaimManifestProof {
        manifest_ref: " platform-claim-manifest-ref-54 ".to_owned(),
        ready_claims: 1,
        dry_run_claims: 1,
        research_only_claims: 0,
        manual_required_claims: 4,
        unavailable_claims: 2,
        entries,
        manual_followups,
        every_claim_names_platform: true,
        every_claim_names_permission_or_manual_followup: true,
        every_claim_names_audit_ref: true,
        no_enforcement_commands_published: true,
        no_live_adapter_execution_claimed: true,
        ui_has_no_policy_authority: true,
    }
}

fn entry(
    target: NetworkPlatformClaimTarget,
    claim_state: NetworkPlatformClaimState,
    capability_ref: CapabilityRef,
) -> NetworkPlatformClaimEntry {
    let capability_ref = match capability_ref {
        CapabilityRef::Missing => None,
        CapabilityRef::Present(value) => Some(value),
    };
    let missing_required_artifacts =
        if capability_ref.is_some() && claim_state != NetworkPlatformClaimState::Unavailable {
            Vec::new()
        } else {
            vec![format!("{target:?}.manual-artifact")]
        };
    NetworkPlatformClaimEntry {
        target,
        claim_state,
        policy_decision_ref: "policy-decision-ref-54".to_owned(),
        parent_rule_ref: "parent-rule-ref-54".to_owned(),
        evidence_refs: vec!["network-evidence-ref-54".to_owned()],
        device_or_os_refs: vec![format!("{target:?}.device-ref")],
        permission_or_entitlement_refs: capability_ref
            .map_or_else(Vec::new, |value| vec![format!("{value}.permission")]),
        adapter_capability_refs: capability_ref
            .map_or_else(Vec::new, |value| vec![value.to_owned()]),
        missing_required_artifacts,
        audit_refs: vec![format!("{target:?}.audit-ref")],
        adapter_authorized_by_proof: false,
        enforcement_command_published: false,
    }
}

fn expected_observations() -> Vec<NetworkLocalPlatformProbeObservation> {
    vec![
        observation(
            NetworkPlatformClaimTarget::WindowsFirewall,
            NetworkLocalPlatformProbeHost::Windows,
            NetworkLocalPlatformProbeState::ReadOnlyObserved,
            NetworkAdapterCapabilityStatusState::DryRun,
            vec![EvidenceRef("netsh-firewall-profile-state-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::WindowsWfp,
            NetworkLocalPlatformProbeHost::Windows,
            NetworkLocalPlatformProbeState::ManualRequired,
            NetworkAdapterCapabilityStatusState::ManualRequired,
            vec![EvidenceRef("pktmon-driver-access-denied-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::AndroidVpnService,
            NetworkLocalPlatformProbeHost::AndroidSdk,
            NetworkLocalPlatformProbeState::ManualRequired,
            NetworkAdapterCapabilityStatusState::ManualRequired,
            vec![EvidenceRef("android-sdk-adb-emulator-no-device-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::LinuxNftables,
            NetworkLocalPlatformProbeHost::LinuxWsl,
            NetworkLocalPlatformProbeState::LabReady,
            NetworkAdapterCapabilityStatusState::DistroReady,
            vec![EvidenceRef("wsl-ubuntu-nft-ip-tcpdump-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::LinuxEbpf,
            NetworkLocalPlatformProbeHost::LinuxWsl,
            NetworkLocalPlatformProbeState::ManualRequired,
            NetworkAdapterCapabilityStatusState::ManualRequired,
            vec![EvidenceRef("wsl-bpftool-missing-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::LinuxTun,
            NetworkLocalPlatformProbeHost::LinuxWsl,
            NetworkLocalPlatformProbeState::ManualRequired,
            NetworkAdapterCapabilityStatusState::ManualRequired,
            vec![EvidenceRef("wsl-tun-manual-check-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::AppleNetworkExtensionMacOs,
            NetworkLocalPlatformProbeHost::MacOsCi,
            NetworkLocalPlatformProbeState::CiOnly,
            NetworkAdapterCapabilityStatusState::Unavailable,
            vec![EvidenceRef("windows-host-macos-ci-unavailable-ref")],
        ),
        observation(
            NetworkPlatformClaimTarget::AppleNetworkExtensionIos,
            NetworkLocalPlatformProbeHost::IosCi,
            NetworkLocalPlatformProbeState::CiOnly,
            NetworkAdapterCapabilityStatusState::Unavailable,
            vec![EvidenceRef("windows-host-ios-ci-unavailable-ref")],
        ),
    ]
}

fn observation(
    target: NetworkPlatformClaimTarget,
    host: NetworkLocalPlatformProbeHost,
    probe_state: NetworkLocalPlatformProbeState,
    capability_status: NetworkAdapterCapabilityStatusState,
    evidence_refs: Vec<EvidenceRef>,
) -> NetworkLocalPlatformProbeObservation {
    NetworkLocalPlatformProbeObservation {
        target,
        host,
        probe_state,
        capability_status,
        evidence_refs: evidence_refs
            .into_iter()
            .map(|value| value.0.to_owned())
            .collect(),
        read_only_probe_executed: probe_state == NetworkLocalPlatformProbeState::ReadOnlyObserved,
        adapter_execution_attempted: false,
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        production_platform_support_claimed: false,
    }
}

fn no_unsupported_claims() -> NetworkLocalPlatformProbeUnsupportedClaims {
    NetworkLocalPlatformProbeUnsupportedClaims {
        exact_url_claimed: false,
        decrypted_payload_claimed: false,
        page_content_claimed: false,
        live_adapter_execution_claimed: false,
        enforcement_command_claimed: false,
        ui_policy_authority_claimed: false,
        production_platform_support_claimed: false,
    }
}
