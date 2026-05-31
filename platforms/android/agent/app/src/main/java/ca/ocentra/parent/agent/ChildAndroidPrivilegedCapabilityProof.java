package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidPrivilegedCapabilityProof {
    public static final String SCHEMA_VERSION = "child-android-privileged-capability-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidPrivilegedCapabilityProof";
    public static final String COMMAND_PRIVILEGED_CAPABILITY_SNAPSHOT_GET =
        "child.android.privileged.capability.snapshot.get";
    public static final String COMMAND_PRIVILEGED_SETTINGS_PROOF_GET =
        "child.android.privileged.settings-proof.get";
    public static final String COMMAND_PRIVILEGED_ENROLLMENT_PROOF_GET =
        "child.android.privileged.enrollment-proof.get";
    public static final String EVENT_PRIVILEGED_CAPABILITY_SNAPSHOT_REPORTED =
        "child.android.privileged.capability.snapshot.reported";
    public static final String EVENT_PRIVILEGED_SETTINGS_PROOF_REPORTED =
        "child.android.privileged.settings-proof.reported";
    public static final String EVENT_PRIVILEGED_ENROLLMENT_PROOF_REPORTED =
        "child.android.privileged.enrollment-proof.reported";
    public static final String BRIDGE_STATE = "package-local-scaffold";
    public static final String EXTERNAL_TRANSPORT_STATE = "not-implemented";
    public static final String USAGE_STATS_SETTINGS_STATE = "manual-settings-required";
    public static final String USAGE_STATS_OBSERVATION_STATE = "manual-device-proof-required";
    public static final String ACCESSIBILITY_STATE = "not-declared";
    public static final String VPN_SERVICE_STATE = "not-declared";
    public static final String DNS_FILTERING_STATE = "not-implemented";
    public static final String DEVICE_OWNER_STATE = "blocked-without-enrollment";
    public static final String MANAGED_PROFILE_STATE = "blocked-without-enrollment";
    public static final String PHYSICAL_DEVICE_STATE = "device-proof-required";
    public static final String CHILD_AGENT_PARITY_STATE = "not-claimed";
    public static final String FIELD_PRIVILEGED_BRIDGE_STATE = "privilegedBridgeState";

    private ChildAndroidPrivilegedCapabilityProof() {}

    public static Bundle createPrivilegedCapabilityBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_PRIVILEGED_BRIDGE_STATE, BRIDGE_STATE);
        status.putString("externalTransportState", EXTERNAL_TRANSPORT_STATE);
        status.putString("usageStatsSettingsState", USAGE_STATS_SETTINGS_STATE);
        status.putString("usageStatsObservationState", USAGE_STATS_OBSERVATION_STATE);
        status.putString("accessibilityState", ACCESSIBILITY_STATE);
        status.putString("vpnServiceState", VPN_SERVICE_STATE);
        status.putString("dnsFilteringState", DNS_FILTERING_STATE);
        status.putString("deviceOwnerState", DEVICE_OWNER_STATE);
        status.putString("managedProfileState", MANAGED_PROFILE_STATE);
        status.putString("physicalDeviceState", PHYSICAL_DEVICE_STATE);
        status.putString("childAgentParityState", CHILD_AGENT_PARITY_STATE);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_PRIVILEGED_CAPABILITY_SNAPSHOT_GET,
                COMMAND_PRIVILEGED_SETTINGS_PROOF_GET,
                COMMAND_PRIVILEGED_ENROLLMENT_PROOF_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_PRIVILEGED_CAPABILITY_SNAPSHOT_REPORTED,
                EVENT_PRIVILEGED_SETTINGS_PROOF_REPORTED,
                EVENT_PRIVILEGED_ENROLLMENT_PROOF_REPORTED
            }
        );
        status.putStringArray(
            "privilegedCapabilityLabels",
            new String[] {
                "usage-stats-settings-access=settings-grant-required",
                "usage-stats-observation=manual-device-proof",
                "accessibility-service-adapter=not-implemented",
                "vpn-service-adapter=not-implemented",
                "dns-filtering-adapter=not-implemented",
                "device-owner-enrollment=blocked",
                "managed-profile-enrollment=blocked",
                "physical-device-proof=device-proof-required",
                "external-child-agent-transport=not-implemented"
            }
        );
        status.putStringArray(
            "notDeclaredPrivilegedAdapters",
            new String[] {
                "accessibility-service-adapter",
                "vpn-service-adapter",
                "dns-filtering-adapter",
                "device-owner-enrollment",
                "managed-profile-enrollment"
            }
        );
        status.putStringArray(
            "manualDeviceProofRequired",
            new String[] { "usage-stats-observation", "physical-device-proof" }
        );
        status.putStringArray(
            "blockedEnrollmentStates",
            new String[] { "device-owner-enrollment", "managed-profile-enrollment" }
        );
        return status;
    }
}
