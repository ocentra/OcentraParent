package ca.ocentra.parent.agent;

import android.app.admin.DevicePolicyManager;
import android.content.Context;
import android.os.Bundle;

public final class AppGameAndroidAuthorityPreflight {
    public static final String SCHEMA_VERSION = "app-game-android-authority-preflight";
    public static final String FIELD_PREFLIGHT_STATE = "preflightState";
    public static final String FIELD_OWNER_STATE = "ownerState";
    public static final String FIELD_DEVICE_OWNER = "deviceOwner";
    public static final String FIELD_PROFILE_OWNER = "profileOwner";
    public static final String FIELD_PROVISIONING_STATE = "provisioningState";
    public static final String FIELD_DEVICE_ADMIN_RECEIVER_DECLARED =
        "deviceAdminReceiverDeclared";
    public static final String OWNER_AUTHORITY_PROVED = "owner-authority-proved";
    public static final String OWNER_AUTHORITY_NOT_PROVED = "owner-authority-not-proved";
    public static final String OWNER_AUTHORITY_UNAVAILABLE = "owner-authority-unavailable";
    public static final String ACTION_BLOCKED = "blocked-before-adapter-dispatch";
    public static final String ACTION_READY_FOR_ADAPTER_PREFLIGHT = "ready-for-adapter-preflight";

    private static final String OWNER_NONE = "no-owner";
    private static final String OWNER_DEVICE = "device-owner";
    private static final String OWNER_PROFILE = "profile-owner";
    private static final String OWNER_BOTH = "device-and-profile-owner";
    private static final String ACTION_HIDE = "hide";
    private static final String ACTION_SUSPEND = "suspend";
    private static final String ACTION_UNINSTALL_BLOCK = "uninstall-block";
    private static final String ACTION_LOCK_TASK = "lock-task";
    private static final String ACTION_MANAGED_CONFIGURATION = "managed-configuration";

    private AppGameAndroidAuthorityPreflight() {}

    public static Bundle createAuthorityPreflightBundle(Context context) {
        Bundle status = new Bundle();
        status.putString("schemaVersion", SCHEMA_VERSION);
        status.putString("packageScope", "self");
        status.putString(FIELD_PROVISIONING_STATE, "external-provisioning-required");
        status.putBoolean(FIELD_DEVICE_ADMIN_RECEIVER_DECLARED, false);
        status.putBoolean("rawPackageNamesStored", false);
        status.putBoolean("rawDeviceSerialStored", false);
        status.putBoolean("adapterDispatchClaimed", false);
        status.putBoolean("platformEnforcementClaimed", false);
        status.putBoolean("childDeviceDeliveryClaimed", false);
        DevicePolicyManager manager = devicePolicyManager(context);
        if (manager == null) {
            putUnavailable(status);
            return status;
        }
        final boolean deviceOwner;
        final boolean profileOwner;
        try {
            String packageName = context.getPackageName();
            deviceOwner = manager.isDeviceOwnerApp(packageName);
            profileOwner = manager.isProfileOwnerApp(packageName);
        } catch (RuntimeException error) {
            putUnavailable(status);
            return status;
        }
        status.putBoolean(FIELD_DEVICE_OWNER, deviceOwner);
        status.putBoolean(FIELD_PROFILE_OWNER, profileOwner);
        status.putString(FIELD_OWNER_STATE, ownerState(deviceOwner, profileOwner));
        boolean ownerAuthorityProved = deviceOwner || profileOwner;
        status.putString(
            FIELD_PREFLIGHT_STATE,
            ownerAuthorityProved ? OWNER_AUTHORITY_PROVED : OWNER_AUTHORITY_NOT_PROVED
        );
        putActionStates(status, ownerAuthorityProved);
        status.putStringArray(
            "openGaps",
            ownerAuthorityProved ?
                new String[] {
                    "android-owner-provisioning-and-device-admin-receiver-not-wired",
                    "android-adapter-dispatch-not-proved",
                    "android-platform-enforcement-not-proved",
                    "android-child-device-delivery-not-proved"
                } :
                new String[] {
                    "android-device-owner-or-profile-owner-not-proved",
                    "android-owner-provisioning-and-device-admin-receiver-not-wired",
                    "android-adapter-dispatch-not-proved",
                    "android-platform-enforcement-not-proved",
                    "android-child-device-delivery-not-proved"
                }
        );
        return status;
    }

    private static DevicePolicyManager devicePolicyManager(Context context) {
        if (context == null) {
            return null;
        }
        Object service = context.getSystemService(Context.DEVICE_POLICY_SERVICE);
        return service instanceof DevicePolicyManager ? (DevicePolicyManager) service : null;
    }

    private static String ownerState(boolean deviceOwner, boolean profileOwner) {
        if (deviceOwner && profileOwner) {
            return OWNER_BOTH;
        }
        if (deviceOwner) {
            return OWNER_DEVICE;
        }
        if (profileOwner) {
            return OWNER_PROFILE;
        }
        return OWNER_NONE;
    }

    private static void putActionStates(Bundle status, boolean ownerAuthorityProved) {
        String actionState = ownerAuthorityProved ? ACTION_READY_FOR_ADAPTER_PREFLIGHT : ACTION_BLOCKED;
        status.putString(ACTION_HIDE, actionState);
        status.putString(ACTION_SUSPEND, actionState);
        status.putString(ACTION_UNINSTALL_BLOCK, actionState);
        status.putString(ACTION_LOCK_TASK, actionState);
        status.putString(ACTION_MANAGED_CONFIGURATION, actionState);
    }

    private static void putUnavailable(Bundle status) {
        status.putString(FIELD_PREFLIGHT_STATE, OWNER_AUTHORITY_UNAVAILABLE);
        status.putString(FIELD_OWNER_STATE, "owner-state-unavailable");
        status.putBoolean(FIELD_DEVICE_OWNER, false);
        status.putBoolean(FIELD_PROFILE_OWNER, false);
        putActionStates(status, false);
        status.putStringArray(
            "openGaps",
            new String[] {
                "android-device-owner-or-profile-owner-not-proved",
                "android-owner-provisioning-and-device-admin-receiver-not-wired",
                "android-device-policy-manager-unavailable",
                "android-adapter-dispatch-not-proved",
                "android-platform-enforcement-not-proved",
                "android-child-device-delivery-not-proved"
            }
        );
    }
}
