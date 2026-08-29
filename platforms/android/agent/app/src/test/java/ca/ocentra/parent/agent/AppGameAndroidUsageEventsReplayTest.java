package ca.ocentra.parent.agent;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertTrue;

import android.app.AppOpsManager;
import android.app.Application;
import android.app.usage.UsageEvents;
import android.app.usage.UsageStatsManager;
import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.Process;

import org.junit.Before;
import org.junit.Test;
import org.junit.runner.RunWith;

import org.robolectric.RobolectricTestRunner;
import org.robolectric.RuntimeEnvironment;
import org.robolectric.Shadows;
import org.robolectric.shadows.ShadowAppOpsManager;
import org.robolectric.shadows.ShadowUsageStatsManager;

import java.util.Arrays;
import java.util.HashSet;

@RunWith(RobolectricTestRunner.class)
public final class AppGameAndroidUsageEventsReplayTest {
    private static final String REPLAY_PREFERENCES = "app_game_android_usage_events_replay";
    private static final String HAS_SNAPSHOT = "hasSnapshot";
    private static final String SAMPLE_STATE = "sampleState";
    private static final String EVENT_COUNT = "eventCount";
    private static final String FOREGROUND_EVENT_COUNT = "foregroundEventCount";
    private static final String OBSERVED_AT = "observedAtEpochMillis";
    private static final String GENERATION = "generation";

    @Before
    public void clearDurableState() {
        application().getSharedPreferences(REPLAY_PREFERENCES, Context.MODE_PRIVATE)
            .edit()
            .clear()
            .commit();
    }

