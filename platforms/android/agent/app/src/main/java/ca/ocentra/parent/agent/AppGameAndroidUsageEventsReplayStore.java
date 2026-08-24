package ca.ocentra.parent.agent;

import android.content.Context;
import android.content.SharedPreferences;

final class AppGameAndroidUsageEventsReplayStore {
    private static final Object LOCK = new Object();
    private static final String PREFERENCES = "app_game_android_usage_events_replay";
    private static final String HAS_SNAPSHOT = "hasSnapshot";
    private static final String SAMPLE_STATE = "sampleState";
    private static final String EVENT_COUNT = "eventCount";
    private static final String FOREGROUND_EVENT_COUNT = "foregroundEventCount";
    private static final String OBSERVED_AT = "observedAtEpochMillis";
    private static final String GENERATION = "generation";

    private AppGameAndroidUsageEventsReplayStore() {}

    static boolean persist(
        Context context,
        String sampleState,
        long eventCount,
        long foregroundEventCount,
        long observedAtEpochMillis
    ) {
        if (context == null || eventCount < 0L || foregroundEventCount < 0L ||
            foregroundEventCount > eventCount || observedAtEpochMillis <= 0L) {
            return false;
        }
        synchronized (LOCK) {
            SharedPreferences preferences = preferences(context);
            long generation = increment(preferences.getLong(GENERATION, 0L));
            return preferences.edit()
                .putBoolean(HAS_SNAPSHOT, true)
                .putString(SAMPLE_STATE, sampleState)
                .putLong(EVENT_COUNT, eventCount)
                .putLong(FOREGROUND_EVENT_COUNT, foregroundEventCount)
                .putLong(OBSERVED_AT, observedAtEpochMillis)
                .putLong(GENERATION, generation)
                .commit();
        }
    }

    static Snapshot read(Context context) {
        synchronized (LOCK) {
            SharedPreferences preferences = preferences(context);
            if (!preferences.getBoolean(HAS_SNAPSHOT, false)) {
                return null;
            }
            String sampleState = preferences.getString(
                SAMPLE_STATE,
                AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_UNAVAILABLE
            );
            long eventCount = nonNegative(preferences.getLong(EVENT_COUNT, 0L));
            long foregroundEventCount = nonNegative(
                preferences.getLong(FOREGROUND_EVENT_COUNT, 0L)
            );
            long observedAt = nonNegative(preferences.getLong(OBSERVED_AT, 0L));
            long generation = nonNegative(preferences.getLong(GENERATION, 0L));
            if (foregroundEventCount > eventCount || observedAt <= 0L || generation <= 0L) {
                return null;
            }
            return new Snapshot(sampleState, eventCount, foregroundEventCount, observedAt, generation);
        }
    }

    private static SharedPreferences preferences(Context context) {
        return context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE);
    }

    private static long nonNegative(long value) {
        return Math.max(0L, value);
    }

    private static long increment(long value) {
        return value == Long.MAX_VALUE ? Long.MAX_VALUE : value + 1L;
    }

    static final class Snapshot {
        final String sampleState;
        final long eventCount;
        final long foregroundEventCount;
        final long observedAtEpochMillis;
        final long generation;

        Snapshot(
            String sampleState,
            long eventCount,
            long foregroundEventCount,
            long observedAtEpochMillis,
            long generation
        ) {
            this.sampleState = sampleState;
            this.eventCount = eventCount;
            this.foregroundEventCount = foregroundEventCount;
            this.observedAtEpochMillis = observedAtEpochMillis;
            this.generation = generation;
        }
    }
}
