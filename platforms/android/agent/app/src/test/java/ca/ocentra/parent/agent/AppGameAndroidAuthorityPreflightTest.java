package ca.ocentra.parent.agent;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import android.app.Application;
import android.app.admin.DevicePolicyManager;
import android.content.Context;
import android.os.Bundle;

import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.RobolectricTestRunner;
import org.robolectric.RuntimeEnvironment;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidAuthorityPreflightTest {
    private static final String[] ACTION_KEYS = {
        "hide",
        "suspend",
        "uninstall-block",
        "lock-task",
        "managed-configuration"
    };

    @Test
    public void nullContextReportsUnavailableAndBlocksEveryAction() {
        Bundle status = AppGameAndroidAuthorityPreflight.createAuthorityPreflightBundle(null);

        assertEquals(
            AppGameAndroidAuthorityPreflight.OWNER_AUTHORITY_UNAVAILABLE,
            status.getString(AppGameAndroidAuthorityPreflight.FIELD_PREFLIGHT_STATE)
        );
        assertEquals("owner-state-unavailable", status.getString(
            AppGameAndroidAuthorityPreflight.FIELD_OWNER_STATE
        ));
        assertFalse(status.getBoolean(AppGameAndroidAuthorityPreflight.FIELD_DEVICE_OWNER));
        assertFalse(status.getBoolean(AppGameAndroidAuthorityPreflight.FIELD_PROFILE_OWNER));
        assertAllActionsBlocked(status);
        assertTrue(status.containsKey("openGaps"));
        assertArrayEquals(
            new String[] {
                "android-device-owner-or-profile-owner-not-proved",
                "android-owner-provisioning-and-device-admin-receiver-not-wired",
                "android-device-policy-manager-unavailable",
                "android-adapter-dispatch-not-proved",
                "android-platform-enforcement-not-proved",
                "android-child-device-delivery-not-proved"
            },
            status.getStringArray("openGaps")
        );
        assertRedactionAndNonClaimFields(status);
    }

    @Test
    public void unprovisionedApplicationContextFailsClosed() {
        Application application = RuntimeEnvironment.getApplication();
        DevicePolicyManager manager = (DevicePolicyManager) application.getSystemService(
            Context.DEVICE_POLICY_SERVICE
        );
        assertNotNull(manager);
        assertFalse(manager.isDeviceOwnerApp(application.getPackageName()));
        assertFalse(manager.isProfileOwnerApp(application.getPackageName()));

        Bundle status = AppGameAndroidAuthorityPreflight.createAuthorityPreflightBundle(application);

        assertEquals(
            AppGameAndroidAuthorityPreflight.OWNER_AUTHORITY_NOT_PROVED,
            status.getString(AppGameAndroidAuthorityPreflight.FIELD_PREFLIGHT_STATE)
        );
        assertEquals("no-owner", status.getString(
            AppGameAndroidAuthorityPreflight.FIELD_OWNER_STATE
        ));
        assertFalse(status.getBoolean(AppGameAndroidAuthorityPreflight.FIELD_DEVICE_OWNER));
        assertFalse(status.getBoolean(AppGameAndroidAuthorityPreflight.FIELD_PROFILE_OWNER));
        assertAllActionsBlocked(status);
        assertTrue(status.containsKey("openGaps"));
        assertArrayEquals(
            new String[] {
                "android-device-owner-or-profile-owner-not-proved",
                "android-owner-provisioning-and-device-admin-receiver-not-wired",
                "android-adapter-dispatch-not-proved",
                "android-platform-enforcement-not-proved",
                "android-child-device-delivery-not-proved"
            },
            status.getStringArray("openGaps")
        );
        assertEquals("app-game-android-authority-preflight", status.getString("schemaVersion"));
        assertEquals("self", status.getString("packageScope"));
        assertEquals(
            "external-provisioning-required",
            status.getString(AppGameAndroidAuthorityPreflight.FIELD_PROVISIONING_STATE)
        );
        assertFalse(status.getBoolean(
            AppGameAndroidAuthorityPreflight.FIELD_DEVICE_ADMIN_RECEIVER_DECLARED
        ));
        assertRedactionAndNonClaimFields(status);
    }

    private static void assertAllActionsBlocked(Bundle status) {
        for (String actionKey : ACTION_KEYS) {
            assertEquals(
                AppGameAndroidAuthorityPreflight.ACTION_BLOCKED,
                status.getString(actionKey)
            );
        }
    }

    private static void assertRedactionAndNonClaimFields(Bundle status) {
        assertFalse(status.getBoolean("rawPackageNamesStored"));
        assertFalse(status.getBoolean("rawDeviceSerialStored"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("childDeviceDeliveryClaimed"));
    }
}
