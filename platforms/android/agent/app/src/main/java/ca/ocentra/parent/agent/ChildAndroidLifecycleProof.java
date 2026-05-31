package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidLifecycleProof {
    public static final String SCHEMA_VERSION = "child-android-protocol-package-lifecycle-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String LAUNCH_ACTIVITY = "ca.ocentra.parent.agent/.MainActivity";
    public static final String FOREGROUND_SERVICE = "ca.ocentra.parent.agent/.OcentraParentAgentService";
    public static final String NATIVE_BRIDGE_CLASS = "ca.ocentra.parent.agent.ChildAndroidLifecycleProof";
    public static final String COMMAND_LIFECYCLE_SNAPSHOT_GET = "child.android.lifecycle.snapshot.get";
    public static final String COMMAND_CAPABILITIES_SNAPSHOT_GET = "child.android.capabilities.snapshot.get";
    public static final String COMMAND_PACKAGE_LIFECYCLE_PROOF_GET =
        "child.android.package.lifecycle.proof.get";
    public static final String EVENT_LIFECYCLE_SNAPSHOT_REPORTED =
        "child.android.lifecycle.snapshot.reported";
    public static final String EVENT_CAPABILITY_SNAPSHOT_REPORTED =
        "child.android.capability.snapshot.reported";
    public static final String EVENT_PACKAGE_LIFECYCLE_PROOF_REPORTED =
        "child.android.package.lifecycle.proof.reported";
    public static final String BRIDGE_STATE = "package-local-scaffold";
    public static final String EXTERNAL_TRANSPORT_STATE = "not-implemented";
    public static final String FOREGROUND_SERVICE_PROOF_STATE = "ci-mechanical-proof";
    public static final String TYPED_PROTOCOL_BRIDGE_PROOF_STATE = "ci-mechanical-proof";
    public static final String PACKAGE_LIFECYCLE_PROOF_STATE = "ci-mechanical-proof";
    public static final String DEVICE_OWNER_PROOF_STATE = "manual-required";
    public static final String ACCESSIBILITY_PROOF_STATE = "manual-required";
    public static final String VPN_DNS_PROOF_STATE = "manual-required";
    public static final String FIELD_BRIDGE_STATE = "bridgeState";

    private ChildAndroidLifecycleProof() {}

    public static Bundle createStatusBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("launchActivity", LAUNCH_ACTIVITY);
        status.putString("foregroundService", FOREGROUND_SERVICE);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_BRIDGE_STATE, BRIDGE_STATE);
        status.putString("externalTransportState", EXTERNAL_TRANSPORT_STATE);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_LIFECYCLE_SNAPSHOT_GET,
                COMMAND_CAPABILITIES_SNAPSHOT_GET,
                COMMAND_PACKAGE_LIFECYCLE_PROOF_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_LIFECYCLE_SNAPSHOT_REPORTED,
                EVENT_CAPABILITY_SNAPSHOT_REPORTED,
                EVENT_PACKAGE_LIFECYCLE_PROOF_REPORTED
            }
        );
        status.putStringArray(
            "manualRequiredCapabilities",
            new String[] {
                "notifications",
                "usage-stats",
                "accessibility-service",
                "vpn-dns-filtering",
                "device-owner-policy",
                "managed-profile"
            }
        );
        return status;
    }
}
