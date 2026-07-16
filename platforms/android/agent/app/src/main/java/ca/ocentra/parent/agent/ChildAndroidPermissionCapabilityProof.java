package ca.ocentra.parent.agent;

import android.os.Bundle;

public final class ChildAndroidPermissionCapabilityProof {
    public static final String SCHEMA_VERSION = "child-android-permission-capability-proof";
    public static final String PACKAGE_ID = "ca.ocentra.parent.agent";
    public static final String NATIVE_BRIDGE_CLASS =
        "ca.ocentra.parent.agent.ChildAndroidPermissionCapabilityProof";
    public static final String COMMAND_PERMISSION_CAPABILITY_SNAPSHOT_GET =
        "child.android.permission.capability.snapshot.get";
    public static final String COMMAND_PERMISSION_PACKAGE_PROOF_GET =
        "child.android.permission.package.proof.get";
    public static final String COMMAND_PERMISSION_RUNTIME_MANUAL_PROOF_GET =
        "child.android.permission.runtime.manual-proof.get";
    public static final String EVENT_PERMISSION_CAPABILITY_SNAPSHOT_REPORTED =
        "child.android.permission.capability.snapshot.reported";
    public static final String EVENT_PERMISSION_PACKAGE_PROOF_REPORTED =
        "child.android.permission.package.proof.reported";
    public static final String EVENT_PERMISSION_RUNTIME_MANUAL_PROOF_REPORTED =
        "child.android.permission.runtime.manual-proof.reported";
    public static final String BRIDGE_STATE = "package-local-scaffold";
    public static final String EXTERNAL_TRANSPORT_STATE = "not-implemented";
    public static final String POST_NOTIFICATIONS_GRANT_STATE = "manual-runtime-required";
    public static final String USAGE_STATS_GRANT_STATE = "manual-settings-required";
    public static final String ACCESSIBILITY_STATE = "not-declared";
    public static final String VPN_DNS_STATE = "not-declared";
    public static final String DEVICE_OWNER_STATE = "blocked-without-enrollment";
    public static final String MANAGED_PROFILE_STATE = "blocked-without-enrollment";
    public static final String FIELD_PERMISSION_BRIDGE_STATE = "permissionBridgeState";

    private ChildAndroidPermissionCapabilityProof() {}

    public static Bundle createPermissionCapabilityBundle() {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageId", PACKAGE_ID);
        status.putString("nativeBridgeClass", NATIVE_BRIDGE_CLASS);
        status.putString(FIELD_PERMISSION_BRIDGE_STATE, BRIDGE_STATE);
        status.putString("externalTransportState", EXTERNAL_TRANSPORT_STATE);
        status.putString("postNotificationsGrantState", POST_NOTIFICATIONS_GRANT_STATE);
        status.putString("usageStatsGrantState", USAGE_STATS_GRANT_STATE);
        status.putString("accessibilityState", ACCESSIBILITY_STATE);
        status.putString("vpnDnsState", VPN_DNS_STATE);
        status.putString("deviceOwnerState", DEVICE_OWNER_STATE);
        status.putString("managedProfileState", MANAGED_PROFILE_STATE);
        status.putStringArray(
            "commands",
            new String[] {
                COMMAND_PERMISSION_CAPABILITY_SNAPSHOT_GET,
                COMMAND_PERMISSION_PACKAGE_PROOF_GET,
                COMMAND_PERMISSION_RUNTIME_MANUAL_PROOF_GET
            }
        );
        status.putStringArray(
            "events",
            new String[] {
                EVENT_PERMISSION_CAPABILITY_SNAPSHOT_REPORTED,
                EVENT_PERMISSION_PACKAGE_PROOF_REPORTED,
                EVENT_PERMISSION_RUNTIME_MANUAL_PROOF_REPORTED
            }
        );
        status.putStringArray(
            "declaredManifestPermissions",
            new String[] {
                "android.permission.FOREGROUND_SERVICE",
                "android.permission.FOREGROUND_SERVICE_DATA_SYNC",
                "android.permission.FOREGROUND_SERVICE_MEDIA_PROJECTION",
                "android.permission.POST_NOTIFICATIONS",
                "android.permission.ACCESS_COARSE_LOCATION",
                "android.permission.ACCESS_FINE_LOCATION",
                "android.permission.ACCESS_BACKGROUND_LOCATION"
            }
        );
        status.putStringArray(
            "manualRuntimePermissions",
            new String[] {
                "android.permission.POST_NOTIFICATIONS",
                "android.permission.ACCESS_COARSE_LOCATION",
                "android.permission.ACCESS_FINE_LOCATION",
                "android.permission.ACCESS_BACKGROUND_LOCATION"
            }
        );
        status.putStringArray(
            "settingsGrantRequiredPermissions",
            new String[] { "android.permission.PACKAGE_USAGE_STATS" }
        );
        status.putStringArray(
            "notDeclaredAdapters",
            new String[] { "accessibility-service", "vpn-dns-service" }
        );
        status.putStringArray(
            "blockedEnrollmentStates",
            new String[] { "device-owner-policy", "managed-profile" }
        );
        status.putStringArray(
            "manualPackageLifecyclePhases",
            new String[] {
                "background-service-start",
                "install",
                "update",
                "reboot-recovery",
                "uninstall"
            }
        );
        return status;
    }
}
