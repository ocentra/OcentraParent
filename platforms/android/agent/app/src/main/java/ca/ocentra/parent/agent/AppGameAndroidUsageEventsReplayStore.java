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
            foregroundEventCount > eventCount || observedAtEpochMillis <= 0L ||
            observedAtEpochMillis > System.currentTimeMillis() + 5000L ||
            !isValidSampleState(sampleState)) {
            return false;
        }
        synchronized (LOCK) {
            SharedPreferences preferences = preferences(context);
            final long generation;
            try {
                if (preferences.getBoolean(HAS_SNAPSHOT, false)) {
                    Snapshot existing = readLocked(preferences);
                    if (existing == null || existing.generation == Long.MAX_VALUE ||
                        observedAtEpochMillis < existing.observedAtEpochMillis) {
                        return false;
                    }
                    generation = existing.generation + 1L;
                } else {
                    if (hasSnapshotFields(preferences)) {
                        return false;
                    }
                    generation = 1L;
                }
            } catch (RuntimeException error) {
                return false;
            }
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
        if (context == null) {
            return null;
        }
        synchronized (LOCK) {
            SharedPreferences preferences = preferences(context);
            try {
                if (!preferences.getBoolean(HAS_SNAPSHOT, false)) {
                    return null;
                }
                return readLocked(preferences);
            } catch (RuntimeException error) {
                return null;
            }
        }
    }

    private static SharedPreferences preferences(Context context) {
        return context.getSharedPreferences(PREFERENCES, Context.MODE_PRIVATE);
    }

    private static Snapshot readLocked(SharedPreferences preferences) {
        if (!hasSnapshotFields(preferences)) {
            return null;
        }
        String sampleState = preferences.getString(SAMPLE_STATE, null);
        long eventCount = preferences.getLong(EVENT_COUNT, Long.MIN_VALUE);
        long foregroundEventCount = preferences.getLong(
            FOREGROUND_EVENT_COUNT,
            Long.MIN_VALUE
        );
        long observedAt = preferences.getLong(OBSERVED_AT, Long.MIN_VALUE);
        long generation = preferences.getLong(GENERATION, Long.MIN_VALUE);
        long now = System.currentTimeMillis();
        if (!isValidSampleState(sampleState) || eventCount < 0L || foregroundEventCount < 0L ||
            foregroundEventCount > eventCount || observedAt <= 0L ||
            observedAt > now + 5000L || generation <= 0L) {
            return null;
        }
        return new Snapshot(sampleState, eventCount, foregroundEventCount, observedAt, generation);
    }

    private static boolean hasSnapshotFields(SharedPreferences preferences) {
        return preferences.contains(SAMPLE_STATE) ||
            preferences.contains(EVENT_COUNT) ||
            preferences.contains(FOREGROUND_EVENT_COUNT) ||
            preferences.contains(OBSERVED_AT) ||
            preferences.contains(GENERATION);
    }

    private static boolean isValidSampleState(String sampleState) {
        return AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_OBSERVED.equals(sampleState) ||
            AppGameAndroidUsageEventsRuntimePreflight.SAMPLE_EMPTY.equals(sampleState);
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
