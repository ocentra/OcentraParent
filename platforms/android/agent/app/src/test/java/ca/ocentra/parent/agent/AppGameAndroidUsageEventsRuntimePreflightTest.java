package ca.ocentra.parent.agent;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertTrue;

import android.app.AppOpsManager;
import android.app.Application;
import android.content.Context;
import android.content.ContextWrapper;
import android.os.Bundle;
import android.os.Process;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.RobolectricTestRunner;
import org.robolectric.RuntimeEnvironment;
import org.robolectric.Shadows;
import org.robolectric.shadows.ShadowAppOpsManager;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidUsageEventsRuntimePreflightTest {
    private static final String USAGE_STATS_SERVICE_STATE = "usageStatsServiceState";
    private static final String REPLAY_PREFERENCES = "app_game_android_usage_events_replay";

    @Before
    public void resetRuntimeState() {
        ShadowAppOpsManager.reset();
        clearReplayState();
    }

    @After
    public void restoreRuntimeState() {
        ShadowAppOpsManager.reset();
        clearReplayState();
    }

    @Test
    public void nullContextFailsClosedWithoutReadinessOrClaims() {
        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(null);

        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_CHECK_UNAVAILABLE,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertEquals(
            "service-unavailable",
            status.getString(USAGE_STATS_SERVICE_STATE)
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE,
            status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertFalse(status.getBoolean(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_CURRENT
        ));
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void appOpsDeniedRemainsBlockedWhenUsageStatsServiceIsVisible() {
        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(
            application()
        );

        assertNotEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals("service-visible", status.getString(USAGE_STATS_SERVICE_STATE));
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertTrue(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_PERMISSION_REQUIRED.equals(
                status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
            ) || AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE.equals(
                status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
            )
        );
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void malformedContextFailsClosedWithoutThrowing() {
        Context context = new ContextWrapper(application()) {
            @Override
            public String getPackageName() {
                throw new IllegalStateException("package-name-unavailable");
            }
        };

        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(context);

        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_CHECK_UNAVAILABLE,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE,
            status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void appOpsServiceFailureFailsClosedWithoutThrowing() {
        Context context = new ContextWrapper(application()) {
            @Override
            public Object getSystemService(String name) {
                if (Context.APP_OPS_SERVICE.equals(name)) {
                    throw new IllegalStateException("app-ops-unavailable");
                }
                return super.getSystemService(name);
            }
        };

        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(context);

        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_CHECK_UNAVAILABLE,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_PERMISSION_REQUIRED,
            status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void grantedAppOpsWithMissingUsageStatsServiceBlocksReadiness() {
        Application application = application();
        allowUsageStats(application);
        Context context = new ContextWrapper(application) {
            @Override
            public Object getSystemService(String name) {
                if (Context.USAGE_STATS_SERVICE.equals(name)) {
                    return null;
                }
                return super.getSystemService(name);
            }
        };

        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(context);

        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals("service-unavailable", status.getString(USAGE_STATS_SERVICE_STATE));
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE,
            status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertFalse(status.getBoolean(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_CURRENT
        ));
        assertRedactedAndUnclaimed(status);
    }

    @Test
    public void usageStatsServiceFailureFailsClosedWithoutThrowing() {
        Application application = application();
        allowUsageStats(application);
        Context context = new ContextWrapper(application) {
            @Override
            public Object getSystemService(String name) {
                if (Context.USAGE_STATS_SERVICE.equals(name)) {
                    throw new IllegalStateException("usage-stats-unavailable");
                }
                return super.getSystemService(name);
            }
        };

        Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(context);

        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals("service-unavailable", status.getString(USAGE_STATS_SERVICE_STATE));
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            status.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE,
            status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertRedactedAndUnclaimed(status);
    }

    private static Application application() {
        return RuntimeEnvironment.getApplication();
    }

    private static void allowUsageStats(Application application) {
        AppOpsManager appOpsManager = (AppOpsManager) application.getSystemService(
            Context.APP_OPS_SERVICE
        );
        ShadowAppOpsManager shadowAppOpsManager = Shadows.shadowOf(appOpsManager);
        shadowAppOpsManager.setMode(
            AppOpsManager.OPSTR_GET_USAGE_STATS,
            Process.myUid(),
            application.getPackageName(),
            AppOpsManager.MODE_ALLOWED
        );
    }

    private static void clearReplayState() {
        application()
            .getSharedPreferences(REPLAY_PREFERENCES, Context.MODE_PRIVATE)
            .edit()
            .clear()
            .commit();
    }

    private static void assertRedactedAndUnclaimed(Bundle status) {
        assertArrayEquals(new String[0], status.getStringArray("proofRefs"));
        assertFalse(status.getBoolean("rawUsageEventsStored"));
        assertFalse(status.getBoolean("packageNamesStored"));
        assertFalse(status.getBoolean("rawActivityRowsStored"));
        assertFalse(status.getBoolean("runtimeCollectionClaimed"));
        assertFalse(status.getBoolean("adapterDispatchClaimed"));
        assertFalse(status.getBoolean("platformEnforcementClaimed"));
        assertFalse(status.getBoolean("childDeviceDeliveryClaimed"));
    }
}
