use super::constants::{
    CHILD_IOS_ENTITLEMENT_BUNDLE_ID, CHILD_IOS_ENTITLEMENT_CLASS_NAME,
    CHILD_IOS_ENTITLEMENT_UPDATED_AT,
};
use super::identifiers::{boundary, bundle_id, class_name, requirement, timestamp};
use super::*;

pub(super) fn sample_child_ios_entitlement_capability_read_model(
) -> ChildIosEntitlementCapabilityReadModel {
    ChildIosEntitlementCapabilityReadModel {
        schema_version: CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION.to_string(),
        bundle_id: bundle_id(CHILD_IOS_ENTITLEMENT_BUNDLE_ID),
        status_surface_class: class_name(CHILD_IOS_ENTITLEMENT_CLASS_NAME),
        protocol_bridge_proof: ChildIosEntitlementProtocolBridgeProof {
            bundle_id: bundle_id(CHILD_IOS_ENTITLEMENT_BUNDLE_ID),
            status_surface_class: class_name(CHILD_IOS_ENTITLEMENT_CLASS_NAME),
            bridge_state: ChildIosEntitlementBridgeState::SimulatorScaffold,
            external_transport_state: ChildIosEntitlementBridgeState::NotImplemented,
            commands: vec![
                ChildIosEntitlementProtocolCommand::CapabilitySnapshotGet,
                ChildIosEntitlementProtocolCommand::PackageProofGet,
                ChildIosEntitlementProtocolCommand::ManualProofGet,
            ],
            events: vec![
                ChildIosEntitlementProtocolEvent::CapabilitySnapshotReported,
                ChildIosEntitlementProtocolEvent::PackageProofReported,
                ChildIosEntitlementProtocolEvent::ManualProofReported,
            ],
            runtime_owner: ChildIosEntitlementRuntimeOwner::IosSwiftScaffold,
            proof_requirement: requirement(
                "iOS simulator scaffold status surface names capability-only launch, recovery, entitlement, provisioning, and supervision states",
            ),
            claim_boundary: boundary(
                "status surface is capability-only; no hidden daemon, persistent background service, launch recovery, external child-agent transport, Apple entitlement, provisioning, or device proof is claimed",
            ),
        },
        surface_proofs: surface_proofs::sample_surface_proofs(),
        package_lifecycle_proofs: lifecycle_proofs::sample_package_lifecycle_proofs(),
        claim_boundaries: sample_claim_boundaries(),
        updated_at: timestamp(CHILD_IOS_ENTITLEMENT_UPDATED_AT),
    }
}

fn sample_claim_boundaries() -> ChildIosEntitlementClaimBoundaries {
    ChildIosEntitlementClaimBoundaries {
        simulator_package: boundary(
            "Xcode project target, bundle id, plist, status view, and package script are source proof only",
        ),
        launch_availability: boundary(
            "simulator and physical-device launch availability remain manual-required or device-proof-required without Apple host or device artifacts",
        ),
        family_controls: boundary(
            "Family Controls remains entitlement-required without Apple approval and device artifacts",
        ),
        device_activity: boundary(
            "DeviceActivity remains entitlement-required without schedule and event artifacts",
        ),
        screen_time: boundary(
            "Screen Time API remains entitlement-required without authorization and behavior artifacts",
        ),
        network_extension: boundary(
            "Network Extension remains entitlement-required without filtering artifacts",
        ),
        notifications: boundary("notification authorization and delivery remain manual-required"),
        background_execution: boundary(
            "background execution remains manual-required without UIBackgroundModes and device proof",
        ),
        recovery_behavior: boundary(
            "launch recovery remains not-implemented; no iOS daemon, relaunch, or persistent background recovery is claimed",
        ),
        provisioning_profile: boundary(
            "provisioning remains manual-required without Apple signing credentials, provisioning profile artifacts, and install evidence",
        ),
        supervision: boundary(
            "supervision remains manual-required without supervised-device enrollment and device artifacts",
        ),
        signing_entitlements: boundary(
            "signing and entitlements remain signing-required; simulator script disables signing",
        ),
        testflight: boundary(
            "TestFlight and App Store distribution remain device-proof-required or planned",
        ),
        device_proof: boundary(
            "physical-device install and runtime behavior remain device-proof-required",
        ),
        capability_only_state: boundary(
            "iOS child runtime remains capability-only; no hidden daemon or persistent background service is claimed",
        ),
        external_transport: boundary(
            "no external LAN or WebSocket iOS child-agent transport is claimed",
        ),
    }
}
