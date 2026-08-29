package ca.ocentra.parent.agent;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import android.app.AppOpsManager;
import android.app.UiAutomation;
import android.content.Context;
import android.content.SharedPreferences;
import android.os.Bundle;
import android.os.ParcelFileDescriptor;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import androidx.test.platform.app.InstrumentationRegistry;

import java.io.IOException;
import java.io.InputStream;

import org.junit.Test;
import org.junit.runner.RunWith;

@RunWith(AndroidJUnit4.class)
public final class AppGameAndroidUsageEventsChildRuntimeReplayInstrumentedTest {
    private static final String USAGE_EVENTS_PREFERENCES =
        "app_game_android_usage_events_replay";
    private static final String CHILD_REPLAY_PREFERENCES =
        "app_game_android_child_runtime_replay";
    private static final int MAX_SHELL_OUTPUT_BYTES = 4096;

    @Test
    public void ownerProducedSourceSupportsDurableReplayGenerationRules() throws Exception {
        Context targetContext = InstrumentationRegistry.getInstrumentation().getTargetContext();
        UiAutomation automation = InstrumentationRegistry.getInstrumentation().getUiAutomation();
        String packageName = targetContext.getPackageName();
        clearReplayState(targetContext);
        try {
            setUsageStatsMode(automation, packageName, "allow");

            AppGameAndroidUsageEventsRuntimePreflight.ChildRuntimeSource firstSource =
                AppGameAndroidUsageEventsRuntimePreflight.createChildRuntimeSource(targetContext);
            assertEquals(
                AppGameAndroidUsageEventsRuntimePreflight.PERMISSION_GRANTED,
                firstSource.permissionState()
            );
            assertTrue(firstSource.current());
            assertTrue(firstSource.generation() > 0L);

            Bundle first = AppGameAndroidUsageEventsChildRuntimeReplay.consume(
                targetContext,
                firstSource
            );
            assertEquals(
                AppGameAndroidUsageEventsChildRuntimeReplay.CONSUMED,
                first.getString(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE)
            );
            assertReplayNonClaims(first);
            assertEquals(
                firstSource.generation(),
                first.getLong(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_SOURCE_GENERATION)
            );
            assertEquals(firstSource.eventCount(), first.getLong("eventCount"));
            assertEquals(
                firstSource.foregroundEventCount(),
                first.getLong("foregroundEventCount")
            );

            Bundle duplicate = AppGameAndroidUsageEventsChildRuntimeReplay.consume(
                targetContext,
                firstSource
            );
            assertEquals(
                AppGameAndroidUsageEventsChildRuntimeReplay.BLOCKED,
                duplicate.getString(
                    AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE
                )
            );
            assertEquals(
                "child-runtime-replay-generation-not-newer",
                duplicate.getString("blockReason")
            );
            assertReplayNonClaims(duplicate);

            AppGameAndroidUsageEventsRuntimePreflight.ChildRuntimeSource newerSource =
                AppGameAndroidUsageEventsRuntimePreflight.createChildRuntimeSource(targetContext);
            assertTrue(newerSource.generation() > firstSource.generation());

            Bundle newer = AppGameAndroidUsageEventsChildRuntimeReplay.consume(
                targetContext,
                newerSource
            );
            assertEquals(
                AppGameAndroidUsageEventsChildRuntimeReplay.CONSUMED,
                newer.getString(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE)
            );
            assertEquals(
                newerSource.generation(),
                newer.getLong(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_SOURCE_GENERATION)
            );
            assertEquals(newerSource.eventCount(), newer.getLong("eventCount"));
            assertEquals(
                newerSource.foregroundEventCount(),
                newer.getLong("foregroundEventCount")
            );
            assertReplayNonClaims(newer);

            Bundle older = AppGameAndroidUsageEventsChildRuntimeReplay.consume(
                targetContext,
                firstSource
            );
            assertEquals(
                AppGameAndroidUsageEventsChildRuntimeReplay.BLOCKED,
                older.getString(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE)
            );
            assertEquals(
                "child-runtime-replay-generation-not-newer",
                older.getString("blockReason")
            );
            assertEquals(
                newerSource.generation(),
                older.getLong(AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_SOURCE_GENERATION)
            );
            assertReplayNonClaims(older);

            Bundle readback = AppGameAndroidUsageEventsChildRuntimeReplay.read(targetContext);
            assertEquals(
                AppGameAndroidUsageEventsChildRuntimeReplay.CONSUMED,
                readback.getString(
                    AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_CONSUMER_STATE
                )
            );
            assertEquals(
                newerSource.generation(),
                readback.getLong(
                    AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_SOURCE_GENERATION
                )
            );
            assertEquals(
                newerSource.sampleState(),
                readback.getString(
                    AppGameAndroidUsageEventsChildRuntimeReplay.FIELD_SOURCE_SAMPLE_STATE
                )
            );
            assertEquals(newerSource.eventCount(), readback.getLong("eventCount"));
            assertEquals(
                newerSource.foregroundEventCount(),
                readback.getLong("foregroundEventCount")
            );
            assertReplayNonClaims(readback);
        } finally {
            setUsageStatsMode(automation, packageName, "default");
            clearReplayState(targetContext);
        }
    }

    private static void setUsageStatsMode(
        UiAutomation automation,
        String packageName,
        String mode
    ) throws IOException {
        ParcelFileDescriptor output = automation.executeShellCommand(
            "appops set " + packageName + " " + AppOpsManager.OPSTR_GET_USAGE_STATS + " " + mode
        );
        if (output == null) {
            return;
        }
        try (InputStream input = new ParcelFileDescriptor.AutoCloseInputStream(output)) {
            byte[] buffer = new byte[256];
            int remaining = MAX_SHELL_OUTPUT_BYTES;
            while (remaining > 0) {
                int read = input.read(buffer, 0, Math.min(buffer.length, remaining));
                if (read <= 0) {
                    return;
                }
                remaining -= read;
            }
        }
    }

    private static void clearReplayState(Context context) {
        clearPreferences(context, USAGE_EVENTS_PREFERENCES);
        clearPreferences(context, CHILD_REPLAY_PREFERENCES);
    }

    private static void clearPreferences(Context context, String name) {
        SharedPreferences preferences = context.getSharedPreferences(name, Context.MODE_PRIVATE);
        if (!preferences.edit().clear().commit()) {
            throw new IllegalStateException("unable to clear " + name);
        }
    }

    private static void assertReplayNonClaims(Bundle replay) {
        assertNotNull(replay.getStringArray("openGaps"));
        assertFalse(replay.getBoolean("rawUsageEventsStored"));
        assertFalse(replay.getBoolean("packageNamesStored"));
        assertFalse(replay.getBoolean("rawActivityRowsStored"));
        assertFalse(replay.getBoolean("childDeviceDeliveryClaimed"));
        assertFalse(replay.getBoolean("adapterDispatchClaimed"));
        assertFalse(replay.getBoolean("platformEnforcementClaimed"));
    }
}