    @Test
    public void persistRejectsInvalidCountsStateAndTimestamps() {
        Context context = application();
        long now = System.currentTimeMillis();

        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
            -1L,
            0L,
            now
        ));
        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
            1L,
            2L,
            now
        ));
        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            "sample-unavailable",
            0L,
            0L,
            now
        ));
        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY,
            0L,
            0L,
            0L
        ));
        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY,
            0L,
            0L,
            now + 60000L
        ));
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));
    }

    @Test
    public void persistAndReadIncrementGenerationForNewerObservation() {
        Context context = application();
        long firstObservedAt = System.currentTimeMillis();

        assertTrue(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
            4L,
            2L,
            firstObservedAt
        ));
        AppGameAndroidUsageEventsReplayStore.Snapshot first =
            AppGameAndroidUsageEventsReplayStore.read(context);
        assertNotNull(first);
        assertEquals(1L, first.generation);
        assertEquals(4L, first.eventCount);
        assertEquals(2L, first.foregroundEventCount);

        assertTrue(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
            5L,
            3L,
            firstObservedAt + 1L
        ));
        AppGameAndroidUsageEventsReplayStore.Snapshot second =
            AppGameAndroidUsageEventsReplayStore.read(context);
        assertNotNull(second);
        assertEquals(2L, second.generation);
        assertEquals(5L, second.eventCount);
        assertEquals(3L, second.foregroundEventCount);
    }

    @Test
    public void olderObservationCannotReplaceCurrentSnapshot() {
        Context context = application();
        long observedAt = System.currentTimeMillis();

        assertTrue(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY,
            0L,
            0L,
            observedAt
        ));
        assertFalse(AppGameAndroidUsageEventsReplayStore.persist(
            context,
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
            2L,
            1L,
            observedAt - 1L
        ));

        AppGameAndroidUsageEventsReplayStore.Snapshot current =
            AppGameAndroidUsageEventsReplayStore.read(context);
        assertNotNull(current);
        assertEquals(AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY, current.sampleState);
        assertEquals(0L, current.eventCount);
        assertEquals(1L, current.generation);
    }

    @Test
    public void usageEventsReductionCountsOnlyInWindowAndPersistsCountOnlySnapshot() {
        Application application = application();
        AppOpsManager appOpsManager = (AppOpsManager) application.getSystemService(
            Context.APP_OPS_SERVICE
        );
        UsageStatsManager usageStatsManager = (UsageStatsManager) application.getSystemService(
            Context.USAGE_STATS_SERVICE
        );
        assertNotNull(appOpsManager);
        assertNotNull(usageStatsManager);

        ShadowAppOpsManager shadowAppOpsManager = Shadows.shadowOf(appOpsManager);
        ShadowUsageStatsManager shadowUsageStatsManager = Shadows.shadowOf(usageStatsManager);
        long now = System.currentTimeMillis();
        try {
            ShadowUsageStatsManager.reset();
            ShadowAppOpsManager.reset();
            shadowAppOpsManager.setMode(
                AppOpsManager.OPSTR_GET_USAGE_STATS,
                Process.myUid(),
                application.getPackageName(),
                AppOpsManager.MODE_ALLOWED
            );
            addUsageEvent(
                shadowUsageStatsManager,
                "com.example.inside.foreground",
                "InsideForeground",
                now - 1000L,
                UsageEvents.Event.MOVE_TO_FOREGROUND
            );
            addUsageEvent(
                shadowUsageStatsManager,
                "com.example.inside.resumed",
                "InsideResumed",
                now - 2000L,
                UsageEvents.Event.ACTIVITY_RESUMED
            );
            addUsageEvent(
                shadowUsageStatsManager,
                "com.example.inside.paused",
                "InsidePaused",
                now - 3000L,
                UsageEvents.Event.ACTIVITY_PAUSED
            );
            addUsageEvent(
                shadowUsageStatsManager,
                "com.example.outside.old",
                "OutsideOld",
                now - AppGameAndroidUsageEventsRuntimePreflight.DEFAULT_SAMPLE_LOOKBACK_MILLIS -
                    1000L,
                UsageEvents.Event.MOVE_TO_FOREGROUND
            );

            Bundle status = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(
                application
            );
            assertEquals(
                AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED,
                status.getString(
                    AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
                )
            );
            assertEquals(
                AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
                status.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
            );
            assertEquals(
                3L,
                status.getLong(
                    AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_EVENT_COUNT_LONG
                )
            );
            assertEquals(
                2L,
                status.getLong(
                    AppGameAndroidUsageEventsRuntimePreflight.FIELD_FOREGROUND_EVENT_COUNT_LONG
                )
            );
            assertEquals(
                AppGameAndroidUsageEventsRuntimePreflight.DURABLE_REPLAY_PERSISTED,
                status.getString(
                    AppGameAndroidUsageEventsRuntimePreflight.FIELD_DURABLE_REPLAY_STATE
                )
            );
            assertTrue(status.getBoolean(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_CURRENT
            ));
            assertEquals(
                3L,
                status.getLong(AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_EVENT_COUNT)
            );
            assertEquals(
                2L,
                status.getLong(
                    AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_FOREGROUND_EVENT_COUNT
                )
            );

            AppGameAndroidUsageEventsReplayStore.Snapshot snapshot =
                AppGameAndroidUsageEventsReplayStore.read(application);
            assertNotNull(snapshot);
            assertEquals(3L, snapshot.eventCount);
            assertEquals(2L, snapshot.foregroundEventCount);
            assertEquals(
                AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED,
                snapshot.sampleState
            );
            SharedPreferences preferences = application.getSharedPreferences(
                REPLAY_PREFERENCES,
                Context.MODE_PRIVATE
            );
            assertEquals(
                new HashSet<>(Arrays.asList(
                    HAS_SNAPSHOT,
                    SAMPLE_STATE,
                    EVENT_COUNT,
                    FOREGROUND_EVENT_COUNT,
                    OBSERVED_AT,
                    GENERATION
                )),
                preferences.getAll().keySet()
            );
            assertRedactedAndUnclaimed(status);
        } finally {
            ShadowUsageStatsManager.reset();
            ShadowAppOpsManager.reset();
            clearDurableState();
        }
    }

    @Test
    public void malformedOrPartialDurableStateIsRejected() {
        Context context = application();
        SharedPreferences preferences = context.getSharedPreferences(
            REPLAY_PREFERENCES,
            Context.MODE_PRIVATE
        );
        long now = System.currentTimeMillis();

        preferences.edit()
            .putBoolean(HAS_SNAPSHOT, true)
            .putString(SAMPLE_STATE, AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY)
            .putLong(EVENT_COUNT, 0L)
            .commit();
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));

        preferences.edit()
            .clear()
            .putString(SAMPLE_STATE, AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY)
            .commit();
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));

        preferences.edit()
            .clear()
            .putBoolean(HAS_SNAPSHOT, true)
            .putString(SAMPLE_STATE, "invalid-state")
            .putLong(EVENT_COUNT, 1L)
            .putLong(FOREGROUND_EVENT_COUNT, 0L)
            .putLong(OBSERVED_AT, now)
            .putLong(GENERATION, 1L)
            .commit();
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));

        preferences.edit()
            .clear()
            .putBoolean(HAS_SNAPSHOT, true)
            .putString(SAMPLE_STATE, AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY)
            .putLong(EVENT_COUNT, 0L)
            .putLong(FOREGROUND_EVENT_COUNT, 0L)
            .putLong(OBSERVED_AT, now)
            .putLong(GENERATION, 0L)
            .commit();
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));

        preferences.edit()
            .clear()
            .putBoolean(HAS_SNAPSHOT, true)
            .putString(SAMPLE_STATE, AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY)
            .putString(EVENT_COUNT, "not-a-long")
            .putLong(FOREGROUND_EVENT_COUNT, 0L)
            .putLong(OBSERVED_AT, now)
            .putLong(GENERATION, 1L)
            .commit();
        assertNull(AppGameAndroidUsageEventsReplayStore.read(context));
    }

    @Test
    public void permissionBlockedAndUnavailablePreflightRedactsAndDoesNotClaim() {
        Bundle unavailable =
            AppGameAndroidUsageEventsRuntimePreflight.createUnavailableRuntimePreflightBundle();
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_CHECK_UNAVAILABLE,
            unavailable.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
            )
        );
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE,
            unavailable.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
        );
        assertRedactedAndUnclaimed(unavailable);

        Bundle blocked = AppGameAndroidUsageEventsRuntimePreflight.createRuntimePreflightBundle(
            application()
        );
        String permissionState = blocked.getString(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_PERMISSION_CHECK_STATE
        );
        assertNotEquals(AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED, permissionState);
        assertEquals(
            AppGameAndroidUsageEventsRuntimePreflight.COLLECTION_BLOCKED,
            blocked.getString(
                AppGameAndroidUsageEventsRuntimePreflight.FIELD_RUNTIME_COLLECTION_STATE
            )
        );
        assertTrue(
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_PERMISSION_REQUIRED.equals(
                blocked.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
            ) || AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE.equals(
                blocked.getString(AppGameAndroidUsageEventsRuntimePreflight.FIELD_SAMPLE_STATE)
            )
        );
        assertFalse(blocked.getBoolean(
            AppGameAndroidUsageEventsRuntimePreflight.FIELD_REPLAY_CURRENT
        ));
        assertRedactedAndUnclaimed(blocked);
    }

    private static Application application() {
        return RuntimeEnvironment.getApplication();
    }

    private static void addUsageEvent(
        ShadowUsageStatsManager usageStatsManager,
        String packageName,
        String className,
        long timestamp,
        int eventType
    ) {
        usageStatsManager.addEvent(
            ShadowUsageStatsManager.EventBuilder.buildEvent()
                .setPackage(packageName)
                .setClass(className)
                .setTimeStamp(timestamp)
                .setEventType(eventType)
                .build()
        );
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
